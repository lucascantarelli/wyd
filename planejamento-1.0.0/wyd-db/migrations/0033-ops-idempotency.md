> Pergunta: "onde o `wyd-admin` guarda a chave de idempotência das rotas do C4, e por quanto tempo?"
> Público: Desenvolvedor novo e quem opera o banco
> Dono: equipe do `wyd-db`
> Base: `lacunas.md` `M30` (decidida em 2026-11-22); `ddl-confirmado.md` §5.33; `docs/arquitetura/contrato-c4.md` §2 e §5; `../../wyd-admin/docs/specs/acoes.md` regra comum
> Revisado em: 2026-11-22

**Estado:** aceito (2026-11-24)

Trigésimo terceiro arquivo da série e o primeiro que tabela em `ops` para servir um serviço, não um
processo. Uma tabela: `ops.idempotency_record`.

!!! note "Decidido em 2026-11-22 (`M30`)"
    O dono fechou as lacunas `LAC-ADM-02` e `LAC-ADM-06` do `wyd-admin`: **tabela nova em `ops`** e
    **retenção de 7 dias**. A série passa de 32 para **33 arquivos**, `Tab.` de 97 para **98** e
    `SCHEMA_VERSION` vai a 34 na troca, já contando o lote `0034`.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0033` (decidido em 2026-11-22; próximo livre depois de `0032`) |
| Arquivo | `crates/wyd-db/migrations/0033-ops-idempotency.sql` |
| Schema | `ops` |
| Posição | 33 de 34 |
| Depende de | `0001` (schema `ops` e default privileges), `0032` (`USAGE` de `ops` para `wyd_admin`) |
| É dependência de | `0034` não a lê; as rotas do C4 dependem dela em runtime |
| Tabelas criadas | 1 |
| Objetos criados | 1 tabela, 1 índice |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6); na troca, vale 34 |

**Por que depois do `0032`:** o `GRANT` desta tabela pressupõe o `USAGE` de `ops` que o `0032` concede ao
papel. Fora isso, nada a ordena — ela não tem FK e nenhuma view a lê.

## 2. Por que tabela nova (e por que aqui)

| Alternativa | Por que não |
|---|---|
| reutilizar a chave natural de cada rota | só `0011`, `0012` e `0034` têm coluna de chave; `account_moderation`, `account_wallet`, `server_channel`, `session`, `city` e `item_price` não têm onde guardá-la |
| `UNIQUE` parcial em `ops.audit_log` | a auditoria é append-only (`M18`); guardar corpo de resposta ali exigiria `UPDATE` e `DELETE` na tabela que existe para não ser editada |
| guardar em memória do processo | o C4 §2 promete replay entre reinícios e entre réplicas; memória não sobrevive a nenhum dos dois |

`ops` e não `game`/`portal` porque a chave é do **serviço de staff**: ela existe para o contrato C4, não
para o jogador nem para o personagem. `ops` também é o schema que `wyd_read` lê por default privilege,
o que dá leitura de BI sem `GRANT` novo.

## 3. DDL

```sql
-- 0033_ops_idempotency: registro durável de idempotência do contrato do wyd-admin.
-- Não tem FK para `ops.audit_log`: como o ator da auditoria, o registro sobrevive ao alvo e não
-- bloqueia escrita. Depende de `0001` (schema `ops`) apenas.

-- migrate:up
CREATE TABLE ops.idempotency_record (
    caller        TEXT NOT NULL,
    method        TEXT NOT NULL,
    route         TEXT NOT NULL,
    key           TEXT NOT NULL,
    body_sha256   TEXT NOT NULL,
    state         TEXT NOT NULL,
    status_code   SMALLINT,
    response_body JSONB,
    audit_id      BIGINT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at     TIMESTAMPTZ,
    CONSTRAINT pk_idempotency_record PRIMARY KEY (caller, method, route, key),
    CONSTRAINT ck_idempotency_record_metodo CHECK (method IN ('POST', 'PUT')),
    CONSTRAINT ck_idempotency_record_chave CHECK (key ~ '^[A-Za-z0-9_-]{16,64}$'),
    CONSTRAINT ck_idempotency_record_corpo CHECK (body_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_idempotency_record_rota CHECK (char_length(route) BETWEEN 1 AND 128),
    CONSTRAINT ck_idempotency_record_estado CHECK (state IN ('in_progress', 'completed', 'failed')),
    CONSTRAINT ck_idempotency_record_resultado CHECK (
        (state = 'completed') = (status_code IS NOT NULL AND response_body IS NOT NULL)
    ),
    CONSTRAINT ck_idempotency_record_fechamento CHECK ((state = 'in_progress') = (closed_at IS NULL))
);

CREATE INDEX ix_idempotency_record_criado ON ops.idempotency_record (created_at);

GRANT SELECT, INSERT, UPDATE ON ops.idempotency_record TO wyd_admin;
```

```sql
-- migrate:down
REVOKE SELECT, INSERT, UPDATE ON ops.idempotency_record FROM wyd_admin;
DROP INDEX ops.ix_idempotency_record_criado;
DROP TABLE ops.idempotency_record;
```

## 4. Colunas

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `caller` | `TEXT` | não | — | identidade de quem chamou (hoje o BFF do portal); compõe o escopo da chave no C4 §2 |
| `method` | `TEXT` | não | — | `POST` ou `PUT`; `GET` é leitura e não se registra |
| `route` | `TEXT` | não | — | rota-modelo com o parâmetro literal (`/accounts/{account_id}/moderation`), não a URL concreta |
| `key` | `TEXT` | não | — | valor de `Idempotency-Key`, 16–64 caracteres de `[A-Za-z0-9_-]` |
| `body_sha256` | `TEXT` | não | — | sha256 hexadecimal do JSON canônico do corpo; corpo diferente com a mesma chave é `409` |
| `state` | `TEXT` | não | — | `in_progress`, `completed` ou `failed` |
| `status_code` | `SMALLINT` | sim | — | status HTTP devolvido, gravado só em `completed` |
| `response_body` | `JSONB` | sim | — | corpo devolvido no replay, gravado só em `completed` |
| `audit_id` | `BIGINT` | sim | — | id de `ops.audit_log` da mesma transação; sem FK, por desenho |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | início da ação; é a coluna do expurgo |
| `closed_at` | `TIMESTAMPTZ` | sim | — | fim da ação; nulo apenas em `in_progress` |

## 5. Contratos e invariantes

| Constraint | Estado que proíbe | Código |
|---|---|---|
| `pk_idempotency_record` | duas linhas com o mesmo `(caller, method, route, key)` | `23505` |
| `ck_idempotency_record_metodo` | `method` fora de `POST`/`PUT` | `23514` |
| `ck_idempotency_record_chave` | chave curta, longa ou com caractere fora do alfabeto | `23514` |
| `ck_idempotency_record_corpo` | hash que não é sha256 hexadecimal | `23514` |
| `ck_idempotency_record_estado` | estado fora dos três | `23514` |
| `ck_idempotency_record_resultado` | `completed` sem status/corpo, ou status/corpo em estado aberto | `23514` |
| `ck_idempotency_record_fechamento` | `in_progress` com `closed_at`, ou estado fechado sem carimbo | `23514` |

**Efeito por transação.** A linha nasce `in_progress` **dentro da transação da ação** e é fechada nela.
O `COMMIT` nunca publica `in_progress`:

1. **Duplicata concorrente** bloqueia na `INSERT` da PK. Quem chega depois espera o `COMMIT` alheio; se
   estourar o prazo de 2 s da transação, recebe `409 ADM_IDEMPOTENCY_IN_PROGRESS` com `Retry-After: 1`.
2. **Transação abortada** leva a linha junto: não existe chave presa nem lease a expirar.
3. **Reenvio com a mesma chave** acha a linha: `completed` devolve status e corpo; corpo diferente é
   `409`; `failed` permite repetir, reusando a linha na mesma transação.

**Retenção.** **7 dias após `created_at`**, qualquer estado, por expurgo diário de operação
(`DELETE … WHERE created_at < now() - interval '7 days'`). O piso é a janela de replay de 24 h do chamador
(`M17`); o teto é o custo — ação de staff, volume baixo. Depois do expurgo, `GET /actions/{key}` responde
`404` e a mesma chave deixa de ser reconhecida. O número é decisão, não medição.

## 6. Permissões

| Papel | Efeito |
|---|---|
| `wyd_admin` | `USAGE` em `ops` (`0032`), `SELECT`/`INSERT`/`UPDATE` na tabela (este arquivo) |
| `wyd_read` | `SELECT`, por default privilege do `0001` — leitura de BI, sem escrita |
| `wyd_game`, `wyd_portal` | nada: o `REVOKE` de `0001` os deixa sem acesso a `ops` |
| manutenção (expurgo) | `DELETE` **não** é de runtime: o expurgo roda com papel de manutenção, como o da fila (`M25`) |

## 7. Provas

| # | Invariante | Comando | Esperado |
|---:|---|---|---|
| 1 | tabela existe e vazia | `SELECT count(*) FROM ops.idempotency_record;` | `0` |
| 2 | chave fora do formato | `INSERT` com `key = 'curta'` | `23514` em `ck_idempotency_record_chave` |
| 3 | corpo fora de sha256 | `body_sha256 = 'xyz'` | `23514` em `ck_idempotency_record_corpo` |
| 4 | `completed` sem corpo | `state = 'completed', status_code = 200` | `23514` em `ck_idempotency_record_resultado` |
| 5 | chave duplicada | duas linhas com o mesmo escopo e chave | segundo `23505` em `pk_idempotency_record` |
| 6 | escrita do admin | `SET ROLE wyd_admin; INSERT` de linha fechada | `INSERT 0 1` |
| 7 | portal não escreve | `SET ROLE wyd_portal; INSERT` na mesma tabela | `42501` |
| 8 | expurgo usa o índice | `EXPLAIN` do `DELETE` por `created_at` | varredura por `ix_idempotency_record_criado` |

## 8. O que este arquivo não decide

- **Não guarda a chave de `ops.delivery_queue` nem de `game.account_ledger`.** Essas continuam nas
  próprias tabelas (`0012`, `0005`); este registro é do contrato C4, não de todo mundo.
- **Não define o prazo da transação.** Os 2 s são do C4 §2 e vivem no `wyd-admin`.
- **Não cria a rotina de expurgo.** Retenção é decisão (`M30`); agendamento é de `wyd-ops`.
- **Não resolve leitura além do `404`.** Listar ações por ator continua sendo `ops.audit_log` (`M18`).

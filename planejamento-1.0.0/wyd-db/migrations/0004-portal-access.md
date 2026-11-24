> Pergunta: "como aplico o `0004_portal_access` e o que passa a existir no banco depois dele?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3, `ddl-confirmado.md` §4, `padrao-banco.md` §2/§8, `lacunas.md` §1 (`M4`, `M5`)
> Revisado em: 2026-11-17

**Estado:** aceito (2026-11-24)

Segundo arquivo do bloco de identidade. Ele cria a **sessão revogável do portal** e a **tabela única
de tokens de cerimônia da pessoa** (recuperação de senha, verificação de e-mail e troca de endereço).
Nenhuma das duas tabelas é lida pelo jogo: sessão de jogo mora em `game.session` (`0007`), e o
cliente 7662 nunca autentica por aqui.

!!! note "Aplicado em 2026-11-24"
    O arquivo `crates/wyd-db/migrations/0004_portal_access.sql` existe e a série aplicou do zero em PostgreSQL 17
    nessa data, como `wyd_migrator` (`planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §6.6.30). O
    DDL continua sendo o de `ddl-confirmado.md` §4, dono do texto; as provas de §9 não foram executadas uma a uma
    nesta leva.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0004` |
| Arquivo | `crates/wyd-db/migrations/0004_portal_access.sql` |
| Schema | `portal` |
| Posição | 4 de 34 (4º do bloco de identidade) |
| Depende de | `0001_bootstrap` (schemas, `ops.fn_set_updated_at`, permissões) e `0003_portal_person` (`portal.person` — a FK e a coluna `password_changed_at` que o fluxo de troca de senha usa) |
| É dependência de | nenhum arquivo da série: nada referencia estas duas tabelas por FK |
| Tabelas criadas | 2 (`portal.person_session`, `portal.person_token`) |
| Objetos criados | 2 tabelas, 6 índices (3 por tabela) |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

O nome do arquivo não tem sufixo de tabela porque ele cobre um assunto — **acesso da pessoa ao
portal** — e não uma entidade. É o único arquivo do bloco de identidade nessa forma.

## 2. Objetivo

Tornar a sessão do portal **revogável e expirável** (por inatividade e por prazo absoluto) e dar à
pessoa **um único mecanismo de token de cerimônia**, com propósito explícito, uso único, validade e
limite de tentativas — de modo que recuperar senha, verificar e-mail e trocar endereço não virem três
tabelas parecidas com três regras diferentes.

## 3. Pré-requisitos

1. `db/roles.sql` aplicado por `wyd-db provision apply` (papéis do cluster) — sem ele, o `0001` falha
   em `AUTHORIZATION`.
2. `0001_bootstrap` aplicado: `portal` existe, pertence a `wyd_owner` e `wyd_portal` tem `USAGE` nele.
3. `0003_portal_person` aplicado: `portal.person` existe. Este arquivo **não** roda antes dele — a FK
   falha com `42P01`/`23503`.
4. Conexão como `wyd_migrator`. Os `GRANT` que valem aqui vêm do `ALTER DEFAULT PRIVILEGES` do
   `0001`, que só alcança objeto **criado por** `wyd_migrator`.

## 4. DDL do `up`

```sql
-- 0004_portal_access: sessão revogável do portal e tokens de cerimônia da pessoa.
-- Depende de `portal.person` (0003). O jogo não lê nenhuma das duas tabelas.

-- migrate:up
CREATE TABLE portal.person_session (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id    BIGINT NOT NULL,
    -- Verificador do cookie: o valor é opaco e só o sha256 dele mora aqui (`A16`, 2026-11-18).
    token_hash   TEXT NOT NULL,
    ip           INET,
    user_agent   TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Último sinal de vida. O timeout IDLE é `last_seen_at + intervalo`, por política; não é coluna.
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Expiração ABSOLUTA: independe de atividade e não se estende.
    expires_at   TIMESTAMPTZ NOT NULL,
    revoked_at   TIMESTAMPTZ,
    CONSTRAINT pk_person_session PRIMARY KEY (id),
    CONSTRAINT fk_person_session_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT uq_person_session_token UNIQUE (token_hash),
    CONSTRAINT ck_person_session_token_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_person_session_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_person_session_revogacao CHECK (revoked_at IS NULL OR revoked_at >= created_at),
    -- Sinal de vida fora da janela da própria sessão é estado impossível.
    CONSTRAINT ck_person_session_sinal CHECK (
        last_seen_at >= created_at AND last_seen_at <= expires_at
    ),
    CONSTRAINT ck_person_session_user_agent CHECK (char_length(user_agent) <= 512)
);

CREATE INDEX ix_person_session_person ON portal.person_session (person_id);
CREATE INDEX ix_person_session_vivas ON portal.person_session (person_id, last_seen_at DESC)
    WHERE revoked_at IS NULL;
-- Reaper de `M22`: varre expiradas sem varrer a tabela.
CREATE INDEX ix_person_session_expira ON portal.person_session (expires_at);

CREATE TABLE portal.person_token (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id     BIGINT NOT NULL,
    -- Cerimônia a que o token dá direito. Três valores, três fluxos, um único formato de token.
    purpose       TEXT NOT NULL,
    -- Só o hash sha256 hex de 64; o valor em claro nunca chega ao banco.
    token_hash    TEXT NOT NULL,
    -- Origem do pedido: auditoria de abuso. O destinatário vem de `person.email`, nunca daqui.
    requested_ip  INET,
    attempt_count SMALLINT NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at    TIMESTAMPTZ NOT NULL,
    used_at       TIMESTAMPTZ,
    CONSTRAINT pk_person_token PRIMARY KEY (id),
    CONSTRAINT fk_person_token_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT uq_person_token_hash UNIQUE (token_hash),
    CONSTRAINT ck_person_token_purpose CHECK (
        purpose IN ('email_verify', 'password_reset', 'email_change')
    ),
    CONSTRAINT ck_person_token_hash_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_person_token_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_person_token_consumo CHECK (used_at IS NULL OR used_at >= created_at),
    CONSTRAINT ck_person_token_tentativas CHECK (attempt_count >= 0)
);

-- Um token vivo por pessoa e propósito: reemitir invalida o anterior (OWASP Forgot Password).
CREATE UNIQUE INDEX ux_person_token_vivo ON portal.person_token (person_id, purpose)
    WHERE used_at IS NULL;
CREATE INDEX ix_person_token_pessoa ON portal.person_token (person_id, created_at DESC);
CREATE INDEX ix_person_token_expira ON portal.person_token (expires_at) WHERE used_at IS NULL;
```

**Duas coisas que mudaram em relação à versão anterior deste arquivo** (decisões de 2026-11-17):
`person_password_reset` **não existe** — virou `person_token`, com `purpose`; e a sessão ganhou
`ck_person_session_sinal`, `ck_person_session_user_agent` e o índice de expiração. O DDL anterior foi
substituído, não corrigido: a série 1.0.0 nunca teve as duas tabelas antigas.

## 5. DDL do `down`

Ordem inversa exata do `up`. Os índices são derrubados **qualificados** de propósito: o runner fecha o
`search_path` da transação, e dentro dela ele fica vazio (`src/migrate/spec.md` §6); então
`DROP INDEX ix_…` sem schema falharia com "index does not exist" mesmo com o índice existindo em
`portal`.

```sql
-- migrate:down
DROP INDEX portal.ix_person_token_expira;
DROP INDEX portal.ix_person_token_pessoa;
DROP INDEX portal.ux_person_token_vivo;
DROP TABLE portal.person_token;
DROP INDEX portal.ix_person_session_expira;
DROP INDEX portal.ix_person_session_vivas;
DROP INDEX portal.ix_person_session_person;
DROP TABLE portal.person_session;
```

Depois deste `down`, `portal` continua com as **3** tabelas de `0003` (`person`, `person_profile`,
`person_consent`). Zero tabela só acontece quando o `down` de `0003` roda — o teste de "0 tabela ao
fim da série" é da série inteira, não deste arquivo.

## 6. Tabelas de colunas

### 6.1 `portal.person_session`

| Coluna | Tipo | Nulo | Default | O que é |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave interna. Não sai do banco; o identificador de fora é o token opaco do cookie. |
| `person_id` | `BIGINT` | não | — | Dona da sessão. FK `ON DELETE CASCADE` — apagar pessoa (reset de ambiente) leva as sessões. |
| `token_hash` | `TEXT` | não | — | sha256 hex de 64 do verificador entregue no cookie. O valor em claro nunca chega ao banco (`A16`); a revogação é `revoked_at`, e esta coluna é a chave de busca da sessão. |
| `ip` | `INET` | sim | — | Origem da sessão, para auditoria. Nunca critério de recusa nem identidade (`P-10`). |
| `user_agent` | `TEXT` | não | `''` | Cliente declarado no login. Limitado a 512 caracteres para que um cliente não grave megabytes. |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | Abertura da sessão. Piso de todas as outras marcas de tempo da linha. |
| `last_seen_at` | `TIMESTAMPTZ` | não | `now()` | Último sinal de vida. A expiração **idle** é calculada como `last_seen_at + intervalo`, na política. |
| `expires_at` | `TIMESTAMPTZ` | não | — | Expiração **absoluta**. Não se estende com atividade: mesmo com uso contínuo, a sessão morre aqui. |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | Preenchido = sessão morta (logout, troca de senha, decisão de staff). A linha **nunca é apagada** em operação. |

### 6.2 `portal.person_token`

| Coluna | Tipo | Nulo | Default | O que é |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave interna. O token em si é o par `token_hash` + valor em claro entregue ao titular. |
| `person_id` | `BIGINT` | não | — | Titular da cerimônia. FK `ON DELETE CASCADE`. |
| `purpose` | `TEXT` | não | — | Qual cerimônia o token autoriza: `email_verify`, `password_reset` ou `email_change`. Um token de um propósito **não** vale em outro. |
| `token_hash` | `TEXT` | não | — | sha256 hex de 64 do valor entregue. O valor em claro nunca chega ao banco. |
| `requested_ip` | `INET` | sim | — | Origem do pedido, para detectar abuso (flood de reset, enumeração). |
| `attempt_count` | `SMALLINT` | não | `0` | Tentativas de consumo já recusadas. O teto é política de código (`src/security/policy.rs`); o banco só impede contador negativo. |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | Emissão. |
| `expires_at` | `TIMESTAMPTZ` | não | — | Validade do token. Curta por definição; o número é política (`B1`), não schema. |
| `used_at` | `TIMESTAMPTZ` | sim | — | Consumo com sucesso. Token consumido não é reutilizado; a linha fica para auditoria. |

## 7. Contratos e invariantes

Cada linha é um estado que o banco **recusa**, com o código que o chamador recebe.

| # | Objeto | O que proíbe | SQLSTATE |
|---:|---|---|---|
| 1 | `pk_person_session` / `pk_person_token` | duas linhas com o mesmo `id` | `23505` |
| 2 | `fk_person_session_person` / `fk_person_token_person` | sessão ou token de pessoa inexistente | `23503` |
| 3 | `uq_person_session_token` | dois registros de sessão com o mesmo `token_hash` | `23505` |
| 4 | `ck_person_session_token_forma` | `token_hash` que não seja exatamente 64 hex minúsculos | `23514` |
| 5 | `ck_person_session_validade` | sessão que expira antes (ou no instante) de nascer | `23514` |
| 6 | `ck_person_session_revogacao` | revogação anterior à criação | `23514` |
| 7 | `ck_person_session_sinal` | `last_seen_at` antes de `created_at` **ou depois de `expires_at`** — é o que faz um "touch" em sessão vencida falhar em vez de ressuscitá-la | `23514` |
| 8 | `ck_person_session_user_agent` | `user_agent` com mais de 512 caracteres | `23514` |
| 9 | `uq_person_token_hash` | o mesmo hash em duas linhas (token reusado entre propósitos) | `23505` |
| 10 | `ux_person_token_vivo` | **um segundo token vivo** para a mesma pessoa e o mesmo propósito — reemitir exige consumir/revogar o anterior | `23505` |
| 11 | `ck_person_token_purpose` | `purpose` fora dos três valores | `23514` |
| 12 | `ck_person_token_hash_forma` | hash que não seja sha256 hex de 64 | `23514` |
| 13 | `ck_person_token_validade` | token que expira antes de nascer | `23514` |
| 14 | `ck_person_token_consumo` | `used_at` anterior à emissão | `23514` |
| 15 | `ck_person_token_tentativas` | `attempt_count` negativo | `23514` |
| 16 | `NOT NULL` de todas as colunas marcadas | linha sem dono, sem hash, sem validade | `23502` |
| 17 | permissão de schema | `wyd_game` tocando `portal` | `42501` |

**Invariante que o banco não expressa e a política precisa garantir:** o timeout **idle**. Ele é
`now() > last_seen_at + intervalo`; como depende do relógio, não vira CHECK. O `CHECK`
`ck_person_session_sinal` é o que impede o efeito colateral perigoso — renovar `last_seen_at` de uma
sessão já vencida.

**Consumo atômico.** O `UPDATE` que consome o token precisa ser condicional
(`UPDATE … SET used_at = now() WHERE token_hash = $1 AND used_at IS NULL`) e devolver a linha; quem
não atualizou nada perdeu a corrida. Dois cliques simultâneos no link não podem trocar a senha duas
vezes.

**Troca de senha encerra sessões.** Na mesma transação que grava `portal.person.password_changed_at`,
todo `person_session` vivo da pessoa recebe `revoked_at = now()`. Sem isso, um token roubado
sobreviveria à troca que deveria matá-lo (`M4`).

## 8. Permissões

Este arquivo **não tem `GRANT` explícito** — diferente de `0002`, que precisa conceder `INSERT`
append-only. Aqui vale o `ALTER DEFAULT PRIVILEGES` do `0001`:

| Papel | Efeito depois de `0004` | Como |
|---|---|---|
| `wyd_portal` | `SELECT, INSERT, UPDATE, DELETE` nas duas tabelas | default privilege do `0001` em `portal` |
| `wyd_game` | **nada**: nem `USAGE` no schema | `0001` não concede `USAGE` de `portal` a `wyd_game` → `42501` |
| `wyd_read` | **nada nas tabelas**: tem `USAGE` no schema e nenhum privilégio de tabela | consequência direta de `0001` §7 |
| `PUBLIC` | nada | `public` é o único schema aberto, e não tem estas tabelas |

`DELETE` existe por permissão, mas **não** é caminho de operação: pessoa não é apagada (`A12` fecha
com tombstone) e sessão/token são revogados/consumidos, não removidos. O `ON DELETE CASCADE` serve ao
reset de ambiente, não ao fluxo normal.

## 9. Prova de execução

PG17 efêmero, sem tocar a série herdada. Pressupõe `provision apply` feito e a conexão como
`wyd_migrator` (com outro papel, os default privileges não valem e as provas 10–12 mentem).

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0001_bootstrap.sql
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0003_portal_person.sql
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0004_portal_access.sql
```

Supõe uma pessoa de teste já criada (`INSERT INTO portal.person (email, password_hash) VALUES
('prova@wyd.local', '$argon2id$…')`).

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | duas tabelas a mais em `portal` | `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'portal';` | `5` (3 de `0003` + 2 daqui) |
| 2 | `token_hash` malformado | `INSERT INTO portal.person_session (person_id, token_hash, expires_at) VALUES (1, 'XYZ', now() + interval '8 hours');` | `23514` em `ck_person_session_token_forma` |
| 3 | `token_hash` repetido | dois `INSERT` com o mesmo `token_hash` | `23505` em `uq_person_session_token` |
| 4 | validade invertida | `… VALUES (1, repeat('a',64), now() - interval '1 hour');` | `23514` em `ck_person_session_validade` |
| 5a | cria a linha com marcas explícitas (o banco não deixa nascer vencida) | `INSERT INTO portal.person_session (person_id, token_hash, created_at, last_seen_at, expires_at) VALUES (1, repeat('c',64), now() - interval '1 hour', now() - interval '1 hour', now() + interval '1 second');` | `INSERT 0 1` |
| 5b | empurra `expires_at` para o passado mantendo `expires_at > created_at` | `UPDATE portal.person_session SET expires_at = now() - interval '30 minutes' WHERE token_hash = repeat('c',64);` | `UPDATE 1` — se der `UPDATE 0`, a linha não existe e 5c não prova nada |
| 5c | touch em sessão vencida | `UPDATE portal.person_session SET last_seen_at = now() WHERE token_hash = repeat('c',64);` | `23514` em `ck_person_session_sinal` |
| 6 | `user_agent` gigante | `… repeat('a', 600)` | `23514` em `ck_person_session_user_agent` |
| 7 | dois tokens vivos | dois `INSERT` com `purpose = 'password_reset'` e `used_at NULL` para a mesma pessoa | `23505` em `ux_person_token_vivo` |
| 8 | reemitir funciona | `UPDATE` marcando `used_at = now()` no primeiro e inserindo o segundo | sucesso |
| 9 | propósito inventado | `purpose = 'reset_all'` | `23514` em `ck_person_token_purpose` |
| 10 | jogo lê sessão do portal | `SET ROLE wyd_game; SELECT count(*) FROM portal.person_session;` | `42501 permission denied for schema portal` |
| 11 | portal escreve | `SET ROLE wyd_portal; INSERT INTO portal.person_session (person_id, token_hash, expires_at) VALUES (1, repeat('b',64), now() + interval '8 hours');` | `INSERT 0 1` |
| 12 | leitura de BI | `SET ROLE wyd_read; SELECT count(*) FROM portal.person_session;` | `42501 permission denied for table person_session` |
| 13 | `down` derruba só o que criou | aplicar o bloco `-- migrate:down` e repetir 1 | `3` |
| 14 | ciclo fecha | refazer o `up` depois do `down` | prova 1 de novo (`5`) |

**Por que a prova 5 tem três passos.** Uma sessão não pode nascer vencida: `created_at` tem default
`now()` e `ck_person_session_validade` exige `expires_at > created_at`, então `UPDATE … WHERE
expires_at < now()` casa **zero linha** e "passa" sem violar nada — foi o defeito da versão anterior
desta prova. E empurrar `expires_at` para o passado com `created_at`/`last_seen_at` no presente não
chega a `ck_person_session_sinal`: quem reprova primeiro é `ck_person_session_validade`, ou seja,
`23514` na constraint errada — o mesmo que não provar. Por isso 5a fixa as três marcas (janela já
vencida, `last_seen_at` dentro dela), 5b move só `expires_at` e 5c tenta o toque. A prova vale pelo
nome da constraint no erro, não só pelo SQLSTATE.

A prova 13 é o gate do arquivo: `down` que levasse uma tabela de `0003` junto seria defeito, e o
`DROP SCHEMA` sem `CASCADE` do `0001` denunciaria o mesmo problema mais adiante.

## 10. O que este arquivo não decide

| Assunto | Onde é decidido | Efeito aqui |
|---|---|---|
| Transporte da sessão: JWT assinado com `jti` × token opaco com `sha256` | `lacunas.md` `A16` (**fechada em 2026-11-18**: token opaco) | nada a mudar: a coluna `token_hash` é o `sha256` de 64 hex do verificador, e o valor em claro nunca entra no banco |
| Fluxo de troca de e-mail | `lacunas.md` `A21` (marco `P4`) | `purpose = 'email_change'` está reservado e **não** há emissor |
| Teto de tentativas de consumo do token | código (`src/security/policy.rs`) | a coluna `attempt_count` existe; o número não é de documento |
| Intervalo de idle da sessão | `lacunas.md` `M4` (fechada) + `src/security/policy.rs` | o campo `last_seen_at` existe; o intervalo é política de código, não coluna |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

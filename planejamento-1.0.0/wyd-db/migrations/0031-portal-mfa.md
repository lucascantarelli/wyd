> Pergunta: "como o segundo fator de staff existe no banco, quem escreve, o que acontece na revogação e por que o segredo não é hash?"
> Público: Desenvolvedor novo e segurança
> Dono: equipe do `wyd-db`
> Base: `ddl-confirmado.md` §5.20 (aprovado em 2026-11-20); `lacunas.md` `A20`; `padrao-banco.md` §3/§4/§5; `wyd-portal/lacunas.md` `LAC-POR-05`
> Revisado em: 2026-11-20

Trigésimo primeiro arquivo da série, adendo ao fim como o `0030`. Cria `portal.person_authenticator`, a
credencial separada de segundo fator que `A20` reservou e que `LAC-POR-05` decidiu para o staff do
portal. **Mecanismo não reaberto:** TOTP, só `admin`/`moderator`, `member` fora da `1.0.0`.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o desenho na rodada de perguntas de 2026-11-20, na opção "Sim, aprovar o desenho
    como está", depois de escolher `id` identity com único parcial e revogação que apaga o segredo. A
    tabela entra na conta de `ddl-confirmado.md` §1 (75 na época; hoje 109 de 109) e esta página fica `concluída` em
    `migrations-checklist.md`.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0031` (confirmado em 2026-11-20; próximo livre depois de `0030`) |
| Arquivo | `crates/wyd-db/migrations/0031_portal_mfa.sql` |
| Schema | `portal` (1 tabela) |
| Posição | 31 de 34 (segundo fator de staff) |
| Depende de | `0001` (schema `portal`, `ops.fn_set_updated_at`, default privileges), `0003` (`portal.person`, dono da credencial) |
| É dependência de | nada na série: nenhuma view de `0029` e nenhuma tabela de `game` a leem |
| Tabelas criadas | 1 (`person_authenticator`) |
| Objetos criados | 1 tabela, 2 índices (1 parcial único), 1 trigger |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6); na troca, o valor acompanha o total de arquivos |

**Por que no fim da série:** o lote não tem FK de saída para `game`/`content` e nenhuma página anterior
o lê. Renumerar arquivos para encaixá-lo perto de `0004` trocaria número citado em dezenas de páginas sem
mudar dependência. A série cresce no fim, contígua, como manda `plano-migrations.md` §2.

## 2. Objetivo

Guardar o segredo TOTP **cifrado** de cada pessoa de staff, com confirmação, último passo aceito e
revogação explícita, para o BFF exigir senha + código no login de `admin`/`moderator` — sem tocar em
`game.account`.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schema `portal`, `ops.fn_set_updated_at()` e default privileges.
2. `0003_portal_person` aplicado: `portal.person` existe (FK de dono).
3. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.20, copiado sem alteração.

```sql
CREATE TABLE portal.person_authenticator (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id        BIGINT NOT NULL,
    -- Mecanismo do segundo fator: `totp` é o único da 1.0.0 (A20); outro `kind` exige decisão nova.
    kind             TEXT NOT NULL DEFAULT 'totp',
    -- Segredo TOTP CIFRADO: o BFF precisa decifrar para validar o código, então hash não serve.
    -- A chave fica no ambiente do BFF, nunca no banco. NULL só existe em linha revogada (ver CHECK).
    secret_encrypted BYTEA,
    -- Inscrição confirmada: NULL = cadastro pendente, que ainda não vale no login.
    confirmed_at     TIMESTAMPTZ,
    -- Último passo TOTP (30 s) aceito; repetir o passo é replay e o BFF recusa (A20).
    last_used_step   BIGINT,
    -- Revogação por perda de dispositivo ou desligamento do fator; revogar apaga o segredo.
    revoked_at       TIMESTAMPTZ,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_person_authenticator PRIMARY KEY (id),
    CONSTRAINT fk_person_authenticator_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_person_authenticator_kind CHECK (kind IN ('totp')),
    -- Segredo e revogação andam juntos: ativo tem segredo; revogado não guarda segredo morto.
    CONSTRAINT ck_person_authenticator_segredo CHECK (
        (revoked_at IS NULL AND secret_encrypted IS NOT NULL)
        OR (revoked_at IS NOT NULL AND secret_encrypted IS NULL)
    ),
    -- Cifra de um segredo TOTP (nonce + segredo + tag) cabe com folga nesta faixa.
    CONSTRAINT ck_person_authenticator_segredo_tamanho CHECK (
        secret_encrypted IS NULL OR octet_length(secret_encrypted) BETWEEN 32 AND 512
    ),
    CONSTRAINT ck_person_authenticator_confirmacao CHECK (
        confirmed_at IS NULL OR confirmed_at >= created_at
    ),
    -- Só se registra passo usado depois de a inscrição valer.
    CONSTRAINT ck_person_authenticator_passo CHECK (
        last_used_step IS NULL OR (last_used_step >= 0 AND confirmed_at IS NOT NULL)
    ),
    CONSTRAINT ck_person_authenticator_revogacao CHECK (
        revoked_at IS NULL OR revoked_at >= created_at
    ),
    CONSTRAINT ck_person_authenticator_revogacao_confirmada CHECK (
        revoked_at IS NULL OR confirmed_at IS NULL OR revoked_at >= confirmed_at
    )
);

-- Um autenticador ATIVO por pessoa e tipo; revogado não bloqueia o próximo cadastro.
CREATE UNIQUE INDEX ux_person_authenticator_ativo
    ON portal.person_authenticator (person_id, kind)
    WHERE revoked_at IS NULL;

-- Lado filho da FK (P-15): fechar/apagar pessoa precisa achar as linhas sem varrer a tabela.
CREATE INDEX ix_person_authenticator_person
    ON portal.person_authenticator (person_id);

CREATE TRIGGER trg_person_authenticator_set_updated_at
    BEFORE UPDATE ON portal.person_authenticator
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP TRIGGER trg_person_authenticator_set_updated_at ON portal.person_authenticator;
DROP INDEX ix_person_authenticator_person;
DROP INDEX ux_person_authenticator_ativo;
DROP TABLE portal.person_authenticator;
```

Sem `GRANT`/`REVOKE`: o lote herda o default privilege de `0001` (ver §8) e não cria papel.

## 6. Tabela de colunas

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave interna; a linha sobrevive à revogação para manter o histórico. |
| `person_id` | `BIGINT` | não | — | FK `portal.person`: dono da credencial. `ON DELETE CASCADE` porque a linha é possuída (`P-15`). |
| `kind` | `TEXT` | não | `'totp'` | Mecanismo do segundo fator; `totp` é o único valor da `1.0.0` (`A20`). |
| `secret_encrypted` | `BYTEA` | sim | — | Segredo TOTP **cifrado**; a chave fica no ambiente do BFF. `NULL` só em linha revogada — revogar apaga o segredo. |
| `confirmed_at` | `TIMESTAMPTZ` | sim | — | Inscrição confirmada; `NULL` = cadastro pendente, que não vale no login. |
| `last_used_step` | `BIGINT` | sim | — | Último passo TOTP (30 s) aceito; impede replay do mesmo código (`A20`). |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | Instante da revogação por perda de dispositivo ou desligamento do fator (`P-20`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos; `updated_at` mantido pelo trigger. |

**Por que `BYTEA` e não `TEXT`:** `padrao-banco.md` §4 manda binário em `BYTEA` e proíbe base64 em
`TEXT`. **Por que não `_hash`:** o BFF precisa decifrar para validar o código; hash tornaria o TOTP
impossível. É a única coluna de segredo reversível da série, e o nome `secret_encrypted` é o que `A20`
reservou.

## 7. Contratos e invariantes

| # | Constraint | Proíbe | Erro |
|---:|---|---|---|
| 1 | `pk_person_authenticator` | duas linhas com o mesmo `id` | `23505` |
| 2 | `fk_person_authenticator_person` | autenticador sem pessoa existente | `23503` |
| 3 | `ck_person_authenticator_kind` | mecanismo fora de `totp` na `1.0.0` | `23514` |
| 4 | `ck_person_authenticator_segredo` | ativo sem segredo, ou revogado guardando segredo | `23514` |
| 5 | `ck_person_authenticator_segredo_tamanho` | cifra fora de 32–512 bytes | `23514` |
| 6 | `ck_person_authenticator_confirmacao` | confirmação anterior à criação | `23514` |
| 7 | `ck_person_authenticator_passo` | passo usado sem inscrição confirmada, ou negativo | `23514` |
| 8 | `ck_person_authenticator_revogacao` | revogação anterior à criação | `23514` |
| 9 | `ck_person_authenticator_revogacao_confirmada` | revogação anterior à confirmação | `23514` |
| 10 | `ux_person_authenticator_ativo` | **dois** autenticadores ativos para a mesma pessoa e tipo | `23505` |

O `23505` de `ux_person_authenticator_ativo` é o sinal de "revogue o anterior primeiro": trocar de
dispositivo é transação única que revoga a linha antiga (`revoked_at`, segredo zerado) e insere a nova,
não `UPDATE` solto no segredo.

## 8. Permissões

Nenhum `GRANT`/`REVOKE` novo: valem os default privileges de `0001` §3.

| Papel | Efeito |
|---|---|
| `wyd_portal` | `SELECT`, `INSERT`, `UPDATE`, `DELETE` — é o BFF que inscreve, confirma, valida o código e revoga. |
| `wyd_read` | tem `USAGE` em `portal`, mas **não** recebe `SELECT` por default; relatório não lê segredo. |
| `wyd_game` | nenhum: o jogo não participa do segundo fator do portal. |
| `wyd_migrator` | dono da criação; aplica o arquivo. |

**Quem lê e quem escreve.** Escrita é do BFF com a credencial `wyd_portal`, coerente com `LAC-POR-01`;
`wyd-admin` não tem `USAGE` em `portal` e não participa deste fluxo. A tela `/admin/seguranca` lê apenas
metadados (`kind`, `confirmed_at`, `last_used_step`, `revoked_at`); `secret_encrypted` nunca é projetado
em resposta, nem em log, nem em `Debug` de struct.

## 9. Retenção, revogação e ciclo de vida

- **Perdeu o dispositivo:** `UPDATE portal.person_authenticator SET revoked_at = now(),
  secret_encrypted = NULL WHERE person_id = $1 AND kind = 'totp' AND revoked_at IS NULL;` — o CHECK
  `ck_person_authenticator_segredo` exige o segredo nulo na mesma instrução, então não existe janela com
  segredo morto. O único parcial libera o cadastro seguinte.
- **Recadastro:** nova linha, novo segredo, `confirmed_at` preenchido só depois de o titular provar um
  código válido. A linha revogada permanece como registro de quando o fator existiu.
- **Pessoa suspensa ou fechada:** a linha **não** é apagada. O BFF recusa a sessão por `person.status`
  antes de avaliar o fator; fechar pessoa é soft-delete com anonimização (`A12`), e a FK `CASCADE` só
  age em expurgo manual — que não é rotina da `1.0.0`.
- **Sem expurgo automático:** a tabela é limitada ao staff do portal. Prazo de retenção de linha
  revogada, se existir, é política do `wyd-ops` (`P-21`), não coluna deste lote.
- **Recuperação do fator** é ato de staff (`A20`), nunca autoatendimento do titular: o fluxo revoga e
  reemite, e cada passo passa por `ops.audit_log` na mesma transação.

## 10. Prova de execução (quando a série for escrita)

1. `psql -v ON_ERROR_STOP=1 -f 0031_portal_mfa.sql` como `wyd_migrator` → `CREATE TABLE`, 2 `CREATE
   INDEX`, `CREATE TRIGGER`.
2. `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'portal';` → **16** (15 de
   `0003`/`0004`/`0011`/`0030` + esta).
3. `INSERT` ativo sem segredo → `23514` em `ck_person_authenticator_segredo`.
4. Dois `INSERT` ativos com a mesma pessoa e `kind` → o segundo dá `23505` em
   `ux_person_authenticator_ativo`.
5. Revogar (`revoked_at = now(), secret_encrypted = NULL`) e inserir outro ativo → `INSERT 0 1`.
6. `UPDATE ... SET revoked_at = now()` sem zerar o segredo → `23514` em `ck_person_authenticator_segredo`.
7. `UPDATE ... SET last_used_step = 42` com `confirmed_at IS NULL` → `23514` em
   `ck_person_authenticator_passo`.
8. `SET ROLE wyd_game; SELECT * FROM portal.person_authenticator;` → `42501 permission denied for schema
   portal`.

## 11. O que este arquivo não decide

| Assunto | Onde fecha |
|---|---|
| algoritmo e custódia da chave de cifra (AES-GCM, rotação, KMS × env) | implementação do `wyd-portal`; a chave nunca entra no banco |
| UX de inscrição, QR code e confirmação | `wyd-portal`, fluxo de `/admin/seguranca` |
| recuperação do segundo fator | `A20`: ato de staff, com auditoria |
| outros `kind` (WebAuthn) | 1.1, com decisão própria; o CHECK não antecipa valor |
| prazo de expurgo de linha revogada | `wyd-ops` (política de retenção) |
| linha `P` para "segredo reversível" em `padrao-banco.md` §4 | dono do padrão; a exceção está documentada em §5.20, não promovida a regra |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

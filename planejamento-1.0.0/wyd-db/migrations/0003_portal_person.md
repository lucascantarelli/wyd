> Pergunta: "como aplico o `0003_portal_person` e que identidade, perfil e consentimento passam a existir?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.1, §5.6 e §5.7; `padrao-banco.md` §2/§5/§8; `lacunas.md` (`B4`, `A12`, `M7`)
> Revisado em: 2026-11-22

**Estado:** aceito (2026-11-24)

Terceiro arquivo da série. Cria a identidade da pessoa do portal e, separadamente, os dados opcionais
de perfil e o histórico versionado de consentimentos. A pessoa é a identidade única; a conta de jogo
é criada depois em `game.account` (`0005`). Perfil ausente é válido e não transforma PII em dado de
jogo.

!!! note "Aplicado em 2026-11-24"
    O arquivo `crates/wyd-db/migrations/0003_portal_person.sql` existe e a série aplicou do zero em PostgreSQL 17
    nessa data, como `wyd_migrator` (`planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §6.6.30). O
    DDL continua sendo o de `ddl-confirmado.md` §5.1, §5.6 e §5.7, dono do texto; as provas de §9 não foram executadas uma a uma
    nesta leva.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0003` |
| Arquivo | `crates/wyd-db/migrations/0003_portal_person.sql` |
| Schema | `portal` |
| Posição | 3 de 34 (identidade da pessoa) |
| Depende de | `0001_bootstrap` (schema, `ops.fn_set_updated_at` e permissões) |
| É dependência de | `0004_portal_access` (`person_session`/`person_token`) e `0005_game_account` (`game.account.person_id`) |
| Tabelas criadas | 3 (`portal.person`, `portal.person_profile`, `portal.person_consent`) |
| Objetos criados | 3 tabelas, 5 índices, 2 triggers |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Persistir identidade, estado de conta de portal, suspensão e fechamento anonimizado, permitindo perfil
de exibição opcional e prova versionada de consentimentos, sem misturar estes dados à conta de jogo.

## 3. Pré-requisitos

1. Papéis do cluster aplicados por `wyd-db provision apply` antes da série.
2. `0001_bootstrap` aplicado: schema `portal`, função `ops.fn_set_updated_at()` e default privileges
   existem.
3. Conexão como `wyd_migrator`, para os default privileges de `0001` incidirem sobre as tabelas.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.1, §5.6 e §5.7.

```sql
-- migrate:up
CREATE TABLE portal.person (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    email             TEXT NOT NULL,
    email_fold        TEXT GENERATED ALWAYS AS (lower(btrim(email))) STORED,
    password_hash     TEXT NOT NULL,
    role              TEXT NOT NULL DEFAULT 'member',
    status            TEXT NOT NULL DEFAULT 'active',
    email_verified_at TIMESTAMPTZ,
    failed_login_count SMALLINT NOT NULL DEFAULT 0,
    locked_until      TIMESTAMPTZ,
    password_changed_at TIMESTAMPTZ,
    last_login_at     TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at         TIMESTAMPTZ,
    suspended_reason  TEXT,
    suspended_by      BIGINT,
    suspended_at      TIMESTAMPTZ,
    CONSTRAINT pk_person PRIMARY KEY (id),
    CONSTRAINT fk_person_suspended_by FOREIGN KEY (suspended_by)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_person_email_len CHECK (char_length(email) <= 254),
    CONSTRAINT ck_person_email_forma CHECK (
        email ~ '^[^@[:space:]]+@[^@[:space:]]+[.][^@[:space:]]{2,}$'
    ),
    CONSTRAINT ck_person_role CHECK (role IN ('member', 'moderator', 'admin')),
    CONSTRAINT ck_person_status CHECK (status IN ('pending', 'active', 'suspended', 'closed')),
    CONSTRAINT ck_person_senha_hasheada CHECK (password_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_person_falhas CHECK (failed_login_count >= 0),
    CONSTRAINT ck_person_senha_trocada CHECK (
        password_changed_at IS NULL OR password_changed_at >= created_at
    ),
    CONSTRAINT ck_person_fechamento_coerente CHECK ((status = 'closed') = (closed_at IS NOT NULL)),
    CONSTRAINT ck_person_fechamento_anonimo CHECK (
        status <> 'closed' OR email = 'closed-' || id || '@removido.invalid'
    ),
    CONSTRAINT ck_person_suspensao_coerente CHECK (
        (status = 'suspended') = (suspended_at IS NOT NULL)
        AND (status = 'suspended' OR (suspended_reason IS NULL AND suspended_by IS NULL))
    ),
    CONSTRAINT ck_person_suspensao_motivo CHECK (
        status <> 'suspended'
        OR (char_length(btrim(suspended_reason)) > 0 AND suspended_by IS NOT NULL)
    )
);
CREATE UNIQUE INDEX ux_person_email ON portal.person (email_fold);
CREATE INDEX ix_person_nao_ativas ON portal.person (status) WHERE status <> 'active';
CREATE INDEX ix_person_suspended_by ON portal.person (suspended_by);
CREATE TRIGGER trg_person_set_updated_at
    BEFORE UPDATE ON portal.person
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.person_profile (
    person_id    BIGINT NOT NULL,
    display_name TEXT,
    birth_date   DATE,
    country      TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_person_profile PRIMARY KEY (person_id),
    CONSTRAINT fk_person_profile_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT ck_person_profile_display_name CHECK (
        display_name IS NULL OR char_length(btrim(display_name)) BETWEEN 2 AND 32
    ),
    CONSTRAINT ck_person_profile_nascimento CHECK (
        birth_date IS NULL OR birth_date >= DATE '1900-01-01'
    ),
    CONSTRAINT ck_person_profile_pais CHECK (country IS NULL OR country ~ '^[A-Z]{2}$')
);
CREATE TRIGGER trg_person_profile_set_updated_at
    BEFORE UPDATE ON portal.person_profile
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.person_consent (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id  BIGINT NOT NULL,
    kind       TEXT NOT NULL,
    version    TEXT NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at TIMESTAMPTZ,
    CONSTRAINT pk_person_consent PRIMARY KEY (id),
    CONSTRAINT fk_person_consent_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT ck_person_consent_kind CHECK (kind IN ('terms', 'privacy', 'marketing')),
    CONSTRAINT ck_person_consent_version CHECK (char_length(btrim(version)) BETWEEN 1 AND 32),
    CONSTRAINT ck_person_consent_revogacao CHECK (revoked_at IS NULL OR revoked_at >= granted_at)
);
CREATE UNIQUE INDEX ux_person_consent_vivo ON portal.person_consent (person_id, kind)
    WHERE revoked_at IS NULL;
CREATE INDEX ix_person_consent_pessoa ON portal.person_consent (person_id, granted_at DESC);
```

## 5. DDL do `down`

Ordem inversa: consentimento e perfil antes da identidade referenciada.

```sql
-- migrate:down
DROP INDEX portal.ix_person_consent_pessoa;
DROP INDEX portal.ux_person_consent_vivo;
DROP TABLE portal.person_consent;
DROP TRIGGER trg_person_profile_set_updated_at ON portal.person_profile;
DROP TABLE portal.person_profile;
DROP TRIGGER trg_person_set_updated_at ON portal.person;
DROP INDEX portal.ix_person_suspended_by;
DROP INDEX portal.ix_person_nao_ativas;
DROP INDEX portal.ux_person_email;
DROP TABLE portal.person;
```

## 6. Tabelas de colunas

### 6.1 `portal.person`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | ID interno; modelo novo (`P-01`). |
| `email` | `TEXT` | não | — | Endereço atual; identidade do portal (`P-22`). |
| `email_fold` | `TEXT` gerada STORED | não | `lower(btrim(email))` | Chave de comparação sem caixa/espaço; derivada no banco. |
| `password_hash` | `TEXT` | não | — | Hash `argon2id`, nunca segredo em claro (`M9`). |
| `role` | `TEXT` | não | `'member'` | Papel de portal, não papel de jogo (`A15`). |
| `status` | `TEXT` | não | `'active'` | Ciclo `pending`/`active`/`suspended`/`closed`; `A7` define ativação pendente. |
| `email_verified_at` | `TIMESTAMPTZ` | sim | — | Verificação do endereço (`A7`). |
| `failed_login_count` | `SMALLINT` | não | `0` | Falhas acumuladas; limite temporal é política de código (`A8`). |
| `locked_until` | `TIMESTAMPTZ` | sim | — | Fim do lockout temporal (`A8`). |
| `password_changed_at` | `TIMESTAMPTZ` | sim | — | Última troca, usada para invalidar sessões (`M4`). |
| `last_login_at` | `TIMESTAMPTZ` | sim | — | Último login concluído. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Criação e atualização; trigger em `updated_at` (`A9`). |
| `closed_at` | `TIMESTAMPTZ` | sim | — | Fechamento/tombstone (`A12`). |
| `suspended_reason` | `TEXT` | sim | — | Motivo da suspensão (`B4`). |
| `suspended_by` | `BIGINT` | sim | — | Autor pessoa; self-FK `RESTRICT` (`B4`). |
| `suspended_at` | `TIMESTAMPTZ` | sim | — | Instante da suspensão (`B4`). |

### 6.2 `portal.person_profile`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `person_id` | `BIGINT` | não | — | Pessoa dona; PK/FK `CASCADE`. |
| `display_name` | `TEXT` | sim | — | Nome público, 2–32 quando informado (`M7`). |
| `birth_date` | `DATE` | sim | — | Data opcional; piso 1900-01-01 (`M7`). |
| `country` | `TEXT` | sim | — | Código ISO alfa-2 em maiúsculas (`M7`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo; trigger em `updated_at` (`A9`). |

### 6.3 `portal.person_consent`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave do registro. |
| `person_id` | `BIGINT` | não | — | Titular, FK `CASCADE`. |
| `kind` | `TEXT` | não | — | `terms`, `privacy` ou `marketing` (`M7`). |
| `version` | `TEXT` | não | — | Versão do documento aceito (`M7`). |
| `granted_at` | `TIMESTAMPTZ` | não | `now()` | Aceite (`P-06`). |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | Revogação; linha histórica não é apagada. |

## 7. Contratos e invariantes

| Objeto | Estado que impede | SQLSTATE |
|---|---|---|
| `ux_person_email` | e-mail dobrado duplicado, inclusive em pessoa fechada | `23505` |
| `ck_person_email_len` / `ck_person_email_forma` | e-mail acima do limite ou malformado | `23514` |
| `ck_person_role` / `ck_person_status` | papel ou estado fora do enum textual | `23514` |
| `ck_person_senha_hasheada` | senha fora do prefixo `argon2id` | `23514` |
| `ck_person_fechamento_coerente` / `_anonimo` | status e instante divergentes; endereço real mantido no tombstone | `23514` |
| `ck_person_suspensao_coerente` / `_motivo` | suspensão sem instante, motivo ou autor | `23514` |
| `fk_person_suspended_by` | autor suspenso inexistente ou apagado | `23503` |
| `ck_person_profile_*` | nome, data ou país fora do formato | `23514` |
| `fk_person_profile_person` / `fk_person_consent_person` | perfil/consentimento sem pessoa | `23503` |
| `ux_person_consent_vivo` | dois consentimentos ativos do mesmo tipo | `23505` |
| `ck_person_consent_revogacao` | revogação anterior ao aceite | `23514` |
| NOT NULL | linha sem campo obrigatório | `23502` |

## 8. Permissões

Nenhum `GRANT` explícito. Default privileges do `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_portal` | `SELECT, INSERT, UPDATE, DELETE` nas três tabelas | default privilege `portal` |
| `wyd_read` | `SELECT` nas três tabelas | default privilege `portal` |
| `wyd_game` | sem `USAGE` em `portal`; nenhum acesso | fronteira `P-17` |
| `PUBLIC` | nenhum acesso | `0001`/provisionamento |

## 9. Prova de execução

PG17 efêmero, sem tocar a série herdada. Supõe `provision apply` e conexão como `wyd_migrator`.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0001_bootstrap.sql
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0003_portal_person.sql
```

| Prova | Comando/consulta | Esperado |
|---|---|---|
| tabelas criadas | `SELECT count(*) FROM information_schema.tables WHERE table_schema='portal';` | `3` |
| e-mail duplicado sem caixa | inserir `Foo@x.com` e `foo@x.com` | segundo `23505` em `ux_person_email` |
| suspensão sem autor | inserir `status='suspended', suspended_at=now(), suspended_reason='teste'` sem `suspended_by` | `23514` em `ck_person_suspensao_motivo` |
| tombstone com endereço real | atualizar status para `closed` sem anonimizar o email | `23514` em `ck_person_fechamento_anonimo` |
| perfil órfão | `INSERT INTO portal.person_profile(person_id) VALUES (999);` | `23503` |
| consentimento duplicado vivo | inserir duas linhas do mesmo `person_id, kind`, ambas sem `revoked_at` | `23505` em `ux_person_consent_vivo` |
| portal pode escrever | `SET ROLE wyd_portal; INSERT INTO portal.person_consent (person_id,kind,version) VALUES (1,'terms','v1');` | `INSERT 0 1` |
| jogo não atravessa schema | `SET ROLE wyd_game; SELECT * FROM portal.person;` | `42501 permission denied for schema portal` |
| `down` respeita dependências | executar `down` acima e contar tabelas em `portal` | `0` |

## 10. O que este arquivo não decide

| Assunto | Fonte dona | Efeito |
|---|---|---|
| Cadastro só por autocadastro, pending até verificação | `lacunas.md` `A7` (fechada em 2026-11-20) | cadastro e ativação são fluxo de código/portal |
| Troca de e-mail | `lacunas.md` `A21` (fechada em 2026-11-20) | `person_token` de `0004`; este lote só guarda pessoa |
| Envio de e-mail/TTL/provedor | `lacunas.md` `B1` | configuração da frente portal, sem coluna neste lote |
| Login/lockout e sessão | código de autenticação, `A8`/`M4` | tabela oferece os campos e checks básicos |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

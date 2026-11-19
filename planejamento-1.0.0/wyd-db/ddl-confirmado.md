# DDL decidido da série 1.0.0 — o que já pode ser escrito

> **Estado: DDL de planejamento, nada executado.** Nenhum arquivo de `crates/wyd-db/` foi criado ou
> alterado. Nenhum Postgres rodou. Este documento é o **dono único do texto DDL** da série nova;
> a **decisão** continua em `lacunas.md`, e a forma em `padrao-banco.md`.
>
> **O que ele responde:** qual DDL está fechado hoje, palavra por palavra, e por que o resto não está.
> **O que ele não faz:** não decide lacuna aberta, não escreve migration aplicada e não promete
> contagem — os números aqui são de objetos deste documento, não do catálogo.
>
> **Assina:** `padrao-banco.md` (P-01…P-22, forma) + decisões fechadas em `lacunas.md` (`M18-A`,
> `A9`, `A12`, `B4`, `B5`) + `plano-migrations.md` (ordem e nomes de arquivo).

---

## 1. O critério de "100%"

O DDL de um arquivo só conta como **fechado** quando as três condições valem:

1. **Toda tabela do arquivo tem colunas decididas** por documento aprovado — não por "o nome já
   diz". Nome de tabela no plano **não** é DDL.
2. **Nenhuma lacuna aberta pode mudar o DDL** (coluna, constraint, índice). Lacuna que muda só
   semântica de código não bloqueia DDL.
3. **A forma segue o padrão**, não o rascunho: schema, singular, `BIGINT` identity, `TIMESTAMPTZ`,
   constraint nomeada, `INET` para IP, `_fold` para chave derivada.

Resultado medido contra `lacunas.md` em **2026-11-17** — os números abaixo são daquela data e não
foram recontados. A nota de `0005` cita `A17`/`M16`, IDs que só entraram no registro de `lacunas.md`
em **2026-11-18** (§2.2 de lá): as duas já eram decisão aberta em 17, em `plano-migrations.md` §5 e
em `migrations-checklist.md` §3; o que é de 18 é o ID, não a contagem.

| Nível | Quantos | Quais |
|---|---:|---|
| arquivos 100% | **5** | `0001_bootstrap.sql`, `0003_portal_person.sql`, `0004_portal_access.sql`, `0005_game_account.sql`, `0007_game_session.sql` |
| tabelas com DDL fechado dentro de arquivo ainda `parcial` | **2** | `ops.audit_log` e `ops.login_attempt` (2 de 4 em `0002`) |
| tabelas com DDL fechado no total | **15** | 1 de `0001` + 2 de `0002` + 3 de `0003` + 2 de `0004` + 5 de `0005` + 2 de `0007` |
| tabelas da série | 92 | `plano-migrations.md` §3 |

Estado de **documentação** e de **aplicação** não é daqui: esta página só diz se o DDL está fechado.
Quem deriva `planejado`/`provisório`/`pronto` do estado do arquivo é `migrations-checklist.md` §1.2,
dono único. A coluna `Estado` de §2 cita esse checklist arquivo a arquivo, sem escala própria.

**15 de 92** é o número honesto. Os outros 78 não têm coluna desenhada em nenhum documento aprovado —
o plano lhes dá nome, dono e ordem, não forma. Escrever DDL para eles agora seria inventar desenho.

---

## 2. Veredito por arquivo (29)

O estado de cada linha é o de `migrations-checklist.md` §2, citado por arquivo: a derivação é o mapa
de §1.2 daquele documento, e a coluna `Estado` de lá é a dona. Esta página não mantém escala própria —
o que ela decide é o **DDL**: qual seção o fecha e o que ainda falta para fechar.

| # | Arquivo | Estado | DDL nesta página | O que falta para fechar |
|---|---|---|---|---|
| 1 | `0001_bootstrap.sql` | `concluída` | §3 | — |
| 2 | `0002_ops_audit.sql` | `parcial` | — | `config_revision` e `config_change` só têm nome; `audit_log` (§5.3) e `login_attempt` (§5.4) estão fechadas |
| 3 | `0003_portal_person.sql` | `pronta` | §5.1, §5.6, §5.7 | — |
| 4 | `0004_portal_access.sql` | `concluída` | §4 | — |
| 5 | `0005_game_account.sql` | `pronta` | §5.2 | `A17` fechada em 2026-11-19 sem objeto (conta não é transferível); `M16` acrescenta `game.account_ledger`, desenhado em §5.2 — 5 tabelas, arquivo inteiro |
| 6 | `0006_game_character.sql` | `bloqueada` | — | 5 tabelas sem coluna desenhada; os campos vêm do `STRUCT_MOB` e de `A14` (soft-delete × sessão) |
| 7 | `0007_game_session.sql` | `pronta` | §5.5 | depende de `game.character(id) BIGINT`, que é do `0006` ainda `bloqueada` (`H`); o resto fechou (`A10`, chave composta de `server_channel`, fusão `M23`) |
| 8 | `0008_game_skills.sql` | `bloqueada` | — | 5 tabelas sem coluna desenhada |
| 9 | `0009_game_inventory.sql` | `bloqueada` | — | 3 tabelas sem coluna desenhada |
| 10 | `0010_game_guild.sql` | `bloqueada` | — | 4 tabelas sem coluna desenhada |
| 11 | `0011_game_economy.sql` | `bloqueada` | — | 10 tabelas sem coluna desenhada; `A19` decidida: 6 de loja/cash em `portal` (`donate_*`, `daily_reward_*`, `kingdom_cape_price`, `donate_shop_item`) e 4 de jogo em `game`; `M17` acrescenta `idempotency_key` único |
| 12 | `0012_ops_delivery.sql` | `bloqueada` | — | `delivery_queue` existe no legado, mas o desenho novo (idempotência é `M17`) não foi aprovado |
| 13 | `0013_content_taxonomy.sql` | `bloqueada` | — | 5 tabelas + `fn_fold` sem DDL aprovado para o schema `content` |
| 14 | `0014_content_item.sql` | `bloqueada` | — | `content.item_catalog` só tem PK e 2 detalhes definidos; as demais colunas vêm do `STRUCT_MOB` (não reescritas) |
| 15 | `0015_content_skill.sql` | `bloqueada` | — | `skill_catalog` idem |
| 16 | `0016_seed_taxonomy.sql` | `bloqueada` | — | depende do DDL de `0013` e do gerador de seed (adiado, `plano-migrations.md` §1) |
| 17 | `0017_seed_item_catalog.sql` | `bloqueada` | — | gerado; 3.220 itens sem `0014` fechado |
| 18 | `0018_seed_skill_catalog.sql` | `bloqueada` | — | gerado; 151 skills sem `0015` fechado |
| 19 | `0019_content_world.sql` | `bloqueada` | — | 13 tabelas sem coluna desenhada |
| 20 | `0020_seed_world.sql` | `bloqueada` | — | gerado; depende de `0019` |
| 21 | `0021_content_rules.sql` | `bloqueada` | — | 5 tabelas sem coluna desenhada |
| 22 | `0022_content_overrides.sql` | `bloqueada` | — | 3 tabelas sem coluna desenhada |
| 23 | `0023_seed_rules.sql` | `bloqueada` | — | gerado; depende de `0021`/`0022` |
| 24 | `0024_content_maps.sql` | `bloqueada` | — | 2 tabelas sem coluna desenhada |
| 25 | `0025_content_combat.sql` | `bloqueada` | — | 4 tabelas sem coluna desenhada |
| 26 | `0026_content_quests_events.sql` | `bloqueada` | — | 6 tabelas sem coluna desenhada |
| 27 | `0027_seed_quests_events.sql` | `bloqueada` | — | gerado; depende de `0026` |
| 28 | `0028_game_quests_events.sql` | `bloqueada` | — | 7 tabelas sem coluna desenhada |
| 29 | `0029_views.sql` | `bloqueada` | — | as 2 views têm propósito decidido, não lista de colunas |

**Leitura do estado:** o bloqueio dos arquivos 8–29 é o mesmo — **falta desenho de coluna**, não
falta decisão de produto. É trabalho de modelagem, e cada tabela fechada sobe o "15 de 92".

---

## 3. `0001_bootstrap.sql` — DDL fechado

Cria os 4 schemas, as permissões de schema, a função de `updated_at` e a linha de versão do
conteúdo. **Nenhuma tabela de domínio nasce aqui** — é o único arquivo que pode rodar antes de tudo.

```sql
-- 0001_bootstrap: schemas, permissões, função de `updated_at` e versão do conteúdo.
-- Pré-requisito de ambiente: db/roles.sql (papéis do cluster e o estado de `public`) antes desta série.

-- migrate:up
CREATE SCHEMA content AUTHORIZATION wyd_owner;
CREATE SCHEMA game    AUTHORIZATION wyd_owner;
CREATE SCHEMA portal  AUTHORIZATION wyd_owner;
CREATE SCHEMA ops     AUTHORIZATION wyd_owner;

-- `public` não é tocado aqui: schema e privilégio dele são estado de ambiente, emitido por
-- `db/roles.sql` (`src/provision/spec.md` §4). O ledger `schema_migrations` nasce nele antes de 0001.

GRANT USAGE ON SCHEMA content TO wyd_game, wyd_portal, wyd_read;
GRANT USAGE ON SCHEMA game    TO wyd_game, wyd_read;   -- portal lê presença por view
GRANT USAGE ON SCHEMA portal  TO wyd_portal, wyd_read; -- jogo lê identidade por view
GRANT USAGE ON SCHEMA ops     TO wyd_read;
-- Quem só ACRESCENTA auditoria também precisa de `USAGE` no schema: sem ele o `GRANT INSERT` do
-- `0002` falha com `42501 permission denied for schema ops`, e não por falta de `INSERT` na tabela.
GRANT USAGE ON SCHEMA ops     TO wyd_game, wyd_portal;

ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA game
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO wyd_game;
ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA content
    GRANT SELECT ON TABLES TO wyd_game, wyd_portal, wyd_read;
ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA portal
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO wyd_portal;
ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA ops
    GRANT SELECT ON TABLES TO wyd_read;

-- Única função de trigger da série (P-16): manter `updated_at`. Uma função para todas as tabelas.
CREATE FUNCTION ops.fn_set_updated_at() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$;

-- Linha única: a versão do conteúdo é uma só por banco, não uma entidade. `id = 1` é a chave.
CREATE TABLE content.content_version (
    id         SMALLINT NOT NULL,
    version    INTEGER NOT NULL,
    note       TEXT NOT NULL DEFAULT '',
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_content_version PRIMARY KEY (id),
    CONSTRAINT ck_content_version_linha_unica CHECK (id = 1),
    CONSTRAINT ck_content_version_versao CHECK (version >= 1)
);
```

```sql
-- migrate:down
DROP TABLE content.content_version;
DROP FUNCTION ops.fn_set_updated_at();

-- Sem CASCADE de propósito: se sobrar objeto de outro arquivo aqui, o down falha alto em vez de
-- apagar o que não é dele. É o gate de que cada arquivo derruba o próprio `up`.
DROP SCHEMA ops;
DROP SCHEMA portal;
DROP SCHEMA game;
DROP SCHEMA content;

-- Nada de `public` no `down`: o estado dele é do provisionamento, e o ledger sobrevive ao reset.
```

**Quatro detalhes que não são estilo:**

1. `content_version` é herdada da `0007` do legado (`id SMALLINT CHECK (id = 1)`, `version`, `note`,
   `applied_at`). O desenho não foi rediscutido; só ganhou schema, PK nomeada e CHECKs nomeados.
2. `INSERT` em coluna identity **não** exige `GRANT` na sequence — o `SERIAL` exigiria. É a
   justificativa prática de `P-03` além da correção de padrão (medido em PG15/17, out/2026[^1]).
3. O `down` depende da ordem reversa da série (`src/migrate/app/runner.md` §3): quando `0001` cai, os
   outros já derrubaram as próprias tabelas. `DROP SCHEMA` sem `CASCADE` é a prova disso.
4. O estado de `public` **não** está aqui, nem no `up` nem no `down`. Ele é emitido por
   `provision render` (`src/provision/spec.md` §4), porque `public` existe com o banco, o ledger nasce
   nele antes do `0001` e ele pertence a `pg_database_owner` no PG15+: rodando como `wyd_migrator`, a
   revogação de `PUBLIC` viraria aviso sem efeito e a concessão a `wyd_migrator` falharia com `42501`
   por falta de grant option. Decisão de 2026-11-18: `ADR-DB-0001`, item 8.

[^1]: <https://now-next.nl/en/insights/postgresql-permission-denied-for-sequence> — matriz de
    privilégios de sequence em PG 17.11 e 15.19, incluindo o caso de identity.

---

## 4. `0004_portal_access.sql` — DDL fechado

Sessão revogável do portal e os tokens de cerimônia da pessoa. Depende de `portal.person` (§5.1,
inclusive `password_changed_at`) e é o segundo arquivo do bloco de identidade. Só `wyd_portal`
escreve aqui.

Versão vigente desde 2026-11-17, com duas decisões do dono: **M4** (expiração absoluta + idle,
`user_agent` limitado, índice de expiração) e **M5** (`person_password_reset` → `person_token` com
`purpose`). Em 2026-11-18 entrou **A16**: o cookie carrega um token opaco e o banco guarda apenas o
`sha256` dele — não há JWT, `jti` em claro nem segredo de assinatura nesta série. O texto anterior
foi **substituído**, não corrigido — `person_password_reset` não
existe na série 1.0.0.

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

Os índices são derrubados **qualificados** (`portal.ix_…`) porque o runner fecha o `search_path` da
transação, e ele fica vazio (`src/migrate/spec.md` §6): sem o schema, `DROP INDEX` falharia mesmo com o
índice existindo.

**Por que este arquivo fecha:** as duas tabelas dependem de `person` apenas por FK, e nenhuma das
duas muda se o desenho de `person_profile`/`person_consent` mudar. `0003` também está inteiro hoje
(§5.1, §5.6, §5.7) e roda **antes** deste arquivo, porque a FK exige a pessoa existindo.

**O que a decisão `M4` acrescenta e não é coluna:** o timeout idle é `last_seen_at + intervalo`; a
troca de senha executa `revoke_all` na mesma transação que grava `person.password_changed_at`. Sem
os dois, marcar `revoked_at` seria trabalho manual e esquecível.

---

## 5. DDL das tabelas já desenhadas

O DDL abaixo está fechado e é o que entra no arquivo quando o resto dele fechar. A ordem do
documento é a ordem em que cada tabela foi fechada, **não** a ordem de arquivo: §5.1, §5.6 e §5.7
são de `0003`; §5.2 é de `0005`; §5.3 e §5.4 são de `0002`; §5.5 é de `0007`, que fecha inteiro com
aquele bloco. A ordem dos `down`
segue a inversa do `up` dentro do mesmo arquivo.

**Enunciação (2026-11-18):** a **leitura por tabela** de §5.1, §5.2, §5.5, §5.6 e §5.7 — papel,
quem escreve e lê, ciclo de vida, o que o schema garante — está em
`docs/dicionario-dados/dominio-acesso.md` §Tabela de tabelas, §Ciclo de vida do dado e §Regras e
armadilhas; aqui fica o **texto DDL** que será copiado no `.sql`, com nome e predicado de cada
constraint. O canal de §5.5 e os blocos de §5.3/§5.4 esperam página de domínio própria: até ela
existir, esta é a única leitura. O DDL muda aqui e a consequência fecha lá, nunca o contrário.

### 5.1 `portal.person` (vai em `0003_portal_person.sql`)

Junta os requisitos medidos em `rascunho-accounts.md` §2 com `B4` (suspensão em colunas), `A12`
(tombstone), `A9`/`D13` (trigger) e `P-10`/`P-13` (forma).

```sql
CREATE TABLE portal.person (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    email             TEXT NOT NULL,
    -- Dobra do e-mail no banco: " Foo@X.com " e "foo@x.com" são a mesma pessoa.
    email_fold        TEXT GENERATED ALWAYS AS (lower(btrim(email))) STORED,
    password_hash     TEXT NOT NULL,
    -- Papel no PORTAL. O papel de jogo é `game.account.role`; são tipos diferentes no Rust.
    role              TEXT NOT NULL DEFAULT 'member',
    status            TEXT NOT NULL DEFAULT 'active',
    email_verified_at TIMESTAMPTZ,
    failed_login_count SMALLINT NOT NULL DEFAULT 0,
    locked_until      TIMESTAMPTZ,
    -- Instante da última troca de senha do portal; NULL = nunca trocada desde o cadastro (M4).
    password_changed_at TIMESTAMPTZ,
    last_login_at     TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at         TIMESTAMPTZ,
    -- Suspensão é estado de `status`; autor, motivo e instante são anotação da MESMA linha (B4).
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
    -- Estado e carimbo não divergem, nas duas pontas.
    CONSTRAINT ck_person_fechamento_coerente CHECK ((status = 'closed') = (closed_at IS NOT NULL)),
    -- Fechar anonimiza: linha fechada NUNCA guarda o endereço real (A12).
    CONSTRAINT ck_person_fechamento_anonimo CHECK (
        status <> 'closed' OR email = 'closed-' || id || '@removido.invalid'
    ),
    CONSTRAINT ck_person_suspensao_coerente CHECK (
        (status = 'suspended') = (suspended_at IS NOT NULL)
        AND (status = 'suspended' OR (suspended_reason IS NULL AND suspended_by IS NULL))
    ),
    -- Suspensa exige motivo E autor (padrão §5.1, R7). Por isso a FK é RESTRICT, não SET NULL.
    CONSTRAINT ck_person_suspensao_motivo CHECK (
        status <> 'suspended'
        OR (char_length(btrim(suspended_reason)) > 0 AND suspended_by IS NOT NULL)
    )
);

-- Índice TOTAL: não filtra `status` e não tem janela de reuso (A12).
CREATE UNIQUE INDEX ux_person_email ON portal.person (email_fold);
CREATE INDEX ix_person_nao_ativas ON portal.person (status) WHERE status <> 'active';
CREATE INDEX ix_person_suspended_by ON portal.person (suspended_by);

CREATE TRIGGER trg_person_set_updated_at
    BEFORE UPDATE ON portal.person
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();
```

```sql
-- migrate:down (do trecho)
DROP TRIGGER trg_person_set_updated_at ON portal.person;
DROP INDEX ix_person_suspended_by;
DROP INDEX ix_person_nao_ativas;
DROP INDEX ux_person_email;
DROP TABLE portal.person;
```

**O que mudou em relação ao rascunho:** schema `portal`, tabela `person` (não `portal_user`),
`failed_logins` → `failed_login_count` (convenção de contador), `suspended_by` passou de
`ON DELETE SET NULL` para `RESTRICT` + CHECK de autor, e todo nome de constraint segue `P-13`.
`D5` do rascunho (não criar profile) **não** decide `person_profile`: `P-22` manda criar, e a
divergência está registrada no veredito do `0003`.

### 5.2 Bloco `game` da conta (`0005_game_account.sql`)

```sql
CREATE TABLE game.account (
    id                  BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id           BIGINT NOT NULL,
    -- Vaga 1..5: o recurso finito que materializa o limite de contas no banco.
    slot                SMALLINT NOT NULL,
    login               TEXT NOT NULL,
    login_fold          TEXT GENERATED ALWAYS AS (lower(btrim(login))) STORED,
    password_hash       TEXT NOT NULL,
    -- Instante da última troca de senha da conta; NULL = nunca trocada desde a criação (M3).
    password_changed_at TIMESTAMPTZ,
    pin_hash            TEXT,
    pin_failure_count   SMALLINT NOT NULL DEFAULT 0,
    pin_locked_until    TIMESTAMPTZ,
    -- A senha de jogo tem 12 bytes no fio: sem contador, o brute force não tem freio (M3).
    login_failure_count SMALLINT NOT NULL DEFAULT 0,
    login_locked_until  TIMESTAMPTZ,
    role                TEXT NOT NULL DEFAULT 'player',
    status              TEXT NOT NULL DEFAULT 'active',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at       TIMESTAMPTZ,
    closed_at           TIMESTAMPTZ,
    CONSTRAINT pk_account PRIMARY KEY (id),
    -- Pessoa nunca é apagada em operação; RESTRICT é a rede de segurança, não o caminho.
    CONSTRAINT fk_account_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_slot_range CHECK (slot BETWEEN 1 AND 5),
    -- 16 é o tamanho do login no fio (GameLogin); senha e PIN são 12 e 6, impostos na borda.
    CONSTRAINT ck_account_login_len CHECK (char_length(login) BETWEEN 1 AND 16),
    CONSTRAINT ck_account_login_forma CHECK (login = btrim(login) AND char_length(login) > 0),
    CONSTRAINT ck_account_role CHECK (role IN ('player', 'moderator', 'admin')),
    -- Ciclo de vida só: bloqueio disciplinar é `game.account_moderation.blocked` (M1).
    CONSTRAINT ck_account_status CHECK (status IN ('active', 'closed')),
    CONSTRAINT ck_account_senha_hasheada CHECK (password_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_account_pin_hasheado CHECK (pin_hash IS NULL OR pin_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_account_pin_falhas CHECK (pin_failure_count >= 0),
    CONSTRAINT ck_account_login_falhas CHECK (login_failure_count >= 0),
    CONSTRAINT ck_account_senha_trocada CHECK (
        password_changed_at IS NULL OR password_changed_at >= created_at
    ),
    CONSTRAINT ck_account_fechamento_coerente CHECK ((status = 'closed') = (closed_at IS NOT NULL))
);

CREATE UNIQUE INDEX ux_account_login ON game.account (login_fold);
-- 6ª conta falha aqui, inclusive com dois cadastros simultâneos. Conta fechada devolve a vaga.
CREATE UNIQUE INDEX ux_account_slot ON game.account (person_id, slot) WHERE status <> 'closed';
CREATE INDEX ix_account_person ON game.account (person_id);

CREATE TRIGGER trg_account_set_updated_at
    BEFORE UPDATE ON game.account
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.account_wallet (
    account_id BIGINT NOT NULL,
    coin       BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_account_wallet PRIMARY KEY (account_id),
    -- Dinheiro não cascateia (`padrao-banco.md` §11): apagar conta com carteira falha alto.
    CONSTRAINT fk_account_wallet_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    -- `Coin` é int no fio (STRUCT_MOB.Coin@28); o teto do banco é o teto do campo.
    CONSTRAINT ck_account_wallet_coin CHECK (coin >= 0 AND coin <= 2147483647)
);

-- Livro de `coin` (`M16`, decidido em 2026-11-19): toda mudança de carteira deixa uma linha.
-- `balance_after` é o saldo no instante do lançamento; `account_wallet.coin` passa a ser o saldo corrente.
CREATE TABLE game.account_ledger (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id    BIGINT NOT NULL,
    delta         BIGINT NOT NULL,
    balance_after BIGINT NOT NULL,
    reason        TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_account_ledger PRIMARY KEY (id),
    CONSTRAINT fk_account_ledger_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_ledger_delta CHECK (delta <> 0),
    CONSTRAINT ck_account_ledger_balance CHECK (
        balance_after >= 0 AND balance_after <= 2147483647
    ),
    CONSTRAINT ck_account_ledger_reason CHECK (char_length(reason) BETWEEN 1 AND 64)
);

CREATE INDEX ix_account_ledger_conta ON game.account_ledger (account_id, created_at DESC);

CREATE TABLE game.account_moderation (
    account_id BIGINT NOT NULL,
    blocked    BOOLEAN NOT NULL DEFAULT false,
    reason     TEXT NOT NULL DEFAULT '',
    blocked_by BIGINT,
    blocked_at TIMESTAMPTZ,
    CONSTRAINT pk_account_moderation PRIMARY KEY (account_id),
    -- Mesma política dos três filhos de `account`: conta não é apagada, então nada cascateia.
    CONSTRAINT fk_account_moderation_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT fk_account_moderation_autor FOREIGN KEY (blocked_by)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_moderation_motivo CHECK (
        NOT blocked OR char_length(btrim(reason)) > 0
    ),
    CONSTRAINT ck_account_moderation_instante CHECK (
        (blocked AND blocked_at IS NOT NULL) OR (NOT blocked AND blocked_at IS NULL)
    ),
    CONSTRAINT ck_account_moderation_autor CHECK (NOT blocked OR blocked_by IS NOT NULL)
);

CREATE INDEX ix_account_moderation_autor ON game.account_moderation (blocked_by);
CREATE INDEX ix_account_moderation_bloqueadas
    ON game.account_moderation (account_id) WHERE blocked;

CREATE TABLE game.pin_reset (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id   BIGINT NOT NULL,
    token_hash   TEXT NOT NULL,
    -- Origem do pedido. O destinatário NÃO mora aqui: é `portal.person.email`, por join (P-22, M2).
    requested_ip INET,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at   TIMESTAMPTZ NOT NULL,
    used_at      TIMESTAMPTZ,
    CONSTRAINT pk_pin_reset PRIMARY KEY (id),
    -- Mesma política dos três filhos de `account`: conta não é apagada, então nada cascateia.
    CONSTRAINT fk_pin_reset_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT uq_pin_reset_token UNIQUE (token_hash),
    CONSTRAINT ck_pin_reset_token_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_pin_reset_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_pin_reset_consumo CHECK (used_at IS NULL OR used_at >= created_at)
);

CREATE INDEX ix_pin_reset_account ON game.pin_reset (account_id, created_at DESC);
```

```sql
-- migrate:down (do trecho, ordem inversa)
DROP INDEX ix_pin_reset_account;
DROP TABLE game.pin_reset;
DROP INDEX ix_account_moderation_bloqueadas;
DROP INDEX ix_account_moderation_autor;
DROP TABLE game.account_moderation;
DROP INDEX ix_account_ledger_conta;
DROP TABLE game.account_ledger;
DROP TABLE game.account_wallet;
DROP TRIGGER trg_account_set_updated_at ON game.account;
DROP INDEX ix_account_person;
DROP INDEX ux_account_slot;
DROP INDEX ux_account_login;
DROP TABLE game.account;
```

**O que ainda pode mudar neste bloco** (por isso o arquivo não fecha):

| Lacuna | Pergunta | Efeito se a resposta for "sim" |
|---|---|---|
| `A17` | conta pode ser transferida/vendida entre pessoas? | **decidido em 2026-11-19: não.** Nenhuma tabela, coluna ou índice novo; `0005` fecha com as 4 tabelas atuais mais o ledger de `M16` |
| `M16` | `coin` precisa de razão de lançamento? | tabela `game.account_ledger` + índice por conta — arquivo novo, não coluna nesta |

`A17` foi fechada em 2026-11-19: conta não é transferível nem vendável, então nenhuma das quatro
tabelas muda por transferência e não há objeto novo. `M16` acrescenta `game.account_ledger`, desenhado
acima; o `0005` passa a 5 tabelas e fecha inteiro. `A19` (loja/cash) não toca aqui: 6 tabelas ficam em
`portal` e 4 em `game` (`plano-migrations.md` §3).

**Decidido em 2026-11-17:** `M1` (status só `active`/`closed`), `M2` (`pin_reset` sem `delivered_to`,
com `requested_ip`) e `M3` (contadores de falha da senha, `password_changed_at`, `updated_at` com
trigger). Diferença declarada: `game.pin_reset` **não** tem o índice "um vivo por conta" que
`ux_person_token_vivo` dá ao portal; o alinhamento do índice fica para a onda 2, quando `0005` for aplicado.

**Política de exclusão do bloco — decidida em 2026-11-18: `RESTRICT` nas três FKs que apontam para
`game.account`** (`account_wallet`, `account_moderation`, `pin_reset`). Nenhuma cascata aqui, e a
razão é aritmética antes de ser de gosto: a carteira é 1:1 com a conta e **sempre existe**, então
`DELETE FROM game.account` já falhava com `RESTRICT` nela — o `CASCADE` das outras duas nunca
dispararia. Manter uma cascata que não roda é duas políticas para a mesma pergunta, e a segunda é
falsa. Com os três em `RESTRICT` a regra é uma só: **conta não se apaga, fecha** (`closed_at`, o
mesmo critério de `A12` para pessoa).

Reset de ambiente também não passa por `DELETE`, e por isso não precisa de cascata: o caminho é o
`down` da série (`DROP TABLE` dos três filhos antes do pai, como no bloco) ou um `TRUNCATE` listando
as quatro tabelas, que atravessa as FKs de `RESTRICT` sem tocar em política nenhuma. Os dois são
operações declaradas e revisadas; um `CASCADE` implícito num `DELETE` de rotina não seria.

### 5.3 `ops.audit_log` (`0002_ops_audit.sql`)

```sql
CREATE TABLE ops.audit_log (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Ator tripartido (M18-A). Sem FK, por desenho: auditoria sobrevive ao ator.
    actor_kind  TEXT NOT NULL,
    actor_id    BIGINT,
    actor_ref   TEXT,
    action      TEXT NOT NULL,
    -- Herdados do legado (TEXT). Virar JSONB é decisão que ninguém tomou; não entra por gosto.
    target      TEXT NOT NULL DEFAULT '',
    detail      TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_audit_log PRIMARY KEY (id),
    CONSTRAINT ck_audit_log_actor_kind CHECK (actor_kind IN ('person', 'account', 'system')),
    CONSTRAINT ck_audit_log_actor_sistema CHECK ((actor_kind = 'system') = (actor_id IS NULL)),
    CONSTRAINT ck_audit_log_actor_ref CHECK ((actor_kind <> 'system') = (actor_ref IS NULL)),
    CONSTRAINT ck_audit_log_action CHECK (char_length(action) > 0)
);

CREATE INDEX ix_audit_log_occurred ON ops.audit_log (occurred_at DESC);
-- "o que o staff 42 fez?" é igualdade + faixa, nessa ordem: kind, id, depois o tempo.
CREATE INDEX ix_audit_log_actor ON ops.audit_log (actor_kind, actor_id, occurred_at DESC);
```

Append-only por **permissão**, não por trigger: além do `SELECT` de `wyd_read` (default privilege do
`0001`), o arquivo concede `INSERT` a `wyd_game` e `wyd_portal` — e nunca `UPDATE`/`DELETE`.

Os `GRANT` abaixo dependem do `USAGE ON SCHEMA ops` que `0001` concede a `wyd_game` e `wyd_portal`:
`INSERT` na tabela sem `USAGE` no schema responde `42501 permission denied for schema ops`, e não
falta de privilégio na tabela. Sem essa linha do `0001`, a prova de append-only de §7 reprovaria pelo
motivo errado.

```sql
GRANT INSERT ON ops.audit_log TO wyd_game, wyd_portal;
```

```sql
-- migrate:down (do trecho)
REVOKE INSERT ON ops.audit_log FROM wyd_game, wyd_portal;
DROP INDEX ix_audit_log_actor;
DROP INDEX ix_audit_log_occurred;
DROP TABLE ops.audit_log;
```

### 5.4 `ops.login_attempt` (`0002_ops_audit.sql`) — decidida em 2026-11-17 (`M6`)

Trilha de tentativa de autenticação, append-only como o `audit_log`. O identificador entra
**hasheado**: `ops` é lido por `wyd_read`, e e-mail/login em claro ali seriam PII fora de `portal`
(`P-22`). O hash ainda responde a pergunta que importa — "quantas falhas para este identificador
na última hora" — sem permitir listar endereços.

```sql
CREATE TABLE ops.login_attempt (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    occurred_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- De onde veio a tentativa: login do portal (e-mail) ou handshake do cliente 7662 (login).
    surface         TEXT NOT NULL,
    -- sha256 hex do `email_fold` (portal) ou do `login_fold` (jogo); nunca o valor em claro.
    identifier_hash TEXT NOT NULL,
    ip              INET,
    result          TEXT NOT NULL,
    CONSTRAINT pk_login_attempt PRIMARY KEY (id),
    CONSTRAINT ck_login_attempt_surface CHECK (surface IN ('portal', 'game')),
    CONSTRAINT ck_login_attempt_identifier CHECK (identifier_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_login_attempt_result CHECK (
        result IN ('ok', 'bad_secret', 'bad_second_factor', 'blocked', 'locked', 'unknown_identifier')
    )
);

-- "quantas falhas neste identificador na última hora" é igualdade + faixa, nessa ordem.
CREATE INDEX ix_login_attempt_identifier ON ops.login_attempt (identifier_hash, occurred_at DESC);
-- Auditoria de abuso por origem (multibox, credential stuffing).
CREATE INDEX ix_login_attempt_ip ON ops.login_attempt (ip, occurred_at DESC) WHERE ip IS NOT NULL;
-- Retenção/reaper por tempo (M22).
CREATE INDEX ix_login_attempt_occurred ON ops.login_attempt (occurred_at DESC);

GRANT INSERT ON ops.login_attempt TO wyd_game, wyd_portal;
```

```sql
-- migrate:down (do trecho)
REVOKE INSERT ON ops.login_attempt FROM wyd_game, wyd_portal;
DROP INDEX ix_login_attempt_occurred;
DROP INDEX ix_login_attempt_ip;
DROP INDEX ix_login_attempt_identifier;
DROP TABLE ops.login_attempt;
```

`unknown_identifier` é registrado de propósito: é o sinal de enumeração de contas, e o hash de um
endereço que não existe não é dado pessoal de ninguém. O que **não** se registra é o valor do
segredo (senha, PIN ou código) — nem em claro, nem hasheado.

**O que falta no `0002`:** colunas de `ops.config_revision` e `ops.config_change` (overlay de taxa
com revisão e autoria). É desenho, não lacuna de produto — `audit_log` e `login_attempt` já estão
fechados.

### 5.5 Bloco `0007` — `server_channel` e `game.session` (DDL fechado em 2026-11-18; fundido em 2026-11-18 por `M23`)

Três decisões fecharam o arquivo, todas a partir do legado medido.

**`A10` — revisada em 2026-11-19 por `A14`: existe `released_reason`.** Quem libera a sessão é o
**canal**, no disconnect, e o **reaper** (varredura de 60 s, `stale` de 180 s) quando o processo cai
sem liberar. A `A14` decidiu que apagar personagem derruba a sessão **e** registra o motivo; por isso
`game.session` ganha `released_reason`, com CHECK fechado (`disconnect`, `reaper`,
`character_deleted`, `admin`) e exigência de preenchimento quando `released_at` existe. O evento
continua em `ops.audit_log` (`actor_kind = 'system'`, `action = 'session_released'`): a coluna é o
estado operacional da sessão; o log é a trilha auditável. As duas fontes respondem perguntas
diferentes e não se substituem.

**Chave de `game.server_channel` — composta `(group_id, channel)`.** É a topologia real do jogo, não
uma escolha de modelagem: grupo é a instalação do servidor, `channel` 1–10 são os canais jogáveis do
`8281` e `channel = 0` é a linha de status do `serv00.htm` (legado `0029`, `0040`, `0041`, e
`docs/glossario.md`). **Um processo por canal** é o modelo de execução do servidor, não escolha de
modelagem — por isso `game.session` carrega o par e a FK composta, em vez de um id sintético que não
significaria nada para o cliente 7662.

**`M23` — `channel_capacity` e `channel_role` fundidas em `server_channel`.** As duas tabelas eram 1:1
com a topologia (mesma PK, mesma FK, mesma linha), não tinham ciclo de vida próprio e existiam por
**uma razão só**: o legado proibia `ALTER TABLE` no `up`, então `max_players`/`maintenance` (`0040`) e
`role` (`0041`) não podiam nascer na tabela original. Os dois arquivos dizem isso literalmente
("Tabela própria em vez de `ALTER TABLE server_channel`"). Numa série criada do zero o motivo não
existe: os três campos entram no `CREATE TABLE` e o schema perde duas tabelas e duas FKs — nenhuma
das duas tinha trigger. O custo declarado é o `channel = 0` (linha de status do `serv00.htm`), que não
tem capacidade nem papel — resolvido com colunas anuláveis e `ck_server_channel_jogavel`, sem `DEFAULT`
(default nessas colunas contradiria a linha de status).

**Dependência declarada (`H`).** A FK `fk_session_character` fixa a forma de `game.character`:
`id BIGINT` como PK. O `0006` segue `bloqueada` (§2), então isso é assunção até ele fechar — se a PK de
`game.character` mudar, o `0007` muda junto.

```sql
-- 0007_game_session: topologia de canal e sessão viva do jogador.
-- Depende de `game.account` (0005) e `game.character` (0006).

-- migrate:up
CREATE TABLE game.server_channel (
    group_id    SMALLINT NOT NULL,
    channel     SMALLINT NOT NULL,
    host        TEXT NOT NULL,
    status_url  TEXT NOT NULL DEFAULT '',
    enabled     BOOLEAN NOT NULL DEFAULT true,
    -- Operação do canal, fundida do legado 0040/0041 (`M23`): 1 linha por canal, não 3 tabelas 1:1.
    -- 1000 = `MAX_USER` do legado (0040); o número é dado de operação, não constante de código.
    -- Sem DEFAULT: a linha de status (`channel = 0`) precisa de NULL nos três (ver CHECK abaixo).
    max_players INTEGER,
    maintenance BOOLEAN,
    role        TEXT,
    note        TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_server_channel PRIMARY KEY (group_id, channel),
    -- Grupo é a instalação; canal 0 é a linha de status, 1–10 são canais jogáveis (legado 0029).
    CONSTRAINT ck_server_channel_grupo CHECK (group_id BETWEEN 0 AND 9),
    CONSTRAINT ck_server_channel_canal CHECK (channel BETWEEN 0 AND 10),
    CONSTRAINT ck_server_channel_host CHECK (host <> ''),
    CONSTRAINT ck_server_channel_status_url CHECK (char_length(status_url) <= 32),
    -- Linha de status não tem capacidade nem papel; canal jogável tem os três, sempre.
    CONSTRAINT ck_server_channel_jogavel CHECK (
        (channel = 0 AND max_players IS NULL AND maintenance IS NULL AND role IS NULL)
     OR (channel > 0 AND max_players IS NOT NULL AND maintenance IS NOT NULL AND role IS NOT NULL)
    ),
    CONSTRAINT ck_server_channel_max CHECK (max_players >= 0),
    -- Dois papéis de canal no jogo; `verde` é o canal de novato, sem PvP (legado 0041).
    CONSTRAINT ck_server_channel_role CHECK (role IN ('normal', 'verde'))
);

CREATE TRIGGER trg_server_channel_set_updated_at
    BEFORE UPDATE ON game.server_channel
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.session (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id     BIGINT NOT NULL,
    -- NULL até a seleção de personagem; a sessão nasce no handshake, antes da escolha.
    character_id   BIGINT,
    group_id       SMALLINT NOT NULL,
    channel        SMALLINT NOT NULL,
    -- Inteiro do handshake (versão do cliente 7662). Não é texto: não há semver no fio.
    client_version INTEGER NOT NULL,
    -- Auditoria de multibox (`B5`): IP conta, nunca identifica (`P-10`).
    ip             INET,
    opened_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    renewed_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    released_at    TIMESTAMPTZ,
    -- Motivo operacional da liberação (`A14`, 2026-11-19); o evento auditável continua em `ops.audit_log`.
    released_reason TEXT,
    CONSTRAINT pk_session PRIMARY KEY (id),
    -- Conta não é apagada (fecha com `closed_at`): RESTRICT é rede de segurança, não caminho.
    CONSTRAINT fk_session_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    -- Personagem é soft-delete (`A14`); a linha nunca some, então a FK não cascateia.
    CONSTRAINT fk_session_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT,
    -- FK composta: a sessão só existe num canal que existe na topologia.
    CONSTRAINT fk_session_canal FOREIGN KEY (group_id, channel)
        REFERENCES game.server_channel (group_id, channel) ON DELETE RESTRICT,
    CONSTRAINT ck_session_canal CHECK (channel BETWEEN 1 AND 10),
    CONSTRAINT ck_session_client_version CHECK (client_version > 0),
    -- Sinal de vida nunca antes da abertura, liberação nunca antes da abertura.
    CONSTRAINT ck_session_sinal CHECK (renewed_at >= opened_at),
    CONSTRAINT ck_session_liberacao CHECK (released_at >= opened_at),
    -- Liberou, tem motivo; viva, não tem. Os quatro motivos são os dois processos e as duas ações.
    CONSTRAINT ck_session_motivo CHECK (
        (released_at IS NULL AND released_reason IS NULL)
     OR (released_at IS NOT NULL AND released_reason IN ('disconnect', 'reaper', 'character_deleted', 'admin'))
    )
);

-- Uma sessão viva por conta: a 2ª conexão da mesma conta falha aqui (nunca por código só).
CREATE UNIQUE INDEX ux_session_viva_por_conta ON game.session (account_id)
    WHERE released_at IS NULL;
-- Uma sessão viva por personagem, quando já houver seleção.
CREATE UNIQUE INDEX ux_session_viva_por_personagem ON game.session (character_id)
    WHERE released_at IS NULL AND character_id IS NOT NULL;
-- Ocupação por canal (status/metrics).
CREATE INDEX ix_session_canal ON game.session (group_id, channel, opened_at DESC);
-- Reaper (60 s, stale 180 s): varre por `renewed_at`, não por `opened_at`.
CREATE INDEX ix_session_reaper ON game.session (renewed_at) WHERE released_at IS NULL;
-- Multibox por IP/hora (`B5`): auditoria, nunca recusa.
CREATE INDEX ix_session_ip ON game.session (ip, opened_at DESC) WHERE ip IS NOT NULL;
```

```sql
-- migrate:down (ordem inversa): índice e trigger caem com a tabela.
DROP TABLE game.session;
DROP TABLE game.server_channel;
```

**Seed: `game.server_channel` não é semeada pela série.** Grupo, `host` e `status_url` são dado de
**instalação**, não conteúdo: cada ambiente tem os seus, e dois ambientes com o mesmo `host` na
migration é mentira sobre o segundo. O seed do legado (`0029`) grava `192.168.100.3`/`.4` dentro do
`up`, e `AGENTS.md` §7 já proíbe versionar `serverlist.bin` com IP privado — o mesmo argumento vale
para uma migration. As linhas nascem de operação — mesmo caminho de `db/roles.sql`, aplicado por
ambiente — e `0007` não as cria.
Consequência direta no `down`: não há `DELETE` de canal porque não há `INSERT` de canal.

```sql
-- fixture do corte vertical (§3.2 do plano-migrations) — teste, não série.
-- Os três campos de operação são explícitos porque as colunas não têm DEFAULT;
-- `channel = 0` é a linha de status e leva NULL nos três (ck_server_channel_jogavel).
INSERT INTO game.server_channel (group_id, channel, host, status_url, max_players, maintenance, role)
VALUES (1, 0, '127.0.0.1', 'http://127.0.0.1/serv00.htm', NULL,  NULL,  NULL),
       (1, 1, '127.0.0.1', '',                        1000,  false, 'normal'),
       (1, 2, '127.0.0.1', '',                        1000,  false, 'verde');
```

**Contrato do port de capacidade — mudou, e não é só de schema.** No legado, **ausência de linha**
em `channel_capacity` significava permissivo (`ChannelCapacity::permissive()`, 1000 e sem
manutenção — `docs/rust/03-portas-e-contratos.md`). Fundida a tabela, canal válido **sempre** tem
linha (PK própria, mais a FK de `game.session`), e `NULL` só existe no `channel = 0`. O contrato
passa a ser: (1) linha ausente = canal inexistente, **erro**, nunca permissivo; (2) `max_players`
nulo = linha de status, que conexão de jogador não consulta; (3) a leitura por `(group_id, channel)`
devolve a linha como está — nada de `COALESCE`/default em runtime, porque 1000 é valor de instalação,
não constante do código. Quem escrever o port (`crates/wyd-db`) implementa essas três regras, não a
anterior.

**O que a decisão custa, declarado:** `released_reason` existe e é obrigatório quando `released_at`
está preenchido; o motivo fica na própria sessão e o evento continua no `audit_log`. Quem não
consultar o log perde o autor do evento, mas não o motivo operacional. A FK composta obriga
todo `INSERT` em `game.session` a trazer `group_id` e `channel` de um canal cadastrado, o que é o
trabalho desejado: canal novo entra em `server_channel`, nunca por texto solto na sessão. E a fusão
exige que a linha de status (`channel = 0`) entre com `NULL, NULL, NULL` explícito nos três campos de
operação — um `INSERT` de instalação (ou da fixture acima), nunca um caminho de runtime.

---

### 5.6 `portal.person_profile` (`0003_portal_person.sql`) — decidida em 2026-11-17 (`M7`)

Registro opcional de exibição, **sem** ser identidade: `person` continua dona de e-mail, papel e
senha. Uma linha por pessoa, criada quando houver dado; ausência de linha é "sem perfil", nunca
erro (`D5` respeitado: nada obrigatório).

```sql
CREATE TABLE portal.person_profile (
    person_id    BIGINT NOT NULL,
    display_name TEXT,
    birth_date   DATE,
    -- ISO 3166-1 alpha-2. `TEXT` + CHECK: por `padrao-banco.md` §4 o limite é CHECK, nunca tipo fixo.
    country      TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_person_profile PRIMARY KEY (person_id),
    CONSTRAINT fk_person_profile_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT ck_person_profile_display_name CHECK (
        display_name IS NULL OR char_length(btrim(display_name)) BETWEEN 2 AND 32
    ),
    -- Só o piso. Data no futuro é recusada na borda: `CURRENT_DATE` não é IMMUTABLE e não entra em CHECK.
    CONSTRAINT ck_person_profile_nascimento CHECK (
        birth_date IS NULL OR birth_date >= DATE '1900-01-01'
    ),
    CONSTRAINT ck_person_profile_pais CHECK (country IS NULL OR country ~ '^[A-Z]{2}$')
);

CREATE TRIGGER trg_person_profile_set_updated_at
    BEFORE UPDATE ON portal.person_profile
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();
```

```sql
-- migrate:down (do trecho)
DROP TRIGGER trg_person_profile_set_updated_at ON portal.person_profile;
DROP TABLE portal.person_profile;
```

### 5.7 `portal.person_consent` (`0003_portal_person.sql`) — decidida em 2026-11-17 (`M7`)

Prova de consentimento **com versão** do documento aceito. "Tem consentimento" sem versão não prova
nada em LGPD/GDPR: o que se guarda é qual texto o titular aceitou e quando. Revogar é preencher
`revoked_at`; o histórico fica nas linhas revogadas, nunca em tabela separada.

```sql
CREATE TABLE portal.person_consent (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id  BIGINT NOT NULL,
    kind       TEXT NOT NULL,
    -- Versão do documento aceito (ex.: `2026-11-01`). Texto curto e versionado, não hash.
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

-- Um consentimento vivo por tipo: aceitar a versão nova revoga a anterior na mesma transação.
CREATE UNIQUE INDEX ux_person_consent_vivo ON portal.person_consent (person_id, kind)
    WHERE revoked_at IS NULL;
CREATE INDEX ix_person_consent_pessoa ON portal.person_consent (person_id, granted_at DESC);
```

```sql
-- migrate:down (do trecho)
DROP INDEX ix_person_consent_pessoa;
DROP INDEX ux_person_consent_vivo;
DROP TABLE portal.person_consent;
```

**Por que `P-22` venceu o `D5`:** o `D5` dizia "sem consumidor medido, não cria profile". O que ele
não via é que sem essas duas tabelas não há onde guardar nome de exibição nem provar base legal —
ambos iriam para dentro de `person` (identidade) ou para fora do banco. As duas nascem mínimas:
nenhuma coluna obrigatória além da chave, e o preenchimento é da onda dona da tela (`M21`, `EXT`).

### 5.8 Decisões de comportamento e retenção (2026-11-19)

Estas não mudam DDL, mas fecham lacunas e viram teste de comportamento:

- **`A13`** — fechar pessoa com conta ativa fecha tudo na mesma transação; não há recusa.
- **`A18`** — conta nasce só no portal; não existe rota de cadastro no cliente.
- **`B1`** — envio real de e-mail na `1.0.0`, provedor escolhido na onda 6, 3 reenvios/h, TTL e
  templates na mesma onda.
- **`M19`** — seed de desenvolvimento usa senha fixa `dev-only`, documentada no próprio seed.
- **`M22`** — retenção: `portal.person_session` 30 dias após `expires_at`; `game.session` 365 dias
  após `released_at`; `ops.audit_log` 730 dias, append-only.

## 6. Adaptações obrigatórias em relação ao rascunho

Onde o rascunho e o padrão divergem, **vale o padrão**. Sem esta tabela, quem copiar o rascunho
escreve a série errada.

| Item | Rascunho | Série 1.0.0 | Fonte |
|---|---|---|---|
| schema | `public` | `content`/`game`/`portal`/`ops` | `P-01` |
| nome de tabela | `portal_user`, `characters`, `guilds` | `person`, `character`, `guild` | `P-02` |
| coluna de FK | `portal_user_id`, `game_account_id` | `person_id`, `account_id` | §3 do padrão |
| PK | `BIGSERIAL PRIMARY KEY` | `BIGINT GENERATED ALWAYS AS IDENTITY` + `pk_<tabela>` | `P-03` |
| nome de constraint | `portal_user_email_len` | `ck_person_email_len`, `fk_…`, `uq_…` | `P-13` |
| IP | `TEXT` (rascunho `D6`) | `INET` | `P-10` |
| suspensão sem autor | permitida pelo CHECK | proibida: motivo **e** autor; FK `RESTRICT` | padrão §5.1, R7 |
| `updated_at` | trigger próprio por tabela | `ops.fn_set_updated_at()` única | `A9`, `P-16` |
| e-mail de pessoa fechada | já anonimizado | mantido, CHECK nomeado | `A12` |
| auditoria | `actor TEXT` (`portal:42`) | `actor_kind` + `actor_id` + `actor_ref` | `M18-A` |

**Consequência operacional declarada:** `INET` e nome qualificado são exatamente o que
`banco-parse`/`check-migrations` ainda não leem (`plano-migrations.md` §1). Com o PR de ferramentas
adiado, esses gates **não** validam esta série — qualquer contagem deles sobre a série nova é
falsa, não é regressão.

---

## 7. Como provar este DDL (quando a série for escrita)

Nada disso foi executado; é o roteiro mínimo, em PG17 efêmero, sem tocar a série viva.

```bash
# 1. papéis do cluster: fora da série, por `provision render` + `provision apply`
wyd-db provision render
WYD_PROVISION_DSN="$ADMIN_DSN" wyd-db provision apply
# 2. a série, na branch `serie-1.0.0`
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0001_bootstrap.sql
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0003_portal_person.sql
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0004_portal_access.sql
```

| Prova | Consulta | Esperado |
|---|---|---|
| 4 schemas + `public` | `\dn` | `content`, `game`, `portal`, `ops`, `public` |
| tabela por schema | `SELECT table_schema, count(*) FROM information_schema.tables WHERE table_schema IN ('content','game','portal','ops') GROUP BY 1` | `content 1`, `portal 5` (3 de `0003` + 2 de `0004`) |
| fronteira: portal não escreve em `game` | `SET ROLE wyd_portal; INSERT INTO game.account (person_id, slot, login, password_hash) VALUES (1, 1, 'x', 'x');` | `permission denied for schema game` |
| fronteira: jogo não lê pessoa crua | `SET ROLE wyd_game; SELECT * FROM portal.person;` | `permission denied for schema portal` |
| append-only: escreve | `SET ROLE wyd_game; INSERT INTO ops.audit_log (actor_kind, action) VALUES ('system', 'prova');` | `INSERT 0 1` — é a prova do `USAGE` de `ops` + do `GRANT INSERT` |
| append-only: não edita nem apaga | `SET ROLE wyd_game; UPDATE ops.audit_log SET action = 'x';` e `DELETE FROM ops.audit_log;` | `42501 permission denied for table audit_log` nos dois (não `for schema ops`) |
| `updated_at` anda | `UPDATE portal.person SET role = 'member' WHERE id = 1;` e reler `updated_at` | valor maior que o `created_at` |
| vaga | duas `INSERT` de `slot = 1` na mesma pessoa | `23505` em `ux_account_slot` |

O ciclo `up → down → up` só passa a valer como gate quando `0001`–`0007` existirem inteiros; com a
série parcial, o teste de "0 tabela ao fim do `down`" não tem o que medir.

---

## 8. O que este documento não faz

1. **Decididas em 2026-11-19:** `A17` não cria objeto (conta não é transferível) e `M16` desenha o
   ledger `game.account_ledger` em §5.2; o que resta em `0005` é execução, não decisão.
2. **Não escreve** arquivo em `crates/wyd-db/`: a série nova é escrita na branch `serie-1.0.0`, em
   `migrations/`, e continua não autorizada a substituir a herdada (`plano-migrations.md` §6).
3. **Não altera** `tools/`: o PR de ferramentas segue adiado por decisão do dono.
4. **Não mede** catálogo: as 15 tabelas daqui são contagem de documento, não de `information_schema`.
5. **Não fecha** `0003`/`0005` por ter as tabelas com DDL fechado: arquivo é unidade de aplicação; meio
   arquivo não roda.

**Próximo passo de desenho, por ordem de retorno:** `0002` (`config_revision`/`config_change`) e
`0006` (`character` e os quatro satélites, com `A14` na semântica do delete).
`0001`, `0003`, `0004`, `0005` e `0007` estão fechados: **13 das 92** tabelas, e é o começo do corte vertical
de `plano-migrations.md` §3.2. `server_channel` **não** é pendência de desenho: `M23` fundiu
`channel_capacity` e `channel_role` na própria linha do canal (§5.5), e a FK de `game.session` só
espera `game.character(id)` — `H` do `0006`, não deste arquivo. Fechados os três restantes, o corte
tem schema completo e deixa de ser papel.

---

*Planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado ou alterado.*

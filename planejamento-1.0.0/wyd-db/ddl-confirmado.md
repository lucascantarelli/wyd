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

Resultado medido contra `lacunas.md` em **2026-11-17**, recontado em **2026-11-20**, quando os lotes
de `0002` e `0006` foram aprovados, e fechado em **2026-11-21**, quando o dono aprovou `0022`, `0025`,
`0026`, `0028` e `0029`. A nota de `0005` cita `A17`/`M16`, IDs que só entraram no registro
de `lacunas.md` em **2026-11-18** (§2.2 de lá): as duas já eram decisão aberta em 17, em
`plano-migrations.md` §5 e em `migrations-checklist.md` §3; o que é de 18 é o ID, não a contagem.

| Nível | Quantos | Quais |
|---|---:|---|
| arquivos 100% | **25** | `0001_bootstrap.sql`, `0002_ops_audit.sql`, `0003_portal_person.sql`, `0004_portal_access.sql`, `0005_game_account.sql`, `0006_game_character.sql`, `0007_game_session.sql`, `0008_game_skills.sql`, `0009_game_inventory.sql`, `0010_game_guild.sql`, `0011_game_economy.sql`, `0012_ops_delivery.sql`, `0013_content_taxonomy.sql`, `0014_content_item.sql`, `0015_content_skill.sql`, `0019_content_world.sql`, `0021_content_rules.sql`, `0022_content_overrides.sql`, `0024_content_maps.sql`, `0025_content_combat.sql`, `0026_content_quests_events.sql`, `0028_game_quests_events.sql`, `0029_views.sql`, `0030_portal_content.sql`, `0031_portal_mfa.sql` |
| tabelas com DDL fechado no total | **97** | 1 de `0001` + 4 de `0002` + 3 de `0003` + 2 de `0004` + 5 de `0005` + 5 de `0006` + 2 de `0007` + 5 de `0008` + 3 de `0009` + 4 de `0010` + 10 de `0011` + 1 de `0012` + 5 de `0013` + 1 de `0014` + 1 de `0015` + 13 de `0019` + 5 de `0021` + 3 de `0022` + 2 de `0024` + 4 de `0025` + 6 de `0026` + 7 de `0028` + 4 de `0030` + 1 de `0031` |
| tabelas desenhadas e **sem aprovação** | **0** | as 20 de §5.23, §5.25, §5.26 e §5.28 foram aprovadas em 2026-11-21; o `0029` é view e não conta |
| tabelas da série | 97 | `plano-migrations.md` §3 (92 originais + 4 do adendo `0030` + 1 do adendo `0031`) |

Estado de **documentação** e de **aplicação** não é daqui: esta página só diz se o DDL está fechado.
Quem deriva `planejado`/`provisório`/`pronto` do estado do arquivo é `migrations-checklist.md` §1.2,
dono único. A coluna `Estado` de §2 cita esse checklist arquivo a arquivo, sem escala própria.

**97 de 97** têm DDL fechado. Os 25 arquivos vão de `0001` a `0024` mais os lotes `0025`, `0026`,
`0028` e `0029`, e os adendos `0030` e `0031`, aprovados pelo dono em 2026-11-20 e 2026-11-21; a
última rodada fechou as 20 tabelas que estavam desenhadas sem aprovação (§5.23, §5.25, §5.26 e
§5.28). Nenhuma tabela da série fica sem coluna desenhada e nenhuma espera o "sim" do dono.
O que resta `bloqueada` são os seis arquivos de seed, presos na verificação do dado de origem — não no
desenho e não em ferramenta (`migrations-checklist.md` §3).

O adendo `0030` (§5.15) existe porque o `wyd-portal` decidiu, em 2026-11-20, quais tabelas o CMS e o
launcher precisam. O **conjunto** é decisão de 2026-11-20; o **desenho de coluna** foi aprovado pelo
dono na mesma data, e por isso a série passou a 30 arquivos / 96 tabelas naquela data e as quatro tabelas contam; com o adendo `0031` (§5.20) ela foi a 31 arquivos e 97 tabelas, e o `0032` (§5.32, `M27`) a levou a **32 arquivos e 97 tabelas**.

Os lotes de `0009`, `0010` e `0011` fecharam em 2026-11-20 (§5.12, §5.13 e §5.14), e o `0012`
(`ops.delivery_queue`) fechou na mesma data (§5.16), junto com `M25` — dreno com `SKIP LOCKED` e
expurgo de 30 dias. O `0013` (`content.taxonomy`, cinco tabelas e `fn_fold`) fechou na mesma data
(§5.17). O `0014` (`content.item_catalog`) fechou na mesma data (§5.18) e o `0015` (`content.skill_catalog`)
também (§5.19). Os seeds `0016`–`0018` seguem `bloqueada` pela verificação do dado de origem, e o
`0019` (`content.world`, 13 tabelas) fechou na mesma data (§5.21).

Os lotes de `0002`, `0006` e `0008` foram aprovados em 2026-11-20 e as quatorze tabelas deles entram na
conta (§5.9, §5.10 e §5.11). Os cinco últimos lotes (§5.23, §5.25, §5.26, §5.28 e §5.29) foram
aprovados em 2026-11-21, com as escolhas de forma registradas no fim de cada seção.

---

## 2. Veredito por arquivo (32)

O estado de cada linha é o de `migrations-checklist.md` §2, citado por arquivo: a derivação é o mapa
de §1.2 daquele documento, e a coluna `Estado` de lá é a dona. Esta página não mantém escala própria —
o que ela decide é o **DDL**: qual seção o fecha e o que ainda falta para fechar.

| # | Arquivo | Estado | DDL nesta página | O que falta para fechar |
|---|---|---|---|---|
| 1 | `0001_bootstrap.sql` | `concluída` | §3 | — |
| 2 | `0002_ops_audit.sql` | `concluída` | §5.3, §5.4, §5.9 | — (aprovado em 2026-11-20; as quatro tabelas contam) |
| 3 | `0003_portal_person.sql` | `concluída` | §5.1, §5.6, §5.7 | — (página escrita em 2026-11-20) |
| 4 | `0004_portal_access.sql` | `concluída` | §4 | — |
| 5 | `0005_game_account.sql` | `concluída` | §5.2 | — (página escrita em 2026-11-20; `A17` sem objeto, `M16` no ledger) |
| 6 | `0006_game_character.sql` | `concluída` | §5.10 | — (aprovado e página escrita em 2026-11-20) |
| 7 | `0007_game_session.sql` | `concluída` | §5.5 | — (página escrita em 2026-11-20; `A10`/`A14`, chave composta e fusão `M23`) |
| 8 | `0008_game_skills.sql` | `concluída` | §5.11 | — (aprovado e página escrita em 2026-11-20) |
| 9 | `0009_game_inventory.sql` | `concluída` | §5.12 | — (aprovado e página escrita em 2026-11-20; as três tabelas contam) |
| 10 | `0010_game_guild.sql` | `concluída` | §5.13 | — (aprovado e página escrita em 2026-11-20; as quatro tabelas contam) |
| 11 | `0011_game_economy.sql` | `concluída` | §5.14 | — (aprovado e página escrita em 2026-11-20; as dez tabelas contam) |
| 12 | `0012_ops_delivery.sql` | `concluída` | §5.16 | — (aprovado e página escrita em 2026-11-20; `M25` fechada na mesma rodada) |
| 13 | `0013_content_taxonomy.sql` | `concluída` | §5.17 | — (aprovado em 2026-11-20; página `0013-content-taxonomy.md` escrita e conferida) |
| 14 | `0014_content_item.sql` | `concluída` | §5.18 | — (aprovado em 2026-11-20; página `0014-content-item.md` escrita e conferida) |
| 15 | `0015_content_skill.sql` | `concluída` | §5.19 | — (aprovado em 2026-11-20; página `0015-content-skill.md` escrita e conferida) |
| 16 | `0016_seed_taxonomy.sql` | `bloqueada` | — | o DDL de `0013` fechou; falta verificar os 20 sistemas, 6 classes, 13 linhagens e 16 categorias na fonte (`plano-migrations.md` §4) |
| 17 | `0017_seed_item_catalog.sql` | `bloqueada` | — | o DDL de `0014` fechou; faltam as contagens de 2026-11-08/15 recontadas na fonte, antes de escrever as linhas |
| 18 | `0018_seed_skill_catalog.sql` | `bloqueada` | — | o DDL de `0015` fechou; falta a curadoria de classe e linhagem das 154 skills, que é dado, não DDL |
| 19 | `0019_content_world.sql` | `concluída` | §5.21 | — (aprovado em 2026-11-20; página `0019-content-world.md` escrita e conferida) |
| 20 | `0020_seed_world.sql` | `bloqueada` | — | o DDL de `0019` fechou; faltam as contagens de templates, NPCs, lojas e regiões verificadas na fonte |
| 21 | `0021_content_rules.sql` | `concluída` | §5.22 | — (aprovado em 2026-11-20; página `0021-content-rules.md` escrita e conferida) |
| 22 | `0022_content_overrides.sql` | `concluída` | §5.23 | — (aprovado em 2026-11-21; página `0022-content-overrides.md` escrita e conferida) |
| 23 | `0023_seed_rules.sql` | `bloqueada` | — | o DDL de `0021` e o de `0022` fecharam; faltam os valores de refino, gemas, SANC e composição verificados na fonte |
| 24 | `0024_content_maps.sql` | `concluída` | §5.24 | — (aprovado em 2026-11-21; página escrita e conferida) |
| 25 | `0025_content_combat.sql` | `concluída` | §5.25 | — (aprovado em 2026-11-21, com `RESTRICT` no item e teto `1.000.000` em `rate_bps`; página escrita e conferida) |
| 26 | `0026_content_quests_events.sql` | `concluída` | §5.26 | — (aprovado em 2026-11-21, com `start_minute` e `id` identity na recompensa; página escrita e conferida) |
| 27 | `0027_seed_quests_events.sql` | `bloqueada` | — | o DDL de `0026` fechou; faltam a cadeia de quests do 7662 e o calendário de eventos verificados (`M19` segue proposta) |
| 28 | `0028_game_quests_events.sql` | `concluída` | §5.28 | — (aprovado em 2026-11-21; página `0028-game-quests-events.md` escrita e conferida) |
| 29 | `0029_views.sql` | `concluída` | §5.29 | — (aprovado em 2026-11-21, sem `GRANT` novo; página `0029-views.md` escrita e conferida) |
| 30 | `0030_portal_content.sql` (adendo) | `concluída` | §5.15 | — (aprovado em 2026-11-20; página `0030-portal-content.md` escrita e conferida) |
| 31 | `0031_portal_mfa.sql` (adendo) | `concluída` | §5.20 | — (aprovado em 2026-11-20; página `0031-portal-mfa.md` escrita e conferida) |
| 32 | `0032_admin_grants.sql` | `concluída` | §5.32 | — (decidido em 2026-11-21 em `M27`: só `GRANT`, sem tabela; página `0032-admin-grants.md` escrita e conferida) |

**Leitura do estado:** nenhuma tabela da série fica sem desenho e nenhum lote espera aprovação. O que
resta `bloqueada` são os seis arquivos de seed (`0016`, `0017`, `0018`, `0020`, `0023` e `0027`), presos
na verificação do dado de origem — não no gerador: os arquivos são escritos à mão quando o dado for
conferido (`plano-migrations.md` §4).

A linha 30 é **adendo**, não parte dos 29 originais: o arquivo `0030_portal_content.sql` foi proposto
em 2026-11-20 para as quatro tabelas de conteúdo do portal e aprovado pelo dono na mesma data. A linha
32, decidida em 2026-11-21 (`M27`), é o único arquivo sem tabela: concede privilégio ao papel
`wyd_admin`, que o provision cria. A soma de §1 fica em **32 arquivos e 97 tabelas**.

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

### 5.9 Lote `0002` — `config_revision` e `config_change` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Aprovar com as mudanças
    que marquei nas perguntas acima"**. As quatro escolhas marcadas eram exatamente as quatro formas
    já escritas nesta proposta; como não sobrou mudança a aplicar, a condição da aprovação ficou
    satisfeita e o texto abaixo é o DDL confirmado, contado em §1.

Desenho coerente com o que já está decidido: `ops` é o schema do que a operação fez; a configuração é
editada pela área administrativa do portal (decisão de escopo de `M21`, 2026-11-20); o autor usa o ator
tripartido de `M18-A`, porque `portal.person` nasce em `0003` e FK para arquivo posterior é proibida
(`padrao-banco.md` §9, item 8); e nenhum valor é `JSONB`, porque estado de jogo é coluna nomeada
(decisão de 2026-11-20).

```sql
CREATE TABLE ops.config_revision (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    -- Quem editou. Mesmo ator tripartido de `audit_log` (M18-A), sem FK: `portal.person` nasce em
    -- `0003`, arquivo posterior — FK para frente é proibida (`padrao-banco.md` §9, item 8).
    author_kind      TEXT NOT NULL,
    author_id        BIGINT,
    author_ref       TEXT,
    -- Por que a configuração mudou. Texto curto de operação, nunca dado de jogo.
    note             TEXT NOT NULL DEFAULT '',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Instante em que a revisão passou a valer. Uma revisão nasce aplicada: não há rascunho na 1.0.0.
    applied_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_config_revision PRIMARY KEY (id),
    CONSTRAINT ck_config_revision_autor CHECK (
        (author_kind = 'person' AND author_id IS NOT NULL AND author_ref IS NULL)
        OR (author_kind = 'system' AND author_id IS NULL AND author_ref IS NOT NULL)
    ),
    CONSTRAINT ck_config_revision_nota CHECK (char_length(note) <= 500),
    CONSTRAINT ck_config_revision_aplicacao CHECK (applied_at >= created_at)
);

CREATE INDEX ix_config_revision_aplicada ON ops.config_revision (applied_at DESC);
CREATE INDEX ix_config_revision_autor ON ops.config_revision (author_kind, author_id, applied_at DESC);

CREATE TABLE ops.config_change (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    revision_id   BIGINT NOT NULL,
    -- Onde a mudança incide: `schema.tabela` + a chave da linha.
    target_schema TEXT NOT NULL,
    target_table  TEXT NOT NULL,
    target_key    TEXT NOT NULL,
    -- Valor antes e depois, em texto canônico do tipo da coluna alvo. Sem JSONB para estado de jogo.
    -- `old_value` nulo = a chave não existia antes; `new_value` nulo = a chave deixou de existir.
    old_value     TEXT,
    new_value     TEXT,
    CONSTRAINT pk_config_change PRIMARY KEY (id),
    CONSTRAINT fk_config_change_revision FOREIGN KEY (revision_id)
        REFERENCES ops.config_revision (id) ON DELETE RESTRICT,
    -- Uma chave muda uma vez por revisão; mudar de novo exige outra revisão.
    CONSTRAINT uq_config_change_alvo UNIQUE (revision_id, target_schema, target_table, target_key),
    CONSTRAINT ck_config_change_schema CHECK (target_schema IN ('content', 'game', 'portal', 'ops')),
    CONSTRAINT ck_config_change_tabela CHECK (target_table ~ '^[a-z][a-z0-9_]*$'),
    CONSTRAINT ck_config_change_chave CHECK (char_length(target_key) BETWEEN 1 AND 200),
    CONSTRAINT ck_config_change_muda CHECK (old_value IS DISTINCT FROM new_value),
    CONSTRAINT ck_config_change_tem_valor CHECK (old_value IS NOT NULL OR new_value IS NOT NULL),
    CONSTRAINT ck_config_change_tamanho CHECK (
        (old_value IS NULL OR char_length(old_value) <= 4000)
        AND (new_value IS NULL OR char_length(new_value) <= 4000)
    )
);

CREATE INDEX ix_config_change_revision ON ops.config_change (revision_id);
-- Histórico de uma chave, do mais novo para o mais antigo.
CREATE INDEX ix_config_change_alvo
    ON ops.config_change (target_schema, target_table, target_key, revision_id DESC);

-- O portal é o único runtime que escreve configuração (área administrativa). Sem UPDATE/DELETE.
GRANT SELECT, INSERT ON ops.config_revision TO wyd_portal;
GRANT SELECT, INSERT ON ops.config_change TO wyd_portal;
```

**Contrato que este desenho pretende garantir.** Revisão sem autor é impossível (`ck_config_revision_autor`
mais `author_kind NOT NULL`); mudança órfã é impossível (`revision_id NOT NULL` + FK `RESTRICT`);
revisar sem mudar nada é impossível (`ck_config_change_muda`); duas mudanças na mesma chave dentro de
uma revisão são impossíveis (`uq_config_change_alvo`); reescrever ou apagar história é impossível por
permissão (o portal só tem `SELECT, INSERT`).

**O que está sendo pedido — quatro escolhas de forma e o lote.**

| # | Escolha aberta | O que esta proposta assume |
|---|---|---|
| 1 | Quem conta como autor | ator tripartido `M18-A`: `author_kind` + `author_id` (pessoa) ou `author_ref` (processo), sem FK para `0003` |
| 2 | Como a revisão é numerada e fechada | o `id` identity **é** o número; a revisão nasce aplicada (`applied_at NOT NULL DEFAULT now()`), sem rascunho e sem `UPDATE` |
| 3 | Se a mudança guarda valor anterior e novo | guarda os dois, em `TEXT` canônico; `old_value` nulo = chave nova, `new_value` nulo = chave removida |
| 4 | O que acontece com o histórico ao reconfigurar | histórico append-only e **sem purge**; voltar atrás é revisão nova, nunca `UPDATE` nem `DELETE` |

**Registro da aprovação.** O dono escolheu a opção condicional ("aprovar com as mudanças marcadas"), e
as quatro escolhas marcadas coincidem com as quatro formas acima — nenhuma alteração foi pedida e
nenhuma foi aplicada. O lote entra como confirmado nesses termos: autor `M18-A`, `id` como número com
revisão aplicada, valor anterior e novo, histórico append-only sem purge. As alternativas recusadas por
omissão continuam registradas: FK para pessoa em arquivo posterior, rascunho com `UPDATE`, valor só
novo, retenção com purge.

### 5.10 Lote `0006` — `character` e os quatro satélites (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar como está"**,
    depois de escolher as quatro formas de §5.10. O texto abaixo é o DDL confirmado e as cinco tabelas
    entram na conta de §1.

Fonte de cada campo: o `STRUCT_MOB`/`STRUCT_SCORE` do cliente 7662 (`Basedef.h` de referência, `H`) é
a origem declarada das colunas de estado; `A14` decide a semântica do soft-delete; `D-39` decide
`content_version` no personagem; `A12`/`P-22` mantêm o e-mail fora de `game`. Nomes de coluna e tipos
seguem `padrao-banco.md` (`BIGINT` identity, `TIMESTAMPTZ`, constraint nomeada), não o struct.

```sql
CREATE TABLE game.character (
    id                 BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id         BIGINT NOT NULL,
    slot               SMALLINT NOT NULL,
    name               TEXT NOT NULL,
    -- Dobra do nome no banco: "Kersef" e "kersef" são o mesmo personagem.
    name_fold          TEXT GENERATED ALWAYS AS (lower(btrim(name))) STORED,
    -- `Class` do STRUCT_MOB: 0–5, faixa herdada do cliente (`H`).
    class              SMALLINT NOT NULL DEFAULT 0,
    -- Camada de progressão: Mortal → God → Arch → Subcelestial → God2 → Nightmare.
    tier               TEXT NOT NULL DEFAULT 'mortal',
    level              INTEGER NOT NULL DEFAULT 1,
    exp                BIGINT NOT NULL DEFAULT 0,
    -- `Coin` do STRUCT_MOB: ouro do personagem, nunca a carteira da conta.
    coin               BIGINT NOT NULL DEFAULT 0,
    -- `Magic` do STRUCT_MOB: poder mágico bruto, separado dos scores.
    magic              INTEGER NOT NULL DEFAULT 0,
    -- `LearnedSkill`: máscara de 32 bits, 4 categorias × 8 bits (`H`).
    learned_skill_mask BIGINT NOT NULL DEFAULT 0,
    -- `ScoreBonus`, `SpecialBonus` e `SkillBonus`: pontos livres a distribuir.
    score_bonus        INTEGER NOT NULL DEFAULT 0,
    special_bonus      INTEGER NOT NULL DEFAULT 0,
    skill_bonus        INTEGER NOT NULL DEFAULT 0,
    -- `Critical` e `SaveMana` do STRUCT_MOB (unsigned char).
    critical           SMALLINT NOT NULL DEFAULT 0,
    save_mana          SMALLINT NOT NULL DEFAULT 0,
    -- `SPX`/`SPY`: posição salva pela gema estelar; nula até a primeira gravação.
    recall_x           SMALLINT,
    recall_y           SMALLINT,
    -- Molde de conteúdo que criou o personagem (D-39); a linha existe desde `0001`.
    content_version    SMALLINT NOT NULL,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Soft-delete (A14): a linha fica; nome e vaga voltam a ficar livres.
    deleted_at         TIMESTAMPTZ,
    CONSTRAINT pk_character PRIMARY KEY (id),
    CONSTRAINT fk_character_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE CASCADE,
    CONSTRAINT fk_character_content_version FOREIGN KEY (content_version)
        REFERENCES content.content_version (id),
    CONSTRAINT ck_character_vaga CHECK (slot BETWEEN 0 AND 3),
    CONSTRAINT ck_character_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 16),
    CONSTRAINT ck_character_classe CHECK (class BETWEEN 0 AND 5),
    CONSTRAINT ck_character_tier CHECK (
        tier IN ('mortal', 'god', 'arch', 'subcelestial', 'god2', 'nightmare')
    ),
    CONSTRAINT ck_character_nivel CHECK (level BETWEEN 1 AND 400),
    CONSTRAINT ck_character_exp CHECK (exp >= 0),
    CONSTRAINT ck_character_moeda CHECK (coin >= 0),
    CONSTRAINT ck_character_magia CHECK (magic >= 0),
    CONSTRAINT ck_character_mascara CHECK (learned_skill_mask BETWEEN 0 AND 4294967295),
    CONSTRAINT ck_character_bonus CHECK (
        score_bonus >= 0 AND special_bonus >= 0 AND skill_bonus >= 0
    ),
    CONSTRAINT ck_character_critico CHECK (critical BETWEEN 0 AND 255),
    CONSTRAINT ck_character_save_mana CHECK (save_mana BETWEEN 0 AND 255),
    CONSTRAINT ck_character_exclusao CHECK (deleted_at IS NULL OR deleted_at >= created_at)
);

-- Nome e vaga únicos entre personagens vivos; apagado não bloqueia reuso (A14).
CREATE UNIQUE INDEX ux_character_nome ON game.character (name_fold) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX ux_character_vaga ON game.character (account_id, slot) WHERE deleted_at IS NULL;
CREATE INDEX ix_character_account ON game.character (account_id);
CREATE INDEX ix_character_content_version ON game.character (content_version);

CREATE TRIGGER trg_character_set_updated_at
    BEFORE UPDATE ON game.character
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_status (
    character_id  BIGINT NOT NULL,
    hp            INTEGER NOT NULL DEFAULT 1,
    mp            INTEGER NOT NULL DEFAULT 0,
    hp_max        INTEGER NOT NULL DEFAULT 1,
    mp_max        INTEGER NOT NULL DEFAULT 0,
    pvp_points    INTEGER NOT NULL DEFAULT 0,
    regen_hp      SMALLINT NOT NULL DEFAULT 0,
    regen_mp      SMALLINT NOT NULL DEFAULT 0,
    resist_fire    SMALLINT NOT NULL DEFAULT 0,
    resist_ice     SMALLINT NOT NULL DEFAULT 0,
    resist_thunder SMALLINT NOT NULL DEFAULT 0,
    resist_magic   SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_character_status PRIMARY KEY (character_id),
    CONSTRAINT fk_character_status_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE,
    CONSTRAINT ck_character_status_hp CHECK (hp BETWEEN 0 AND hp_max),
    CONSTRAINT ck_character_status_mp CHECK (mp BETWEEN 0 AND mp_max),
    CONSTRAINT ck_character_status_pvp CHECK (pvp_points >= 0),
    CONSTRAINT ck_character_status_regen CHECK (regen_hp >= 0 AND regen_mp >= 0),
    CONSTRAINT ck_character_status_resist CHECK (
        resist_fire BETWEEN -128 AND 127 AND resist_ice BETWEEN -128 AND 127
        AND resist_thunder BETWEEN -128 AND 127 AND resist_magic BETWEEN -128 AND 127
    )
);

CREATE TABLE game.character_scores (
    character_id       BIGINT NOT NULL,
    base_ac            INTEGER NOT NULL DEFAULT 0,
    base_damage        INTEGER NOT NULL DEFAULT 0,
    base_attack_run    SMALLINT NOT NULL DEFAULT 0,
    base_direction     SMALLINT NOT NULL DEFAULT 0,
    base_chaos_rate    SMALLINT NOT NULL DEFAULT 0,
    base_str           SMALLINT NOT NULL DEFAULT 0,
    base_int           SMALLINT NOT NULL DEFAULT 0,
    base_dex           SMALLINT NOT NULL DEFAULT 0,
    base_con           SMALLINT NOT NULL DEFAULT 0,
    current_ac         INTEGER NOT NULL DEFAULT 0,
    current_damage     INTEGER NOT NULL DEFAULT 0,
    current_attack_run SMALLINT NOT NULL DEFAULT 0,
    current_direction  SMALLINT NOT NULL DEFAULT 0,
    current_chaos_rate SMALLINT NOT NULL DEFAULT 0,
    current_str        SMALLINT NOT NULL DEFAULT 0,
    current_int        SMALLINT NOT NULL DEFAULT 0,
    current_dex        SMALLINT NOT NULL DEFAULT 0,
    current_con        SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_character_scores PRIMARY KEY (character_id),
    CONSTRAINT fk_character_scores_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE,
    CONSTRAINT ck_character_scores_atributos CHECK (
        base_str >= 0 AND base_int >= 0 AND base_dex >= 0 AND base_con >= 0
        AND current_str >= 0 AND current_int >= 0 AND current_dex >= 0 AND current_con >= 0
    ),
    CONSTRAINT ck_character_scores_unsigned CHECK (
        base_attack_run BETWEEN 0 AND 255 AND current_attack_run BETWEEN 0 AND 255
        AND base_direction BETWEEN 0 AND 255 AND current_direction BETWEEN 0 AND 255
        AND base_chaos_rate BETWEEN 0 AND 255 AND current_chaos_rate BETWEEN 0 AND 255
    )
);

CREATE TABLE game.character_special (
    character_id BIGINT NOT NULL,
    special_1    INTEGER NOT NULL DEFAULT 0,
    special_2    INTEGER NOT NULL DEFAULT 0,
    special_3    INTEGER NOT NULL DEFAULT 0,
    special_4    INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT pk_character_special PRIMARY KEY (character_id),
    CONSTRAINT fk_character_special_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE,
    CONSTRAINT ck_character_special CHECK (
        special_1 >= 0 AND special_2 >= 0 AND special_3 >= 0 AND special_4 >= 0
    )
);

CREATE TABLE game.character_short_skill (
    character_id BIGINT NOT NULL,
    slot_1       SMALLINT NOT NULL DEFAULT 0,
    slot_2       SMALLINT NOT NULL DEFAULT 0,
    slot_3       SMALLINT NOT NULL DEFAULT 0,
    slot_4       SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_character_short_skill PRIMARY KEY (character_id),
    CONSTRAINT fk_character_short_skill_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE,
    CONSTRAINT ck_character_short_skill CHECK (
        slot_1 BETWEEN 0 AND 255 AND slot_2 BETWEEN 0 AND 255
        AND slot_3 BETWEEN 0 AND 255 AND slot_4 BETWEEN 0 AND 255
    )
);
```

**Chaves, índices e o que cada constraint impede.**

| Tabela | PK | FK | Índices | Constraint que importa |
|---|---|---|---|---|
| `game.character` | `id` | `account_id → game.account` (`CASCADE`); `content_version → content.content_version` | `ux_character_nome` (parcial), `ux_character_vaga` (parcial), `ix_character_account`, `ix_character_content_version` | nome único entre vivos; vaga 0–3 única por conta; nível 1–400; classe 0–5 |
| `game.character_status` | `character_id` | `character_id → game.character` (`CASCADE`) | — (PK cobre) | `hp ≤ hp_max`, `mp ≤ mp_max`, `hp ≥ 0` |
| `game.character_scores` | `character_id` | `character_id → game.character` (`CASCADE`) | — | atributos não negativos; campos de 8 bits em 0–255 |
| `game.character_special` | `character_id` | `character_id → game.character` (`CASCADE`) | — | quatro especiais não negativos |
| `game.character_short_skill` | `character_id` | `character_id → game.character` (`CASCADE`) | — | quatro slots em 0–255 |

**Permissões.** Nenhum `GRANT` novo: o default privilege de `0001` já dá `SELECT, INSERT, UPDATE, DELETE`
em `game` para `wyd_game` e `SELECT` para `wyd_read`. `wyd_portal` não tem `USAGE` em `game`, então lê
personagem só pela view de `0029` — a fronteira de `P-17` continua valendo.

**O que este desenho deixa de fora, de propósito.** `Equip`/`Carry` vão para `0009`; `Guild`/`GuildLevel`
são normalizados em `0010` (`guild_member`); `Merchant`, `Clan`, `Quest` e `Rsv` são campos de NPC ou sem
semântica verificada e não viram coluna. `Arch`/`arch_of_id` não está no `STRUCT_MOB` e fica fora deste
lote.

**As quatro escolhas de forma aprovadas com o lote.**

| # | Escolha de forma | O que o lote fixa |
|---|---|---|
| 1 | Unicidade do nome | `name_fold` gerada (`lower(btrim(name))`) e índice único **parcial** entre vivos; apagado libera o nome |
| 2 | Layout dos scores | base e atual na mesma tabela, prefixados `base_*` e `current_*`; sem JSONB |
| 3 | Especiais e barra de skills | colunas fixas `special_1..4` e `slot_1..4` (uma linha por personagem), não linhas por slot |
| 4 | Campos do struct sem semântica de jogador (`Merchant`, `Clan`, `Quest`, `Rsv`, `Guild`, `GuildLevel`) | não viram coluna em `character`; guilda é normalizada em `0010` |

**Registro da aprovação.** O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção
"Sim, aprovar como está", depois de escolher as quatro formas acima. Nenhuma coluna mudou depois da
proposta. `0006` passa a `pronta`, as cinco tabelas entram no total de §1 e o arquivo só vira
`concluída` quando a página de migration existir e for conferida.

### 5.11 Lote `0008` — `character_skill` e os quatro satélites (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar como está"**,
    depois de escolher as quatro formas de §5.11. O texto abaixo é o DDL confirmado e as cinco tabelas
    entram na conta de §1.

Origem de cada campo: `STRUCT_MOB`/`STRUCT_SCORE` do cliente 7662 (`H`), a tabela `characters` do
legado e as tabelas de progressão `character_lineage`/`character_skills`/`character_affects`/
`character_chaos` (`H`, referência, nunca cópia). `A14` mantém o soft-delete só em `game.character`.

```sql
CREATE TABLE game.character_skill (
    character_id BIGINT NOT NULL,
    skill_index  SMALLINT NOT NULL,
    mastery      SMALLINT NOT NULL DEFAULT 0,
    learned_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_skill PRIMARY KEY (character_id, skill_index),
    CONSTRAINT fk_character_skill_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_skill_indice CHECK (skill_index BETWEEN 0 AND 247),
    CONSTRAINT ck_character_skill_maestria CHECK (mastery BETWEEN 0 AND 320)
);

-- Sem FK para `content.skill_catalog` (0015): o catálogo é de arquivo posterior e cobre só
-- parte dos 248 índices do cliente.
CREATE INDEX ix_character_skill_indice ON game.character_skill (skill_index);
CREATE TRIGGER trg_character_skill_set_updated_at
    BEFORE UPDATE ON game.character_skill
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_learned_skill (
    character_id BIGINT NOT NULL,
    category     SMALLINT NOT NULL,
    mask         SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_learned_skill PRIMARY KEY (character_id, category),
    CONSTRAINT fk_character_learned_skill_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_learned_skill_categoria CHECK (category BETWEEN 0 AND 3),
    CONSTRAINT ck_character_learned_skill_mascara CHECK (mask BETWEEN 0 AND 255)
);
CREATE TRIGGER trg_character_learned_skill_set_updated_at
    BEFORE UPDATE ON game.character_learned_skill
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_lineage (
    character_id BIGINT NOT NULL,
    lineage_code TEXT NOT NULL,
    mastery      SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_lineage PRIMARY KEY (character_id, lineage_code),
    CONSTRAINT fk_character_lineage_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_lineage_codigo CHECK (lineage_code ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_character_lineage_maestria CHECK (mastery BETWEEN 0 AND 320)
);

-- Sem FK para `content.skill_lineage` (0013): catálogo de conteúdo é de arquivo posterior.
CREATE INDEX ix_character_lineage_codigo ON game.character_lineage (lineage_code);
CREATE TRIGGER trg_character_lineage_set_updated_at
    BEFORE UPDATE ON game.character_lineage
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_affect (
    character_id    BIGINT NOT NULL,
    affect_type     SMALLINT NOT NULL,
    value           SMALLINT NOT NULL,
    level           INTEGER NOT NULL DEFAULT 0,
    remaining_ticks BIGINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_affect PRIMARY KEY (character_id, affect_type),
    CONSTRAINT fk_character_affect_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_affect_tipo CHECK (affect_type >= 0),
    CONSTRAINT ck_character_affect_nivel CHECK (level >= 0),
    CONSTRAINT ck_character_affect_restante CHECK (remaining_ticks >= 0)
);
CREATE INDEX ix_character_affect_tipo ON game.character_affect (affect_type);
CREATE TRIGGER trg_character_affect_set_updated_at
    BEFORE UPDATE ON game.character_affect
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_chaos (
    character_id BIGINT NOT NULL,
    cp           SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_chaos PRIMARY KEY (character_id),
    CONSTRAINT fk_character_chaos_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_chaos_cp CHECK (cp BETWEEN -75 AND 75)
);
CREATE TRIGGER trg_character_chaos_set_updated_at
    BEFORE UPDATE ON game.character_chaos
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

**Chaves, índices e o que cada constraint impede.**

| Tabela | PK | FK | Índices | Constraint que importa |
|---|---|---|---|---|
| `character_skill` | `(character_id, skill_index)` | `character_id → character` (`CASCADE`/`NO ACTION`) | `ix_character_skill_indice` | índice 0–247; maestria 0–320 |
| `character_learned_skill` | `(character_id, category)` | idem | — | categoria 0–3; máscara 0–255 |
| `character_lineage` | `(character_id, lineage_code)` | idem | `ix_character_lineage_codigo` | código `snake_case`; maestria 0–320 |
| `character_affect` | `(character_id, affect_type)` | idem | `ix_character_affect_tipo` | tipo e nível não negativos; ticks ≥ 0 |
| `character_chaos` | `character_id` | idem | — | `cp` entre −75 e 75 |

**Permissões.** Nenhum `GRANT` novo: valem os default privileges do `0001` em `game` (`SELECT, INSERT,
UPDATE, DELETE` para `wyd_game`, `SELECT` para `wyd_read`). `wyd_portal` não tem `USAGE` em `game`.

**Fronteira do lote.** `skill_index` e `lineage_code` **não** têm FK: o catálogo está em `0013`/`0015`
(arquivo posterior) e é parcial. `character` e os cinco campos de estado continuam em `0006`; guilda em
`0010`, item em `0009`, conteúdo em `0013`–`0027`. O limite de 32 afetos e o teto de maestria por classe
são regra de jogo, não coluna.

**As quatro escolhas de forma aprovadas com o lote.**

| # | Escolha de forma | O que o lote fixa |
|---|---|---|
| 1 | Onde mora a máscara de skills aprendidas | em `character_learned_skill` (4 linhas por categoria) e `character.learned_skill_mask` do `0006` fica como **cache do fio**, escrito na mesma transação |
| 2 | Níveis/graus de skill | só `mastery` (0–320) por skill; o grau exibido é derivado de faixas de maestria, sem coluna |
| 3 | Linhagem/herança de skill | tabela 1:N por linhagem (`lineage_code TEXT`), sem FK ao catálogo posterior |
| 4 | Afetos: forma e tempo | `affect_type` numérico, `value`, `level` e `remaining_ticks` (tick de 8 s é código); limite de 32 no core |

**Registro da aprovação.** O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção
"Sim, aprovar como está", depois de escolher as quatro formas acima. Nenhuma coluna mudou depois da
proposta. `0008` passou a `pronta` e depois a `concluída`, com a página de migration escrita e
conferida contra este texto; as cinco tabelas entram no total de §1.

### 5.12 Lote `0009` — `character_item`, `vault_item` e `refine_pity` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar como está"**,
    depois de escolher as cinco formas de §5.12. O texto abaixo é o DDL confirmado e as três tabelas
    entram na conta de §1.

Origem de cada campo: `STRUCT_ITEM` do cliente 7662 (`H`), as tabelas `character_items`/`vault_items`
(`0003_inventario.sql` do legado, `H`) e `refine_pity` (`0043_piedade_refino.sql`, `H`). O catálogo de
item (`content.item_catalog`) é de `0014` e não aparece aqui.

```sql
CREATE TABLE game.character_item (
    character_id BIGINT NOT NULL,
    slot         SMALLINT NOT NULL,
    item_index   INTEGER NOT NULL,
    qty          INTEGER NOT NULL DEFAULT 1,
    flags        INTEGER NOT NULL DEFAULT 0,
    refine       SMALLINT NOT NULL DEFAULT 0,
    ancient      BOOLEAN NOT NULL DEFAULT false,
    gem_effect   SMALLINT NOT NULL DEFAULT 0,
    gem_value    SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_item PRIMARY KEY (character_id, slot),
    CONSTRAINT fk_character_item_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_item_slot CHECK (slot BETWEEN 0 AND 79),
    CONSTRAINT ck_character_item_qtd CHECK (qty >= 1),
    CONSTRAINT ck_character_item_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_character_item_gema CHECK (gem_effect BETWEEN 0 AND 4 AND gem_value >= 0),
    -- Efeito e valor andam juntos: um sem o outro é linha inconsistente.
    CONSTRAINT ck_character_item_gema_par CHECK ((gem_effect = 0) = (gem_value = 0)),
    -- Gema em item comum exige +10; Ancient é a exceção.
    CONSTRAINT ck_character_item_gema_refino CHECK (gem_effect = 0 OR ancient OR refine >= 10)
);

-- Sem FK para `content.item_catalog` (0014): catálogo é de arquivo posterior.
CREATE INDEX ix_character_item_indice ON game.character_item (item_index);
CREATE TRIGGER trg_character_item_set_updated_at
    BEFORE UPDATE ON game.character_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.vault_item (
    account_id BIGINT NOT NULL,
    slot       SMALLINT NOT NULL,
    item_index INTEGER NOT NULL,
    qty        INTEGER NOT NULL DEFAULT 1,
    flags      INTEGER NOT NULL DEFAULT 0,
    refine     SMALLINT NOT NULL DEFAULT 0,
    ancient    BOOLEAN NOT NULL DEFAULT false,
    gem_effect SMALLINT NOT NULL DEFAULT 0,
    gem_value  SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_vault_item PRIMARY KEY (account_id, slot),
    CONSTRAINT fk_vault_item_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_vault_item_slot CHECK (slot BETWEEN 0 AND 127),
    CONSTRAINT ck_vault_item_qtd CHECK (qty >= 1),
    CONSTRAINT ck_vault_item_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_vault_item_gema CHECK (gem_effect BETWEEN 0 AND 4 AND gem_value >= 0),
    CONSTRAINT ck_vault_item_gema_par CHECK ((gem_effect = 0) = (gem_value = 0)),
    CONSTRAINT ck_vault_item_gema_refino CHECK (gem_effect = 0 OR ancient OR refine >= 10)
);

CREATE INDEX ix_vault_item_indice ON game.vault_item (item_index);
CREATE TRIGGER trg_vault_item_set_updated_at
    BEFORE UPDATE ON game.vault_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.refine_pity (
    character_id BIGINT NOT NULL,
    slot         SMALLINT NOT NULL,
    pity         SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_refine_pity PRIMARY KEY (character_id, slot),
    -- Piedade sem item no slot é estado impossível; sai junto com o item.
    CONSTRAINT fk_refine_pity_item FOREIGN KEY (character_id, slot)
        REFERENCES game.character_item (character_id, slot)
        ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_refine_pity_valor CHECK (pity BETWEEN 0 AND 20)
);
CREATE TRIGGER trg_refine_pity_set_updated_at
    BEFORE UPDATE ON game.refine_pity
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

**Chaves, índices e o que cada constraint impede.**

| Tabela | PK | FK | Índices | Constraint que importa |
|---|---|---|---|---|
| `character_item` | `(character_id, slot)` | `character_id → character` (`CASCADE`/`NO ACTION`) | `ix_character_item_indice` | slot 0–79; qty ≥ 1; refino 0–15; gema coerente |
| `vault_item` | `(account_id, slot)` | `account_id → account` (`CASCADE`/`NO ACTION`) | `ix_vault_item_indice` | slot 0–127; mesmas regras de item |
| `refine_pity` | `(character_id, slot)` | `(character_id, slot) → character_item` (`CASCADE`/`NO ACTION`) | — (PK cobre) | piedade 0–20; sem item no slot não existe linha |

**Permissões.** Nenhum `GRANT` novo: default privileges do `0001` em `game` (`SELECT, INSERT, UPDATE,
DELETE` para `wyd_game`, `SELECT` para `wyd_read`). `wyd_portal` não tem `USAGE` em `game`; o admin vê
inventário pela view de `0029`.

**Fronteira do lote.** `item_index` não tem FK: catálogo em `0014`, preços em `0011`, guilda em `0010`.
Nome, efeito, classe e preço de item **não** entram aqui. `refine_pity` é do slot do personagem, não da
conta.

**As cinco escolhas de forma aprovadas com o lote.**

| # | Escolha de forma | O que o lote fixa |
|---|---|---|
| 1 | Baú é da conta ou do personagem | da **conta** (`vault_item.account_id`), como no legado |
| 2 | Personagem e baú compartilham tabela | **não**: duas tabelas com as mesmas colunas de item, donos diferentes |
| 3 | Quantidade/durabilidade | `qty` por linha (empilhamento); durabilidade não existe no cliente e não vira coluna |
| 4 | Limite de slots | personagem 0–79 (16 equip + 64 carry); baú 0–127 |
| 5 | O que `refine_pity` guarda e por quanto tempo | contador 0–20 por `(personagem, slot)`, apagado com o item; sem histórico |

**Registro da aprovação.** O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção
"Sim, aprovar como está", depois de escolher as cinco formas acima. Nenhuma coluna mudou depois da
proposta. `0009` passa a `concluída`, as três tabelas entram no total de §1 e a página de migration já
conferida contra este texto.

### 5.13 Lote `0010` — `guild`, `guild_member`, `guild_relation` e `guild_tower` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar como está"**,
    depois de escolher as quatro formas de §5.13. O texto abaixo é o DDL confirmado e as quatro tabelas
    entram na conta de §1.

Origem de cada campo: `guilds`/`guild_members` do legado (`0004_guildas.sql`, `H`),
`guild_relations`/`guild_tower` (`0028_estado_de_jogo.sql`, `H`), `STRUCT_MOB.Guild` e
`GuildLevel` e `STRUCT_GUILDINFO` do cliente 7662 (`H`). Não há FK para `content`: zona de guilda é
`0019`.

```sql
CREATE TABLE game.guild (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    name       TEXT NOT NULL,
    name_fold  TEXT GENERATED ALWAYS AS (lower(btrim(name))) STORED,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild PRIMARY KEY (id),
    CONSTRAINT ck_guild_nome CHECK (char_length(btrim(name)) BETWEEN 3 AND 16)
);

-- Nome único entre guildas existentes: `name_fold` compara sem caixa e sem espaço de borda.
CREATE UNIQUE INDEX ux_guild_nome ON game.guild (name_fold);
CREATE TRIGGER trg_guild_set_updated_at
    BEFORE UPDATE ON game.guild
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.guild_member (
    guild_id     BIGINT NOT NULL,
    character_id BIGINT NOT NULL,
    role         TEXT NOT NULL DEFAULT 'member',
    joined_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_member PRIMARY KEY (guild_id, character_id),
    CONSTRAINT fk_guild_member_guild FOREIGN KEY (guild_id)
        REFERENCES game.guild (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_guild_member_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_guild_member_papel CHECK (role IN ('leader', 'sub_leader', 'member'))
);

-- Um personagem pertence a uma guilda só (regra do legado, garantida no banco).
CREATE UNIQUE INDEX ux_guild_member_personagem ON game.guild_member (character_id);
-- Um líder por guilda, inclusive sob concorrência.
CREATE UNIQUE INDEX ux_guild_member_lider ON game.guild_member (guild_id) WHERE role = 'leader';
CREATE TRIGGER trg_guild_member_set_updated_at
    BEFORE UPDATE ON game.guild_member
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.guild_relation (
    guild_id        BIGINT NOT NULL,
    target_guild_id BIGINT NOT NULL,
    kind            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_relation PRIMARY KEY (guild_id, target_guild_id),
    CONSTRAINT fk_guild_relation_guild FOREIGN KEY (guild_id)
        REFERENCES game.guild (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_guild_relation_target FOREIGN KEY (target_guild_id)
        REFERENCES game.guild (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_guild_relation_kind CHECK (kind IN ('ally', 'hostile')),
    CONSTRAINT ck_guild_relation_distinta CHECK (guild_id <> target_guild_id)
);

CREATE INDEX ix_guild_relation_target ON game.guild_relation (target_guild_id);
CREATE TRIGGER trg_guild_relation_set_updated_at
    BEFORE UPDATE ON game.guild_relation
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.guild_tower (
    id             SMALLINT NOT NULL DEFAULT 1,
    owner_guild_id BIGINT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_tower PRIMARY KEY (id),
    CONSTRAINT ck_guild_tower_singleton CHECK (id = 1),
    -- GTorre sem dono é `NULL`, nunca um id sentinela: guilda dissolvida zera o dono.
    CONSTRAINT fk_guild_tower_owner FOREIGN KEY (owner_guild_id)
        REFERENCES game.guild (id) ON DELETE SET NULL ON UPDATE NO ACTION
);
CREATE TRIGGER trg_guild_tower_set_updated_at
    BEFORE UPDATE ON game.guild_tower
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

**Chaves, índices e o que cada constraint impede.**

| Tabela | PK | FK | Índices | Constraint que importa |
|---|---|---|---|---|
| `guild` | `id` | — | `ux_guild_nome` | nome 3–16 e único sem caixa |
| `guild_member` | `(guild_id, character_id)` | `guild_id → guild`, `character_id → character` (`CASCADE`) | `ux_guild_member_personagem`, `ux_guild_member_lider` (parcial) | um personagem em uma guilda; um líder por guilda; papel fechado |
| `guild_relation` | `(guild_id, target_guild_id)` | ambas → `guild` (`CASCADE`) | `ix_guild_relation_target` | sem auto-relação; tipo `ally`/`hostile` |
| `guild_tower` | `id` | `owner_guild_id → guild` (`SET NULL`) | — | linha única (`id = 1`) |

**Permissões.** Nenhum `GRANT` novo: default privileges do `0001` em `game` (`SELECT, INSERT, UPDATE,
DELETE` para `wyd_game`, `SELECT` para `wyd_read`). `wyd_portal` não tem `USAGE` em `game`; o admin vê
guilda pela view de `0029`.

**Fronteira do lote.** Zona de guilda, `ChargeGuild`/`ChallangeGuild` e spawn de guilda são conteúdo
(`guild_zone`, `0019`). Nome e nível de guilda vindos de `STRUCT_GUILDINFO` não viram coluna aqui
além do nome: fama e nível de guilda não têm semântica medida e ficam para decisão própria. Relação
de guilda é estado do lote, não catálogo.

**As quatro escolhas de forma aprovadas com o lote.**

| # | Escolha de forma | O que o lote fixa |
|---|---|---|
| 1 | Liderança e cargos | `role` textual em `guild_member` (`leader`/`sub_leader`/`member`), um líder por guilda por índice parcial; sem coluna `master_character_id` |
| 2 | Unicidade do nome | `name_fold` gerada e índice **total** `ux_guild_nome`; guilda que acaba é apagada, não fica como linha morta |
| 3 | Saída de membro | sair ou ser expulso apaga o vínculo (`guild_member`), sem histórico; apagar a guilda também remove seus vínculos por FK `ON DELETE CASCADE` |
| 4 | Dono da GTorre | `owner_guild_id` **anulável** com `ON DELETE SET NULL`; ausência de linha ou `NULL` = sem dono |

**Registro da aprovação.** O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção
"Sim, aprovar como está", depois de escolher as quatro formas acima. Nenhuma coluna mudou depois da
proposta. `0010` passa a `concluída`, as quatro tabelas entram no total de §1 e a página de migration
já conferida contra este texto.

### 5.14 Lote `0011` — economia: `city`, `item_price`, `personal_shop`, `city_tax_ledger` e as seis de cash (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar como está"**,
    depois de escolher as quatro formas de §5.14. O texto abaixo é o DDL confirmado e as dez tabelas
    entram na conta de §1.

Origem de cada campo: `city`, `item_price`, `personal_shop` e `city_tax_ledger` do legado
(`0010_economia.sql`, `H`); `donate_shop_item` e `daily_reward_item` (`0010_economia.sql`, `H`);
`kingdom_cape_price` e `daily_reward_claim` (`0028_estado_de_jogo.sql`, `H`); `donate_credit`
(`0042_credito_donate.sql`, `H`) e `donate_debit` (`0045_donate_debito.sql`, `H`). `A19` mantém 6
tabelas em `portal` e 4 em `game`; `M17` exige chave de idempotência única.

```sql
CREATE TABLE game.city (
    code          TEXT NOT NULL,
    name          TEXT NOT NULL,
    -- Mapa da cidade não é publicado pelo cliente 7662 (`H`); fica nulo até prova de trace.
    map_id        SMALLINT,
    is_capital    BOOLEAN NOT NULL DEFAULT false,
    tax_rate_bps  INTEGER NOT NULL DEFAULT 1000,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_city PRIMARY KEY (code),
    CONSTRAINT ck_city_code CHECK (code ~ '^[a-z][a-z0-9_]{2,15}$'),
    CONSTRAINT ck_city_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 32),
    CONSTRAINT ck_city_mapa CHECK (map_id IS NULL OR map_id BETWEEN 0 AND 255),
    CONSTRAINT ck_city_imposto CHECK (tax_rate_bps BETWEEN 0 AND 10000)
);
CREATE TRIGGER trg_city_set_updated_at
    BEFORE UPDATE ON game.city
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.item_price (
    item_index INTEGER NOT NULL,
    npc_id     INTEGER NOT NULL,
    buy_price  BIGINT NOT NULL DEFAULT 0,
    sell_price BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_price PRIMARY KEY (item_index, npc_id),
    CONSTRAINT ck_item_price_compra CHECK (buy_price >= 0),
    CONSTRAINT ck_item_price_venda CHECK (sell_price >= 0)
);

-- Sem FK para `content.item_catalog`/`content.npc_definition` (0014/0019): conteúdo é posterior.
CREATE INDEX ix_item_price_npc ON game.item_price (npc_id);
CREATE TRIGGER trg_item_price_set_updated_at
    BEFORE UPDATE ON game.item_price
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.personal_shop (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    character_id BIGINT NOT NULL,
    city_code    TEXT NOT NULL,
    name         TEXT NOT NULL,
    is_open      BOOLEAN NOT NULL DEFAULT true,
    tax_bps      INTEGER NOT NULL DEFAULT 1000,
    tax_paid     BIGINT NOT NULL DEFAULT 0,
    opened_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at    TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_personal_shop PRIMARY KEY (id),
    CONSTRAINT fk_personal_shop_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_personal_shop_city FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_personal_shop_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 24),
    CONSTRAINT ck_personal_shop_imposto CHECK (tax_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_personal_shop_pago CHECK (tax_paid >= 0),
    CONSTRAINT ck_personal_shop_fechamento CHECK (closed_at IS NULL OR closed_at >= opened_at),
    -- Aberta não tem data de fechamento; fechada tem.
    CONSTRAINT ck_personal_shop_estado CHECK (is_open = (closed_at IS NULL))
);

-- Uma loja aberta por personagem e a busca por cidade são os dois acessos quentes.
CREATE UNIQUE INDEX ux_personal_shop_aberta ON game.personal_shop (character_id) WHERE is_open;
CREATE INDEX ix_personal_shop_cidade ON game.personal_shop (city_code) WHERE is_open;
CREATE TRIGGER trg_personal_shop_set_updated_at
    BEFORE UPDATE ON game.personal_shop
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.city_tax_ledger (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    city_code   TEXT NOT NULL,
    shop_id     BIGINT,
    amount      BIGINT NOT NULL,
    reason      TEXT NOT NULL,
    collected_by BIGINT,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_city_tax_ledger PRIMARY KEY (id),
    CONSTRAINT fk_city_tax_ledger_city FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_city_tax_ledger_shop FOREIGN KEY (shop_id)
        REFERENCES game.personal_shop (id) ON DELETE SET NULL ON UPDATE NO ACTION,
    CONSTRAINT fk_city_tax_ledger_quem FOREIGN KEY (collected_by)
        REFERENCES game.character (id) ON DELETE SET NULL ON UPDATE NO ACTION,
    CONSTRAINT ck_city_tax_ledger_valor CHECK (amount >= 0),
    CONSTRAINT ck_city_tax_ledger_motivo CHECK (
        reason IN ('shop_sale', 'npc_sale', 'portal', 'collection')
    )
);

CREATE INDEX ix_city_tax_ledger_cidade ON game.city_tax_ledger (city_code, occurred_at DESC);

CREATE TABLE portal.donate_shop_item (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    item_index    INTEGER NOT NULL,
    price_credits INTEGER NOT NULL,
    duration_days SMALLINT,
    enabled       BOOLEAN NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_shop_item PRIMARY KEY (id),
    CONSTRAINT ck_donate_shop_item_preco CHECK (price_credits > 0),
    CONSTRAINT ck_donate_shop_item_duracao CHECK (duration_days IS NULL OR duration_days > 0)
);
CREATE TRIGGER trg_donate_shop_item_set_updated_at
    BEFORE UPDATE ON portal.donate_shop_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.daily_reward_item (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    day          SMALLINT,
    item_index   INTEGER,
    gold         BIGINT NOT NULL DEFAULT 0,
    qty          SMALLINT NOT NULL DEFAULT 1,
    account_wide BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_daily_reward_item PRIMARY KEY (id),
    CONSTRAINT ck_daily_reward_item_dia CHECK (day IS NULL OR day BETWEEN 1 AND 7),
    CONSTRAINT ck_daily_reward_item_ouro CHECK (gold >= 0),
    CONSTRAINT ck_daily_reward_item_qtd CHECK (qty > 0),
    -- Recompensa sem item e sem ouro não entrega nada.
    CONSTRAINT ck_daily_reward_item_conteudo CHECK (item_index IS NOT NULL OR gold > 0)
);
CREATE TRIGGER trg_daily_reward_item_set_updated_at
    BEFORE UPDATE ON portal.daily_reward_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.daily_reward_claim (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id     BIGINT NOT NULL,
    reward_item_id BIGINT,
    claim_date     DATE NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_daily_reward_claim PRIMARY KEY (id),
    -- Um resgate por conta e dia UTC: o `23505` é o sinal de "já resgatou".
    CONSTRAINT uq_daily_reward_claim_dia UNIQUE (account_id, claim_date),
    CONSTRAINT fk_daily_reward_claim_reward FOREIGN KEY (reward_item_id)
        REFERENCES portal.daily_reward_item (id) ON DELETE RESTRICT ON UPDATE NO ACTION
);

CREATE TABLE portal.kingdom_cape_price (
    id            SMALLINT NOT NULL DEFAULT 1,
    version       BIGINT NOT NULL DEFAULT 1,
    hekalotia_cost INTEGER NOT NULL DEFAULT 8,
    akelonia_cost  INTEGER NOT NULL DEFAULT 8,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_kingdom_cape_price PRIMARY KEY (id),
    CONSTRAINT ck_kingdom_cape_price_singleton CHECK (id = 1),
    CONSTRAINT ck_kingdom_cape_price_versao CHECK (version >= 1),
    CONSTRAINT ck_kingdom_cape_price_hekalotia CHECK (hekalotia_cost BETWEEN 4 AND 32),
    CONSTRAINT ck_kingdom_cape_price_akelonia CHECK (akelonia_cost BETWEEN 4 AND 32)
);
CREATE TRIGGER trg_kingdom_cape_price_set_updated_at
    BEFORE UPDATE ON portal.kingdom_cape_price
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.donate_credit (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    idempotency_key UUID NOT NULL,
    account_id      BIGINT NOT NULL,
    credits         BIGINT NOT NULL,
    created_by      BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_credit PRIMARY KEY (id),
    CONSTRAINT uq_donate_credit_idem UNIQUE (idempotency_key),
    CONSTRAINT ck_donate_credit_valor CHECK (credits > 0)
);

CREATE TABLE portal.donate_debit (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    idempotency_key UUID NOT NULL,
    account_id      BIGINT NOT NULL,
    credits         BIGINT NOT NULL,
    reason          TEXT NOT NULL DEFAULT 'portal',
    created_by      BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_debit PRIMARY KEY (id),
    CONSTRAINT uq_donate_debit_idem UNIQUE (idempotency_key),
    CONSTRAINT ck_donate_debit_valor CHECK (credits > 0),
    CONSTRAINT ck_donate_debit_motivo CHECK (reason IN ('portal', 'shop_purchase'))
);

CREATE INDEX ix_donate_debit_conta ON portal.donate_debit (account_id, created_at DESC);
```

**Chaves, índices e o que cada constraint impede.**

| Tabela | PK | FK | Índices | Constraint que importa |
|---|---|---|---|---|
| `city` | `code` | — | — | código `snake_case`; imposto em bps 0–10000 |
| `item_price` | `(item_index, npc_id)` | — (conteúdo em `0014`/`0019`) | `ix_item_price_npc` | preços não negativos; um preço por item e mercador |
| `personal_shop` | `id` | `character_id → character` (`CASCADE`), `city_code → city` (`RESTRICT`) | `ux_personal_shop_aberta` (parcial), `ix_personal_shop_cidade` (parcial) | uma loja aberta por personagem; estado e `closed_at` coerentes |
| `city_tax_ledger` | `id` | `city_code → city` (`RESTRICT`), `shop_id → personal_shop` (`SET NULL`), `collected_by → character` (`SET NULL`) | `ix_city_tax_ledger_cidade` | valor não negativo; quatro motivos fechados |
| `donate_shop_item` | `id` | — (item em `0014`) | — | preço em créditos > 0; duração positiva ou nula |
| `daily_reward_item` | `id` | — (item em `0014`) | — | dia 1–7 ou nulo; entrega item ou ouro |
| `daily_reward_claim` | `id` | `reward_item_id → daily_reward_item` (`RESTRICT`) | `uq_daily_reward_claim_dia` | um resgate por conta e dia |
| `kingdom_cape_price` | `id` | — | — | linha única; versão ≥ 1; custo 4–32 |
| `donate_credit` | `id` | — (conta em `game`, por decisão de fronteira) | `uq_donate_credit_idem` | chave única; crédito > 0 |
| `donate_debit` | `id` | — (conta em `game`, por decisão de fronteira) | `uq_donate_debit_idem`, `ix_donate_debit_conta` | chave única; débito > 0; dois motivos |

**Permissões.** Nenhum `GRANT` novo: default privileges do `0001`. Em `game`, `wyd_game` escreve e
`wyd_read` lê; em `portal`, `wyd_portal` escreve e `wyd_read` lê. `wyd_game` não tem `USAGE` em
`portal` e `wyd_portal` não tem `USAGE` em `game`; as duas fronteiras seguem de `P-17`.

**Fronteira do lote.** Catálogo de item, de NPC e de região são `0014`/`0019`/`0024`; aqui só índices e
preços. `character.coin` e `account_wallet.coin` são de `0006`/`0005`; este lote registra razão, não
saldo. Entrega de item comprado é `0012`.

**As quatro escolhas de forma aprovadas com o lote.**

| # | Escolha de forma | O que o lote fixa |
|---|---|---|
| 1 | Fronteira das seis de cash em `portal` | `account_id` **sem FK** para `game.account`, porque `wyd_portal` não tem `USAGE` em `game`; a conta é validada na borda |
| 2 | Chave de idempotência (`M17`) | `UUID` nativo, `UNIQUE` por tabela; replay de 24 h é política do BFF, não coluna |
| 3 | Preço por mercador | PK `(item_index, npc_id)` com `npc_id` **obrigatório**; preço de catálogo fica em `0014` |
| 4 | Loja pessoal | uma loja **aberta** por personagem (índice parcial) e `ON DELETE CASCADE` do personagem |

**Registro da aprovação.** O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção
"Sim, aprovar como está", depois de escolher as quatro formas acima. Nenhuma coluna mudou depois da
proposta. `0011` passa a `concluída`, as dez tabelas entram no total de §1 e a página de migration já
conferida contra este texto.

### 5.15 Lote `0030` — conteúdo do portal: `news`, `banner`, `guide` e `launcher_release` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o desenho na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar o desenho
    como está"**. As quatro tabelas entram na conta de §1 (49 de 96 na época; hoje 97 de 97), a série passa a
    `0030_portal_content.sql` como 30º arquivo e a página `migrations/0030-portal-content.md` está
    `concluída` no checklist.

**Reafirmado em 2026-11-21:** o lote permanece como está e **não há lote novo** para estas quatro
tabelas. O texto abaixo continua sendo o único DDL delas; a decisão está em `plano-migrations.md` §1.

O dono decidiu em 2026-11-20 **quais** tabelas o CMS e o launcher precisam (`LAC-POR-02`,
`LAC-POR-03`, em `wyd-portal/lacunas.md` §3): três de conteúdo editorial e uma de versão. O desenho
abaixo é o confirmado nesta data e é o que a página de migration copia.

Forma: `portal`, porque é conteúdo do site, não estado de jogo; sem FK para `game`/`content`
(`P-17`, e `portal` não tem `USAGE` em `game`); autor em `portal.person` com `RESTRICT`; estado
explícito em `status` + carimbo, sem `deleted_at` (`P-20`).

```sql
CREATE TABLE portal.news (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    slug                 TEXT NOT NULL,
    title                TEXT NOT NULL,
    summary              TEXT NOT NULL DEFAULT '',
    body_md              TEXT NOT NULL,
    cover_url            TEXT,
    status               TEXT NOT NULL DEFAULT 'draft',
    pinned               BOOLEAN NOT NULL DEFAULT false,
    published_at         TIMESTAMPTZ,
    archived_at          TIMESTAMPTZ,
    author_person_id     BIGINT NOT NULL,
    updated_by_person_id BIGINT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_news PRIMARY KEY (id),
    CONSTRAINT fk_news_author FOREIGN KEY (author_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_news_updated_by FOREIGN KEY (updated_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_news_slug CHECK (
        slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(slug) BETWEEN 3 AND 120
    ),
    CONSTRAINT ck_news_titulo CHECK (char_length(btrim(title)) BETWEEN 1 AND 160),
    CONSTRAINT ck_news_resumo CHECK (char_length(summary) <= 400),
    CONSTRAINT ck_news_corpo CHECK (char_length(body_md) BETWEEN 1 AND 200000),
    CONSTRAINT ck_news_capa CHECK (cover_url IS NULL OR cover_url ~ '^https://'),
    CONSTRAINT ck_news_status CHECK (status IN ('draft', 'published', 'archived')),
    CONSTRAINT ck_news_publicacao CHECK ((status <> 'draft') = (published_at IS NOT NULL)),
    CONSTRAINT ck_news_arquivo CHECK ((status = 'archived') = (archived_at IS NOT NULL)),
    CONSTRAINT ck_news_fixada CHECK (NOT pinned OR status = 'published')
);

CREATE UNIQUE INDEX ux_news_slug ON portal.news (slug);
CREATE INDEX ix_news_publicadas ON portal.news (published_at DESC) WHERE status = 'published';
CREATE INDEX ix_news_autor ON portal.news (author_person_id);

CREATE TRIGGER trg_news_set_updated_at
    BEFORE UPDATE ON portal.news
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.banner (
    id                    BIGINT GENERATED ALWAYS AS IDENTITY,
    slot                  TEXT NOT NULL,
    title                 TEXT NOT NULL,
    subtitle              TEXT NOT NULL DEFAULT '',
    image_url             TEXT NOT NULL,
    link_url              TEXT,
    featured_character_id BIGINT,
    priority              SMALLINT NOT NULL DEFAULT 100,
    enabled               BOOLEAN NOT NULL DEFAULT true,
    starts_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    ends_at               TIMESTAMPTZ,
    author_person_id      BIGINT NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_banner PRIMARY KEY (id),
    CONSTRAINT fk_banner_author FOREIGN KEY (author_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_banner_slot CHECK (slot IN ('home_hero', 'home_side', 'login', 'launcher')),
    CONSTRAINT ck_banner_titulo CHECK (char_length(btrim(title)) BETWEEN 1 AND 80),
    CONSTRAINT ck_banner_subtitulo CHECK (char_length(subtitle) <= 200),
    CONSTRAINT ck_banner_imagem CHECK (image_url ~ '^https://'),
    CONSTRAINT ck_banner_link CHECK (link_url IS NULL OR link_url ~ '^https://'),
    CONSTRAINT ck_banner_personagem CHECK (featured_character_id IS NULL OR featured_character_id > 0),
    CONSTRAINT ck_banner_prioridade CHECK (priority BETWEEN 0 AND 999),
    CONSTRAINT ck_banner_janela CHECK (ends_at IS NULL OR ends_at > starts_at)
);

CREATE INDEX ix_banner_vigentes ON portal.banner (slot, priority) WHERE enabled;
CREATE INDEX ix_banner_personagem ON portal.banner (featured_character_id)
    WHERE featured_character_id IS NOT NULL;

CREATE TRIGGER trg_banner_set_updated_at
    BEFORE UPDATE ON portal.banner
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.guide (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    slug                 TEXT NOT NULL,
    category             TEXT NOT NULL DEFAULT 'geral',
    title                TEXT NOT NULL,
    body_md              TEXT NOT NULL,
    status               TEXT NOT NULL DEFAULT 'draft',
    published_at         TIMESTAMPTZ,
    archived_at          TIMESTAMPTZ,
    author_person_id     BIGINT NOT NULL,
    updated_by_person_id BIGINT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guide PRIMARY KEY (id),
    CONSTRAINT fk_guide_author FOREIGN KEY (author_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_guide_updated_by FOREIGN KEY (updated_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_guide_slug CHECK (
        slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(slug) BETWEEN 3 AND 120
    ),
    CONSTRAINT ck_guide_categoria CHECK (category ~ '^[a-z][a-z0-9_-]{1,31}$'),
    CONSTRAINT ck_guide_titulo CHECK (char_length(btrim(title)) BETWEEN 1 AND 160),
    CONSTRAINT ck_guide_corpo CHECK (char_length(body_md) BETWEEN 1 AND 200000),
    CONSTRAINT ck_guide_status CHECK (status IN ('draft', 'published', 'archived')),
    CONSTRAINT ck_guide_publicacao CHECK ((status <> 'draft') = (published_at IS NOT NULL)),
    CONSTRAINT ck_guide_arquivo CHECK ((status = 'archived') = (archived_at IS NOT NULL))
);

CREATE UNIQUE INDEX ux_guide_slug ON portal.guide (slug);
CREATE INDEX ix_guide_publicados ON portal.guide (category, published_at DESC)
    WHERE status = 'published';
CREATE INDEX ix_guide_autor ON portal.guide (author_person_id);

CREATE TRIGGER trg_guide_set_updated_at
    BEFORE UPDATE ON portal.guide
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.launcher_release (
    id                     BIGINT GENERATED ALWAYS AS IDENTITY,
    channel                TEXT NOT NULL DEFAULT 'stable',
    version                TEXT NOT NULL,
    manifest_url           TEXT NOT NULL,
    manifest_sha256        CHAR(64) NOT NULL,
    min_version            TEXT,
    notes_md               TEXT NOT NULL DEFAULT '',
    status                 TEXT NOT NULL DEFAULT 'draft',
    released_at            TIMESTAMPTZ,
    revoked_at             TIMESTAMPTZ,
    published_by_person_id BIGINT,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_launcher_release PRIMARY KEY (id),
    CONSTRAINT fk_launcher_release_publisher FOREIGN KEY (published_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_launcher_release_canal CHECK (channel IN ('stable', 'beta')),
    CONSTRAINT ck_launcher_release_versao CHECK (
        version ~ '^[0-9]+[.][0-9]+[.][0-9]+(-[a-z0-9.]+)?$'
    ),
    CONSTRAINT ck_launcher_release_min_versao CHECK (
        min_version IS NULL OR min_version ~ '^[0-9]+[.][0-9]+[.][0-9]+(-[a-z0-9.]+)?$'
    ),
    CONSTRAINT ck_launcher_release_manifesto CHECK (manifest_url ~ '^https://'),
    CONSTRAINT ck_launcher_release_sha CHECK (manifest_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_launcher_release_notas CHECK (char_length(notes_md) <= 20000),
    CONSTRAINT ck_launcher_release_status CHECK (status IN ('draft', 'published', 'revoked')),
    CONSTRAINT ck_launcher_release_publicacao CHECK ((status <> 'draft') = (released_at IS NOT NULL)),
    CONSTRAINT ck_launcher_release_revogacao CHECK ((status = 'revoked') = (revoked_at IS NOT NULL)),
    CONSTRAINT ck_launcher_release_autor CHECK (
        status = 'draft' OR published_by_person_id IS NOT NULL
    )
);

CREATE UNIQUE INDEX ux_launcher_release_versao ON portal.launcher_release (channel, version);
CREATE UNIQUE INDEX ux_launcher_release_vigente ON portal.launcher_release (channel)
    WHERE status = 'published';
CREATE INDEX ix_launcher_release_historico ON portal.launcher_release (channel, released_at DESC)
    WHERE status <> 'draft';

CREATE TRIGGER trg_launcher_release_set_updated_at
    BEFORE UPDATE ON portal.launcher_release
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

```sql
-- migrate:down
DROP TRIGGER trg_launcher_release_set_updated_at ON portal.launcher_release;
DROP INDEX ix_launcher_release_historico;
DROP INDEX ux_launcher_release_vigente;
DROP INDEX ux_launcher_release_versao;
DROP TABLE portal.launcher_release;

DROP TRIGGER trg_guide_set_updated_at ON portal.guide;
DROP INDEX ix_guide_autor;
DROP INDEX ix_guide_publicados;
DROP INDEX ux_guide_slug;
DROP TABLE portal.guide;

DROP TRIGGER trg_banner_set_updated_at ON portal.banner;
DROP INDEX ix_banner_personagem;
DROP INDEX ix_banner_vigentes;
DROP TABLE portal.banner;

DROP TRIGGER trg_news_set_updated_at ON portal.news;
DROP INDEX ix_news_autor;
DROP INDEX ix_news_publicadas;
DROP INDEX ux_news_slug;
DROP TABLE portal.news;
```

**Permissões.** Nenhum `GRANT` novo: default privileges de `0001` em `portal` dão
`SELECT, INSERT, UPDATE, DELETE` a `wyd_portal`; `wyd_read` tem `USAGE` mas não `SELECT` por default;
`wyd_game` não alcança. O papel `wyd_admin` existe desde 2026-11-21 (`M27`, `0032_admin_grants.sql`),
mas não alcança estas quatro tabelas. Elas são escritas pelo **BFF do portal** com a credencial
`wyd_portal`; o serviço `wyd-admin` **não** as escreve. `wyd_portal` não serve para `game` (o `0001`
não dá `USAGE` no schema a `wyd_portal`).

**O que este lote não fecha:** histórico de revisão do texto e prazo de expurgo, listados em
`migrations/0030-portal-content.md` §11.

### 5.16 Lote `0012` — `ops.delivery_queue` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção **"Sim, aprovar como está"**,
    depois de fechar `M25`: dreno com `SELECT … FOR UPDATE SKIP LOCKED` em transação curta, e expurgo
    de `applied`/`failed` 30 dias após `closed_at`, com `pending` nunca expurgada. O texto abaixo é o
    DDL confirmado e a tabela entra na conta de §1.

A ponte única Cold → Hot: o portal **enfileira** o pedido, o servidor de jogo **drena** e aplica, e
a linha guarda quem pediu, para quem e o que foi feito.

O que já está decidido e o texto abaixo respeita:

| Decisão | Efeito no desenho |
|---|---|
| `M17` (2026-11-19) | `idempotency_key UUID NOT NULL` com `UNIQUE`: reenvio do BFF devolve a mesma linha |
| `LAC-POR-01` / `padrao-banco.md` §2 e §8 | portal não escreve estado vivo; a fila é o caminho, e quem aplica é o jogo |
| `P-15` / `P-20` | `RESTRICT` nas FKs de destino — concessão pendente impede apagar conta ou personagem; estado explícito com carimbo coerente |
| `P-21` / `M22` / `M25` (2026-11-20) | retenção declarada: `applied`/`failed` expurgadas 30 dias após `closed_at`; `pending` nunca |

```sql
CREATE TABLE ops.delivery_queue (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    -- Chave do pedido gerada pelo BFF: reenvio devolve a mesma entrega, nunca enfileira duas (M17).
    idempotency_key      UUID NOT NULL,
    -- `gold` credita a carteira da conta; `item` e `level` exigem personagem.
    kind                 TEXT NOT NULL,
    account_id           BIGINT,
    character_id         BIGINT,
    item_index           INTEGER,
    qty                  INTEGER,
    gold_amount          BIGINT,
    level_delta          INTEGER,
    status               TEXT NOT NULL DEFAULT 'pending',
    attempt_count        SMALLINT NOT NULL DEFAULT 0,
    -- Instante em que o dreno tomou a linha para si.
    claimed_at           TIMESTAMPTZ,
    -- Instante em que a linha deixou a fila, por sucesso ou por falha.
    closed_at            TIMESTAMPTZ,
    failure_reason       TEXT,
    -- Staff que pediu a concessão pela web. Pessoa nunca é apagada (A12), então RESTRICT é seguro.
    created_by_person_id BIGINT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_delivery_queue PRIMARY KEY (id),
    CONSTRAINT uq_delivery_queue_idem UNIQUE (idempotency_key),
    CONSTRAINT fk_delivery_queue_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_delivery_queue_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_delivery_queue_autor FOREIGN KEY (created_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_delivery_queue_tipo CHECK (kind IN ('item', 'gold', 'level')),
    CONSTRAINT ck_delivery_queue_estado CHECK (
        status IN ('pending', 'claimed', 'applied', 'failed')
    ),
    CONSTRAINT ck_delivery_queue_tentativas CHECK (attempt_count >= 0),
    -- Cada tipo carrega só os campos dele, com o destino que o jogo sabe aplicar.
    CONSTRAINT ck_delivery_queue_campos CHECK (
        (kind = 'item' AND character_id IS NOT NULL AND item_index IS NOT NULL AND qty >= 1
            AND gold_amount IS NULL AND level_delta IS NULL)
        OR (kind = 'gold' AND account_id IS NOT NULL AND gold_amount > 0
            AND item_index IS NULL AND qty IS NULL AND level_delta IS NULL)
        OR (kind = 'level' AND character_id IS NOT NULL AND level_delta > 0
            AND item_index IS NULL AND qty IS NULL AND gold_amount IS NULL)
    ),
    -- Pendente não tem reivindicação; qualquer outro estado foi reivindicado antes.
    CONSTRAINT ck_delivery_queue_reivindicacao CHECK (
        (status <> 'pending') = (claimed_at IS NOT NULL)
    ),
    CONSTRAINT ck_delivery_queue_fechamento CHECK (
        (status IN ('applied', 'failed')) = (closed_at IS NOT NULL)
    ),
    CONSTRAINT ck_delivery_queue_motivo CHECK (
        (status = 'failed') = (failure_reason IS NOT NULL)
    ),
    CONSTRAINT ck_delivery_queue_motivo_tamanho CHECK (
        failure_reason IS NULL OR char_length(failure_reason) BETWEEN 1 AND 200
    ),
    CONSTRAINT ck_delivery_queue_ordem CHECK (closed_at IS NULL OR closed_at >= claimed_at)
);

-- O dreno lê pendentes em ordem de `id`, sem `Sort` na consulta da fila.
CREATE INDEX ix_delivery_queue_pendentes ON ops.delivery_queue (id) WHERE status = 'pending';
-- Toda FK tem índice no lado filho (`P-15`); nulo não entra no índice.
CREATE INDEX ix_delivery_queue_conta ON ops.delivery_queue (account_id)
    WHERE account_id IS NOT NULL;
CREATE INDEX ix_delivery_queue_personagem ON ops.delivery_queue (character_id)
    WHERE character_id IS NOT NULL;
CREATE INDEX ix_delivery_queue_autor ON ops.delivery_queue (created_by_person_id)
    WHERE created_by_person_id IS NOT NULL;
-- Expurgo por retenção varre só o que já fechou.
CREATE INDEX ix_delivery_queue_fechadas ON ops.delivery_queue (closed_at)
    WHERE closed_at IS NOT NULL;
CREATE INDEX ix_delivery_queue_status ON ops.delivery_queue (status, created_at DESC);

CREATE TRIGGER trg_delivery_queue_set_updated_at
    BEFORE UPDATE ON ops.delivery_queue
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- O portal enfileira e acompanha; o jogo drena e marca. Ninguém apaga em runtime (P-16).
GRANT SELECT, INSERT ON ops.delivery_queue TO wyd_portal;
GRANT SELECT, UPDATE ON ops.delivery_queue TO wyd_game;
```

```sql
-- migrate:down
DROP TRIGGER trg_delivery_queue_set_updated_at ON ops.delivery_queue;
DROP INDEX ops.ix_delivery_queue_status;
DROP INDEX ops.ix_delivery_queue_fechadas;
DROP INDEX ops.ix_delivery_queue_autor;
DROP INDEX ops.ix_delivery_queue_personagem;
DROP INDEX ops.ix_delivery_queue_conta;
DROP INDEX ops.ix_delivery_queue_pendentes;
DROP TABLE ops.delivery_queue;
```

**Chaves, índices e o que cada constraint impede.**

| Tabela | PK | FK | Índices | Constraint que importa |
|---|---|---|---|---|
| `delivery_queue` | `id` | `account_id → game.account`, `character_id → game.character`, `created_by_person_id → portal.person` (as três `RESTRICT`) | `uq_delivery_queue_idem`, `ix_delivery_queue_pendentes`, `ix_delivery_queue_conta`, `ix_delivery_queue_personagem`, `ix_delivery_queue_autor`, `ix_delivery_queue_fechadas`, `ix_delivery_queue_status` | tipo fechado; um campo por tipo; estado e carimbo coerentes; chave do pedido única |

**Permissões.** `ops` recebe `USAGE` de `wyd_game`, `wyd_portal` e `wyd_read` no `0001`, e `SELECT` por
default privilege. Este arquivo concede o resto:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_portal` | `SELECT, INSERT` | `GRANT` explícito deste arquivo |
| `wyd_game` | `SELECT, UPDATE` | `GRANT` explícito deste arquivo |
| `wyd_read` | `SELECT` | default privilege do `0001` em `ops` |
| `PUBLIC` | nada | `0001`/provisionamento |

Sem `DELETE` em runtime: expurgo é rotina de operação, com a retenção que `M25` fixar. Sem `INSERT`
para o jogo: quem escreve na fila é a web; o jogo só lê e marca.

**O que o banco não expressa.** Que a entrega aplique **uma vez** no jogo não é `CHECK`: o banco
garante a linha única e o estado, não o efeito no personagem. Quem prova é o teste de dreno com a
mesma linha reivindicada duas vezes (`M25`) e o `CREATE UNIQUE INDEX` de concessão pendente quando a
entrega vira item.

**As escolhas de forma que este lote fixa, e o que `M25` ainda decide.**

| # | Escolha de forma | O que esta proposta fixa |
|---|---|---|
| 1 | Destino por tipo | `gold` na carteira da conta; `item` e `level` no personagem; `ck_delivery_queue_campos` impede campo de outro tipo |
| 2 | Autoria | `created_by_person_id` com FK `RESTRICT` para `portal.person`; `0002` não tem FK só porque `0003` vem depois |
| 3 | Idempotência (`M17`) | `UUID` único; o reenvio devolve a linha existente, e a janela de 24 h é política do BFF |
| 4 | Ciclo de vida | `pending` → `claimed` → `applied`/`failed`, com carimbo coerente e `attempt_count` para reenvio |
| 5 | Consumo do dreno (`M25`) | `SELECT … FOR UPDATE SKIP LOCKED` em transação curta: cada canal reivindica um lote, marca `claimed`/`claimed_at` e aplica fora da transação; `ix_delivery_queue_pendentes` serve a leitura em ordem de `id` |
| 6 | Retenção (`M25`) | `applied`/`failed` expurgadas 30 dias após `closed_at`; `pending` nunca é expurgada; `ix_delivery_queue_fechadas` serve o expurgo |

**Registro da aprovação.** O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção
"Sim, aprovar como está", depois de responder as duas perguntas de `M25`: o dreno usa
`SELECT … FOR UPDATE SKIP LOCKED` numa transação curta e marca `claimed` antes de aplicar; o expurgo
remove `applied`/`failed` 30 dias depois de `closed_at` e nunca remove `pending`. Nenhuma coluna mudou
depois da proposta. `0012` passa a `concluída`, `delivery_queue` entra no total de §1 e a página de
migration já conferida contra este texto.

### 5.17 Lote `0013` — taxonomias do conteúdo: `game_system`, `game_class`, `skill_lineage`, `item_kind`, `item_effect` e `fn_fold` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as duas escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **chave natural `slug TEXT` como PK** nas quatro tabelas de look-up (`snake_case`,
    2–32 caracteres) e **`item_effect` como tabela de look-up**, com os 55 `EF_*` semeados em `0017`.
    As cinco tabelas entram na conta de §1 (54 de 96 na época; hoje 55) e a página `migrations/0013-content-taxonomy.md`
    fica `concluída` no checklist.

As cinco tabelas de look-up que o resto do schema `content` referencia — `P-08`: conjunto editável pelo
negócio é tabela, não lista dentro de `CHECK` — e a função de dobra que `0014` usa em coluna gerada.

Forma: chave natural **`slug`** em `snake_case` (2–32 caracteres) como PK — o mesmo domínio de
`game.character_lineage.lineage_code` (`ck_character_lineage_codigo`, `0008`) e o padrão de
`game.city.code` (`0011`). `P-04` proíbe `id` sintético onde a chave natural é a identidade, e quem
consome já carrega o código: `lineage_code` e o futuro `item_catalog.kind_slug` não precisam de join
para resolver o nome. Nenhuma tabela daqui tem FK de saída (`plano-migrations.md` §3.1, item 5).

`game_class` é a única com coluna de regra: `mastery_cap`, porque `0008-game-skills.md` §10 manda ler o
teto de maestria por classe em `content.game_class`. O default `320` é o teto global de
`ck_character_skill_maestria` — não restringe, e a classe só pode reduzir.

```sql
-- 0013_content_taxonomy: taxonomias do conteúdo e dobra de busca.
-- Sem FK de saída (plano-migrations.md §3.1, item 5): o schema `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
-- Dobra de busca (P-11): acento e caixa fora, pontas aparadas. IMMUTABLE porque `0014` a usa em coluna
-- gerada indexada. O lower final só vê ASCII, então o resultado não depende da collation do banco.
CREATE FUNCTION content.fn_fold(input TEXT) RETURNS text
LANGUAGE sql
IMMUTABLE
AS $$
    SELECT lower(translate(
        translate(btrim(input),
                  'ÁÀÂÃÄÅÉÈÊËÍÌÎÏÓÒÔÕÖÚÙÛÜÇÑÝ',
                  'AAAAAAEEEEIIIIOOOOOUUUUCNY'),
        'áàâãäåéèêëíìîïóòôõöúùûüçñý',
        'aaaaaaeeeeiiiiooooouuuucny'));
$$;

CREATE TABLE content.game_system (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_game_system PRIMARY KEY (slug),
    CONSTRAINT ck_game_system_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_game_system_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TABLE content.game_class (
    slug        TEXT NOT NULL,
    name        TEXT NOT NULL,
    -- Teto de maestria da classe: `0008` manda ler daqui (`0008-game-skills.md` §10). 320 é o teto
    -- global de `ck_character_skill_maestria`; o default não restringe e a classe só pode reduzir.
    mastery_cap SMALLINT NOT NULL DEFAULT 320,
    enabled     BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_game_class PRIMARY KEY (slug),
    CONSTRAINT ck_game_class_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_game_class_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_game_class_maestria CHECK (mastery_cap BETWEEN 0 AND 320)
);

CREATE TABLE content.skill_lineage (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_skill_lineage PRIMARY KEY (slug),
    CONSTRAINT ck_skill_lineage_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_skill_lineage_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TABLE content.item_kind (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_kind PRIMARY KEY (slug),
    CONSTRAINT ck_item_kind_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_item_kind_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TABLE content.item_effect (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_effect PRIMARY KEY (slug),
    CONSTRAINT ck_item_effect_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_item_effect_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TRIGGER trg_game_system_set_updated_at
    BEFORE UPDATE ON content.game_system
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_game_class_set_updated_at
    BEFORE UPDATE ON content.game_class
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_skill_lineage_set_updated_at
    BEFORE UPDATE ON content.skill_lineage
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_item_kind_set_updated_at
    BEFORE UPDATE ON content.item_kind
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_item_effect_set_updated_at
    BEFORE UPDATE ON content.item_effect
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

```sql
-- migrate:down
DROP TRIGGER trg_item_effect_set_updated_at ON content.item_effect;
DROP TABLE content.item_effect;
DROP TRIGGER trg_item_kind_set_updated_at ON content.item_kind;
DROP TABLE content.item_kind;
DROP TRIGGER trg_skill_lineage_set_updated_at ON content.skill_lineage;
DROP TABLE content.skill_lineage;
DROP TRIGGER trg_game_class_set_updated_at ON content.game_class;
DROP TABLE content.game_class;
DROP TRIGGER trg_game_system_set_updated_at ON content.game_system;
DROP TABLE content.game_system;
DROP FUNCTION content.fn_fold(text);
```

**Permissões.** Nenhum `GRANT` novo: o default privilege de `0001` em `content` dá `SELECT` a
`wyd_game`, `wyd_portal` e `wyd_read`. Escrever taxonomia é ato de release (`0016` para as quatro de
look-up e `0017` para os 55 `EF_*`, ambos gerados), não de runtime — nenhum papel de aplicação recebe
`INSERT`/`UPDATE`.

**O que este lote não fecha:** o conteúdo das cinco tabelas, o vínculo item × efeito (`0014`,
`effect_names`), o número de categoria no fio, se o cliente o tiver, e a evolução de classe. Lista
completa em `migrations/0013-content-taxonomy.md` §10.

---

### 5.18 Lote `0014` — catálogo de item: `content.item_catalog` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as três escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **só as colunas com consumidor provado** (visual e `nUnique`/`nPos`/`Extra` ficam
    fora), **`kind_slug` anulável** (`NULL` = categoria não provada) e **`item_index INTEGER` como PK**.
    A tabela entra na conta de §1 (55 de 96 na época; hoje 56) e a página `migrations/0014-content-item.md` fica
    `concluída` no checklist.

Uma tabela, 3.220 linhas no seed de `0017` (`plano-migrations.md` §3). É o alvo das FKs que `0009`,
`0011` e `0012` deixaram para trás por ordem de arquivo, e a tabela que a wydpedia do portal lê.

**O que a fonte oferece.** `STRUCT_ITEMLIST` (`deprecated/tmp/upstream/w2pp/Source/Code/Basedef.h:1162`,
lido em 2026-11-20) declara `Name`, `IndexMesh`, `IndexTexture`, `IndexVisualEffect`, `ReqLvl`,
`ReqStr`, `ReqInt`, `ReqDex`, `ReqCon`, `stEffect[MAX_STATICEFFECT]`, `Price`, `nUnique`, `nPos`,
`Extra` e `Grade`; `STRUCT_ITEM` (`:500`) declara `sIndex` e três pares efeito/valor. **Ficam fora**
`IndexMesh`, `IndexTexture` e `IndexVisualEffect` (o cliente renderiza por eles; nem o servidor nem o
portal da `1.0.0` os leem) e `nUnique`, `nPos` e `Extra` (sem semântica documentada no legado e sem
consumidor). Não são `H` de schema: o lote não inventa significado para eles, e campo fora não é
dívida — entra em migração aditiva quando um consumidor provar uso, como `plano-migrations.md` §4
permite.

**Forma.** PK natural **`item_index INTEGER`** (0–32767): o número do cliente é a identidade (`P-04`),
e `game.character_item`, `game.vault_item`, `game.item_price` e `ops.delivery_queue` já carregam
`item_index INTEGER` — com `SMALLINT` a FK futura exigiria recriar as quatro. `kind_slug` é
**anulável**: `NULL` diz "categoria não provada", que não é o mesmo que "categoria X"; preencher com
categoria sintética seria inventar dado no seed. `name_fold` é coluna gerada por `content.fn_fold`
(`0013`, `P-11`). `effect_names TEXT[]` com GIN é o vínculo que `0013` §10 aponta; `TEXT[]` não aceita
FK, e por isso `trg_item_catalog_effects` recusa símbolo fora de `content.item_effect` e efeito
repetido — sem ele o GIN indexa lixo. `enabled` existe para aposentar linha sem `DELETE`, que
quebraria `game.character_item` e `game.item_price`.

```sql
-- 0014_content_item: catálogo de itens do cliente (3.220 linhas no seed de 0017).
-- Sem FK de saída (plano-migrations.md §3.1, item 5): `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.item_catalog (
    item_index   INTEGER NOT NULL,
    name         TEXT NOT NULL,
    -- P-11: dobra única no banco, indexável. `content.fn_fold` é IMMUTABLE desde o `0013`.
    name_fold    TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    -- NULL = categoria não provada. Categoria sintética no seed seria inventar dado.
    kind_slug    TEXT,
    -- Símbolos `EF_*`. `TEXT[]` não aceita FK: quem garante o conjunto é o trigger abaixo.
    effect_names TEXT[] NOT NULL DEFAULT '{}',
    base_price   BIGINT NOT NULL DEFAULT 0,
    -- Número do cliente, não letra: a correspondência N/M/A/Le/Anct/E não está provada.
    grade        SMALLINT NOT NULL,
    req_level    SMALLINT NOT NULL DEFAULT 0,
    req_str      SMALLINT NOT NULL DEFAULT 0,
    req_int      SMALLINT NOT NULL DEFAULT 0,
    req_dex      SMALLINT NOT NULL DEFAULT 0,
    req_con      SMALLINT NOT NULL DEFAULT 0,
    -- Aposentadoria sem DELETE: linha referenciada por character_item/item_price não pode sumir.
    enabled      BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_catalog PRIMARY KEY (item_index),
    CONSTRAINT ck_item_catalog_indice CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_item_catalog_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_item_catalog_preco CHECK (base_price >= 0),
    CONSTRAINT ck_item_catalog_grade CHECK (grade >= 0),
    CONSTRAINT ck_item_catalog_requisitos CHECK (
        req_level BETWEEN 0 AND 32767 AND req_str BETWEEN 0 AND 32767
        AND req_int BETWEEN 0 AND 32767 AND req_dex BETWEEN 0 AND 32767
        AND req_con BETWEEN 0 AND 32767),
    CONSTRAINT ck_item_catalog_efeitos CHECK (array_position(effect_names, NULL) IS NULL),
    CONSTRAINT fk_item_catalog_kind FOREIGN KEY (kind_slug)
        REFERENCES content.item_kind (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Efeito repetido ou símbolo fora de `content.item_effect` não passam: sem isto o GIN indexa lixo.
CREATE FUNCTION content.fn_item_catalog_effects() RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    faltando TEXT;
BEGIN
    IF cardinality(NEW.effect_names) = 0 THEN
        RETURN NEW;
    END IF;
    IF cardinality(NEW.effect_names)
       <> (SELECT count(DISTINCT e) FROM unnest(NEW.effect_names) AS t(e)) THEN
        RAISE EXCEPTION 'item % repete efeito em effect_names', NEW.item_index
            USING ERRCODE = '23514', CONSTRAINT = 'ck_item_catalog_efeitos';
    END IF;
    SELECT e INTO faltando
      FROM unnest(NEW.effect_names) AS t(e)
     WHERE NOT EXISTS (SELECT 1 FROM content.item_effect ie WHERE ie.slug = t.e)
     LIMIT 1;
    IF FOUND THEN
        RAISE EXCEPTION 'efeito % nao existe em content.item_effect', faltando
            USING ERRCODE = '23503', CONSTRAINT = 'fk_item_catalog_efeito';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_item_catalog_effects
    BEFORE INSERT OR UPDATE OF effect_names ON content.item_catalog
    FOR EACH ROW EXECUTE FUNCTION content.fn_item_catalog_effects();

CREATE TRIGGER trg_item_catalog_set_updated_at
    BEFORE UPDATE ON content.item_catalog
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- Busca por prefixo do nome dobrado, filtro por categoria e filtro por efeito (P-11 e o GIN do plano).
CREATE INDEX ix_item_catalog_name_fold ON content.item_catalog (name_fold text_pattern_ops);
CREATE INDEX ix_item_catalog_kind ON content.item_catalog (kind_slug);
CREATE INDEX ix_item_catalog_effect_names ON content.item_catalog USING GIN (effect_names);
```

```sql
-- migrate:down
DROP TRIGGER trg_item_catalog_set_updated_at ON content.item_catalog;
DROP TRIGGER trg_item_catalog_effects ON content.item_catalog;
DROP TABLE content.item_catalog;
DROP FUNCTION content.fn_item_catalog_effects();
```

### 5.19 Lote `0015` — catálogo de skill: `content.skill_catalog` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as três escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **`delay` e `affect_time` gravados brutos** (o runtime converte), **`class_slug` e
    `lineage_slug` anuláveis** com FK (`NULL` = vínculo não provado) e **`skill_index SMALLINT` como
    PK** (0–247). A tabela entra na conta de §1 (75 de 97 na época) e a página `migrations/0015-content-skill.md`
    fica `concluída` no checklist.

Uma tabela, 154 linhas no seed de `0018` — 151 do cliente e 3 da Kersef (`plano-migrations.md` §3).
Fecha o catálogo que `0008` deixou sem FK: `game.character_skill.skill_index` é `SMALLINT` 0–247 e o
catálogo cobre só parte desses índices, então a FK continua fora por decisão, não por esquecimento.

**O que a fonte oferece.** `STRUCT_SPELL` (`deprecated/tmp/upstream/w2pp/Source/Code/Basedef.h:1110`,
lido em 2026-11-20) declara `SkillPoint`, `TargetType`, `ManaSpent`, `Delay`, `Range`,
`InstanceType/Value`, `TickType/Value`, `AffectType/Value/Time`, `Act[8]`, `InstanceAttribute`,
`TickAttribute`, `Aggressive`, `MaxTarget`, `bParty`, `AffectResist` e `Passive`; o parser de
`SkillData.csv` (`Basedef.cpp:6659`) lê 22 colunas nessa ordem, e as duas de texto caem no mesmo
buffer — só a segunda sobrevive, que é a sequência de ativação. **Fica fora** `Act[8]` (animação do
cliente; nem servidor nem portal da `1.0.0` a leem), e `name` não vem do CSV: é curadoria do seed
`0018`, porque o parser descarta o nome.

**Forma.** PK `skill_index SMALLINT` (0–247): casa com `game.character_skill.skill_index` (`0008`) e
com `MAX_SKILLINDEX 248` do cliente (`Basedef.h:200`), então a FK futura é `REFERENCES` direto.
`class_slug` e `lineage_slug` são **anuláveis**, com FK para `content.game_class` e
`content.skill_lineage` (`0013`): das 154 skills, 63 têm linhagem medida (`plano-migrations.md` §4),
e `NULL` diz "não provado" sem inventar vínculo nem bloquear o seed. `delay_ticks` e `affect_time`
ficam **brutos**: o cliente divide `AffectTime` por 4 na carga, mas isso é comportamento de runtime,
não dado do catálogo — a mesma regra do `grade` em `0014`. `aggressive`, `party` e `passive` são
`BOOLEAN` porque o CSV traz 0/1; a coluna diz o que o campo é.

```sql
-- 0015_content_skill: catálogo de skills do cliente (154 linhas no seed de 0018).
-- Sem FK de saída (plano-migrations.md §3.1, item 5): `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.skill_catalog (
    skill_index        SMALLINT NOT NULL,
    name               TEXT NOT NULL,
    -- P-11: dobra única no banco, indexável. `content.fn_fold` é IMMUTABLE desde o `0013`.
    name_fold          TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    -- NULL = vínculo não provado: das 154 skills, 63 têm linhagem medida.
    class_slug         TEXT,
    lineage_slug       TEXT,
    skill_point        SMALLINT NOT NULL DEFAULT 0,
    mana_spent         SMALLINT NOT NULL DEFAULT 0,
    -- `delay_ticks` e `affect_time` ficam brutos, como vêm do SkillData.csv. O runtime aplica a
    -- conversão que o cliente faz na carga; o catálogo não traduz (mesma regra do `grade` em `0014`).
    delay_ticks        SMALLINT NOT NULL DEFAULT 0,
    affect_time        SMALLINT NOT NULL DEFAULT 0,
    range_tiles        SMALLINT NOT NULL DEFAULT 0,
    target_type        SMALLINT NOT NULL DEFAULT 0,
    instance_type      SMALLINT NOT NULL DEFAULT 0,
    instance_value     SMALLINT NOT NULL DEFAULT 0,
    tick_type          SMALLINT NOT NULL DEFAULT 0,
    tick_value         SMALLINT NOT NULL DEFAULT 0,
    affect_type        SMALLINT NOT NULL DEFAULT 0,
    affect_value       SMALLINT NOT NULL DEFAULT 0,
    instance_attribute SMALLINT NOT NULL DEFAULT 0,
    tick_attribute     SMALLINT NOT NULL DEFAULT 0,
    -- O CSV traz 0/1; BOOLEAN diz o que o campo é. Valor fora de 0/1 seria lote novo.
    aggressive         BOOLEAN NOT NULL DEFAULT false,
    max_target         SMALLINT NOT NULL DEFAULT 1,
    party              BOOLEAN NOT NULL DEFAULT false,
    affect_resist      SMALLINT NOT NULL DEFAULT 0,
    passive            BOOLEAN NOT NULL DEFAULT false,
    -- Aposentadoria sem DELETE: linha referenciada por progressão de personagem não pode sumir.
    enabled            BOOLEAN NOT NULL DEFAULT true,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_skill_catalog PRIMARY KEY (skill_index),
    CONSTRAINT ck_skill_catalog_indice CHECK (skill_index BETWEEN 0 AND 247),
    CONSTRAINT ck_skill_catalog_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_skill_catalog_nao_negativos CHECK (
        skill_point >= 0 AND mana_spent >= 0 AND delay_ticks >= 0 AND affect_time >= 0
        AND range_tiles >= 0 AND max_target >= 0),
    CONSTRAINT fk_skill_catalog_class FOREIGN KEY (class_slug)
        REFERENCES content.game_class (slug) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_skill_catalog_lineage FOREIGN KEY (lineage_slug)
        REFERENCES content.skill_lineage (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TRIGGER trg_skill_catalog_set_updated_at
    BEFORE UPDATE ON content.skill_catalog
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- Busca por prefixo do nome dobrado e filtro por classe/linhagem (P-11 e a leitura do portal).
CREATE INDEX ix_skill_catalog_name_fold ON content.skill_catalog (name_fold text_pattern_ops);
CREATE INDEX ix_skill_catalog_afinidade ON content.skill_catalog (class_slug, lineage_slug);
```

```sql
-- migrate:down
DROP TRIGGER trg_skill_catalog_set_updated_at ON content.skill_catalog;
DROP TABLE content.skill_catalog;
```

### 5.20 Lote `0031` — segundo fator de staff: `portal.person_authenticator` (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as duas escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **`id` identity com único parcial `(person_id, kind) WHERE revoked_at IS NULL`** e
    **revogação com `revoked_at` + `secret_encrypted = NULL`**. Também aprovou o desenho completo na
    opção "Sim, aprovar o desenho como está". A tabela entra na conta de §1 (75 de 97 na época) e a página
    `migrations/0031-portal-mfa.md` fica `concluída` no checklist.

Uma tabela. Fecha a última peça de DDL que o `wyd-portal` pedia: a forma que `A20` reservou
(`person_id, kind, secret_encrypted, confirmed_at, last_used_step`) deixa de ser nome e passa a ser
schema. O mecanismo **não** é reaberto: segundo fator de staff é **TOTP**, em credencial separada de
`portal.person`, obrigatório para `admin`/`moderator` antes de o portal ir a público; `member` fica fora
da `1.0.0` (`LAC-POR-05`, `A20`). Não há coluna em `game.account` e o jogo não lê esta tabela.

**Política de segredo (a exceção declarada).** `padrao-banco.md` §3 manda segredo com sufixo `_hash` e
§4 proíbe token em claro. TOTP não aceita hash: o BFF precisa **decifrar** o segredo para calcular o
código. Por isso a coluna é `secret_encrypted BYTEA` — o nome e a intenção que `A20` reservou — e o
tipo segue a linha "binário → `BYTEA`" de `padrao-banco.md` §4. A **chave** de cifra mora no ambiente do
BFF, nunca no banco nem em `docs/`: `seguranca.md` já atribui ao portal os segredos que o `wyd-db` não lê
(`SESSION_SECRET` é o precedente). É a única coluna de segredo reversível da série; qualquer outra
continua hash.

**Forma.** PK `id` identity, como o resto da série, porque revogar não pode apagar a linha: o histórico
"este fator existiu e foi revogado em tal data" é o que responde a incidente de staff. O único parcial
`ux_person_authenticator_ativo` garante **um autenticador ativo por pessoa e tipo** e libera o próximo
cadastro depois da revogação. `kind` é conjunto fechado (`totp`) — mecanismo novo entra com decisão
própria, não com linha nova de CHECK. `confirmed_at` nulo é inscrição pendente: o fator só vale no login
depois de confirmado. `last_used_step` é o passo TOTP de 30 s já aceito, para recusar replay (`A20`).
`revoked_at` é estado explícito com carimbo (`P-20`), e o CHECK `ck_person_authenticator_segredo` amarra
segredo e revogação nas duas pontas: **ativo tem segredo, revogado não guarda segredo**. FK para
`portal.person` com `ON DELETE CASCADE` (linha possuída, `P-15`), e índice no lado filho.

**Quem lê e quem escreve.** Herda o default privilege de `0001`: `wyd_portal` tem
`SELECT, INSERT, UPDATE, DELETE`; `wyd_read` não recebe `SELECT` em `portal` por default; `wyd_game` não
alcança o schema. Quem escreve é o **BFF** do portal, coerente com `LAC-POR-01` — `wyd-admin` não lê
`portal` e não participa do segundo fator. A tela `/admin/seguranca` lê **metadados** (`kind`,
`confirmed_at`, `last_used_step`, `revoked_at`), nunca `secret_encrypted`; o valor decifrado existe só na
memória do BFF durante a validação e não entra em log.

**Retenção e revogação.** Perder o dispositivo ou desligar o fator é `UPDATE` com `revoked_at = now()`,
`secret_encrypted = NULL` e `updated_at` pelo trigger, nunca `DELETE`. Pessoa desativada
(`status = 'suspended'`) ou fechada (`status = 'closed'`) não perde a linha: o BFF recusa a sessão antes
de olhar o fator, e o fechamento de pessoa é soft-delete com anonimização (`A12`), então a FK `CASCADE`
só age em expurgo manual. Não há expurgo automático na `1.0.0`: a tabela é limitada ao staff, e prazo, se
existir, é política do `wyd-ops`.

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

```sql
-- migrate:down
DROP TRIGGER trg_person_authenticator_set_updated_at ON portal.person_authenticator;
DROP INDEX ix_person_authenticator_person;
DROP INDEX ux_person_authenticator_ativo;
DROP TABLE portal.person_authenticator;
```

**O que este lote não decide:** algoritmo e custódia da chave de cifra (implementação do BFF), UX de
inscrição e de recuperação do fator (fluxo de staff, `A20`), outros `kind` (WebAuthn volta em 1.1 com
decisão própria), prazo de expurgo de linha revogada (`wyd-ops`) e uma linha `P` para "segredo
reversível" em `padrao-banco.md` §4 — a exceção está documentada aqui, mas promover intenção nova a
regra do padrão é decisão do dono do padrão.

---

### 5.21 Lote `0019` — mundo do conteúdo: 13 tabelas (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono respondeu as seis decisões de forma em 2026-11-20, nas opções recomendadas (chave natural
    onde o insumo tem código, blob do NPC fora, array para bloco numérico, `guild_zone` só com
    definição, FK de item em duas das três tabelas e `map_id` em `world_item_seed`) e **revisou as
    colunas `H` antes de aprovar**: `merchant`, `eff1..eff3`/`effv1..effv3`, os cinco blocos crus,
    `rotate` e o grupo de gerador do NPC foram mantidos. As 13 tabelas entram na conta de §1
    (75 de 97 na época) e a página `migrations/0019-content-world.md` fica `concluída` no checklist.

As 13 tabelas de `plano-migrations.md` §3: `content_source`, `npc_template`, `npc_definition`,
`npc_shop_item`, `level_item`, `guard_spawn`, `guard_spawn_reward`, `mob_merc`, `game_setting`,
`game_matrix`, `region`, `world_item_seed` e `guild_zone`. É o maior lote da série e o primeiro bloco
de `content` sem consumidor no jogo já fechado.

**O que a fonte oferece, e o que é hipótese.** A lista de tabelas é `P` (`plano-migrations.md` §3,
linha 19). A **forma** não tem documento de coluna no planejamento novo: vem das fichas do legado em
`deprecated/.agents/prompts/análises/wyd-db/database/tabelas/` (geradas do
`0008_conteudo_mundo.sql`, nível 4), lidas em 2026-11-20 como **hipótese de conteúdo** e nunca como
verdade. Duas exceções: `content_source`, cujas colunas estão em `plano-migrations.md` §4
(`path`, `sha256`, tamanho, linhas), e `guild_zone`, citada por `0010` (`0010-game-guild.md` §11).
Por isso **toda coluna nova deste lote é `H`** — o cabeçalho de cada tabela diz de qual arquivo do
legado ela veio, e a página `0019-content-world.md` marca linha a linha. O que está provado é a
existência, o schema e a ordem; o nome de cada campo é proposta.

**Seis decisões de forma (respondidas em 2026-11-20).**
1. **Identidade:** chave natural onde o insumo tem código — `path`, `template_name`, `id` do NPC,
   `(npc_id, slot)`, `ordinal`, `guard_index`, `(guard_index, req_slot)`, `generator_index`,
   `(section, key)`, `(section, block_index, line_index)`, `(clan, zone)`. `region` e `world_item_seed`
   ganham `id BIGINT identity` + único natural, porque o arquivo não numera a linha de forma estável.
   `npc_definition.id` é `INTEGER` porque `game.item_price.npc_id` (`0011`) é `INTEGER`.
2. **Blob do NPC fora:** `npc_template` guarda `file_name`, `source_size`, `sha256`,
   `encoding_origin` e `name_recovered`; o `STRUCT_MOB` de 816 B não tem consumidor na `1.0.0`.
3. **Bloco posicional:** `SMALLINT[]` com CHECK de tamanho em `game_matrix.row_values` e
   `guard_spawn_reward.req_values` (precedente `effect_names TEXT[]` em `0014`); `JSONB` só em
   `npc_definition.generator_data`, `mob_merc.items` e `level_item.extras`, onde o dado é heterogêneo
   e `P-09` autoriza.
4. **`guild_zone` só com definição:** `clan`, `zone`, `victory`, `city_tax`. `charge_guild`,
   `challenge_guild`, `challenge_money` e `tax_vault` são estado de guerra e vão para o lote de `game`
   (`0028`); conteúdo muda por deploy, não por disputa.
5. **FK de item:** `level_item` e `world_item_seed` apontam para `content.item_catalog` (`0014`);
   `npc_shop_item` **não** tem FK, porque o insumo de loja cita item fora do catálogo e um índice
   negativo (`H`, 22 casos) — fica `CHECK (item_index > 0)` e o seed reporta o que falta.
6. **`world_item_seed` ganha `map_id`** (0–255, mesmo domínio de `game.city.map_id`) com único
   `(map_id, pos_x, pos_y, item_index)`: sem mapa o item não tem lugar, e o único dá idempotência.

**FKs.** Só para tabela já fechada ou do próprio lote: `npc_definition → npc_template`,
`npc_shop_item → npc_definition` (`CASCADE`, linha possuída), `guard_spawn → npc_definition`,
`guard_spawn_reward → guard_spawn` (`CASCADE`), `mob_merc → npc_template`, `level_item` e
`world_item_seed → item_catalog`. Nenhuma FK para `game`/`portal` (`plano-migrations.md` §3.1).
`level_item.class_id` fica sem FK porque `content.game_class` (`0013`) tem `slug`, não o código
numérico do LevelItem.txt.

**Quem lê e quem escreve.** Herda o default privilege de `0001`: `wyd_game` e `wyd_portal` leem
`content`, `wyd_read` lê, e ninguém escreve em runtime — a escrita é do seed (`0020`) e da edição de
configuração pelo portal, auditada em `ops.config_change` (`0002`). `game_setting`, `region`,
`world_item_seed`, `guild_zone` e `npc_definition` têm `enabled` e `updated_at` com trigger porque são
aposentados ou editados sem `DELETE`; as tabelas de import puro não têm carimbo.

```sql
-- 0019_content_world: mundo do cliente — proveniência, NPC/gerador, loja de NPC, item de nível,
-- guarda, mercador, configuração/matriz do gameconfig, região, item de chão e zona de guilda.
-- As 13 tabelas são as de `plano-migrations.md` §3; a forma vem das fichas de tabela do legado
-- (`deprecated/.agents/prompts/análises/wyd-db/database/tabelas/`, geradas do `0008_conteudo_mundo.sql`)
-- e é hipótese `H` até a aprovação do dono — exceto `content_source`, cujas colunas estão em
-- `plano-migrations.md` §4, e `guild_zone`, citada por `0010`.
-- FK só para tabela já fechada (`content.npc_template`, `content.npc_definition`, `content.item_catalog`);
-- `content` não aponta para `game`/`portal` (`plano-migrations.md` §3.1).
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.content_source (
    path           TEXT NOT NULL,
    sha256         TEXT NOT NULL,
    size_bytes     INTEGER NOT NULL,
    row_count      INTEGER NOT NULL,
    loaded_by      TEXT NOT NULL,
    transform      TEXT NOT NULL DEFAULT '',
    imported_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_content_source PRIMARY KEY (path),
    CONSTRAINT ck_content_source_path CHECK (char_length(btrim(path)) BETWEEN 1 AND 512),
    CONSTRAINT ck_content_source_sha256 CHECK (sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_content_source_tamanho CHECK (size_bytes > 0),
    CONSTRAINT ck_content_source_linhas CHECK (row_count >= 0),
    CONSTRAINT ck_content_source_autor CHECK (char_length(btrim(loaded_by)) BETWEEN 1 AND 64)
);

-- `blob` do STRUCT_MOB (816 B) fica fora: nenhum consumidor da 1.0.0 lê o binário, e o `sha256`
-- mais `source_size` preservam a prova do insumo. `source_size` mantém 756/920 porque 222 linhas do
-- legado declaram tamanho errado (H) — o CHECK registra o defeito em vez de escondê-lo.
CREATE TABLE content.npc_template (
    template_name     TEXT NOT NULL,
    file_name         TEXT NOT NULL,
    source_size       SMALLINT NOT NULL,
    sha256            TEXT NOT NULL,
    encoding_origin   TEXT NOT NULL DEFAULT 'utf8',
    name_recovered    TEXT,
    CONSTRAINT pk_npc_template PRIMARY KEY (template_name),
    CONSTRAINT ck_npc_template_nome CHECK (char_length(btrim(template_name)) BETWEEN 1 AND 96),
    CONSTRAINT ck_npc_template_arquivo CHECK (char_length(btrim(file_name)) BETWEEN 1 AND 128),
    CONSTRAINT ck_npc_template_tamanho CHECK (source_size IN (756, 816, 920)),
    CONSTRAINT ck_npc_template_sha256 CHECK (sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_npc_template_codificacao CHECK (encoding_origin IN ('utf8', 'cp949-as-latin1')),
    CONSTRAINT ck_npc_template_recuperado CHECK (
        name_recovered IS NULL
        OR (encoding_origin = 'cp949-as-latin1'
            AND char_length(btrim(name_recovered)) BETWEEN 1 AND 96)
    )
);

-- `id` é o código do insumo (NPCGener), não identidade gerada: `game.item_price.npc_id` é INTEGER
-- (`0011`) e a FK futura precisa casar sem remapear. O `slug` é o nome estável para o portal.
CREATE TABLE content.npc_definition (
    id                 INTEGER NOT NULL,
    slug               TEXT NOT NULL,
    template_name      TEXT NOT NULL,
    display_name       TEXT NOT NULL DEFAULT '',
    enabled            BOOLEAN NOT NULL DEFAULT true,
    map_id             SMALLINT NOT NULL DEFAULT 0,
    pos_x              SMALLINT NOT NULL DEFAULT 0,
    pos_y              SMALLINT NOT NULL DEFAULT 0,
    route_type         SMALLINT NOT NULL DEFAULT 0,
    -- Campo Merchant do NPCGener: 0 = não comerciante. A semântica acima de 0 é UNVERIFIED (H).
    merchant           SMALLINT NOT NULL DEFAULT 0,
    origin             TEXT NOT NULL DEFAULT 'content',
    generator_index    INTEGER,
    follower_template  TEXT NOT NULL DEFAULT '',
    minute_generate    INTEGER NOT NULL DEFAULT 0,
    min_group          SMALLINT NOT NULL DEFAULT 0,
    max_group          SMALLINT NOT NULL DEFAULT 0,
    max_num_mob        SMALLINT NOT NULL DEFAULT 1,
    formation          SMALLINT NOT NULL DEFAULT 0,
    -- Bloco do gerador ainda não decodificado: `P-09` manda JSONB para dado aberto/UNVERIFIED.
    generator_data     JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_npc_definition PRIMARY KEY (id),
    CONSTRAINT ck_npc_definition_id CHECK (id > 0),
    CONSTRAINT ck_npc_definition_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_npc_definition_nome CHECK (char_length(btrim(display_name)) <= 64),
    CONSTRAINT ck_npc_definition_mapa CHECK (map_id BETWEEN 0 AND 255),
    CONSTRAINT ck_npc_definition_posicao CHECK (
        pos_x BETWEEN 0 AND 32767 AND pos_y BETWEEN 0 AND 32767),
    CONSTRAINT ck_npc_definition_origem CHECK (origin IN ('content', 'custom')),
    CONSTRAINT ck_npc_definition_gerador CHECK (generator_index IS NULL OR generator_index >= 0),
    CONSTRAINT ck_npc_definition_seguidor CHECK (char_length(btrim(follower_template)) <= 96),
    CONSTRAINT ck_npc_definition_tempo CHECK (minute_generate >= 0),
    CONSTRAINT ck_npc_definition_grupo CHECK (
        min_group BETWEEN 0 AND 255 AND max_group BETWEEN 0 AND 255 AND min_group <= max_group),
    CONSTRAINT ck_npc_definition_teto CHECK (max_num_mob >= 1),
    CONSTRAINT ck_npc_definition_formacao CHECK (formation >= 0),
    CONSTRAINT ck_npc_definition_payload CHECK (jsonb_typeof(generator_data) = 'object'),
    CONSTRAINT fk_npc_definition_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Sem FK de `item_index`: o insumo de loja cita índice que o catálogo do cliente não tem, e um deles
-- é negativo (H, 22 casos). `CHECK (item_index > 0)` recusa o negativo e o seed de `0020` reporta o
-- que falta, em vez de o lote falhar na carga.
CREATE TABLE content.npc_shop_item (
    npc_id      INTEGER NOT NULL,
    slot        SMALLINT NOT NULL,
    item_index  INTEGER NOT NULL,
    eff1        SMALLINT NOT NULL DEFAULT 0,
    effv1       SMALLINT NOT NULL DEFAULT 0,
    eff2        SMALLINT NOT NULL DEFAULT 0,
    effv2       SMALLINT NOT NULL DEFAULT 0,
    eff3        SMALLINT NOT NULL DEFAULT 0,
    effv3       SMALLINT NOT NULL DEFAULT 0,
    quantity    SMALLINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_npc_shop_item PRIMARY KEY (npc_id, slot),
    CONSTRAINT ck_npc_shop_item_slot CHECK (slot BETWEEN 0 AND 26),
    CONSTRAINT ck_npc_shop_item_item CHECK (item_index > 0),
    CONSTRAINT ck_npc_shop_item_quantidade CHECK (quantity BETWEEN 1 AND 255),
    CONSTRAINT fk_npc_shop_item_npc FOREIGN KEY (npc_id)
        REFERENCES content.npc_definition (id) ON UPDATE RESTRICT ON DELETE CASCADE
);

-- `class_id` 0–3 fica sem FK: `content.game_class` (`0013`) tem `slug`, não o código numérico do
-- LevelItem.txt. A correspondência código → slug é curadoria do seed de `0017`/`0018` (`H`).
CREATE TABLE content.level_item (
    ordinal      SMALLINT NOT NULL,
    source_seq   SMALLINT NOT NULL,
    source_line  INTEGER NOT NULL,
    level        SMALLINT NOT NULL,
    class_id     SMALLINT NOT NULL,
    -- TYPE do LevelItem.txt: -1 independente de classe, 0 independente de pontos, 1 STR, 2 INT, 3 DEX.
    type_id      SMALLINT NOT NULL,
    item_index   INTEGER NOT NULL,
    extras       JSONB NOT NULL DEFAULT '[]'::jsonb,
    CONSTRAINT pk_level_item PRIMARY KEY (ordinal),
    CONSTRAINT ck_level_item_ordinal CHECK (ordinal > 0),
    CONSTRAINT ck_level_item_seq CHECK (source_seq >= 0),
    CONSTRAINT ck_level_item_origem CHECK (source_line > 0),
    CONSTRAINT ck_level_item_nivel CHECK (level BETWEEN 1 AND 255),
    CONSTRAINT ck_level_item_classe CHECK (class_id BETWEEN 0 AND 3),
    CONSTRAINT ck_level_item_tipo CHECK (type_id BETWEEN -1 AND 3),
    CONSTRAINT ck_level_item_item CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_level_item_extras CHECK (jsonb_typeof(extras) = 'array'),
    CONSTRAINT fk_level_item_catalogo FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Guard.txt: o NPC guardião e a exigência de itens. O bloco de 8 números por exigência não tem
-- semântica provada; fica como array posicional, com tamanho fixo, sem nome inventado.
CREATE TABLE content.guard_spawn (
    guard_index  SMALLINT NOT NULL,
    npc_id       INTEGER NOT NULL,
    source_line  INTEGER NOT NULL,
    CONSTRAINT pk_guard_spawn PRIMARY KEY (guard_index),
    CONSTRAINT ck_guard_spawn_indice CHECK (guard_index >= 0),
    CONSTRAINT ck_guard_spawn_origem CHECK (source_line > 0),
    CONSTRAINT fk_guard_spawn_npc FOREIGN KEY (npc_id)
        REFERENCES content.npc_definition (id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE content.guard_spawn_reward (
    guard_index  SMALLINT NOT NULL,
    req_slot     SMALLINT NOT NULL,
    req_values   SMALLINT[] NOT NULL,
    CONSTRAINT pk_guard_spawn_reward PRIMARY KEY (guard_index, req_slot),
    CONSTRAINT ck_guard_spawn_reward_slot CHECK (req_slot BETWEEN 0 AND 7),
    CONSTRAINT ck_guard_spawn_reward_valores CHECK (
        array_length(req_values, 1) = 8 AND array_position(req_values, NULL) IS NULL),
    CONSTRAINT fk_guard_spawn_reward_guarda FOREIGN KEY (guard_index)
        REFERENCES content.guard_spawn (guard_index) ON UPDATE RESTRICT ON DELETE CASCADE
);

CREATE TABLE content.mob_merc (
    generator_index  INTEGER NOT NULL,
    template_name    TEXT NOT NULL,
    renew_seconds    INTEGER NOT NULL,
    reborn_seconds   INTEGER NOT NULL,
    -- Lista de itens do mercador ainda não normalizada; `P-09` admite JSONB para dado UNVERIFIED.
    items            JSONB NOT NULL DEFAULT '[]'::jsonb,
    CONSTRAINT pk_mob_merc PRIMARY KEY (generator_index),
    CONSTRAINT ck_mob_merc_indice CHECK (generator_index >= 0),
    CONSTRAINT ck_mob_merc_renovacao CHECK (renew_seconds >= 0),
    CONSTRAINT ck_mob_merc_renascimento CHECK (reborn_seconds >= 0),
    CONSTRAINT ck_mob_merc_itens CHECK (jsonb_typeof(items) = 'array'),
    CONSTRAINT fk_mob_merc_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Configuração efetiva do servidor (gameconfig.txt). O portal edita esta tabela pelo dono de
-- `config_revision`/`config_change` (`0002`), e o jogo lê no boot; por isso tem carimbo e trigger.
CREATE TABLE content.game_setting (
    section      TEXT NOT NULL,
    key          TEXT NOT NULL,
    -- `evstart` tem 4 números: fica em `value_text`; `value_int` só existe com inteiro único (H).
    value_int    BIGINT,
    value_text   TEXT NOT NULL DEFAULT '',
    source_line  INTEGER NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_game_setting PRIMARY KEY (section, key),
    CONSTRAINT ck_game_setting_secao CHECK (char_length(btrim(section)) BETWEEN 1 AND 64),
    CONSTRAINT ck_game_setting_chave CHECK (char_length(btrim(key)) BETWEEN 1 AND 64),
    CONSTRAINT ck_game_setting_origem CHECK (source_line > 0)
);

-- Matrizes do gameconfig (Treasure, Item Drop Bonus) na ordem do arquivo. Não há nome de coluna para
-- cada posição; o array preserva a ordem e o CHECK garante que a linha não é vazia.
CREATE TABLE content.game_matrix (
    section       TEXT NOT NULL,
    block_index   SMALLINT NOT NULL,
    line_index    SMALLINT NOT NULL,
    row_values    SMALLINT[] NOT NULL,
    source_line   INTEGER NOT NULL,
    CONSTRAINT pk_game_matrix PRIMARY KEY (section, block_index, line_index),
    CONSTRAINT ck_game_matrix_secao CHECK (char_length(btrim(section)) BETWEEN 1 AND 64),
    CONSTRAINT ck_game_matrix_bloco CHECK (block_index >= 0),
    CONSTRAINT ck_game_matrix_linha CHECK (line_index >= 0),
    CONSTRAINT ck_game_matrix_valores CHECK (
        array_length(row_values, 1) >= 1 AND array_position(row_values, NULL) IS NULL),
    CONSTRAINT ck_game_matrix_origem CHECK (source_line > 0)
);

-- Regiões de PvP/siege do Regions.txt. Nome repetido é legítimo (Erion/Armia com retângulos
-- diferentes), por isso o único é o retângulo inteiro, não o nome. `id` é identidade porque o insumo
-- não numera a região de forma estável.
CREATE TABLE content.region (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    name         TEXT NOT NULL,
    name_fold    TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    x1           INTEGER NOT NULL,
    y1           INTEGER NOT NULL,
    x2           INTEGER NOT NULL,
    y2           INTEGER NOT NULL,
    source_line  INTEGER NOT NULL,
    enabled      BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_region PRIMARY KEY (id),
    CONSTRAINT uq_region_retangulo UNIQUE (name, x1, y1, x2, y2),
    CONSTRAINT ck_region_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_region_retangulo CHECK (x2 >= x1 AND y2 >= y1),
    CONSTRAINT ck_region_origem CHECK (source_line > 0)
);

-- Itens no chão por mapa. O legado não tinha a coluna de mapa; sem ela o item não tem lugar, então
-- `map_id` entra com o mesmo domínio de `game.city.map_id` (`0011`). O único dá idempotência ao seed.
CREATE TABLE content.world_item_seed (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    map_id      SMALLINT NOT NULL,
    item_index  INTEGER NOT NULL,
    pos_x       SMALLINT NOT NULL,
    pos_y       SMALLINT NOT NULL,
    rotate      SMALLINT NOT NULL DEFAULT 0,
    enabled     BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_world_item_seed PRIMARY KEY (id),
    CONSTRAINT uq_world_item_seed_posicao UNIQUE (map_id, pos_x, pos_y, item_index),
    CONSTRAINT ck_world_item_seed_mapa CHECK (map_id BETWEEN 0 AND 255),
    CONSTRAINT ck_world_item_seed_item CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_world_item_seed_posicao CHECK (
        pos_x BETWEEN 0 AND 32767 AND pos_y BETWEEN 0 AND 32767),
    CONSTRAINT ck_world_item_seed_rotacao CHECK (rotate >= 0),
    CONSTRAINT fk_world_item_seed_catalogo FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Zona de guilda: só a definição. `charge_guild`, `challenge_guild`, `challenge_money` e `tax_vault`
-- são estado mutável de guerra e vão para o lote de `game` (`0028`); conteúdo muda por deploy, não por
-- disputa. `clan` é o código de clã do cliente, não FK para `game.guild`.
CREATE TABLE content.guild_zone (
    clan        SMALLINT NOT NULL,
    zone        SMALLINT NOT NULL,
    victory     SMALLINT NOT NULL DEFAULT 0,
    city_tax    SMALLINT NOT NULL DEFAULT 0,
    enabled     BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_zone PRIMARY KEY (clan, zone),
    CONSTRAINT ck_guild_zone_clan CHECK (clan >= 0),
    CONSTRAINT ck_guild_zone_zona CHECK (zone >= 0),
    CONSTRAINT ck_guild_zone_vitoria CHECK (victory >= 0),
    CONSTRAINT ck_guild_zone_imposto CHECK (city_tax >= 0)
);

-- Um NPC por posição de gerador no mapa; gerador sem índice não entra no único.
CREATE UNIQUE INDEX ux_npc_definition_gerador
    ON content.npc_definition (map_id, generator_index)
    WHERE generator_index IS NOT NULL;
CREATE INDEX ix_npc_definition_mapa ON content.npc_definition (map_id);
CREATE INDEX ix_npc_definition_template ON content.npc_definition (template_name);
CREATE INDEX ix_guard_spawn_npc ON content.guard_spawn (npc_id);
-- Leitura do level-up: itens por classe e nível.
CREATE INDEX ix_level_item_classe ON content.level_item (class_id, level);
-- Busca por prefixo do nome dobrado da região (P-11).
CREATE INDEX ix_region_name_fold ON content.region (name_fold text_pattern_ops);

CREATE TRIGGER trg_npc_definition_set_updated_at
    BEFORE UPDATE ON content.npc_definition
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
CREATE TRIGGER trg_game_setting_set_updated_at
    BEFORE UPDATE ON content.game_setting
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
CREATE TRIGGER trg_region_set_updated_at
    BEFORE UPDATE ON content.region
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
CREATE TRIGGER trg_world_item_seed_set_updated_at
    BEFORE UPDATE ON content.world_item_seed
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
CREATE TRIGGER trg_guild_zone_set_updated_at
    BEFORE UPDATE ON content.guild_zone
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

```sql
-- migrate:down
DROP TRIGGER trg_guild_zone_set_updated_at ON content.guild_zone;
DROP TRIGGER trg_world_item_seed_set_updated_at ON content.world_item_seed;
DROP TRIGGER trg_region_set_updated_at ON content.region;
DROP TRIGGER trg_game_setting_set_updated_at ON content.game_setting;
DROP TRIGGER trg_npc_definition_set_updated_at ON content.npc_definition;
DROP TABLE content.guild_zone;
DROP TABLE content.world_item_seed;
DROP TABLE content.region;
DROP TABLE content.game_matrix;
DROP TABLE content.game_setting;
DROP TABLE content.mob_merc;
DROP TABLE content.guard_spawn_reward;
DROP TABLE content.guard_spawn;
DROP TABLE content.level_item;
DROP TABLE content.npc_shop_item;
DROP TABLE content.npc_definition;
DROP TABLE content.npc_template;
DROP TABLE content.content_source;
```

**O que este lote não decide:** a semântica dos blocos UNVERIFIED (`merchant` acima de 0,
`generator_data`, `req_values`, `items`, `extras`) continua `H` e só fecha com trace do servidor; a
normalização de `mob_merc.items` em tabela filha é lote futuro, não este; o estado de guerra de
`guild_zone` está em `0028`; e o seed de `0020` segue `bloqueada` pela verificação do dado de origem
(`plano-migrations.md` §4).

---

### 5.22 Lote `0021` — regras de refino e composição (DDL confirmado em 2026-11-20)

!!! note "Aprovado em 2026-11-20"
    O dono respondeu as decisões de forma e deu o "sim" ao conjunto em 2026-11-20. As cinco tabelas
    entram na conta de §1 (75 de 97 na época) e a página `migrations/0021-content-rules.md` fica `concluída` no
    checklist. As colunas seguem `H` até trace do servidor: aprovar o desenho não as promove a `P`.

O escopo (cinco nomes e ordem) é `plano-migrations.md` §3, linha 21: `refine_rate`,
`refine_gem_effect`, `sanc_rate`, `combine_value` e `accessory_refine_rule`. As quatro primeiras fichas
são fontes do legado para `0009_refino.sql`; `accessory_refine_rule` vem de `0037_config_editavel.sql`
mais regras Bahamut/Kersef. São nível 4: hipótese de conteúdo, não decisão. O dado novo do plano sobre
o acessório explicita que é posterior ao cliente 7662 e não tem regra oficial confirmada.

**Forma e contratos.** PKs naturais, sem identidade: `(kind, step)`, `item_index`,
`(powder, refine_index)`, `(family, key)`, `(kind, recipe, step_from)`. `rate_bps INTEGER` para taxas e
`coin BIGINT` para custo em moeda (`padrao-banco.md` §4); `combine_value` exige exatamente um dos dois.
A falta de taxa em +11 é ausência de linha, nunca `0%`. `sanc_rate` só recebe células com fonte
explícita; defaults compilados não são seed. O `effect_code` reusa `content.item_effect.slug` de `0013`,
e o item da gema, bônus de falha e tabelas da série apontam a `content.item_catalog` de `0014` com
`RESTRICT`. `0022` mantém os overrides das três tabelas que já planeja; este lote é base seed-only,
read-only em runtime, e `0023` é o seed posterior.

**Índices.** As PKs cobrem as chaves de busca naturais das outras FKs. O único secundário é parcial em
`accessory_refine_rule.fail_bonus_item`, FK nullable não coberta pela PK. Sem índice preventivo para
`effect_code`: `item_effect` é lookup somente-leitura e a leitura primária é pela gema (`P-15`, §6).

**H explícitas.** Nomes, intervalos e significado de cada coluna da fonte antiga são `H` até trace/
curadoria próprios do novo planejamento. Em especial: família e semântica de `refine_rate.step`; se o
mapa antigo de `effect_code` coincide com todo slug habilitado de `0013`; significado/exaustividade das
células de `sanc_rate`; famílias/chaves e associação taxa/custo de `combine_value`; receitas, transições,
falha, bônus e materiais de `accessory_refine_rule` (origem posterior ao 7662). Não copiamos `confidence`,
`content_source` nem `notes` por linha: proveniência do seed pertence a `content.content_source` (`0019`),
`H` fica na documentação, e edição viva exige override/auditoria próprios.

Sem `COMMENT ON`, como toda a série; a semântica fica nesta página.

```sql
-- 0021_content_rules: regras base de refino, SANC, composição, gemas e acessórios.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabelas base são seed-only e read-only em runtime; overrides das três tabelas previstas ficam em 0022.

-- migrate:up
CREATE TABLE content.refine_rate (
    kind      TEXT NOT NULL,
    step      SMALLINT NOT NULL,
    rate_bps  INTEGER NOT NULL,
    CONSTRAINT pk_refine_rate PRIMARY KEY (kind, step),
    CONSTRAINT ck_refine_rate_kind CHECK (kind IN ('dust', 'compose')),
    CONSTRAINT ck_refine_rate_step CHECK (step BETWEEN 1 AND 15),
    CONSTRAINT ck_refine_rate_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- effect_code references the already-closed editable lookup (0013); item_index is the gem's natural key.
CREATE TABLE content.refine_gem_effect (
    item_index       INTEGER NOT NULL,
    effect_code      TEXT NOT NULL,
    flat_value       SMALLINT NOT NULL DEFAULT 0,
    per_level_value  SMALLINT NOT NULL DEFAULT 0,
    min_refine       SMALLINT NOT NULL DEFAULT 10,
    max_refine       SMALLINT NOT NULL DEFAULT 15,
    allows_ancient   BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_refine_gem_effect PRIMARY KEY (item_index),
    CONSTRAINT ck_refine_gem_effect_item CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_refine_gem_effect_valores CHECK (flat_value >= 0 AND per_level_value >= 0),
    CONSTRAINT ck_refine_gem_effect_faixa CHECK (
        min_refine BETWEEN 0 AND 15 AND max_refine BETWEEN min_refine AND 15),
    CONSTRAINT fk_refine_gem_effect_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_refine_gem_effect_codigo FOREIGN KEY (effect_code)
        REFERENCES content.item_effect (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE content.sanc_rate (
    powder        TEXT NOT NULL,
    refine_index  SMALLINT NOT NULL,
    rate_bps      INTEGER NOT NULL,
    CONSTRAINT pk_sanc_rate PRIMARY KEY (powder, refine_index),
    CONSTRAINT ck_sanc_rate_powder CHECK (powder IN ('PO', 'PL', 'AMAGO')),
    CONSTRAINT ck_sanc_rate_index CHECK (refine_index BETWEEN 0 AND 15),
    CONSTRAINT ck_sanc_rate_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- Exactly one typed value is present: rates are bps, coin amounts are BIGINT minor units.
CREATE TABLE content.combine_value (
    family    TEXT NOT NULL,
    key       TEXT NOT NULL,
    rate_bps  INTEGER,
    coin      BIGINT,
    CONSTRAINT pk_combine_value PRIMARY KEY (family, key),
    CONSTRAINT ck_combine_value_family CHECK (char_length(btrim(family)) BETWEEN 1 AND 64),
    CONSTRAINT ck_combine_value_key CHECK (char_length(btrim(key)) BETWEEN 1 AND 96),
    CONSTRAINT ck_combine_value_one_unit CHECK (
        (rate_bps IS NOT NULL AND coin IS NULL AND rate_bps BETWEEN 0 AND 10000)
        OR (rate_bps IS NULL AND coin IS NOT NULL AND coin >= 0)
    )
);

CREATE TABLE content.accessory_refine_rule (
    kind              TEXT NOT NULL,
    recipe            TEXT NOT NULL,
    step_from         SMALLINT NOT NULL,
    step_to           SMALLINT NOT NULL,
    rate_bps          INTEGER NOT NULL,
    fail_mode         TEXT NOT NULL,
    fail_bonus_item   INTEGER,
    fail_bonus_bps    INTEGER NOT NULL DEFAULT 0,
    gold_cost         BIGINT NOT NULL DEFAULT 0,
    material_item     INTEGER,
    material_name     TEXT NOT NULL DEFAULT '',
    material_qty      SMALLINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_accessory_refine_rule PRIMARY KEY (kind, recipe, step_from),
    CONSTRAINT ck_accessory_refine_rule_kind CHECK (
        kind IN ('earring', 'necklace', 'belt', 'amulet', 'ankh', 'spirit_stone')),
    CONSTRAINT ck_accessory_refine_rule_recipe CHECK (char_length(btrim(recipe)) BETWEEN 1 AND 96),
    CONSTRAINT ck_accessory_refine_rule_step CHECK (
        step_from BETWEEN 0 AND 14 AND step_to = step_from + 1),
    CONSTRAINT ck_accessory_refine_rule_rate CHECK (rate_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_accessory_refine_rule_fail CHECK (fail_mode IN ('keep', 'reset', 'destroy')),
    CONSTRAINT ck_accessory_refine_rule_bonus CHECK (
        fail_bonus_bps BETWEEN 0 AND 10000 AND
        (fail_bonus_item IS NOT NULL OR fail_bonus_bps = 0)),
    CONSTRAINT ck_accessory_refine_rule_gold CHECK (gold_cost >= 0),
    CONSTRAINT ck_accessory_refine_rule_material CHECK (
        material_item IS NULL OR material_item BETWEEN 0 AND 32767),
    CONSTRAINT ck_accessory_refine_rule_material_name CHECK (char_length(material_name) <= 96),
    CONSTRAINT ck_accessory_refine_rule_material_qty CHECK (material_qty > 0),
    CONSTRAINT fk_accessory_refine_rule_bonus FOREIGN KEY (fail_bonus_item)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- P-15: index the nullable child FK. PKs cover the other FK access paths; item_effect is lookup-only.
CREATE INDEX ix_accessory_refine_rule_fail_bonus_item
    ON content.accessory_refine_rule (fail_bonus_item)
    WHERE fail_bonus_item IS NOT NULL;
```

```sql
-- migrate:down
DROP INDEX content.ix_accessory_refine_rule_fail_bonus_item;
DROP TABLE content.accessory_refine_rule;
DROP TABLE content.combine_value;
DROP TABLE content.sanc_rate;
DROP TABLE content.refine_gem_effect;
DROP TABLE content.refine_rate;
```

**O que não decide:** conteúdo efetivo dos seeds (`0023`), tratamento de falta de linha pelo runtime,
valores exatos das taxas e receitas — todos exigem fonte verificada; edição base em runtime (não prevista);
qualquer política de acessório diferente do Bahamut (sem prova no cliente 7662).

---

### 5.23 Lote `0022` — overrides de refino, SANC e composição (DDL confirmado em 2026-11-21)

**Aprovado em 2026-11-21.** O dono aprovou o conjunto como apresentado ("Sim, aprovar como está"),
sobre as seis decisões de forma registradas na página `migrations/0022-content-overrides.md` §8. As
três tabelas contam em §1, o lote está `concluída` em `migrations-checklist.md` §2 e a página já
estava escrita e conferida.

O escopo (três nomes e ordem) é `plano-migrations.md` §3, linha 22: `refine_rate_override`,
`sanc_rate_override` e `combine_value_override`. As fichas vêm de `0037_config_editavel.sql` no legado,
a mesma origem do acessório de §5.22: são nível 4, hipótese de conteúdo, não decisão. As bases foram
fechadas em 2026-11-20 (`ddl-confirmado.md` §5.22), e o `0002` (§5.9) fechou `ops.config_revision` e
`ops.config_change`, que é onde a autoria e o antes/depois da edição vivem.

**Forma e contratos.** PK natural de cada override é exatamente a PK da base (`kind, step`;
`powder, refine_index`; `family, key`), e a FK composta aponta para ela com `ON DELETE CASCADE`:
override de chave inexistente é impossível, e remover a linha base leva a edição junto, sem `COALESCE`
sem origem. A substituição é por linha inteira: uma linha de override vale no lugar da base para sua
chave. `updated_at` é o único carimbo da linha, porque o runtime do jogo lê `content` e precisa de um
marcador barato; quem editou e o valor anterior ficam em `ops.config_change`, então não há
`updated_by` — não existe FK possível de `content` para `portal`. SANC repete `rate_bps INTEGER`
0–10000, e `combine_value_override` repete `rate_bps`/`coin` com CHECK de exatamente um, para o
`COALESCE` não mudar de unidade nem de tipo entre base e edição. Reset é `DELETE` da linha, sem
tombstone: o histórico está em `ops`.

**Índices.** Nenhum, e nenhum é preciso: as PKs são as próprias chaves das FKs compostas, e não há
coluna de busca secundária. Índice preventivo em `updated_at` não entra: o loader verifica a tabela,
não faz varredura por faixa de tempo.

**Privilégios.** `0001` concede só `SELECT` em `content` por default, e o `0021` fechou como
seed-only/read-only. A área administrativa do portal é o único runtime que edita configuração
(`M21`, §5.9), então este lote concede `SELECT, INSERT, UPDATE, DELETE` em cada override a
`wyd_portal`; o `down` revoga antes de derrubar as tabelas. `wyd_game` e `wyd_read` continuam com o
`SELECT` herdado do default, sem privilégio novo.

**H explícitas.** Permanecem `H`: por que o staff edita cada família e com que frequência; se o legado
editava `percent` e o novo lê bps (a conversão é do contrato de leitura, não dado copiado); famílias e
chaves de `combine_value_override`; e a semântica de `notes`/`content_source` do legado, que não são
copiadas — proveniência do seed pertence a `content.content_source` (`0019`) e a edição viva a `ops`.
Como em §5.22, aprovar o desenho não promove nenhuma dessas colunas a `P`.

Sem `COMMENT ON`, como toda a série; a semântica fica nesta página e na página do lote.

```sql
-- 0022_content_overrides: edição do staff sobre refino, SANC e composição.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Override sem linha base é impossível; apagar a linha de override é o reset para o valor da base.

-- migrate:up
CREATE TABLE content.refine_rate_override (
    kind       TEXT NOT NULL,
    step       SMALLINT NOT NULL,
    rate_bps   INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_refine_rate_override PRIMARY KEY (kind, step),
    CONSTRAINT fk_refine_rate_override_base FOREIGN KEY (kind, step)
        REFERENCES content.refine_rate (kind, step) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_refine_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

CREATE TABLE content.sanc_rate_override (
    powder       TEXT NOT NULL,
    refine_index SMALLINT NOT NULL,
    rate_bps     INTEGER NOT NULL,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_sanc_rate_override PRIMARY KEY (powder, refine_index),
    CONSTRAINT fk_sanc_rate_override_base FOREIGN KEY (powder, refine_index)
        REFERENCES content.sanc_rate (powder, refine_index) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_sanc_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- Same typed value as the base table: exactly one of rate_bps/coin, checked here too (FK does not
-- constrain values). Per-row replacement: a row here replaces the whole base row for its key.
CREATE TABLE content.combine_value_override (
    family     TEXT NOT NULL,
    key        TEXT NOT NULL,
    rate_bps   INTEGER,
    coin       BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_combine_value_override PRIMARY KEY (family, key),
    CONSTRAINT fk_combine_value_override_base FOREIGN KEY (family, key)
        REFERENCES content.combine_value (family, key) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_combine_value_override_one_unit CHECK (
        (rate_bps IS NOT NULL AND coin IS NULL AND rate_bps BETWEEN 0 AND 10000)
        OR (rate_bps IS NULL AND coin IS NOT NULL AND coin >= 0)
    )
);

-- 0001 grants only SELECT on `content`. The portal admin area is the only runtime that edits
-- configuration (M21, §5.9): DELETE here is the reset to the base value.
GRANT SELECT, INSERT, UPDATE, DELETE ON content.refine_rate_override TO wyd_portal;
GRANT SELECT, INSERT, UPDATE, DELETE ON content.sanc_rate_override TO wyd_portal;
GRANT SELECT, INSERT, UPDATE, DELETE ON content.combine_value_override TO wyd_portal;
```

```sql
-- migrate:down
REVOKE SELECT, INSERT, UPDATE, DELETE ON content.combine_value_override FROM wyd_portal;
REVOKE SELECT, INSERT, UPDATE, DELETE ON content.sanc_rate_override FROM wyd_portal;
REVOKE SELECT, INSERT, UPDATE, DELETE ON content.refine_rate_override FROM wyd_portal;
DROP TABLE content.combine_value_override;
DROP TABLE content.sanc_rate_override;
DROP TABLE content.refine_rate_override;
```

**O que não decide:** quais linhas o staff edita em produção; o formato de `target_key` em
`ops.config_change` para chave composta (`kind|step` versus outra convenção); o `COALESCE` no
runtime; os valores dos seeds (`0023`); e se `0025` reusa este mesmo padrão para
`drop_rate_override`/`content_rate_override` — lote próprio, decisão própria.

### 5.24 Lote `0024` — `map_definition` e `teleport_route` (DDL confirmado em 2026-11-21)

!!! note "Aprovado em 2026-11-21"
    O dono respondeu as seis decisões de forma e deu o "sim" ao conjunto em 2026-11-21, na opção
    **"Sim, aprovar como está"**. As duas tabelas entram na conta de §1 e o `0024` fica
    `concluída` em `migrations-checklist.md` §2. Aprovar o desenho não promove a `P` o que a ficha
    deixou em `H`.

O escopo — dois nomes e a ordem — é `plano-migrations.md` §3, linha 24: `map_definition` e
`teleport_route`. As duas fichas vêm de `0011_rotas_mundo.sql` do legado (nível 4): dão coluna e faixa,
não decisão. O planejamento novo decide a forma; a semântica histórica que ninguém reverificou
continua `H`.

**Por que `content`.** Mapa e rota são dado lido no boot do servidor, não estado de personagem: quem se
move é `game.character`; para onde se move é `content`. `content.region` fechou em §5.21 e é o dono
único do retângulo — `map_definition` só aponta para ela, sem repetir coordenada.

**Chaves.** `map_id` é a chave natural do cliente e vira a PK sem identidade (`P-04`: sem `id`
sintético onde a chave natural é a identidade), como `item_index` em §5.18 e `skill_index` em §5.19.
`teleport_route` não tem chave natural: o mesmo par de coordenadas pode ser oferecido por mais de um
NPC, então a linha leva `id BIGINT GENERATED ALWAYS AS IDENTITY` (`P-03`) e um único sobre a rota
inteira — origem, destino e NPC — com `NULLS NOT DISTINCT`, para que a rota sem NPC também não
duplique.

**Forma da rota.** Coordenada explícita origem→destino, como exige o "bug clássico de teleporte" de
`mundo.md` §5.1: o destino é um ponto, nunca o centro de uma região — Armia e Erion têm dois retângulos
cada, e o primeiro deles colocaria o jogador no lugar errado.

**Moeda do custo.** `coin` é o ouro **do personagem** (`game.character.coin`, `STRUCT_MOB.Coin`), o
saldo que o jogador gasta andando pelo mundo. A carteira da conta é outra coisa:
`game.account_wallet.coin` (§5.2), com ledger próprio (`M16`), não é debitada por teleporte. A rota
guarda o preço e não referencia carteira nenhuma; quem debita é o runtime, contra a linha do
personagem.

**Sem carimbo nem procedência por linha.** Tabela base seed-only e read-only em runtime, como as cinco
de §5.22: `created_at`/`updated_at` não entram, e `content_source`/`confidence` da ficha não são
copiados — proveniência de dataset é `content.content_source` (`0019`) e `H` fica nesta página.
`region` e `npc_definition` mantêm carimbo porque fecharam em §5.21 com ele; este lote segue o
precedente mais novo de conteúdo base.

```sql
-- 0024_content_maps: definição de mapa e rota de teleporte por coordenada.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabela base seed-only e read-only em runtime: mapa e rota são dado, não estado de personagem.

-- migrate:up
-- Chave natural do cliente: o id do mapa é o que o STRUCT_MOB já carrega. Sem identidade (P-04).
CREATE TABLE content.map_definition (
    map_id     SMALLINT NOT NULL,
    name       TEXT NOT NULL,
    -- Região é retângulo em `content.region` (0019); nula enquanto o mapa não tiver retângulo.
    region_id  BIGINT,
    is_pvp     BOOLEAN NOT NULL DEFAULT false,
    is_safe    BOOLEAN NOT NULL DEFAULT false,
    is_dungeon BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_map_definition PRIMARY KEY (map_id),
    CONSTRAINT fk_map_definition_region FOREIGN KEY (region_id)
        REFERENCES content.region (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_map_definition_id CHECK (map_id >= 0),
    CONSTRAINT ck_map_definition_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

-- P-15: FK nullable indexada. A leitura quente é por retângulo; a busca por mapa é a PK.
CREATE INDEX ix_map_definition_region ON content.map_definition (region_id)
    WHERE region_id IS NOT NULL;

-- Coordenada explícita origem→destino: destino é ponto, nunca "centro da região" (mundo.md §5.1).
CREATE TABLE content.teleport_route (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    from_map_id    SMALLINT NOT NULL,
    from_x         SMALLINT NOT NULL,
    from_y         SMALLINT NOT NULL,
    to_map_id      SMALLINT NOT NULL,
    to_x           SMALLINT NOT NULL,
    to_y           SMALLINT NOT NULL,
    -- NPC que oferece a rota; nulo quando a rota é do mapa (zona de teleporte, portal).
    npc_id         INTEGER,
    -- Preço em ouro do PERSONAGEM (`game.character.coin`), nunca a carteira da conta.
    coin           BIGINT NOT NULL DEFAULT 0,
    req_level      SMALLINT NOT NULL DEFAULT 1,
    -- Item exigido (pergaminho, pedido de caça): índice do catálogo, não preço.
    req_item_index INTEGER,
    enabled        BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_teleport_route PRIMARY KEY (id),
    -- Dois NPCs podem oferecer o mesmo par; o mesmo NPC não repete a rota.
    CONSTRAINT uq_teleport_route_rota UNIQUE NULLS NOT DISTINCT (
        from_map_id, from_x, from_y, to_map_id, to_x, to_y, npc_id),
    CONSTRAINT fk_teleport_route_origem FOREIGN KEY (from_map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_teleport_route_destino FOREIGN KEY (to_map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    -- Apagar o NPC desliga a rota em vez de derrubar o mapa: o destino continua válido.
    CONSTRAINT fk_teleport_route_npc FOREIGN KEY (npc_id)
        REFERENCES content.npc_definition (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_teleport_route_item FOREIGN KEY (req_item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_teleport_route_custo CHECK (coin >= 0),
    CONSTRAINT ck_teleport_route_nivel CHECK (req_level >= 1),
    CONSTRAINT ck_teleport_route_item CHECK (
        req_item_index IS NULL OR req_item_index BETWEEN 0 AND 32767),
    -- SMALLINT não negativo, igual a `npc_definition.pos_x`: o teto 4095 é regra do loader do
    -- cliente, não do banco (`mundo.md` §3 preserva um y2 = 6841 no retângulo da região).
    CONSTRAINT ck_teleport_route_origem_pos CHECK (
        from_x BETWEEN 0 AND 32767 AND from_y BETWEEN 0 AND 32767),
    CONSTRAINT ck_teleport_route_destino_pos CHECK (
        to_x BETWEEN 0 AND 32767 AND to_y BETWEEN 0 AND 32767)
);

-- P-15: as FKs nullable e a de destino. `from_map_id` já é prefixo do único.
CREATE INDEX ix_teleport_route_npc ON content.teleport_route (npc_id)
    WHERE npc_id IS NOT NULL;
CREATE INDEX ix_teleport_route_item ON content.teleport_route (req_item_index)
    WHERE req_item_index IS NOT NULL;
CREATE INDEX ix_teleport_route_destino ON content.teleport_route (to_map_id);
```

```sql
-- migrate:down
DROP INDEX content.ix_teleport_route_destino;
DROP INDEX content.ix_teleport_route_item;
DROP INDEX content.ix_teleport_route_npc;
DROP TABLE content.teleport_route;
DROP INDEX content.ix_map_definition_region;
DROP TABLE content.map_definition;
```

**Contrato que este desenho pretende garantir.** Rota sem mapa de origem ou de destino é impossível
(`NOT NULL` + FK `RESTRICT`); mapa apagado com rota apontando para ele falha alto; a mesma rota para o
mesmo NPC não duplica (`uq_teleport_route_rota`, inclusive sem NPC); apagar o NPC desliga a rota sem
derrubar o mapa (`ON DELETE SET NULL`); custo negativo é impossível (`ck_teleport_route_custo`); e o
mapa aceita qualquer id não negativo, sem o teto 255 da ficha.

**Decisões de forma respondidas em 2026-11-21.**

| # | Escolha aberta | Resposta do dono |
|---|---|---|
| 1 | Identidade de `map_definition` | PK natural `map_id SMALLINT`, sem identidade |
| 2 | Faixa de `map_id` | `map_id >= 0`, sem teto artificial |
| 3 | Flags do mapa | `is_pvp`, `is_safe` e `is_dungeon` |
| 4 | Forma da rota | coordenada explícita origem→destino |
| 5 | Identidade de `teleport_route` | `id` identity + único por coordenada e NPC, `NULLS NOT DISTINCT` |
| 6 | Custo | `coin BIGINT NOT NULL DEFAULT 0`, pago pelo ouro do personagem |

**Registro da aprovação.** O dono aprovou o conjunto em 2026-11-21, na opção "Sim, aprovar como está",
depois de responder as seis formas acima. Nenhuma coluna mudou depois da proposta: as duas tabelas
passam a contar em §1, o `0024` fica `concluída` e a página já estava escrita e conferida contra este
texto.

**O que este lote não decide:** o significado de cada flag por mapa (as três colunas são `H`: o legado
não prova onde `is_pvp`/`is_safe`/`is_dungeon` valem); a lista de mapas do 7662 (`mundo.md` §6,
UNVERIFIED — a tabela recebe o dado, não o inventa); a lista de teleportes por NPC (UNVERIFIED); o
preço em item (`req_item_index` é índice, não valor); o bloqueio de teleporte durante cerco, que é
regra de `wyd-server` (`mundo.md` §2 e `guerras.md`); e o seed de `0020`, ainda bloqueado pela
verificação do dado de origem.

---

### 5.25 Lote `0025` — combate: stats, equipamento e multiplicadores (DDL confirmado em 2026-11-21)

**Aprovado em 2026-11-21**, com as duas escolhas abertas respondidas pelo dono: `RESTRICT` na FK do
item em `drop_rate_override` e teto `1.000.000` em `content_rate_override.rate_bps`. As quatro tabelas
contam em §1, o lote está `concluída` em `migrations-checklist.md` §2 e a página
`migrations/0025-content-combat.md` já estava escrita e conferida.

O escopo — quatro nomes e a ordem — é `plano-migrations.md` §3, linha 25: `mob_template_stat`,
`mob_template_equip`, `drop_rate_override` e `content_rate_override`. As fichas vêm de
`0012_combate.sql` do legado (nível 4): dão coluna e faixa, não decisão.

**Filhos do template.** `mob_template_stat` usa o próprio `template_name` como PK (`P-04`) e
`mob_template_equip` usa `(template_name, slot)`, com `ON DELETE CASCADE`: o stat e o equipamento não
existem sem o template, e o template é o pai possuído (`P-15`). O item do equipamento e o item do drop
apontam `content.item_catalog` (`0014`) com `RESTRICT`.

**Drop é peso, não probabilidade.** `weight` é comparado ao roll 1..1.000.000 do servidor
(`drops.md`); por isso o CHECK é `> 0` e não uma faixa 0–100. `min_qty`/`max_qty` descrevem a pilha
dropada, com `max_qty >= min_qty`.

**Multiplicador por família.** `content_rate_override` é tabela raiz: `kind` é a PK e `rate_bps` é o
multiplicador em basis points (10000 = 100%). O nome diz “override” porque o valor base continua em
`content.game_setting` (`0019`); a multiplicação em si é do runtime, não do banco.

**Sem carimbo nem procedência por linha.** Tabela base seed-only e read-only em runtime, como §5.22 e
§5.24: `content_source`, `confidence` e `notes` das fichas não entram — proveniência de dataset é
`content.content_source` (`0019`) e `H` fica nesta página.

**Nenhum `GRANT` novo.** `0001` já concede `SELECT` em `content` a `wyd_game`, `wyd_portal` e
`wyd_read`, e nenhum runtime escreve estas quatro tabelas.

```sql
-- 0025_content_combat: stats, equipamento e multiplicadores de combate e drop.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabelas base seed-only e read-only em runtime; nenhum GRANT novo (0001 já concede SELECT em content).

-- migrate:up
-- Stats do template: PK é o próprio template e o filho é possuído por ele (P-04, P-15).
CREATE TABLE content.mob_template_stat (
    template_name TEXT NOT NULL,
    level         SMALLINT NOT NULL DEFAULT 1,
    max_hp        BIGINT NOT NULL DEFAULT 1,
    attack        INTEGER NOT NULL DEFAULT 0,
    defense       INTEGER NOT NULL DEFAULT 0,
    absorb        INTEGER NOT NULL DEFAULT 0,
    magic_attack  INTEGER NOT NULL DEFAULT 0,
    magic_defense INTEGER NOT NULL DEFAULT 0,
    exp_reward    BIGINT NOT NULL DEFAULT 0,
    gold_min      BIGINT NOT NULL DEFAULT 0,
    gold_max      BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_mob_template_stat PRIMARY KEY (template_name),
    CONSTRAINT fk_mob_template_stat_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_mob_template_stat_nivel CHECK (level BETWEEN 1 AND 400),
    CONSTRAINT ck_mob_template_stat_hp CHECK (max_hp > 0),
    CONSTRAINT ck_mob_template_stat_ataque CHECK (attack >= 0 AND magic_attack >= 0),
    CONSTRAINT ck_mob_template_stat_defesa CHECK (
        defense >= 0 AND magic_defense >= 0 AND absorb >= 0),
    CONSTRAINT ck_mob_template_stat_exp CHECK (exp_reward >= 0),
    CONSTRAINT ck_mob_template_stat_ouro CHECK (gold_min >= 0 AND gold_max >= gold_min)
);

-- Equipamento por template e slot; o item vem do catálogo (0014), não de texto livre.
CREATE TABLE content.mob_template_equip (
    template_name TEXT NOT NULL,
    slot          SMALLINT NOT NULL,
    item_index    INTEGER NOT NULL,
    CONSTRAINT pk_mob_template_equip PRIMARY KEY (template_name, slot),
    CONSTRAINT fk_mob_template_equip_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_mob_template_equip_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_mob_template_equip_slot CHECK (slot BETWEEN 0 AND 15),
    CONSTRAINT ck_mob_template_equip_item CHECK (item_index BETWEEN 0 AND 32767)
);

-- P-15: FK do item fora do prefixo da PK.
CREATE INDEX ix_mob_template_equip_item ON content.mob_template_equip (item_index);

-- Drop é peso relativo no roll 1..1.000.000 (drops.md), não probabilidade absoluta.
CREATE TABLE content.drop_rate_override (
    template_name TEXT NOT NULL,
    item_index    INTEGER NOT NULL,
    weight        BIGINT NOT NULL,
    min_qty       SMALLINT NOT NULL DEFAULT 1,
    max_qty       SMALLINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_drop_rate_override PRIMARY KEY (template_name, item_index),
    CONSTRAINT fk_drop_rate_override_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE CASCADE,
    -- Item com regra de drop não é apagado em silêncio: o catálogo é conteúdo e a regra é de combate.
    CONSTRAINT fk_drop_rate_override_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_drop_rate_override_peso CHECK (weight > 0),
    CONSTRAINT ck_drop_rate_override_qtd CHECK (min_qty > 0 AND max_qty >= min_qty),
    CONSTRAINT ck_drop_rate_override_item CHECK (item_index BETWEEN 0 AND 32767)
);

CREATE INDEX ix_drop_rate_override_item ON content.drop_rate_override (item_index);

-- Multiplicador por família de taxa: `kind` é a chave, e o valor 10000 = 100%.
CREATE TABLE content.content_rate_override (
    kind     TEXT NOT NULL,
    rate_bps INTEGER NOT NULL,
    CONSTRAINT pk_content_rate_override PRIMARY KEY (kind),
    CONSTRAINT ck_content_rate_override_kind CHECK (
        kind IN ('exp', 'drop', 'gold', 'skill_point')),
    CONSTRAINT ck_content_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 1000000)
);
```

```sql
-- migrate:down
DROP TABLE content.content_rate_override;
DROP INDEX content.ix_drop_rate_override_item;
DROP TABLE content.drop_rate_override;
DROP INDEX content.ix_mob_template_equip_item;
DROP TABLE content.mob_template_equip;
DROP TABLE content.mob_template_stat;
```

**Contrato que este desenho pretende garantir.** Stat sem template é impossível (`NOT NULL` + FK
`CASCADE`); equipamento no mesmo slot do mesmo template não duplica (`pk_mob_template_equip`); drop com
peso zero ou negativo é impossível; pilha invertida (`max_qty < min_qty`) é impossível; e família de
taxa fora das quatro conhecidas é impossível.

**Escolhas respondidas pelo dono em 2026-11-21.**

| # | Escolha | Decisão do dono |
|---|---|---|
| 1 | O que acontece ao apagar um item que tem regra de drop | `RESTRICT`: o catálogo não apaga em silêncio uma regra de combate (a ficha dizia `CASCADE`) |
| 2 | Teto de `content_rate_override.rate_bps` | `1.000.000` (10.000% em bps): mantém margem para multiplicador alto sem trocar o tipo |

**Registro da aprovação.** O dono aprovou o conjunto em 2026-11-21, sem alterar coluna, constraint ou
índice depois da proposta. As quatro tabelas passam a contar em §1 e o `0025` fica `concluída`; a página
`migrations/0025-content-combat.md` já estava escrita e conferida contra este texto.

**O que este lote não decide:** os valores de stat, drop e multiplicador (seed de `0020`/`0023`,
bloqueados pela verificação do dado de origem); a ordem do roll e o empate de peso no runtime
(`wyd-server`); a extração dos stats
do `STRUCT_MOB`, que a ficha declara não feita (`H`); e se `content_rate_override` substitui
`content.game_setting` ou convive com ele — este lote só guarda o multiplicador.

---

### 5.26 Lote `0026` — quests, eventos e arena: seis tabelas de conteúdo (DDL confirmado em 2026-11-21)

**Aprovado em 2026-11-21**, com as duas escolhas abertas respondidas pelo dono: `start_minute SMALLINT`
em `event_schedule` e `id` identity com `UNIQUE NULLS NOT DISTINCT` em `quest_reward`. As seis tabelas
contam em §1, o lote está `concluída` em `migrations-checklist.md` §2 e a página
`migrations/0026-content-quests-events.md` já estava escrita e conferida.

O escopo — seis nomes e a ordem — é `plano-migrations.md` §3, linha 26: `quest`, `quest_step`,
`quest_reward`, `world_event`, `event_schedule` e `arena`. As fichas vêm de `0013_quests.sql`,
`0014_eventos.sql` e `0015_arena.sql` do legado (nível 4).

**Chaves naturais.** `quest`, `world_event` e `arena` têm `code` legível e estável como PK, sem
identidade (`P-04`), com o mesmo formato de slug de `0019`. `quest_step` usa `(quest_code, step)` e
`event_schedule` usa identidade `BIGINT` (`P-03`): a agenda não tem chave natural — o mesmo evento pode
ter duas janelas no mesmo dia.

**Recompensa com identidade.** A ficha de `quest_reward` declara PK de quatro colunas com
`item_index NOT NULL` e, no mesmo arquivo, um CHECK que exige item só quando `kind = 'item'` — as duas
coisas não podem valer juntas para recompensa de ouro ou EXP. Aqui a linha tem `id` identity
(`P-03`), `item_index` anulável e um único `(quest_code, kind, item_index)` com `NULLS NOT DISTINCT`:
duas recompensas de ouro para a mesma quest continuam impossíveis, e o item só aparece quando o tipo é
`item`.

**Agenda sem `TIME`.** `P-06` proíbe `time` para evento. O horário vira `start_minute SMALLINT`
(0–1439, minuto do dia) e o fuso continua explícito em `timezone`, para o evento não andar quando o host
muda de fuso. O único `(event_code, weekday, start_minute)` fecha a janela duplicada.

**Posição e alvo do passo.** `reach` exige mapa e coordenada (`ck_quest_step_reach_posicao`), como o
“bug clássico de teleporte” de §5.24 exige ponto explícito. `target_id` é polimórfico (mob, item, mapa)
e por isso não tem FK: a integridade é do seed.

**Sem carimbo nem procedência por linha.** Como §5.22, §5.24 e §5.25: `content_source` e `confidence`
não entram; proveniência de dataset é `content.content_source` (`0019`).

**Nenhum `GRANT` novo.** `0001` já concede `SELECT` em `content`; ninguém escreve estas tabelas em
runtime.

```sql
-- 0026_content_quests_events: quests, agenda de eventos e arena como definição.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabelas base seed-only e read-only em runtime; nenhum GRANT novo (0001 já concede SELECT em content).

-- migrate:up
-- Quest é conteúdo: `code` é a chave legível e estável, sem identidade (P-04).
CREATE TABLE content.quest (
    code             TEXT NOT NULL,
    name             TEXT NOT NULL,
    kind             TEXT NOT NULL,
    min_level        SMALLINT NOT NULL DEFAULT 1,
    class_code       TEXT,
    repeatable       BOOLEAN NOT NULL DEFAULT false,
    cooldown_seconds INTEGER NOT NULL DEFAULT 0,
    -- Lista de códigos de quest anterior. Array não tem FK: a integridade é do seed (ver página §10).
    requires         TEXT[] NOT NULL DEFAULT '{}',
    description      TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_quest PRIMARY KEY (code),
    CONSTRAINT fk_quest_classe FOREIGN KEY (class_code)
        REFERENCES content.game_class (slug) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_quest_code CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_quest_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 120),
    CONSTRAINT ck_quest_kind CHECK (kind IN ('main', 'class', 'daily', 'event', 'sub')),
    CONSTRAINT ck_quest_nivel CHECK (min_level BETWEEN 1 AND 400),
    CONSTRAINT ck_quest_cooldown CHECK (cooldown_seconds >= 0),
    -- Uma quest não exige a si mesma, não carrega código vazio e não vira lista sem fim.
    CONSTRAINT ck_quest_requires CHECK (
        NOT (code = ANY (requires))
        AND array_position(requires, '') IS NULL
        AND coalesce(array_length(requires, 1), 0) <= 32)
);

CREATE INDEX ix_quest_kind_nivel ON content.quest (kind, min_level);
-- P-15: FK de classe é nullable e não aparece na PK.
CREATE INDEX ix_quest_classe ON content.quest (class_code) WHERE class_code IS NOT NULL;

-- Passo é filho possuído da quest; alvo é polimórfico (mob, item, mapa), por isso não tem FK.
CREATE TABLE content.quest_step (
    quest_code  TEXT NOT NULL,
    step        SMALLINT NOT NULL,
    objective   TEXT NOT NULL,
    target_id   INTEGER,
    count       INTEGER NOT NULL DEFAULT 1,
    map_id      SMALLINT,
    pos_x       SMALLINT,
    pos_y       SMALLINT,
    description TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_quest_step PRIMARY KEY (quest_code, step),
    CONSTRAINT fk_quest_step_quest FOREIGN KEY (quest_code)
        REFERENCES content.quest (code) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_quest_step_mapa FOREIGN KEY (map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_quest_step_step CHECK (step > 0),
    CONSTRAINT ck_quest_step_objetivo CHECK (
        objective IN ('kill', 'collect', 'talk', 'reach', 'refine', 'compose', 'use_item', 'escort')),
    CONSTRAINT ck_quest_step_alvo CHECK (target_id IS NULL OR target_id >= 0),
    CONSTRAINT ck_quest_step_count CHECK (count > 0),
    -- Posição só é obrigatória em `reach`; nos demais objetivos ela é opcional.
    CONSTRAINT ck_quest_step_reach_posicao CHECK (
        objective <> 'reach'
        OR (map_id IS NOT NULL AND pos_x IS NOT NULL AND pos_y IS NOT NULL)),
    CONSTRAINT ck_quest_step_posicao CHECK (
        (pos_x IS NULL OR pos_x BETWEEN 0 AND 32767)
        AND (pos_y IS NULL OR pos_y BETWEEN 0 AND 32767))
);

CREATE INDEX ix_quest_step_mapa ON content.quest_step (map_id) WHERE map_id IS NOT NULL;

-- Recompensa por tipo. A ficha tornava `item_index` NOT NULL numa PK de quatro colunas, o que
-- contradiz o próprio CHECK (`kind = 'item'` exige item; ouro e EXP não têm item). Aqui a linha
-- tem identidade e o único fecha a dupla (tipo, item) sem obrigar item em toda recompensa.
CREATE TABLE content.quest_reward (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    quest_code TEXT NOT NULL,
    kind       TEXT NOT NULL,
    item_index INTEGER,
    amount     BIGINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_quest_reward PRIMARY KEY (id),
    CONSTRAINT uq_quest_reward UNIQUE NULLS NOT DISTINCT (quest_code, kind, item_index),
    CONSTRAINT fk_quest_reward_quest FOREIGN KEY (quest_code)
        REFERENCES content.quest (code) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_quest_reward_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_quest_reward_kind CHECK (
        kind IN ('item', 'gold', 'exp', 'fame', 'skill_point')),
    CONSTRAINT ck_quest_reward_item CHECK ((kind = 'item') = (item_index IS NOT NULL)),
    CONSTRAINT ck_quest_reward_amount CHECK (amount > 0)
);

CREATE INDEX ix_quest_reward_item ON content.quest_reward (item_index)
    WHERE item_index IS NOT NULL;

-- Evento de mundo: chave legível, sem identidade.
CREATE TABLE content.world_event (
    code          TEXT NOT NULL,
    name          TEXT NOT NULL,
    kind          TEXT NOT NULL,
    enabled       BOOLEAN NOT NULL DEFAULT true,
    requires_city BOOLEAN NOT NULL DEFAULT false,
    description   TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_world_event PRIMARY KEY (code),
    CONSTRAINT ck_world_event_code CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_world_event_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 120),
    CONSTRAINT ck_world_event_kind CHECK (
        kind IN ('kefra', 'noatun', 'city_war', 'rvr', 'tower', 'battle_royale', 'custom'))
);

-- Agenda: horário em minuto do dia, não `TIME` — P-06 proíbe `time` para evento, e o minuto
-- continua comparável sem depender do fuso do host. `timezone` diz em que fuso o minuto vale.
CREATE TABLE content.event_schedule (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    event_code       TEXT NOT NULL,
    weekday          SMALLINT NOT NULL,
    start_minute     SMALLINT NOT NULL,
    duration_minutes SMALLINT NOT NULL,
    timezone         TEXT NOT NULL DEFAULT 'America/Sao_Paulo',
    every_n_weeks    SMALLINT NOT NULL DEFAULT 1,
    enabled          BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_event_schedule PRIMARY KEY (id),
    CONSTRAINT uq_event_schedule UNIQUE (event_code, weekday, start_minute),
    CONSTRAINT fk_event_schedule_event FOREIGN KEY (event_code)
        REFERENCES content.world_event (code) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_event_schedule_weekday CHECK (weekday BETWEEN 0 AND 6),
    CONSTRAINT ck_event_schedule_inicio CHECK (start_minute BETWEEN 0 AND 1439),
    CONSTRAINT ck_event_schedule_duracao CHECK (duration_minutes > 0),
    CONSTRAINT ck_event_schedule_fuso CHECK (char_length(btrim(timezone)) BETWEEN 1 AND 64),
    CONSTRAINT ck_event_schedule_periodo CHECK (every_n_weeks BETWEEN 1 AND 8)
);

-- Arena é definição; a partida é estado e fica em 0028.
CREATE TABLE content.arena (
    code      TEXT NOT NULL,
    name      TEXT NOT NULL,
    kind      TEXT NOT NULL,
    team_size SMALLINT NOT NULL DEFAULT 16,
    rounds    SMALLINT NOT NULL DEFAULT 3,
    round_ms  INTEGER NOT NULL DEFAULT 300000,
    respawns  SMALLINT NOT NULL DEFAULT 1,
    min_level SMALLINT NOT NULL DEFAULT 200,
    no_drop   BOOLEAN NOT NULL DEFAULT true,
    enabled   BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_arena PRIMARY KEY (code),
    CONSTRAINT ck_arena_code CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_arena_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_arena_kind CHECK (kind IN ('royal', 'battle_royale')),
    CONSTRAINT ck_arena_time CHECK (team_size BETWEEN 1 AND 64),
    CONSTRAINT ck_arena_rounds CHECK (rounds BETWEEN 1 AND 9),
    CONSTRAINT ck_arena_round_ms CHECK (round_ms > 0),
    CONSTRAINT ck_arena_respawns CHECK (respawns BETWEEN 0 AND 9),
    CONSTRAINT ck_arena_nivel CHECK (min_level BETWEEN 1 AND 400)
);
```

```sql
-- migrate:down
DROP TABLE content.arena;
DROP TABLE content.event_schedule;
DROP TABLE content.world_event;
DROP INDEX content.ix_quest_reward_item;
DROP TABLE content.quest_reward;
DROP INDEX content.ix_quest_step_mapa;
DROP TABLE content.quest_step;
DROP INDEX content.ix_quest_classe;
DROP INDEX content.ix_quest_kind_nivel;
DROP TABLE content.quest;
```

**Contrato que este desenho pretende garantir.** Passo órfão é impossível (FK `CASCADE`); `reach` sem
posição é impossível; passo repetido na mesma quest é impossível (PK); janela de evento duplicada é
impossível (único); recompensa de item sem item é impossível, e recompensa de ouro com item também
(`ck_quest_reward_item`); quest que exige a si mesma é impossível.

**Escolhas respondidas pelo dono em 2026-11-21.**

| # | Escolha | Decisão do dono |
|---|---|---|
| 1 | Como guardar o horário da agenda | `start_minute SMALLINT` 0–1439 (`P-06` recusa `TIME`); a ficha pedia `start_time TIME` |
| 2 | Como identificar uma recompensa | `id` identity + `UNIQUE NULLS NOT DISTINCT` em `(quest_code, kind, item_index)`; a ficha pedia PK de quatro colunas com `item_index` obrigatório |

**Registro da aprovação.** O dono aprovou o conjunto em 2026-11-21, sem alterar coluna, constraint ou
índice depois da proposta. As seis tabelas passam a contar em §1 e o `0026` fica `concluída`; a página
`migrations/0026-content-quests-events.md` já estava escrita e conferida contra este texto.

**O que este lote não decide:** a cadeia de quests do 7662 (a ficha a declara não fechada; seed seria
invenção); a integridade referencial de `quest.requires`, que é lista e não FK (validação do seed); o
calendário real de Kefra/Noatun (seed de `0027`, bloqueado pela verificação do dado de origem); e a
execução de evento, que é
`wyd-server`.

---

### 5.28 Lote `0028` — progresso de quest, eventos e arena no jogo (DDL confirmado em 2026-11-21)

**Aprovado em 2026-11-21.** As quatro escolhas de forma já eram decididas pelo canônico e o dono
aprovou o conjunto sem alterá-las. As sete tabelas contam em §1, o lote está `concluída` em
`migrations-checklist.md` §2 e a página `migrations/0028-game-quests-events.md` já estava escrita e
conferida.

O escopo — sete nomes e a ordem — é `plano-migrations.md` §3, linha 28: `character_quest`,
`event_participation`, `kefra_state`, `castle_state`, `arena_match`, `arena_round` e `arena_score`.
As fichas vêm de `0013_quests.sql`, `0014_eventos.sql` e `0015_arena.sql` do legado (nível 4), e são o
único bloco de `game` que referencia `content` — por isso fecha depois de `0026`.

**Estado de jogador, não conteúdo.** Aqui a linha é escrita pelo servidor enquanto o jogo roda:
progresso de quest, histórico de evento, placar de arena e o estado único de Kefra e do castelo. O
portal não escreve nenhuma destas tabelas e as lê pela view de `0029`.

**Chaves.** `character_quest` usa `(character_id, quest_code)`; `arena_round` usa
`(match_id, round_no)`; `arena_score` usa `(match_id, round_no, character_id)`. As três são filhas
possuídas e usam `CASCADE` no pai que as define. `event_participation`, `arena_match` e
`arena_score` recebem identidade `BIGINT` (`P-03`). `kefra_state` e `castle_state` são **linha única**:
`id SMALLINT` com `CHECK (id = 1)`, sem identidade, como `content.content_version` (`0001`).

**Conteúdo apagado não apaga jogador.** As FKs para `content.quest`, `content.world_event` e
`content.arena` usam `RESTRICT` — remover definição com estado dependente falha alto, em vez de levar
progresso ou histórico junto (a ficha dizia `CASCADE`). Já as FKs para `game.character` e `game.guild`
usam `CASCADE`/`SET NULL` conforme o dono da linha: progresso e placar somem com o personagem;
participação e estado de Kefra/castelo apenas perdem a referência.

**Sem carimbo de conteúdo.** `created_at`/`updated_at` não entram: quem carimba evento é `at`, e estado
único não é editado por staff. Nenhum `GRANT` novo: `0001` já dá `SELECT, INSERT, UPDATE, DELETE` em
`game` para `wyd_game`, e o portal continua sem escrita.

```sql
-- 0028_game_quests_events: progresso de quest, histórico de evento, estado de Kefra/castelo e arena.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Estado de jogador: escreve só o servidor; o portal lê pelas views de 0029. Nenhum GRANT novo.

-- migrate:up
-- Progresso de quest: PK composta (personagem, quest) e filho possuído do personagem.
CREATE TABLE game.character_quest (
    character_id BIGINT NOT NULL,
    quest_code   TEXT NOT NULL,
    step         SMALLINT NOT NULL DEFAULT 1,
    progress     INTEGER NOT NULL DEFAULT 0,
    started_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    CONSTRAINT pk_character_quest PRIMARY KEY (character_id, quest_code),
    CONSTRAINT fk_character_quest_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    -- Remover a quest do conteúdo não apaga progresso de jogador: a FK recusa alto.
    CONSTRAINT fk_character_quest_quest FOREIGN KEY (quest_code)
        REFERENCES content.quest (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_character_quest_step CHECK (step > 0),
    CONSTRAINT ck_character_quest_progress CHECK (progress >= 0),
    CONSTRAINT ck_character_quest_conclusao CHECK (
        completed_at IS NULL OR completed_at >= started_at)
);

CREATE INDEX ix_character_quest_quest ON game.character_quest (quest_code);

-- Histórico de participação: linha some com evento? Não — recusa alto; personagem e clã viram nulo.
CREATE TABLE game.event_participation (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    event_code   TEXT NOT NULL,
    character_id BIGINT,
    guild_id     BIGINT,
    result       TEXT NOT NULL,
    score        BIGINT NOT NULL DEFAULT 0,
    at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_event_participation PRIMARY KEY (id),
    CONSTRAINT fk_event_participation_event FOREIGN KEY (event_code)
        REFERENCES content.world_event (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_event_participation_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_event_participation_guild FOREIGN KEY (guild_id)
        REFERENCES game.guild (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT ck_event_participation_result CHECK (
        result IN ('win', 'lose', 'draw', 'participated')),
    CONSTRAINT ck_event_participation_score CHECK (score >= 0)
);

CREATE INDEX ix_event_participation_event
    ON game.event_participation (event_code, at DESC);
CREATE INDEX ix_event_participation_character
    ON game.event_participation (character_id) WHERE character_id IS NOT NULL;
CREATE INDEX ix_event_participation_guild
    ON game.event_participation (guild_id) WHERE guild_id IS NOT NULL;

-- Linha única de estado do Kefra: `id = 1` é a chave, sem identidade (como content_version).
CREATE TABLE game.kefra_state (
    id            SMALLINT NOT NULL,
    last_kill_at  TIMESTAMPTZ,
    next_spawn_at TIMESTAMPTZ,
    kill_count    INTEGER NOT NULL DEFAULT 0,
    last_guild_id BIGINT,
    CONSTRAINT pk_kefra_state PRIMARY KEY (id),
    CONSTRAINT fk_kefra_state_guild FOREIGN KEY (last_guild_id)
        REFERENCES game.guild (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT ck_kefra_state_linha_unica CHECK (id = 1),
    CONSTRAINT ck_kefra_state_kill_count CHECK (kill_count >= 0)
);

CREATE INDEX ix_kefra_state_guild ON game.kefra_state (last_guild_id)
    WHERE last_guild_id IS NOT NULL;

-- Dono do castelo, cidade e cofre; a cidade é a de 0011 e não some com o castelo.
CREATE TABLE game.castle_state (
    id                SMALLINT NOT NULL,
    owner_guild_id    BIGINT,
    city_code         TEXT,
    vault_gold        BIGINT NOT NULL DEFAULT 0,
    last_collected_at TIMESTAMPTZ,
    last_war_at       TIMESTAMPTZ,
    CONSTRAINT pk_castle_state PRIMARY KEY (id),
    CONSTRAINT fk_castle_state_guild FOREIGN KEY (owner_guild_id)
        REFERENCES game.guild (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_castle_state_cidade FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_castle_state_linha_unica CHECK (id = 1),
    CONSTRAINT ck_castle_state_cofre CHECK (vault_gold >= 0)
);

CREATE INDEX ix_castle_state_guild ON game.castle_state (owner_guild_id)
    WHERE owner_guild_id IS NOT NULL;
CREATE INDEX ix_castle_state_cidade ON game.castle_state (city_code)
    WHERE city_code IS NOT NULL;

-- Partida de arena: histórico, não estado vivo; `arena_code` é a definição de 0026.
CREATE TABLE game.arena_match (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    arena_code  TEXT NOT NULL,
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at    TIMESTAMPTZ,
    winner_team SMALLINT,
    CONSTRAINT pk_arena_match PRIMARY KEY (id),
    CONSTRAINT fk_arena_match_arena FOREIGN KEY (arena_code)
        REFERENCES content.arena (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_arena_match_fim CHECK (ended_at IS NULL OR ended_at >= started_at),
    CONSTRAINT ck_arena_match_vencedor CHECK (winner_team IN (1, 2))
);

CREATE INDEX ix_arena_match_arena ON game.arena_match (arena_code, started_at DESC);

CREATE TABLE game.arena_round (
    match_id    BIGINT NOT NULL,
    round_no    SMALLINT NOT NULL,
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at    TIMESTAMPTZ,
    winner_team SMALLINT,
    is_tiebreak BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_arena_round PRIMARY KEY (match_id, round_no),
    CONSTRAINT fk_arena_round_match FOREIGN KEY (match_id)
        REFERENCES game.arena_match (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_arena_round_numero CHECK (round_no BETWEEN 1 AND 9),
    CONSTRAINT ck_arena_round_fim CHECK (ended_at IS NULL OR ended_at >= started_at),
    CONSTRAINT ck_arena_round_vencedor CHECK (winner_team IN (1, 2))
);

-- Placar por jogador e round; a FK composta garante que o round existe na partida citada.
CREATE TABLE game.arena_score (
    match_id     BIGINT NOT NULL,
    round_no     SMALLINT NOT NULL,
    character_id BIGINT NOT NULL,
    team         SMALLINT NOT NULL,
    kills        SMALLINT NOT NULL DEFAULT 0,
    assists      SMALLINT NOT NULL DEFAULT 0,
    damage       BIGINT NOT NULL DEFAULT 0,
    deaths       SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_arena_score PRIMARY KEY (match_id, round_no, character_id),
    CONSTRAINT fk_arena_score_round FOREIGN KEY (match_id, round_no)
        REFERENCES game.arena_round (match_id, round_no) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_arena_score_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_arena_score_time CHECK (team IN (1, 2)),
    CONSTRAINT ck_arena_score_kills CHECK (kills >= 0),
    CONSTRAINT ck_arena_score_assists CHECK (assists >= 0),
    CONSTRAINT ck_arena_score_damage CHECK (damage >= 0),
    CONSTRAINT ck_arena_score_deaths CHECK (deaths >= 0)
);

CREATE INDEX ix_arena_score_character ON game.arena_score (character_id);
```

```sql
-- migrate:down
DROP INDEX game.ix_arena_score_character;
DROP TABLE game.arena_score;
DROP TABLE game.arena_round;
DROP INDEX game.ix_arena_match_arena;
DROP TABLE game.arena_match;
DROP INDEX game.ix_castle_state_cidade;
DROP INDEX game.ix_castle_state_guild;
DROP TABLE game.castle_state;
DROP INDEX game.ix_kefra_state_guild;
DROP TABLE game.kefra_state;
DROP INDEX game.ix_event_participation_guild;
DROP INDEX game.ix_event_participation_character;
DROP INDEX game.ix_event_participation_event;
DROP TABLE game.event_participation;
DROP INDEX game.ix_character_quest_quest;
DROP TABLE game.character_quest;
```

**Contrato que este desenho pretende garantir.** Progresso sem personagem ou sem quest é impossível;
placar de round inexistente é impossível (FK composta); round fora de 1–9 é impossível; equipe fora de
1/2 é impossível; duas linhas de estado de Kefra ou do castelo são impossíveis (`CHECK (id = 1)`); e
apagar quest/evento/arena com estado dependente falha alto.

**Escolhas de forma (decididas pelo canônico, não pelo gosto).**

| # | Escolha | Decisão e por quê |
|---|---|---|
| 1 | PK de `character_quest`, `arena_round`, `arena_score` | compostas (`P-04`): o par/trio já é único e é o acesso |
| 2 | Identidade das tabelas de histórico | `BIGINT GENERATED ALWAYS AS IDENTITY` (`P-03`); a ficha usava `INTEGER` |
| 3 | `ON DELETE` das FKs de conteúdo | `RESTRICT`; `CASCADE` da ficha apagaria progresso/histórico de jogador |
| 4 | Linha única de Kefra e castelo | `SMALLINT` + `CHECK (id = 1)`, sem identidade, como `content_version` |

**Registro da aprovação.** O dono aprovou o conjunto em 2026-11-21, sem alterar nenhuma das quatro
decisões acima. As sete tabelas passam a contar em §1 e o `0028` fica `concluída`; a página
`migrations/0028-game-quests-events.md` já estava escrita e conferida contra este texto.

**O que este lote não decide:** as regras de progresso, recompensa e placar (`wyd-server`); se o estado
de Kefra é por canal ou por instalação (a ficha tem uma linha só; multi-canal exigiria outra chave);
a retenção de `event_participation` e `arena_*` (`M22` cobre sessão e auditoria, não estas); e a
fronteira de leitura do portal, que depende da view e da permissão de `0029`.

---

### 5.29 Lote `0029` — views de leitura cruzada (DDL confirmado em 2026-11-21)

**Aprovado em 2026-11-21**, com a escolha de permissão respondida pelo dono: nenhum `GRANT` novo, nem
`USAGE` no schema cruzado, até haver decisão explícita de fronteira. As duas views são criadas e ficam
ilegíveis pelo outro domínio; o `0029` fica `concluída` em `migrations-checklist.md` §2 e não muda a
contagem de §1, porque não cria tabela.

O escopo é `plano-migrations.md` §3, linha 29: `game.v_live_session` e `portal.v_person_identity`.
Não há ficha de tabela para elas; a fonte é `padrao-banco.md` §7, que fixa propósito e nome, mais a
regra `P-17` (view é o contrato de leitura entre domínios, não acesso à tabela interna).

**Presença sem vazar o jogo.** `game.v_live_session` projeta a sessão viva (`released_at IS NULL`) com
`session_id`, `account_id`, `character_id`, `character_name`, `group_id`, `channel`,
`client_version`, `opened_at` e `renewed_at`. Fica de fora o que não serve ao painel: `ip` (`P-10`),
motivo de liberação e qualquer coluna de `game.account`. `LEFT JOIN` em `character` porque a sessão
nasce no handshake, antes da seleção de personagem.

**Identidade sem credencial.** `portal.v_person_identity` projeta `person_id`, `role` e `status` — o
mínimo que o jogo precisa para saber se a pessoa pode entrar. Ficam de fora e-mail, `email_fold`,
hash de senha, PIN, contadores de falha e anotações de suspensão (`A12`, `P-17`).

**Nenhum `GRANT` neste texto.** As views nascem com o dono `wyd_migrator` e rodam com privilégio do
dono; a leitura por `wyd_portal`/`wyd_game` depende de permissão que este lote não cria — `0001` não
dá `USAGE` em `game` ao portal nem `USAGE` em `portal` ao jogo. Isso é escolha de fronteira, levada ao
dono abaixo, e não um detalhe de sintaxe.

```sql
-- 0029_views: presença para o portal e identidade da pessoa para o jogo.
-- As duas views existem para que nenhum domínio leia a tabela interna do outro (P-17, padrao-banco §7).
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Nenhum GRANT neste arquivo: permissão de leitura é decisão própria (ver página §7 e §10).

-- migrate:up
-- Presença viva por sessão: sem IP, sem e-mail e sem tabela interna do jogo exposta.
-- LEFT JOIN porque a sessão nasce no handshake, antes da seleção de personagem.
CREATE VIEW game.v_live_session AS
SELECT s.id             AS session_id,
       s.account_id     AS account_id,
       s.character_id   AS character_id,
       c.name           AS character_name,
       s.group_id       AS group_id,
       s.channel        AS channel,
       s.client_version AS client_version,
       s.opened_at      AS opened_at,
       s.renewed_at     AS renewed_at
FROM game.session s
LEFT JOIN game.character c ON c.id = s.character_id
WHERE s.released_at IS NULL;

-- Identidade mínima da pessoa para o jogo: id, papel e estado; nunca e-mail, hash ou PIN.
CREATE VIEW portal.v_person_identity AS
SELECT p.id     AS person_id,
       p.role   AS role,
       p.status AS status
FROM portal.person p;
```

```sql
-- migrate:down
DROP VIEW portal.v_person_identity;
DROP VIEW game.v_live_session;
```

**Contrato que este desenho pretende garantir.** A view de presença só mostra sessão viva; nenhuma
coluna sensível de pessoa ou de sessão é projetada; e a definição não depende de tabela que ainda não
exista (as duas views são do último arquivo da série).

**Escolha respondida pelo dono em 2026-11-21.**

| # | Escolha | Decisão do dono |
|---|---|---|
| 1 | Como o portal lê `game.v_live_session` sem `USAGE` em `game` | Nenhuma permissão por enquanto: sem `GRANT USAGE` no schema e sem `SELECT` na view, até haver decisão explícita de fronteira |

**Registro da aprovação.** O dono aprovou o conjunto em 2026-11-21 com a opção "sem permissão por
enquanto". As duas views passam a existir e ficam ilegíveis para o outro domínio; o `0029` fica
`concluída` e a página `migrations/0029-views.md` já estava escrita e conferida contra este texto.

**O que este lote não decide:** a política de retenção da sessão (`M22`); o formato do painel de
monitoramento (`wyd-portal`, `LAC-POR-06`); se a view ganha `security_invoker` (depende da permissão
acima); e a criação de view materializada, que `P-17` proíbe até haver medição.

### 5.32 Lote `0032` — `GRANT`s do papel `wyd_admin` (DDL confirmado em 2026-11-21)

**Decidido em 2026-11-21 (`M27`, opção A + S1 + C1 + D1).** O papel nasce fora da série, em
`db/roles.sql` (`src/provision/spec.md` §2 e §4); este arquivo só concede. **Sem tabela, sem índice e
sem objeto:** `Tab.` continua 97 e a série passa a 32 arquivos.

```sql
-- migrate:up
GRANT USAGE ON SCHEMA game TO wyd_admin;
GRANT USAGE ON SCHEMA ops TO wyd_admin;

GRANT SELECT, INSERT, UPDATE ON game.account_moderation TO wyd_admin;
GRANT SELECT, UPDATE ON game.account_wallet TO wyd_admin;
GRANT SELECT, INSERT ON game.account_ledger TO wyd_admin;
GRANT SELECT, UPDATE ON game.server_channel TO wyd_admin;
GRANT SELECT, UPDATE ON game.session TO wyd_admin;
GRANT SELECT, UPDATE ON game.city TO wyd_admin;
GRANT SELECT, UPDATE ON game.item_price TO wyd_admin;

GRANT INSERT ON ops.audit_log TO wyd_admin;
GRANT SELECT (id) ON ops.audit_log TO wyd_admin;

-- migrate:down
REVOKE SELECT (id) ON ops.audit_log FROM wyd_admin;
REVOKE INSERT ON ops.audit_log FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.item_price FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.city FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.session FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.server_channel FROM wyd_admin;
REVOKE SELECT, INSERT ON game.account_ledger FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.account_wallet FROM wyd_admin;
REVOKE SELECT, INSERT, UPDATE ON game.account_moderation FROM wyd_admin;
REVOKE USAGE ON SCHEMA ops FROM wyd_admin;
REVOKE USAGE ON SCHEMA game FROM wyd_admin;
```

**Por que esses privilégios.** `USAGE` abre o schema; a escrita vem da tabela. `SELECT` nas sete tabelas
cobre `expected_*`, `WHERE` e `RETURNING` sem depender de outra leitura (S1). `DELETE` fica de fora
porque nenhuma rota do C4 pede. Em `ops.audit_log`, `INSERT` é a auditoria (C1) e `SELECT (id)` existe
porque o `200` de §4.1–§4.4 devolve `audit_id`; nenhuma outra coluna é legível.

**Por que arquivo novo (D1).** Reabrir `0001` obrigaria a reescrever DDL já aplicado, e `db/roles.sql`
não concede privilégio de schema da série (`src/provision/spec.md` §1). O `0032` não reordena nada: só
`0031` o precede e nenhuma view o lê.

**O que não muda:** as sete tabelas, `Tab.` 97, e o `INSERT` de `wyd_game`/`wyd_portal` em
`ops.audit_log` (`0002`). Tabela criada depois da troca **não** herda estes privilégios: não há
`ALTER DEFAULT PRIVILEGES` aqui.

**Fechado fora deste lote:** `M28` — decidida em 2026-11-21 na redação B: a "API interna" de §5.15 e de
`migrations/0030-portal-content.md` §8 é o serviço `wyd-admin`; as quatro tabelas de `portal` ficam com
o BFF do portal. A correção é só de prosa, sem `GRANT` nem DDL.

---

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
3. **Não altera** `tools/`: o PR de ferramentas foi liberado em 2026-11-20 (`plano-migrations.md` §1)
   e é entrega própria; aqui só se escreve DDL.
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

## 9. Forma do modelo e fechamento do DDL (decidido em 2026-11-20)

Três decisões do dono, na mesma rodada em que `A7`, `A21` e `P3` fecharam:

1. **Estado de jogo em colunas nomeadas.** `0006` e `0008`–`0012`: cada campo do `STRUCT_MOB` vira
   coluna tipada na tabela dona, em `snake_case`, com origem declarada na página; sem `JSONB` de
   atributos e sem espelhar tabela do legado.
2. **`content` redesenhado.** `0013`–`0027`: o schema é escrito no padrão vigente, com o cliente 7662
   como fonte e o legado só como `H`; cada campo declara origem, e cada tabela tem constraint
   nomeada, índice e permissão como as já fechadas. Não há espelho 1:1 do legado.
3. **Fechamento em lote.** Um arquivo da série por lote: o DDL de cada lote entra **neste arquivo** só
   depois do "sim" do dono, e é daqui que a página de migration copia, nunca de paráfrase. A ordem é
   a da série (`plano-migrations.md` §3); cada lote traz colunas, constraints nomeadas, índices,
   permissões e prova de execução, no molde de `0004`.

Consequências:

- O lote `0002` (`ops.config_revision` e `ops.config_change`) fechou em 2026-11-20 (§5.9) e é a trilha
  de auditoria da configuração editável pela área administrativa do portal
  (`wyd-portal/wyd-portal-prompt.md` §6).
- O lote `0006` (`character` e os quatro satélites) fechou em 2026-11-20 (§5.10); `A14` decide a
  semântica do delete.
- O lote `0008` (`character_skill` e os quatro satélites) fechou em 2026-11-20 (§5.11) e o `0009`
  (`character_item`, `vault_item` e `refine_pity`) fechou na mesma data (§5.12).
- O lote `0010` (`guild` e os três satélites) fechou em 2026-11-20 (§5.13) e o `0011` (economia, 10
  tabelas) fechou na mesma data (§5.14).
- O adendo `0030` (conteúdo do portal: `news`, `banner`, `guide`, `launcher_release`) foi desenhado e
  aprovado em 2026-11-20 (§5.15); com o `0031` e o `0032` (só `GRANT`), a série está em 32 arquivos /
  97 tabelas.
- O lote `0012` (`ops.delivery_queue`) fechou em 2026-11-20 (§5.16), com `M25`: o dreno usa
  `SELECT … FOR UPDATE SKIP LOCKED` e a linha fechada é expurgada 30 dias após `closed_at`.
- Nenhum lote segue pendente: o `0021` (cinco tabelas) fechou em 2026-11-20 (§5.22) e o `0024` (mapa
  e teleporte) em 2026-11-21 (§5.24). Em 2026-11-21 o dono aprovou também os cinco últimos — `0022`
  (§5.23), `0025` (§5.25), `0026` (§5.26), `0028` (§5.28) e `0029` (§5.29) —, então as 97 tabelas da
  série têm DDL confirmado.
- O lote `0013` (taxonomias do conteúdo: cinco tabelas e `fn_fold`) fechou em 2026-11-20 (§5.17), com
  as duas escolhas de forma aprovadas.
- O lote `0014` (`content.item_catalog`) fechou em 2026-11-20 (§5.18), com as três escolhas de forma
  aprovadas, e o `0015` (`content.skill_catalog`) também (§5.19); o próximo lote com DDL de tabela é
  `0019` (`content.world`), com os seeds `0016`–`0018` bloqueados pelo dado de origem; depois dele,
  `0021` (§5.22) e `0022` (§5.23), ambos aprovados.
- A contagem de §1 só muda quando um lote é aprovado e o texto dele entra aqui: escrever página não
  altera DDL, e desenho aprovado em conversa não é DDL até estar neste arquivo.

---

*Planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado ou alterado.*

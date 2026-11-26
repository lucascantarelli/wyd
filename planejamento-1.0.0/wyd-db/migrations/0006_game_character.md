> Pergunta: "como aplico o `0006_game_character` e que personagem e estado básico passam a existir?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.10; `padrao-banco.md` §1/§5/§9; `lacunas.md` `A14`
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-24)

Sexto arquivo. Cria a identidade do personagem e quatro satélites 1:1 para status, scores, especiais e
barra curta de skills. É o primeiro estado persistente de personagem; itens vão em `0009`, skills
aprendidas em `0008`, guilda em `0010`, e conteúdo nos arquivos `0013` em diante.

!!! note "Aplicado em 2026-11-24"
    O arquivo `crates/wyd-db/migrations/0006_game_character.sql` existe e a série aplicou do zero em PostgreSQL 17
    nessa data, como `wyd_migrator` (`planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §6.6.30). O
    DDL continua sendo o de `ddl-confirmado.md` §5.10, dono do texto; as provas de §9 não foram executadas uma a uma
    nesta leva.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0006` |
| Arquivo | `crates/wyd-db/migrations/0006_game_character.sql` |
| Schema | `game` (FK para `content.content_version`) |
| Posição | 6 de 34 (personagem e estado-base) |
| Depende de | `0001` (schemas/função), `0005` (`game.account`), versão `content.content_version` semeada por `0016` antes da criação de personagem |
| É dependência de | `0007` (`game.session.character_id`), `0008` (skills), `0009` (itens), `0010` (guilda) |
| Tabelas criadas | 5 (`character`, `character_status`, `character_scores`, `character_special`, `character_short_skill`) |
| Objetos criados | 5 tabelas, 4 índices, 1 trigger |
| `SCHEMA_VERSION` | não muda aqui: a migração troca a série para `34` somente no procedimento de `plano-migrations.md` §6 |

## 2. Objetivo

Persistir personagem, vaga e atributos de fio; manter scores, status, especiais e barra curta em tabelas
1:1, com soft-delete que libera nome e vaga sem apagar a identidade histórica.

## 3. Pré-requisitos

1. `0001_bootstrap`, `0005_game_account` e as migrations que criam/semeiam `content.content_version`
   aplicados (`0016` insere a versão inicial).
2. Conta ativa existente em `game.account`.
3. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.10.

```sql
-- migrate:up
CREATE TABLE game.character (
    id                 BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id         BIGINT NOT NULL,
    slot               SMALLINT NOT NULL,
    name               TEXT NOT NULL,
    name_fold          TEXT GENERATED ALWAYS AS (lower(btrim(name))) STORED,
    class              SMALLINT NOT NULL DEFAULT 0,
    tier               TEXT NOT NULL DEFAULT 'mortal',
    level              INTEGER NOT NULL DEFAULT 1,
    exp                BIGINT NOT NULL DEFAULT 0,
    coin               BIGINT NOT NULL DEFAULT 0,
    magic              INTEGER NOT NULL DEFAULT 0,
    learned_skill_mask BIGINT NOT NULL DEFAULT 0,
    score_bonus        INTEGER NOT NULL DEFAULT 0,
    special_bonus      INTEGER NOT NULL DEFAULT 0,
    skill_bonus        INTEGER NOT NULL DEFAULT 0,
    critical           SMALLINT NOT NULL DEFAULT 0,
    save_mana          SMALLINT NOT NULL DEFAULT 0,
    recall_x           SMALLINT,
    recall_y           SMALLINT,
    content_version    SMALLINT NOT NULL,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
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

## 5. DDL do `down`

Tabelas filhas primeiro; a função de trigger é do `0001` e não é removida aqui.

```sql
-- migrate:down
DROP TABLE game.character_short_skill;
DROP TABLE game.character_special;
DROP TABLE game.character_scores;
DROP TABLE game.character_status;
DROP TRIGGER trg_character_set_updated_at ON game.character;
DROP INDEX game.ix_character_content_version;
DROP INDEX game.ix_character_account;
DROP INDEX game.ux_character_vaga;
DROP INDEX game.ux_character_nome;
DROP TABLE game.character;
```

## 6. Tabelas de colunas

### 6.1 `game.character`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Identificador interno do personagem. |
| `account_id` | `BIGINT` | não | — | Dono da conta de jogo, FK `CASCADE`. |
| `slot` | `SMALLINT` | não | — | Vaga 0–3 por conta (`H`, legado/cliente). |
| `name` | `TEXT` | não | — | Nome visível, 1–16 chars (`H`). |
| `name_fold` | `TEXT` gerada | não | `lower(btrim(name))` | Chave dobrada para unicidade entre vivos. |
| `class` | `SMALLINT` | não | `0` | Classe 0–5 (`STRUCT_MOB.Class`, `H`). |
| `tier` | `TEXT` | não | `'mortal'` | Progressão declarada pela decisão aprovada do lote. |
| `level` | `INTEGER` | não | `1` | Nível 1–400 (`H`). |
| `exp` | `BIGINT` | não | `0` | Experiência, não negativa (`STRUCT_MOB.Exp`, `H`). |
| `coin` | `BIGINT` | não | `0` | Moeda carregada no personagem (`STRUCT_MOB.Coin`, `H`), separada da carteira. |
| `magic` | `INTEGER` | não | `0` | Valor de `STRUCT_MOB.Magic` (`H`). |
| `learned_skill_mask` | `BIGINT` | não | `0` | Máscara compatível com 32 bits; cache do fio por aprovação de `0008`. |
| `score_bonus` / `special_bonus` / `skill_bonus` | `INTEGER` | não | `0` | Pontos disponíveis (`STRUCT_MOB`, `H`). |
| `critical` / `save_mana` | `SMALLINT` | não | `0` | Campos unsigned-char do struct (`H`). |
| `recall_x` / `recall_y` | `SMALLINT` | sim | — | Posição de retorno `SPX`/`SPY` (`H`). |
| `content_version` | `SMALLINT` | não | — | Versão de conteúdo que criou personagem (`D-39`); FK a `content.content_version`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Criação e atualização via trigger (`A9`). |
| `deleted_at` | `TIMESTAMPTZ` | sim | — | Soft-delete; a linha fica, nome/vaga liberam (`A14`). |

### 6.2 Satélites 1:1

| Tabela/colunas | Tipos e defaults | Nulo | Origem / significado |
|---|---|---|---|
| `character_status.character_id` | `BIGINT` PK/FK | não | 1:1 ao personagem, `CASCADE`. |
| `hp`, `hp_max`, `mp_max` | `INTEGER`, defaults 1/1/0 | não | Atual e máximos; invariantes atuais ≤ máximo (`STRUCT_SCORE`, `H`). |
| `mp`, `pvp_points` | `INTEGER`, defaults 0 | não | Atual e fama/PvP (`H`). |
| `regen_hp`, `regen_mp`, `resist_*` | `SMALLINT`, defaults 0 | não | Regen e resistências fire/ice/thunder/magic (`STRUCT_MOB`, `H`). |
| `character_scores.character_id` | `BIGINT` PK/FK | não | 1:1, `CASCADE`. |
| `base_*`, `current_*` AC/damage | `INTEGER`, defaults 0 | não | Score base e efetivo (`STRUCT_SCORE`, `H`). |
| `base_*`, `current_*` run/direction/chaos | `SMALLINT`, defaults 0 | não | Campos unsigned-char do score (`H`). |
| `base_*`, `current_*` str/int/dex/con | `SMALLINT`, defaults 0 | não | Atributos base e efetivos (`STRUCT_SCORE`, `H`). |
| `character_special.character_id` | `BIGINT` PK/FK | não | 1:1, `CASCADE`. |
| `special_1..4` | `INTEGER`, defaults 0 | não | `STRUCT_SCORE.Special[4]` (`H`). |
| `character_short_skill.character_id` | `BIGINT` PK/FK | não | 1:1, `CASCADE`. |
| `slot_1..4` | `SMALLINT`, defaults 0 | não | Quatro bytes de barra curta (`H`). |

## 7. Contratos e invariantes

| Objeto | Estado que impede | SQLSTATE |
|---|---|---|
| `ux_character_nome` / `ux_character_vaga` | nome dobrado ou vaga duplicada entre personagens vivos | `23505` |
| `fk_character_account` / `fk_character_content_version` | dono ou versão de conteúdo inexistente | `23503` |
| `ck_character_*` | vaga, nome, classe, tier, nível, moeda, máscara, bônus, bytes ou timestamp fora da faixa | `23514` |
| FK/PK dos satélites | estado duplicado ou de personagem inexistente | `23505` / `23503` |
| `ck_character_status_hp` / `_mp` | HP/MP acima do máximo ou negativo | `23514` |
| `ck_character_scores_*`, `ck_character_special`, `ck_character_short_skill` | atributos negativos ou valores de byte fora de 0–255 | `23514` |
| FK `ON DELETE CASCADE` dos satélites | deixa que remoção física de personagem de teste remova seus filhos; fluxo operacional usa `deleted_at` | cascata |

## 8. Permissões

Default privileges de `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_game` | `SELECT, INSERT, UPDATE, DELETE` nas cinco tabelas | default privilege `game` |
| `wyd_read` | `SELECT` | default privilege `game` |
| `wyd_portal` | nenhum acesso cru; sem `USAGE` em `game` | fronteira `P-17`; consumo futuro por view de `0029` |
| `PUBLIC` | nenhum acesso | provisionamento/`0001` |

## 9. Prova de execução

PG17 efêmero, `0001`, `0005`, `content.content_version` aplicada/semeada e pessoa/conta de teste já
existentes. O `0007` exige um `server_channel` de fixture antes da sessão, mas não para este lote.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0006_game_character.sql
```

| Prova | Comando/consulta | Esperado |
|---|---|---|
| cinco tabelas | contar nomes do lote em `information_schema.tables` | `5` |
| personagem sem conta | inserir `account_id=999` | `23503` em `fk_character_account` |
| vaga inválida | inserir `slot=4` | `23514` em `ck_character_vaga` |
| nome igual sem caixa | dois personagens vivos com `Kersef`/`kersef` | segundo `23505` em `ux_character_nome` |
| nome/vaga reutilizáveis | marcar o primeiro `deleted_at`, inserir outro com mesmo nome e vaga | sucesso |
| HP acima do máximo | inserir status `hp=2, hp_max=1` | `23514` em `ck_character_status_hp` |
| especiais negativos | inserir `special_1=-1` | `23514` em `ck_character_special` |
| `updated_at` avança | atualizar personagem e reler timestamp | trigger o torna posterior |
| `down` remove filhos e pai | executar `down`, contar cinco tabelas | `0` |

## 10. O que este arquivo não decide

| Assunto | Fonte dona | Efeito |
|---|---|---|
| Campos byte a byte do cliente | `STRUCT_MOB`/`STRUCT_SCORE` do cliente 7662, status `H` até captura/catálogo verificado | nomes do schema são normalização atual, não afirmação nova sobre wire |
| Skills aprendidas e coerência da máscara | lote `0008` confirmado | `learned_skill_mask` é cache do fio; tabela de skills é fonte |
| Itens/equip/carry | lote `0009` | nenhuma coluna de item aqui |
| Guilda | lote `0010` | `Guild`/`GuildLevel` não são duplicados aqui |
| Fórmulas e limites por classe | `wyd-core`/`wyd-sim`, ainda a planejar | este schema só aplica faixas absolutas já decididas |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*
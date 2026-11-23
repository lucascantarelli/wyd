> Pergunta: "quais tabelas o mundo do cliente precisa no banco, com que colunas, e o que nelas é fonte e o que é hipótese?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 19) e §4; `ddl-confirmado.md` §5.21 (aprovado em 2026-11-20); fichas de tabela do legado em `deprecated/.agents/prompts/análises/wyd-db/database/tabelas/`
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-23)

Décimo nono arquivo da série e o maior lote: **13 tabelas** de `content` para mundo, NPC, loja,
configuração e zona de guilda. Todas as colunas são **`H`** — vêm das fichas do legado, não de
documento de coluna do planejamento novo —, exceto as de `content_source` (`plano-migrations.md` §4)
e os campos de ciclo de vida que são padrão da série.

!!! note "Aprovado em 2026-11-20"
    O dono respondeu as seis decisões de forma em 2026-11-20 (chave natural onde o insumo tem código;
    blob do NPC fora; array para bloco numérico e `JSONB` só no heterogêneo; `guild_zone` só com
    definição; FK de item em `level_item`/`world_item_seed`; `map_id` em `world_item_seed`) e revisou
    as colunas `H` antes de aprovar: `merchant`, `eff1..eff3`/`effv1..effv3`, os cinco blocos crus,
    `rotate` e o grupo de gerador do NPC foram todos mantidos. O lote conta em `ddl-confirmado.md` §1
    (75 na época; hoje 109 de 109) e esta página fica `concluída` em `migrations-checklist.md` §2.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0019` (próximo lote sem desenho, `migrations-checklist.md` §2, linha 102) |
| Arquivo | `crates/wyd-db/migrations/0019_content_world.sql` |
| Schema | `content` (13 tabelas) |
| Posição | 19 de 34 (mundo do conteúdo) |
| Depende de | `0001` (schema `content`, `ops.fn_set_updated_at`, default privileges), `0013` (`content.fn_fold`, `content.game_class`), `0014` (`content.item_catalog`) |
| É dependência de | `0020` (seed do mundo, gerado), `0024`/`0026` por conteúdo, `0028` (estado de guerra de `guild_zone`) |
| Tabelas criadas | 13: `content_source`, `npc_template`, `npc_definition`, `npc_shop_item`, `level_item`, `guard_spawn`, `guard_spawn_reward`, `mob_merc`, `game_setting`, `game_matrix`, `region`, `world_item_seed`, `guild_zone` |
| Objetos criados | 13 tabelas, 5 triggers de `updated_at`, 6 índices (1 único parcial) |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Dar forma ao conteúdo de mundo que o servidor lê no boot e o portal edita com auditoria: quem é cada
NPC e onde ele nasce, o que a loja dele vende, o que cada nível entrega, o que o guarda exige, o
mercador, os parâmetros e matrizes do `gameconfig.txt`, as regiões de PvP/siege, os itens no chão e as
zonas de guilda — sem tocar em `game` e sem criar FK para schema posterior.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schema `content`, `ops.fn_set_updated_at()` e default privileges.
2. `0013_content_taxonomy` aplicado: `content.fn_fold()` existe (usado por `region.name_fold`).
3. `0014_content_item` aplicado: `content.item_catalog` existe (FK de `level_item` e `world_item_seed`).
4. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.21, copiado sem alteração.

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

## 5. DDL do `down`

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

Sem `GRANT`/`REVOKE`: o lote herda o default privilege de `0001` e não cria papel. Sem `COMMENT ON`,
como toda a série: a semântica de coluna vive nesta página.

## 6. Colunas, com origem

`Origem`: **`P`** = planejado em documento do novo planejamento; **`H`** = hipótese lida da ficha do
legado, pendente de trace do servidor; **série** = campo de ciclo de vida que é padrão de todas as
tabelas de conteúdo (`enabled`, `created_at`, `updated_at`).

### 6.1 `content.content_source` — 7 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `path` | `TEXT` | não | — | `P` | Caminho do insumo, relativo à raiz (`plano-migrations.md` §4). |
| `sha256` | `TEXT` | não | — | `P` | Hash do arquivo importado; prova de que o dado é o mesmo. |
| `size_bytes` | `INTEGER` | não | — | `P` | Tamanho do arquivo em bytes. |
| `row_count` | `INTEGER` | não | — | `P` | Linhas aproveitadas do arquivo. |
| `loaded_by` | `TEXT` | não | — | `H` | Quem/o que carregou (script ou migration). |
| `transform` | `TEXT` | não | `''` | `H` | Transformação aplicada ao insumo. |
| `imported_at` | `TIMESTAMPTZ` | não | `now()` | `H` | Instante da importação. |

### 6.2 `content.npc_template` — 6 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `template_name` | `TEXT` | não | — | `H` | Nome do template (chave do `STRUCT_MOB`); PK. |
| `file_name` | `TEXT` | não | — | `H` | Arquivo de origem do template. |
| `source_size` | `SMALLINT` | não | — | `H` | Tamanho declarado na origem; 222 linhas do legado divergem (756/920 em vez de 816). |
| `sha256` | `TEXT` | não | — | `H` | Hash do blob de origem (o blob não entra). |
| `encoding_origin` | `TEXT` | não | `'utf8'` | `H` | Codificação do nome na origem (`utf8` ou `cp949-as-latin1`). |
| `name_recovered` | `TEXT` | sim | — | `H` | Nome recuperado por decodificação alternativa (20 casos). |

### 6.3 `content.npc_definition` — 21 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `id` | `INTEGER` | não | — | `H` | Código do NPC no insumo (NPCGener); casa com `game.item_price.npc_id` (`0011`). |
| `slug` | `TEXT` | não | — | `H` | Identificador textual estável para o portal. |
| `template_name` | `TEXT` | não | — | `H` | FK `npc_template`: corpo do NPC. |
| `display_name` | `TEXT` | não | `''` | `H` | Nome exibido ao jogador. |
| `enabled` | `BOOLEAN` | não | `true` | série | Aposentadoria sem `DELETE`. |
| `map_id` | `SMALLINT` | não | `0` | `H` | Mapa do cliente. |
| `pos_x` / `pos_y` | `SMALLINT` | não | `0` | `H` | Coordenada no mapa. |
| `route_type` | `SMALLINT` | não | `0` | `H` | Tipo de rota/patrulha do gerador. |
| `merchant` | `SMALLINT` | não | `0` | `H` | Campo Merchant (0 = não comerciante); acima de 0 é UNVERIFIED. |
| `origin` | `TEXT` | não | `'content'` | `H` | `content` ou `custom`; `custom` não é sobrescrito pelo import. |
| `generator_index` | `INTEGER` | sim | — | `H` | Índice do gerador no arquivo; nulo no NPC fixo. |
| `follower_template` | `TEXT` | não | `''` | `H` | Template do seguidor/invocação. |
| `minute_generate` | `INTEGER` | não | `0` | `H` | Intervalo de geração em minutos. |
| `min_group` / `max_group` | `SMALLINT` | não | `0` | `H` | Tamanho mínimo e máximo do grupo gerado. |
| `max_num_mob` | `SMALLINT` | não | `1` | `H` | Teto de mobs vivos do gerador. |
| `formation` | `SMALLINT` | não | `0` | `H` | Formação do grupo gerado. |
| `generator_data` | `JSONB` | não | `'{}'` | `H` | Bloco cru do gerador ainda não decodificado (`P-09`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | série | Carimbos; `updated_at` pelo trigger. |

### 6.4 `content.npc_shop_item` — 10 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `npc_id` | `INTEGER` | não | — | `H` | FK `npc_definition` (`CASCADE`): dono da loja. |
| `slot` | `SMALLINT` | não | — | `H` | Posição no contêiner da loja (0–26). |
| `item_index` | `INTEGER` | não | — | `H` | Item vendido; **sem FK**, índice do catálogo não provado. |
| `eff1`…`eff3` | `SMALLINT` | não | `0` | `H` | Código de efeito do bloco 1–3. |
| `effv1`…`effv3` | `SMALLINT` | não | `0` | `H` | Valor do efeito do bloco 1–3. |
| `quantity` | `SMALLINT` | não | `1` | `H` | Quantidade disponível. |

### 6.5 `content.level_item` — 8 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `ordinal` | `SMALLINT` | não | — | `H` | Ordem após deduplicar o bloco repetido do arquivo; PK. |
| `source_seq` | `SMALLINT` | não | — | `H` | Sequência no arquivo de origem. |
| `source_line` | `INTEGER` | não | — | `H` | Linha do arquivo (rastreabilidade). |
| `level` | `SMALLINT` | não | — | `H` | Nível. |
| `class_id` | `SMALLINT` | não | — | `H` | Classe 0–3; sem FK porque `game_class` tem `slug`, não código. |
| `type_id` | `SMALLINT` | não | — | `H` | TYPE do LevelItem.txt (−1 a 3). |
| `item_index` | `INTEGER` | não | — | `H` | Item entregue; FK `item_catalog`. |
| `extras` | `JSONB` | não | `'[]'` | `H` | Bloco posicional extra não decodificado. |

### 6.6 `content.guard_spawn` e 6.7 `content.guard_spawn_reward` — 3 colunas cada

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `guard_index` | `SMALLINT` | não | — | `H` | Índice do guarda no `Guard.txt`; PK (e FK na tabela filha, `CASCADE`). |
| `npc_id` | `INTEGER` | não | — | `H` | FK `npc_definition`: NPC que representa o guarda. |
| `source_line` | `INTEGER` | não | — | `H` | Linha do arquivo. |
| `req_slot` | `SMALLINT` | não | — | `H` | Posição da exigência no bloco (0–7); PK composta com `guard_index`. |
| `req_values` | `SMALLINT[]` | não | — | `H` | 8 números posicionais, sem semântica provada; tamanho fixo no CHECK. |

### 6.8 `content.mob_merc` — 5 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `generator_index` | `INTEGER` | não | — | `H` | Índice do gerador do mercador; PK. |
| `template_name` | `TEXT` | não | — | `H` | FK `npc_template`: template do mercador. |
| `renew_seconds` | `INTEGER` | não | — | `H` | Intervalo de renovação do estoque. |
| `reborn_seconds` | `INTEGER` | não | — | `H` | Tempo de renascimento. |
| `items` | `JSONB` | não | `'[]'` | `H` | Lista de itens ainda não normalizada (`P-09`). |

### 6.9 `content.game_setting` — 7 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `section` | `TEXT` | não | — | `H` | Seção do `gameconfig.txt`; PK com `key`. |
| `key` | `TEXT` | não | — | `H` | Chave do parâmetro dentro da seção. |
| `value_int` | `BIGINT` | sim | — | `H` | Valor inteiro, só quando o parâmetro é um inteiro único. |
| `value_text` | `TEXT` | não | `''` | `H` | Valor textual (`evstart` tem 4 números). |
| `source_line` | `INTEGER` | não | — | `H` | Linha do arquivo. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | série | Carimbos; é a configuração editável pelo portal. |

### 6.10 `content.game_matrix` — 5 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `section` | `TEXT` | não | — | `H` | Seção do `gameconfig.txt`; PK composta. |
| `block_index` | `SMALLINT` | não | — | `H` | Bloco da matriz dentro da seção. |
| `line_index` | `SMALLINT` | não | — | `H` | Linha dentro do bloco. |
| `row_values` | `SMALLINT[]` | não | — | `H` | Valores na ordem do arquivo, sem nome por posição. |
| `source_line` | `INTEGER` | não | — | `H` | Linha do arquivo. |

### 6.11 `content.region` — 11 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | `H` | Identidade; o arquivo não numera a região de forma estável. |
| `name` | `TEXT` | não | — | `H` | Nome exibido; repetido é legítimo. |
| `name_fold` | `TEXT` gerado | não | `fn_fold(name)` | série | Busca sem acento/caixa (`P-11`). |
| `x1` / `y1` / `x2` / `y2` | `INTEGER` | não | — | `H` | Retângulo da região. |
| `source_line` | `INTEGER` | não | — | `H` | Linha do `Regions.txt`. |
| `enabled` | `BOOLEAN` | não | `true` | série | Aposentadoria sem `DELETE`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | série | Carimbos; `updated_at` pelo trigger. |

### 6.12 `content.world_item_seed` — 9 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | `H` | Identidade; o insumo não tem chave. |
| `map_id` | `SMALLINT` | não | — | `P` | Mapa do item no chão (0–255, igual a `game.city.map_id`); **não existia no legado**. |
| `item_index` | `INTEGER` | não | — | `H` | Item no chão; FK `item_catalog`. |
| `pos_x` / `pos_y` | `SMALLINT` | não | — | `H` | Coordenada do item. |
| `rotate` | `SMALLINT` | não | `0` | `H` | Rotação/ângulo do item no mapa. |
| `enabled` | `BOOLEAN` | não | `true` | `H` | Linha ativa sem apagar. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | série | Carimbos; `updated_at` pelo trigger. |

### 6.13 `content.guild_zone` — 7 colunas

| Coluna | Tipo | Nulo | Default | Origem | Significado |
|---|---|---|---|---|---|
| `clan` | `SMALLINT` | não | — | `H` | Código de clã do cliente (não FK para `game.guild`); PK com `zone`. |
| `zone` | `SMALLINT` | não | — | `H` | Identificador da zona. |
| `victory` | `SMALLINT` | não | `0` | `H` | Condição de vitória da disputa. |
| `city_tax` | `SMALLINT` | não | `0` | `H` | Imposto da cidade associada à zona. |
| `enabled` | `BOOLEAN` | não | `true` | série | Aposentadoria sem `DELETE`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | série | Carimbos; `updated_at` pelo trigger. |

**Fora de propósito:** `charge_guild`, `challenge_guild`, `challenge_money` e `tax_vault` saem do
conteúdo porque são estado de guerra mutável; entram no lote de `game` (`0028`).

## 7. Contratos e invariantes

| # | Objeto | Proíbe | Erro |
|---:|---|---|---|
| 1 | `pk_*` das 13 tabelas | chave repetida | `23505` |
| 2 | `uq_region_retangulo` | duas regiões com o mesmo nome e o mesmo retângulo | `23505` |
| 3 | `uq_world_item_seed_posicao` | o mesmo item duas vezes no mesmo tile do mesmo mapa | `23505` |
| 4 | `ux_npc_definition_gerador` | dois NPCs no mesmo índice de gerador do mesmo mapa | `23505` |
| 5 | `ck_npc_template_tamanho` | `source_size` fora de 756/816/920 | `23514` |
| 6 | `ck_content_source_sha256` | hash que não é 64 hex | `23514` |
| 7 | `ck_npc_definition_grupo` | `min_group > max_group` | `23514` |
| 8 | `ck_npc_shop_item_item` | item de loja ≤ 0 (o legado tinha um negativo) | `23514` |
| 9 | `ck_guard_spawn_reward_valores` | bloco de exigência com tamanho ≠ 8 | `23514` |
| 10 | `ck_game_setting_secao` / `ck_game_matrix_secao` | seção vazia | `23514` |
| 11 | `ck_region_retangulo` | retângulo invertido (`x2 < x1` ou `y2 < y1`) | `23514` |
| 12 | `fk_npc_definition_template` | NPC com template inexistente | `23503` |
| 13 | `fk_npc_shop_item_npc` | loja de NPC inexistente | `23503` |
| 14 | `fk_guard_spawn_reward_guarda` | exigência de guarda inexistente | `23503` |
| 15 | `fk_level_item_catalogo` / `fk_world_item_seed_catalogo` | item fora do catálogo (`0014`) | `23503` |

**Cascatas:** `npc_shop_item` e `guard_spawn_reward` caem com o pai (linha possuída, `P-15`); template
e catálogo usam `RESTRICT`, porque apagar conteúdo referenciado é decisão de seed, não de runtime.

## 8. Permissões

Nenhum `GRANT`/`REVOKE` novo: valem os default privileges de `0001` §3.

| Papel | Efeito |
|---|---|
| `wyd_game` | `SELECT` em `content`: lê NPC, loja, configuração e mundo no boot. |
| `wyd_portal` | `SELECT` e, na configuração, escrita auditada em `ops.config_change` (`0002`). |
| `wyd_read` | `SELECT`; não lê segredo porque não há segredo neste lote. |
| `wyd_migrator` | dono da criação; aplica o arquivo. |

## 9. Origem, `H` e o que falta provar

- **Fonte de conteúdo:** fichas de `deprecated/.agents/prompts/análises/wyd-db/database/tabelas/`
  (`content_source.md`, `npc_template.md`, `npc_definition.md`, `npc_shop_item.md`, `level_item.md`,
  `guard_spawn.md`, `guard_spawn_reward.md`, `mob_merc.md`, `game_setting.md`, `game_matrix.md`,
  `region.md`, `world_item_seed.md`, `guild_zone.md`), geradas do `0008_conteudo_mundo.sql` do legado.
  Nível 4: inspiram, não decidem.
- **`H` a provar com trace:** semântica de `merchant` acima de 0; conteúdo de `generator_data`,
  `req_values`, `items` e `extras`; correspondência `class_id` 0–3 → `game_class.slug`; se todo
  `npc_shop_item.item_index` existe em `item_catalog`; se `rotate` é ângulo ou índice.
- **`P` de verdade:** a lista das 13 tabelas e a ordem (`plano-migrations.md` §3), as colunas de
  `content_source` (§4), `map_id` de `world_item_seed` (decisão de 2026-11-20) e os campos de série.

## 10. Prova de execução (quando a série for escrita)

1. `psql -v ON_ERROR_STOP=1 -f 0019_content_world.sql` como `wyd_migrator` → 13 `CREATE TABLE`,
   5 `CREATE TRIGGER` e os índices listados.
2. `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'content';` → **13** neste
   arquivo, somando ao que `0013`/`0014`/`0015` já criaram.
3. `INSERT` de NPC com `template_name` inexistente → `23503` em `fk_npc_definition_template`.
4. Dois NPCs com o mesmo `(map_id, generator_index)` não nulo → `23505` em `ux_npc_definition_gerador`.
5. `INSERT` em `guard_spawn_reward` com `req_values` de 7 posições → `23514` em
   `ck_guard_spawn_reward_valores`.
6. `INSERT` em `level_item` com `item_index` fora do catálogo → `23503` em `fk_level_item_catalogo`.
7. `SET ROLE wyd_game; INSERT INTO content.game_setting …` → `42501` (`content` é leitura para o jogo).
8. `UPDATE content.region SET name = name` → `updated_at` muda pelo trigger.

## 11. O que este arquivo não decide

| Assunto | Onde fecha |
|---|---|
| semântica dos blocos UNVERIFIED (`merchant`, `generator_data`, `req_values`, `items`, `extras`) | trace do `wyd-server`, depois da implementação |
| normalização de `mob_merc.items` em tabela filha | lote futuro, com decisão própria |
| estado de guerra de `guild_zone` (desafio, cofre, imposto acumulado) | lote de `game` (`0028`) |
| correspondência `class_id` → `slug` de classe | seed `0017`/`0018`, curadoria |
| catálogo de chaves editáveis de `game_setting` | planejamento do `wyd-portal` + `0002` |
| seed do mundo (contagens, idempotência, `content_source`) | `0020` (escrito à mão), bloqueado pela verificação do dado de origem |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

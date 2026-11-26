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
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE content.guild_zone, content.world_item_seed, content.region, content.game_matrix,
           content.game_setting, content.mob_merc, content.guard_spawn_reward, content.guard_spawn,
           content.level_item, content.npc_shop_item, content.npc_definition, content.npc_template,
           content.content_source;

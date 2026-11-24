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

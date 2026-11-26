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
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE content.accessory_refine_rule, content.combine_value, content.sanc_rate,
           content.refine_gem_effect, content.refine_rate;

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

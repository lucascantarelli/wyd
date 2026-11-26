-- migrate:up
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
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE game.character_chaos, game.character_affect, game.character_lineage,
           game.character_learned_skill, game.character_skill;

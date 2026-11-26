-- migrate:up
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
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE game.guild_tower, game.guild_relation, game.guild_member, game.guild;

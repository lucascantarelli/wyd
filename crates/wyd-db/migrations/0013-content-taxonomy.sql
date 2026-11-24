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

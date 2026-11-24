-- migrate:up
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

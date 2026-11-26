-- 0014_content_item: catálogo de itens do cliente (3.220 linhas no seed de 0017).
-- Sem FK de saída (plano-migrations.md §3.1, item 5): `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.item_catalog (
    item_index   INTEGER NOT NULL,
    name         TEXT NOT NULL,
    -- P-11: dobra única no banco, indexável. `content.fn_fold` é IMMUTABLE desde o `0013`.
    name_fold    TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    -- NULL = categoria não provada. Categoria sintética no seed seria inventar dado.
    kind_slug    TEXT,
    -- Símbolos `EF_*`. `TEXT[]` não aceita FK: quem garante o conjunto é o trigger abaixo.
    effect_names TEXT[] NOT NULL DEFAULT '{}',
    base_price   BIGINT NOT NULL DEFAULT 0,
    -- Número do cliente, não letra: a correspondência N/M/A/Le/Anct/E não está provada.
    grade        SMALLINT NOT NULL,
    req_level    SMALLINT NOT NULL DEFAULT 0,
    req_str      SMALLINT NOT NULL DEFAULT 0,
    req_int      SMALLINT NOT NULL DEFAULT 0,
    req_dex      SMALLINT NOT NULL DEFAULT 0,
    req_con      SMALLINT NOT NULL DEFAULT 0,
    -- Aposentadoria sem DELETE: linha referenciada por character_item/item_price não pode sumir.
    enabled      BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_catalog PRIMARY KEY (item_index),
    CONSTRAINT ck_item_catalog_indice CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_item_catalog_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_item_catalog_preco CHECK (base_price >= 0),
    CONSTRAINT ck_item_catalog_grade CHECK (grade >= 0),
    CONSTRAINT ck_item_catalog_requisitos CHECK (
        req_level BETWEEN 0 AND 32767 AND req_str BETWEEN 0 AND 32767
        AND req_int BETWEEN 0 AND 32767 AND req_dex BETWEEN 0 AND 32767
        AND req_con BETWEEN 0 AND 32767),
    CONSTRAINT ck_item_catalog_efeitos CHECK (array_position(effect_names, NULL) IS NULL),
    CONSTRAINT fk_item_catalog_kind FOREIGN KEY (kind_slug)
        REFERENCES content.item_kind (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Efeito repetido ou símbolo fora de `content.item_effect` não passam: sem isto o GIN indexa lixo.
CREATE FUNCTION content.fn_item_catalog_effects() RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    faltando TEXT;
BEGIN
    IF cardinality(NEW.effect_names) = 0 THEN
        RETURN NEW;
    END IF;
    IF cardinality(NEW.effect_names)
       <> (SELECT count(DISTINCT e) FROM unnest(NEW.effect_names) AS t(e)) THEN
        RAISE EXCEPTION 'item % repete efeito em effect_names', NEW.item_index
            USING ERRCODE = '23514', CONSTRAINT = 'ck_item_catalog_efeitos';
    END IF;
    SELECT e INTO faltando
      FROM unnest(NEW.effect_names) AS t(e)
     WHERE NOT EXISTS (SELECT 1 FROM content.item_effect ie WHERE ie.slug = t.e)
     LIMIT 1;
    IF FOUND THEN
        RAISE EXCEPTION 'efeito % nao existe em content.item_effect', faltando
            USING ERRCODE = '23503', CONSTRAINT = 'fk_item_catalog_efeito';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_item_catalog_effects
    BEFORE INSERT OR UPDATE OF effect_names ON content.item_catalog
    FOR EACH ROW EXECUTE FUNCTION content.fn_item_catalog_effects();

CREATE TRIGGER trg_item_catalog_set_updated_at
    BEFORE UPDATE ON content.item_catalog
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- Busca por prefixo do nome dobrado, filtro por categoria e filtro por efeito (P-11 e o GIN do plano).
CREATE INDEX ix_item_catalog_name_fold ON content.item_catalog (name_fold text_pattern_ops);
CREATE INDEX ix_item_catalog_kind ON content.item_catalog (kind_slug);
CREATE INDEX ix_item_catalog_effect_names ON content.item_catalog USING GIN (effect_names);
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE content.item_catalog;
-- A função do trigger não tem mais gatilho depois da tabela; o trigger caiu junto com ela.
DROP FUNCTION content.fn_item_catalog_effects();

-- 0015_content_skill: catálogo de skills do cliente (154 linhas no seed de 0018).
-- Sem FK de saída (plano-migrations.md §3.1, item 5): `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.skill_catalog (
    skill_index        SMALLINT NOT NULL,
    name               TEXT NOT NULL,
    -- P-11: dobra única no banco, indexável. `content.fn_fold` é IMMUTABLE desde o `0013`.
    name_fold          TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    -- NULL = vínculo não provado: das 154 skills, 63 têm linhagem medida.
    class_slug         TEXT,
    lineage_slug       TEXT,
    skill_point        SMALLINT NOT NULL DEFAULT 0,
    mana_spent         SMALLINT NOT NULL DEFAULT 0,
    -- `delay_ticks` e `affect_time` ficam brutos, como vêm do SkillData.csv. O runtime aplica a
    -- conversão que o cliente faz na carga; o catálogo não traduz (mesma regra do `grade` em `0014`).
    delay_ticks        SMALLINT NOT NULL DEFAULT 0,
    affect_time        SMALLINT NOT NULL DEFAULT 0,
    range_tiles        SMALLINT NOT NULL DEFAULT 0,
    target_type        SMALLINT NOT NULL DEFAULT 0,
    instance_type      SMALLINT NOT NULL DEFAULT 0,
    instance_value     SMALLINT NOT NULL DEFAULT 0,
    tick_type          SMALLINT NOT NULL DEFAULT 0,
    tick_value         SMALLINT NOT NULL DEFAULT 0,
    affect_type        SMALLINT NOT NULL DEFAULT 0,
    affect_value       SMALLINT NOT NULL DEFAULT 0,
    instance_attribute SMALLINT NOT NULL DEFAULT 0,
    tick_attribute     SMALLINT NOT NULL DEFAULT 0,
    -- O CSV traz 0/1; BOOLEAN diz o que o campo é. Valor fora de 0/1 seria lote novo.
    aggressive         BOOLEAN NOT NULL DEFAULT false,
    max_target         SMALLINT NOT NULL DEFAULT 1,
    party              BOOLEAN NOT NULL DEFAULT false,
    affect_resist      SMALLINT NOT NULL DEFAULT 0,
    passive            BOOLEAN NOT NULL DEFAULT false,
    -- Aposentadoria sem DELETE: linha referenciada por progressão de personagem não pode sumir.
    enabled            BOOLEAN NOT NULL DEFAULT true,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_skill_catalog PRIMARY KEY (skill_index),
    CONSTRAINT ck_skill_catalog_indice CHECK (skill_index BETWEEN 0 AND 247),
    CONSTRAINT ck_skill_catalog_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_skill_catalog_nao_negativos CHECK (
        skill_point >= 0 AND mana_spent >= 0 AND delay_ticks >= 0 AND affect_time >= 0
        AND range_tiles >= 0 AND max_target >= 0),
    CONSTRAINT fk_skill_catalog_class FOREIGN KEY (class_slug)
        REFERENCES content.game_class (slug) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_skill_catalog_lineage FOREIGN KEY (lineage_slug)
        REFERENCES content.skill_lineage (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TRIGGER trg_skill_catalog_set_updated_at
    BEFORE UPDATE ON content.skill_catalog
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- Busca por prefixo do nome dobrado e filtro por classe/linhagem (P-11 e a leitura do portal).
CREATE INDEX ix_skill_catalog_name_fold ON content.skill_catalog (name_fold text_pattern_ops);
CREATE INDEX ix_skill_catalog_afinidade ON content.skill_catalog (class_slug, lineage_slug);
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE content.skill_catalog;

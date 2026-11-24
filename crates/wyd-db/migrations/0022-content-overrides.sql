-- 0022_content_overrides: edição do staff sobre refino, SANC e composição.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Override sem linha base é impossível; apagar a linha de override é o reset para o valor da base.

-- migrate:up
CREATE TABLE content.refine_rate_override (
    kind       TEXT NOT NULL,
    step       SMALLINT NOT NULL,
    rate_bps   INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_refine_rate_override PRIMARY KEY (kind, step),
    CONSTRAINT fk_refine_rate_override_base FOREIGN KEY (kind, step)
        REFERENCES content.refine_rate (kind, step) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_refine_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

CREATE TABLE content.sanc_rate_override (
    powder       TEXT NOT NULL,
    refine_index SMALLINT NOT NULL,
    rate_bps     INTEGER NOT NULL,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_sanc_rate_override PRIMARY KEY (powder, refine_index),
    CONSTRAINT fk_sanc_rate_override_base FOREIGN KEY (powder, refine_index)
        REFERENCES content.sanc_rate (powder, refine_index) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_sanc_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- Same typed value as the base table: exactly one of rate_bps/coin, checked here too (FK does not
-- constrain values). Per-row replacement: a row here replaces the whole base row for its key.
CREATE TABLE content.combine_value_override (
    family     TEXT NOT NULL,
    key        TEXT NOT NULL,
    rate_bps   INTEGER,
    coin       BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_combine_value_override PRIMARY KEY (family, key),
    CONSTRAINT fk_combine_value_override_base FOREIGN KEY (family, key)
        REFERENCES content.combine_value (family, key) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_combine_value_override_one_unit CHECK (
        (rate_bps IS NOT NULL AND coin IS NULL AND rate_bps BETWEEN 0 AND 10000)
        OR (rate_bps IS NULL AND coin IS NOT NULL AND coin >= 0)
    )
);

-- 0001 grants only SELECT on `content`. The portal admin area is the only runtime that edits
-- configuration (M21, §5.9): DELETE here is the reset to the base value.
GRANT SELECT, INSERT, UPDATE, DELETE ON content.refine_rate_override TO wyd_portal;
GRANT SELECT, INSERT, UPDATE, DELETE ON content.sanc_rate_override TO wyd_portal;
GRANT SELECT, INSERT, UPDATE, DELETE ON content.combine_value_override TO wyd_portal;

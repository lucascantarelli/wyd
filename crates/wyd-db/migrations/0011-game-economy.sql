-- migrate:up
CREATE TABLE game.city (
    code          TEXT NOT NULL,
    name          TEXT NOT NULL,
    -- Mapa da cidade não é publicado pelo cliente 7662 (`H`); fica nulo até prova de trace.
    map_id        SMALLINT,
    is_capital    BOOLEAN NOT NULL DEFAULT false,
    tax_rate_bps  INTEGER NOT NULL DEFAULT 1000,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_city PRIMARY KEY (code),
    CONSTRAINT ck_city_code CHECK (code ~ '^[a-z][a-z0-9_]{2,15}$'),
    CONSTRAINT ck_city_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 32),
    CONSTRAINT ck_city_mapa CHECK (map_id IS NULL OR map_id BETWEEN 0 AND 255),
    CONSTRAINT ck_city_imposto CHECK (tax_rate_bps BETWEEN 0 AND 10000)
);
CREATE TRIGGER trg_city_set_updated_at
    BEFORE UPDATE ON game.city
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.item_price (
    item_index INTEGER NOT NULL,
    npc_id     INTEGER NOT NULL,
    buy_price  BIGINT NOT NULL DEFAULT 0,
    sell_price BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_price PRIMARY KEY (item_index, npc_id),
    CONSTRAINT ck_item_price_compra CHECK (buy_price >= 0),
    CONSTRAINT ck_item_price_venda CHECK (sell_price >= 0)
);

-- Sem FK para `content.item_catalog`/`content.npc_definition` (0014/0019): conteúdo é posterior.
CREATE INDEX ix_item_price_npc ON game.item_price (npc_id);
CREATE TRIGGER trg_item_price_set_updated_at
    BEFORE UPDATE ON game.item_price
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.personal_shop (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    character_id BIGINT NOT NULL,
    city_code    TEXT NOT NULL,
    name         TEXT NOT NULL,
    is_open      BOOLEAN NOT NULL DEFAULT true,
    tax_bps      INTEGER NOT NULL DEFAULT 1000,
    tax_paid     BIGINT NOT NULL DEFAULT 0,
    opened_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at    TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_personal_shop PRIMARY KEY (id),
    CONSTRAINT fk_personal_shop_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_personal_shop_city FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_personal_shop_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 24),
    CONSTRAINT ck_personal_shop_imposto CHECK (tax_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_personal_shop_pago CHECK (tax_paid >= 0),
    CONSTRAINT ck_personal_shop_fechamento CHECK (closed_at IS NULL OR closed_at >= opened_at),
    -- Aberta não tem data de fechamento; fechada tem.
    CONSTRAINT ck_personal_shop_estado CHECK (is_open = (closed_at IS NULL))
);

-- Uma loja aberta por personagem e a busca por cidade são os dois acessos quentes.
CREATE UNIQUE INDEX ux_personal_shop_aberta ON game.personal_shop (character_id) WHERE is_open;
CREATE INDEX ix_personal_shop_cidade ON game.personal_shop (city_code) WHERE is_open;
CREATE TRIGGER trg_personal_shop_set_updated_at
    BEFORE UPDATE ON game.personal_shop
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.city_tax_ledger (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    city_code   TEXT NOT NULL,
    shop_id     BIGINT,
    amount      BIGINT NOT NULL,
    reason      TEXT NOT NULL,
    collected_by BIGINT,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_city_tax_ledger PRIMARY KEY (id),
    CONSTRAINT fk_city_tax_ledger_city FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_city_tax_ledger_shop FOREIGN KEY (shop_id)
        REFERENCES game.personal_shop (id) ON DELETE SET NULL ON UPDATE NO ACTION,
    CONSTRAINT fk_city_tax_ledger_quem FOREIGN KEY (collected_by)
        REFERENCES game.character (id) ON DELETE SET NULL ON UPDATE NO ACTION,
    CONSTRAINT ck_city_tax_ledger_valor CHECK (amount >= 0),
    CONSTRAINT ck_city_tax_ledger_motivo CHECK (
        reason IN ('shop_sale', 'npc_sale', 'portal', 'collection')
    )
);

CREATE INDEX ix_city_tax_ledger_cidade ON game.city_tax_ledger (city_code, occurred_at DESC);

CREATE TABLE portal.donate_shop_item (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    item_index    INTEGER NOT NULL,
    price_credits INTEGER NOT NULL,
    duration_days SMALLINT,
    enabled       BOOLEAN NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_shop_item PRIMARY KEY (id),
    CONSTRAINT ck_donate_shop_item_preco CHECK (price_credits > 0),
    CONSTRAINT ck_donate_shop_item_duracao CHECK (duration_days IS NULL OR duration_days > 0)
);
CREATE TRIGGER trg_donate_shop_item_set_updated_at
    BEFORE UPDATE ON portal.donate_shop_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.daily_reward_item (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    day          SMALLINT,
    item_index   INTEGER,
    gold         BIGINT NOT NULL DEFAULT 0,
    qty          SMALLINT NOT NULL DEFAULT 1,
    account_wide BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_daily_reward_item PRIMARY KEY (id),
    CONSTRAINT ck_daily_reward_item_dia CHECK (day IS NULL OR day BETWEEN 1 AND 7),
    CONSTRAINT ck_daily_reward_item_ouro CHECK (gold >= 0),
    CONSTRAINT ck_daily_reward_item_qtd CHECK (qty > 0),
    -- Recompensa sem item e sem ouro não entrega nada.
    CONSTRAINT ck_daily_reward_item_conteudo CHECK (item_index IS NOT NULL OR gold > 0)
);
CREATE TRIGGER trg_daily_reward_item_set_updated_at
    BEFORE UPDATE ON portal.daily_reward_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.daily_reward_claim (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id     BIGINT NOT NULL,
    reward_item_id BIGINT,
    claim_date     DATE NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_daily_reward_claim PRIMARY KEY (id),
    -- Um resgate por conta e dia UTC: o `23505` é o sinal de "já resgatou".
    CONSTRAINT uq_daily_reward_claim_dia UNIQUE (account_id, claim_date),
    CONSTRAINT fk_daily_reward_claim_reward FOREIGN KEY (reward_item_id)
        REFERENCES portal.daily_reward_item (id) ON DELETE RESTRICT ON UPDATE NO ACTION
);

CREATE TABLE portal.kingdom_cape_price (
    id            SMALLINT NOT NULL DEFAULT 1,
    version       BIGINT NOT NULL DEFAULT 1,
    hekalotia_cost INTEGER NOT NULL DEFAULT 8,
    akelonia_cost  INTEGER NOT NULL DEFAULT 8,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_kingdom_cape_price PRIMARY KEY (id),
    CONSTRAINT ck_kingdom_cape_price_singleton CHECK (id = 1),
    CONSTRAINT ck_kingdom_cape_price_versao CHECK (version >= 1),
    CONSTRAINT ck_kingdom_cape_price_hekalotia CHECK (hekalotia_cost BETWEEN 4 AND 32),
    CONSTRAINT ck_kingdom_cape_price_akelonia CHECK (akelonia_cost BETWEEN 4 AND 32)
);
CREATE TRIGGER trg_kingdom_cape_price_set_updated_at
    BEFORE UPDATE ON portal.kingdom_cape_price
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.donate_credit (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    idempotency_key UUID NOT NULL,
    account_id      BIGINT NOT NULL,
    credits         BIGINT NOT NULL,
    created_by      BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_credit PRIMARY KEY (id),
    CONSTRAINT uq_donate_credit_idem UNIQUE (idempotency_key),
    CONSTRAINT ck_donate_credit_valor CHECK (credits > 0)
);

CREATE TABLE portal.donate_debit (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    idempotency_key UUID NOT NULL,
    account_id      BIGINT NOT NULL,
    credits         BIGINT NOT NULL,
    reason          TEXT NOT NULL DEFAULT 'portal',
    created_by      BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_debit PRIMARY KEY (id),
    CONSTRAINT uq_donate_debit_idem UNIQUE (idempotency_key),
    CONSTRAINT ck_donate_debit_valor CHECK (credits > 0),
    CONSTRAINT ck_donate_debit_motivo CHECK (reason IN ('portal', 'shop_purchase'))
);

CREATE INDEX ix_donate_debit_conta ON portal.donate_debit (account_id, created_at DESC);

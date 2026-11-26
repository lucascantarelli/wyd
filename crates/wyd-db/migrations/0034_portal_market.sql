-- 0034_portal_market: mercado do portal (10 tabelas em `portal`) e custódia do item (1 em `game`).
-- Depende de `0009` (`game.character_item`, `game.character`) e de `0001` (`ops.fn_set_updated_at`).

-- migrate:up
CREATE TABLE portal.market_wallet (
    account_id BIGINT NOT NULL,
    currency   TEXT NOT NULL DEFAULT 'portal_credit',
    balance    BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_wallet PRIMARY KEY (account_id),
    CONSTRAINT ck_market_wallet_moeda CHECK (currency IN ('portal_credit')),
    CONSTRAINT ck_market_wallet_saldo CHECK (balance >= 0)
);
CREATE TRIGGER trg_market_wallet_set_updated_at
    BEFORE UPDATE ON portal.market_wallet
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.market_ledger (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id      BIGINT NOT NULL,
    kind            TEXT NOT NULL,
    amount          BIGINT NOT NULL,
    balance_after   BIGINT NOT NULL,
    ref_kind        TEXT,
    ref_id          BIGINT,
    idempotency_key TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_ledger PRIMARY KEY (id),
    CONSTRAINT ck_market_ledger_tipo CHECK (kind IN (
        'recarga', 'compra', 'venda', 'taxa', 'retencao_lance', 'devolucao_lance', 'estorno'
    )),
    CONSTRAINT ck_market_ledger_valor CHECK (amount <> 0),
    CONSTRAINT ck_market_ledger_saldo CHECK (balance_after >= 0),
    CONSTRAINT ck_market_ledger_ref CHECK ((ref_kind IS NULL) = (ref_id IS NULL)),
    CONSTRAINT ck_market_ledger_ref_tipo CHECK (
        ref_kind IS NULL OR ref_kind IN ('anuncio', 'transacao', 'disputa', 'recarga')
    ),
    CONSTRAINT uq_market_ledger_idempotency UNIQUE (idempotency_key)
);
CREATE INDEX ix_market_ledger_conta_criado ON portal.market_ledger (account_id, created_at DESC);

CREATE TABLE portal.market_fee_rule (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    fee_bps              INTEGER NOT NULL,
    effective_from       TIMESTAMPTZ NOT NULL,
    created_by_person_id BIGINT,
    note                 TEXT NOT NULL DEFAULT '',
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_fee_rule PRIMARY KEY (id),
    CONSTRAINT uq_market_fee_rule_vigencia UNIQUE (effective_from),
    CONSTRAINT ck_market_fee_rule_bps CHECK (fee_bps BETWEEN 0 AND 10000)
);

CREATE TABLE portal.market_listing (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    seller_account_id BIGINT NOT NULL,
    character_id      BIGINT NOT NULL,
    item_index        INTEGER NOT NULL,
    item_kind_slug    TEXT,
    qty               INTEGER NOT NULL DEFAULT 1,
    refine            SMALLINT NOT NULL DEFAULT 0,
    ancient           BOOLEAN NOT NULL DEFAULT false,
    gem_effect        SMALLINT NOT NULL DEFAULT 0,
    gem_value         SMALLINT NOT NULL DEFAULT 0,
    flags             INTEGER NOT NULL DEFAULT 0,
    kind              TEXT NOT NULL,
    price             BIGINT NOT NULL,
    buyout_price      BIGINT,
    fee_bps           INTEGER NOT NULL,
    ends_at           TIMESTAMPTZ,
    status            TEXT NOT NULL DEFAULT 'rascunho',
    escrow_ref        BIGINT,
    current_bid_id    BIGINT,
    bid_count         INTEGER NOT NULL DEFAULT 0,
    view_count        INTEGER NOT NULL DEFAULT 0,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_listing PRIMARY KEY (id),
    CONSTRAINT ck_market_listing_tipo CHECK (kind IN ('venda', 'leilao')),
    CONSTRAINT ck_market_listing_estado CHECK (status IN (
        'rascunho', 'ativo', 'vendido', 'expirado', 'cancelado', 'disputa'
    )),
    CONSTRAINT ck_market_listing_preco CHECK (price > 0),
    CONSTRAINT ck_market_listing_compra_ja CHECK (buyout_price IS NULL OR buyout_price >= price),
    CONSTRAINT ck_market_listing_prazo CHECK ((kind = 'leilao') = (ends_at IS NOT NULL)),
    CONSTRAINT ck_market_listing_custodia CHECK (status <> 'ativo' OR escrow_ref IS NOT NULL),
    CONSTRAINT ck_market_listing_qtd CHECK (qty >= 1),
    CONSTRAINT ck_market_listing_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_market_listing_taxa CHECK (fee_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_market_listing_popularidade CHECK (bid_count >= 0 AND view_count >= 0),
    CONSTRAINT ck_market_listing_categoria CHECK (
        item_kind_slug IS NULL OR char_length(item_kind_slug) BETWEEN 1 AND 64
    )
);
CREATE INDEX ix_market_listing_item_status_preco
    ON portal.market_listing (item_index, status, price);
CREATE INDEX ix_market_listing_kind_status_preco
    ON portal.market_listing (item_kind_slug, status, price);
CREATE INDEX ix_market_listing_status_popularidade
    ON portal.market_listing (status, bid_count DESC, view_count DESC);
CREATE INDEX ix_market_listing_status_prazo
    ON portal.market_listing (status, ends_at) WHERE status = 'ativo';
CREATE TRIGGER trg_market_listing_set_updated_at
    BEFORE UPDATE ON portal.market_listing
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.market_bid (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    listing_id        BIGINT NOT NULL,
    bidder_account_id BIGINT NOT NULL,
    amount            BIGINT NOT NULL,
    status            TEXT NOT NULL DEFAULT 'ativo',
    hold_ledger_id    BIGINT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_bid PRIMARY KEY (id),
    CONSTRAINT fk_market_bid_listing FOREIGN KEY (listing_id)
        REFERENCES portal.market_listing (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_market_bid_ledger FOREIGN KEY (hold_ledger_id)
        REFERENCES portal.market_ledger (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_market_bid_valor CHECK (amount > 0),
    CONSTRAINT ck_market_bid_estado CHECK (status IN (
        'ativo', 'superado', 'vencedor', 'perdido', 'cancelado'
    ))
);
CREATE UNIQUE INDEX ux_market_bid_comprador_ativo
    ON portal.market_bid (listing_id, bidder_account_id) WHERE status = 'ativo';
CREATE INDEX ix_market_bid_listing_criado ON portal.market_bid (listing_id, created_at DESC);
CREATE INDEX ix_market_bid_ledger ON portal.market_bid (hold_ledger_id)
    WHERE hold_ledger_id IS NOT NULL;

CREATE TABLE portal.market_transaction (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    listing_id        BIGINT NOT NULL,
    buyer_account_id  BIGINT NOT NULL,
    seller_account_id BIGINT NOT NULL,
    price_gross       BIGINT NOT NULL,
    fee_amount        BIGINT NOT NULL,
    net_amount        BIGINT NOT NULL,
    fee_bps           INTEGER NOT NULL,
    escrow_ref        BIGINT,
    settled_at        TIMESTAMPTZ NOT NULL,
    idempotency_key   TEXT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_transaction PRIMARY KEY (id),
    CONSTRAINT fk_market_transaction_listing FOREIGN KEY (listing_id)
        REFERENCES portal.market_listing (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT uq_market_transaction_listing UNIQUE (listing_id),
    CONSTRAINT uq_market_transaction_idempotency UNIQUE (idempotency_key),
    CONSTRAINT ck_market_transaction_partes CHECK (buyer_account_id <> seller_account_id),
    CONSTRAINT ck_market_transaction_soma CHECK (fee_amount + net_amount = price_gross),
    CONSTRAINT ck_market_transaction_valores CHECK (
        price_gross > 0 AND fee_amount >= 0 AND net_amount >= 0
    )
);
CREATE INDEX ix_market_transaction_comprador
    ON portal.market_transaction (buyer_account_id, settled_at DESC);
CREATE INDEX ix_market_transaction_vendedor
    ON portal.market_transaction (seller_account_id, settled_at DESC);

CREATE TABLE portal.market_review (
    id                  BIGINT GENERATED ALWAYS AS IDENTITY,
    transaction_id      BIGINT NOT NULL,
    reviewer_account_id BIGINT NOT NULL,
    seller_account_id   BIGINT NOT NULL,
    rating              SMALLINT NOT NULL,
    comment             TEXT,
    state               TEXT NOT NULL DEFAULT 'visivel',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_review PRIMARY KEY (id),
    CONSTRAINT fk_market_review_transaction FOREIGN KEY (transaction_id)
        REFERENCES portal.market_transaction (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT uq_market_review_transaction UNIQUE (transaction_id),
    CONSTRAINT ck_market_review_nota CHECK (rating BETWEEN 1 AND 5),
    CONSTRAINT ck_market_review_comentario CHECK (
        comment IS NULL OR char_length(comment) <= 500
    ),
    CONSTRAINT ck_market_review_estado CHECK (state IN ('visivel', 'oculto'))
);

CREATE TABLE portal.market_reputation (
    account_id      BIGINT NOT NULL,
    sales_completed INTEGER NOT NULL DEFAULT 0,
    sales_disputed  INTEGER NOT NULL DEFAULT 0,
    disputes_lost   INTEGER NOT NULL DEFAULT 0,
    rating_sum      BIGINT NOT NULL DEFAULT 0,
    rating_count    INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_reputation PRIMARY KEY (account_id),
    CONSTRAINT ck_market_reputation_contadores CHECK (
        sales_completed >= 0 AND sales_disputed >= 0 AND disputes_lost >= 0 AND rating_count >= 0
    ),
    CONSTRAINT ck_market_reputation_notas CHECK ((rating_count = 0) = (rating_sum = 0))
);
CREATE TRIGGER trg_market_reputation_set_updated_at
    BEFORE UPDATE ON portal.market_reputation
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.market_notification (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id BIGINT NOT NULL,
    kind       TEXT NOT NULL,
    ref_kind   TEXT NOT NULL,
    ref_id     BIGINT NOT NULL,
    read_at    TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_notification PRIMARY KEY (id),
    CONSTRAINT ck_market_notification_tipo CHECK (kind IN (
        'lance_recebido', 'lance_superado', 'arrematado', 'vendido', 'expirado',
        'disputa_aberta', 'disputa_decidida', 'avaliacao_recebida'
    )),
    CONSTRAINT ck_market_notification_ref CHECK (ref_kind IN ('anuncio', 'transacao', 'disputa'))
);
CREATE INDEX ix_market_notification_conta_nao_lida
    ON portal.market_notification (account_id) WHERE read_at IS NULL;

CREATE TABLE portal.market_dispute (
    id                    BIGINT GENERATED ALWAYS AS IDENTITY,
    transaction_id        BIGINT NOT NULL,
    opened_by_account_id  BIGINT NOT NULL,
    reason_code           TEXT NOT NULL,
    description           TEXT NOT NULL,
    state                 TEXT NOT NULL DEFAULT 'aberta',
    resolution            TEXT,
    decided_by_person_id  BIGINT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_dispute PRIMARY KEY (id),
    CONSTRAINT fk_market_dispute_transaction FOREIGN KEY (transaction_id)
        REFERENCES portal.market_transaction (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_market_dispute_motivo CHECK (reason_code IN (
        'nao_recebido', 'divergente', 'cobranca_indevida', 'outro'
    )),
    CONSTRAINT ck_market_dispute_estado CHECK (state IN (
        'aberta', 'em_analise', 'resolvida_comprador', 'resolvida_vendedor', 'cancelada'
    )),
    CONSTRAINT ck_market_dispute_desfecho CHECK (
        (state IN ('resolvida_comprador', 'resolvida_vendedor'))
            = (resolution IS NOT NULL AND decided_by_person_id IS NOT NULL)
    ),
    CONSTRAINT ck_market_dispute_descricao CHECK (char_length(description) BETWEEN 1 AND 2000)
);
CREATE UNIQUE INDEX ux_market_dispute_aberta
    ON portal.market_dispute (transaction_id) WHERE state IN ('aberta', 'em_analise');
CREATE INDEX ix_market_dispute_estado_aberta
    ON portal.market_dispute (state, created_at) WHERE state IN ('aberta', 'em_analise');
CREATE INDEX ix_market_dispute_transaction ON portal.market_dispute (transaction_id);
CREATE TRIGGER trg_market_dispute_set_updated_at
    BEFORE UPDATE ON portal.market_dispute
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.market_escrow (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    character_id    BIGINT NOT NULL,
    item_index      INTEGER NOT NULL,
    qty             INTEGER NOT NULL,
    refine          SMALLINT NOT NULL,
    ancient         BOOLEAN NOT NULL,
    gem_effect      SMALLINT NOT NULL,
    gem_value       SMALLINT NOT NULL,
    flags           INTEGER NOT NULL,
    state           TEXT NOT NULL DEFAULT 'retido',
    listing_ref     BIGINT,
    transaction_ref BIGINT,
    idempotency_key TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at       TIMESTAMPTZ,
    CONSTRAINT pk_market_escrow PRIMARY KEY (id),
    CONSTRAINT fk_market_escrow_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT uq_market_escrow_idempotency UNIQUE (idempotency_key),
    CONSTRAINT ck_market_escrow_estado CHECK (state IN ('retido', 'entregue', 'devolvido')),
    CONSTRAINT ck_market_escrow_qtd CHECK (qty >= 1),
    CONSTRAINT ck_market_escrow_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_market_escrow_fechamento CHECK ((state = 'retido') = (closed_at IS NULL))
);
CREATE INDEX ix_market_escrow_estado ON game.market_escrow (state, created_at);
CREATE INDEX ix_market_escrow_character ON game.market_escrow (character_id);

REVOKE UPDATE, DELETE ON portal.market_ledger FROM wyd_portal;
REVOKE INSERT, UPDATE, DELETE ON game.market_escrow FROM wyd_game;
GRANT SELECT, INSERT, UPDATE ON game.market_escrow TO wyd_admin;
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
-- Os `REVOKE` do `up` não precisam de espelho: o privilégio morre com a tabela.
DROP TABLE game.market_escrow, portal.market_dispute, portal.market_notification,
           portal.market_reputation, portal.market_review, portal.market_transaction,
           portal.market_bid, portal.market_listing, portal.market_fee_rule,
           portal.market_ledger, portal.market_wallet;

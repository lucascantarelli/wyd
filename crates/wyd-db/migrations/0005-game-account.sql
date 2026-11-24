-- migrate:up
CREATE TABLE game.account (
    id                  BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id           BIGINT NOT NULL,
    slot                SMALLINT NOT NULL,
    login               TEXT NOT NULL,
    login_fold          TEXT GENERATED ALWAYS AS (lower(btrim(login))) STORED,
    password_hash       TEXT NOT NULL,
    password_changed_at TIMESTAMPTZ,
    pin_hash            TEXT,
    pin_failure_count   SMALLINT NOT NULL DEFAULT 0,
    pin_locked_until    TIMESTAMPTZ,
    login_failure_count SMALLINT NOT NULL DEFAULT 0,
    login_locked_until  TIMESTAMPTZ,
    role                TEXT NOT NULL DEFAULT 'player',
    status              TEXT NOT NULL DEFAULT 'active',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at       TIMESTAMPTZ,
    closed_at           TIMESTAMPTZ,
    CONSTRAINT pk_account PRIMARY KEY (id),
    CONSTRAINT fk_account_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_slot_range CHECK (slot BETWEEN 1 AND 5),
    CONSTRAINT ck_account_login_len CHECK (char_length(login) BETWEEN 1 AND 16),
    CONSTRAINT ck_account_login_forma CHECK (login = btrim(login) AND char_length(login) > 0),
    CONSTRAINT ck_account_role CHECK (role IN ('player', 'moderator', 'admin')),
    CONSTRAINT ck_account_status CHECK (status IN ('active', 'closed')),
    CONSTRAINT ck_account_senha_hasheada CHECK (password_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_account_pin_hasheado CHECK (pin_hash IS NULL OR pin_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_account_pin_falhas CHECK (pin_failure_count >= 0),
    CONSTRAINT ck_account_login_falhas CHECK (login_failure_count >= 0),
    CONSTRAINT ck_account_senha_trocada CHECK (
        password_changed_at IS NULL OR password_changed_at >= created_at
    ),
    CONSTRAINT ck_account_fechamento_coerente CHECK ((status = 'closed') = (closed_at IS NOT NULL))
);
CREATE UNIQUE INDEX ux_account_login ON game.account (login_fold);
CREATE UNIQUE INDEX ux_account_slot ON game.account (person_id, slot) WHERE status <> 'closed';
CREATE INDEX ix_account_person ON game.account (person_id);
CREATE TRIGGER trg_account_set_updated_at
    BEFORE UPDATE ON game.account
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.account_wallet (
    account_id BIGINT NOT NULL,
    coin       BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_account_wallet PRIMARY KEY (account_id),
    CONSTRAINT fk_account_wallet_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_wallet_coin CHECK (coin >= 0 AND coin <= 2147483647)
);

CREATE TABLE game.account_ledger (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id    BIGINT NOT NULL,
    delta         BIGINT NOT NULL,
    balance_after BIGINT NOT NULL,
    reason        TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_account_ledger PRIMARY KEY (id),
    CONSTRAINT fk_account_ledger_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_ledger_delta CHECK (delta <> 0),
    CONSTRAINT ck_account_ledger_balance CHECK (
        balance_after >= 0 AND balance_after <= 2147483647
    ),
    CONSTRAINT ck_account_ledger_reason CHECK (char_length(reason) BETWEEN 1 AND 64)
);
CREATE INDEX ix_account_ledger_conta ON game.account_ledger (account_id, created_at DESC);

CREATE TABLE game.account_moderation (
    account_id BIGINT NOT NULL,
    blocked    BOOLEAN NOT NULL DEFAULT false,
    reason     TEXT NOT NULL DEFAULT '',
    blocked_by BIGINT,
    blocked_at TIMESTAMPTZ,
    CONSTRAINT pk_account_moderation PRIMARY KEY (account_id),
    CONSTRAINT fk_account_moderation_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT fk_account_moderation_autor FOREIGN KEY (blocked_by)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_moderation_motivo CHECK (
        NOT blocked OR char_length(btrim(reason)) > 0
    ),
    CONSTRAINT ck_account_moderation_instante CHECK (
        (blocked AND blocked_at IS NOT NULL) OR (NOT blocked AND blocked_at IS NULL)
    ),
    CONSTRAINT ck_account_moderation_autor CHECK (NOT blocked OR blocked_by IS NOT NULL)
);
CREATE INDEX ix_account_moderation_autor ON game.account_moderation (blocked_by);
CREATE INDEX ix_account_moderation_bloqueadas
    ON game.account_moderation (account_id) WHERE blocked;

CREATE TABLE game.pin_reset (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id   BIGINT NOT NULL,
    token_hash   TEXT NOT NULL,
    requested_ip INET,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at   TIMESTAMPTZ NOT NULL,
    used_at      TIMESTAMPTZ,
    CONSTRAINT pk_pin_reset PRIMARY KEY (id),
    CONSTRAINT fk_pin_reset_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT uq_pin_reset_token UNIQUE (token_hash),
    CONSTRAINT ck_pin_reset_token_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_pin_reset_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_pin_reset_consumo CHECK (used_at IS NULL OR used_at >= created_at)
);
CREATE INDEX ix_pin_reset_account ON game.pin_reset (account_id, created_at DESC);

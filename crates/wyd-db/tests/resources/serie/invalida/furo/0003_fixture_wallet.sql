-- migrate:up
CREATE TABLE fixture.wallet (
    id    BIGINT NOT NULL,
    saldo BIGINT NOT NULL,
    CONSTRAINT pk_wallet PRIMARY KEY (id),
    CONSTRAINT ck_wallet_saldo CHECK (saldo >= 0)
);

-- migrate:down
DROP TABLE fixture.wallet;

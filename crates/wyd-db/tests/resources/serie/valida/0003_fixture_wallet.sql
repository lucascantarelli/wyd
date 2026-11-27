-- migrate:up
CREATE TABLE fixture.wallet (
    person_id BIGINT NOT NULL,
    saldo     BIGINT NOT NULL,
    CONSTRAINT pk_wallet PRIMARY KEY (person_id),
    CONSTRAINT fk_wallet_person FOREIGN KEY (person_id)
        REFERENCES fixture.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_wallet_saldo CHECK (saldo >= 0)
);

-- migrate:down
DROP TABLE fixture.wallet;

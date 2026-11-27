-- Defeito único: o `INSERT` mora em arquivo de DDL, cujo nome não tem `_seed_`.
-- A ordem está certa (o seed vem depois do `CREATE TABLE`), então `CAP-04` passa.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   BIGINT NOT NULL,
    nome TEXT NOT NULL,
    CONSTRAINT pk_item PRIMARY KEY (id)
);

INSERT INTO fixture.item (id, nome) VALUES (1, 'fixture');

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

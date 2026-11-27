-- Defeito único: a `PRIMARY KEY` não tem `CONSTRAINT nome`.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   BIGINT PRIMARY KEY,
    nome TEXT NOT NULL
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

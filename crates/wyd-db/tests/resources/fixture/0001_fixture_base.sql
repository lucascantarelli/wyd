-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   INTEGER PRIMARY KEY,
    nome TEXT NOT NULL
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

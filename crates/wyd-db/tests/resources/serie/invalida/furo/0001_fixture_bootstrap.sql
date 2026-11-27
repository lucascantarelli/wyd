-- Defeito único: falta o 0002 entre este arquivo e o seguinte.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   BIGINT NOT NULL,
    nome TEXT NOT NULL,
    CONSTRAINT pk_item PRIMARY KEY (id)
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

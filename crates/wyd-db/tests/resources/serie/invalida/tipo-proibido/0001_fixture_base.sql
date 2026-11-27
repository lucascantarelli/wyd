-- Defeito único: `VARCHAR(50)` em coluna; o padrão pede `TEXT`.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   BIGINT NOT NULL,
    nome VARCHAR(50) NOT NULL,
    CONSTRAINT pk_item PRIMARY KEY (id)
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

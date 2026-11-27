-- Defeito único: o `INSERT` vem antes do `CREATE TABLE` que ele preenche.
-- O nome do arquivo tem `_seed_`, então `CAP-05` continua satisfeito.

-- migrate:up
CREATE SCHEMA fixture;

INSERT INTO fixture.item (id, nome) VALUES (1, 'fixture');

CREATE TABLE fixture.item (
    id   BIGINT NOT NULL,
    nome TEXT NOT NULL,
    CONSTRAINT pk_item PRIMARY KEY (id)
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

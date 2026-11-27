-- Fixture do contrato: série contígua mínima que satisfaz os sete invariantes.
-- Sem conexão: o alvo `contrato_migrations` lê este texto, não aplica nada.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   BIGINT NOT NULL,
    nome TEXT NOT NULL,
    CONSTRAINT pk_item PRIMARY KEY (id),
    CONSTRAINT ck_item_nome CHECK (nome <> '')
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

-- Defeito único: `fixture.extra` nasce no `up` e sobrevive ao `down`.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.item (
    id   BIGINT NOT NULL,
    nome TEXT NOT NULL,
    CONSTRAINT pk_item PRIMARY KEY (id)
);

CREATE TABLE fixture.extra (
    id BIGINT NOT NULL,
    CONSTRAINT pk_extra PRIMARY KEY (id)
);

-- migrate:down
DROP TABLE fixture.item;

DROP SCHEMA fixture;

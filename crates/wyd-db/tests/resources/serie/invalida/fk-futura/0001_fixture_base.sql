-- Defeito único: a FK de `fixture.a` aponta para `fixture.b`, que nasce no 0002.

-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.a (
    id   BIGINT NOT NULL,
    b_id BIGINT NOT NULL,
    CONSTRAINT pk_a PRIMARY KEY (id),
    CONSTRAINT fk_a_b FOREIGN KEY (b_id)
        REFERENCES fixture.b (id) ON DELETE RESTRICT
);

-- migrate:down
DROP TABLE fixture.a;

DROP SCHEMA fixture;

-- migrate:up
CREATE TABLE fixture.b (
    id BIGINT NOT NULL,
    CONSTRAINT pk_b PRIMARY KEY (id)
);

-- migrate:down
DROP TABLE fixture.b;

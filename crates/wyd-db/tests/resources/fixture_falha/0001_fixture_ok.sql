-- migrate:up
CREATE SCHEMA fixture;

CREATE TABLE fixture.base (
    id INTEGER PRIMARY KEY
);

-- migrate:down
DROP TABLE fixture.base;

DROP SCHEMA fixture;

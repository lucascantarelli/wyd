-- migrate:up
CREATE TABLE fixture.meio (
    id INTEGER PRIMARY KEY
);

SELECT 1 / 0;

-- migrate:down
DROP TABLE fixture.meio;

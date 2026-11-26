-- migrate:up
CREATE VIEW fixture.item_contagem AS
    SELECT count(*) AS total FROM fixture.item;

-- migrate:down
DROP VIEW fixture.item_contagem;

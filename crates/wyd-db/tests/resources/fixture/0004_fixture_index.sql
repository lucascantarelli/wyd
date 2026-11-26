-- migrate:up
CREATE INDEX ix_fixture_item_nome ON fixture.item (nome);

-- migrate:down
DROP INDEX fixture.ix_fixture_item_nome;

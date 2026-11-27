-- migrate:up
INSERT INTO fixture.item (id, nome) VALUES (1, 'fixture');

-- migrate:down
DELETE FROM fixture.item WHERE id = 1;

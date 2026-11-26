-- migrate:up
INSERT INTO fixture.item (id, nome) VALUES (1, 'um'), (2, 'dois');

-- migrate:down
DELETE FROM fixture.item;

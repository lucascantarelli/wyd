-- migrate:up
-- Papel de staff do `wyd-admin` (`M27`): criado fora da série por `db/roles.sql`.
GRANT USAGE ON SCHEMA game TO wyd_admin;
GRANT USAGE ON SCHEMA ops TO wyd_admin;

GRANT SELECT, INSERT, UPDATE ON game.account_moderation TO wyd_admin;
GRANT SELECT, UPDATE ON game.account_wallet TO wyd_admin;
GRANT SELECT, INSERT ON game.account_ledger TO wyd_admin;
GRANT SELECT, UPDATE ON game.server_channel TO wyd_admin;
GRANT SELECT, UPDATE ON game.session TO wyd_admin;
GRANT SELECT, UPDATE ON game.city TO wyd_admin;
GRANT SELECT, UPDATE ON game.item_price TO wyd_admin;
-- Custódia de item do mercado (`M31`, C4 §4.9–§4.13): o item sai da mochila e volta para ela na mesma
-- transação. `DELETE` existe só aqui, e só nesta tabela.
GRANT SELECT, INSERT, DELETE ON game.character_item TO wyd_admin;
GRANT SELECT ON game.character TO wyd_admin;

GRANT INSERT ON ops.audit_log TO wyd_admin;
GRANT SELECT (id) ON ops.audit_log TO wyd_admin;

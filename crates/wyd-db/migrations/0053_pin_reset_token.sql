-- 0053_pin_reset_token: token de uso único do reset da segunda senha (autoatendimento).
--
-- O reset de PIN pelo staff (`account_admin/reset_numeric_pin`) exige alguém do time; o
-- autoatendimento precisa de um canal de prova que o jogador controla: o e-mail. A coluna
-- `accounts.email` (formato + unicidade parcial) mora na criação da tabela, em `0001_contas`;
-- aqui fica só o token.
--
-- Só o **hash** do token é gravado (`sha256` hex calculado no BFF), com validade curta e uso
-- único — o link do e-mail nunca fica recuperável no banco.

-- migrate:up
CREATE TABLE pin_reset_token (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    account_id   BIGINT NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    token_hash   TEXT NOT NULL,
    -- Destino usado no envio (auditoria de qual endereço recebeu o link).
    delivered_to TEXT NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at   TIMESTAMPTZ NOT NULL,
    used_at      TIMESTAMPTZ,
    CONSTRAINT pin_reset_token_hash_unique UNIQUE (token_hash),
    CONSTRAINT pin_reset_token_hash_formato CHECK (char_length(token_hash) = 64),
    CONSTRAINT pin_reset_token_validade CHECK (expires_at > created_at)
);

CREATE INDEX pin_reset_token_account_idx ON pin_reset_token (account_id, created_at DESC);

-- migrate:down
DROP INDEX pin_reset_token_account_idx;
DROP TABLE pin_reset_token;

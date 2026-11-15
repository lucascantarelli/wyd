-- 0001_contas: contas de acesso (login, credencial, papel, e-mail de contato).
-- Convenção: -- migrate:up / -- migrate:down (ver docs/arquitetura/modelo-banco.md §4).
--
-- `email` nasce AQUI, na criação da tabela: é o canal de prova do autoatendimento da segunda
-- senha (reset por link) e a coluna já existia por `ALTER TABLE` na 0053. Coluna nova nasce na
-- criação — `tests/contrato/catalogo.rs` reprova `ALTER TABLE` no `up` (modelo-banco.md §4).

-- migrate:up
CREATE TABLE accounts (
    id          BIGSERIAL PRIMARY KEY,
    login       TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    pin_hash    TEXT,
    -- Estado do bloqueio do PIN (persistido: reiniciar o processo não zera tentativas).
    pin_failures SMALLINT NOT NULL DEFAULT 0,
    pin_locked_until TIMESTAMPTZ,
    role        TEXT NOT NULL DEFAULT 'player',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at TIMESTAMPTZ,
    -- E-mail de contato (opcional: conta sem e-mail continua entrando; só o reset fica sem
    -- canal). Nulável de propósito e único só quando preenchido — duas contas sem e-mail não
    -- colidem, e uma conta por endereço é o que dá sentido ao link de reset.
    email       TEXT,
    CONSTRAINT accounts_login_nonempty CHECK (char_length(login) > 0),
    CONSTRAINT accounts_role CHECK (role IN ('player', 'moderator', 'admin')),
    CONSTRAINT accounts_pin_failures CHECK (pin_failures >= 0),
    CONSTRAINT accounts_email_formato CHECK (
        email IS NULL
        OR (
            char_length(email) <= 254
            AND email ~ '^[^@[:space:]]+@[^@[:space:]]+\.[^@[:space:]]+$'
        )
    )
);
CREATE UNIQUE INDEX accounts_login_unique ON accounts (login);
CREATE UNIQUE INDEX accounts_email_unique ON accounts (email) WHERE email IS NOT NULL;

-- migrate:down
DROP INDEX accounts_email_unique;
DROP INDEX accounts_login_unique;
DROP TABLE accounts;

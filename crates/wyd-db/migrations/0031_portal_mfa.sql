-- migrate:up
CREATE TABLE portal.person_authenticator (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id        BIGINT NOT NULL,
    -- Mecanismo do segundo fator: `totp` é o único da 1.0.0 (A20); outro `kind` exige decisão nova.
    kind             TEXT NOT NULL DEFAULT 'totp',
    -- Segredo TOTP CIFRADO: o BFF precisa decifrar para validar o código, então hash não serve.
    -- A chave fica no ambiente do BFF, nunca no banco. NULL só existe em linha revogada (ver CHECK).
    secret_encrypted BYTEA,
    -- Inscrição confirmada: NULL = cadastro pendente, que ainda não vale no login.
    confirmed_at     TIMESTAMPTZ,
    -- Último passo TOTP (30 s) aceito; repetir o passo é replay e o BFF recusa (A20).
    last_used_step   BIGINT,
    -- Revogação por perda de dispositivo ou desligamento do fator; revogar apaga o segredo.
    revoked_at       TIMESTAMPTZ,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_person_authenticator PRIMARY KEY (id),
    CONSTRAINT fk_person_authenticator_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_person_authenticator_kind CHECK (kind IN ('totp')),
    -- Segredo e revogação andam juntos: ativo tem segredo; revogado não guarda segredo morto.
    CONSTRAINT ck_person_authenticator_segredo CHECK (
        (revoked_at IS NULL AND secret_encrypted IS NOT NULL)
        OR (revoked_at IS NOT NULL AND secret_encrypted IS NULL)
    ),
    -- Cifra de um segredo TOTP (nonce + segredo + tag) cabe com folga nesta faixa.
    CONSTRAINT ck_person_authenticator_segredo_tamanho CHECK (
        secret_encrypted IS NULL OR octet_length(secret_encrypted) BETWEEN 32 AND 512
    ),
    CONSTRAINT ck_person_authenticator_confirmacao CHECK (
        confirmed_at IS NULL OR confirmed_at >= created_at
    ),
    -- Só se registra passo usado depois de a inscrição valer.
    CONSTRAINT ck_person_authenticator_passo CHECK (
        last_used_step IS NULL OR (last_used_step >= 0 AND confirmed_at IS NOT NULL)
    ),
    CONSTRAINT ck_person_authenticator_revogacao CHECK (
        revoked_at IS NULL OR revoked_at >= created_at
    ),
    CONSTRAINT ck_person_authenticator_revogacao_confirmada CHECK (
        revoked_at IS NULL OR confirmed_at IS NULL OR revoked_at >= confirmed_at
    )
);

-- Um autenticador ATIVO por pessoa e tipo; revogado não bloqueia o próximo cadastro.
CREATE UNIQUE INDEX ux_person_authenticator_ativo
    ON portal.person_authenticator (person_id, kind)
    WHERE revoked_at IS NULL;

-- Lado filho da FK (P-15): fechar/apagar pessoa precisa achar as linhas sem varrer a tabela.
CREATE INDEX ix_person_authenticator_person
    ON portal.person_authenticator (person_id);

CREATE TRIGGER trg_person_authenticator_set_updated_at
    BEFORE UPDATE ON portal.person_authenticator
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE portal.person_authenticator;

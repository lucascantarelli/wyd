-- 0004_portal_access: sessão revogável do portal e tokens de cerimônia da pessoa.
-- Depende de `portal.person` (0003). O jogo não lê nenhuma das duas tabelas.

-- migrate:up
CREATE TABLE portal.person_session (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id    BIGINT NOT NULL,
    -- Verificador do cookie: o valor é opaco e só o sha256 dele mora aqui (`A16`, 2026-11-18).
    token_hash   TEXT NOT NULL,
    ip           INET,
    user_agent   TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Último sinal de vida. O timeout IDLE é `last_seen_at + intervalo`, por política; não é coluna.
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Expiração ABSOLUTA: independe de atividade e não se estende.
    expires_at   TIMESTAMPTZ NOT NULL,
    revoked_at   TIMESTAMPTZ,
    CONSTRAINT pk_person_session PRIMARY KEY (id),
    CONSTRAINT fk_person_session_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT uq_person_session_token UNIQUE (token_hash),
    CONSTRAINT ck_person_session_token_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_person_session_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_person_session_revogacao CHECK (revoked_at IS NULL OR revoked_at >= created_at),
    -- Sinal de vida fora da janela da própria sessão é estado impossível.
    CONSTRAINT ck_person_session_sinal CHECK (
        last_seen_at >= created_at AND last_seen_at <= expires_at
    ),
    CONSTRAINT ck_person_session_user_agent CHECK (char_length(user_agent) <= 512)
);

CREATE INDEX ix_person_session_person ON portal.person_session (person_id);
CREATE INDEX ix_person_session_vivas ON portal.person_session (person_id, last_seen_at DESC)
    WHERE revoked_at IS NULL;
-- Reaper de `M22`: varre expiradas sem varrer a tabela.
CREATE INDEX ix_person_session_expira ON portal.person_session (expires_at);

CREATE TABLE portal.person_token (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id     BIGINT NOT NULL,
    -- Cerimônia a que o token dá direito. Três valores, três fluxos, um único formato de token.
    purpose       TEXT NOT NULL,
    -- Só o hash sha256 hex de 64; o valor em claro nunca chega ao banco.
    token_hash    TEXT NOT NULL,
    -- Origem do pedido: auditoria de abuso. O destinatário vem de `person.email`, nunca daqui.
    requested_ip  INET,
    attempt_count SMALLINT NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at    TIMESTAMPTZ NOT NULL,
    used_at       TIMESTAMPTZ,
    CONSTRAINT pk_person_token PRIMARY KEY (id),
    CONSTRAINT fk_person_token_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT uq_person_token_hash UNIQUE (token_hash),
    CONSTRAINT ck_person_token_purpose CHECK (
        purpose IN ('email_verify', 'password_reset', 'email_change')
    ),
    CONSTRAINT ck_person_token_hash_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_person_token_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_person_token_consumo CHECK (used_at IS NULL OR used_at >= created_at),
    CONSTRAINT ck_person_token_tentativas CHECK (attempt_count >= 0)
);

-- Um token vivo por pessoa e propósito: reemitir invalida o anterior (OWASP Forgot Password).
CREATE UNIQUE INDEX ux_person_token_vivo ON portal.person_token (person_id, purpose)
    WHERE used_at IS NULL;
CREATE INDEX ix_person_token_pessoa ON portal.person_token (person_id, created_at DESC);
CREATE INDEX ix_person_token_expira ON portal.person_token (expires_at) WHERE used_at IS NULL;
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE portal.person_token, portal.person_session;

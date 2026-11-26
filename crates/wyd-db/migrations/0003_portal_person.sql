-- migrate:up
CREATE TABLE portal.person (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    email             TEXT NOT NULL,
    email_fold        TEXT GENERATED ALWAYS AS (lower(btrim(email))) STORED,
    password_hash     TEXT NOT NULL,
    role              TEXT NOT NULL DEFAULT 'member',
    status            TEXT NOT NULL DEFAULT 'active',
    email_verified_at TIMESTAMPTZ,
    failed_login_count SMALLINT NOT NULL DEFAULT 0,
    locked_until      TIMESTAMPTZ,
    password_changed_at TIMESTAMPTZ,
    last_login_at     TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at         TIMESTAMPTZ,
    suspended_reason  TEXT,
    suspended_by      BIGINT,
    suspended_at      TIMESTAMPTZ,
    CONSTRAINT pk_person PRIMARY KEY (id),
    CONSTRAINT fk_person_suspended_by FOREIGN KEY (suspended_by)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_person_email_len CHECK (char_length(email) <= 254),
    CONSTRAINT ck_person_email_forma CHECK (
        email ~ '^[^@[:space:]]+@[^@[:space:]]+[.][^@[:space:]]{2,}$'
    ),
    CONSTRAINT ck_person_role CHECK (role IN ('member', 'moderator', 'admin')),
    CONSTRAINT ck_person_status CHECK (status IN ('pending', 'active', 'suspended', 'closed')),
    CONSTRAINT ck_person_senha_hasheada CHECK (password_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_person_falhas CHECK (failed_login_count >= 0),
    CONSTRAINT ck_person_senha_trocada CHECK (
        password_changed_at IS NULL OR password_changed_at >= created_at
    ),
    CONSTRAINT ck_person_fechamento_coerente CHECK ((status = 'closed') = (closed_at IS NOT NULL)),
    CONSTRAINT ck_person_fechamento_anonimo CHECK (
        status <> 'closed' OR email = 'closed-' || id || '@removido.invalid'
    ),
    CONSTRAINT ck_person_suspensao_coerente CHECK (
        (status = 'suspended') = (suspended_at IS NOT NULL)
        AND (status = 'suspended' OR (suspended_reason IS NULL AND suspended_by IS NULL))
    ),
    CONSTRAINT ck_person_suspensao_motivo CHECK (
        status <> 'suspended'
        OR (char_length(btrim(suspended_reason)) > 0 AND suspended_by IS NOT NULL)
    )
);
CREATE UNIQUE INDEX ux_person_email ON portal.person (email_fold);
CREATE INDEX ix_person_nao_ativas ON portal.person (status) WHERE status <> 'active';
CREATE INDEX ix_person_suspended_by ON portal.person (suspended_by);
CREATE TRIGGER trg_person_set_updated_at
    BEFORE UPDATE ON portal.person
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.person_profile (
    person_id    BIGINT NOT NULL,
    display_name TEXT,
    birth_date   DATE,
    country      TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_person_profile PRIMARY KEY (person_id),
    CONSTRAINT fk_person_profile_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT ck_person_profile_display_name CHECK (
        display_name IS NULL OR char_length(btrim(display_name)) BETWEEN 2 AND 32
    ),
    CONSTRAINT ck_person_profile_nascimento CHECK (
        birth_date IS NULL OR birth_date >= DATE '1900-01-01'
    ),
    CONSTRAINT ck_person_profile_pais CHECK (country IS NULL OR country ~ '^[A-Z]{2}$')
);
CREATE TRIGGER trg_person_profile_set_updated_at
    BEFORE UPDATE ON portal.person_profile
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.person_consent (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id  BIGINT NOT NULL,
    kind       TEXT NOT NULL,
    version    TEXT NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at TIMESTAMPTZ,
    CONSTRAINT pk_person_consent PRIMARY KEY (id),
    CONSTRAINT fk_person_consent_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE CASCADE,
    CONSTRAINT ck_person_consent_kind CHECK (kind IN ('terms', 'privacy', 'marketing')),
    CONSTRAINT ck_person_consent_version CHECK (char_length(btrim(version)) BETWEEN 1 AND 32),
    CONSTRAINT ck_person_consent_revogacao CHECK (revoked_at IS NULL OR revoked_at >= granted_at)
);
CREATE UNIQUE INDEX ux_person_consent_vivo ON portal.person_consent (person_id, kind)
    WHERE revoked_at IS NULL;
CREATE INDEX ix_person_consent_pessoa ON portal.person_consent (person_id, granted_at DESC);
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
-- Perde pessoa, perfil e consentimento: dado pessoal não é reconstruível pelo `up` (`M32`).
DROP TABLE portal.person_consent, portal.person_profile, portal.person;

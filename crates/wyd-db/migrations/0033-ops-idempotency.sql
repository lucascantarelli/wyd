-- 0033_ops_idempotency: registro durável de idempotência do contrato do wyd-admin.
-- Não tem FK para `ops.audit_log`: como o ator da auditoria, o registro sobrevive ao alvo e não
-- bloqueia escrita. Depende de `0001` (schema `ops`) apenas.

-- migrate:up
CREATE TABLE ops.idempotency_record (
    caller        TEXT NOT NULL,
    method        TEXT NOT NULL,
    route         TEXT NOT NULL,
    key           TEXT NOT NULL,
    body_sha256   TEXT NOT NULL,
    state         TEXT NOT NULL,
    status_code   SMALLINT,
    response_body JSONB,
    audit_id      BIGINT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at     TIMESTAMPTZ,
    CONSTRAINT pk_idempotency_record PRIMARY KEY (caller, method, route, key),
    CONSTRAINT ck_idempotency_record_metodo CHECK (method IN ('POST', 'PUT')),
    CONSTRAINT ck_idempotency_record_chave CHECK (key ~ '^[A-Za-z0-9_-]{16,64}$'),
    CONSTRAINT ck_idempotency_record_corpo CHECK (body_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_idempotency_record_rota CHECK (char_length(route) BETWEEN 1 AND 128),
    CONSTRAINT ck_idempotency_record_estado CHECK (state IN ('in_progress', 'completed', 'failed')),
    CONSTRAINT ck_idempotency_record_resultado CHECK (
        (state = 'completed') = (status_code IS NOT NULL AND response_body IS NOT NULL)
    ),
    CONSTRAINT ck_idempotency_record_fechamento CHECK ((state = 'in_progress') = (closed_at IS NULL))
);

CREATE INDEX ix_idempotency_record_criado ON ops.idempotency_record (created_at);

GRANT SELECT, INSERT, UPDATE ON ops.idempotency_record TO wyd_admin;

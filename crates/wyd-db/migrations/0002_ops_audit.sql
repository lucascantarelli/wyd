-- 0002_ops_audit: auditoria, tentativa de login e histórico de configuração.
-- Depende de `0001` (schemas, papéis, default privileges). Nenhuma FK para arquivo posterior.

-- migrate:up
CREATE TABLE ops.audit_log (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Ator tripartido (M18-A). Sem FK, por desenho: auditoria sobrevive ao ator.
    actor_kind  TEXT NOT NULL,
    actor_id    BIGINT,
    actor_ref   TEXT,
    action      TEXT NOT NULL,
    -- Herdados do legado (TEXT). Virar JSONB é decisão que ninguém tomou; não entra por gosto.
    target      TEXT NOT NULL DEFAULT '',
    detail      TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_audit_log PRIMARY KEY (id),
    CONSTRAINT ck_audit_log_actor_kind CHECK (actor_kind IN ('person', 'account', 'system')),
    CONSTRAINT ck_audit_log_actor_sistema CHECK ((actor_kind = 'system') = (actor_id IS NULL)),
    CONSTRAINT ck_audit_log_actor_ref CHECK ((actor_kind <> 'system') = (actor_ref IS NULL)),
    CONSTRAINT ck_audit_log_action CHECK (char_length(action) > 0)
);

CREATE INDEX ix_audit_log_occurred ON ops.audit_log (occurred_at DESC);
-- "o que o staff 42 fez?" é igualdade + faixa, nessa ordem: kind, id, depois o tempo.
CREATE INDEX ix_audit_log_actor ON ops.audit_log (actor_kind, actor_id, occurred_at DESC);

GRANT INSERT ON ops.audit_log TO wyd_game, wyd_portal;

CREATE TABLE ops.login_attempt (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    occurred_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- De onde veio a tentativa: login do portal (e-mail) ou handshake do cliente 7662 (login).
    surface         TEXT NOT NULL,
    -- sha256 hex do `email_fold` (portal) ou do `login_fold` (jogo); nunca o valor em claro.
    identifier_hash TEXT NOT NULL,
    ip              INET,
    result          TEXT NOT NULL,
    CONSTRAINT pk_login_attempt PRIMARY KEY (id),
    CONSTRAINT ck_login_attempt_surface CHECK (surface IN ('portal', 'game')),
    CONSTRAINT ck_login_attempt_identifier CHECK (identifier_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_login_attempt_result CHECK (
        result IN ('ok', 'bad_secret', 'bad_second_factor', 'blocked', 'locked', 'unknown_identifier')
    )
);

CREATE INDEX ix_login_attempt_identifier ON ops.login_attempt (identifier_hash, occurred_at DESC);
CREATE INDEX ix_login_attempt_ip ON ops.login_attempt (ip, occurred_at DESC) WHERE ip IS NOT NULL;
CREATE INDEX ix_login_attempt_occurred ON ops.login_attempt (occurred_at DESC);

GRANT INSERT ON ops.login_attempt TO wyd_game, wyd_portal;

CREATE TABLE ops.config_revision (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    -- Quem editou. Mesmo ator tripartido de `audit_log` (M18-A), sem FK: `portal.person` nasce em
    -- `0003`, arquivo posterior — FK para frente é proibida (`padrao-banco.md` §9, item 8).
    author_kind      TEXT NOT NULL,
    author_id        BIGINT,
    author_ref       TEXT,
    -- Por que a configuração mudou. Texto curto de operação, nunca dado de jogo.
    note             TEXT NOT NULL DEFAULT '',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Instante em que a revisão passou a valer. Uma revisão nasce aplicada: não há rascunho na 1.0.0.
    applied_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_config_revision PRIMARY KEY (id),
    CONSTRAINT ck_config_revision_autor CHECK (
        (author_kind = 'person' AND author_id IS NOT NULL AND author_ref IS NULL)
        OR (author_kind = 'system' AND author_id IS NULL AND author_ref IS NOT NULL)
    ),
    CONSTRAINT ck_config_revision_nota CHECK (char_length(note) <= 500),
    CONSTRAINT ck_config_revision_aplicacao CHECK (applied_at >= created_at)
);

CREATE INDEX ix_config_revision_aplicada ON ops.config_revision (applied_at DESC);
CREATE INDEX ix_config_revision_autor ON ops.config_revision (author_kind, author_id, applied_at DESC);

CREATE TABLE ops.config_change (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    revision_id   BIGINT NOT NULL,
    -- Onde a mudança incide: `schema.tabela` + a chave da linha.
    target_schema TEXT NOT NULL,
    target_table  TEXT NOT NULL,
    target_key    TEXT NOT NULL,
    -- Valor antes e depois, em texto canônico do tipo da coluna alvo. Sem JSONB para estado de jogo.
    -- `old_value` nulo = a chave não existia antes; `new_value` nulo = a chave deixou de existir.
    old_value     TEXT,
    new_value     TEXT,
    CONSTRAINT pk_config_change PRIMARY KEY (id),
    CONSTRAINT fk_config_change_revision FOREIGN KEY (revision_id)
        REFERENCES ops.config_revision (id) ON DELETE RESTRICT,
    -- Uma chave muda uma vez por revisão; mudar de novo exige outra revisão.
    CONSTRAINT uq_config_change_alvo UNIQUE (revision_id, target_schema, target_table, target_key),
    CONSTRAINT ck_config_change_schema CHECK (target_schema IN ('content', 'game', 'portal', 'ops')),
    CONSTRAINT ck_config_change_tabela CHECK (target_table ~ '^[a-z][a-z0-9_]*$'),
    CONSTRAINT ck_config_change_chave CHECK (char_length(target_key) BETWEEN 1 AND 200),
    CONSTRAINT ck_config_change_muda CHECK (old_value IS DISTINCT FROM new_value),
    CONSTRAINT ck_config_change_tem_valor CHECK (old_value IS NOT NULL OR new_value IS NOT NULL),
    CONSTRAINT ck_config_change_tamanho CHECK (
        (old_value IS NULL OR char_length(old_value) <= 4000)
        AND (new_value IS NULL OR char_length(new_value) <= 4000)
    )
);

CREATE INDEX ix_config_change_revision ON ops.config_change (revision_id);
-- Histórico de uma chave, do mais novo para o mais antigo.
CREATE INDEX ix_config_change_alvo
    ON ops.config_change (target_schema, target_table, target_key, revision_id DESC);

-- O portal é o único runtime que escreve configuração (área administrativa). Sem UPDATE/DELETE.
GRANT SELECT, INSERT ON ops.config_revision TO wyd_portal;
GRANT SELECT, INSERT ON ops.config_change TO wyd_portal;
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
-- Perde a trilha de auditoria e o histórico de configuração; o `down` não arquiva cópia antes de derrubar.
DROP TABLE ops.config_change, ops.config_revision, ops.login_attempt, ops.audit_log;

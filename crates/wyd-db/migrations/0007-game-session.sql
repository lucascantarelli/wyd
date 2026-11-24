-- migrate:up
CREATE TABLE game.server_channel (
    group_id    SMALLINT NOT NULL,
    channel     SMALLINT NOT NULL,
    host        TEXT NOT NULL,
    status_url  TEXT NOT NULL DEFAULT '',
    enabled     BOOLEAN NOT NULL DEFAULT true,
    max_players INTEGER,
    maintenance BOOLEAN,
    role        TEXT,
    note        TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_server_channel PRIMARY KEY (group_id, channel),
    CONSTRAINT ck_server_channel_grupo CHECK (group_id BETWEEN 0 AND 9),
    CONSTRAINT ck_server_channel_canal CHECK (channel BETWEEN 0 AND 10),
    CONSTRAINT ck_server_channel_host CHECK (host <> ''),
    CONSTRAINT ck_server_channel_status_url CHECK (char_length(status_url) <= 32),
    CONSTRAINT ck_server_channel_jogavel CHECK (
        (channel = 0 AND max_players IS NULL AND maintenance IS NULL AND role IS NULL)
     OR (channel > 0 AND max_players IS NOT NULL AND maintenance IS NOT NULL AND role IS NOT NULL)
    ),
    CONSTRAINT ck_server_channel_max CHECK (max_players >= 0),
    CONSTRAINT ck_server_channel_role CHECK (role IN ('normal', 'verde'))
);
CREATE TRIGGER trg_server_channel_set_updated_at
    BEFORE UPDATE ON game.server_channel
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.session (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id     BIGINT NOT NULL,
    character_id   BIGINT,
    group_id       SMALLINT NOT NULL,
    channel        SMALLINT NOT NULL,
    client_version INTEGER NOT NULL,
    ip             INET,
    opened_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    renewed_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    released_at    TIMESTAMPTZ,
    released_reason TEXT,
    CONSTRAINT pk_session PRIMARY KEY (id),
    CONSTRAINT fk_session_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT fk_session_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT,
    CONSTRAINT fk_session_canal FOREIGN KEY (group_id, channel)
        REFERENCES game.server_channel (group_id, channel) ON DELETE RESTRICT,
    CONSTRAINT ck_session_canal CHECK (channel BETWEEN 1 AND 10),
    CONSTRAINT ck_session_client_version CHECK (client_version > 0),
    CONSTRAINT ck_session_sinal CHECK (renewed_at >= opened_at),
    CONSTRAINT ck_session_liberacao CHECK (released_at >= opened_at),
    CONSTRAINT ck_session_motivo CHECK (
        (released_at IS NULL AND released_reason IS NULL)
     OR (released_at IS NOT NULL AND released_reason IN ('disconnect', 'reaper', 'character_deleted', 'admin'))
    )
);
CREATE UNIQUE INDEX ux_session_viva_por_conta ON game.session (account_id)
    WHERE released_at IS NULL;
CREATE UNIQUE INDEX ux_session_viva_por_personagem ON game.session (character_id)
    WHERE released_at IS NULL AND character_id IS NOT NULL;
CREATE INDEX ix_session_canal ON game.session (group_id, channel, opened_at DESC);
CREATE INDEX ix_session_reaper ON game.session (renewed_at) WHERE released_at IS NULL;
CREATE INDEX ix_session_ip ON game.session (ip, opened_at DESC) WHERE ip IS NOT NULL;

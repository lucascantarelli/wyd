-- 0028_game_quests_events: progresso de quest, histórico de evento, estado de Kefra/castelo e arena.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Estado de jogador: escreve só o servidor; o portal lê pelas views de 0029. Nenhum GRANT novo.

-- migrate:up
-- Progresso de quest: PK composta (personagem, quest) e filho possuído do personagem.
CREATE TABLE game.character_quest (
    character_id BIGINT NOT NULL,
    quest_code   TEXT NOT NULL,
    step         SMALLINT NOT NULL DEFAULT 1,
    progress     INTEGER NOT NULL DEFAULT 0,
    started_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    CONSTRAINT pk_character_quest PRIMARY KEY (character_id, quest_code),
    CONSTRAINT fk_character_quest_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    -- Remover a quest do conteúdo não apaga progresso de jogador: a FK recusa alto.
    CONSTRAINT fk_character_quest_quest FOREIGN KEY (quest_code)
        REFERENCES content.quest (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_character_quest_step CHECK (step > 0),
    CONSTRAINT ck_character_quest_progress CHECK (progress >= 0),
    CONSTRAINT ck_character_quest_conclusao CHECK (
        completed_at IS NULL OR completed_at >= started_at)
);

CREATE INDEX ix_character_quest_quest ON game.character_quest (quest_code);

-- Histórico de participação: linha some com evento? Não — recusa alto; personagem e clã viram nulo.
CREATE TABLE game.event_participation (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    event_code   TEXT NOT NULL,
    character_id BIGINT,
    guild_id     BIGINT,
    result       TEXT NOT NULL,
    score        BIGINT NOT NULL DEFAULT 0,
    at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_event_participation PRIMARY KEY (id),
    CONSTRAINT fk_event_participation_event FOREIGN KEY (event_code)
        REFERENCES content.world_event (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_event_participation_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_event_participation_guild FOREIGN KEY (guild_id)
        REFERENCES game.guild (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT ck_event_participation_result CHECK (
        result IN ('win', 'lose', 'draw', 'participated')),
    CONSTRAINT ck_event_participation_score CHECK (score >= 0)
);

CREATE INDEX ix_event_participation_event
    ON game.event_participation (event_code, at DESC);
CREATE INDEX ix_event_participation_character
    ON game.event_participation (character_id) WHERE character_id IS NOT NULL;
CREATE INDEX ix_event_participation_guild
    ON game.event_participation (guild_id) WHERE guild_id IS NOT NULL;

-- Linha única de estado do Kefra: `id = 1` é a chave, sem identidade (como content_version).
CREATE TABLE game.kefra_state (
    id            SMALLINT NOT NULL,
    last_kill_at  TIMESTAMPTZ,
    next_spawn_at TIMESTAMPTZ,
    kill_count    INTEGER NOT NULL DEFAULT 0,
    last_guild_id BIGINT,
    CONSTRAINT pk_kefra_state PRIMARY KEY (id),
    CONSTRAINT fk_kefra_state_guild FOREIGN KEY (last_guild_id)
        REFERENCES game.guild (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT ck_kefra_state_linha_unica CHECK (id = 1),
    CONSTRAINT ck_kefra_state_kill_count CHECK (kill_count >= 0)
);

CREATE INDEX ix_kefra_state_guild ON game.kefra_state (last_guild_id)
    WHERE last_guild_id IS NOT NULL;

-- Dono do castelo, cidade e cofre; a cidade é a de 0011 e não some com o castelo.
CREATE TABLE game.castle_state (
    id                SMALLINT NOT NULL,
    owner_guild_id    BIGINT,
    city_code         TEXT,
    vault_gold        BIGINT NOT NULL DEFAULT 0,
    last_collected_at TIMESTAMPTZ,
    last_war_at       TIMESTAMPTZ,
    CONSTRAINT pk_castle_state PRIMARY KEY (id),
    CONSTRAINT fk_castle_state_guild FOREIGN KEY (owner_guild_id)
        REFERENCES game.guild (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_castle_state_cidade FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_castle_state_linha_unica CHECK (id = 1),
    CONSTRAINT ck_castle_state_cofre CHECK (vault_gold >= 0)
);

CREATE INDEX ix_castle_state_guild ON game.castle_state (owner_guild_id)
    WHERE owner_guild_id IS NOT NULL;
CREATE INDEX ix_castle_state_cidade ON game.castle_state (city_code)
    WHERE city_code IS NOT NULL;

-- Partida de arena: histórico, não estado vivo; `arena_code` é a definição de 0026.
CREATE TABLE game.arena_match (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    arena_code  TEXT NOT NULL,
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at    TIMESTAMPTZ,
    winner_team SMALLINT,
    CONSTRAINT pk_arena_match PRIMARY KEY (id),
    CONSTRAINT fk_arena_match_arena FOREIGN KEY (arena_code)
        REFERENCES content.arena (code) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_arena_match_fim CHECK (ended_at IS NULL OR ended_at >= started_at),
    CONSTRAINT ck_arena_match_vencedor CHECK (winner_team IN (1, 2))
);

CREATE INDEX ix_arena_match_arena ON game.arena_match (arena_code, started_at DESC);

CREATE TABLE game.arena_round (
    match_id    BIGINT NOT NULL,
    round_no    SMALLINT NOT NULL,
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at    TIMESTAMPTZ,
    winner_team SMALLINT,
    is_tiebreak BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_arena_round PRIMARY KEY (match_id, round_no),
    CONSTRAINT fk_arena_round_match FOREIGN KEY (match_id)
        REFERENCES game.arena_match (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_arena_round_numero CHECK (round_no BETWEEN 1 AND 9),
    CONSTRAINT ck_arena_round_fim CHECK (ended_at IS NULL OR ended_at >= started_at),
    CONSTRAINT ck_arena_round_vencedor CHECK (winner_team IN (1, 2))
);

-- Placar por jogador e round; a FK composta garante que o round existe na partida citada.
CREATE TABLE game.arena_score (
    match_id     BIGINT NOT NULL,
    round_no     SMALLINT NOT NULL,
    character_id BIGINT NOT NULL,
    team         SMALLINT NOT NULL,
    kills        SMALLINT NOT NULL DEFAULT 0,
    assists      SMALLINT NOT NULL DEFAULT 0,
    damage       BIGINT NOT NULL DEFAULT 0,
    deaths       SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_arena_score PRIMARY KEY (match_id, round_no, character_id),
    CONSTRAINT fk_arena_score_round FOREIGN KEY (match_id, round_no)
        REFERENCES game.arena_round (match_id, round_no) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_arena_score_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_arena_score_time CHECK (team IN (1, 2)),
    CONSTRAINT ck_arena_score_kills CHECK (kills >= 0),
    CONSTRAINT ck_arena_score_assists CHECK (assists >= 0),
    CONSTRAINT ck_arena_score_damage CHECK (damage >= 0),
    CONSTRAINT ck_arena_score_deaths CHECK (deaths >= 0)
);

CREATE INDEX ix_arena_score_character ON game.arena_score (character_id);
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE game.arena_score, game.arena_round, game.arena_match, game.castle_state,
           game.kefra_state, game.event_participation, game.character_quest;

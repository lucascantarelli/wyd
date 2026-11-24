-- 0026_content_quests_events: quests, agenda de eventos e arena como definição.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabelas base seed-only e read-only em runtime; nenhum GRANT novo (0001 já concede SELECT em content).

-- migrate:up
-- Quest é conteúdo: `code` é a chave legível e estável, sem identidade (P-04).
CREATE TABLE content.quest (
    code             TEXT NOT NULL,
    name             TEXT NOT NULL,
    kind             TEXT NOT NULL,
    min_level        SMALLINT NOT NULL DEFAULT 1,
    class_code       TEXT,
    repeatable       BOOLEAN NOT NULL DEFAULT false,
    cooldown_seconds INTEGER NOT NULL DEFAULT 0,
    -- Lista de códigos de quest anterior. Array não tem FK: a integridade é do seed (ver página §10).
    requires         TEXT[] NOT NULL DEFAULT '{}',
    description      TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_quest PRIMARY KEY (code),
    CONSTRAINT fk_quest_classe FOREIGN KEY (class_code)
        REFERENCES content.game_class (slug) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_quest_code CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_quest_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 120),
    CONSTRAINT ck_quest_kind CHECK (kind IN ('main', 'class', 'daily', 'event', 'sub')),
    CONSTRAINT ck_quest_nivel CHECK (min_level BETWEEN 1 AND 400),
    CONSTRAINT ck_quest_cooldown CHECK (cooldown_seconds >= 0),
    -- Uma quest não exige a si mesma, não carrega código vazio e não vira lista sem fim.
    CONSTRAINT ck_quest_requires CHECK (
        NOT (code = ANY (requires))
        AND array_position(requires, '') IS NULL
        AND coalesce(array_length(requires, 1), 0) <= 32)
);

CREATE INDEX ix_quest_kind_nivel ON content.quest (kind, min_level);
-- P-15: FK de classe é nullable e não aparece na PK.
CREATE INDEX ix_quest_classe ON content.quest (class_code) WHERE class_code IS NOT NULL;

-- Passo é filho possuído da quest; alvo é polimórfico (mob, item, mapa), por isso não tem FK.
CREATE TABLE content.quest_step (
    quest_code  TEXT NOT NULL,
    step        SMALLINT NOT NULL,
    objective   TEXT NOT NULL,
    target_id   INTEGER,
    count       INTEGER NOT NULL DEFAULT 1,
    map_id      SMALLINT,
    pos_x       SMALLINT,
    pos_y       SMALLINT,
    description TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_quest_step PRIMARY KEY (quest_code, step),
    CONSTRAINT fk_quest_step_quest FOREIGN KEY (quest_code)
        REFERENCES content.quest (code) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_quest_step_mapa FOREIGN KEY (map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_quest_step_step CHECK (step > 0),
    CONSTRAINT ck_quest_step_objetivo CHECK (
        objective IN ('kill', 'collect', 'talk', 'reach', 'refine', 'compose', 'use_item', 'escort')),
    CONSTRAINT ck_quest_step_alvo CHECK (target_id IS NULL OR target_id >= 0),
    CONSTRAINT ck_quest_step_count CHECK (count > 0),
    -- Posição só é obrigatória em `reach`; nos demais objetivos ela é opcional.
    CONSTRAINT ck_quest_step_reach_posicao CHECK (
        objective <> 'reach'
        OR (map_id IS NOT NULL AND pos_x IS NOT NULL AND pos_y IS NOT NULL)),
    CONSTRAINT ck_quest_step_posicao CHECK (
        (pos_x IS NULL OR pos_x BETWEEN 0 AND 32767)
        AND (pos_y IS NULL OR pos_y BETWEEN 0 AND 32767))
);

CREATE INDEX ix_quest_step_mapa ON content.quest_step (map_id) WHERE map_id IS NOT NULL;

-- Recompensa por tipo. A ficha tornava `item_index` NOT NULL numa PK de quatro colunas, o que
-- contradiz o próprio CHECK (`kind = 'item'` exige item; ouro e EXP não têm item). Aqui a linha
-- tem identidade e o único fecha a dupla (tipo, item) sem obrigar item em toda recompensa.
CREATE TABLE content.quest_reward (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    quest_code TEXT NOT NULL,
    kind       TEXT NOT NULL,
    item_index INTEGER,
    amount     BIGINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_quest_reward PRIMARY KEY (id),
    CONSTRAINT uq_quest_reward UNIQUE NULLS NOT DISTINCT (quest_code, kind, item_index),
    CONSTRAINT fk_quest_reward_quest FOREIGN KEY (quest_code)
        REFERENCES content.quest (code) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_quest_reward_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_quest_reward_kind CHECK (
        kind IN ('item', 'gold', 'exp', 'fame', 'skill_point')),
    CONSTRAINT ck_quest_reward_item CHECK ((kind = 'item') = (item_index IS NOT NULL)),
    CONSTRAINT ck_quest_reward_amount CHECK (amount > 0)
);

CREATE INDEX ix_quest_reward_item ON content.quest_reward (item_index)
    WHERE item_index IS NOT NULL;

-- Evento de mundo: chave legível, sem identidade.
CREATE TABLE content.world_event (
    code          TEXT NOT NULL,
    name          TEXT NOT NULL,
    kind          TEXT NOT NULL,
    enabled       BOOLEAN NOT NULL DEFAULT true,
    requires_city BOOLEAN NOT NULL DEFAULT false,
    description   TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_world_event PRIMARY KEY (code),
    CONSTRAINT ck_world_event_code CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_world_event_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 120),
    CONSTRAINT ck_world_event_kind CHECK (
        kind IN ('kefra', 'noatun', 'city_war', 'rvr', 'tower', 'battle_royale', 'custom'))
);

-- Agenda: horário em minuto do dia, não `TIME` — P-06 proíbe `time` para evento, e o minuto
-- continua comparável sem depender do fuso do host. `timezone` diz em que fuso o minuto vale.
CREATE TABLE content.event_schedule (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    event_code       TEXT NOT NULL,
    weekday          SMALLINT NOT NULL,
    start_minute     SMALLINT NOT NULL,
    duration_minutes SMALLINT NOT NULL,
    timezone         TEXT NOT NULL DEFAULT 'America/Sao_Paulo',
    every_n_weeks    SMALLINT NOT NULL DEFAULT 1,
    enabled          BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_event_schedule PRIMARY KEY (id),
    CONSTRAINT uq_event_schedule UNIQUE (event_code, weekday, start_minute),
    CONSTRAINT fk_event_schedule_event FOREIGN KEY (event_code)
        REFERENCES content.world_event (code) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_event_schedule_weekday CHECK (weekday BETWEEN 0 AND 6),
    CONSTRAINT ck_event_schedule_inicio CHECK (start_minute BETWEEN 0 AND 1439),
    CONSTRAINT ck_event_schedule_duracao CHECK (duration_minutes > 0),
    CONSTRAINT ck_event_schedule_fuso CHECK (char_length(btrim(timezone)) BETWEEN 1 AND 64),
    CONSTRAINT ck_event_schedule_periodo CHECK (every_n_weeks BETWEEN 1 AND 8)
);

-- Arena é definição; a partida é estado e fica em 0028.
CREATE TABLE content.arena (
    code      TEXT NOT NULL,
    name      TEXT NOT NULL,
    kind      TEXT NOT NULL,
    team_size SMALLINT NOT NULL DEFAULT 16,
    rounds    SMALLINT NOT NULL DEFAULT 3,
    round_ms  INTEGER NOT NULL DEFAULT 300000,
    respawns  SMALLINT NOT NULL DEFAULT 1,
    min_level SMALLINT NOT NULL DEFAULT 200,
    no_drop   BOOLEAN NOT NULL DEFAULT true,
    enabled   BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_arena PRIMARY KEY (code),
    CONSTRAINT ck_arena_code CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),
    CONSTRAINT ck_arena_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_arena_kind CHECK (kind IN ('royal', 'battle_royale')),
    CONSTRAINT ck_arena_time CHECK (team_size BETWEEN 1 AND 64),
    CONSTRAINT ck_arena_rounds CHECK (rounds BETWEEN 1 AND 9),
    CONSTRAINT ck_arena_round_ms CHECK (round_ms > 0),
    CONSTRAINT ck_arena_respawns CHECK (respawns BETWEEN 0 AND 9),
    CONSTRAINT ck_arena_nivel CHECK (min_level BETWEEN 1 AND 400)
);

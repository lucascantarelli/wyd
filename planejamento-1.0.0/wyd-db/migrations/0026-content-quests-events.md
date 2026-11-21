> Pergunta: "como aplico o `0026_content_quests_events` e como quest, agenda de evento e arena passam a existir no banco?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 26); `ddl-confirmado.md` §5.26 (confirmado em 2026-11-21); fichas legadas de `0013_quests.sql`, `0014_eventos.sql` e `0015_arena.sql`
> Revisado em: 2026-11-21

Vigésimo sexto arquivo da série: 6 tabelas no schema `content`. As fichas vêm do legado (nível 4) e
são **hipótese `H`**; a cadeia de quests do 7662 está declarada não fechada, então a tabela recebe o
dado, não o inventa.

!!! note "Aprovado em 2026-11-21"
    O dono aprovou o conjunto em 2026-11-21, com `start_minute SMALLINT` na agenda e `id` identity com
    `UNIQUE NULLS NOT DISTINCT` na recompensa (§8). O lote está `concluída` em `migrations-checklist.md`
    §2 e as seis tabelas contam entre as 97 com DDL fechado.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0026` (checklist, linha 110) |
| Arquivo | `crates/wyd-db/migrations/0026_content_quests_events.sql` |
| Schema | `content` (6 tabelas) |
| Posição | 26 de 31 |
| Depende de | `0001`; `0013` (`content.game_class`); `0014` (`content.item_catalog`); `0024` (`content.map_definition`) |
| É dependência de | `0027` (seed) e `0028` (`character_quest`, `event_participation`, `arena_*`) |
| Tabelas criadas | `quest`, `quest_step`, `quest_reward`, `world_event`, `event_schedule`, `arena` |
| Objetos criados | 6 tabelas, 6 FKs, 2 únicos extras, 4 índices secundários, 0 triggers |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar a definição de quest, passo e recompensa, o evento de mundo com sua agenda e a definição de
arena. É o conteúdo que o servidor carrega no boot e que `0028` referencia como estado de jogador.

## 3. Pré-requisitos

1. `0001_bootstrap`: schema `content` e default privileges.
2. `0013_content_taxonomy`: `content.game_class(slug)`.
3. `0014_content_item`: `content.item_catalog(item_index)`.
4. `0024_content_maps`: `content.map_definition(map_id)` (FK de `quest_step`).
5. Conexão como `wyd_migrator`; o único com `NULLS NOT DISTINCT` exige PostgreSQL 15+ (alvo PG17).

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.26, copiado sem alteração.

```sql
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
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP TABLE content.arena;
DROP TABLE content.event_schedule;
DROP TABLE content.world_event;
DROP INDEX content.ix_quest_reward_item;
DROP TABLE content.quest_reward;
DROP INDEX content.ix_quest_step_mapa;
DROP TABLE content.quest_step;
DROP INDEX content.ix_quest_classe;
DROP INDEX content.ix_quest_kind_nivel;
DROP TABLE content.quest;
```

## 6. Colunas

Origem `H` = hipótese derivada da ficha do legado; `P` = provada por fonte do planejamento novo.

### `content.quest`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `code` | `TEXT` | não | `H` | Chave legível e estável; PK sem identidade (`P-04`), formato de slug. |
| `name` | `TEXT` | não | `H` | Nome exibido; 1–120. |
| `kind` | `TEXT` | não | `H` | `main`, `class`, `daily`, `event`, `sub`. |
| `min_level` | `SMALLINT` | não | `H` | Nível mínimo, 1–400. |
| `class_code` | `TEXT` | sim | `H` | Classe que aceita; nula = todas. FK para `game_class(slug)`. |
| `repeatable` | `BOOLEAN` | não | `H` | Pode ser repetida. |
| `cooldown_seconds` | `INTEGER` | não | `H` | Recarga entre repetições, `>= 0`. |
| `requires` | `TEXT[]` | não | `H` | Códigos de quest anterior; lista, sem FK (ver §10). |
| `description` | `TEXT` | não | `H` | Texto de autoria/portal. |

### `content.quest_step`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `quest_code` | `TEXT` | não | `P` | Quest dona; metade da PK e FK `CASCADE`. |
| `step` | `SMALLINT` | não | `H` | Ordem do passo, `> 0`. |
| `objective` | `TEXT` | não | `H` | Oito verbos conhecidos (`kill`…`escort`). |
| `target_id` | `INTEGER` | sim | `H` | Alvo polimórfico (mob, item, mapa); sem FK. |
| `count` | `INTEGER` | não | `H` | Quantidade necessária, `> 0`. |
| `map_id` | `SMALLINT` | sim | `H` | Mapa do passo; FK `RESTRICT` para `map_definition`. |
| `pos_x` | `SMALLINT` | sim | `H` | Coordenada X; obrigatória em `reach`. |
| `pos_y` | `SMALLINT` | sim | `H` | Coordenada Y; obrigatória em `reach`. |
| `description` | `TEXT` | não | `H` | Texto de autoria/portal. |

### `content.quest_reward`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `BIGINT` | não | `P` | Identidade (`P-03`); a ficha tinha PK de quatro colunas, incoerente com o próprio CHECK. |
| `quest_code` | `TEXT` | não | `P` | Quest que paga; FK `CASCADE`. |
| `kind` | `TEXT` | não | `H` | `item`, `gold`, `exp`, `fame`, `skill_point`. |
| `item_index` | `INTEGER` | sim | `H` | Item da recompensa; obrigatório só quando `kind = 'item'`. |
| `amount` | `BIGINT` | não | `H` | Quantidade, `> 0`. |

### `content.world_event`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `code` | `TEXT` | não | `H` | Chave legível; PK. |
| `name` | `TEXT` | não | `H` | Nome do evento. |
| `kind` | `TEXT` | não | `H` | `kefra`, `noatun`, `city_war`, `rvr`, `tower`, `battle_royale`, `custom`. |
| `enabled` | `BOOLEAN` | não | `H` | Desliga sem apagar. |
| `requires_city` | `BOOLEAN` | não | `H` | Exige cidade/território. |
| `description` | `TEXT` | não | `H` | Texto de autoria/portal. |

### `content.event_schedule`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `BIGINT` | não | `P` | Identidade (`P-03`); a agenda não tem chave natural. |
| `event_code` | `TEXT` | não | `P` | Evento agendado; FK `CASCADE`. |
| `weekday` | `SMALLINT` | não | `H` | Dia da semana, 0 = domingo. |
| `start_minute` | `SMALLINT` | não | `H` | Minuto do dia, 0–1439; evita `TIME`, proibido por `P-06`. |
| `duration_minutes` | `SMALLINT` | não | `H` | Duração, `> 0`. |
| `timezone` | `TEXT` | não | `H` | Nome IANA; o evento não anda com o fuso do host. |
| `every_n_weeks` | `SMALLINT` | não | `H` | Periodicidade, 1–8. |
| `enabled` | `BOOLEAN` | não | `H` | Desliga sem apagar. |

### `content.arena`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `code` | `TEXT` | não | `H` | Chave legível; PK. |
| `name` | `TEXT` | não | `H` | Nome da arena. |
| `kind` | `TEXT` | não | `H` | `royal` ou `battle_royale`. |
| `team_size` | `SMALLINT` | não | `H` | Jogadores por equipe, 1–64. |
| `rounds` | `SMALLINT` | não | `H` | Rounds, 1–9. |
| `round_ms` | `INTEGER` | não | `H` | Duração do round em ms, `> 0`. |
| `respawns` | `SMALLINT` | não | `H` | Ressurreições, 0–9. |
| `min_level` | `SMALLINT` | não | `H` | Nível mínimo, 1–400. |
| `no_drop` | `BOOLEAN` | não | `H` | Sem drop ao morrer. |
| `enabled` | `BOOLEAN` | não | `H` | Desliga sem apagar. |

Sem `content_source`/`confidence`: proveniência fica em `content.content_source` (`0019`), como §5.22.

## 7. Constraints e índices

- PKs: `quest(code)`, `quest_step(quest_code, step)`, `quest_reward(id)`, `world_event(code)`,
  `event_schedule(id)`, `arena(code)`.
- FKs: `quest.class_code` → `game_class(slug)` em `RESTRICT`; `quest_step` → `quest` em `CASCADE` e →
  `map_definition` em `RESTRICT`; `quest_reward` → `quest` em `CASCADE` e → `item_catalog` em
  `RESTRICT`; `event_schedule` → `world_event` em `CASCADE`.
- Únicos: `uq_quest_reward (quest_code, kind, item_index) NULLS NOT DISTINCT` e
  `uq_event_schedule (event_code, weekday, start_minute)`.
- Índices: `ix_quest_kind_nivel`, `ix_quest_classe` (parcial), `ix_quest_step_mapa` (parcial),
  `ix_quest_reward_item` (parcial) — as três parciais são FKs nullable fora da PK (`P-15`).
- Nenhum `GRANT` novo: `0001` já concede `SELECT` em `content`.

### Diferenças em relação às fichas do legado

| Ficha | Aqui | Por quê |
|---|---|---|
| `quest.class_code` → `game_class(code)` | → `content.game_class(slug)` | a chave de classe em `0013` é `slug`; `code` não existe |
| `quest_reward` PK `(quest_code, kind, amount, item_index)` com `item_index NOT NULL` | `id` identity + único `NULLS NOT DISTINCT` | a ficha contradiz o próprio CHECK: ouro/EXP não têm item, e a PK obrigaria todos a ter |
| `event_schedule.start_time TIME` | `start_minute SMALLINT` 0–1439 | `P-06` proíbe `time` para evento; minuto é comparável e não depende do fuso do host |
| `event_schedule.id INTEGER` | `BIGINT` identity | `P-03` |
| `quest_step.pos_x/pos_y 0..4095` | `0..32767` | 4095 é teto do loader, não do banco (mesma decisão de §5.24) |
| `content_source`/`confidence` | ausentes | proveniência em `content.content_source` (`0019`) |
| `quest.requires TEXT[]` | mantido | lista de códigos não vira tabela sem sair do escopo de 97 nomes; integridade é do seed |

## 8. Decisões de forma

1. `quest`, `world_event` e `arena` com `code` como PK natural; `quest_step` com PK composta.
2. `quest_reward` com identidade e único `(quest_code, kind, item_index)` `NULLS NOT DISTINCT`.
3. Agenda com `start_minute` + `timezone`, sem `TIME` (`P-06`).
4. Posição obrigatória apenas em `reach`; `target_id` polimórfico sem FK.
5. Horário da agenda: `start_minute SMALLINT` 0–1439, respondido pelo dono em 2026-11-21 (`P-06`
   recusa `TIME`).
6. Identidade da recompensa: `id` identity + `UNIQUE NULLS NOT DISTINCT (quest_code, kind, item_index)`,
   respondido pelo dono em 2026-11-21.

## 9. Prova prevista (não executada)

1. Passo repetido na mesma quest → `23505` em `pk_quest_step`; duas janelas iguais → `23505` em `uq_event_schedule`.
2. `reach` sem `map_id`/`pos_x`/`pos_y` → `23514`.
3. `quest_reward` de ouro com `item_index` preenchido, ou de item sem item → `23514`.
4. Duas recompensas de ouro iguais para a mesma quest → `23505`; uma de ouro e uma de item → aceitas.
5. `quest.requires` contendo o próprio `code` → `23514`.
6. `DELETE` de `quest` → passos e recompensas somem (`CASCADE`); `DELETE` de mapa usado por passo → `23503`.
7. `SET ROLE wyd_game; INSERT INTO content.quest …` → `42501` (só `SELECT`).
8. `down` derruba índices e tabelas sem erro de dependência.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| Cadeia de quests do 7662 | ficha declara não fechada; seed seria invenção (`0027`) |
| Integridade de `quest.requires` | lista sem FK; validação é do seed |
| Calendário real de Kefra/Noatun | `0027`, bloqueado pela verificação do dado de origem |
| Execução de evento e progresso de quest | `wyd-server` |
| Validação do fuso IANA | runtime; o CHECK só limita tamanho |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*

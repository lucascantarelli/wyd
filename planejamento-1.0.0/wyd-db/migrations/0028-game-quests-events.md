> Pergunta: "como aplico o `0028_game_quests_events` e como progresso de quest, histórico de evento e arena passam a existir no banco?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 28); `ddl-confirmado.md` §5.28 (confirmado em 2026-11-21); fichas legadas de `0013_quests.sql`, `0014_eventos.sql` e `0015_arena.sql`
> Revisado em: 2026-11-21

**Estado:** aceito (2026-11-24)

Vigésimo oitavo arquivo da série: 7 tabelas no schema `game`. É o único bloco de `game` que referencia
`content`, por isso fecha depois de `0026`. As fichas vêm do legado (nível 4) e são **hipótese `H`**.

!!! note "Aprovado em 2026-11-21"
    O dono aprovou o conjunto em 2026-11-21, sem alterar as quatro decisões de §8. O lote está
    `concluída` em `migrations-checklist.md` §2 e as sete tabelas contam entre as 109 com DDL fechado.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0028` (checklist, linha 112) |
| Arquivo | `crates/wyd-db/migrations/0028-game-quests-events.sql` |
| Schema | `game` (7 tabelas) |
| Posição | 28 de 34 |
| Depende de | `0001`; `0006` (`game.character`); `0010` (`game.guild`); `0011` (`game.city`); `0026` (`content.quest`, `content.world_event`, `content.arena`) |
| É dependência de | `0029` (view de presença não lê estas tabelas) — nenhuma |
| Tabelas criadas | `character_quest`, `event_participation`, `kefra_state`, `castle_state`, `arena_match`, `arena_round`, `arena_score` |
| Objetos criados | 7 tabelas, 12 FKs, 0 únicos extras, 9 índices secundários, 0 triggers |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar o estado de jogador que nasce das definições de `0026`: progresso de quest, participação em
evento, placar de arena e o estado único de Kefra e do castelo. Escrita só do servidor.

## 3. Pré-requisitos

1. `0001_bootstrap`: schema `game` e default privileges (`SELECT, INSERT, UPDATE, DELETE` para `wyd_game`).
2. `0006_game_character`: `game.character(id)`.
3. `0010_game_guild`: `game.guild(id)`.
4. `0011_game_economy`: `game.city(code)`.
5. `0026_content_quests_events`: `content.quest`, `content.world_event` e `content.arena`.
6. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.28, copiado sem alteração.

```sql
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
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP INDEX game.ix_arena_score_character;
DROP TABLE game.arena_score;
DROP TABLE game.arena_round;
DROP INDEX game.ix_arena_match_arena;
DROP TABLE game.arena_match;
DROP INDEX game.ix_castle_state_cidade;
DROP INDEX game.ix_castle_state_guild;
DROP TABLE game.castle_state;
DROP INDEX game.ix_kefra_state_guild;
DROP TABLE game.kefra_state;
DROP INDEX game.ix_event_participation_guild;
DROP INDEX game.ix_event_participation_character;
DROP INDEX game.ix_event_participation_event;
DROP TABLE game.event_participation;
DROP INDEX game.ix_character_quest_quest;
DROP TABLE game.character_quest;
```

## 6. Colunas

Origem `H` = hipótese derivada da ficha do legado; `P` = provada por fonte do planejamento novo.

### `game.character_quest`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | `P` | Dono do progresso; metade da PK e FK `CASCADE`. |
| `quest_code` | `TEXT` | não | `P` | Quest em progresso; FK `RESTRICT` para `content.quest`. |
| `step` | `SMALLINT` | não | `H` | Passo atual, `> 0`. |
| `progress` | `INTEGER` | não | `H` | Progresso do passo, `>= 0`. |
| `started_at` | `TIMESTAMPTZ` | não | `H` | Quando aceitou. |
| `completed_at` | `TIMESTAMPTZ` | sim | `H` | Quando concluiu; `>= started_at`. |

### `game.event_participation`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `BIGINT` | não | `P` | Identidade (`P-03`); histórico sem chave natural. |
| `event_code` | `TEXT` | não | `P` | Evento disputado; FK `RESTRICT`. |
| `character_id` | `BIGINT` | sim | `H` | Personagem; `SET NULL` no delete. |
| `guild_id` | `BIGINT` | sim | `H` | Clã do participante; `SET NULL`. |
| `result` | `TEXT` | não | `H` | `win`, `lose`, `draw`, `participated`. |
| `score` | `BIGINT` | não | `H` | Pontuação, `>= 0`. |
| `at` | `TIMESTAMPTZ` | não | `H` | Momento do evento. |

### `game.kefra_state`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | `P` | Linha única; `CHECK (id = 1)`, sem identidade. |
| `last_kill_at` | `TIMESTAMPTZ` | sim | `H` | Última morte do Kefra. |
| `next_spawn_at` | `TIMESTAMPTZ` | sim | `H` | Próximo nascimento. |
| `kill_count` | `INTEGER` | não | `H` | Total de mortes, `>= 0`. |
| `last_guild_id` | `BIGINT` | sim | `H` | Clã que matou por último; `SET NULL`. |

### `game.castle_state`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | `P` | Linha única; `CHECK (id = 1)`. |
| `owner_guild_id` | `BIGINT` | sim | `H` | Clã dono; `SET NULL`. |
| `city_code` | `TEXT` | sim | `H` | Cidade associada; FK `RESTRICT` para `game.city(code)`. |
| `vault_gold` | `BIGINT` | não | `H` | Ouro no cofre, `>= 0`. |
| `last_collected_at` | `TIMESTAMPTZ` | sim | `H` | Última coleta. |
| `last_war_at` | `TIMESTAMPTZ` | sim | `H` | Última guerra disputada. |

### `game.arena_match`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `BIGINT` | não | `P` | Identidade (`P-03`); a ficha usava `INTEGER`. |
| `arena_code` | `TEXT` | não | `P` | Definição em `content.arena`; FK `RESTRICT`. |
| `started_at` | `TIMESTAMPTZ` | não | `H` | Início da partida. |
| `ended_at` | `TIMESTAMPTZ` | sim | `H` | Fim; `>= started_at`. |
| `winner_team` | `SMALLINT` | sim | `H` | Equipe vencedora, 1 ou 2. |

### `game.arena_round`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `match_id` | `BIGINT` | não | `P` | Partida dona; metade da PK e FK `CASCADE`. |
| `round_no` | `SMALLINT` | não | `H` | Número do round, 1–9. |
| `started_at` | `TIMESTAMPTZ` | não | `H` | Início do round. |
| `ended_at` | `TIMESTAMPTZ` | sim | `H` | Fim; `>= started_at`. |
| `winner_team` | `SMALLINT` | sim | `H` | Vencedor do round, 1 ou 2. |
| `is_tiebreak` | `BOOLEAN` | não | `H` | Round de desempate. |

### `game.arena_score`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `match_id` | `BIGINT` | não | `P` | Parte da PK e da FK composta para o round. |
| `round_no` | `SMALLINT` | não | `P` | Idem. |
| `character_id` | `BIGINT` | não | `P` | Jogador do placar; FK `CASCADE`. |
| `team` | `SMALLINT` | não | `H` | Equipe, 1 ou 2. |
| `kills` | `SMALLINT` | não | `H` | Abates, `>= 0`. |
| `assists` | `SMALLINT` | não | `H` | Assistências, `>= 0`. |
| `damage` | `BIGINT` | não | `H` | Dano causado, `>= 0`. |
| `deaths` | `SMALLINT` | não | `H` | Mortes, `>= 0`. |

## 7. Constraints e índices

- PKs: `character_quest(character_id, quest_code)`, `event_participation(id)`, `kefra_state(id)`,
  `castle_state(id)`, `arena_match(id)`, `arena_round(match_id, round_no)`,
  `arena_score(match_id, round_no, character_id)`.
- FKs: `character_quest` → `character` (`CASCADE`) e → `quest` (`RESTRICT`); `event_participation` →
  `world_event` (`RESTRICT`), `character` (`SET NULL`) e `guild` (`SET NULL`); `kefra_state` →
  `guild` (`SET NULL`); `castle_state` → `guild` (`SET NULL`) e `city` (`RESTRICT`); `arena_match` →
  `arena` (`RESTRICT`); `arena_round` → `arena_match` (`CASCADE`); `arena_score` → `arena_round`
  (`CASCADE`) e `character` (`CASCADE`).
- Índices: `ix_character_quest_quest`, `ix_event_participation_event`,
  `ix_event_participation_character` (parcial), `ix_event_participation_guild` (parcial),
  `ix_kefra_state_guild` (parcial), `ix_castle_state_guild` (parcial), `ix_castle_state_cidade`
  (parcial), `ix_arena_match_arena` e `ix_arena_score_character` — todas as FKs fora da PK (`P-15`).
- Nenhum `GRANT` novo: `0001` já dá `SELECT, INSERT, UPDATE, DELETE` em `game` para `wyd_game`, e o
  portal não recebe escrita.

### Diferenças em relação às fichas do legado

| Ficha | Aqui | Por quê |
|---|---|---|
| `character_id` → `characters(id)` | → `game.character(id)` | nome e schema do canônico (`P-02`, `0006`) |
| `quest_code` → `quest` em `CASCADE` | `RESTRICT` | apagar definição não pode apagar progresso de jogador |
| `event_code` → `world_event` em `CASCADE` | `RESTRICT` | idem para histórico de evento |
| `arena_code` → `arena` em `CASCADE` | `RESTRICT` | idem para partidas registradas |
| `arena_match.id INTEGER` | `BIGINT` identity | `P-03` |
| `character_quest` com escrita de “seed/staff” | servidor | o próprio título da ficha diz escrita só do servidor; seed não cria progresso |
| `kefra_state`/`castle_state` `id SMALLINT` | mantido com `CHECK (id = 1)` | linha única, como `content_version` |

## 8. Decisões de forma

1. PKs compostas onde a chave natural já é o acesso; identidade nas tabelas de histórico.
2. `RESTRICT` nas FKs para `content`: conteúdo apagado não leva estado de jogador junto.
3. Linha única de Kefra e castelo com `SMALLINT` + `CHECK (id = 1)`.
4. Escrita só do servidor; o portal lê pela view de `0029`.

## 9. Prova de execução

As consultas abaixo não foram executadas uma a uma; em 2026-11-24 rodaram a aplicação da série
do zero e os 13 cenários de `sistema_pg` (`planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md`
§6.6.30).

1. `INSERT` de progresso com personagem ou quest inexistente → `23503`.
2. `DELETE` de `content.quest`/`world_event`/`arena` com estado dependente → `23503` (`RESTRICT`).
3. `DELETE` de personagem → progresso e placar somem; `event_participation.character_id` vira nulo.
4. `DELETE` de guilda → `event_participation.guild_id`, `kefra_state.last_guild_id` e
   `castle_state.owner_guild_id` viram nulo.
5. Segundo `INSERT` em `kefra_state`/`castle_state` → `23514` (`CHECK (id = 1)`).
6. `arena_round` com `round_no = 10`, `team = 3` ou `ended_at < started_at` → `23514`.
7. `arena_score` para round inexistente → `23503` (FK composta).
8. `SET ROLE wyd_portal; INSERT INTO game.character_quest …` → `42501`.
9. `down` derruba índices e tabelas sem erro de dependência.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| Regras de progresso, recompensa e placar | `wyd-server` |
| Estado de Kefra por canal versus instalação | arquitetura; a ficha tem uma linha só |
| Retenção de `event_participation` e `arena_*` | `M22` não cobre estas tabelas; política futura |
| Leitura do portal | `0029` e a permissão da view |
| Valores do seed de quests/eventos/arena | `0027`, bloqueado pela verificação do dado de origem |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*

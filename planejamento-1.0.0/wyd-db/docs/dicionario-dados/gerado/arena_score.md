> Pergunta: "o que `game.arena_score` guarda por jogador e round e por que a FK do round é composta?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Placar de um jogador num round: equipe, abates, assistências, dano e mortes. É o detalhe que a partida
(`game.arena_match`) não guarda e o round (`game.arena_round`) não detalha por jogador.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `match_id` | `BIGINT` | não | — | partida; PK com `round_no` e `character_id` |
| `round_no` | `SMALLINT` | não | — | round; PK |
| `character_id` | `BIGINT` | não | — | jogador; PK e FK `CASCADE` |
| `team` | `SMALLINT` | não | — | equipe do jogador no round, 1 ou 2 |
| `kills` | `SMALLINT` | não | `0` | abates, `>= 0` |
| `assists` | `SMALLINT` | não | `0` | assistências, `>= 0` |
| `damage` | `BIGINT` | não | `0` | dano causado, `>= 0` |
| `deaths` | `SMALLINT` | não | `0` | mortes, `>= 0` |

Sem carimbos: o round já tem `started_at`/`ended_at`, e o placar é do round.

## Chave primária

`pk_arena_score (match_id, round_no, character_id)` — um placar por jogador e round. `team` **não** entra
na chave: o jogador aparece uma vez no round, e a equipe é atributo da linha, não da identidade.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK composta):** `fk_arena_score_round`:
  `(match_id, round_no) → game.arena_round (match_id, round_no)`, `ON UPDATE RESTRICT`,
  `ON DELETE CASCADE`. A composição é o que impede placar de citar round de outra partida: um
  `(match_id, round_no)` isolado apontaria para o round errado se a FK fosse só em `round_no`.
- **Saída, `CASCADE` (1 FK):** `fk_arena_score_character`: `character_id → game.character (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — apagar personagem apaga o placar dele, ao contrário da
  partida, que não é de um personagem.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o placar.

## Unicidade

Só a PK: não há segundo único, e a mesma linha não se repete porque `(match_id, round_no, character_id)` é
a identidade.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_arena_score_time` | `team` diferente de 1 e de 2 |
| `ck_arena_score_kills` | `kills` negativo |
| `ck_arena_score_assists` | `assists` negativo |
| `ck_arena_score_damage` | `damage` negativo |
| `ck_arena_score_deaths` | `deaths` negativo |

`team` é obrigatório (sem `NULL`), diferente de `winner_team` na partida e no round, que aceitam nulo: o
placar sempre sabe de que lado o jogador estava.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_arena_score_character (character_id)` | lado filho da FK do personagem (`P-15`); sustenta o `CASCADE` e a pergunta "como este jogador se saiu" |

A leitura por round usa o prefixo `(match_id, round_no)` da PK.

## Armadilha de leitura

`damage` é `BIGINT` e `kills`/`assists`/`deaths` são `SMALLINT`: somar os quatro no mesmo tipo estreito
estoura, e o total de dano de um jogador exige `BIGINT`. O placar **não** guarda posição final nem
pontuação de campeonato — a única ordem é por round, e o vencedor está em `winner_team` do round/partida.
`team` é por round, não por partida: nada no schema impede o mesmo personagem aparecer com times
diferentes em rounds da mesma partida, e a regra de troca é do runtime.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001-bootstrap.md` §7). Nenhum seed cria placar.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028-game-quests-events.md` — arquivo `0028_game_quests_events.sql`.

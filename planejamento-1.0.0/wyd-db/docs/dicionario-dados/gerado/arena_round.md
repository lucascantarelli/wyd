> Pergunta: "o que `game.arena_round` guarda de cada round e por que o desempate não é um round fora da faixa?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Round de uma partida de arena: número, início, fim, equipe vencedora e marca de desempate. É filho
possuído da partida e o alvo da FK composta do placar.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `match_id` | `BIGINT` | não | — | partida dona; PK com `round_no` |
| `round_no` | `SMALLINT` | não | — | número do round, 1–9 |
| `started_at` | `TIMESTAMPTZ` | não | `now()` | início do round |
| `ended_at` | `TIMESTAMPTZ` | sim | — | fim; não anterior a `started_at` |
| `winner_team` | `SMALLINT` | sim | — | vencedor do round, 1 ou 2 |
| `is_tiebreak` | `BOOLEAN` | não | `false` | round de desempate |

Sem `updated_at`: `started_at`/`ended_at` são os carimbos.

## Chave primária

`pk_arena_round (match_id, round_no)` — um round por número dentro da partida. A PK é também o alvo da FK
composta de `game.arena_score`: o placar cita `(match_id, round_no)` e não pode apontar para round de
outra partida.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_arena_round_match`: `match_id → game.arena_match (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — round é filho possuído da partida.
- **Entrada, `CASCADE` (1 FK):** `fk_arena_score_round` — o placar é filho do round e some com ele.
- **Sem FK para `content.arena`:** a definição do modo é da partida, não do round; repetir a FK aqui
  criaria dois caminhos para o mesmo vínculo.

## Unicidade

Só a PK. A sequência de rounds pode ter buracos: os rounds 1, 3 e 5 da mesma partida são válidos, e o
schema não exige contiguidade — nem que `round_no` respeite `content.arena.rounds`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_arena_round_numero` | `round_no` fora de 1–9 |
| `ck_arena_round_fim` | `ended_at` anterior a `started_at` |
| `ck_arena_round_vencedor` | `winner_team` diferente de 1 e de 2, quando preenchido |

`is_tiebreak` não tem `CHECK` de coerência com `round_no`: o round de desempate **ocupa** um número de
1 a 9, porque a faixa é do schema e não existe round 10.

## Índices

Nenhum além do índice da PK. A FK de saída usa a PK do pai; a FK de entrada (`arena_score`) é indexada
pelo próprio lado filho, e a leitura por partida usa o prefixo `match_id` da PK.

## Armadilha de leitura

`is_tiebreak = true` marca desempate, mas não cria round extra: uma partida de 3 rounds com desempate tem
no máximo um round 1–9 marcado, e ler "rounds + desempate" como `rounds + 1` erra a contagem. `round_no`
não é gerado nem contíguo, e o teto 9 é do schema; `content.arena.rounds` é a regra do modo, e o schema
não amarra as duas. `winner_team` nulo é round sem desfecho registrado, não empate — empate não tem valor
próprio aqui.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001-bootstrap.md` §7). Nenhum seed cria round.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028-game-quests-events.md` — arquivo `0028-game-quests-events.sql`.

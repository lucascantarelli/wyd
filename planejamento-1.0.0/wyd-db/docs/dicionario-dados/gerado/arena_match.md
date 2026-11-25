> Pergunta: "o que `game.arena_match` registra da partida e por que o placar não mora aqui?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Partida de arena: qual modo de `content.arena`, quando começou, quando terminou e qual equipe venceu.
É o cabeçalho do histórico — round e placar por jogador ficam em `game.arena_round` e `game.arena_score`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade da partida (`P-03`) |
| `arena_code` | `TEXT` | não | — | modo de `content.arena`; FK `RESTRICT` |
| `started_at` | `TIMESTAMPTZ` | não | `now()` | início |
| `ended_at` | `TIMESTAMPTZ` | sim | — | fim; não anterior a `started_at` |
| `winner_team` | `SMALLINT` | sim | — | equipe vencedora, 1 ou 2 |

Sem `created_at`/`updated_at`: `started_at`/`ended_at` são os carimbos, e a partida não é editada depois
de fechada.

## Chave primária

`pk_arena_match (id)` — identidade sintética (`P-03`); a ficha do legado usava `INTEGER`, e o canônico
usa `BIGINT`. Não há chave natural: duas partidas do mesmo modo podem começar no mesmo instante.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (1 FK):** `fk_arena_match_arena`: `arena_code → content.arena (code)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — apagar modo com partida registrada falha alto, em vez de
  levar o histórico junto (a ficha dizia `CASCADE`).
- **Entrada, `CASCADE` (1 FK):** `fk_arena_round_match` — round é filho possuído e some com a partida.
- **Sem FK para personagem:** a partida não é "de um personagem"; quem joga está em `arena_score`.

## Unicidade

Só a PK: não há único por modo ou por instante — duas partidas simultâneas do mesmo modo são registros
legítimos, e o schema não impede.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_arena_match_fim` | `ended_at` anterior a `started_at` |
| `ck_arena_match_vencedor` | `winner_team` diferente de 1 e de 2, quando preenchido |

`winner_team` nulo **passa** no `CHECK`, porque `NULL IN (1, 2)` não é verdadeiro nem falso: o nulo é o
estado "partida sem vencedor" (em andamento ou sem desfecho), e o `CHECK` só recusa valor preenchido fora
de 1/2.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_arena_match_arena (arena_code, started_at DESC)` | histórico de um modo, da partida mais recente para a mais antiga; sustenta o `RESTRICT` do modo |

## Armadilha de leitura

A partida **não** guarda placar nem número de jogadores: o resultado por jogador está em `arena_score`, e
somar `kills` exige juntar os rounds. `ended_at` nulo é partida em andamento, e `winner_team` nulo nesse
estado não é erro. `winner_team` preenchido com `ended_at` nulo é aceito pelo schema: quem fecha a
partida e grava o vencedor na ordem certa é o runtime.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001-bootstrap.md` §7). Nenhum seed cria partida.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028-game-quests-events.md` — arquivo `0028-game-quests-events.sql`.

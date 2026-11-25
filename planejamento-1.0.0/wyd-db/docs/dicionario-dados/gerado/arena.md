> Pergunta: "o que `content.arena` define do modo de arena e por que a partida não mora aqui?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.26; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada; o seed `0027` segue `bloqueada`.

## Papel

Definição do modo de arena por código: tipo, jogadores por equipe, número e duração dos rounds,
ressurreições, nível mínimo e duas chaves de regra — sem drop e ligado/desligado. Partida, round e placar
são estado de jogador e ficam em `game.arena_match`, `game.arena_round` e `game.arena_score` (`0028`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `code` | `TEXT` | não | — | slug `^[a-z][a-z0-9_]{1,63}$`; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–64 após aparar |
| `kind` | `TEXT` | não | — | `royal` ou `battle_royale` |
| `team_size` | `SMALLINT` | não | `16` | jogadores por equipe, 1–64 |
| `rounds` | `SMALLINT` | não | `3` | rounds, 1–9 |
| `round_ms` | `INTEGER` | não | `300000` | duração do round em ms, maior que zero |
| `respawns` | `SMALLINT` | não | `1` | ressurreições, 0–9 |
| `min_level` | `SMALLINT` | não | `200` | nível mínimo, 1–400 |
| `no_drop` | `BOOLEAN` | não | `true` | sem drop ao morrer |
| `enabled` | `BOOLEAN` | não | `false` | desliga sem apagar |

Sem carimbos e sem coluna de origem: definição de conteúdo, seed-only.

## Chave primária

`pk_arena (code)` — chave natural legível, sem identidade (`P-04`). É o código que a partida em `0028`
carrega na FK.

## FKs de saída e de entrada

- **Saída:** nenhuma — é tabela raiz de definição, sem FK de saída.
- **Entrada, `RESTRICT` (1 FK):** `fk_arena_match_arena` (`0028`) — apagar modo com partida registrada
  falha alto, em vez de levar o histórico junto.
- **Entrada:** nenhuma outra; round e placar apontam para a partida, não para a definição.

## Unicidade

Só a PK. Nome e combinação de parâmetros podem repetir entre modos: o `code` é a única identidade.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_arena_code` | `code` fora de minúsculas, dígitos e `_`, começando por letra, com 2–64 caracteres |
| `ck_arena_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |
| `ck_arena_kind` | `kind` fora de `royal` e `battle_royale` |
| `ck_arena_time` | `team_size` fora de 1–64 |
| `ck_arena_rounds` | `rounds` fora de 1–9 |
| `ck_arena_round_ms` | `round_ms` menor ou igual a zero |
| `ck_arena_respawns` | `respawns` fora de 0–9 |
| `ck_arena_nivel` | `min_level` fora de 1–400 |

`no_drop` e `enabled` não têm `CHECK`: são booleanos, e `enabled` nasce `false` — modo recém-semeado fica
desligado até o seed (ou o staff) ligá-lo.

## Índices

Nenhum além do índice da PK. Sem FK de saída e sem coluna de busca secundária declarada.

## Armadilha de leitura

`enabled = false` é o default: a existência da linha não significa que o modo está aberto, e ler a tabela
como "arenas disponíveis" sem filtrar `enabled` publica modo desligado. `rounds` é limite do **schema**
(1–9), e o round de desempate em `0028` (`is_tiebreak`) não cria round extra fora dessa faixa. `round_ms`
é milissegundo inteiro: `300000` é 5 minutos, não 300 s por engano de unidade. E `min_level = 200` é
default de conteúdo, não regra de elegibilidade provada — a regra de entrada é do runtime.

## Papéis

- **Escreve:** `wyd_migrator`, no seed de quests e eventos (`0027`, `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0026-content-quests-events.md` — arquivo `0026-content-quests-events.sql`.

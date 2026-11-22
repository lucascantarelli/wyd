> Pergunta: "o que `content.world_event` define do evento e o que a tabela não guarda de horário?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.26; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada; o seed `0027` segue `bloqueada`.

## Papel

Definição de evento de mundo por código: nome, tipo e duas chaves de comportamento — se está ligado e se
exige cidade. O horário **não** mora aqui: é linha de `content.event_schedule`, e a participação é estado
em `game.event_participation` (`0028`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `code` | `TEXT` | não | — | slug `^[a-z][a-z0-9_]{1,63}$`; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–120 após aparar |
| `kind` | `TEXT` | não | — | sete valores fechados |
| `enabled` | `BOOLEAN` | não | `true` | desliga sem apagar |
| `requires_city` | `BOOLEAN` | não | `false` | exige cidade/território |
| `description` | `TEXT` | não | `''` | texto de autoria/portal |

Sem carimbos e sem coluna de origem: definição de conteúdo, seed-only.

## Chave primária

`pk_world_event (code)` — chave natural legível, sem identidade (`P-04`). O código é o que a agenda e o
histórico de participação carregam.

## FKs de saída e de entrada

- **Saída:** nenhuma — é tabela raiz de definição, sem FK de saída. `requires_city` é booleano e não
  aponta para `game.city`.
- **Entrada, `CASCADE` (1 FK):** `fk_event_schedule_event` — a agenda é filha da definição e some com
  ela.
- **Entrada, `RESTRICT` (1 FK):** `fk_event_participation_event` (`0028`) — apagar evento com histórico
  de participação falha alto, em vez de levar o histórico junto.

## Unicidade

Só a PK. Dois eventos podem ter o mesmo `name`, e o `kind` se repete entre eventos sem restrição.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_world_event_code` | `code` fora de minúsculas, dígitos e `_`, começando por letra, com 2–64 caracteres |
| `ck_world_event_nome` | `name` vazio, só com espaço, ou acima de 120 caracteres |
| `ck_world_event_kind` | `kind` fora de `kefra`, `noatun`, `city_war`, `rvr`, `tower`, `battle_royale` e `custom` |

`enabled` e `requires_city` não têm `CHECK`: são booleanos, e o efeito de cada um é do runtime.

## Índices

Nenhum além do índice da PK. Não há FK de saída nem coluna de busca secundária; o lado filho da agenda é
indexado pelo próprio único de `event_schedule`.

## Armadilha de leitura

`requires_city` não diz **qual** cidade nem como o território é disputado — é sinalizador, e a regra está
em `game.city`/`game.castle_state` (`0011`/`0028`). `enabled = true` não significa que o evento acontece:
sem linha em `event_schedule`, ele nunca dispara. E `kind = 'custom'` é a válvula para evento que não se
encaixa nas seis famílias, não um tipo com semântica definida.

## Papéis

- **Escreve:** `wyd_migrator`, no seed de quests e eventos (`0027`, `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0026-content-quests-events.md` — arquivo `0026_content_quests_events.sql`.

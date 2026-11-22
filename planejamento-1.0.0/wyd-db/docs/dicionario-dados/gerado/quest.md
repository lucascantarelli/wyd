> Pergunta: "o que `content.quest` define por código e por que a lista de pré-requisitos não tem FK?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.26; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada; o seed `0027` segue `bloqueada`.

## Papel

Cabeça da definição de quest: código estável, nome, tipo, nível mínimo, classe aceita, recarga e lista de
quests anteriores. O conteúdo é carregado no boot e nenhum estado de jogador nasce aqui — quem progride é
`game.character_quest` (`0028`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `code` | `TEXT` | não | — | slug `^[a-z][a-z0-9_]{1,63}$`; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–120 após aparar |
| `kind` | `TEXT` | não | — | `main`, `class`, `daily`, `event` ou `sub` |
| `min_level` | `SMALLINT` | não | `1` | nível mínimo, 1–400 |
| `class_code` | `TEXT` | sim | — | classe aceita; `NULL` = todas |
| `repeatable` | `BOOLEAN` | não | `false` | pode ser repetida |
| `cooldown_seconds` | `INTEGER` | não | `0` | recarga entre repetições, `>= 0` |
| `requires` | `TEXT[]` | não | `'{}'` | códigos de quest anterior, no máximo 32 |
| `description` | `TEXT` | não | `''` | texto de autoria/portal |

Sem carimbos e sem coluna de origem: a proveniência do dataset é `content.content_source` (`0019`), e a
`description` não tem limite de `CHECK` — é texto de autoria, não campo de exibição fechado.

## Chave primária

`pk_quest (code)` — o código é a identidade (`P-04`), sem `id` sintético: é ele que `quest_step`,
`quest_reward` e o progresso do jogador carregam, e um número interno obrigaria tradução em três lugares.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (1 FK):** `fk_quest_classe`: `class_code → content.game_class (slug)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`. `NULL` continua significando "qualquer classe"; a FK só age
  quando há classe.
- **Entrada, `CASCADE` (2 FKs):** `fk_quest_step_quest` e `fk_quest_reward_quest` — passo e recompensa
  são filhos possuídos e somem com a quest.
- **Entrada, `RESTRICT` (1 FK):** `fk_character_quest_quest` (`0028`) — apagar definição com progresso
  registrado falha alto, em vez de levar o progresso junto.
- **Sem FK em `requires`:** array de texto não referencia tabela; a integridade da cadeia é do seed
  (§10 de `migrations/0026-content-quests-events.md`).

## Unicidade

Só a PK: o código é único por definição, e não há segundo único — duas quests podem ter o mesmo nome.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_quest_code` | `code` fora de minúsculas, dígitos e `_`, começando por letra, com 2–64 caracteres |
| `ck_quest_nome` | `name` vazio, só com espaço, ou acima de 120 caracteres |
| `ck_quest_kind` | `kind` fora de `main`, `class`, `daily`, `event` e `sub` |
| `ck_quest_nivel` | `min_level` fora de 1–400 |
| `ck_quest_cooldown` | `cooldown_seconds` negativo |
| `ck_quest_requires` | `code` presente na própria lista, elemento vazio, ou mais de 32 elementos |

O `CHECK` de `requires` não verifica se o código citado existe, nem se há ciclo entre duas quests: ele
impede só a auto-referência e o array sem fim.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_quest_kind_nivel (kind, min_level)` | listar quests de um tipo a partir de um nível |
| `ix_quest_classe (class_code) WHERE class_code IS NOT NULL` | lado filho da FK nullable, fora da PK (`P-15`) |

## Armadilha de leitura

`requires` é lista de códigos **sem FK**: apagar ou renomear a quest citada deixa a referência quebrada em
silêncio, e detectar isso é trabalho do seed, não do banco. `cooldown_seconds` existe mesmo com
`repeatable = false` — o schema não amarra os dois, e o runtime é que decide se a recarga vale. E
`class_code` nulo não é "sem classe definida": é "todas as classes", por desenho.

## Papéis

- **Escreve:** `wyd_migrator`, no seed de quests (`0027`, `bloqueada`, `migrations-checklist.md` §2);
  nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0026-content-quests-events.md` — arquivo `0026_content_quests_events.sql`.

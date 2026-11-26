> Pergunta: "o que `game.character_skill` guarda por skill e por que o índice não tem FK para o catálogo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.11; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Maestria do personagem por skill: uma linha por `skill_index` com o contador de maestria (0–320), sem
grau nem nível de skill como coluna.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `skill_index` | `SMALLINT` | não | — | índice de skill do cliente, 0–247 |
| `mastery` | `SMALLINT` | não | `0` | maestria, 0–320 |
| `learned_at` | `TIMESTAMPTZ` | não | `now()` | primeira maestria na skill |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_character_skill (character_id, skill_index)` — a mesma skill não tem duas linhas no mesmo
personagem; a chave é a posição, não um id próprio.

## FKs de saída e de entrada

- **Saída:** `fk_character_skill_character`: `character_id → game.character (id)`, `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.
- **Sem FK para conteúdo:** `skill_index` não aponta para `content.skill_catalog` (`0015`), porque o
  catálogo é de arquivo posterior e cobre só parte dos 248 índices do cliente.

## Unicidade

Só a PK. O índice de skill é único por personagem, não global.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_skill_indice` | `skill_index` fora de 0–247 |
| `ck_character_skill_maestria` | maestria fora de 0–320 |

O teto por classe é de `content.game_class.mastery_cap`; o `CHECK` é o limite global do cliente.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_character_skill_indice (skill_index)` | lado filho: skills por índice entre personagens |

Trigger `trg_character_skill_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0008_game_skills.md` — arquivo `0008_game_skills.sql`.

> Pergunta: "o que `game.guild_relation` guarda e por que a relação é direcionada?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.13; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Relação declarada de uma guilda com outra — `ally` ou `hostile` —, uma linha por par e por direção.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `guild_id` | `BIGINT` | não | — | PK e FK para `guild` (origem) |
| `target_guild_id` | `BIGINT` | não | — | PK e FK para `guild` (alvo) |
| `kind` | `TEXT` | não | — | `ally` ou `hostile` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | declaração |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_guild_relation (guild_id, target_guild_id)` — um tipo de relação por par direcionado.

## FKs de saída e de entrada

- **Saída:** `fk_guild_relation_guild`: `guild_id → game.guild (id)`, `CASCADE`/`NO ACTION`;
  `fk_guild_relation_target`: `target_guild_id → game.guild (id)`, `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK. A relação é direcionada: a linha de A para B não cria a de B para A.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guild_relation_kind` | tipo fora de `ally`/`hostile` |
| `ck_guild_relation_distinta` | `guild_id = target_guild_id` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_guild_relation_target (target_guild_id)` | lado filho da FK de alvo: relações que apontam para uma guilda |

Trigger `trg_guild_relation_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0010_game_guild.md` — arquivo `0010_game_guild.sql`.

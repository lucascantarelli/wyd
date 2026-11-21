> Pergunta: "o que `game.guild_tower` guarda e por que o dono pode ser nulo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.13; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Estado único da GTorre: guarda só a guilda dona; a zona da torre é conteúdo (`content.guild_zone`, `0019`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | `1` | PK; `CHECK (id = 1)` |
| `owner_guild_id` | `BIGINT` | sim | — | guilda dona; `NULL` = sem dono |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_guild_tower (id)` com `ck_guild_tower_singleton (id = 1)` — linha única, sem identidade: estado
único não é entidade.

## FKs de saída e de entrada

- **Saída:** `fk_guild_tower_owner`: `owner_guild_id → game.guild (id)`, `SET NULL`/`NO ACTION` — guilda
  dissolvida zera o dono; não existe id sentinela.
- **Entrada:** nenhuma.

## Unicidade

Só a PK, que o `CHECK` de singleton restringe a uma linha.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guild_tower_singleton` | qualquer `id` diferente de 1 |

## Índices

Nenhum além da PK. Trigger `trg_guild_tower_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0010-game-guild.md` — arquivo `0010_game_guild.sql`.

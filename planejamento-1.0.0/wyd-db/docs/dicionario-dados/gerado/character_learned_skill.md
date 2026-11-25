> Pergunta: "o que `game.character_learned_skill` guarda e por que a máscara é por categoria?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.11; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Máscara de 8 bits das skills aprendidas, uma linha por categoria (0–3): até quatro linhas por
personagem, não uma máscara única.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `category` | `SMALLINT` | não | — | categoria 0–3 |
| `mask` | `SMALLINT` | não | `0` | 8 bits por categoria, 0–255 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_character_learned_skill (character_id, category)` — uma linha por categoria; a categoria é a chave,
não a skill.

## FKs de saída e de entrada

- **Saída:** `fk_character_learned_skill_character`: `character_id → game.character (id)`,
  `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_learned_skill_categoria` | categoria fora de 0–3 |
| `ck_character_learned_skill_mascara` | máscara fora de 0–255 |

## Índices

Nenhum além da PK. Trigger `trg_character_learned_skill_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0008-game-skills.md` — arquivo `0008-game-skills.sql`.

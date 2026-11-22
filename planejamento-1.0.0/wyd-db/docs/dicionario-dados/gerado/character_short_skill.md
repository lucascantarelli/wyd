> Pergunta: "o que `game.character_short_skill` guarda e por que a barra tem quatro slots fixos?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.10; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Barra de atalhos de skill do personagem: quatro slots, uma linha por personagem.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `slot_1` | `SMALLINT` | não | `0` | 0–255 |
| `slot_2` | `SMALLINT` | não | `0` | 0–255 |
| `slot_3` | `SMALLINT` | não | `0` | 0–255 |
| `slot_4` | `SMALLINT` | não | `0` | 0–255 |

## Chave primária

`pk_character_short_skill (character_id)` — 1:1 com o personagem.

## FKs de saída e de entrada

- **Saída:** `fk_character_short_skill_character`: `character_id → game.character (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_short_skill` | qualquer slot fora de 0–255 |

## Índices

Nenhum além da PK.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.

## Migration

Forma (dona): `migrations/0006-game-character.md` — arquivo `0006_game_character.sql`.

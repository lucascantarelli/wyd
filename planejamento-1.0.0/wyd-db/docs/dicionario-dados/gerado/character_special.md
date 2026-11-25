> Pergunta: "o que `game.character_special` guarda e por que são quatro colunas fixas?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.10; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Os quatro valores especiais do personagem, em colunas fixas — uma linha por personagem, não uma linha
por slot.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `special_1` | `INTEGER` | não | `0` | não negativo |
| `special_2` | `INTEGER` | não | `0` | não negativo |
| `special_3` | `INTEGER` | não | `0` | não negativo |
| `special_4` | `INTEGER` | não | `0` | não negativo |

## Chave primária

`pk_character_special (character_id)` — 1:1 com o personagem.

## FKs de saída e de entrada

- **Saída:** `fk_character_special_character`: `character_id → game.character (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_special` | qualquer especial negativo |

## Índices

Nenhum além da PK.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.

## Migration

Forma (dona): `migrations/0006-game-character.md` — arquivo `0006-game-character.sql`.

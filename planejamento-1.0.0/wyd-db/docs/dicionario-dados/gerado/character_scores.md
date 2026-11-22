> Pergunta: "o que `game.character_scores` guarda e por que base e atual ficam na mesma linha?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.10; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Scores de combate do personagem, com o par `base_*`/`current_*` na mesma linha, sem JSONB.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `base_ac` | `INTEGER` | não | `0` | defesa base |
| `base_damage` | `INTEGER` | não | `0` | dano base |
| `base_attack_run` | `SMALLINT` | não | `0` | 0–255 |
| `base_direction` | `SMALLINT` | não | `0` | 0–255 |
| `base_chaos_rate` | `SMALLINT` | não | `0` | 0–255 |
| `base_str` | `SMALLINT` | não | `0` | não negativo |
| `base_int` | `SMALLINT` | não | `0` | não negativo |
| `base_dex` | `SMALLINT` | não | `0` | não negativo |
| `base_con` | `SMALLINT` | não | `0` | não negativo |
| `current_ac` | `INTEGER` | não | `0` | defesa atual |
| `current_damage` | `INTEGER` | não | `0` | dano atual |
| `current_attack_run` | `SMALLINT` | não | `0` | 0–255 |
| `current_direction` | `SMALLINT` | não | `0` | 0–255 |
| `current_chaos_rate` | `SMALLINT` | não | `0` | 0–255 |
| `current_str` | `SMALLINT` | não | `0` | não negativo |
| `current_int` | `SMALLINT` | não | `0` | não negativo |
| `current_dex` | `SMALLINT` | não | `0` | não negativo |
| `current_con` | `SMALLINT` | não | `0` | não negativo |

## Chave primária

`pk_character_scores (character_id)` — uma linha por personagem.

## FKs de saída e de entrada

- **Saída:** `fk_character_scores_character`: `character_id → game.character (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_scores_atributos` | qualquer atributo (base ou atual) negativo |
| `ck_character_scores_unsigned` | `attack_run`, `direction` ou `chaos_rate` fora de 0–255 |

## Índices

Nenhum além da PK.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.

## Migration

Forma (dona): `migrations/0006-game-character.md` — arquivo `0006_game_character.sql`.

> Pergunta: "o que `game.character_status` guarda de vitalidade e resistência do personagem?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.10; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Estado corrente de HP/MP, PvP, regeneração e resistências — satélite 1:1 de `character`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `hp` | `INTEGER` | não | `1` | 0 até `hp_max` |
| `mp` | `INTEGER` | não | `0` | 0 até `mp_max` |
| `hp_max` | `INTEGER` | não | `1` | teto de HP |
| `mp_max` | `INTEGER` | não | `0` | teto de MP |
| `pvp_points` | `INTEGER` | não | `0` | não negativo |
| `regen_hp` | `SMALLINT` | não | `0` | não negativo |
| `regen_mp` | `SMALLINT` | não | `0` | não negativo |
| `resist_fire` | `SMALLINT` | não | `0` | −128 a 127 |
| `resist_ice` | `SMALLINT` | não | `0` | −128 a 127 |
| `resist_thunder` | `SMALLINT` | não | `0` | −128 a 127 |
| `resist_magic` | `SMALLINT` | não | `0` | −128 a 127 |

## Chave primária

`pk_character_status (character_id)` — uma linha por personagem; o satélite não tem identidade própria.

## FKs de saída e de entrada

- **Saída:** `fk_character_status_character`: `character_id → game.character (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_status_hp` | `hp` negativo ou acima de `hp_max` |
| `ck_character_status_mp` | `mp` negativo ou acima de `mp_max` |
| `ck_character_status_pvp` | pontos de PvP negativos |
| `ck_character_status_regen` | regeneração negativa |
| `ck_character_status_resist` | resistência fora de −128–127 |

## Índices

Nenhum além da PK: o acesso é sempre por personagem.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.

## Migration

Forma (dona): `migrations/0006-game-character.md` — arquivo `0006_game_character.sql`.

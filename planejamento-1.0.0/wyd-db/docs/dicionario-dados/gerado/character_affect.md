> Pergunta: "o que `game.character_affect` guarda de um efeito ativo e por que ele é apagado ao expirar?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.11; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Afeto/estado ativo do personagem — tipo, valor, nível e ticks restantes —, uma linha por tipo. É o
único dado de progressão que morre em runtime: efeito expirado é removido, não marcado.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `affect_type` | `SMALLINT` | não | — | tipo numérico do afeto, `>= 0` |
| `value` | `SMALLINT` | não | — | valor do efeito; sem `CHECK` próprio |
| `level` | `INTEGER` | não | `0` | nível do efeito, `>= 0` |
| `remaining_ticks` | `BIGINT` | não | `0` | ticks restantes, `>= 0` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | aplicação do efeito |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_character_affect (character_id, affect_type)` — um efeito de cada tipo por personagem; reaplicar
atualiza a linha, não cria outra.

## FKs de saída e de entrada

- **Saída:** `fk_character_affect_character`: `character_id → game.character (id)`,
  `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_affect_tipo` | `affect_type` negativo |
| `ck_character_affect_nivel` | `level` negativo |
| `ck_character_affect_restante` | `remaining_ticks` negativo |

`value` não tem faixa no schema; o limite de 32 afetos simultâneos é regra do core, não `CHECK`.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_character_affect_tipo (affect_type)` | efeitos ativos por tipo, no tick |

Trigger `trg_character_affect_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0008_game_skills.md` — arquivo `0008_game_skills.sql`.

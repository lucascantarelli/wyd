> Pergunta: "o que `game.refine_pity` guarda e por que a piedade morre junto com o item?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.12; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Contador de piedade de refino por posição do personagem (0–20), sem histórico de tentativa: é contador,
não ledger.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e parte da FK para `character_item` |
| `slot` | `SMALLINT` | não | — | PK e posição do item no personagem |
| `pity` | `SMALLINT` | não | `0` | contador 0–20 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_refine_pity (character_id, slot)` — uma piedade por posição; a chave é a posição do item.

## FKs de saída e de entrada

- **Saída:** `fk_refine_pity_item`: `(character_id, slot) → game.character_item (character_id, slot)`,
  `CASCADE`/`NO ACTION` — é a única FK composta do lote e amarra a piedade à existência do item.
- **Entrada:** nenhuma.

Não há FK direta para `character`: ela chega pelo item.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_refine_pity_valor` | `pity` fora de 0–20 |

## Índices

Nenhum além da PK: `(character_id, slot)` já cobre a leitura pela chave do item. Trigger
`trg_refine_pity_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0009-game-inventory.md` — arquivo `0009_game_inventory.sql`.

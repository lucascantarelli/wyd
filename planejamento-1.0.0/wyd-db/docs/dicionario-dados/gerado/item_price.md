> Pergunta: "o que `game.item_price` guarda e por que `npc_id` é obrigatório na chave?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Preço de compra e de venda de um item em um mercador específico, um par por linha. Preço de catálogo
(genérico) fica em `content`, não aqui.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `item_index` | `INTEGER` | não | — | PK; índice de item do cliente |
| `npc_id` | `INTEGER` | não | — | PK; mercador que pratica o preço |
| `buy_price` | `BIGINT` | não | `0` | preço de compra, `>= 0` |
| `sell_price` | `BIGINT` | não | `0` | preço de venda, `>= 0` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_item_price (item_index, npc_id)` — um preço por item e mercador; `npc_id` é obrigatório, não
anulável, porque preço sem mercador não tem dono.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** nenhuma.
- **Sem FK para conteúdo:** nem `item_index` (`content.item_catalog`, `0014`) nem `npc_id`
  (`content.npc_definition`, `0019`) apontam para catálogo, porque os dois arquivos são posteriores.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_item_price_compra` | preço de compra negativo |
| `ck_item_price_venda` | preço de venda negativo |

Não há `CHECK` de `sell_price <= buy_price`: a relação entre os dois é regra de economia, não de schema.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_item_price_npc (npc_id)` | catálogo de preços de um mercador |

Trigger `trg_item_price_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).
- **Administra:** `wyd_admin` — `SELECT, UPDATE` concedidos por `0032`; sem `INSERT`/`DELETE`.

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011_game_economy.sql`.

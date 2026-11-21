> Pergunta: "o que `portal.donate_shop_item` guarda e o que o preço em créditos exige?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Oferta da loja cash do portal: qual item, por quantos créditos e por quanto tempo. É catálogo de oferta,
não entrega — a entrega é `ops.delivery_queue` (`0012`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `item_index` | `INTEGER` | não | — | índice de item do cliente; sem FK |
| `price_credits` | `INTEGER` | não | — | preço em créditos, `> 0` |
| `duration_days` | `SMALLINT` | sim | — | duração em dias; nulo = permanente |
| `enabled` | `BOOLEAN` | não | `true` | oferta ativa |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da oferta |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_donate_shop_item (id)` — cada oferta é uma linha; o item pode aparecer em mais de uma oferta.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** nenhuma.
- **Sem FK:** `item_index` é de `content.item_catalog` (`0014`, posterior a `0011`).

## Unicidade

Só a PK: não há regra de uma oferta por item.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_donate_shop_item_preco` | preço em créditos menor ou igual a zero |
| `ck_donate_shop_item_duracao` | duração preenchida e menor ou igual a zero |

## Índices

Nenhum além da PK. Trigger `trg_donate_shop_item_set_updated_at`.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`, `wyd_read`.
- **Não lê:** `wyd_game` (sem `USAGE` em `portal`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011_game_economy.sql`.

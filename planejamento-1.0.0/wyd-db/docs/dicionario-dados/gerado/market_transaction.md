> Pergunta: "o que `portal.market_transaction` guarda de uma liquidação e como o dinheiro fecha?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Liquidação de **um** anúncio: o que o comprador pagou, a taxa da casa, o líquido do vendedor, a taxa
congelada, a custódia liquidada e a chave de idempotência do fechamento.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `listing_id` | `BIGINT` | não | — | anúncio liquidado; FK `RESTRICT`, único |
| `buyer_account_id` | `BIGINT` | não | — | conta compradora; sem FK |
| `seller_account_id` | `BIGINT` | não | — | conta vendedora; sem FK |
| `price_gross` | `BIGINT` | não | — | valor cheio, `> 0` |
| `fee_amount` | `BIGINT` | não | — | taxa da casa, `>= 0` |
| `net_amount` | `BIGINT` | não | — | líquido do vendedor, `>= 0` |
| `fee_bps` | `INTEGER` | não | — | taxa congelada na liquidação |
| `escrow_ref` | `BIGINT` | sim | — | custódia liquidada; sem FK (é de `game`) |
| `settled_at` | `TIMESTAMPTZ` | não | — | instante da liquidação |
| `idempotency_key` | `TEXT` | não | — | chave do BFF; única (`M17`) |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |

## Chave primária

`pk_market_transaction (id)` — identidade sintética; o fato é a liquidação, e ela é única por anúncio pelo
único próprio.

## FKs de saída e de entrada

- **Saída (1):** `fk_market_transaction_listing`: `listing_id → portal.market_listing (id)`,
  `ON DELETE RESTRICT`, `ON UPDATE NO ACTION`. `escrow_ref` guarda o número de `game.market_escrow` sem FK
  (`P-17`).
- **Entrada:** `market_review.transaction_id` → `RESTRICT` (único, uma avaliação por transação) e
  `market_dispute.transaction_id` → `RESTRICT`.

## Unicidade

- `uq_market_transaction_listing UNIQUE (listing_id)` — **um** anúncio liquida uma vez; leilão relistado é
  anúncio novo.
- `uq_market_transaction_idempotency UNIQUE (idempotency_key)` — repetir a chave devolve a mesma liquidação
  em vez de pagar duas vezes.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_transaction_partes` | comprador igual ao vendedor |
| `ck_market_transaction_soma` | `fee_amount + net_amount <> price_gross` |
| `ck_market_transaction_valores` | `price_gross <= 0`, `fee_amount` ou `net_amount` negativo |

`ck_market_transaction_soma` é conservação de dinheiro no schema: o que sai do comprador é exatamente taxa
mais líquido.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_market_transaction_comprador (buyer_account_id, settled_at DESC)` | histórico de compras da conta |
| `ix_market_transaction_vendedor (seller_account_id, settled_at DESC)` | histórico de vendas da conta |

## Armadilha de leitura

`fee_bps` aqui é a taxa **efetivamente aplicada**, não necessariamente a de `market_listing.fee_bps`:
recalcular `fee_amount` a partir do bps atual de `market_fee_rule` dá outro número e não corrige nada.
`escrow_ref` pode ser nulo (liquidação registrada antes de a custódia fechar) — nulo não é "item não
entregue", é vínculo ainda não gravado. `settled_at` é o instante do negócio e pode diferir de `created_at`,
que é o da linha. E `buyer_account_id = seller_account_id` é impossível por `CHECK`, então autocompra não
existe nem para teste.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`;
  a linha nasce fechada e não muda.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034-portal-market.sql`; DDL em
`ddl-confirmado.md` §5.34.

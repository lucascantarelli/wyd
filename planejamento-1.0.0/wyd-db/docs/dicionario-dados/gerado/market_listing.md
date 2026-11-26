> Pergunta: "o que `portal.market_listing` guarda de um anúncio — o retrato da instância, o preço e a custódia?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Anúncio de **item real** no mercado: venda direta ou leilão, com o **retrato** da instância (refino,
ancient, gema, flags), o preço, a taxa congelada, o estado editorial e a referência da custódia que tirou o
item do inventário. O item em si não está aqui: está em `game.market_escrow`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `seller_account_id` | `BIGINT` | não | — | conta vendedora; sem FK |
| `character_id` | `BIGINT` | não | — | personagem de origem; sem FK (é de `game`) |
| `item_index` | `INTEGER` | não | — | item do catálogo; sem FK (`0014` é seed) |
| `item_kind_slug` | `TEXT` | sim | — | categoria copiada para filtro, 1–64; sem FK |
| `qty` | `INTEGER` | não | `1` | quantidade, `>= 1` |
| `refine` | `SMALLINT` | não | `0` | refino retratado, 0–15 |
| `ancient` | `BOOLEAN` | não | `false` | retrato de `character_item` (`0009`) |
| `gem_effect` | `SMALLINT` | não | `0` | efeito de gema retratado |
| `gem_value` | `SMALLINT` | não | `0` | valor de gema retratado |
| `flags` | `INTEGER` | não | `0` | bits herdados do legado |
| `kind` | `TEXT` | não | — | `venda` ou `leilao` |
| `price` | `BIGINT` | não | — | preço de saída, `> 0` |
| `buyout_price` | `BIGINT` | sim | — | compra imediata do leilão, `>= price` |
| `fee_bps` | `INTEGER` | não | — | taxa congelada na publicação, 0–10000 |
| `ends_at` | `TIMESTAMPTZ` | sim | — | prazo do leilão; nulo em venda direta |
| `status` | `TEXT` | não | `'rascunho'` | estado editorial |
| `escrow_ref` | `BIGINT` | sim | — | `game.market_escrow.id`; sem FK |
| `current_bid_id` | `BIGINT` | sim | — | lance corrente; sem FK (referência circular) |
| `bid_count` | `INTEGER` | não | `0` | contador de lances |
| `view_count` | `INTEGER` | não | `0` | contador de visitas |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_market_listing (id)` — identidade sintética; o anúncio é um fato novo a cada relistagem.

## FKs de saída e de entrada

- **Saída:** nenhuma. `character_id` e `escrow_ref` são de `game` (`P-17`), `item_kind_slug` é cópia sem FK
  para `content.item_catalog` (`0014`), e `current_bid_id` **não** tem FK porque `market_bid` já aponta para
  o anúncio — a FK de volta fecharia um ciclo entre as duas tabelas.
- **Entrada:** `market_bid.listing_id` → `RESTRICT` e `market_transaction.listing_id` → `RESTRICT`
  (único): apagar anúncio com lance ou liquidação falha alto.

## Unicidade

Só a PK. O DDL não declara único natural no anúncio — dois anúncios do mesmo item pelo mesmo vendedor são
permitidos, e limitar isso é regra de produto no BFF, não constraint.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_listing_tipo` | `kind` fora de `venda` e `leilao` |
| `ck_market_listing_estado` | `status` fora de `rascunho`, `ativo`, `vendido`, `expirado`, `cancelado` e `disputa` |
| `ck_market_listing_preco` | `price <= 0` |
| `ck_market_listing_compra_ja` | `buyout_price` menor que `price` |
| `ck_market_listing_prazo` | leilão sem `ends_at`, ou venda direta com `ends_at` |
| `ck_market_listing_custodia` | `status = 'ativo'` com `escrow_ref` nulo |
| `ck_market_listing_qtd` | `qty < 1` |
| `ck_market_listing_refino` | `refine` fora de 0–15 |
| `ck_market_listing_taxa` | `fee_bps` fora de 0–10000 |
| `ck_market_listing_popularidade` | `bid_count` ou `view_count` negativo |
| `ck_market_listing_categoria` | `item_kind_slug` vazio ou acima de 64 caracteres |

`ck_market_listing_prazo` amarra tipo e prazo nas duas direções (`(kind = 'leilao') = (ends_at IS NOT
NULL)`): não existe leilão sem fim nem venda direta com prazo.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_market_listing_item_status_preco (item_index, status, price)` | busca do mercado por item |
| `ix_market_listing_kind_status_preco (item_kind_slug, status, price)` | navegação por categoria |
| `ix_market_listing_status_popularidade (status, bid_count DESC, view_count DESC)` | vitrine ordenada por movimento |
| `ix_market_listing_status_prazo (status, ends_at) WHERE status = 'ativo'` | fechamento de leilão: só anúncio ativo tem prazo a vencer |

Os três primeiros são totais; o quarto é **parcial** e cobre só `ativo`, porque `ends_at` de encerrado não
interessa ao job de fechamento. `trg_market_listing_set_updated_at` mantém `updated_at`.

## Armadilha de leitura

`status = 'ativo'` **exige** `escrow_ref`, mas `escrow_ref` preenchido não é prova de que o item ainda está
retido: `game.market_escrow.state` é quem diz. `current_bid_id` é ponteiro de conveniência e pode ficar
para trás se o lance for superado — a verdade do maior lance é `market_bid` com `status = 'ativo'`; não
trate o ponteiro como vencedor. O retrato (`refine`, `ancient`, `gem_*`, `flags`) é **cópia**, e comparar
com `game.market_escrow` é conferência de custódia, não redundância. `price` é o preço de saída; em leilão
ele é o piso, e o valor efetivo só existe em `market_transaction`. `buyout_price` nulo não significa "sem
compra imediata" em venda direta — significa que a coluna não se aplica.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`,
  pelo BFF; `wyd_game` não alcança `portal`.
- **Lê:** `wyd_portal` e `wyd_read` (herdado).

## Migration

Forma (dona): `migrations/0034_portal_market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

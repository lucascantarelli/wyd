> Pergunta: "o que `portal.market_bid` guarda de um lance e por que só existe um ativo por comprador e anúncio?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Lance de leilão: valor, comprador, estado e o lançamento de crédito que **retém** o valor enquanto o lance
está de pé. Subir lance é `UPDATE` do lance ativo, não linha nova.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `listing_id` | `BIGINT` | não | — | anúncio; FK `RESTRICT` |
| `bidder_account_id` | `BIGINT` | não | — | conta compradora; sem FK |
| `amount` | `BIGINT` | não | — | valor do lance, `> 0` |
| `status` | `TEXT` | não | `'ativo'` | estado do lance |
| `hold_ledger_id` | `BIGINT` | sim | — | retenção no extrato; FK `RESTRICT` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |

## Chave primária

`pk_market_bid (id)` — identidade sintética; o histórico de lances de um anúncio é o próprio conjunto de
linhas.

## FKs de saída e de entrada

- **Saída (2):** `fk_market_bid_listing`: `listing_id → portal.market_listing (id)`, `ON DELETE RESTRICT`,
  `ON UPDATE NO ACTION`; `fk_market_bid_ledger`: `hold_ledger_id → portal.market_ledger (id)`,
  `ON DELETE RESTRICT`, `ON UPDATE NO ACTION`. Um lance não apaga o anúncio, e um lançamento de retenção
  citado por lance não é apagado.
- **Entrada:** nenhuma. `market_listing.current_bid_id` guarda o número **sem FK**, para não fechar ciclo.

## Unicidade

`ux_market_bid_comprador_ativo UNIQUE (listing_id, bidder_account_id) WHERE status = 'ativo'` — **um** lance
ativo por comprador e anúncio. É parcial: superado, vencedor, perdido e cancelado não ocupam a vaga, então
o mesmo comprador pode dar outro lance depois de ser superado — e o `UPDATE` do ativo é o caminho normal.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_bid_valor` | `amount <= 0` |
| `ck_market_bid_estado` | `status` fora de `ativo`, `superado`, `vencedor`, `perdido` e `cancelado` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_market_bid_comprador_ativo (listing_id, bidder_account_id) WHERE status = 'ativo'` | garante um ativo por comprador e anúncio |
| `ix_market_bid_listing_criado (listing_id, created_at DESC)` | histórico do anúncio, do mais recente para o mais antigo |
| `ix_market_bid_ledger (hold_ledger_id) WHERE hold_ledger_id IS NOT NULL` | lado filho da FK, só nas linhas com retenção |

## Armadilha de leitura

`amount` é o valor **retido**, não o saldo do comprador: o dinheiro está em `market_ledger` na linha de
`hold_ledger_id`, e devolver ou capturar a retenção é lançamento novo (`devolucao_lance`, `compra`). O lance
ativo é **um** por comprador, mas o vencedor não é o de maior `amount` por `MAX` cego: é o ativo de maior
valor no instante do fechamento, e `status = 'vencedor'` é quem carrega essa decisão. `hold_ledger_id`
nulo é lance sem retenção registrada — estado de trânsito, não lance grátis. E `status = 'perdido'` existe
separado de `superado`: o primeiro perdeu o leilão, o segundo foi ultrapassado durante ele.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`;
  `wyd_game` não alcança `portal`.
- **Lê:** `wyd_portal` e `wyd_read` (herdado).

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

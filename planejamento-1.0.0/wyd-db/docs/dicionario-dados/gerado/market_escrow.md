> Pergunta: "o que `game.market_escrow` guarda do item em custódia e por que quem escreve é o executor, não o jogo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

**Custódia do item** fora do inventário: o retrato do item retirado de `game.character_item` no momento do
anúncio, o personagem de origem, o estado (`retido`, `entregue`, `devolvido`), as referências do anúncio e
da transação e a chave de idempotência. É a única tabela de `game` no lote `0034`, porque é o único ponto
em que o item deixa de existir no inventário.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `character_id` | `BIGINT` | não | — | personagem de origem; FK `RESTRICT` |
| `item_index` | `INTEGER` | não | — | item do catálogo; sem FK (`0014` é seed) |
| `qty` | `INTEGER` | não | — | quantidade, `>= 1` |
| `refine` | `SMALLINT` | não | — | refino, 0–15 |
| `ancient` | `BOOLEAN` | não | — | retrato de `character_item` (`0009`) |
| `gem_effect` | `SMALLINT` | não | — | efeito de gema |
| `gem_value` | `SMALLINT` | não | — | valor de gema |
| `flags` | `INTEGER` | não | — | bits herdados do legado |
| `state` | `TEXT` | não | `'retido'` | estado da custódia |
| `listing_ref` | `BIGINT` | sim | — | `portal.market_listing.id`; sem FK |
| `transaction_ref` | `BIGINT` | sim | — | `portal.market_transaction.id`; sem FK |
| `idempotency_key` | `TEXT` | não | — | chave do executor; única |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `closed_at` | `TIMESTAMPTZ` | sim | — | fechamento; nulo só em `retido` |

## Chave primária

`pk_market_escrow (id)` — identidade sintética; o mesmo item pode ser custodiado mais de uma vez ao longo da
vida, em linhas diferentes.

## FKs de saída e de entrada

- **Saída (1):** `fk_market_escrow_character`: `character_id → game.character (id)`, `ON DELETE RESTRICT`,
  `ON UPDATE NO ACTION` — custódia não perde o dono, e apagar personagem com item retido falha alto.
  `listing_ref` e `transaction_ref` guardam números de `portal` **sem FK**: `game` não tem `USAGE` em
  `portal` (`P-17`), e a fronteira é do executor.
- **Entrada:** nenhuma. `portal.market_listing.escrow_ref` guarda o número desta linha sem FK, pelo mesmo
  motivo.

## Unicidade

`uq_market_escrow_idempotency UNIQUE (idempotency_key)` — repetir a chave do executor não retira o item duas
vezes do inventário; o segundo `INSERT` falha em `23505`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_escrow_estado` | `state` fora de `retido`, `entregue` e `devolvido` |
| `ck_market_escrow_qtd` | `qty < 1` |
| `ck_market_escrow_refino` | `refine` fora de 0–15 |
| `ck_market_escrow_fechamento` | `retido` com `closed_at`, ou estado fechado sem `closed_at` |

`ck_market_escrow_fechamento` faz de `retido` o único estado aberto: entregue e devolvido têm carimbo, e
retido não tem.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_market_escrow_estado (state, created_at)` | custódia aberta (`retido`) e conferência por estado, da mais antiga para a mais nova |
| `ix_market_escrow_character (character_id)` | lado filho da FK: custódias de um personagem |

## Armadilha de leitura

`state = 'retido'` é o item **fora** do inventário; `entregue` e `devolvido` são o fim, e a linha **não** é
apagada — `DELETE` não é concedido nem ao `wyd_admin` neste lote. O retrato (`refine`, `ancient`, `gem_*`,
`flags`) é cópia do momento da retenção: comparar com `character_item` depois é conferência, não prova de
divergência. `listing_ref`/`transaction_ref` nulos são estado de trânsito (itens retidos antes de o anúncio
ou a liquidação gravar o vínculo), não custódia órfã. E `item_index` sem FK para `content.item_catalog` é
decisão do lote: o catálogo é seed de `0014`.

## Papéis

- **Escreve:** `wyd_admin` — `GRANT SELECT, INSERT, UPDATE` dado pelo próprio `0034`; **sem `DELETE`**. É a
  diferença declarada do lote: quem move item é o executor (`LAC-POR-01`), não o jogo.
- **Não escreve:** `wyd_game` tem `INSERT`, `UPDATE` e `DELETE` **revogados** pelo `0034`; sobra o `SELECT`
  herdado do default privilege de `game` (`0001_bootstrap.md` §7) — o jogo lê a custódia, não a move.
- **Lê:** `wyd_admin`, `wyd_game` e `wyd_read`.

## Migration

Forma (dona): `migrations/0034_portal_market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

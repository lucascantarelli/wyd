> Pergunta: "o que `portal.market_dispute` guarda de uma disputa e por que decidir exige autor e desfecho?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Disputa de uma transação: motivo, descrição, estado, e — quando decidida — desfecho e autor da decisão. Uma
disputa **viva** por transação; decidida, outra pode abrir.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `transaction_id` | `BIGINT` | não | — | transação disputada; FK `RESTRICT` |
| `opened_by_account_id` | `BIGINT` | não | — | conta que abriu; sem FK |
| `reason_code` | `TEXT` | não | — | motivo em lista fechada |
| `description` | `TEXT` | não | — | relato, 1–2000 caracteres |
| `state` | `TEXT` | não | `'aberta'` | estado do fluxo |
| `resolution` | `TEXT` | sim | — | desfecho escrito |
| `decided_by_person_id` | `BIGINT` | sim | — | pessoa que decidiu; sem FK |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_market_dispute (id)` — identidade sintética; várias disputas da mesma transação são possíveis ao longo
do tempo, então a transação não serve de chave.

## FKs de saída e de entrada

- **Saída (1):** `fk_market_dispute_transaction`: `transaction_id → portal.market_transaction (id)`,
  `ON DELETE RESTRICT`, `ON UPDATE NO ACTION`. `opened_by_account_id` é número de `game.account` e
  `decided_by_person_id` é número de `portal.person` **sem FK declarada** — o autor da decisão é rastro, e
  fechar pessoa (`A12`) não pode travar a disputa.
- **Entrada:** nenhuma.

## Unicidade

`ux_market_dispute_aberta UNIQUE (transaction_id) WHERE state IN ('aberta', 'em_analise')` — **uma** disputa
viva por transação. É parcial de propósito: decidida ou cancelada, a vaga libera, e a mesma transação pode
ser disputada de novo com histórico preservado.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_dispute_motivo` | `reason_code` fora de `nao_recebido`, `divergente`, `cobranca_indevida` e `outro` |
| `ck_market_dispute_estado` | `state` fora de `aberta`, `em_analise`, `resolvida_comprador`, `resolvida_vendedor` e `cancelada` |
| `ck_market_dispute_desfecho` | resolução sem `resolution` ou sem `decided_by_person_id`, ou não-resolução com algum deles |
| `ck_market_dispute_descricao` | `description` vazia ou acima de 2000 caracteres |

`ck_market_dispute_desfecho` é equivalência nas duas direções: as duas resoluções **exigem** desfecho e
autor, e `aberta`, `em_analise` e `cancelada` **não podem** tê-los.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_market_dispute_aberta (transaction_id) WHERE state IN ('aberta', 'em_analise')` | garante uma disputa viva por transação |
| `ix_market_dispute_estado_aberta (state, created_at) WHERE state IN ('aberta', 'em_analise')` | fila de moderação, da mais antiga para a mais nova |
| `ix_market_dispute_transaction (transaction_id)` | lado filho da FK: histórico da transação |

`trg_market_dispute_set_updated_at` mantém `updated_at` no `UPDATE`.

## Armadilha de leitura

`state = 'cancelada'` **não** é resolução: a `CHECK` proíbe `resolution` e autor nesse estado, então
"cancelada com desfecho" não existe no schema. `resolution` é texto livre, não código — decidir
"comprador" ou "vendedor" é `state`, e o texto é o porquê. `decided_by_person_id` sem FK pode apontar para
pessoa já anonimizada: o id continua sendo a prova de qual staff decidiu. E a fila de moderação é a
**parcial** (`aberta`/`em_analise`), não `state IN (…)` sobre a tabela toda — o índice só existe para esses
dois estados.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`,
  pelo BFF e pela moderação.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

> Pergunta: "o que `portal.market_ledger` guarda por lançamento e por que o extrato é append-only?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Extrato do crédito do mercado: cada lançamento que muda `market_wallet.balance`, com motivo, valor, saldo
depois e chave de idempotência. É **append-only por permissão**, como `game.account_ledger` (`M16`): o
saldo é derivado dele, e reescrever um lançamento reescreveria história.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `account_id` | `BIGINT` | não | — | conta do extrato; sem FK |
| `kind` | `TEXT` | não | — | motivo em lista fechada |
| `amount` | `BIGINT` | não | — | variação com sinal; nunca zero |
| `balance_after` | `BIGINT` | não | — | saldo depois do lançamento; nunca negativo |
| `ref_kind` | `TEXT` | sim | — | tipo do documento de origem |
| `ref_id` | `BIGINT` | sim | — | id do documento de origem |
| `idempotency_key` | `TEXT` | não | — | chave do BFF, única na tabela (`M17`) |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo do lançamento |

## Chave primária

`pk_market_ledger (id)` — identidade sintética; o lançamento é um fato, não um estado a ser localizado por
chave natural.

## FKs de saída e de entrada

- **Saída:** nenhuma. `account_id` é número de `game.account` (sem `USAGE` cruzado, `P-17`) e
  `ref_kind`/`ref_id` são **referência polimórfica sem FK**: `anuncio`, `transacao`, `disputa` e `recarga`
  apontam para tabelas diferentes, e a FK única não existe para alvos múltiplos.
- **Entrada:** `market_bid.hold_ledger_id` → `market_ledger (id)`, `RESTRICT`.

## Unicidade

`uq_market_ledger_idempotency UNIQUE (idempotency_key)` — a chave do BFF é única **na tabela toda**, não
por conta: repetir a chave é replay do mesmo lançamento, e um segundo `INSERT` falha em `23505` em vez de
duplicar dinheiro.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_ledger_tipo` | `kind` fora de `recarga`, `compra`, `venda`, `taxa`, `retencao_lance`, `devolucao_lance` e `estorno` |
| `ck_market_ledger_valor` | `amount = 0` |
| `ck_market_ledger_saldo` | `balance_after` negativo |
| `ck_market_ledger_ref` | `ref_kind` sem `ref_id` ou `ref_id` sem `ref_kind` |
| `ck_market_ledger_ref_tipo` | `ref_kind` fora de `anuncio`, `transacao`, `disputa` e `recarga` |

`amount` **aceita negativo** (compra, taxa e retenção de lance saem); o que o `CHECK` barra é o lançamento
nulo, não o débito.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_market_ledger_conta_criado (account_id, created_at DESC)` | extrato da conta, do mais recente para o mais antigo |

## Armadilha de leitura

`balance_after` é o saldo **no instante** do lançamento, não o atual: ler o último por `created_at` dá o
saldo, e é a mesma conta que `market_wallet.balance` deve mostrar — divergência entre os dois é defeito de
escrita, não de leitura. `ref_kind`/`ref_id` podem ser nulos (recarga e estorno internos), então juntar com
`market_transaction` exige `JOIN` condicional. E o extrato **não se corrige**: `REVOKE UPDATE, DELETE …
FROM wyd_portal` no próprio `0034` deixa `wyd_portal` só com `INSERT` e `SELECT`; erro se conserta com
lançamento de `estorno`.

## Papéis

- **Escreve:** `wyd_portal` — `INSERT` e `SELECT` herdados do default privilege, **com `UPDATE` e `DELETE`
  revogados** pelo `0034`; sem escrita para `wyd_game`.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034_portal_market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

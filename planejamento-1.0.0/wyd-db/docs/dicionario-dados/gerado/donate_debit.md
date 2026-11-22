> Pergunta: "o que `portal.donate_debit` registra e por que o motivo é fechado em dois valores?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Extrato de **débito** de cash no portal: cada gasto com motivo (`portal` ou `shop_purchase`) e chave de
idempotência. Como o crédito, é extrato, não saldo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `idempotency_key` | `UUID` | não | — | chave da operação; unique |
| `account_id` | `BIGINT` | não | — | conta debitada; sem FK |
| `credits` | `BIGINT` | não | — | créditos debitados, `> 0` |
| `reason` | `TEXT` | não | `'portal'` | `portal` ou `shop_purchase` |
| `created_by` | `BIGINT` | sim | — | pessoa/staff que originou; sem FK |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do débito |

## Chave primária

`pk_donate_debit (id)` — cada débito é uma linha; a operação é identificada pela chave de idempotência.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** nenhuma.
- **Sem FK para `game.account`:** `account_id` e `created_by` são números por decisão de fronteira
  (`P-17`, `A19`); o portal não tem `USAGE` em `game`.

## Unicidade

`uq_donate_debit_idem (idempotency_key)` — unique **total**: repetir a chave é `23505`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_donate_debit_valor` | débito menor ou igual a zero |
| `ck_donate_debit_motivo` | motivo fora de `portal`/`shop_purchase` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `uq_donate_debit_idem (idempotency_key)` | detectar repetição da mesma operação |
| `ix_donate_debit_conta (account_id, created_at DESC)` | extrato recente da conta |

Sem `updated_at` nem trigger: o carimbo é só `created_at`.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`, `wyd_read`.
- **Não lê:** `wyd_game` (sem `USAGE` em `portal`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011_game_economy.sql`.

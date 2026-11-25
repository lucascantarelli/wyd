> Pergunta: "o que `portal.donate_credit` registra e como a chave de idempotência evita crédito dobrado?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Extrato de **crédito** de cash no portal: uma linha por operação creditada, com chave de idempotência.
Não é saldo — o saldo é derivado do extrato.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `idempotency_key` | `UUID` | não | — | chave da operação; unique |
| `account_id` | `BIGINT` | não | — | conta creditada; sem FK |
| `credits` | `BIGINT` | não | — | créditos creditados, `> 0` |
| `created_by` | `BIGINT` | sim | — | pessoa/staff que originou; sem FK |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do crédito |

## Chave primária

`pk_donate_credit (id)` — cada crédito é uma linha; a operação é identificada pela chave de idempotência.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** nenhuma.
- **Sem FK para `game.account`:** `account_id` é número porque `wyd_portal` não tem `USAGE` em `game`
  (`P-17`, `A19`); a conta é validada na borda. `created_by` também não tem FK.

## Unicidade

`uq_donate_credit_idem (idempotency_key)` — unique **total**: repetir a chave é `23505`, e o replay de
24 h é política do BFF, não coluna (`M17`).

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_donate_credit_valor` | crédito menor ou igual a zero |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `uq_donate_credit_idem (idempotency_key)` | detectar repetição da mesma operação |

Sem `updated_at` nem trigger: o carimbo é só `created_at`.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`, `wyd_read`.
- **Não lê:** `wyd_game` (sem `USAGE` em `portal`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011-game-economy.sql`.

> Pergunta: "o que `game.account_ledger` registra e como ele se relaciona com o saldo da carteira?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.2; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Livro de `coin` (`M16`): toda mudança de carteira deixa uma linha com o saldo resultante.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | conta dona |
| `delta` | `BIGINT` | não | — | variação; nunca zero |
| `balance_after` | `BIGINT` | não | — | saldo no instante do lançamento |
| `reason` | `TEXT` | não | — | 1–64 caracteres |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do lançamento |

`account_wallet.coin` é o saldo corrente; `balance_after` é a fotografia de cada lançamento.

## Chave primária

`pk_account_ledger (id)` — cada lançamento é uma linha; não há chave natural.

## FKs de saída e de entrada

- **Saída:** `fk_account_ledger_account`: `account_id → game.account (id)`, `RESTRICT`.
- **Entrada:** nenhuma.

## Unicidade

Nenhuma além da PK: o mesmo motivo pode lançar várias vezes.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_account_ledger_delta` | `delta` igual a zero |
| `ck_account_ledger_balance` | saldo resultante negativo ou acima de `2147483647` |
| `ck_account_ledger_reason` | motivo vazio ou acima de 64 caracteres |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_account_ledger_conta (account_id, created_at DESC)` | extrato da conta |

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal`.
- **Administra:** `wyd_admin` — `SELECT, INSERT` concedidos por `0032`; o extrato não se corrige, então
  `UPDATE` e `DELETE` ficam fora.

## Migration

Forma (dona): `migrations/0005_game_account.md` — arquivo `0005_game_account.sql`.

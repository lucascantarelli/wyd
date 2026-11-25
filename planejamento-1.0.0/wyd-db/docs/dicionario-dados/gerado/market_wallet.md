> Pergunta: "o que `portal.market_wallet` guarda do crédito do mercado e por que o saldo não tem FK para a conta?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Saldo de **crédito do mercado** (`portal_credit`), uma linha por conta, separado do cash de
`donate_credit`/`donate_debit`. É o único lugar em que o saldo do mercado é lido direto; o histórico é
`market_ledger`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | conta dona; PK e sem FK |
| `currency` | `TEXT` | não | `'portal_credit'` | uma moeda só no `1.0.0` |
| `balance` | `BIGINT` | não | `0` | saldo corrente; nunca negativo |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_market_wallet (account_id)` — a conta **é** a chave: uma carteira por conta, sem `id` artificial. Isso
torna a criação idempotente por natureza (`INSERT … ON CONFLICT DO NOTHING`).

## FKs de saída e de entrada

- **Saída:** nenhuma. `account_id` guarda o número da conta de `game.account`, e `portal` não tem `USAGE`
  em `game` (`P-17`); a integridade é do BFF, como em `donate_credit`/`donate_debit`.
- **Entrada:** nenhuma tabela aponta para a carteira; o extrato (`market_ledger`) referencia a conta, não a
  carteira.

## Unicidade

Só a PK. Não há segundo único: a carteira é a conta.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_wallet_moeda` | `currency` fora de `portal_credit` |
| `ck_market_wallet_saldo` | `balance` negativo |

## Índices

Nenhum além da PK. A carteira é lida pela própria chave, e a varredura por conta é o extrato
(`ix_market_ledger_conta_criado`). `trg_market_wallet_set_updated_at` mantém `updated_at` no `UPDATE`, via
`ops.fn_set_updated_at()` (`0001`).

## Armadilha de leitura

`balance` é **cache do extrato**, não a soma de `donate_credit` menos `donate_debit`: são dois créditos
diferentes (o cash do portal e o crédito do mercado), e converter um no outro é operação do BFF, não do
schema. `balance = 0` não prova que a conta nunca operou — a linha nasce na primeira operação. E
`currency` aceitar um valor só não é convite a `UPDATE`: trocar a moeda de uma carteira com saldo deixaria
o extrato sem sentido.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`
  (`0001-bootstrap.md` §7), pelo BFF.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034-portal-market.sql`; DDL em
`ddl-confirmado.md` §5.34.

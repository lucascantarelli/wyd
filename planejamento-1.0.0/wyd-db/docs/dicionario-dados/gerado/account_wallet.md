> Pergunta: "o que `game.account_wallet` guarda e por que o saldo não cascateia ao apagar a conta?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.2; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Saldo corrente de `coin` da conta — uma linha por conta, 1:1.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | PK e FK para `account` |
| `coin` | `BIGINT` | não | `0` | 0 até `2147483647` (teto de `i32`) |

## Chave primária

`pk_account_wallet (account_id)` — uma carteira por conta; o saldo não tem histórico aqui.

## FKs de saída e de entrada

- **Saída:** `fk_account_wallet_account`: `account_id → game.account (id)`, `RESTRICT` — dinheiro não cascateia
  (`padrao-banco.md` §11): apagar conta com carteira falha alto.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_account_wallet_coin` | saldo negativo ou acima de `2147483647` |

O teto é o do campo `Coin` no fio (`STRUCT_MOB.Coin@28`), não um número de negócio.

## Índices

Nenhum além da PK.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0005-game-account.md` — arquivo `0005_game_account.sql`.

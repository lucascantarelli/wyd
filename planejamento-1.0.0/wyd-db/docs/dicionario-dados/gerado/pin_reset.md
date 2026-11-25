> Pergunta: "o que `game.pin_reset` guarda e por que o destinatário do link não está nesta tabela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.2; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Token de redefinição de PIN da conta, com hash, validade e consumo; o e-mail vem de
`portal.person.email` por join, nunca daqui (`P-22`, `M2`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | conta dona |
| `token_hash` | `TEXT` | não | — | hex de 64 |
| `requested_ip` | `INET` | sim | — | origem do pedido |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | emissão |
| `expires_at` | `TIMESTAMPTZ` | não | — | validade |
| `used_at` | `TIMESTAMPTZ` | sim | — | consumo |

## Chave primária

`pk_pin_reset (id)` — cada pedido é uma linha.

## FKs de saída e de entrada

- **Saída:** `fk_pin_reset_account`: `account_id → game.account (id)`, `RESTRICT`.
- **Entrada:** nenhuma.

## Unicidade

`uq_pin_reset_token (token_hash)` — hash único; não há único parcial de "token vivo" nesta tabela.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_pin_reset_token_forma` | hash que não é hex de 64 |
| `ck_pin_reset_validade` | validade não posterior à emissão |
| `ck_pin_reset_consumo` | consumo anterior à emissão |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_pin_reset_account (account_id, created_at DESC)` | pedidos de uma conta, do mais recente para o mais antigo |

O resgate é por `token_hash` (o único); o índice de leitura é o da conta.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal`.

## Migration

Forma (dona): `migrations/0005-game-account.md` — arquivo `0005-game-account.sql`.

> Pergunta: "o que `game.account` guarda, como o schema limita a cinco contas por pessoa e por que bloqueio disciplinar não mora aqui?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.2; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Conta de jogo: vaga, login, senha e PIN, ligada à pessoa do portal por FK — sem copiar e-mail.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `person_id` | `BIGINT` | não | — | dona da conta; FK `RESTRICT` |
| `slot` | `SMALLINT` | não | — | vaga 1–5 (`ux_account_slot`) |
| `login` | `TEXT` | não | — | 1–16 caracteres, já aparado |
| `login_fold` | `TEXT` gerada | sim | `lower(btrim(login))` | `STORED`, sem `NOT NULL`; nunca nula porque `login` não é |
| `password_hash` | `TEXT` | não | — | só `$argon2id$…` |
| `password_changed_at` | `TIMESTAMPTZ` | sim | — | nulo = nunca trocada (`M3`) |
| `pin_hash` | `TEXT` | sim | — | opcional; também argon2id |
| `pin_failure_count` | `SMALLINT` | não | `0` | falhas de PIN |
| `pin_locked_until` | `TIMESTAMPTZ` | sim | — | bloqueio do PIN |
| `login_failure_count` | `SMALLINT` | não | `0` | falhas de senha |
| `login_locked_until` | `TIMESTAMPTZ` | sim | — | bloqueio de login |
| `role` | `TEXT` | não | `'player'` | `player`, `moderator` ou `admin` |
| `status` | `TEXT` | não | `'active'` | só `active`/`closed` — ciclo de vida |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |
| `last_login_at` | `TIMESTAMPTZ` | sim | — | último login |
| `closed_at` | `TIMESTAMPTZ` | sim | — | fechamento |

## Chave primária

`pk_account (id)` — a conta é entidade; o login é único, não chave.

## FKs de saída e de entrada

- **Saída:** `fk_account_person`: `person_id → portal.person (id)`, `RESTRICT` — pessoa não é
  apagada em operação.
- **Entrada:** `account_wallet`, `account_ledger`, `account_moderation` e `pin_reset` (`RESTRICT`);
  `game.character` e `vault_item` (`CASCADE`); `game.session` e `ops.delivery_queue` (`RESTRICT`).

## Unicidade

- `ux_account_login (login_fold)` — **total**: login de conta fechada não é reutilizado;
- `ux_account_slot (person_id, slot) WHERE status <> 'closed'` — a 6ª conta falha aqui, inclusive com
  cadastros simultâneos; conta fechada devolve a vaga.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_account_slot_range` | vaga fora de 1–5 |
| `ck_account_login_len` | login vazio ou acima de 16 |
| `ck_account_login_forma` | login com espaço nas pontas ou vazio |
| `ck_account_role` | papel fora dos três |
| `ck_account_status` | estado fora de `active`/`closed` |
| `ck_account_senha_hasheada` | hash que não é argon2id |
| `ck_account_pin_hasheado` | PIN presente fora de argon2id |
| `ck_account_pin_falhas` | contador de PIN negativo |
| `ck_account_login_falhas` | contador de login negativo |
| `ck_account_senha_trocada` | troca anterior à criação |
| `ck_account_fechamento_coerente` | `closed` sem `closed_at`, ou o inverso |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_account_login` | login |
| `ux_account_slot` (parcial) | limite de contas por pessoa |
| `ix_account_person (person_id)` | contas da pessoa |

Trigger `trg_account_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game` (CRUD por default privilege de `game`).
- **Lê:** `wyd_game`, `wyd_read` (`SELECT` default); o portal não tem `USAGE` em `game`.
- **Bloqueio disciplinar:** não é coluna daqui — é `game.account_moderation` (dono:
  [moderação](../dominio-moderacao.md)).

## Migration

Forma (dona): `migrations/0005-game-account.md` — arquivo `0005-game-account.sql`.

> Pergunta: "o que `ops.login_attempt` guarda de cada tentativa de autenticação e por que não há e-mail nem login em claro?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.4; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Trilha append-only de tentativa de login, com o identificador hasheado para não vazar PII em `ops`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `occurred_at` | `TIMESTAMPTZ` | não | `now()` | instante da tentativa |
| `surface` | `TEXT` | não | — | `portal` (e-mail) ou `game` (login 7662) |
| `identifier_hash` | `TEXT` | não | — | `sha256` hex do identificador dobrado |
| `ip` | `INET` | sim | — | origem, sem identificar a pessoa |
| `result` | `TEXT` | não | — | desfecho da tentativa |

## Chave primária

`pk_login_attempt (id)` — sequência de eventos, sem chave natural.

## FKs de saída e de entrada

- **Saída:** nenhuma, pelo mesmo motivo de `audit_log`: a trilha sobrevive ao ator.
- **Entrada:** nenhuma.

## Unicidade

Nenhuma além da PK: a mesma tentativa pode se repetir.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_login_attempt_surface` | superfície fora de `portal`/`game` |
| `ck_login_attempt_identifier` | hash que não é hex de 64 |
| `ck_login_attempt_result` | resultado fora de `ok`, `bad_secret`, `bad_second_factor`, `blocked`, `locked`, `unknown_identifier` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_login_attempt_identifier (identifier_hash, occurred_at DESC)` | falhas por identificador na última hora |
| `ix_login_attempt_ip (ip, occurred_at DESC) WHERE ip IS NOT NULL` | abuso por origem (multibox, stuffing) |
| `ix_login_attempt_occurred (occurred_at DESC)` | retenção de 30 dias (`Q-RET-2`) |

## Papéis

- **Escreve:** `wyd_game` e `wyd_portal`, só `INSERT`.
- **Lê:** `wyd_read`.
- **Não escreve:** sem `UPDATE`/`DELETE` para os runtimes.

## Migration

Forma (dona): `migrations/0002-ops-audit.md` — arquivo `0002-ops-audit.sql`.

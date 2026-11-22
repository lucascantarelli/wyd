> Pergunta: "o que `portal.person_session` guarda de uma sessão do portal e por que só o hash do cookie mora no banco?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §4; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Sessão revogável do portal, com expiração absoluta; o cookie carrega token opaco e o banco só o `sha256`
(`A16`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `person_id` | `BIGINT` | não | — | dona da sessão |
| `token_hash` | `TEXT` | não | — | hex de 64; nunca o valor em claro |
| `ip` | `INET` | sim | — | origem |
| `user_agent` | `TEXT` | não | `''` | até 512 caracteres |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | abertura |
| `last_seen_at` | `TIMESTAMPTZ` | não | `now()` | sinal de vida; o idle é política, não coluna |
| `expires_at` | `TIMESTAMPTZ` | não | — | expiração **absoluta**, não se estende |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | revogação |

## Chave primária

`pk_person_session (id)` — uma linha por sessão emitida; revogar não apaga.

## FKs de saída e de entrada

- **Saída:** `fk_person_session_person`: `person_id → portal.person (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

`uq_person_session_token (token_hash)` — o mesmo verificador não pertence a duas sessões.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_person_session_token_forma` | verificador que não é hex de 64 |
| `ck_person_session_validade` | expiração não posterior à criação |
| `ck_person_session_revogacao` | revogação anterior à criação |
| `ck_person_session_sinal` | `last_seen_at` antes da criação ou depois da expiração |
| `ck_person_session_user_agent` | `user_agent` acima de 512 caracteres |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_person_session_person (person_id)` | lado filho da FK / sessões da pessoa |
| `ix_person_session_vivas (person_id, last_seen_at DESC) WHERE revoked_at IS NULL` | sessões vivas |
| `ix_person_session_expira (expires_at)` | reaper de `M22` |

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`; o jogo não lê sessão de portal.
- **Não lê:** `wyd_read`.

Retenção: 30 dias após `expires_at` (`M22`).

## Migration

Forma (dona): `migrations/0004-portal-access.md` — arquivo `0004_portal_access.sql`.

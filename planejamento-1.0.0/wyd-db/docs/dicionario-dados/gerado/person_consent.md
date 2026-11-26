> Pergunta: "o que `portal.person_consent` prova e por que a versão do documento é guardada?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.7; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Prova versionada de consentimento: qual texto a pessoa aceitou e quando, com revogação.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `person_id` | `BIGINT` | não | — | dono do consentimento |
| `kind` | `TEXT` | não | — | `terms`, `privacy` ou `marketing` |
| `version` | `TEXT` | não | — | versão do documento, 1–32 caracteres |
| `granted_at` | `TIMESTAMPTZ` | não | `now()` | aceite |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | revogação; nulo = vivo |

## Chave primária

`pk_person_consent (id)` — histórico: aceitar a versão nova revoga a anterior, sem apagar a linha.

## FKs de saída e de entrada

- **Saída:** `fk_person_consent_person`: `person_id → portal.person (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

`ux_person_consent_vivo (person_id, kind) WHERE revoked_at IS NULL` — um consentimento vivo por tipo;
as linhas revogadas não bloqueiam.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_person_consent_kind` | tipo fora de `terms`/`privacy`/`marketing` |
| `ck_person_consent_version` | versão vazia ou acima de 32 caracteres |
| `ck_person_consent_revogacao` | revogação anterior ao aceite |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_person_consent_vivo` (parcial) | consentimento vigente por tipo |
| `ix_person_consent_pessoa (person_id, granted_at DESC)` | histórico da pessoa |

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`; o jogo não lê consentimento.
- **Não lê:** `wyd_read`.

## Migration

Forma (dona): `migrations/0003_portal_person.md` — arquivo `0003_portal_person.sql`.

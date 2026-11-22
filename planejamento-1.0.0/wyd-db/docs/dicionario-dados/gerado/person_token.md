> Pergunta: "o que `portal.person_token` guarda e por que reemitir um token invalida o anterior?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §4; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Token de cerimônia da pessoa — verificação de e-mail, troca de senha e troca de e-mail —, com hash,
validade e consumo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `person_id` | `BIGINT` | não | — | dona do token |
| `purpose` | `TEXT` | não | — | `email_verify`, `password_reset` ou `email_change` |
| `token_hash` | `TEXT` | não | — | hex de 64 |
| `requested_ip` | `INET` | sim | — | origem do pedido; o destinatário vem de `person.email` |
| `attempt_count` | `SMALLINT` | não | `0` | tentativas de uso |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | emissão |
| `expires_at` | `TIMESTAMPTZ` | não | — | validade |
| `used_at` | `TIMESTAMPTZ` | sim | — | consumo; nulo = vivo |

## Chave primária

`pk_person_token (id)` — cada emissão é uma linha; o histórico de tokens consumidos fica.

## FKs de saída e de entrada

- **Saída:** `fk_person_token_person`: `person_id → portal.person (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

- `uq_person_token_hash (token_hash)` — hash único;
- `ux_person_token_vivo (person_id, purpose) WHERE used_at IS NULL` — um token vivo por pessoa e
  propósito; reemitir invalida o anterior (OWASP Forgot Password).

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_person_token_purpose` | propósito fora dos três |
| `ck_person_token_hash_forma` | hash que não é hex de 64 |
| `ck_person_token_validade` | validade não posterior à emissão |
| `ck_person_token_consumo` | consumo anterior à emissão |
| `ck_person_token_tentativas` | contador negativo |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_person_token_vivo` (parcial) | token vigente por pessoa/propósito |
| `ix_person_token_pessoa (person_id, created_at DESC)` | histórico por pessoa |
| `ix_person_token_expira (expires_at) WHERE used_at IS NULL` | limpeza de vencidos |

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`.
- **Não lê:** `wyd_read`.

## Migration

Forma (dona): `migrations/0004-portal-access.md` — arquivo `0004_portal_access.sql`.

> Pergunta: "o que `portal.person_profile` guarda e por que a ausência de linha não é erro?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.6; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Perfil de exibição opcional, 1:1 com a pessoa, sem ser identidade.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `person_id` | `BIGINT` | não | — | PK e FK para `person` |
| `display_name` | `TEXT` | sim | — | 2–32 caracteres quando existe |
| `birth_date` | `DATE` | sim | — | piso `1900-01-01`; futuro é recusado na borda |
| `country` | `TEXT` | sim | — | ISO 3166-1 alpha-2 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_person_profile (person_id)` — uma linha por pessoa; a ausência de linha é "sem perfil" (`D5`
respeitado: nada obrigatório).

## FKs de saída e de entrada

- **Saída:** `fk_person_profile_person`: `person_id → portal.person (id)`, `CASCADE`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK: não há segundo perfil para a mesma pessoa.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_person_profile_display_name` | nome presente com tamanho fora de 2–32 |
| `ck_person_profile_nascimento` | data anterior a `1900-01-01` |
| `ck_person_profile_pais` | país que não é duas letras maiúsculas |

O futuro em `birth_date` **não** é `CHECK`: `CURRENT_DATE` não é `IMMUTABLE`; a recusa é da borda.

## Índices

Nenhum além da PK — a tabela é lida pela chave.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`; o jogo não lê perfil.
- **Não lê:** `wyd_read` (sem `SELECT` default em `portal`).

## Migration

Forma (dona): `migrations/0003-portal-person.md` — arquivo `0003_portal_person.sql`.

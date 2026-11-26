> Pergunta: "o que `ops.config_revision` registra e por que uma revisão não tem rascunho?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.9; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Cabeçalho de uma revisão de configuração: quem mudou, quando passou a valer e por quê.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK **e** número da revisão |
| `author_kind` | `TEXT` | não | — | `person` ou `system` |
| `author_id` | `BIGINT` | sim | — | pessoa; sem FK (nasce em `0003`) |
| `author_ref` | `TEXT` | sim | — | processo, quando `system` |
| `note` | `TEXT` | não | `''` | motivo, até 500 caracteres |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação |
| `applied_at` | `TIMESTAMPTZ` | não | `now()` | quando passou a valer |

## Chave primária

`pk_config_revision (id)` — o `id` identity é o número da revisão; não há rascunho na `1.0.0`.

## FKs de saída e de entrada

- **Saída:** nenhuma. O autor usa o ator tripartido sem FK, porque `portal.person` nasce em `0003`, arquivo posterior; FK para frente é proibida (`plano-migrations.md` §2).
- **Entrada:** `ops.config_change.revision_id → ops.config_revision (id)`, `RESTRICT`.

## Unicidade

Só a PK. Duas revisões no mesmo instante são permitidas; a ordem é o `id`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_config_revision_autor` | `person` sem `author_id` ou com `author_ref`; `system` sem `author_ref` ou com `author_id` |
| `ck_config_revision_nota` | nota acima de 500 caracteres |
| `ck_config_revision_aplicacao` | `applied_at` anterior a `created_at` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_config_revision_aplicada (applied_at DESC)` | revisões mais recentes |
| `ix_config_revision_autor (author_kind, author_id, applied_at DESC)` | histórico por autor |

## Papéis

- **Escreve:** `wyd_portal`, só `SELECT, INSERT` (área administrativa).
- **Lê:** `wyd_read` (`SELECT` default de `ops`).
- **Não escreve:** sem `UPDATE`/`DELETE`: voltar atrás é revisão nova.

## Migration

Forma (dona): `migrations/0002_ops_audit.md` — arquivo `0002_ops_audit.sql`.

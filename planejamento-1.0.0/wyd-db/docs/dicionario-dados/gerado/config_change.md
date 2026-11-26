> Pergunta: "o que `ops.config_change` guarda de cada mudança e o que impede reescrever o histórico?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.9; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Uma linha por chave alterada dentro de uma revisão, com valor anterior e novo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `revision_id` | `BIGINT` | não | — | revisão dona da mudança |
| `target_schema` | `TEXT` | não | — | `content`, `game`, `portal` ou `ops` |
| `target_table` | `TEXT` | não | — | tabela alvo |
| `target_key` | `TEXT` | não | — | chave da linha, em texto |
| `old_value` | `TEXT` | sim | — | nulo = a chave não existia |
| `new_value` | `TEXT` | sim | — | nulo = a chave foi removida |

## Chave primária

`pk_config_change (id)` — identifica a mudança; o alvo é único por revisão, não globalmente.

## FKs de saída e de entrada

- **Saída:** `fk_config_change_revision`: `revision_id → ops.config_revision (id)`, `RESTRICT`.
- **Entrada:** nenhuma.

## Unicidade

`uq_config_change_alvo (revision_id, target_schema, target_table, target_key)` — uma chave muda uma vez
por revisão; mudar de novo exige outra revisão.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_config_change_schema` | schema fora dos quatro da série |
| `ck_config_change_tabela` | nome que não casa `^[a-z][a-z0-9_]*$` |
| `ck_config_change_chave` | chave fora de 1–200 caracteres |
| `ck_config_change_muda` | `old_value` igual a `new_value` (`IS DISTINCT FROM`) |
| `ck_config_change_tem_valor` | os dois valores nulos |
| `ck_config_change_tamanho` | valor acima de 4.000 caracteres |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_config_change_revision (revision_id)` | mudanças de uma revisão |
| `ix_config_change_alvo (target_schema, target_table, target_key, revision_id DESC)` | histórico de uma chave |

## Papéis

- **Escreve:** `wyd_portal`, só `SELECT, INSERT`.
- **Lê:** `wyd_read` (`SELECT` default de `ops`).
- **Não escreve:** sem `UPDATE`/`DELETE`; histórico append-only, sem purge.

## Migration

Forma (dona): `migrations/0002_ops_audit.md` — arquivo `0002_ops_audit.sql`.

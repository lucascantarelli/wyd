> Pergunta: "o que `ops.audit_log` registra e como as permissões impedem os runtimes de reescrever a trilha?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.3; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Trilha de auditoria da operação: quem fez o quê, quando, sem FK que a amarre ao ator.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `occurred_at` | `TIMESTAMPTZ` | não | `now()` | instante do evento |
| `actor_kind` | `TEXT` | não | — | `person`, `account` ou `system` |
| `actor_id` | `BIGINT` | sim | — | id do ator; nulo só para `system` |
| `actor_ref` | `TEXT` | sim | — | referência do processo; nulo fora de `system` |
| `action` | `TEXT` | não | — | verbo auditado |
| `target` | `TEXT` | não | `''` | alvo em texto |
| `detail` | `TEXT` | não | `''` | detalhe livre |

## Chave primária

`pk_audit_log (id)` — identifica a linha; não há chave natural, porque o mesmo ator repete a mesma ação.

## FKs de saída e de entrada

- **Saída:** nenhuma, **por desenho**: auditoria sobrevive ao ator (`M18-A`).
- **Entrada:** nenhuma.

## Unicidade

Nenhuma além da PK: eventos iguais em sequência são legítimos.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_audit_log_actor_kind` | `actor_kind` fora de `person`/`account`/`system` |
| `ck_audit_log_actor_sistema` | `system` com `actor_id` preenchido, ou o inverso |
| `ck_audit_log_actor_ref` | `actor_ref` preenchido para quem não é `system`, ou ausente para `system` |
| `ck_audit_log_action` | `action` vazia |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_audit_log_occurred (occurred_at DESC)` | linha do tempo e retenção |
| `ix_audit_log_actor (actor_kind, actor_id, occurred_at DESC)` | "o que o staff 42 fez?" |

## Papéis

- **Escreve:** `wyd_game` e `wyd_portal` só com `INSERT`; `wyd_read` lê.
- **Não escreve:** nenhum runtime recebe `UPDATE`/`DELETE`; o append-only vem do `GRANT`, não de trigger.
- **Administra:** `wyd_admin` — `INSERT` e `SELECT (id)` concedidos por `0032`; só o `id` é legível, e
  `UPDATE`/`DELETE` ficam fora.

## Migration

Forma (dona): `migrations/0002-ops-audit.md` — arquivo `0002-ops-audit.sql`.

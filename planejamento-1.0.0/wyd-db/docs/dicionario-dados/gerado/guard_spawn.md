> Pergunta: "o que `content.guard_spawn` guarda do Guard.txt e por que a exigência fica em outra tabela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel
**Sem arquivo no repositório:** `Guard.txt` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Guarda por índice do `Guard.txt`, ligado ao NPC que o representa. Guarda só a cabeça do registro: a
exigência de itens é linha filha, em `guard_spawn_reward`.

## Colunas
**Sem arquivo no repositório:** `Guard.txt` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `guard_index` | `SMALLINT` | não | — | índice do guarda no `Guard.txt` |
| `npc_id` | `INTEGER` | não | — | NPC que representa o guarda (FK) |
| `source_line` | `INTEGER` | não | — | linha do arquivo de origem |

## Chave primária

`pk_guard_spawn (guard_index)` — o índice do arquivo é a chave natural; o schema não numera o guarda de
outra forma, e não há identidade própria.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_guard_spawn_npc`: `npc_id → content.npc_definition (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — guarda não perde o NPC em silêncio.
- **Entrada, `CASCADE`:** `guard_spawn_reward` (`fk_guard_spawn_reward_guarda`) — a exigência é linha
  possuída do guarda.

## Unicidade

Só a PK: um NPC pode ser guarda de mais de um índice, e o schema não proíbe isso.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guard_spawn_indice` | `guard_index` negativo |
| `ck_guard_spawn_origem` | `source_line` menor ou igual a zero |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_guard_spawn_npc (npc_id)` | guardas que usam um NPC |

Sem carimbos e sem trigger: import puro.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019-content-world.md` §8.

## Migration

Forma (dona): `migrations/0019-content-world.md` — arquivo `0019-content-world.sql`.

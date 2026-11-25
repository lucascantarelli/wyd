> Pergunta: "o que `game.character_lineage` guarda e por que `lineage_code` não tem FK?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.11; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Linhagem escolhida pelo personagem e a maestria nela, uma linha por `lineage_code` — é histórico por
linha, não "uma linhagem por personagem".

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `lineage_code` | `TEXT` | não | — | código `snake_case` de 2–32 caracteres |
| `mastery` | `SMALLINT` | não | `0` | maestria na linhagem, 0–320 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_character_lineage (character_id, lineage_code)` — o código entra na chave; o mesmo personagem pode
ter mais de uma linhagem registrada.

## FKs de saída e de entrada

- **Saída:** `fk_character_lineage_character`: `character_id → game.character (id)`,
  `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.
- **Sem FK para conteúdo:** `lineage_code` não aponta para `content.skill_lineage` (`0013`), porque o
  catálogo é de arquivo posterior.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_lineage_codigo` | código fora de `^[a-z][a-z0-9_]{1,31}$` |
| `ck_character_lineage_maestria` | maestria fora de 0–320 |

A regex fixa o mesmo formato de `content.skill_lineage.slug`: minúsculas, dígitos e sublinhado, de 2 a
32 caracteres.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_character_lineage_codigo (lineage_code)` | personagens por linhagem |

Trigger `trg_character_lineage_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0008-game-skills.md` — arquivo `0008-game-skills.sql`.

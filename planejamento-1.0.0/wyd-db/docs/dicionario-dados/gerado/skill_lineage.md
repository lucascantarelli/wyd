> Pergunta: "o que `content.skill_lineage` guarda e por que o código de linhagem do personagem não tem FK para ela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.17; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Linhagem de skill — a árvore a que uma skill pertence. É taxonomia de look-up: `slug` estável, nome
exibido e aposentadoria sem `DELETE`. O conteúdo (13 linhagens) está previsto no seed `0016`, que segue
`bloqueada` até as contagens serem reconferidas na fonte (`plano-migrations.md` §4). Quem referencia é
o catálogo de skill (`0015`), não o personagem.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | código da linhagem, `snake_case` de 2–32; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–64 caracteres após aparar |
| `enabled` | `BOOLEAN` | não | `true` | `false` aposenta a linhagem sem `DELETE` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_skill_lineage (slug)` — o código é a identidade (`P-04`); nome não é chave.

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada, `RESTRICT` (1 FK):** `fk_skill_catalog_lineage` em `content.skill_catalog.lineage_slug`
  (`0015`), `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`. Linhagem referenciada por skill não é apagada.
- **Sem FK de `game.character_lineage`:** `lineage_code` (`0008`) usa o **mesmo domínio** de código —
  minúsculas, dígitos e sublinhado, 2–32 — mas o planejamento não criou FK entre `game` e `content`
  (`gerado/character_lineage.md` §"FKs de saída e de entrada"). A conferência do código é de regex em
  `0008`, não de integridade referencial.

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_skill_lineage_slug` | `slug` fora de `^[a-z][a-z0-9_]{1,31}$` |
| `ck_skill_lineage_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |

Não há coluna de vínculo com classe, sistema ou skill: a linhagem é um código plano, e a relação com a
classe mora em `skill_catalog` (`0015`).

## Índices

Nenhum além do índice da PK. Trigger `trg_skill_lineage_set_updated_at` (`BEFORE UPDATE`, por linha).

## Armadilha de leitura

O mesmo texto de código aparece em dois schemas com regras diferentes: aqui é FK; em
`game.character_lineage` é só `CHECK` de formato. Um `slug` aposentado (`enabled = false`) continua
aceito por `character_lineage`, e nada no banco impede que um personagem carregue uma linhagem que o
conteúdo já aposentou — a decisão é do seed e do runtime.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0016`), que está `bloqueada`.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7). Dono da permissão: `migrations/0013-content-taxonomy.md` §8.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0013-content-taxonomy.md` — arquivo `0013-content-taxonomy.sql`.

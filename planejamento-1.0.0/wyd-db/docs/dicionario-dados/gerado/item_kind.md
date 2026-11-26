> Pergunta: "o que `content.item_kind` guarda e o que significa um item de catálogo sem categoria?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.17; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Categoria de item — a taxonomia que o catálogo (`content.item_catalog`, `0014`) referencia em
`kind_slug`. Código estável, nome exibido e aposentadoria sem `DELETE`. O conteúdo (16 categorias) está
previsto no seed `0016`, que segue `bloqueada` até as contagens serem reconferidas na fonte
(`plano-migrations.md` §4). Não guarda regra de equipamento nem de empilhamento: isso não está
decidido nesta série.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | código da categoria, `snake_case` de 2–32; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–64 caracteres após aparar |
| `enabled` | `BOOLEAN` | não | `true` | `false` aposenta a categoria sem `DELETE` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_item_kind (slug)` — código estável como identidade (`P-04`). Nome não é chave e não tem `UNIQUE`.

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada, `RESTRICT` (1 FK):** `fk_item_catalog_kind` em `content.item_catalog.kind_slug` (`0014`),
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`. Categoria usada por item não é apagada em silêncio.
- **A FK é do valor, não da linha:** `item_catalog.kind_slug` é **anulável**; `NULL` significa
  categoria não provada, não erro de dado (`gerado/item_catalog.md` §"Armadilha de leitura").

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_item_kind_slug` | `slug` fora de `^[a-z][a-z0-9_]{1,31}$` |
| `ck_item_kind_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |

Nenhum `CHECK` obriga item de catálogo a ter categoria: a nulidade é decisão de `0014`, e o schema
aceita linha de categoria sem nenhum item.

## Índices

Nenhum além do índice da PK. Trigger `trg_item_kind_set_updated_at` (`BEFORE UPDATE`, por linha).

## Armadilha de leitura

`enabled = false` **não** invalida os itens que já apontam para o `slug`: a FK só olha existência, não
aposentadoria. Aposentar categoria é ato de seed; reclassificar item é `UPDATE` em
`item_catalog.kind_slug`, que o runtime não tem permissão de fazer (`content` é só `SELECT`).

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0016`), que está `bloqueada`.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7). Dono da permissão: `migrations/0013_content_taxonomy.md` §8.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0013_content_taxonomy.md` — arquivo `0013_content_taxonomy.sql`.

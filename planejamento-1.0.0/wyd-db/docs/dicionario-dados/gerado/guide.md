> Pergunta: "o que `portal.guide` guarda de cada guia e por que a categoria é texto, não tabela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.15; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Guia editorial (Wydpedia): `slug` estável, categoria, título, corpo em Markdown, estado, autoria e
carimbos. Mesmo formato de ciclo de vida de `portal.news`, com recorte por categoria na listagem.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `slug` | `TEXT` | não | — | endereço estável, minúsculas/dígitos/hífen, 3–120 |
| `category` | `TEXT` | não | `'geral'` | 2–32 caracteres no padrão de slug |
| `title` | `TEXT` | não | — | 1–160 após aparar |
| `body_md` | `TEXT` | não | — | corpo em Markdown, 1–200.000 caracteres |
| `status` | `TEXT` | não | `'draft'` | `draft`, `published` ou `archived` |
| `published_at` | `TIMESTAMPTZ` | sim | — | obrigatório fora de `draft` |
| `archived_at` | `TIMESTAMPTZ` | sim | — | obrigatório **só** em `archived` |
| `author_person_id` | `BIGINT` | não | — | autor; FK `RESTRICT` |
| `updated_by_person_id` | `BIGINT` | sim | — | último editor; FK `RESTRICT` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_guide (id)` — identidade sintética; o `slug` é a chave editorial com índice único próprio.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (2 FKs):** `fk_guide_author` (`author_person_id`) e `fk_guide_updated_by`
  (`updated_by_person_id`) → `portal.person (id)`, `ON DELETE RESTRICT`, `ON UPDATE NO ACTION`.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o guia.
- **Sem FK para categoria:** `category` é texto com `CHECK`, não referência a tabela.

## Unicidade

`ux_guide_slug UNIQUE (slug)` — um endereço por guia, inclusive arquivado. Não há único por categoria: a
mesma categoria agrupa quantos guias existirem.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guide_slug` | `slug` fora de `^[a-z0-9]+(-[a-z0-9]+)*$` ou fora de 3–120 caracteres |
| `ck_guide_categoria` | `category` fora de `^[a-z][a-z0-9_-]{1,31}$` |
| `ck_guide_titulo` | `title` vazio, só com espaço, ou acima de 160 caracteres |
| `ck_guide_corpo` | `body_md` vazio ou acima de 200.000 caracteres |
| `ck_guide_status` | `status` fora de `draft`, `published` e `archived` |
| `ck_guide_publicacao` | `published_at` nulo com `status` diferente de `draft`, ou preenchido em `draft` |
| `ck_guide_arquivo` | `archived_at` nulo em `archived`, ou preenchido fora de `archived` |

`ck_guide_categoria` aceita hífen e sublinhado no meio (`^[a-z][a-z0-9_-]{1,31}$`) e exige 2 caracteres no
mínimo; a lista de categorias não é fechada pelo schema.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_guide_slug (slug)` | endereço único e busca por `slug` |
| `ix_guide_publicados (category, published_at DESC) WHERE status = 'published'` | listagem pública por categoria, da publicação mais recente |
| `ix_guide_autor (author_person_id)` | lado filho da FK do autor (`P-15`) |

`trg_guide_set_updated_at` mantém `updated_at` no `UPDATE`, via `ops.fn_set_updated_at()` (`0001`).

## Armadilha de leitura

`category` não é enum: `CHECK` de formato só impede categoria vazia ou com caractere estranho, e a lista
de categorias válidas é do CMS — duas páginas podem divergir sobre "as categorias do guia" sem que o banco
esteja errado. `archived` exige `published_at` (não se arquiva o que nunca saiu) e `archived_at`
preenchido. O `slug` continua reservado depois de arquivado. E `body_md` não tem coluna de idioma: guia em
outro idioma é decisão de produto, não linha nova.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`
  (`0001-bootstrap.md` §7), pelo BFF do CMS. Nenhum seed cria guia.
- **Não leem:** `wyd_read` tem `USAGE` em `portal` mas não recebe `SELECT` por default; `wyd_game` não
  alcança o schema; `wyd_admin` não recebe estas tabelas em `0032`.

## Migration

Forma (dona): `migrations/0030-portal-content.md` — arquivo `0030-portal-content.sql`; DDL em
`ddl-confirmado.md` §5.15.

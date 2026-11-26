> Pergunta: "o que `portal.news` guarda de cada notícia e por que o estado e o carimbo andam juntos?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.15; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Notícia do site: `slug` estável, título, resumo, corpo em Markdown, capa opcional, estado editorial,
destaque e autoria. É conteúdo do portal, escrito pelo BFF com a credencial `wyd_portal` — não é estado
de jogo e não tem FK para `game`/`content`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `slug` | `TEXT` | não | — | endereço estável, minúsculas/dígitos/hífen, 3–120 |
| `title` | `TEXT` | não | — | 1–160 após aparar |
| `summary` | `TEXT` | não | `''` | resumo, até 400 caracteres |
| `body_md` | `TEXT` | não | — | corpo em Markdown, 1–200.000 caracteres |
| `cover_url` | `TEXT` | sim | — | capa; só `https://` |
| `status` | `TEXT` | não | `'draft'` | `draft`, `published` ou `archived` |
| `pinned` | `BOOLEAN` | não | `false` | destaque na listagem |
| `published_at` | `TIMESTAMPTZ` | sim | — | obrigatório fora de `draft` |
| `archived_at` | `TIMESTAMPTZ` | sim | — | obrigatório **só** em `archived` |
| `author_person_id` | `BIGINT` | não | — | autor; FK `RESTRICT` |
| `updated_by_person_id` | `BIGINT` | sim | — | último editor; FK `RESTRICT` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_news (id)` — identidade sintética; o `slug` é a chave editorial e tem índice único próprio, mas não é
a PK porque o endereço pode ser corrigido sem trocar a linha.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (2 FKs):** `fk_news_author` (`author_person_id`) e `fk_news_updated_by`
  (`updated_by_person_id`) → `portal.person (id)`, `ON DELETE RESTRICT`, `ON UPDATE NO ACTION`. Pessoa que
  assina ou editou não é apagada enquanto a notícia existir.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para a notícia.
- **Sem FK para imagem ou mídia:** `cover_url` é `TEXT` com `^https://`, e não há tabela de upload.

## Unicidade

`ux_news_slug UNIQUE (slug)` — um endereço por notícia, inclusive depois de arquivada: o link antigo
continua apontando para a mesma linha.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_news_slug` | `slug` fora de `^[a-z0-9]+(-[a-z0-9]+)*$` ou fora de 3–120 caracteres |
| `ck_news_titulo` | `title` vazio, só com espaço, ou acima de 160 caracteres |
| `ck_news_resumo` | `summary` acima de 400 caracteres |
| `ck_news_corpo` | `body_md` vazio ou acima de 200.000 caracteres |
| `ck_news_capa` | `cover_url` fora de `^https://` |
| `ck_news_status` | `status` fora de `draft`, `published` e `archived` |
| `ck_news_publicacao` | `published_at` nulo com `status` diferente de `draft`, ou preenchido em `draft` |
| `ck_news_arquivo` | `archived_at` nulo em `archived`, ou preenchido fora de `archived` |
| `ck_news_fixada` | `pinned = true` com `status` diferente de `published` |

Os dois `CHECK` de coerência são de **equivalência**: `archived` também exige `published_at`, porque sair
do rascunho é publicar antes de arquivar.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_news_slug (slug)` | endereço único e busca por `slug` |
| `ix_news_publicadas (published_at DESC) WHERE status = 'published'` | listagem pública, da mais recente para a mais antiga |
| `ix_news_autor (author_person_id)` | lado filho da FK do autor (`P-15`) |

`trg_news_set_updated_at` mantém `updated_at` no `UPDATE`, via `ops.fn_set_updated_at()` (`0001`).

## Armadilha de leitura

`archived_at` preenchido **não** significa que a notícia já foi publicada: quem garante isso é
`ck_news_publicacao`, que exige `published_at` em `archived`. `pinned` só existe em `published` — destaque
em rascunho é recusado. O `slug` é único para sempre: reaproveitar o endereço de uma notícia arquivada
falha em `ux_news_slug`, e a saída é arquivar mantendo o endereço. `summary` e `body_md` são texto puro; a
renderização de Markdown é do BFF.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`
  (`0001_bootstrap.md` §7), pelo BFF do CMS. Nenhum seed cria notícia.
- **Não leem:** `wyd_read` tem `USAGE` em `portal` mas não recebe `SELECT` por default; `wyd_game` não
  alcança o schema; `wyd_admin` não recebe estas tabelas em `0032`.

## Migration

Forma (dona): `migrations/0030_portal_content.md` — arquivo `0030_portal_content.sql`; DDL em
`ddl-confirmado.md` §5.15.

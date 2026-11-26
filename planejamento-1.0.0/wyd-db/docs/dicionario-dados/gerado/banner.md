> Pergunta: "o que `portal.banner` guarda de cada banner e por que o personagem em destaque não tem FK?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.15; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Banner de uma posição do site ou do launcher: título, subtítulo, imagem, link opcional, personagem em
destaque, prioridade, janela de exibição e autoria. É conteúdo editorial do portal, escrito pelo BFF do
CMS.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `slot` | `TEXT` | não | — | posição: `home_hero`, `home_side`, `login` ou `launcher` |
| `title` | `TEXT` | não | — | 1–80 após aparar |
| `subtitle` | `TEXT` | não | `''` | até 200 caracteres |
| `image_url` | `TEXT` | não | — | só `https://` |
| `link_url` | `TEXT` | sim | — | destino opcional; só `https://` |
| `featured_character_id` | `BIGINT` | sim | — | personagem em destaque; maior que zero, **sem FK** |
| `priority` | `SMALLINT` | não | `100` | ordenação, 0–999 |
| `enabled` | `BOOLEAN` | não | `true` | liga/desliga sem apagar |
| `starts_at` | `TIMESTAMPTZ` | não | `now()` | início da janela |
| `ends_at` | `TIMESTAMPTZ` | sim | — | fim opcional; posterior ao início |
| `author_person_id` | `BIGINT` | não | — | autor; FK `RESTRICT` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_banner (id)` — identidade sintética. Não há chave natural: o mesmo `slot` aceita vários banners, e a
ordem entre eles é `priority`.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (1 FK):** `fk_banner_author`: `author_person_id → portal.person (id)`,
  `ON DELETE RESTRICT`, `ON UPDATE NO ACTION` — pessoa que assina banner não é apagada.
- **Sem FK em `featured_character_id`:** personagem é de `game`, e `portal` não tem `USAGE` nesse schema
  (`P-17`). O `CHECK` só garante o piso `> 0`; validar o personagem é do BFF.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o banner.

## Unicidade

Nenhum único: dois banners podem ocupar a mesma posição ao mesmo tempo, e nada no schema impede dois
`enabled` no mesmo `slot` com a mesma `priority` — o desempate é do BFF.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_banner_slot` | `slot` fora de `home_hero`, `home_side`, `login` e `launcher` |
| `ck_banner_titulo` | `title` vazio, só com espaço, ou acima de 80 caracteres |
| `ck_banner_subtitulo` | `subtitle` acima de 200 caracteres |
| `ck_banner_imagem` | `image_url` fora de `^https://` |
| `ck_banner_link` | `link_url` preenchido fora de `^https://` |
| `ck_banner_personagem` | `featured_character_id` menor ou igual a zero |
| `ck_banner_prioridade` | `priority` fora de 0–999 |
| `ck_banner_janela` | `ends_at` não posterior a `starts_at` |

`ck_banner_janela` aceita `ends_at` nulo: banner sem fim declarado não expira pelo schema.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_banner_vigentes (slot, priority) WHERE enabled` | banners ligados de uma posição, na ordem de prioridade |
| `ix_banner_personagem (featured_character_id) WHERE featured_character_id IS NOT NULL` | banners que destacam um personagem; índice parcial sem FK, para consulta, não para integridade |

`trg_banner_set_updated_at` mantém `updated_at` no `UPDATE`, via `ops.fn_set_updated_at()` (`0001`).

## Armadilha de leitura

`enabled = true` **não** significa "no ar": a consulta pública também filtra `starts_at <= now()` e
`ends_at` nulo ou futuro, e essa janela não tem `CHECK` contra o relógio. se `priority` menor vem primeiro é decisão do BFF: o schema guarda o número e o índice, não a direção da
ordem. `featured_character_id` sem FK aceita personagem
inexistente, e `> 0` é o único piso. E `ends_at` no passado não desliga nada: a linha continua `enabled`
até alguém desligar; a limpeza é do `wyd-ops`.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`
  (`0001_bootstrap.md` §7), pelo BFF do CMS. Nenhum seed cria banner.
- **Não leem:** `wyd_read` tem `USAGE` em `portal` mas não recebe `SELECT` por default; `wyd_game` não
  alcança o schema; `wyd_admin` não recebe estas tabelas em `0032`.

## Migration

Forma (dona): `migrations/0030_portal_content.md` — arquivo `0030_portal_content.sql`; DDL em
`ddl-confirmado.md` §5.15.

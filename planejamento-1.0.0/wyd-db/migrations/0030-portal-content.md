> Pergunta: "como aplico o `0030_portal_content` e como notícia, banner, guia e versão de launcher passam a existir no banco?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (adendo); `ddl-confirmado.md` §5.15 (confirmado); `padrao-banco.md` §1/§5/§9/§11 (P-13, P-20); `wyd-portal/lacunas.md` (`LAC-POR-02`, `LAC-POR-03`)
> Revisado em: 2026-11-20

Trigésimo arquivo da série. Cria as quatro tabelas de conteúdo do portal que o dono decidiu em
2026-11-20: notícia (`news`), banner (`banner`), guia editorial (`guide`) e versão de launcher
(`launcher_release`). Todas em `portal`, porque são conteúdo do site, não estado de jogo.

!!! note "Aprovado em 2026-11-20"
    O **conjunto** das quatro tabelas foi decidido em 2026-11-20 (`LAC-POR-02`, `LAC-POR-03`) e o
    **desenho de coluna** foi aprovado pelo dono na mesma data, na opção "Sim, aprovar o desenho como
    está". A série passou a **30 arquivos / 96 tabelas** naquela data; hoje são 32/97 com o `0031` e o `0032` (só `GRANT`). As quatro tabelas entram na conta de
    DDL fechado (`ddl-confirmado.md` §1) — 49 na época da aprovação, 57 hoje — e esta página
    está `concluída` em `migrations-checklist.md`.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0030` (confirmado em 2026-11-20; próximo livre depois de `0029`) |
| Arquivo | `crates/wyd-db/migrations/0030_portal_content.sql` |
| Schema | `portal` (4 tabelas) |
| Posição | 30 de 32 (conteúdo editorial do portal) |
| Depende de | `0001` (schema `portal`, `ops.fn_set_updated_at`, default privileges), `0003` (`portal.person`, autor das linhas) |
| É dependência de | nada na série: `0029` (views) não lê estas tabelas |
| Tabelas criadas | 4 (`news`, `banner`, `guide`, `launcher_release`) |
| Objetos criados | 4 tabelas, 11 índices (6 parciais), 4 triggers |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6); na troca, o valor acompanha o total de arquivos |

**Por que depois de `0029`:** o lote não tem FK de saída para `game`/`content` e nenhuma view de
`0029` o lê, então ele não precisa vir antes. Renumerar `0029` só para colocar conteúdo antes das
views trocaria o número citado em vinte páginas sem mudar dependência nenhuma. A série cresce no fim,
como manda `plano-migrations.md` §2 (contíguo, sem lacuna) e a regra de arquivo novo do checklist §6.

## 2. Objetivo

Guardar o texto e as imagens que o portal publica — notícia, banner, guia — e o ponteiro da versão
vigente do launcher por canal, com autor, estado de publicação e janela de exibição.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schema `portal`, `ops.fn_set_updated_at()` e default privileges.
2. `0003_portal_person` aplicado: `portal.person` existe (FK de autor).
3. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.15, copiado sem alteração.

```sql
CREATE TABLE portal.news (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    slug                 TEXT NOT NULL,
    title                TEXT NOT NULL,
    summary              TEXT NOT NULL DEFAULT '',
    body_md              TEXT NOT NULL,
    cover_url            TEXT,
    status               TEXT NOT NULL DEFAULT 'draft',
    pinned               BOOLEAN NOT NULL DEFAULT false,
    published_at         TIMESTAMPTZ,
    archived_at          TIMESTAMPTZ,
    author_person_id     BIGINT NOT NULL,
    updated_by_person_id BIGINT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_news PRIMARY KEY (id),
    CONSTRAINT fk_news_author FOREIGN KEY (author_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_news_updated_by FOREIGN KEY (updated_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_news_slug CHECK (
        slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(slug) BETWEEN 3 AND 120
    ),
    CONSTRAINT ck_news_titulo CHECK (char_length(btrim(title)) BETWEEN 1 AND 160),
    CONSTRAINT ck_news_resumo CHECK (char_length(summary) <= 400),
    CONSTRAINT ck_news_corpo CHECK (char_length(body_md) BETWEEN 1 AND 200000),
    CONSTRAINT ck_news_capa CHECK (cover_url IS NULL OR cover_url ~ '^https://'),
    CONSTRAINT ck_news_status CHECK (status IN ('draft', 'published', 'archived')),
    CONSTRAINT ck_news_publicacao CHECK ((status <> 'draft') = (published_at IS NOT NULL)),
    CONSTRAINT ck_news_arquivo CHECK ((status = 'archived') = (archived_at IS NOT NULL)),
    CONSTRAINT ck_news_fixada CHECK (NOT pinned OR status = 'published')
);

CREATE UNIQUE INDEX ux_news_slug ON portal.news (slug);
CREATE INDEX ix_news_publicadas ON portal.news (published_at DESC) WHERE status = 'published';
CREATE INDEX ix_news_autor ON portal.news (author_person_id);

CREATE TRIGGER trg_news_set_updated_at
    BEFORE UPDATE ON portal.news
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.banner (
    id                    BIGINT GENERATED ALWAYS AS IDENTITY,
    slot                  TEXT NOT NULL,
    title                 TEXT NOT NULL,
    subtitle              TEXT NOT NULL DEFAULT '',
    image_url             TEXT NOT NULL,
    link_url              TEXT,
    featured_character_id BIGINT,
    priority              SMALLINT NOT NULL DEFAULT 100,
    enabled               BOOLEAN NOT NULL DEFAULT true,
    starts_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    ends_at               TIMESTAMPTZ,
    author_person_id      BIGINT NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_banner PRIMARY KEY (id),
    CONSTRAINT fk_banner_author FOREIGN KEY (author_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_banner_slot CHECK (slot IN ('home_hero', 'home_side', 'login', 'launcher')),
    CONSTRAINT ck_banner_titulo CHECK (char_length(btrim(title)) BETWEEN 1 AND 80),
    CONSTRAINT ck_banner_subtitulo CHECK (char_length(subtitle) <= 200),
    CONSTRAINT ck_banner_imagem CHECK (image_url ~ '^https://'),
    CONSTRAINT ck_banner_link CHECK (link_url IS NULL OR link_url ~ '^https://'),
    CONSTRAINT ck_banner_personagem CHECK (featured_character_id IS NULL OR featured_character_id > 0),
    CONSTRAINT ck_banner_prioridade CHECK (priority BETWEEN 0 AND 999),
    CONSTRAINT ck_banner_janela CHECK (ends_at IS NULL OR ends_at > starts_at)
);

CREATE INDEX ix_banner_vigentes ON portal.banner (slot, priority) WHERE enabled;
CREATE INDEX ix_banner_personagem ON portal.banner (featured_character_id)
    WHERE featured_character_id IS NOT NULL;

CREATE TRIGGER trg_banner_set_updated_at
    BEFORE UPDATE ON portal.banner
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.guide (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    slug                 TEXT NOT NULL,
    category             TEXT NOT NULL DEFAULT 'geral',
    title                TEXT NOT NULL,
    body_md              TEXT NOT NULL,
    status               TEXT NOT NULL DEFAULT 'draft',
    published_at         TIMESTAMPTZ,
    archived_at          TIMESTAMPTZ,
    author_person_id     BIGINT NOT NULL,
    updated_by_person_id BIGINT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guide PRIMARY KEY (id),
    CONSTRAINT fk_guide_author FOREIGN KEY (author_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_guide_updated_by FOREIGN KEY (updated_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_guide_slug CHECK (
        slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(slug) BETWEEN 3 AND 120
    ),
    CONSTRAINT ck_guide_categoria CHECK (category ~ '^[a-z][a-z0-9_-]{1,31}$'),
    CONSTRAINT ck_guide_titulo CHECK (char_length(btrim(title)) BETWEEN 1 AND 160),
    CONSTRAINT ck_guide_corpo CHECK (char_length(body_md) BETWEEN 1 AND 200000),
    CONSTRAINT ck_guide_status CHECK (status IN ('draft', 'published', 'archived')),
    CONSTRAINT ck_guide_publicacao CHECK ((status <> 'draft') = (published_at IS NOT NULL)),
    CONSTRAINT ck_guide_arquivo CHECK ((status = 'archived') = (archived_at IS NOT NULL))
);

CREATE UNIQUE INDEX ux_guide_slug ON portal.guide (slug);
CREATE INDEX ix_guide_publicados ON portal.guide (category, published_at DESC)
    WHERE status = 'published';
CREATE INDEX ix_guide_autor ON portal.guide (author_person_id);

CREATE TRIGGER trg_guide_set_updated_at
    BEFORE UPDATE ON portal.guide
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.launcher_release (
    id                     BIGINT GENERATED ALWAYS AS IDENTITY,
    channel                TEXT NOT NULL DEFAULT 'stable',
    version                TEXT NOT NULL,
    manifest_url           TEXT NOT NULL,
    manifest_sha256        CHAR(64) NOT NULL,
    min_version            TEXT,
    notes_md               TEXT NOT NULL DEFAULT '',
    status                 TEXT NOT NULL DEFAULT 'draft',
    released_at            TIMESTAMPTZ,
    revoked_at             TIMESTAMPTZ,
    published_by_person_id BIGINT,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_launcher_release PRIMARY KEY (id),
    CONSTRAINT fk_launcher_release_publisher FOREIGN KEY (published_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_launcher_release_canal CHECK (channel IN ('stable', 'beta')),
    CONSTRAINT ck_launcher_release_versao CHECK (
        version ~ '^[0-9]+[.][0-9]+[.][0-9]+(-[a-z0-9.]+)?$'
    ),
    CONSTRAINT ck_launcher_release_min_versao CHECK (
        min_version IS NULL OR min_version ~ '^[0-9]+[.][0-9]+[.][0-9]+(-[a-z0-9.]+)?$'
    ),
    CONSTRAINT ck_launcher_release_manifesto CHECK (manifest_url ~ '^https://'),
    CONSTRAINT ck_launcher_release_sha CHECK (manifest_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_launcher_release_notas CHECK (char_length(notes_md) <= 20000),
    CONSTRAINT ck_launcher_release_status CHECK (status IN ('draft', 'published', 'revoked')),
    CONSTRAINT ck_launcher_release_publicacao CHECK ((status <> 'draft') = (released_at IS NOT NULL)),
    CONSTRAINT ck_launcher_release_revogacao CHECK ((status = 'revoked') = (revoked_at IS NOT NULL)),
    CONSTRAINT ck_launcher_release_autor CHECK (
        status = 'draft' OR published_by_person_id IS NOT NULL
    )
);

CREATE UNIQUE INDEX ux_launcher_release_versao ON portal.launcher_release (channel, version);
CREATE UNIQUE INDEX ux_launcher_release_vigente ON portal.launcher_release (channel)
    WHERE status = 'published';
CREATE INDEX ix_launcher_release_historico ON portal.launcher_release (channel, released_at DESC)
    WHERE status <> 'draft';

CREATE TRIGGER trg_launcher_release_set_updated_at
    BEFORE UPDATE ON portal.launcher_release
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP TRIGGER trg_launcher_release_set_updated_at ON portal.launcher_release;
DROP INDEX ix_launcher_release_historico;
DROP INDEX ux_launcher_release_vigente;
DROP INDEX ux_launcher_release_versao;
DROP TABLE portal.launcher_release;

DROP TRIGGER trg_guide_set_updated_at ON portal.guide;
DROP INDEX ix_guide_autor;
DROP INDEX ix_guide_publicados;
DROP INDEX ux_guide_slug;
DROP TABLE portal.guide;

DROP TRIGGER trg_banner_set_updated_at ON portal.banner;
DROP INDEX ix_banner_personagem;
DROP INDEX ix_banner_vigentes;
DROP TABLE portal.banner;

DROP TRIGGER trg_news_set_updated_at ON portal.news;
DROP INDEX ix_news_autor;
DROP INDEX ix_news_publicadas;
DROP INDEX ux_news_slug;
DROP TABLE portal.news;
```

Sem `GRANT`/`REVOKE`: o lote herda o default privilege de `0001` (ver §8) e não cria papel.

## 6. Tabelas de colunas

### 6.1 `portal.news`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | Chave. |
| `slug` | `TEXT` | não | — | Endereço estável `/noticias/[slug]`. |
| `title` | `TEXT` | não | — | Título exibido (1–160). |
| `summary` | `TEXT` | não | `''` | Resumo da lista e do cartão. |
| `body_md` | `TEXT` | não | — | Corpo em Markdown, renderizado pelo portal. |
| `cover_url` | `TEXT` | sim | — | Imagem de capa; só `https://`. Arquivo mora no `wyd-ops`, não no banco. |
| `status` | `TEXT` | não | `'draft'` | `draft`, `published` ou `archived`. |
| `pinned` | `BOOLEAN` | não | `false` | Destaque na home. |
| `published_at` | `TIMESTAMPTZ` | sim | — | Instante da publicação; nulo só em rascunho. |
| `archived_at` | `TIMESTAMPTZ` | sim | — | Instante do arquivamento; preenchido só em `archived`. |
| `author_person_id` | `BIGINT` | não | — | FK `portal.person`: quem criou. |
| `updated_by_person_id` | `BIGINT` | sim | — | FK `portal.person`: quem editou por último. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos; `updated_at` mantido por trigger. |

### 6.2 `portal.banner`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | Chave. |
| `slot` | `TEXT` | não | — | `home_hero`, `home_side`, `login` ou `launcher`. |
| `title` / `subtitle` | `TEXT` | não | — / `''` | Texto do banner (1–80 / até 200). |
| `image_url` | `TEXT` | não | — | Imagem; só `https://`. |
| `link_url` | `TEXT` | sim | — | Destino do clique; `https://` ou nulo. |
| `featured_character_id` | `BIGINT` | sim | — | `game.character.id` em destaque, **sem FK** (`P-17`; mesmo padrão de `portal.donate_credit.account_id` em `0011`). |
| `priority` | `SMALLINT` | não | `100` | Ordem no slot (0–999; menor primeiro). |
| `enabled` | `BOOLEAN` | não | `true` | Liga/desliga sem apagar. |
| `starts_at` / `ends_at` | `TIMESTAMPTZ` | não / sim | `now()` / — | Janela de exibição; `ends_at` nulo = sem fim. |
| `author_person_id` | `BIGINT` | não | — | FK `portal.person`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos. |

### 6.3 `portal.guide`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | Chave. |
| `slug` | `TEXT` | não | — | Endereço `/wydpedia/guias/[slug]`. |
| `category` | `TEXT` | não | `'geral'` | Agrupamento (`geral`, `classe`, `sistema`, `evento`…). |
| `title` | `TEXT` | não | — | Título (1–160). |
| `body_md` | `TEXT` | não | — | Corpo em Markdown. |
| `status` | `TEXT` | não | `'draft'` | `draft`, `published` ou `archived`. |
| `published_at` / `archived_at` | `TIMESTAMPTZ` | sim | — | Carimbos coerentes com `status`. |
| `author_person_id` | `BIGINT` | não | — | FK `portal.person`. |
| `updated_by_person_id` | `BIGINT` | sim | — | FK `portal.person`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos. |

### 6.4 `portal.launcher_release`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | Chave. |
| `channel` | `TEXT` | não | `'stable'` | `stable` ou `beta`. |
| `version` | `TEXT` | não | — | Versão semântica (`1.2.3`, `1.3.0-beta.1`). |
| `manifest_url` | `TEXT` | não | — | URL `https://` do manifesto assinado. |
| `manifest_sha256` | `CHAR(64)` | não | — | Hash do manifesto, hex minúsculo. |
| `min_version` | `TEXT` | sim | — | Versão mínima que pode atualizar direto. |
| `notes_md` | `TEXT` | não | `''` | Notas da versão, Markdown (até 20.000). |
| `status` | `TEXT` | não | `'draft'` | `draft`, `published` ou `revoked`. |
| `released_at` | `TIMESTAMPTZ` | sim | — | Instante da publicação. |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | Instante da revogação. |
| `published_by_person_id` | `BIGINT` | sim | — | FK `portal.person`; obrigatório fora de rascunho. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos. |

## 7. Contratos e invariantes

| Constraint | Proíbe | Erro |
|---|---|---|
| `ux_news_slug`, `ux_guide_slug` | duas notícias/guias no mesmo endereço | `23505` |
| `ck_news_publicacao` / `ck_guide_publicacao` | publicado sem `published_at`, ou rascunho com ele | `23514` |
| `ck_news_arquivo` / `ck_guide_arquivo` | `archived` sem carimbo, ou carimbo fora de `archived` | `23514` |
| `ck_news_fixada` | destaque em rascunho/arquivada | `23514` |
| `fk_news_author` (e irmãs) | apagar pessoa que assina conteúdo | `23503` |
| `ck_banner_janela` | fim antes do início | `23514` |
| `ck_banner_imagem` / `ck_news_capa` | imagem fora de `https://` | `23514` |
| `ux_launcher_release_versao` | mesma versão no mesmo canal | `23505` |
| `ux_launcher_release_vigente` | **dois** lançamentos publicados no mesmo canal | `23505` |
| `ck_launcher_release_sha` | hash que não é 64 hex minúsculo | `23514` |
| `ck_launcher_release_autor` | publicar sem autor | `23514` |

O `23505` de `ux_launcher_release_vigente` é o sinal de "revogue o anterior primeiro": publicar
versão nova é transação única que revoga a vigente e publica a nova, não `UPDATE` solto.

## 8. Permissões

Nenhum `GRANT`/`REVOKE` novo: valem os default privileges de `0001` §3.

| Papel | Efeito |
|---|---|
| `wyd_portal` | `SELECT`, `INSERT`, `UPDATE`, `DELETE` nas quatro tabelas — é quem serve o site e o BFF administrativo. |
| `wyd_read` | tem `USAGE` em `portal`, mas **não** recebe `SELECT` por default neste schema; leitura pública passa pelo BFF. |
| `wyd_game` | nenhum: jogo não lê conteúdo editorial. |
| `wyd_migrator` | dono da criação; aplica o arquivo. |

**Escrita pelo `wyd-admin`:** não existe papel `wyd_admin` no `provision` (`src/provision/spec.md` não
cria papel de serviço além dos quatro). Hoje a API interna usa a credencial `wyd_portal`; se o
`wyd-admin` ganhar papel próprio, o `GRANT` vem em lote novo — este arquivo **não** inventa papel.
Nenhuma escrita parte do navegador: o BFF grava e registra autor em `ops.audit_log`.

## 9. Retenção, leitura e escrita

- **Sem `deleted_at` (P-20).** Estado é `status` + carimbo. Notícia e guia saem do ar por
  `archived`; linha arquivada continua legível para auditoria e link antigo.
- **Sem `DELETE` de conteúdo pela aplicação**, apesar do privilégio existir: apagar notícia publicada
  quebra link e histórico. Expurgo, se um dia houver, é job do `wyd-ops` com prazo declarado — não
  está decidido neste lote.
- **Banner expirado** (`ends_at` no passado) permanece; a consulta pública filtra a janela e
  `enabled`. Limpeza é do `wyd-ops`.
- **Launcher** nunca é apagado: versão antiga fica `revoked`. O histórico é o que permite responder
  "qual manifesto estava no ar em tal data".
- **Leitura pública:** `news`/`guide` só `status = 'published'` e `published_at <= now()`; `banner`
  só `enabled`, dentro da janela, ordenado por `priority`; `launcher_release` só a vigente
  (`status = 'published'`) do canal pedido. Rascunho nunca sai do BFF.
- **Escrita:** `/admin/conteudo/noticias`, `/admin/conteudo/wydpedia` e `/admin/launcher` do portal,
  com autor em `ops.audit_log` (`0002`), pela credencial `wyd_portal`. Ver `LAC-POR-01`: ação em
  `game` continua sendo do `wyd-admin`, e estas tabelas **não** são estado de jogo.

## 10. Prova de execução (quando a série for escrita)

1. `psql -v ON_ERROR_STOP=1 -f 0030_portal_content.sql` como `wyd_migrator` → `CREATE TABLE` ×4.
2. `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'portal';` → **15** depois de
   `0003` (3) + `0004` (2) + `0011` (6) + estas 4; a conta por tela é a de
   `wyd-portal/docs/arquitetura/consumo-do-banco.md` §7.
3. `SET ROLE wyd_portal; INSERT INTO portal.news (slug, title, body_md, author_person_id) VALUES
   ('teste', 'Teste', 'corpo', 1);` → `INSERT 0 1`, `status = 'draft'`.
4. `UPDATE portal.news SET status = 'published';` → `23514` (`ck_news_publicacao`), provando que o
   carimbo é obrigatório.
5. `INSERT INTO portal.launcher_release (version, manifest_url, manifest_sha256, status,
   released_at, published_by_person_id) VALUES (...)` duas vezes com `status = 'published'` no mesmo
   canal → o segundo dá `23505` (`ux_launcher_release_vigente`).
6. `SET ROLE wyd_game; SELECT * FROM portal.news;` → `42501 permission denied for schema portal`.

## 11. O que este arquivo não decide

| Assunto | Onde fecha |
|---|---|
| histórico de revisão de notícia/guia (versão anterior do texto) | produto; hoje só `updated_by` + auditoria |
| prazo de expurgo de conteúdo arquivado | `wyd-ops` (política de retenção) |
| papel de banco próprio do `wyd-admin` | `provision`/`wyd-admin`, quando planejado |
| RLS por linha | fora do escopo da `1.0.0`; permissão é por papel |
| renderização e sanitização do Markdown | `wyd-portal` (BFF), não o banco |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

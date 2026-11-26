-- migrate:up
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
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE portal.launcher_release, portal.guide, portal.banner, portal.news;

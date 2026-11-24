-- 0001_bootstrap: schemas, permissões, função de `updated_at` e versão do conteúdo.
-- Pré-requisito de ambiente: db/roles.sql (papéis do cluster e o estado de `public`) antes desta série.

-- migrate:up
CREATE SCHEMA content AUTHORIZATION wyd_owner;
CREATE SCHEMA game    AUTHORIZATION wyd_owner;
CREATE SCHEMA portal  AUTHORIZATION wyd_owner;
CREATE SCHEMA ops     AUTHORIZATION wyd_owner;

-- `public` não é tocado aqui: schema e privilégio dele são estado de ambiente, emitido por
-- `db/roles.sql` (`src/provision/spec.md` §4). O ledger `schema_migrations` nasce nele antes de 0001.

GRANT USAGE ON SCHEMA content TO wyd_game, wyd_portal, wyd_read;
GRANT USAGE ON SCHEMA game    TO wyd_game, wyd_read;   -- portal lê presença por view
GRANT USAGE ON SCHEMA portal  TO wyd_portal, wyd_read; -- jogo lê identidade por view
GRANT USAGE ON SCHEMA ops     TO wyd_read;
-- Quem só ACRESCENTA auditoria também precisa de `USAGE` no schema: sem ele o `GRANT INSERT` do
-- `0002` falha com `42501 permission denied for schema ops`, e não por falta de `INSERT` na tabela.
GRANT USAGE ON SCHEMA ops     TO wyd_game, wyd_portal;

ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA game
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO wyd_game;
ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA content
    GRANT SELECT ON TABLES TO wyd_game, wyd_portal, wyd_read;
ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA portal
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO wyd_portal;
ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator IN SCHEMA ops
    GRANT SELECT ON TABLES TO wyd_read;

-- Única função de trigger da série (P-16): manter `updated_at`. Uma função para todas as tabelas.
CREATE FUNCTION ops.fn_set_updated_at() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$;

-- Linha única: a versão do conteúdo é uma só por banco, não uma entidade. `id = 1` é a chave.
CREATE TABLE content.content_version (
    id         SMALLINT NOT NULL,
    version    INTEGER NOT NULL,
    note       TEXT NOT NULL DEFAULT '',
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_content_version PRIMARY KEY (id),
    CONSTRAINT ck_content_version_linha_unica CHECK (id = 1),
    CONSTRAINT ck_content_version_versao CHECK (version >= 1)
);

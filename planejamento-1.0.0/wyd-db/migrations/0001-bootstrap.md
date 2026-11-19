> Pergunta: "como aplico o `0001_bootstrap` e o que passa a existir no banco depois dele?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3, `ddl-confirmado.md` §3, `padrao-banco.md` §2/§8
> Revisado em: 2026-11-17

Primeiro arquivo da série `migrations/`. Ele não cria tabela de domínio: cria o **terreno** —
os quatro schemas, a fronteira de permissão entre eles, a única função de trigger da série e a linha
de versão do conteúdo. Tudo que vem depois pressupõe este arquivo aplicado.

!!! warning "Nada disto rodou ainda"
    O DDL abaixo é **planejado** (`P`): nenhum Postgres foi executado contra ele e o arquivo
    `crates/wyd-db/migrations/0001_bootstrap.sql` ainda não existe. O texto é o de
    `ddl-confirmado.md` §3, que é o dono do DDL; esta página não o reescreve por conta própria.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0001` |
| Arquivo | `crates/wyd-db/migrations/0001_bootstrap.sql` |
| Schema | nenhum próprio — **cria** `content`, `game`, `portal`, `ops` |
| Posição | 1 de 29 (primeiro da série) |
| Depende de | `db/roles.sql` aplicado por `wyd-db provision apply` (papéis e o estado de `public` não vão na série) |
| É dependência de | todos os outros 28 arquivos |
| Tabelas criadas | 1 (`content.content_version`) |
| Objetos criados | 4 schemas, 1 função (`ops.fn_set_updated_at`), 4 blocos de `ALTER DEFAULT PRIVILEGES`, os `GRANT USAGE` dos quatro schemas — **nada** em `public` |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Criar os quatro schemas de domínio com os papéis de cada um, de modo que **o banco** recuse escrita
cruzada (portal não escreve em `game`, jogo não lê `portal.person` cru) e que toda tabela criada
depois por `wyd_migrator` já nasça com o privilégio certo — mais a função única de `updated_at` e a
linha que registra a versão do conteúdo.

## 3. Pré-requisitos de ambiente

Os cinco papéis de cluster — `wyd_owner`, `wyd_migrator`, `wyd_game`, `wyd_portal` e `wyd_read` —
precisam existir **antes** deste arquivo, junto com a membership `wyd_migrator` → `wyd_owner`: sem eles
o `0001` falha em `AUTHORIZATION` e em `GRANT` com `role "wyd_owner" does not exist`. O estado
desejado dos papéis e o artefato `db/roles.sql` — gerado por `wyd-db provision render`, nunca escrito à
mão — estão especificados em `planejamento-1.0.0/wyd-db/src/provision/spec.md` (marca **planejamento**);
como provisionar (`WYD_PROVISION_DSN`, `provision render|plan|apply|check`) e por que papel fica fora da
série estão em ADR-DB-0001. Senha e `pg_hba` não entram no artefato: a regra está em `spec.md` §7.

Um efeito deste arquivo depende de quem o executa: `ALTER DEFAULT PRIVILEGES FOR ROLE wyd_migrator` só
vale para objetos **criados por** `wyd_migrator`. Aplicar o `up` com outro papel não produz erro na
migration, mas faz a fronteira de §7 não valer para as tabelas seguintes. É por isso que o runner tem preflight estrito de `current_user` (`src/migrate/spec.md` §7; a decisão é de `ADR-DB-0001`, item 6).

O estado do schema `public` também é pré-requisito de ambiente, e este arquivo **não** o define:
`provision apply` revoga o `CREATE` de `PUBLIC` ali e concede `USAGE` e `CREATE` a `wyd_migrator`,
porque o runner cria o ledger em `public` antes do primeiro arquivo e, no PG15+, `public` pertence a
`pg_database_owner` — quem roda a série não tem como alterar esse ACL. Texto emitido e porquê:
`src/provision/spec.md` §4; decisão em `ADR-DB-0001`, item 8. Sem esse passo o `apply` falha no
`prepare` com `42501`, antes de chegar a este arquivo.

## 4. DDL do `up`

Arquivo completo, pronto para colar. Os marcadores `-- migrate:up` e `-- migrate:down` ficam sozinhos
na linha porque o runner os procura assim.

```sql
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
```

**Por que nada disto é `ALTER`:** a série é escrita do zero, sem produção e sem schema anterior, então
cada arquivo nasce na forma final (`P-19`). Não existe `IF NOT EXISTS` aqui de propósito — rodar
`0001` num banco que já tem os schemas **deve** falhar alto, porque isso significa série aplicada em
lugar errado.

## 5. Tabela criada: `content.content_version`

Uma linha só, para sempre. Ela responde "que versão de conteúdo este banco tem?" sem consultar seed
nem contar linhas de catálogo.

| Coluna | Tipo | Nulo | Default | O que é |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | — | Chave literal `1`. Não é identity: a tabela tem uma linha, o id é a constante que a identifica, e `INSERT` explícito é a intenção (§6). |
| `version` | `INTEGER` | não | — | Versão do conteúdo aplicado; incrementa uma vez por seed aplicado (`plano-migrations.md` §4). Começa em `1` no primeiro seed. |
| `note` | `TEXT` | não | `''` | Observação livre do seed que gravou a linha — texto curto de operação, nunca dado de jogo. |
| `applied_at` | `TIMESTAMPTZ` | não | `now()` | Instante do último seed. `now()` é do servidor, nunca do cliente (`P-06`). |

Não há `updated_at` e não há trigger nesta tabela: quem a escreve é o próprio seed, e ele grava
`applied_at` na mesma instrução. A função `ops.fn_set_updated_at()` existe aqui para as tabelas que
**têm** `updated_at` a partir de `0003`.

## 6. Invariantes e o erro que cada uma devolve

| Constraint / objeto | O que proíbe | SQLSTATE |
|---|---|---|
| `pk_content_version` | duas linhas — o par `id` repetido | `23505` (`unique_violation`) |
| `ck_content_version_linha_unica` | `id` diferente de `1`, inclusive `NULL` (o `NOT NULL` vem antes) | `23514` (`check_violation`) |
| `ck_content_version_versao` | `version` menor que `1` | `23514` |
| `id NOT NULL`, `version NOT NULL` | linha sem chave ou sem versão | `23502` (`not_null_violation`) |
| `content.content_version` sem `INSERT` para `wyd_game`/`wyd_portal` | jogo ou portal gravando versão de conteúdo | `42501` (`insufficient_privilege`) |

Consequência prática: `INSERT INTO content.content_version (version) VALUES (7)` **falha** —
`id` não tem default. O `INSERT` correto é explícito e idempotente por natureza, porque a segunda
tentativa bate na PK:

```sql
INSERT INTO content.content_version (id, version, note) VALUES (1, 7, 'seed taxonomia')
ON CONFLICT (id) DO UPDATE
SET version = EXCLUDED.version, note = EXCLUDED.note, applied_at = now();
```

Esse `UPSERT` é a forma que os seis arquivos de seed usam (`plano-migrations.md` §4). Ele vive nos
seeds, não aqui: `0001` só cria a tabela.

## 7. Permissões: quem pode o quê depois deste arquivo

`USAGE` no schema é o direito de **atravessar** o schema; o direito de ler/escrever tabela vem dos
default privileges. Os dois são necessários.

| Schema | `USAGE` para | Tabela criada depois herda |
|---|---|---|
| `content` | `wyd_game`, `wyd_portal`, `wyd_read` | `SELECT` para os três; **nenhuma escrita de runtime** |
| `game` | `wyd_game`, `wyd_read` | `SELECT, INSERT, UPDATE, DELETE` para `wyd_game` |
| `portal` | `wyd_portal`, `wyd_read` | `SELECT, INSERT, UPDATE, DELETE` para `wyd_portal` |
| `ops` | `wyd_read`, `wyd_game`, `wyd_portal` | `SELECT` só para `wyd_read`; `INSERT` entra por arquivo (ex. `0002`). `wyd_game`/`wyd_portal` têm `USAGE` e **nenhum** privilégio de tabela além do `INSERT` explícito |
| `public` | nada nesta série: estado do provisionamento (`src/provision/spec.md` §4) | nada além do ledger, que o runner cria antes deste arquivo |

`portal` **não** tem `USAGE` em `game` e `game` **não** tem `USAGE` em `portal`: é o que faz
`permission denied for schema game` quando o portal tenta escrever estado vivo. A leitura cruzada
existe, mas por view em `0029`, que é criada no schema de quem lê (`P-17`).

Duas consequências que costumam surpreender:

- **`content` não recebe `INSERT` de ninguém em runtime.** Catálogo é escrito por migrator/seed. Um
  bug que tente gravar item em produção falha no banco, não na revisão de código.
- **`ops` é leitura para `wyd_read` e escrita só por `INSERT` para `wyd_game`/`wyd_portal`.** Eles
  têm `USAGE` no schema — sem ele o `GRANT INSERT` de `0002` respondería `42501 permission denied for
  schema ops` e a prova de append-only reprovaria pelo motivo errado — mas **nenhum** `SELECT`,
  `UPDATE` ou `DELETE` de tabela: append-only por permissão (`P-16`).

## 8. DDL do `down`

Ordem inversa exata do `up`. Roda depois de todos os outros `down`, porque o runner percorre a série ao
contrário (`src/migrate/app/runner.md` §3).

```sql
-- migrate:down
DROP TABLE content.content_version;
DROP FUNCTION ops.fn_set_updated_at();

-- Sem CASCADE de propósito: se sobrar objeto de outro arquivo aqui, o down falha alto em vez de
-- apagar o que não é dele. É o gate de que cada arquivo derruba o próprio `up`.
DROP SCHEMA ops;
DROP SCHEMA portal;
DROP SCHEMA game;
DROP SCHEMA content;

-- Nada de `public` no `down`: o estado dele é do provisionamento, e o ledger sobrevive ao reset.
```

`DROP SCHEMA` sem `CASCADE` é o teste mais barato da série: se `0007` esqueceu uma tabela em `game`, o
`down` do `0001` estoura com `cannot drop schema game because other objects depend on it` — e o erro
aponta para o arquivo culpado. Com `CASCADE` o mesmo defeito passaria verde e a tabela órfã sumiria
sem que ninguém soubesse que ela ficou.

!!! note "`public` saiu desta série — decidido em 2026-11-18"
    O `up` e o `down` deste arquivo **não** tocam em `public`. O estado dele — `CREATE` revogado de
    `PUBLIC`, `USAGE` e `CREATE` para `wyd_migrator` — é emitido por `provision render` e aplicado por
    `provision apply` antes do primeiro `apply` da série (`src/provision/spec.md` §4). Três fatos, na
    ordem em que decidem: `public` existe com o banco, ao contrário dos quatro schemas que este arquivo
    cria; o runner cria o ledger ali antes do primeiro arquivo (`src/migrate/spec.md` §3), então o
    privilégio tem de ser anterior a ele; e, no PG15+, `public` pertence a `pg_database_owner`, de modo
    que rodando como `wyd_migrator` a revogação de `PUBLIC` seria aviso sem efeito e a concessão a
    `wyd_migrator` falharia com `42501` por falta de grant option — as duas linhas não produziam o
    estado que anunciavam. Como efeito, o `down` não revoga `USAGE` no meio da transação (era o que
    quebrava o `forget` do ledger no reset de `src/migrate/spec.md` §8.1) e o `GRANT` a `PUBLIC` que
    devolvia estado de PG14 deixou de existir. DDL de `ddl-confirmado.md` §3; decisão em
    `ADR-DB-0001`, item 8.

## 9. Prova de execução

PG17 efêmero, sem tocar a série herdada nem o banco de desenvolvimento de ninguém. A prova supõe o
pré-requisito de §3 já aplicado (`wyd-db provision apply`) e a conexão sendo **`wyd_migrator`** — o
preflight do runner (`src/migrate/spec.md` §7) recusa qualquer outro papel; com outro papel, os default
privileges de §7 não valem e as provas 6–8 mentem.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0001_bootstrap.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | 4 schemas + `public` | `\dn` | `content`, `game`, `ops`, `portal`, `public` |
| 2 | `public` sem escrita para todos (estado do provisionamento) | `SELECT has_schema_privilege('public', 'public', 'CREATE');` | `f` |
| 3 | migrator escreve o ledger em `public` (idem) | `SELECT has_schema_privilege('wyd_migrator', 'public', 'CREATE');` | `t` |
| 4 | portal atravessa `game`? | `SELECT has_schema_privilege('wyd_portal', 'game', 'USAGE');` | `f` |
| 5 | jogo lê `portal` cru? | `SELECT has_schema_privilege('wyd_game', 'portal', 'USAGE');` | `f` |
| 6 | default privileges gravados | `SELECT defaclnamespace::regnamespace, defaclobjtype, array_to_string(defaclacl, ', ') FROM pg_default_acl ORDER BY 1;` | 4 linhas, uma por schema, `defaclobjtype = r` |
| 7 | a tabela herdou o `SELECT` de `content` | `SET ROLE wyd_read; SELECT version FROM content.content_version;` | erro `0 rows` (tabela vazia), **não** `permission denied` |
| 8 | runtime não escreve conteúdo | `SET ROLE wyd_game; INSERT INTO content.content_version (id, version) VALUES (1, 1);` | `42501 insufficient_privilege` |
| 9 | linha única | `INSERT INTO content.content_version (id, version) VALUES (2, 1);` | `23514` em `ck_content_version_linha_unica` |
| 10 | versão mínima | `INSERT INTO content.content_version (id, version) VALUES (1, 0);` | `23514` em `ck_content_version_versao` |
| 11 | contagem por schema | `SELECT table_schema, count(*) FROM information_schema.tables WHERE table_schema IN ('content','game','portal','ops') GROUP BY 1;` | `content | 1` |
| 12 | `down` limpa tudo | aplicar o bloco `-- migrate:down` e repetir 11 | 0 linhas |
| 13 | ciclo fecha | refazer `up` após o `down` | prova 1 e 11 de novo, idênticas |
| 14 | `USAGE` de `ops` para quem só acrescenta | `SELECT has_schema_privilege('wyd_game','ops','USAGE'), has_schema_privilege('wyd_portal','ops','USAGE');` | `t \| t` — é o que faz o `GRANT INSERT` de `0002` valer |
| 15 | o `down` não fecha o ledger | aplicar o bloco `-- migrate:down` e então `SELECT count(*) FROM public.schema_migrations;` | `0` — **não** `42501`: prova que o `USAGE` do migrator em `public` não vem da série |

A prova 13 (up → down → up) é o gate de verdade deste arquivo: ele só existe depois que `0001`–`0007`
estiverem inteiros, porque até lá os outros schemas têm menos objeto do que terão.

## 10. O que este arquivo não decide

| Assunto | Onde é decidido |
|---|---|
| Papel e senha reais por ambiente | fora do git; `db/roles.sql` só declara os nomes |
| Ordem, número e vizinhança do arquivo na série | `plano-migrations.md` §2–§3 — a posição desta página é consequência, não decisão dela |
| Conteúdo de `content_version` (qual `version`, qual `note`) | seeds (`0016`–`0027`) |
| Se a série substitui a herdada | **não**; `plano-migrations.md` §6, não autorizado em 2026-11-17 |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

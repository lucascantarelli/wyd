> Pergunta: "como aplico o `0013_content_taxonomy` e como o schema `content` passa a ter taxonomia e dobra de busca?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (arquivo 13) e §3.1; `ddl-confirmado.md` §5.17 (confirmado); `padrao-banco.md` §1 (`P-04`, `P-08`, `P-11`, `P-13`), §3, §4 e §7; `migrations/0008-game-skills.md` §10
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-23)

Décimo terceiro arquivo da série e o primeiro do schema `content`. Cria as **cinco taxonomias** que o
resto do conteúdo referencia — sistema de jogo, classe, linhagem de skill, categoria de item e efeito de
item — e a **função de dobra** usada nas colunas geradas de busca.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as duas escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **chave natural `slug TEXT` como PK** nas quatro tabelas de look-up e **`item_effect`
    como tabela de look-up**, com os 55 `EF_*` semeados em `0017`. As cinco tabelas entram nas **54 de
    96** com DDL fechado (`ddl-confirmado.md` §1) e esta página está `concluída` em
    `migrations-checklist.md` §2.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0013` (confirmado em 2026-11-20) |
| Arquivo | `crates/wyd-db/migrations/0013_content_taxonomy.sql` |
| Schema | `content` (5 tabelas + 1 função) |
| Posição | 13 de 34 (abre o bloco de conteúdo, `0013`–`0027`) |
| Depende de | `0001` (schema `content`, `ops.fn_set_updated_at`, default privileges). **Nada** de `portal`/`game`: não há FK de saída |
| É dependência de | `0014` (`fn_fold` em `name_fold`, `item_kind`/`item_effect`), `0015` (classe e linhagem de skill), `0016` (seed das quatro taxonomias) e `0017` (seed dos 55 `EF_*`) |
| Tabelas criadas | 5 (`game_system`, `game_class`, `skill_lineage`, `item_kind`, `item_effect`) |
| Objetos criados | 5 tabelas, 5 índices (um por PK), 5 triggers, 1 função |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Dar ao schema `content` as taxonomias que o catálogo, as skills e o seed referenciam — cada uma com
código estável, nome exibido e aposentadoria sem `DELETE` — e a função `IMMUTABLE` de dobra que
transforma nome em chave de busca indexável.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schema `content`, `ops.fn_set_updated_at()` e default privileges existem.
2. Conexão como `wyd_migrator` (o preflight do runner recusa outro papel, `src/migrate/spec.md` §7).
3. Nenhum arquivo de conteúdo anterior: o `0013` é o primeiro da faixa.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.17, copiado sem alteração. Os dois marcadores ficam **sozinhos na
linha** (`-- migrate:up` e `-- migrate:down`), como `plano-migrations.md` §2 exige.

```sql
-- 0013_content_taxonomy: taxonomias do conteúdo e dobra de busca.
-- Sem FK de saída (plano-migrations.md §3.1, item 5): o schema `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
-- Dobra de busca (P-11): acento e caixa fora, pontas aparadas. IMMUTABLE porque `0014` a usa em coluna
-- gerada indexada. O lower final só vê ASCII, então o resultado não depende da collation do banco.
CREATE FUNCTION content.fn_fold(input TEXT) RETURNS text
LANGUAGE sql
IMMUTABLE
AS $$
    SELECT lower(translate(
        translate(btrim(input),
                  'ÁÀÂÃÄÅÉÈÊËÍÌÎÏÓÒÔÕÖÚÙÛÜÇÑÝ',
                  'AAAAAAEEEEIIIIOOOOOUUUUCNY'),
        'áàâãäåéèêëíìîïóòôõöúùûüçñý',
        'aaaaaaeeeeiiiiooooouuuucny'));
$$;

CREATE TABLE content.game_system (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_game_system PRIMARY KEY (slug),
    CONSTRAINT ck_game_system_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_game_system_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TABLE content.game_class (
    slug        TEXT NOT NULL,
    name        TEXT NOT NULL,
    -- Teto de maestria da classe: `0008` manda ler daqui (`0008-game-skills.md` §10). 320 é o teto
    -- global de `ck_character_skill_maestria`; o default não restringe e a classe só pode reduzir.
    mastery_cap SMALLINT NOT NULL DEFAULT 320,
    enabled     BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_game_class PRIMARY KEY (slug),
    CONSTRAINT ck_game_class_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_game_class_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_game_class_maestria CHECK (mastery_cap BETWEEN 0 AND 320)
);

CREATE TABLE content.skill_lineage (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_skill_lineage PRIMARY KEY (slug),
    CONSTRAINT ck_skill_lineage_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_skill_lineage_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TABLE content.item_kind (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_kind PRIMARY KEY (slug),
    CONSTRAINT ck_item_kind_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_item_kind_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TABLE content.item_effect (
    slug       TEXT NOT NULL,
    name       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_effect PRIMARY KEY (slug),
    CONSTRAINT ck_item_effect_slug CHECK (slug ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_item_effect_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

CREATE TRIGGER trg_game_system_set_updated_at
    BEFORE UPDATE ON content.game_system
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_game_class_set_updated_at
    BEFORE UPDATE ON content.game_class
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_skill_lineage_set_updated_at
    BEFORE UPDATE ON content.skill_lineage
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_item_kind_set_updated_at
    BEFORE UPDATE ON content.item_kind
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TRIGGER trg_item_effect_set_updated_at
    BEFORE UPDATE ON content.item_effect
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

## 5. DDL do `down`

Ordem inversa exata: cada trigger cai antes da tabela dele, a função cai por último.

```sql
-- migrate:down
DROP TRIGGER trg_item_effect_set_updated_at ON content.item_effect;
DROP TABLE content.item_effect;
DROP TRIGGER trg_item_kind_set_updated_at ON content.item_kind;
DROP TABLE content.item_kind;
DROP TRIGGER trg_skill_lineage_set_updated_at ON content.skill_lineage;
DROP TABLE content.skill_lineage;
DROP TRIGGER trg_game_class_set_updated_at ON content.game_class;
DROP TABLE content.game_class;
DROP TRIGGER trg_game_system_set_updated_at ON content.game_system;
DROP TABLE content.game_system;
DROP FUNCTION content.fn_fold(text);
```

## 6. Tabelas de colunas

As cinco tabelas compartilham o esqueleto de look-up: chave `slug`, nome exibido, `enabled` e carimbos.
Só `game_class` acrescenta coluna de regra.

### 6.1 `content.game_system`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | Identificador estável do sistema (`snake_case`, 2–32). Chave. |
| `name` | `TEXT` | não | — | Nome exibido, 1–64, em pt-BR. |
| `enabled` | `BOOLEAN` | não | `true` | `false` aposenta a linha **sem** `DELETE`, então a FK de arquivo posterior nunca quebra. |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | Criação da linha. |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | Última edição; mantido por trigger. |

### 6.2 `content.game_class`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | Identificador da classe (`snake_case`, 2–32). Chave. |
| `name` | `TEXT` | não | — | Nome exibido, 1–64. |
| `mastery_cap` | `SMALLINT` | não | `320` | Teto de maestria da classe, 0–320. `0008-game-skills.md` §10 manda ler daqui. |
| `enabled` | `BOOLEAN` | não | `true` | Aposentadoria sem `DELETE`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos; `updated_at` por trigger. |

### 6.3 `content.skill_lineage`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | Código da linhagem (`snake_case`, 2–32). Chave. É o **mesmo domínio** de `game.character_lineage.lineage_code` (`0008`), que não tem FK por desenho. |
| `name` | `TEXT` | não | — | Nome exibido, 1–64. |
| `enabled` | `BOOLEAN` | não | `true` | Aposentadoria sem `DELETE`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos. |

### 6.4 `content.item_kind`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | Categoria de item (`snake_case`, 2–32). Chave. O catálogo de `0014` guarda este código como `kind_slug`. |
| `name` | `TEXT` | não | — | Nome exibido, 1–64. |
| `enabled` | `BOOLEAN` | não | `true` | Aposentadoria sem `DELETE`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos. |

### 6.5 `content.item_effect`

| Coluna | Tipo | Nulo | Default | Significado |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | Efeito em `snake_case`, 2–32, derivado do constante do cliente: `EF_HUMAN` → `ef_human`. Chave. |
| `name` | `TEXT` | não | — | Nome exibido, 1–64. |
| `enabled` | `BOOLEAN` | não | `true` | Aposentadoria sem `DELETE`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Carimbos. |

## 7. Contratos e invariantes

| Constraint | Proíbe | Erro |
|---|---|---|
| `pk_game_system` (e as quatro irmãs) | duas linhas com o mesmo `slug` | `23505` |
| `ck_*_slug` (uma por tabela) | `slug` fora de `^[a-z][a-z0-9_]{1,31}$`: maiúscula, hífen, 1 caractere ou 33+ | `23514` |
| `ck_*_nome` (uma por tabela) | nome vazio, só espaço, ou acima de 64 caracteres | `23514` |
| `ck_game_class_maestria` | `mastery_cap` fora de 0–320 | `23514` |
| `content.fn_fold` | — (não é constraint): a dobra não depende da collation, porque troca os acentos antes do `lower` | — |

Nenhuma FK nasce neste arquivo: as tabelas são **alvo**, não filhas. Quem aponta para elas chega em
`0014` (`item_catalog`) e no seed de `0016`/`0017`.

## 8. Permissões

Nenhum `GRANT`/`REVOKE` novo: vale o default privilege de `0001` §3, que dá `SELECT` em `content` aos
três papéis de leitura.

| Papel | Efeito |
|---|---|
| `wyd_read` | `SELECT` nas cinco tabelas — leitura crua para relatório e para o `wyd-db`. |
| `wyd_game` | `SELECT`; **sem** `INSERT`/`UPDATE`/`DELETE`: taxonomia não é estado de jogo. |
| `wyd_portal` | `SELECT`; **sem** escrita — não existe rota de CMS para taxonomia (o CMS de `0030` escreve só em `portal`). |
| `wyd_migrator` | dono das tabelas e único com `INSERT`: aplica este arquivo e os seeds `0016`/`0017`. |

Editar taxonomia é **ato de release**, não de operação: entra por seed gerado e revisado. Se um dia
virar tela administrativa, é lote novo com `GRANT` explícito — este arquivo não inventa papel nem
permissão de escrita.

## 9. Prova de execução

1. `psql -v ON_ERROR_STOP=1 -f 0013_content_taxonomy.sql` como `wyd_migrator` → `CREATE FUNCTION`, `CREATE TABLE` ×5, `CREATE TRIGGER` ×5.
2. `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'content';` → `6` (as 5 + `content_version` do `0001`).
3. `SET ROLE wyd_migrator; INSERT INTO content.item_kind (slug, name) VALUES ('arma', 'Arma');` → `INSERT 0 1`, com `enabled = true`.
4. Repetir o mesmo `slug` → `23505` em `pk_item_kind`.
5. `INSERT INTO content.item_kind (slug, name) VALUES ('Arma', 'Arma');` → `23514` em `ck_item_kind_slug`.
6. `SELECT content.fn_fold('  Água Fria ');` → `agua fria`.
7. `SET ROLE wyd_game; SELECT count(*) FROM content.game_class;` → número; `INSERT INTO content.game_class (slug, name) VALUES ('x', 'X');` → `42501 permission denied for table game_class`.
8. `SET ROLE wyd_portal; INSERT INTO content.game_system (slug, name) VALUES ('x', 'X');` → `42501`.
9. Aplicar o `down` e repetir 2 → `1`; `SELECT to_regprocedure('content.fn_fold(text)');` → `NULL`.

## 10. O que este arquivo não decide

| Assunto | Onde fecha |
|---|---|
| Conteúdo das quatro taxonomias de look-up (20 sistemas, 6 classes, 13 linhagens, 16 categorias) | `0016_seed_taxonomy` |
| Os 55 `EF_*` de `item_effect` | `0017_seed_item_catalog` |
| Vínculo item × efeito (`item_catalog.effect_names`) | `0014_content_item` |
| Número de categoria no fio, se o cliente tiver um | `0014`/cliente; este arquivo **não** cria coluna numérica |
| Evolução de classe (`evolution_of`) | produto, sem decisão registrada |
| Edição de taxonomia por tela administrativa | hoje é seed de release; se virar tela, lote novo com `GRANT` |
| Tradução dos nomes | fora da `1.0.0`: o `name` é pt-BR |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

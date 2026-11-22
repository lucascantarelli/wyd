> Pergunta: "como aplico o `0014_content_item` e como o catálogo de item do cliente vira tabela `content`?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (arquivo 14) e §3.1; `ddl-confirmado.md` §5.18 (confirmado); `padrao-banco.md` §1 (`P-04`, `P-07`, `P-09`, `P-11`), §4 e §7; `migrations/0013-content-taxonomy.md` §7 e §10
> Revisado em: 2026-11-20

Décimo quarto arquivo da série e o segundo do schema `content`. Cria **uma** tabela: o catálogo de itens
que o cliente entrega (3.220 linhas no seed de `0017`), alvo das FKs que `0009`, `0011` e `0012`
deixaram para trás por ordem de arquivo.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as três escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **só as colunas com consumidor provado** (visual e `nUnique`/`nPos`/`Extra` ficam
    fora), **`kind_slug` anulável** (`NULL` = categoria não provada) e **`item_index INTEGER` como PK**.
    A tabela entra na conta de §1 (55 de 96 na época; hoje 56) e a página `migrations/0014-content-item.md` fica
    `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0014` (confirmado em 2026-11-20) |
| Arquivo | `crates/wyd-db/migrations/0014_content_item.sql` |
| Schema | `content` (1 tabela + 1 função de trigger) |
| Posição | 14 de 34 (segundo do bloco de conteúdo, `0013`–`0027`) |
| Depende de | `0001` (schema `content`, `ops.fn_set_updated_at`, default privileges) e `0013` (`content.fn_fold`, `content.item_kind`, `content.item_effect`). **Nada** de `portal`/`game`: não há FK de saída |
| É dependência de | `0017` (seed dos 3.220 itens) e da migração aditiva que trouxer as FKs de `0009`/`0011`/`0012` |
| Tabelas criadas | 1 (`item_catalog`) |
| Objetos criados | 1 tabela, 3 índices (PK, categoria e GIN), 2 triggers, 1 função |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar o que o cliente sabe de cada item — nome, categoria, efeitos, preço-base, grau e requisitos —
numa tabela que o servidor lê para validar equipar e usar, e que o portal lê para a wydpedia. O número
do cliente (`item_index`) é a chave: é o que as instâncias, os preços e a fila de entrega já carregam.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schema `content`, `ops.fn_set_updated_at()` e default privileges existem.
2. `0013_content_taxonomy` aplicado: `content.fn_fold(text)` (a coluna gerada depende dela),
   `content.item_kind` (a FK de categoria) e `content.item_effect` (o trigger de efeito).
3. Conexão como `wyd_migrator` (o preflight do runner recusa outro papel, `src/migrate/spec.md` §7).

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.18, copiado sem alteração. Os dois marcadores ficam **sozinhos na
linha** (`-- migrate:up` e `-- migrate:down`), como `plano-migrations.md` §2 exige.

```sql
-- 0014_content_item: catálogo de itens do cliente (3.220 linhas no seed de 0017).
-- Sem FK de saída (plano-migrations.md §3.1, item 5): `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.item_catalog (
    item_index   INTEGER NOT NULL,
    name         TEXT NOT NULL,
    -- P-11: dobra única no banco, indexável. `content.fn_fold` é IMMUTABLE desde o `0013`.
    name_fold    TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    -- NULL = categoria não provada. Categoria sintética no seed seria inventar dado.
    kind_slug    TEXT,
    -- Símbolos `EF_*`. `TEXT[]` não aceita FK: quem garante o conjunto é o trigger abaixo.
    effect_names TEXT[] NOT NULL DEFAULT '{}',
    base_price   BIGINT NOT NULL DEFAULT 0,
    -- Número do cliente, não letra: a correspondência N/M/A/Le/Anct/E não está provada.
    grade        SMALLINT NOT NULL,
    req_level    SMALLINT NOT NULL DEFAULT 0,
    req_str      SMALLINT NOT NULL DEFAULT 0,
    req_int      SMALLINT NOT NULL DEFAULT 0,
    req_dex      SMALLINT NOT NULL DEFAULT 0,
    req_con      SMALLINT NOT NULL DEFAULT 0,
    -- Aposentadoria sem DELETE: linha referenciada por character_item/item_price não pode sumir.
    enabled      BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_catalog PRIMARY KEY (item_index),
    CONSTRAINT ck_item_catalog_indice CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_item_catalog_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_item_catalog_preco CHECK (base_price >= 0),
    CONSTRAINT ck_item_catalog_grade CHECK (grade >= 0),
    CONSTRAINT ck_item_catalog_requisitos CHECK (
        req_level BETWEEN 0 AND 32767 AND req_str BETWEEN 0 AND 32767
        AND req_int BETWEEN 0 AND 32767 AND req_dex BETWEEN 0 AND 32767
        AND req_con BETWEEN 0 AND 32767),
    CONSTRAINT ck_item_catalog_efeitos CHECK (array_position(effect_names, NULL) IS NULL),
    CONSTRAINT fk_item_catalog_kind FOREIGN KEY (kind_slug)
        REFERENCES content.item_kind (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Efeito repetido ou símbolo fora de `content.item_effect` não passam: sem isto o GIN indexa lixo.
CREATE FUNCTION content.fn_item_catalog_effects() RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    faltando TEXT;
BEGIN
    IF cardinality(NEW.effect_names) = 0 THEN
        RETURN NEW;
    END IF;
    IF cardinality(NEW.effect_names)
       <> (SELECT count(DISTINCT e) FROM unnest(NEW.effect_names) AS t(e)) THEN
        RAISE EXCEPTION 'item % repete efeito em effect_names', NEW.item_index
            USING ERRCODE = '23514', CONSTRAINT = 'ck_item_catalog_efeitos';
    END IF;
    SELECT e INTO faltando
      FROM unnest(NEW.effect_names) AS t(e)
     WHERE NOT EXISTS (SELECT 1 FROM content.item_effect ie WHERE ie.slug = t.e)
     LIMIT 1;
    IF FOUND THEN
        RAISE EXCEPTION 'efeito % nao existe em content.item_effect', faltando
            USING ERRCODE = '23503', CONSTRAINT = 'fk_item_catalog_efeito';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_item_catalog_effects
    BEFORE INSERT OR UPDATE OF effect_names ON content.item_catalog
    FOR EACH ROW EXECUTE FUNCTION content.fn_item_catalog_effects();

CREATE TRIGGER trg_item_catalog_set_updated_at
    BEFORE UPDATE ON content.item_catalog
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- Busca por prefixo do nome dobrado, filtro por categoria e filtro por efeito (P-11 e o GIN do plano).
CREATE INDEX ix_item_catalog_name_fold ON content.item_catalog (name_fold text_pattern_ops);
CREATE INDEX ix_item_catalog_kind ON content.item_catalog (kind_slug);
CREATE INDEX ix_item_catalog_effect_names ON content.item_catalog USING GIN (effect_names);
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP TRIGGER trg_item_catalog_set_updated_at ON content.item_catalog;
DROP TRIGGER trg_item_catalog_effects ON content.item_catalog;
DROP TABLE content.item_catalog;
DROP FUNCTION content.fn_item_catalog_effects();
```

## 6. Tabela de colunas

### 6.1 `content.item_catalog`

| Coluna | Tipo | Nulo | Default | Origem e significado |
|---|---|---|---|---|
| `item_index` | `INTEGER` | não | — | `V`: índice do item no cliente (`STRUCT_ITEM.sIndex`, `Basedef.h:502`); é a identidade (`P-04`) e o valor que `0009`/`0011`/`0012` já carregam. |
| `name` | `TEXT` | não | — | `V`: campo `Name` do `STRUCT_ITEMLIST` (`Basedef.h:1164`); nome exibido, 1–64 caracteres depois de aparado. |
| `name_fold` | `TEXT` | gerada | — | `V`: `P-11` — dobra por `content.fn_fold` (`0013`), sem depender da collation; é a chave de busca por prefixo. |
| `kind_slug` | `TEXT` | sim | — | `P`: categoria de `content.item_kind` (`0013`). `NULL` = categoria não provada; o seed de `0017` só preenche o que estiver mapeado. |
| `effect_names` | `TEXT[]` | não | `'{}'` | `P`: símbolos `EF_*` de `content.item_effect` (`0013` §10), na ordem em que o item os aplica; validados pelo trigger. |
| `base_price` | `BIGINT` | não | `0` | `V`: `P-07` (inteiro na menor unidade) e §5.14, escolha 3 — preço de catálogo mora aqui, não em `game.item_price`, que é por mercador. |
| `grade` | `SMALLINT` | não | — | `P`: número do grau do cliente, sem tradução para letra; o mapeamento N/M/A/Le/Anct/E não está provado e este lote não o inventa. |
| `req_level` | `SMALLINT` | não | `0` | `V`: campo `ReqLvl` do `STRUCT_ITEMLIST` (`Basedef.h:1168`); 0 = sem exigência. |
| `req_str` | `SMALLINT` | não | `0` | `V`: campo `ReqStr` (`Basedef.h:1169`). |
| `req_int` | `SMALLINT` | não | `0` | `V`: campo `ReqInt` (`Basedef.h:1170`). |
| `req_dex` | `SMALLINT` | não | `0` | `V`: campo `ReqDex` (`Basedef.h:1171`). |
| `req_con` | `SMALLINT` | não | `0` | `V`: campo `ReqCon` (`Basedef.h:1172`). |
| `enabled` | `BOOLEAN` | não | `true` | `P`: aposentar sem `DELETE`; a linha continua válida para `character_item` e `item_price`. |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | `V`: padrão da série (`padrao-banco.md` §4). |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | `V`: carimbado por `trg_item_catalog_set_updated_at`. |

## 7. Contratos e invariantes

| Nome | Garante | Erro |
|---|---|---|
| `pk_item_catalog` | um item por `item_index` | `23505` |
| `ck_item_catalog_indice` | `item_index` entre 0 e 32767 (o `short` do cliente) | `23514` |
| `ck_item_catalog_nome` | nome vazio, só espaço ou acima de 64 caracteres | `23514` |
| `ck_item_catalog_preco` | `base_price` negativo | `23514` |
| `ck_item_catalog_grade` | `grade` negativo | `23514` |
| `ck_item_catalog_requisitos` | qualquer requisito fora de 0–32767 | `23514` |
| `ck_item_catalog_efeitos` | `NULL` dentro de `effect_names` | `23514` |
| `fk_item_catalog_kind` | `kind_slug` que não existe em `content.item_kind` | `23503` |
| `trg_item_catalog_effects` | efeito repetido (`23514`, nomeando `ck_item_catalog_efeitos`) ou símbolo fora de `content.item_effect` (`23503`, nomeando `fk_item_catalog_efeito`) | `23514`/`23503` |
| `trg_item_catalog_set_updated_at` | carimbo de `updated_at` em todo `UPDATE` | — |

**O que o banco não expressa.** Nome não é chave: dois itens podem ter o mesmo nome, então não há
`UNIQUE (name)` — quem identifica é `item_index`. Categoria nula é ausência de prova, não erro. A
integridade de `effect_names` é de trigger, porque `TEXT[]` não aceita FK; trocar o `slug` de uma linha
de `content.item_effect` depois do seed não é coberto por `ON UPDATE RESTRICT` — aposentar efeito é
`enabled = false`, como `0013` §7 diz. As FKs das tabelas filhas (`character_item`, `vault_item`,
`item_price`, `delivery_queue`) não nascem aqui: os três arquivos fecharam sem elas, e acrescentá-las é
migração aditiva.

## 8. Permissões

Nenhum `GRANT`/`REVOKE` novo: vale o default privilege de `0001` §3, que dá `SELECT` em `content` aos
três papéis de leitura.

| Papel | Efeito |
|---|---|
| `wyd_read` | `SELECT` — leitura crua para relatório e para o portal. |
| `wyd_game` | `SELECT`; **sem** escrita: catálogo é conteúdo de release, não estado de jogo. |
| `wyd_portal` | `SELECT`; **sem** escrita — o CMS de `0030` escreve em `portal`, não em `content`. |
| `wyd_migrator` | dono da tabela e único com `INSERT`: aplica este arquivo e o seed `0017`. |

## 9. Prova de execução

1. `psql -v ON_ERROR_STOP=1 -f 0014_content_item.sql` como `wyd_migrator` → `CREATE TABLE`, `CREATE FUNCTION`, `CREATE TRIGGER` ×2, `CREATE INDEX` ×3.
2. `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'content';` → `7` (as 5 do `0013` + `content_version` + esta).
3. `INSERT INTO content.item_catalog (item_index, name, kind_slug, effect_names, grade) VALUES (1, 'Poção de Vida', 'consumivel', '{EF_HP}', 1);` → `INSERT 0 1`, com `name_fold = 'pocao de vida'`.
4. Repetir o mesmo `item_index` → `23505` em `pk_item_catalog`.
5. `INSERT` com `kind_slug = 'inexistente'` → `23503` em `fk_item_catalog_kind`.
6. `INSERT` com `effect_names = '{EF_NAO_EXISTE}'` → `23503` em `fk_item_catalog_efeito`.
7. `INSERT` com `effect_names = '{EF_HP,EF_HP}'` → `23514` em `ck_item_catalog_efeitos`.
8. `INSERT` com `req_level = 40000` → `23514` em `ck_item_catalog_requisitos`.
9. `SET ROLE wyd_game; UPDATE content.item_catalog SET name = 'x';` → `42501 permission denied for table item_catalog`.
10. Aplicar o `down` e repetir 2 → `6`; `SELECT to_regprocedure('content.fn_item_catalog_effects()');` → `NULL`.

## 10. O que este arquivo não decide

| Assunto | Onde fecha |
|---|---|
| Os 3.220 itens: nome, categoria, efeitos, preço, grau e requisitos | `0017_seed_item_catalog` (gerado) |
| Mapeamento grau → letra (N/M/A/Le/Anct/E) | exibição do portal; este lote guarda o número, não traduz |
| Visual do cliente (`IndexMesh`, `IndexTexture`, `IndexVisualEffect`) | migração aditiva, se um consumidor provar uso |
| `nUnique`, `nPos`, `Extra` | idem: sem semântica documentada no legado, não viram coluna |
| FK de `character_item`, `vault_item`, `item_price` e `delivery_queue` para cá | migração aditiva; os três arquivos fecharam sem FK por ordem |
| Sets e bônus de conjunto | sem decisão registrada |
| Tradução dos nomes | fora da `1.0.0`: o `name` é pt-BR |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

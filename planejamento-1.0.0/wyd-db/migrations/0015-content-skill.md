> Pergunta: "como aplico o `0015_content_skill` e como o catálogo de skill do cliente vira tabela `content`?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (arquivo 15) e §3.1; `ddl-confirmado.md` §5.19 (confirmado); `padrao-banco.md` §1 (`P-04`, `P-11`), §4 e §7; `migrations/0013-content-taxonomy.md` §7; `migrations/0008-game-skills.md` §10
> Revisado em: 2026-11-20

Décimo quinto arquivo da série e o terceiro do schema `content`. Cria **uma** tabela: o catálogo de
skills que o cliente entrega (154 linhas no seed de `0018`), com os números de combate que o servidor
lê e os vínculos de classe e linhagem que o portal filtra.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou as três escolhas de forma na rodada de perguntas de 2026-11-20, nas opções
    recomendadas: **`delay` e `affect_time` gravados brutos** (o runtime converte), **`class_slug` e
    `lineage_slug` anuláveis** com FK (`NULL` = vínculo não provado) e **`skill_index SMALLINT` como
    PK** (0–247). A tabela entra na conta de §1 (75 na época; hoje 109 de 109) e a página `migrations/0015-content-skill.md`
    fica `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0015` (confirmado em 2026-11-20) |
| Arquivo | `crates/wyd-db/migrations/0015_content_skill.sql` |
| Schema | `content` (1 tabela) |
| Posição | 15 de 34 (terceiro do bloco de conteúdo, `0013`–`0027`) |
| Depende de | `0001` (schema `content`, `ops.fn_set_updated_at`, default privileges) e `0013` (`content.fn_fold`, `content.game_class`, `content.skill_lineage`). **Nada** de `portal`/`game`: não há FK de saída |
| É dependência de | `0018` (seed das 154 skills) e da migração aditiva que, se um dia houver, ligar progressão ao catálogo |
| Tabelas criadas | 1 (`skill_catalog`) |
| Objetos criados | 1 tabela, 3 índices (PK, prefixo e afinidade), 1 trigger |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar o que o cliente sabe de cada skill — custo, mana, alcance, atraso, alvo, efeito por instante e
por tick, resistência e os três sinalizadores — numa tabela que o servidor lê para executar a skill e
que o portal lê para listar por classe e linhagem. `skill_index` é a chave porque é o número que
`game.character_skill` já carrega.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schema `content`, `ops.fn_set_updated_at()` e default privileges existem.
2. `0013_content_taxonomy` aplicado: `content.fn_fold(text)` (a coluna gerada depende dela),
   `content.game_class` e `content.skill_lineage` (as duas FKs).
3. Conexão como `wyd_migrator` (o preflight do runner recusa outro papel, `src/migrate/spec.md` §7).

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.19, copiado sem alteração. Os dois marcadores ficam **sozinhos na
linha** (`-- migrate:up` e `-- migrate:down`), como `plano-migrations.md` §2 exige.

```sql
-- 0015_content_skill: catálogo de skills do cliente (154 linhas no seed de 0018).
-- Sem FK de saída (plano-migrations.md §3.1, item 5): `content` não aponta para `game`/`portal`.
-- Quem lê são wyd_game, wyd_portal e wyd_read pelo default privilege de `0001`; ninguém escreve em runtime.

-- migrate:up
CREATE TABLE content.skill_catalog (
    skill_index        SMALLINT NOT NULL,
    name               TEXT NOT NULL,
    -- P-11: dobra única no banco, indexável. `content.fn_fold` é IMMUTABLE desde o `0013`.
    name_fold          TEXT GENERATED ALWAYS AS (content.fn_fold(name)) STORED,
    -- NULL = vínculo não provado: das 154 skills, 63 têm linhagem medida.
    class_slug         TEXT,
    lineage_slug       TEXT,
    skill_point        SMALLINT NOT NULL DEFAULT 0,
    mana_spent         SMALLINT NOT NULL DEFAULT 0,
    -- `delay_ticks` e `affect_time` ficam brutos, como vêm do SkillData.csv. O runtime aplica a
    -- conversão que o cliente faz na carga; o catálogo não traduz (mesma regra do `grade` em `0014`).
    delay_ticks        SMALLINT NOT NULL DEFAULT 0,
    affect_time        SMALLINT NOT NULL DEFAULT 0,
    range_tiles        SMALLINT NOT NULL DEFAULT 0,
    target_type        SMALLINT NOT NULL DEFAULT 0,
    instance_type      SMALLINT NOT NULL DEFAULT 0,
    instance_value     SMALLINT NOT NULL DEFAULT 0,
    tick_type          SMALLINT NOT NULL DEFAULT 0,
    tick_value         SMALLINT NOT NULL DEFAULT 0,
    affect_type        SMALLINT NOT NULL DEFAULT 0,
    affect_value       SMALLINT NOT NULL DEFAULT 0,
    instance_attribute SMALLINT NOT NULL DEFAULT 0,
    tick_attribute     SMALLINT NOT NULL DEFAULT 0,
    -- O CSV traz 0/1; BOOLEAN diz o que o campo é. Valor fora de 0/1 seria lote novo.
    aggressive         BOOLEAN NOT NULL DEFAULT false,
    max_target         SMALLINT NOT NULL DEFAULT 1,
    party              BOOLEAN NOT NULL DEFAULT false,
    affect_resist      SMALLINT NOT NULL DEFAULT 0,
    passive            BOOLEAN NOT NULL DEFAULT false,
    -- Aposentadoria sem DELETE: linha referenciada por progressão de personagem não pode sumir.
    enabled            BOOLEAN NOT NULL DEFAULT true,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_skill_catalog PRIMARY KEY (skill_index),
    CONSTRAINT ck_skill_catalog_indice CHECK (skill_index BETWEEN 0 AND 247),
    CONSTRAINT ck_skill_catalog_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64),
    CONSTRAINT ck_skill_catalog_nao_negativos CHECK (
        skill_point >= 0 AND mana_spent >= 0 AND delay_ticks >= 0 AND affect_time >= 0
        AND range_tiles >= 0 AND max_target >= 0),
    CONSTRAINT fk_skill_catalog_class FOREIGN KEY (class_slug)
        REFERENCES content.game_class (slug) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_skill_catalog_lineage FOREIGN KEY (lineage_slug)
        REFERENCES content.skill_lineage (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TRIGGER trg_skill_catalog_set_updated_at
    BEFORE UPDATE ON content.skill_catalog
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- Busca por prefixo do nome dobrado e filtro por classe/linhagem (P-11 e a leitura do portal).
CREATE INDEX ix_skill_catalog_name_fold ON content.skill_catalog (name_fold text_pattern_ops);
CREATE INDEX ix_skill_catalog_afinidade ON content.skill_catalog (class_slug, lineage_slug);
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP TRIGGER trg_skill_catalog_set_updated_at ON content.skill_catalog;
DROP TABLE content.skill_catalog;
```

## 6. Tabela de colunas

### 6.1 `content.skill_catalog`

| Coluna | Tipo | Nulo | Default | Origem e significado |
|---|---|---|---|---|
| `skill_index` | `SMALLINT` | não | — | `V`: `MAX_SKILLINDEX 248` (`Basedef.h:200`) e `game.character_skill.skill_index SMALLINT` 0–247 (`0008`); o índice do cliente é a identidade. |
| `name` | `TEXT` | não | — | `P`: curadoria do seed `0018` — o parser de `SkillData.csv` descarta o nome (`Basedef.cpp:6659`); 1–64 caracteres depois de aparado. |
| `name_fold` | `TEXT` | gerada | — | `V`: `P-11` — dobra por `content.fn_fold` (`0013`); é a chave de busca por prefixo. |
| `class_slug` | `TEXT` | sim | — | `P`: classe de `content.game_class` (`0013`). `NULL` = vínculo não provado; o seed `0018` só preenche o que a curadoria sustentar. |
| `lineage_slug` | `TEXT` | sim | — | `P`: linhagem de `content.skill_lineage` (`0013`). 63 das 154 skills têm linhagem medida (`plano-migrations.md` §4); o resto fica `NULL`. |
| `skill_point` | `SMALLINT` | não | `0` | `V`: campo `SkillPoint` do `STRUCT_SPELL` (`Basedef.h:1112`), gravado como vem do CSV. |
| `mana_spent` | `SMALLINT` | não | `0` | `V`: campo `ManaSpent` (`Basedef.h:1114`). |
| `delay_ticks` | `SMALLINT` | não | `0` | `V`: campo `Delay` (`Basedef.h:1115`), **bruto**: nenhuma conversão no catálogo. |
| `affect_time` | `SMALLINT` | não | `0` | `V`: campo `AffectTime` (`Basedef.h:1123`), **bruto**; o cliente divide por 4 na carga (`Basedef.cpp:6699`) e o runtime novo faz o mesmo fora do banco. |
| `range_tiles` | `SMALLINT` | não | `0` | `V`: campo `Range` (`Basedef.h:1116`). |
| `target_type` | `SMALLINT` | não | `0` | `V`: campo `TargetType` (`Basedef.h:1113`); o significado dos valores é do runtime, não do catálogo. |
| `instance_type` | `SMALLINT` | não | `0` | `V`: campo `InstanceType` (`Basedef.h:1117`). |
| `instance_value` | `SMALLINT` | não | `0` | `V`: campo `InstanceValue` (`Basedef.h:1118`). |
| `tick_type` | `SMALLINT` | não | `0` | `V`: campo `TickType` (`Basedef.h:1119`). |
| `tick_value` | `SMALLINT` | não | `0` | `V`: campo `TickValue` (`Basedef.h:1120`). |
| `affect_type` | `SMALLINT` | não | `0` | `V`: campo `AffectType` (`Basedef.h:1121`). |
| `affect_value` | `SMALLINT` | não | `0` | `V`: campo `AffectValue` (`Basedef.h:1122`). |
| `instance_attribute` | `SMALLINT` | não | `0` | `V`: campo `InstanceAttribute` (`Basedef.h:1125`). |
| `tick_attribute` | `SMALLINT` | não | `0` | `V`: campo `TickAttribute` (`Basedef.h:1126`). |
| `aggressive` | `BOOLEAN` | não | `false` | `V`: campo `Aggressive` (`Basedef.h:1127`), 0/1 no CSV; `BOOLEAN` diz o que o campo é. |
| `max_target` | `SMALLINT` | não | `1` | `V`: campo `MaxTarget` (`Basedef.h:1128`). |
| `party` | `BOOLEAN` | não | `false` | `V`: campo `bParty` (`Basedef.h:1129`). |
| `affect_resist` | `SMALLINT` | não | `0` | `V`: campo `AffectResist` (`Basedef.h:1130`). |
| `passive` | `BOOLEAN` | não | `false` | `V`: campo `Passive` (`Basedef.h:1131`). |
| `enabled` | `BOOLEAN` | não | `true` | `P`: aposentar sem `DELETE`; a linha continua válida para a progressão de personagem. |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | `V`: padrão da série (`padrao-banco.md` §4). |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | `V`: carimbado por `trg_skill_catalog_set_updated_at`. |

## 7. Contratos e invariantes

| Nome | Garante | Erro |
|---|---|---|
| `pk_skill_catalog` | uma skill por `skill_index` | `23505` |
| `ck_skill_catalog_indice` | `skill_index` entre 0 e 247 (o teto do cliente) | `23514` |
| `ck_skill_catalog_nome` | nome vazio, só espaço ou acima de 64 caracteres | `23514` |
| `ck_skill_catalog_nao_negativos` | custo, mana, atraso, tempo, alcance ou alvo negativo | `23514` |
| `fk_skill_catalog_class` | `class_slug` que não existe em `content.game_class` | `23503` |
| `fk_skill_catalog_lineage` | `lineage_slug` que não existe em `content.skill_lineage` | `23503` |
| `trg_skill_catalog_set_updated_at` | carimbo de `updated_at` em todo `UPDATE` | — |

**O que o banco não expressa.** Nome não é chave: duas skills podem ter o mesmo nome, então não há
`UNIQUE (name)` — quem identifica é `skill_index`. `target_type`, `affect_type`, `tick_type` e os
pares `*_value` guardam o número do cliente e nada dizem sobre o que ele significa: a semântica é do
runtime, e o catálogo não traduz. Vínculo nulo é ausência de prova, não erro. Nenhuma FK aponta para
cá: `game.character_skill` cobre 0–247 e o catálogo cobre parte disso, então a FK falharia para skill
fora do catálogo; ligar progressão ao catálogo exigiria migração aditiva com essa decisão.

## 8. Permissões

Nenhum `GRANT`/`REVOKE` novo: vale o default privilege de `0001` §3, que dá `SELECT` em `content` aos
três papéis de leitura.

| Papel | Efeito |
|---|---|
| `wyd_read` | `SELECT` — leitura crua para relatório e para o portal. |
| `wyd_game` | `SELECT`; **sem** escrita: catálogo é conteúdo de release, não estado de jogo. |
| `wyd_portal` | `SELECT`; **sem** escrita — o CMS de `0030` escreve em `portal`, não em `content`. |
| `wyd_migrator` | dono da tabela e único com `INSERT`: aplica este arquivo e o seed `0018`. |

## 9. Prova de execução

1. `psql -v ON_ERROR_STOP=1 -f 0015_content_skill.sql` como `wyd_migrator` → `CREATE TABLE`, `CREATE TRIGGER`, `CREATE INDEX` ×2 (fora o da PK).
2. `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'content';` → `8` (as 5 do `0013` + `item_catalog` + `content_version` + esta).
3. `INSERT INTO content.skill_catalog (skill_index, name, class_slug, skill_point, mana_spent) VALUES (1, 'Golpe', 'transknight', 1, 10);` → `INSERT 0 1`, com `name_fold = 'golpe'`.
4. Repetir o mesmo `skill_index` → `23505` em `pk_skill_catalog`.
5. `INSERT` com `skill_index = 248` → `23514` em `ck_skill_catalog_indice`.
6. `INSERT` com `class_slug = 'inexistente'` → `23503` em `fk_skill_catalog_class`.
7. `INSERT` com `lineage_slug = 'inexistente'` → `23503` em `fk_skill_catalog_lineage`.
8. `INSERT` com `max_target = -1` → `23514` em `ck_skill_catalog_nao_negativos`.
9. `SET ROLE wyd_game; UPDATE content.skill_catalog SET name = 'x';` → `42501 permission denied for table skill_catalog`.
10. Aplicar o `down` e repetir 2 → `7`; `SELECT to_regclass('content.skill_catalog');` → `NULL`.

## 10. O que este arquivo não decide

| Assunto | Onde fecha |
|---|---|
| As 154 linhas: nome, classe, linhagem, custos e efeitos | `0018_seed_skill_catalog` (gerado) |
| Índice provisório das 3 skills da Kersef | `0018`; se colidir com o índice do cliente, a decisão é lá |
| Significado de `target_type`, `affect_type`, `tick_type` e dos `*_value` | runtime do servidor; o catálogo guarda o número |
| `Act[8]` (sequência de ativação do cliente) | fora do banco: animação do cliente |
| FK de `game.character_skill` para cá | migração aditiva, com a decisão de catálogo parcial |
| Tradução dos nomes | fora da `1.0.0`: o `name` é pt-BR |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*

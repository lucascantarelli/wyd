> Pergunta: "como as regras-base de refino, SANC, composição, gemas e acessórios são tipadas e ligadas aos catálogos?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 21), `ddl-confirmado.md` §5.22 (aprovado em 2026-11-20), fichas legadas do domínio refino
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-24)

Vigésimo primeiro arquivo da série: 5 tabelas no schema `content`. As fichas de coluna vêm das
fontes históricas de refino (nível 4); são **hipótese `H`** até validação própria, não colunas provadas
por aparecerem no legado.

!!! note "Aprovado em 2026-11-20"
    O dono respondeu as decisões de §8 e aprovou o conjunto em 2026-11-20. As cinco tabelas entram na
    conta de `ddl-confirmado.md` §1 (75 na época; hoje 109 de 109) e esta página fica `concluída` na checklist. As colunas
    seguem `H` até trace do servidor — aprovar o desenho não as promove a `P`.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0021` (checklist, linha 105) |
| Arquivo | `crates/wyd-db/migrations/0021_content_rules.sql` |
| Schema | `content` (5 tabelas) |
| Posição | 21 de 34 |
| Depende de | `0001` (schema, privilégios); `0013` (`content.item_effect`); `0014` (`content.item_catalog`) |
| É dependência de | `0022` (overrides de três tabelas) e `0023` (seed das regras) |
| Tabelas criadas | `refine_rate`, `refine_gem_effect`, `sanc_rate`, `combine_value`, `accessory_refine_rule` |
| Objetos criados | 5 tabelas, 3 FKs, 1 índice secundário parcial |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Definir a estrutura-base consultável em runtime para regras de refino e composição, mantendo taxas em
bps e custos em `BIGINT`. Os valores concretos entram por seed em `0023`; edição de três famílias é
separada em overrides de `0022`. O acessório é conteúdo posterior ao 7662 e permanece `H`.

## 3. Pré-requisitos

1. `0001_bootstrap`: schema `content` e default privileges (SELECT para `wyd_game`, `wyd_portal`, `wyd_read`).
2. `0013_content_taxonomy`: `content.item_effect(slug)`.
3. `0014_content_item`: `content.item_catalog(item_index)`.
4. Aplicação posterior de `0022`/`0023` respeita a ordem da série; nenhuma tabela posterior é FK deste lote.

## 4. DDL do `up`

Texto proposto em `ddl-confirmado.md` §5.22, copiado sem alteração.

```sql
-- 0021_content_rules: regras base de refino, SANC, composição, gemas e acessórios.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabelas base são seed-only e read-only em runtime; overrides das três tabelas previstas ficam em 0022.

-- migrate:up
CREATE TABLE content.refine_rate (
    kind      TEXT NOT NULL,
    step      SMALLINT NOT NULL,
    rate_bps  INTEGER NOT NULL,
    CONSTRAINT pk_refine_rate PRIMARY KEY (kind, step),
    CONSTRAINT ck_refine_rate_kind CHECK (kind IN ('dust', 'compose')),
    CONSTRAINT ck_refine_rate_step CHECK (step BETWEEN 1 AND 15),
    CONSTRAINT ck_refine_rate_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- effect_code references the already-closed editable lookup (0013); item_index is the gem's natural key.
CREATE TABLE content.refine_gem_effect (
    item_index       INTEGER NOT NULL,
    effect_code      TEXT NOT NULL,
    flat_value       SMALLINT NOT NULL DEFAULT 0,
    per_level_value  SMALLINT NOT NULL DEFAULT 0,
    min_refine       SMALLINT NOT NULL DEFAULT 10,
    max_refine       SMALLINT NOT NULL DEFAULT 15,
    allows_ancient   BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_refine_gem_effect PRIMARY KEY (item_index),
    CONSTRAINT ck_refine_gem_effect_item CHECK (item_index BETWEEN 0 AND 32767),
    CONSTRAINT ck_refine_gem_effect_valores CHECK (flat_value >= 0 AND per_level_value >= 0),
    CONSTRAINT ck_refine_gem_effect_faixa CHECK (
        min_refine BETWEEN 0 AND 15 AND max_refine BETWEEN min_refine AND 15),
    CONSTRAINT fk_refine_gem_effect_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_refine_gem_effect_codigo FOREIGN KEY (effect_code)
        REFERENCES content.item_effect (slug) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE content.sanc_rate (
    powder        TEXT NOT NULL,
    refine_index  SMALLINT NOT NULL,
    rate_bps      INTEGER NOT NULL,
    CONSTRAINT pk_sanc_rate PRIMARY KEY (powder, refine_index),
    CONSTRAINT ck_sanc_rate_powder CHECK (powder IN ('PO', 'PL', 'AMAGO')),
    CONSTRAINT ck_sanc_rate_index CHECK (refine_index BETWEEN 0 AND 15),
    CONSTRAINT ck_sanc_rate_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- Exactly one typed value is present: rates are bps, coin amounts are BIGINT minor units.
CREATE TABLE content.combine_value (
    family    TEXT NOT NULL,
    key       TEXT NOT NULL,
    rate_bps  INTEGER,
    coin      BIGINT,
    CONSTRAINT pk_combine_value PRIMARY KEY (family, key),
    CONSTRAINT ck_combine_value_family CHECK (char_length(btrim(family)) BETWEEN 1 AND 64),
    CONSTRAINT ck_combine_value_key CHECK (char_length(btrim(key)) BETWEEN 1 AND 96),
    CONSTRAINT ck_combine_value_one_unit CHECK (
        (rate_bps IS NOT NULL AND coin IS NULL AND rate_bps BETWEEN 0 AND 10000)
        OR (rate_bps IS NULL AND coin IS NOT NULL AND coin >= 0)
    )
);

CREATE TABLE content.accessory_refine_rule (
    kind              TEXT NOT NULL,
    recipe            TEXT NOT NULL,
    step_from         SMALLINT NOT NULL,
    step_to           SMALLINT NOT NULL,
    rate_bps          INTEGER NOT NULL,
    fail_mode         TEXT NOT NULL,
    fail_bonus_item   INTEGER,
    fail_bonus_bps    INTEGER NOT NULL DEFAULT 0,
    gold_cost         BIGINT NOT NULL DEFAULT 0,
    material_item     INTEGER,
    material_name     TEXT NOT NULL DEFAULT '',
    material_qty      SMALLINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_accessory_refine_rule PRIMARY KEY (kind, recipe, step_from),
    CONSTRAINT ck_accessory_refine_rule_kind CHECK (
        kind IN ('earring', 'necklace', 'belt', 'amulet', 'ankh', 'spirit_stone')),
    CONSTRAINT ck_accessory_refine_rule_recipe CHECK (char_length(btrim(recipe)) BETWEEN 1 AND 96),
    CONSTRAINT ck_accessory_refine_rule_step CHECK (
        step_from BETWEEN 0 AND 14 AND step_to = step_from + 1),
    CONSTRAINT ck_accessory_refine_rule_rate CHECK (rate_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_accessory_refine_rule_fail CHECK (fail_mode IN ('keep', 'reset', 'destroy')),
    CONSTRAINT ck_accessory_refine_rule_bonus CHECK (
        fail_bonus_bps BETWEEN 0 AND 10000 AND
        (fail_bonus_item IS NOT NULL OR fail_bonus_bps = 0)),
    CONSTRAINT ck_accessory_refine_rule_gold CHECK (gold_cost >= 0),
    CONSTRAINT ck_accessory_refine_rule_material CHECK (
        material_item IS NULL OR material_item BETWEEN 0 AND 32767),
    CONSTRAINT ck_accessory_refine_rule_material_name CHECK (char_length(material_name) <= 96),
    CONSTRAINT ck_accessory_refine_rule_material_qty CHECK (material_qty > 0),
    CONSTRAINT fk_accessory_refine_rule_bonus FOREIGN KEY (fail_bonus_item)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- P-15: index the nullable child FK. PKs cover the other FK access paths; item_effect is lookup-only.
CREATE INDEX ix_accessory_refine_rule_fail_bonus_item
    ON content.accessory_refine_rule (fail_bonus_item)
    WHERE fail_bonus_item IS NOT NULL;
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP INDEX content.ix_accessory_refine_rule_fail_bonus_item;
DROP TABLE content.accessory_refine_rule;
DROP TABLE content.combine_value;
DROP TABLE content.sanc_rate;
DROP TABLE content.refine_gem_effect;
DROP TABLE content.refine_rate;
```

Sem `COMMENT ON`; sem `GRANT`/`REVOKE`, pois as tabelas são seed-only em runtime e herdam os
privilégios de leitura de `0001`.

## 6. Colunas e origem

`P` = decisão deste planejamento aprovada pelo dono; `H` = hipótese de conteúdo da ficha do legado;
`padrão` = tipo/regra já normatizado em `padrao-banco.md`. Valores de seed não foram copiados.

### 6.1 `content.refine_rate` — 3 colunas

| Coluna | Tipo | Nulo | Default | Origem | Descrição |
|---|---|---|---|---|---|
| `kind` | `TEXT` | não | — | H | Família `dust`/`compose`; conjunto fechado proposto. |
| `step` | `SMALLINT` | não | — | H | Passo 1–15 da regra. |
| `rate_bps` | `INTEGER` | não | — | H + padrão | Taxa de 0–10000 bps; sem taxa publicada não há linha. |

### 6.2 `content.refine_gem_effect` — 7 colunas

| Coluna | Tipo | Nulo | Default | Origem | Descrição |
|---|---|---|---|---|---|
| `item_index` | `INTEGER` | não | — | H | Índice natural da gema; FK para `item_catalog`. |
| `effect_code` | `TEXT` | não | — | H | FK para `item_effect.slug` (`0013`). |
| `flat_value` | `SMALLINT` | não | `0` | H | Valor fixo do efeito. |
| `per_level_value` | `SMALLINT` | não | `0` | H | Valor por nível. |
| `min_refine` | `SMALLINT` | não | `10` | H | Refino mínimo aplicável. |
| `max_refine` | `SMALLINT` | não | `15` | H | Refino máximo aplicável. |
| `allows_ancient` | `BOOLEAN` | não | `true` | H | Se a regra aceita item Ancient. |

### 6.3 `content.sanc_rate` — 3 colunas

| Coluna | Tipo | Nulo | Default | Origem | Descrição |
|---|---|---|---|---|---|
| `powder` | `TEXT` | não | — | H | Chave antiga de pó (`PO`, `PL`, `AMAGO`), conjunto proposto. |
| `refine_index` | `SMALLINT` | não | — | H | Índice do refino. |
| `rate_bps` | `INTEGER` | não | — | H + padrão | Chance de 0–10000 bps; conversão de % fica a cargo do seed. |

### 6.4 `content.combine_value` — 4 colunas

| Coluna | Tipo | Nulo | Default | Origem | Descrição |
|---|---|---|---|---|---|
| `family` | `TEXT` | não | — | H | Família da regra. |
| `key` | `TEXT` | não | — | H | Chave estável dentro da família. |
| `rate_bps` | `INTEGER` | sim | — | H + padrão | Taxa, presente quando a unidade é percentual. |
| `coin` | `BIGINT` | sim | — | H + padrão | Custo em moeda, presente quando a unidade é moeda. |

### 6.5 `content.accessory_refine_rule` — 12 colunas

| Coluna | Tipo | Nulo | Default | Origem | Descrição |
|---|---|---|---|---|---|
| `kind` | `TEXT` | não | — | H | Tipo de acessório. |
| `recipe` | `TEXT` | não | — | H | Nome da receita. |
| `step_from` / `step_to` | `SMALLINT` | não | — | H | Passo de origem/destino (destino = origem + 1). |
| `rate_bps` | `INTEGER` | não | — | H + padrão | Taxa de 0–10000 bps. |
| `fail_mode` | `TEXT` | não | — | H | `keep`, `reset` ou `destroy`. |
| `fail_bonus_item` | `INTEGER` | sim | — | H | Item bônus opcional; FK `RESTRICT` para `item_catalog`. |
| `fail_bonus_bps` | `INTEGER` | não | `0` | H + padrão | Chance do bônus. |
| `gold_cost` | `BIGINT` | não | `0` | H + padrão | Custo em moeda. |
| `material_item` | `INTEGER` | sim | — | H | Índice de material sem FK; pode não existir no catálogo 7662. |
| `material_name` | `TEXT` | não | `''` | H | Rótulo publicado do material. |
| `material_qty` | `SMALLINT` | não | `1` | H + padrão | Quantidade exigida. |

## 7. Constraints, FKs e índices

- PKs naturais: `(kind, step)`, `item_index`, `(powder, refine_index)`, `(family, key)`,
  `(kind, recipe, step_from)`.
- CHECKs nomeados limitam conjuntos e faixas; `combine_value` exige exatamente uma de `rate_bps` e
  `coin`; `step_to = step_from + 1`; bônus positivo exige `fail_bonus_item`.
- FKs `RESTRICT`: `refine_gem_effect.item_index → item_catalog`, `effect_code → item_effect`,
  `accessory_refine_rule.fail_bonus_item → item_catalog`.
- Índice secundário parcial só em `accessory_refine_rule.fail_bonus_item`; PKs atendem buscas naturais.
- Não há `updated_at`/`updated_by`: base seed-only, nenhuma escrita do staff em runtime aprovada.

## 8. Decisões aprovadas em 2026-11-20

1. Acessório posterior ao 7662: manter `accessory_refine_rule` no lote; seed futuro limitado às receitas do Bahamut como `H`.
2. Remover `content_source`, `confidence`, `notes`: proveniência é por dataset em `0019`; `H` vive na página; escrita/edição futura exige contrato próprio.
3. `combine_value`: duas colunas tipadas `rate_bps`/`coin`, exatamente uma preenchida.
4. SANC: `rate_bps INTEGER` 0–10000.
5. Gema: FK `item_index` para catálogo com `ON DELETE RESTRICT`.
6. Material de acessório: manter `material_item` e `material_name`, sem FK em `material_item`.
7. `refine_rate` PK natural `(kind, step)`.
8. FK `refine_gem_effect.effect_code → item_effect.slug`, `RESTRICT`.
9. Acessório base é seed-only/read-only; sem override novo em `0022`.
10. Omitir linha sem taxa publicada; não gravar zero nem `NULL`.
11. SANC seed só de células com fonte explícita; defaults compilados ficam fora do seed.
12. `fail_bonus_item` FK nullable `RESTRICT`.
13. Overrides separados em `0022`; base e carga em `0023`.
14. Remover `updated_by`/`updated_at` da tabela de acessório.
15. Índices só quando necessários às FKs e às chaves; nenhum preventivo em lookup somente-leitura.

## 9. Prova de execução

As consultas abaixo não foram executadas uma a uma; em 2026-11-24 rodaram a aplicação da série
do zero e os 13 cenários de `sistema_pg` (`planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md`
§6.6.30).

1. PK duplicada em cada tabela → `23505`.
2. `rate_bps` < 0 ou > 10000 → `23514`.
3. Ambas ou nenhuma das colunas `rate_bps`/`coin` preenchidas → `23514`.
4. Efeito inexistente, item de gema inexistente ou bônus fora de `item_catalog` → `23503`.
5. Remover efeito/item referenciado → `23503` por `RESTRICT`.
6. `fail_bonus_bps > 0` sem `fail_bonus_item` → `23514`.
7. `step_to != step_from + 1` → `23514`.
8. Papel `wyd_game`/`wyd_portal` tenta `INSERT` em tabela base → `42501`.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| Valores das taxas/regras e quais linhas entram nos seeds | `0023`, só com fontes verificadas; material legado segue `H` |
| Ausência de taxa no runtime e fallback de leitura | contrato do `wyd-sim`/`wyd-server`; este DDL não converte ausência em zero |
| Overrides e `COALESCE(override, base)` | `0022` |
| Edição de acessório pelo staff | não aprovada; exigiria override/auditoria e decisão de escopo |
| Regras fora do Bahamut para acessório | fonte nova e decisão própria; cliente 7662 não as prova |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*

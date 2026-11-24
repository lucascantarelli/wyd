> Pergunta: "como aplico o `0025_content_combat` e como stats, equipamento e multiplicadores de combate passam a existir no banco?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 25); `ddl-confirmado.md` §5.25 (confirmado em 2026-11-21); fichas legadas de `0012_combate.sql`; `deprecated/docs/wyd/sistemas/drops.md`
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-23)

Vigésimo quinto arquivo da série: 4 tabelas no schema `content`. As fichas de coluna vêm de
`0012_combate.sql` (nível 4); são **hipótese `H`** até validação própria, não colunas provadas.

!!! note "Aprovado em 2026-11-21"
    O dono aprovou o conjunto em 2026-11-21, com `RESTRICT` na FK do item e teto `1.000.000` em
    `content_rate_override.rate_bps` (§8). O lote está `concluída` em `migrations-checklist.md` §2 e as
    quatro tabelas contam entre as 109 com DDL fechado.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0025` (checklist, linha 109) |
| Arquivo | `crates/wyd-db/migrations/0025_content_combat.sql` |
| Schema | `content` (4 tabelas) |
| Posição | 25 de 34 |
| Depende de | `0001` (schema, privilégios); `0014` (`content.item_catalog`); `0019` (`content.npc_template`) |
| É dependência de | `0026` (`quest_step` não usa estas tabelas) — nenhuma da série |
| Tabelas criadas | `mob_template_stat`, `mob_template_equip`, `drop_rate_override`, `content_rate_override` |
| Objetos criados | 4 tabelas, 5 FKs, 0 únicos extras, 2 índices secundários, 0 triggers |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar o que o servidor precisa para lutar e dropar: stats por template de mob, equipamento por slot,
peso de drop por item e multiplicador por família de taxa. É conteúdo lido no boot; nenhum estado de
jogador nasce aqui.

## 3. Pré-requisitos

1. `0001_bootstrap`: schema `content` e default privileges (só `SELECT` para `wyd_game`, `wyd_portal` e `wyd_read`).
2. `0014_content_item`: `content.item_catalog(item_index)`.
3. `0019_content_world`: `content.npc_template(template_name)`.
4. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.25, copiado sem alteração.

```sql
-- 0025_content_combat: stats, equipamento e multiplicadores de combate e drop.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabelas base seed-only e read-only em runtime; nenhum GRANT novo (0001 já concede SELECT em content).

-- migrate:up
-- Stats do template: PK é o próprio template e o filho é possuído por ele (P-04, P-15).
CREATE TABLE content.mob_template_stat (
    template_name TEXT NOT NULL,
    level         SMALLINT NOT NULL DEFAULT 1,
    max_hp        BIGINT NOT NULL DEFAULT 1,
    attack        INTEGER NOT NULL DEFAULT 0,
    defense       INTEGER NOT NULL DEFAULT 0,
    absorb        INTEGER NOT NULL DEFAULT 0,
    magic_attack  INTEGER NOT NULL DEFAULT 0,
    magic_defense INTEGER NOT NULL DEFAULT 0,
    exp_reward    BIGINT NOT NULL DEFAULT 0,
    gold_min      BIGINT NOT NULL DEFAULT 0,
    gold_max      BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_mob_template_stat PRIMARY KEY (template_name),
    CONSTRAINT fk_mob_template_stat_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_mob_template_stat_nivel CHECK (level BETWEEN 1 AND 400),
    CONSTRAINT ck_mob_template_stat_hp CHECK (max_hp > 0),
    CONSTRAINT ck_mob_template_stat_ataque CHECK (attack >= 0 AND magic_attack >= 0),
    CONSTRAINT ck_mob_template_stat_defesa CHECK (
        defense >= 0 AND magic_defense >= 0 AND absorb >= 0),
    CONSTRAINT ck_mob_template_stat_exp CHECK (exp_reward >= 0),
    CONSTRAINT ck_mob_template_stat_ouro CHECK (gold_min >= 0 AND gold_max >= gold_min)
);

-- Equipamento por template e slot; o item vem do catálogo (0014), não de texto livre.
CREATE TABLE content.mob_template_equip (
    template_name TEXT NOT NULL,
    slot          SMALLINT NOT NULL,
    item_index    INTEGER NOT NULL,
    CONSTRAINT pk_mob_template_equip PRIMARY KEY (template_name, slot),
    CONSTRAINT fk_mob_template_equip_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT fk_mob_template_equip_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_mob_template_equip_slot CHECK (slot BETWEEN 0 AND 15),
    CONSTRAINT ck_mob_template_equip_item CHECK (item_index BETWEEN 0 AND 32767)
);

-- P-15: FK do item fora do prefixo da PK.
CREATE INDEX ix_mob_template_equip_item ON content.mob_template_equip (item_index);

-- Drop é peso relativo no roll 1..1.000.000 (drops.md), não probabilidade absoluta.
CREATE TABLE content.drop_rate_override (
    template_name TEXT NOT NULL,
    item_index    INTEGER NOT NULL,
    weight        BIGINT NOT NULL,
    min_qty       SMALLINT NOT NULL DEFAULT 1,
    max_qty       SMALLINT NOT NULL DEFAULT 1,
    CONSTRAINT pk_drop_rate_override PRIMARY KEY (template_name, item_index),
    CONSTRAINT fk_drop_rate_override_template FOREIGN KEY (template_name)
        REFERENCES content.npc_template (template_name) ON UPDATE RESTRICT ON DELETE CASCADE,
    -- Item com regra de drop não é apagado em silêncio: o catálogo é conteúdo e a regra é de combate.
    CONSTRAINT fk_drop_rate_override_item FOREIGN KEY (item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_drop_rate_override_peso CHECK (weight > 0),
    CONSTRAINT ck_drop_rate_override_qtd CHECK (min_qty > 0 AND max_qty >= min_qty),
    CONSTRAINT ck_drop_rate_override_item CHECK (item_index BETWEEN 0 AND 32767)
);

CREATE INDEX ix_drop_rate_override_item ON content.drop_rate_override (item_index);

-- Multiplicador por família de taxa: `kind` é a chave, e o valor 10000 = 100%.
CREATE TABLE content.content_rate_override (
    kind     TEXT NOT NULL,
    rate_bps INTEGER NOT NULL,
    CONSTRAINT pk_content_rate_override PRIMARY KEY (kind),
    CONSTRAINT ck_content_rate_override_kind CHECK (
        kind IN ('exp', 'drop', 'gold', 'skill_point')),
    CONSTRAINT ck_content_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 1000000)
);
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP TABLE content.content_rate_override;
DROP INDEX content.ix_drop_rate_override_item;
DROP TABLE content.drop_rate_override;
DROP INDEX content.ix_mob_template_equip_item;
DROP TABLE content.mob_template_equip;
DROP TABLE content.mob_template_stat;
```

## 6. Colunas

Origem `H` = hipótese derivada da ficha do legado; `P` = provada por fonte do planejamento novo.

### `content.mob_template_stat`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | `P` | Mesmo nome de `content.npc_template`; é a chave natural e a PK (`P-04`). |
| `level` | `SMALLINT` | não | `H` | Nível do mob, 1–400 (teto de personagem em §5.10); o legado não prova a faixa. |
| `max_hp` | `BIGINT` | não | `H` | HP máximo; `BIGINT` porque a ficha o declara largo. |
| `attack` | `INTEGER` | não | `H` | Ataque físico. |
| `defense` | `INTEGER` | não | `H` | Defesa física. |
| `absorb` | `INTEGER` | não | `H` | Absorção de dano. |
| `magic_attack` | `INTEGER` | não | `H` | Ataque mágico. |
| `magic_defense` | `INTEGER` | não | `H` | Defesa mágica. |
| `exp_reward` | `BIGINT` | não | `H` | EXP concedida ao morrer. |
| `gold_min` | `BIGINT` | não | `H` | Ouro mínimo dropado. |
| `gold_max` | `BIGINT` | não | `H` | Ouro máximo dropado; `>= gold_min` por CHECK. |

### `content.mob_template_equip`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | `P` | Template dono do equipamento; metade da PK. |
| `slot` | `SMALLINT` | não | `H` | Slot de equipamento, 0–15 pela ficha. |
| `item_index` | `INTEGER` | não | `H` | Índice do item no catálogo (`0014`), não texto livre. |

### `content.drop_rate_override`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | `P` | Template que sofre o override; metade da PK. |
| `item_index` | `INTEGER` | não | `H` | Item dropado, do catálogo. |
| `weight` | `BIGINT` | não | `H` | Peso relativo no roll 1..1.000.000 (`drops.md`), não probabilidade. |
| `min_qty` | `SMALLINT` | não | `H` | Pilha mínima dropada, `> 0`. |
| `max_qty` | `SMALLINT` | não | `H` | Pilha máxima, `>= min_qty`. |

### `content.content_rate_override`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `kind` | `TEXT` | não | `H` | Família multiplicada: `exp`, `drop`, `gold`, `skill_point`. |
| `rate_bps` | `INTEGER` | não | `H` | Multiplicador em basis points; 10000 = 100%. |

Não há `content_source`, `confidence` nem `notes`: proveniência de dataset fica em
`content.content_source` (`0019`) e `H` fica nesta página, como §5.22 e §5.24.

## 7. Constraints e índices

- PKs: `mob_template_stat(template_name)`; `mob_template_equip(template_name, slot)`;
  `drop_rate_override(template_name, item_index)`; `content_rate_override(kind)`.
- FKs: template em `CASCADE` (filho possuído); item em `RESTRICT` (`item_catalog` é conteúdo e a regra
  é de combate); nenhuma FK de saída em `content_rate_override` (é tabela raiz).
- Índices secundários: `ix_mob_template_equip_item` e `ix_drop_rate_override_item`, ambos para a FK do
  item, que não é prefixo de PK (`P-15`). Nenhum índice preventivo.
- CHECKs de valor e de faixa por coluna; nenhum CHECK semântico sobre combate (isso é runtime).
- Nenhum `GRANT` novo: `0001` já concede `SELECT` em `content`.

### Diferenças em relação às fichas do legado

| Ficha (`0012_combate.sql`) | Aqui | Por quê |
|---|---|---|
| `drop_rate_override.item_index` → `CASCADE` | `RESTRICT` | catálogo é conteúdo; apagar item não pode apagar regra de drop em silêncio |
| `mob_template_stat.max_hp BIGINT` | mantido | a ficha já declara `BIGINT`; sem motivo para estreitar |
| `content_rate_override.notes` | ausente | observação livre não vira coluna: fica na página e em `ops.config_change`, como §5.23 |
| `content_source`/`confidence` | ausentes | proveniência em `content.content_source` (`0019`) |
| `item_index` 0–6499 citado na ficha | `BETWEEN 0 AND 32767` | a faixa do cliente é do loader; o banco aceita o domínio do catálogo (`0014`) |

## 8. Decisões de forma

1. Stats e equipamento são filhos do template, com `CASCADE`; o item é referência, com `RESTRICT`.
2. `drop_rate_override` guarda peso (`BIGINT > 0`) e pilha (`min_qty`/`max_qty`), não percentual.
3. `content_rate_override` tem `kind` como PK e `rate_bps` em basis points.
4. `ON DELETE` do item no drop: `RESTRICT`, respondido pelo dono em 2026-11-21 contra o `CASCADE` da
   ficha; item apagado não leva regra de drop em silêncio.
5. Teto de `content_rate_override.rate_bps`: `1.000.000` (10.000%), respondido pelo dono em 2026-11-21.

## 9. Prova prevista (não executada)

1. `template_name` inexistente em `mob_template_stat`, `mob_template_equip` ou `drop_rate_override` → `23503`.
2. `DELETE` de `content.npc_template` → linhas filhas somem (`CASCADE`).
3. `DELETE` de item com drop ou equipamento → `23503` (`RESTRICT`).
4. `weight <= 0`, `min_qty = 0` ou `max_qty < min_qty` → `23514`.
5. `kind` fora das quatro famílias ou `rate_bps` fora de 0–1.000.000 → `23514`.
6. `SET ROLE wyd_game; INSERT INTO content.drop_rate_override …` → `42501` (só `SELECT`).
7. `down` derruba índices e tabelas sem erro de dependência.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| Valores de stat, drop e multiplicador | `0020`/`0023`, bloqueados pela verificação do dado de origem |
| Ordem do roll e desempate de peso | `wyd-server` |
| Extração dos stats do `STRUCT_MOB` | `H` declarada na ficha; exige trace do cliente |
| Relação entre `content_rate_override` e `content.game_setting` | `wyd-server`/`wyd-sim`; este lote só guarda o multiplicador |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*

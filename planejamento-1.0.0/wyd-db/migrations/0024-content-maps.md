> Pergunta: "como aplico o `0024_content_maps` e como mapa e rota de teleporte passam a existir no banco?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 24); `ddl-confirmado.md` §5.24 (confirmado em 2026-11-21); fichas legadas de `0011_rotas_mundo.sql`; `deprecated/docs/wyd/sistemas/mundo.md` §5/§6
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-23)

Vigésimo quarto arquivo da série: 2 tabelas no schema `content`. As fichas de coluna vêm das fontes
históricas de mundo (nível 4); são **hipótese `H`** até validação própria, não colunas provadas por
aparecerem no legado.

!!! note "Aprovado em 2026-11-21"
    As seis decisões de forma foram respondidas em 2026-11-21 e o dono aprovou o conjunto na mesma
    data, na opção "Sim, aprovar como está". O `0024` está `concluída` na checklist e as duas tabelas
    contam em `ddl-confirmado.md` §1.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0024` (checklist, linha 108) |
| Arquivo | `crates/wyd-db/migrations/0024_content_maps.sql` |
| Schema | `content` (2 tabelas) |
| Posição | 24 de 34 |
| Depende de | `0001` (schema, privilégios); `0014` (`content.item_catalog`); `0019` (`content.region`, `content.npc_definition`) |
| É dependência de | `0020` (seed do mundo) e `0026` (`quest_step.map_id`, segundo a ficha de `map_definition`; `H`) |
| Tabelas criadas | `map_definition`, `teleport_route` |
| Objetos criados | 2 tabelas, 5 FKs, 1 único, 4 índices secundários, 0 triggers |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Dar nome e atributos a cada mapa do cliente e guardar o teleporte como par de coordenadas
origem→destino, com o NPC que o oferece, o preço e o requisito de nível ou item. É o que falta para o
jogador não ficar preso depois de uma morte ou de uma troca de área (`mundo.md` §5).

## 3. Pré-requisitos

1. `0001_bootstrap`: schema `content` e default privileges (só `SELECT` para `wyd_game`, `wyd_portal` e `wyd_read`).
2. `0019_content_world`: `content.region(id)` e `content.npc_definition(id)`.
3. `0014_content_item`: `content.item_catalog(item_index)`.
4. Conexão como `wyd_migrator`. O único com `NULLS NOT DISTINCT` exige PostgreSQL 15 ou superior; o alvo é PG17.

## 4. DDL do `up`

Texto proposto em `ddl-confirmado.md` §5.24, copiado sem alteração.

```sql
-- 0024_content_maps: definição de mapa e rota de teleporte por coordenada.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabela base seed-only e read-only em runtime: mapa e rota são dado, não estado de personagem.

-- migrate:up
-- Chave natural do cliente: o id do mapa é o que o STRUCT_MOB já carrega. Sem identidade (P-04).
CREATE TABLE content.map_definition (
    map_id     SMALLINT NOT NULL,
    name       TEXT NOT NULL,
    -- Região é retângulo em `content.region` (0019); nula enquanto o mapa não tiver retângulo.
    region_id  BIGINT,
    is_pvp     BOOLEAN NOT NULL DEFAULT false,
    is_safe    BOOLEAN NOT NULL DEFAULT false,
    is_dungeon BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_map_definition PRIMARY KEY (map_id),
    CONSTRAINT fk_map_definition_region FOREIGN KEY (region_id)
        REFERENCES content.region (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_map_definition_id CHECK (map_id >= 0),
    CONSTRAINT ck_map_definition_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

-- P-15: FK nullable indexada. A leitura quente é por retângulo; a busca por mapa é a PK.
CREATE INDEX ix_map_definition_region ON content.map_definition (region_id)
    WHERE region_id IS NOT NULL;

-- Coordenada explícita origem→destino: destino é ponto, nunca "centro da região" (mundo.md §5.1).
CREATE TABLE content.teleport_route (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    from_map_id    SMALLINT NOT NULL,
    from_x         SMALLINT NOT NULL,
    from_y         SMALLINT NOT NULL,
    to_map_id      SMALLINT NOT NULL,
    to_x           SMALLINT NOT NULL,
    to_y           SMALLINT NOT NULL,
    -- NPC que oferece a rota; nulo quando a rota é do mapa (zona de teleporte, portal).
    npc_id         INTEGER,
    -- Preço em ouro do PERSONAGEM (`game.character.coin`), nunca a carteira da conta.
    coin           BIGINT NOT NULL DEFAULT 0,
    req_level      SMALLINT NOT NULL DEFAULT 1,
    -- Item exigido (pergaminho, pedido de caça): índice do catálogo, não preço.
    req_item_index INTEGER,
    enabled        BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_teleport_route PRIMARY KEY (id),
    -- Dois NPCs podem oferecer o mesmo par; o mesmo NPC não repete a rota.
    CONSTRAINT uq_teleport_route_rota UNIQUE NULLS NOT DISTINCT (
        from_map_id, from_x, from_y, to_map_id, to_x, to_y, npc_id),
    CONSTRAINT fk_teleport_route_origem FOREIGN KEY (from_map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_teleport_route_destino FOREIGN KEY (to_map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    -- Apagar o NPC desliga a rota em vez de derrubar o mapa: o destino continua válido.
    CONSTRAINT fk_teleport_route_npc FOREIGN KEY (npc_id)
        REFERENCES content.npc_definition (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_teleport_route_item FOREIGN KEY (req_item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_teleport_route_custo CHECK (coin >= 0),
    CONSTRAINT ck_teleport_route_nivel CHECK (req_level >= 1),
    CONSTRAINT ck_teleport_route_item CHECK (
        req_item_index IS NULL OR req_item_index BETWEEN 0 AND 32767),
    -- SMALLINT não negativo, igual a `npc_definition.pos_x`: o teto 4095 é regra do loader do
    -- cliente, não do banco (`mundo.md` §3 preserva um y2 = 6841 no retângulo da região).
    CONSTRAINT ck_teleport_route_origem_pos CHECK (
        from_x BETWEEN 0 AND 32767 AND from_y BETWEEN 0 AND 32767),
    CONSTRAINT ck_teleport_route_destino_pos CHECK (
        to_x BETWEEN 0 AND 32767 AND to_y BETWEEN 0 AND 32767)
);

-- P-15: as FKs nullable e a de destino. `from_map_id` já é prefixo do único.
CREATE INDEX ix_teleport_route_npc ON content.teleport_route (npc_id)
    WHERE npc_id IS NOT NULL;
CREATE INDEX ix_teleport_route_item ON content.teleport_route (req_item_index)
    WHERE req_item_index IS NOT NULL;
CREATE INDEX ix_teleport_route_destino ON content.teleport_route (to_map_id);
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP INDEX content.ix_teleport_route_destino;
DROP INDEX content.ix_teleport_route_item;
DROP INDEX content.ix_teleport_route_npc;
DROP TABLE content.teleport_route;
DROP INDEX content.ix_map_definition_region;
DROP TABLE content.map_definition;
```

## 6. Colunas

Origem `H` = hipótese derivada da ficha do legado; `P` = provada por fonte do planejamento novo ou por
decisão de forma registrada em 2026-11-21.

### `content.map_definition`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `map_id` | `SMALLINT` | não | `P` | Id do mapa no cliente (`STRUCT_MOB`); é a chave natural, então é a PK sem identidade (`P-04`) e sem o teto 255 da ficha. |
| `name` | `TEXT` | não | `H` | Nome exibido, da ficha; a faixa 1–64 espelha `region.name` (§5.21) em vez do `name <> ''` da ficha. |
| `region_id` | `BIGINT` | sim | `H` | Região do mapa; `BIGINT` porque `content.region.id` é identidade `BIGINT` (§5.21), não o `SMALLINT` da ficha. Nula enquanto o mapa não tiver retângulo. |
| `is_pvp` | `BOOLEAN` | não | `H` | PvP habilitado, `false` por omissão; quais mapas têm cada flag é UNVERIFIED (`mundo.md` §6). |
| `is_safe` | `BOOLEAN` | não | `H` | Zona segura (sem combate), `false` por omissão; mesma lacuna de valor por mapa. |
| `is_dungeon` | `BOOLEAN` | não | `H` | Instância/dungeon, `false` por omissão; mesma lacuna de valor por mapa. |

### `content.teleport_route`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `id` | `BIGINT` | não | `P` | Identidade `GENERATED ALWAYS` (`P-03`): a rota não tem chave natural única porque o mesmo par pode ser oferecido por mais de um NPC. |
| `from_map_id` | `SMALLINT` | não | `H` | Mapa de origem; FK `RESTRICT` para `map_definition`, que este lote cria. |
| `from_x` | `SMALLINT` | não | `H` | X de origem; `0..32767`, teto do `SMALLINT` não negativo, igual a `npc_definition.pos_x` — o 4095 é regra do loader. |
| `from_y` | `SMALLINT` | não | `H` | Y de origem, mesma faixa e mesma razão. |
| `to_map_id` | `SMALLINT` | não | `H` | Mapa de destino; FK `RESTRICT`. |
| `to_x` | `SMALLINT` | não | `H` | X de destino explícito, nunca o centro da região (`mundo.md` §5.1). |
| `to_y` | `SMALLINT` | não | `H` | Y de destino explícito, mesma razão. |
| `npc_id` | `INTEGER` | sim | `H` | NPC que oferece a rota; `SET NULL` no delete. Nulo = rota do mapa (zona de teleporte, portal). |
| `coin` | `BIGINT` | não | `H` | Preço em ouro **do personagem** (`game.character.coin`); a ficha chamava `cost_gold INTEGER`, e a unidade menor do padrão é `BIGINT` (`padrao-banco.md` §4). |
| `req_level` | `SMALLINT` | não | `H` | Nível mínimo para usar, `>= 1`; o legado não prova o teto, então não há teto. |
| `req_item_index` | `INTEGER` | sim | `H` | Item exigido (pergaminho, pedido de caça), como índice do catálogo (`0014`) e com `RESTRICT`. |
| `enabled` | `BOOLEAN` | não | `H` | Desliga a rota sem apagar a linha; o único cobre a rota mesmo desligada. |

Não há `content_source`/`confidence`/`notes` por linha: proveniência de dataset fica em
`content.content_source` (`0019`) e `H` fica nesta página, como decidido em §5.22. Não há `created_at`/
`updated_at`: tabela base seed-only e read-only em runtime, também como §5.22.

## 7. Constraints e índices

- PKs: `map_definition` em `map_id` (chave natural do cliente); `teleport_route` em `id` (identidade).
- `uq_teleport_route_rota` é `UNIQUE NULLS NOT DISTINCT (from_map_id, from_x, from_y, to_map_id, to_x, to_y, npc_id)`:
  o mesmo NPC não repete a rota, e a rota sem NPC também não duplica. Dois NPCs podem vender o mesmo trecho.
- FKs com `ON UPDATE RESTRICT`: mapa (`RESTRICT` nos dois lados da rota), item (`RESTRICT`) e NPC
  (`SET NULL` — apagar o NPC desliga a rota, não derruba o mapa).
- CHECKs de valor: `map_id >= 0`, nome 1–64 em `btrim`, `coin >= 0`, `req_level >= 1`, item 0–32767 e
  coordenadas 0–32767 nos dois lados.
- Índices secundários: `ix_map_definition_region` (parcial, FK nullable), `ix_teleport_route_npc`
  (parcial), `ix_teleport_route_item` (parcial) e `ix_teleport_route_destino`. `from_map_id` já é
  prefixo do único; não há índice preventivo além das FKs (`P-15`).
- Nenhum `GRANT` novo: `0001` já dá `SELECT` em `content` para `wyd_game`, `wyd_portal` e `wyd_read`, e
  nenhum runtime escreve estas tabelas. `wyd_portal` não recebe escrita — o CMS escreve em `portal`
  (`0030`), não em `content`.

### Diferenças em relação à ficha do legado

| Ficha (`0011_rotas_mundo.sql`) | Aqui | Por quê |
|---|---|---|
| `map_id CHECK 0..255` | `map_id >= 0` | decisão de 2026-11-21; a lista do 7662 é UNVERIFIED e 255 não tem fonte no planejamento novo |
| `region_id SMALLINT` | `region_id BIGINT` | `content.region.id` é identidade `BIGINT` (§5.21) |
| `name <> ''` | 1–64 em `btrim(name)` | espelha `region.name` e recusa nome só de espaço |
| `cost_gold INTEGER` | `coin BIGINT` | unidade menor com `BIGINT` (`padrao-banco.md` §4), e o nome diz de quem é o saldo |
| `id INTEGER` | `id BIGINT` identity | `P-03` |
| único sem `npc_id` | único com `npc_id`, `NULLS NOT DISTINCT` | dois NPCs podem oferecer o mesmo trecho |
| índice `npc_id WHERE enabled` | `npc_id WHERE npc_id IS NOT NULL` | o índice serve à FK (`SET NULL`) e a consulta por NPC inclui rota desligada |
| coordenada `0..4095` | `0..32767` | 4095 é teto do loader; `mundo.md` §3 preserva um `y2 = 6841` no retângulo |
| `content_source`, `confidence` | ausentes | proveniência em `content.content_source` (`0019`); `H` na página, como §5.22 |

## 8. Decisões respondidas em 2026-11-21

1. `map_definition` sem identidade: `map_id SMALLINT` é a PK (`P-04`).
2. `map_id >= 0`, sem teto artificial — o 255 da ficha não tem fonte no planejamento novo.
3. As três flags entram: `is_pvp`, `is_safe` e `is_dungeon`, todas `BOOLEAN NOT NULL DEFAULT false`.
4. Rota por coordenada explícita origem→destino, nunca por região (`mundo.md` §5.1).
5. `teleport_route` com `id BIGINT` identity e único por coordenada e NPC, `NULLS NOT DISTINCT`.
6. Custo em `coin BIGINT NOT NULL DEFAULT 0`: é o ouro do **personagem** (`game.character.coin`), não a
   carteira da conta (`game.account_wallet.coin`, §5.2) e não o cash do portal (`0011`). O lote guarda o
   preço; o débito é do runtime, contra a linha do personagem.

## 9. Prova prevista (não executada)

1. `map_id` repetido → `23505` em `pk_map_definition`.
2. Mesma rota (origem, destino e NPC) duas vezes → `23505` em `uq_teleport_route_rota`; a mesma rota com
   `npc_id` diferente → aceita.
3. Duas rotas sem NPC com as mesmas coordenadas → `23505` (é o caso que `NULLS NOT DISTINCT` fecha).
4. `from_map_id` ou `to_map_id` inexistente → `23503`.
5. `DELETE` de mapa com rota apontando para ele → `23503` (não cascateia).
6. `DELETE` do NPC que oferece a rota → a linha continua com `npc_id` nulo.
7. `DELETE` do item exigido por rota → `23503`.
8. `coin < 0`, `req_level < 1` ou coordenada negativa → `23514`.
9. Coordenada 4000 é aceita: o teto de 4095 não está no banco.
10. `SET ROLE wyd_game; INSERT INTO content.map_definition …` → `42501` (só `SELECT`).
11. `down` derruba índices e tabelas sem erro de dependência.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| Quais mapas têm `is_pvp`, `is_safe` ou `is_dungeon` | conteúdo; valor por mapa é `H`, sem fonte no planejamento novo |
| Lista de mapas do cliente 7662 | `0020`, com dataset verificado (`mundo.md` §6, UNVERIFIED) |
| Lista de teleportes por NPC | `0020`; `mundo.md` §6 a declara UNVERIFIED |
| Preço em item da rota | `0020`; aqui só existe o índice exigido |
| Bloqueio de teleporte durante cerco e portal do Kefra por canal | `wyd-server` (`mundo.md` §2 e `guerras.md`) |
| Se o gasto do ouro do personagem precisa de razão contábil | `M16` cobre a carteira da conta (`game.account_ledger`), não o ouro do personagem; ledger próprio é decisão de arquitetura, fora deste lote |
| Seed de mundo | `0020`, parado no gerador (`plano-migrations.md` §1) |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*

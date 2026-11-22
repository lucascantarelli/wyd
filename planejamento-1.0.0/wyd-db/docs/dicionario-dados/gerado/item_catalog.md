> Pergunta: "o que `content.item_catalog` guarda de cada item do cliente e por que `item_index` é a chave?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.18; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Catálogo do item do cliente (3.220 linhas no seed de `0017`): nome, categoria, efeitos, preço-base, grau
e requisitos. O jogo lê para validar equipar e usar; o portal lê para a wydpedia. `item_index` é o
número do cliente e é a chave, porque é o que as instâncias, os preços e a fila de entrega já carregam.
Não guarda visual do cliente (`IndexMesh`, `IndexTexture`, `IndexVisualEffect`) nem `nUnique`/`nPos`/
`Extra`: ficaram fora por falta de consumidor provado.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `item_index` | `INTEGER` | não | — | índice do item no cliente (`STRUCT_ITEM.sIndex`) |
| `name` | `TEXT` | não | — | nome exibido (campo `Name` do `STRUCT_ITEMLIST`) |
| `name_fold` | `TEXT` gerada `STORED` | não | — | expressão `content.fn_fold(name)`; chave da busca por prefixo |
| `kind_slug` | `TEXT` | sim | — | categoria de `content.item_kind`; `NULL` = categoria não provada |
| `effect_names` | `TEXT[]` | não | `'{}'` | símbolos `EF_*` na ordem em que o item os aplica |
| `base_price` | `BIGINT` | não | `0` | preço de catálogo na menor unidade; preço por mercador é `game.item_price` |
| `grade` | `SMALLINT` | não | — | número do grau do cliente, sem tradução para letra |
| `req_level` / `req_str` / `req_int` / `req_dex` / `req_con` | `SMALLINT` | não | `0` | requisitos de nível, força, inteligência, destreza e constituição; 0 = sem exigência |
| `enabled` | `BOOLEAN` | não | `true` | aposentadoria sem `DELETE` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbos; `updated_at` pelo trigger |

`name_fold` nunca é nula porque `name` é `NOT NULL` e `content.fn_fold` devolve `NULL` só para entrada
nula — mas a coluna **não declara** `NOT NULL`, então o catálogo a reporta como anulável.

## Chave primária

`pk_item_catalog (item_index)` — o número do cliente é a identidade (`P-04`). Nome não é chave: dois
itens podem ter o mesmo nome, e não há `UNIQUE (name)`.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_item_catalog_kind`: `kind_slug → content.item_kind (slug)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`.
- **Entrada, `RESTRICT` (8 FKs):** `fk_level_item_catalogo` e `fk_world_item_seed_catalogo` (`0019`);
  `fk_refine_gem_effect_item` e `fk_accessory_refine_rule_bonus` (`0021`); `fk_teleport_route_item`
  (`0024`); `fk_mob_template_equip_item` e `fk_drop_rate_override_item` (`0025`); `fk_quest_reward_item`
  (`0026`). Item referenciado não é apagado em silêncio.
- **Sem FK de `character_item`, `vault_item`, `item_price` e `delivery_queue`:** esses arquivos fecharam
  na série antes deste e a FK seria migração aditiva (`0014-content-item.md` §10).
- **Sem FK em `effect_names`:** `TEXT[]` não referencia tabela; quem garante o conjunto é o trigger.

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_item_catalog_indice` | `item_index` fora de 0–32767 (o `short` do cliente) |
| `ck_item_catalog_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |
| `ck_item_catalog_preco` | `base_price` negativo |
| `ck_item_catalog_grade` | `grade` negativo |
| `ck_item_catalog_requisitos` | qualquer requisito fora de 0–32767 |
| `ck_item_catalog_efeitos` | `NULL` em alguma posição de `effect_names` |

`trg_item_catalog_effects` cobre o que `CHECK` não expressa em array: efeito repetido levanta `23514`
nomeando `ck_item_catalog_efeitos`, e símbolo ausente de `content.item_effect` levanta `23503` nomeando
`fk_item_catalog_efeito` — esse segundo nome **não** é constraint, é só o rótulo do erro, porque
`TEXT[]` não aceita FK.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_item_catalog_name_fold (name_fold text_pattern_ops)` | busca por prefixo do nome dobrado (`P-11`) |
| `ix_item_catalog_kind (kind_slug)` | itens de uma categoria |
| `ix_item_catalog_effect_names` (`GIN` em `effect_names`) | itens que aplicam um efeito |

Triggers `trg_item_catalog_set_updated_at` e `trg_item_catalog_effects`.

## Armadilha de leitura

`grade` é número, não letra: N/M/A/Le/Anct/E não está provado, e o lote guarda o número sem traduzir.
`kind_slug` nulo é ausência de prova, não erro de dado. E aposentar efeito em `content.item_effect` é
`enabled = false`: trocar o `slug` depois do seed não é coberto por `ON UPDATE RESTRICT`.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0017`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0014-content-item.md` §8.

## Migration

Forma (dona): `migrations/0014-content-item.md` — arquivo `0014_content_item.sql`.

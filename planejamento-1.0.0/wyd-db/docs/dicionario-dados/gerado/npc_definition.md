> Pergunta: "o que `content.npc_definition` guarda de um NPC no mapa e o que o banco deixa em aberto?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

NPC no mapa: corpo (`template_name`), posição, origem, rota, grupo de gerador e seguidor. `id` é o
**código do insumo** (NPCGener), não identidade gerada — `game.item_price.npc_id` é `INTEGER` e a FK
futura precisa casar sem remapear. Não guarda atributo de combate (é `0025`, por template) nem o que a
loja vende (é `npc_shop_item`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `INTEGER` | não | — | código do NPC no insumo (NPCGener) |
| `slug` | `TEXT` | não | — | nome estável para o portal |
| `template_name` | `TEXT` | não | — | corpo do NPC |
| `display_name` | `TEXT` | não | `''` | nome exibido; vazio é aceito |
| `enabled` | `BOOLEAN` | não | `true` | aposentadoria sem `DELETE` |
| `map_id` | `SMALLINT` | não | `0` | mapa do cliente |
| `pos_x` / `pos_y` | `SMALLINT` | não | `0` | coordenada no mapa |
| `route_type` | `SMALLINT` | não | `0` | tipo de rota/patrulha do gerador |
| `merchant` | `SMALLINT` | não | `0` | campo Merchant do insumo: 0 = não comerciante |
| `origin` | `TEXT` | não | `'content'` | `content` ou `custom`; `custom` não é sobrescrito pelo import |
| `generator_index` | `INTEGER` | sim | — | índice do gerador; nulo no NPC fixo |
| `follower_template` | `TEXT` | não | `''` | template do seguidor/invocação |
| `minute_generate` | `INTEGER` | não | `0` | intervalo de geração em minutos |
| `min_group` / `max_group` | `SMALLINT` | não | `0` | tamanho mínimo e máximo do grupo gerado |
| `max_num_mob` | `SMALLINT` | não | `1` | teto de mobs vivos do gerador |
| `formation` | `SMALLINT` | não | `0` | formação do grupo gerado |
| `generator_data` | `JSONB` | não | `'{}'` | bloco cru do gerador, ainda não decodificado |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbos; `updated_at` pelo trigger |

## Chave primária

`pk_npc_definition (id)` — o código do insumo é a chave porque o preço por NPC de `0011` já é
`INTEGER`; gerar identidade própria obrigaria a remapear essa referência depois.

## FKs de saída e de entrada

- **Saída:** `fk_npc_definition_template`: `template_name → content.npc_template (template_name)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`.
- **Entrada, `CASCADE`:** `npc_shop_item` (`fk_npc_shop_item_npc`) — a loja é linha possuída do NPC.
- **Entrada, `RESTRICT`:** `guard_spawn` (`fk_guard_spawn_npc`) — guarda não perde o NPC em silêncio.
- **Entrada, `SET NULL`:** `teleport_route.npc_id` (`fk_teleport_route_npc`, `0024`) — apagar o NPC
  desliga a rota em vez de derrubar o mapa.

## Unicidade

`ux_npc_definition_gerador (map_id, generator_index) WHERE generator_index IS NOT NULL` — índice único
**parcial**: um NPC por posição de gerador no mesmo mapa, e gerador sem índice (`NULL`) fica fora da
regra.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_npc_definition_id` | `id` menor ou igual a zero |
| `ck_npc_definition_slug` | `slug` fora de `^[a-z][a-z0-9_]{1,63}$` |
| `ck_npc_definition_nome` | `display_name` acima de 64 caracteres após aparar |
| `ck_npc_definition_mapa` | `map_id` fora de 0–255 |
| `ck_npc_definition_posicao` | `pos_x` ou `pos_y` fora de 0–32767 |
| `ck_npc_definition_origem` | `origin` fora de `content`/`custom` |
| `ck_npc_definition_gerador` | `generator_index` negativo |
| `ck_npc_definition_seguidor` | `follower_template` acima de 96 caracteres após aparar |
| `ck_npc_definition_tempo` | `minute_generate` negativo |
| `ck_npc_definition_grupo` | `min_group`/`max_group` fora de 0–255 ou `min_group > max_group` |
| `ck_npc_definition_teto` | `max_num_mob` menor que 1 |
| `ck_npc_definition_formacao` | `formation` negativo |
| `ck_npc_definition_payload` | `generator_data` que não é objeto JSON |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_npc_definition_gerador (map_id, generator_index)` parcial | um gerador por posição no mapa |
| `ix_npc_definition_mapa (map_id)` | NPCs de um mapa |
| `ix_npc_definition_template (template_name)` | quem usa um corpo de NPC |

Trigger `trg_npc_definition_set_updated_at`.

## Armadilha de leitura

`merchant` acima de 0 é **`H`** — a semântica está declarada em `migrations/0019_content_world.md` §9,
não aqui. `display_name` aceita string vazia e `generator_data` aceita qualquer objeto: o schema garante
a forma, não o significado do bloco.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.

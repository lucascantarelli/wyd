> Pergunta: "o que `content.teleport_route` guarda de uma rota e o que impede duas rotas iguais?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.24; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Rota de teleporte como par de coordenadas explícitas origem→destino, com o NPC que a oferece
(opcional), o preço em ouro do personagem e o requisito de nível ou de item. O destino é ponto, nunca
"centro da região" (decisão 4 de `migrations/0024-content-maps.md` §8): a rota leva a uma coordenada, e
quem chega não depende do retângulo da região.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | identidade da rota (`P-03`) |
| `from_map_id` | `SMALLINT` | não | — | mapa de origem (FK) |
| `from_x` / `from_y` | `SMALLINT` | não | — | coordenada de origem |
| `to_map_id` | `SMALLINT` | não | — | mapa de destino (FK) |
| `to_x` / `to_y` | `SMALLINT` | não | — | coordenada de destino |
| `npc_id` | `INTEGER` | sim | — | NPC que oferece a rota (FK); nulo = rota do próprio mapa (zona de teleporte, portal) |
| `coin` | `BIGINT` | não | `0` | preço em ouro do **personagem**, não da carteira da conta |
| `req_level` | `SMALLINT` | não | `1` | nível mínimo para usar |
| `req_item_index` | `INTEGER` | sim | — | item exigido (pergaminho, pedido de caça), como índice do catálogo (FK) |
| `enabled` | `BOOLEAN` | não | `true` | desliga a rota sem apagar a linha |

## Chave primária

`pk_teleport_route (id)` — identidade (`P-03`), porque a rota não tem chave natural: o mesmo trecho pode
ser oferecido por mais de um NPC.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_teleport_route_origem` (`from_map_id → content.map_definition (map_id)`) e
  `fk_teleport_route_destino` (`to_map_id → content.map_definition (map_id)`), `ON UPDATE RESTRICT`,
  `ON DELETE RESTRICT` — mapa com rota apontando não é apagado, e a rota não cascateia.
- **Saída, `SET NULL`:** `fk_teleport_route_npc` (`npc_id → content.npc_definition (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE SET NULL`) — apagar o NPC desliga a rota em vez de derrubar o mapa; o
  destino continua válido.
- **Saída, `RESTRICT`:** `fk_teleport_route_item` (`req_item_index → content.item_catalog (item_index)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`).
- **Entrada:** nenhuma.

## Unicidade

`uq_teleport_route_rota` — único sobre `(from_map_id, from_x, from_y, to_map_id, to_x, to_y, npc_id)`
com `NULLS NOT DISTINCT`: o mesmo NPC não repete o trecho, e duas rotas **sem** NPC com as mesmas coordenadas
também colidem — é o que `NULLS NOT DISTINCT` fecha, já que `NULL` comum não colidiria. Dois NPCs podem
vender o mesmo trecho. `enabled` não entra no único: desligar a rota não libera a combinação.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_teleport_route_custo` | `coin` negativo |
| `ck_teleport_route_nivel` | `req_level` menor que 1 |
| `ck_teleport_route_item` | `req_item_index` fora de 0–32767 |
| `ck_teleport_route_origem_pos` | `from_x` ou `from_y` fora de 0–32767 |
| `ck_teleport_route_destino_pos` | `to_x` ou `to_y` fora de 0–32767 |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_teleport_route_npc (npc_id) WHERE npc_id IS NOT NULL` | FK nullable (`P-15`) e rotas oferecidas por um NPC |
| `ix_teleport_route_item (req_item_index) WHERE req_item_index IS NOT NULL` | FK nullable (`P-15`) e rotas que exigem um item |
| `ix_teleport_route_destino (to_map_id)` | chegadas por mapa de destino |

`from_map_id` já é prefixo do único e não ganha índice próprio. Sem carimbos e sem trigger: tabela base
de seed, read-only em runtime.

## Armadilha de leitura

A coordenada aceita vai a 32767, não a 4095: o teto menor é regra do loader do cliente, não do banco
(`migrations/0024-content-maps.md` §8), e uma rota com `x = 4000` é aceita. `coin` é ouro do
**personagem** (`game.character.coin`), não a carteira da conta nem o cash do portal — o débito é do
runtime, contra a linha do personagem, e o lote guarda só o preço. `req_level` não tem teto porque o
legado não prova nenhum.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0024-content-maps.md` §7.

## Migration

Forma (dona): `migrations/0024-content-maps.md` — arquivo `0024-content-maps.sql`.

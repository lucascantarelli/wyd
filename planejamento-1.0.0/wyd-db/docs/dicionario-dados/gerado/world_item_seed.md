> Pergunta: "o que `content.world_item_seed` guarda do item no chão e o que garante que o seed não duplica?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Item no chão por mapa e coordenada. O legado **não tinha coluna de mapa**; sem ela o item não tem
lugar, então `map_id` entra no desenho novo com o mesmo domínio de `game.city.map_id`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | identidade; o insumo não tem chave |
| `map_id` | `SMALLINT` | não | — | mapa do item no chão; não existia no legado |
| `item_index` | `INTEGER` | não | — | item no chão (FK) |
| `pos_x` / `pos_y` | `SMALLINT` | não | — | coordenada do item |
| `rotate` | `SMALLINT` | não | `0` | rotação/ângulo do item no mapa |
| `enabled` | `BOOLEAN` | não | `true` | linha ativa sem apagar |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbos; `updated_at` pelo trigger |

## Chave primária

`pk_world_item_seed (id)` — identidade própria porque o insumo não tem chave.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_world_item_seed_catalogo`:
  `item_index → content.item_catalog (item_index)` (`0014`), `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`
  — item de chão não sai do catálogo em silêncio.
- **Entrada:** nenhuma.

## Unicidade

`uq_world_item_seed_posicao (map_id, pos_x, pos_y, item_index)` — é o que dá **idempotência ao seed**:
recarregar o mesmo insumo não cria o mesmo item duas vezes no mesmo tile. O único **não inclui
`enabled`**: desativar a linha não libera a posição para outro item.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_world_item_seed_mapa` | `map_id` fora de 0–255 |
| `ck_world_item_seed_item` | `item_index` fora de 0–32767 |
| `ck_world_item_seed_posicao` | `pos_x` ou `pos_y` fora de 0–32767 |
| `ck_world_item_seed_rotacao` | `rotate` negativo |

## Índices

Só a PK e o único — não há leitura por outro predicado no desenho atual. Trigger
`trg_world_item_seed_set_updated_at`.

## Armadilha de leitura

`rotate` é `H`: o insumo não diz se o número é ângulo ou índice de rotação — a pendência está declarada
em `migrations/0019_content_world.md` §9.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.

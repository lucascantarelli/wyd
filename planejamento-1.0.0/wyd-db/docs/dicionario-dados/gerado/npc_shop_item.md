> Pergunta: "o que `content.npc_shop_item` guarda da loja de um NPC e por que não há FK para o item?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Item vendido por um NPC, na posição do contêiner da loja, com três blocos de efeito e valor. Não guarda
preço — preço é `game.item_price` — nem valida o item contra o catálogo, por decisão registrada abaixo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `npc_id` | `INTEGER` | não | — | dono da loja (FK) |
| `slot` | `SMALLINT` | não | — | posição no contêiner da loja |
| `item_index` | `INTEGER` | não | — | item vendido; sem FK, por decisão |
| `eff1`…`eff3` | `SMALLINT` | não | `0` | código de efeito de cada bloco |
| `effv1`…`effv3` | `SMALLINT` | não | `0` | valor do efeito de cada bloco |
| `quantity` | `SMALLINT` | não | `1` | quantidade disponível |

## Chave primária

`pk_npc_shop_item (npc_id, slot)` — a posição na loja é a chave: cada slot tem no máximo um item.

## FKs de saída e de entrada

- **Saída, `CASCADE`:** `fk_npc_shop_item_npc`: `npc_id → content.npc_definition (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — apagar o NPC leva a loja junto.
- **Entrada:** nenhuma. `game.item_price` também usa `npc_id`, mas não referencia esta tabela, e a
  ligação entre as duas é por código, não por constraint.

## Unicidade

Só a PK: o mesmo `item_index` pode aparecer em slots diferentes da mesma loja.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_npc_shop_item_slot` | `slot` fora de 0–26 |
| `ck_npc_shop_item_item` | `item_index` menor ou igual a zero |
| `ck_npc_shop_item_quantidade` | `quantity` fora de 1–255 |

## Índices

Nenhum além da PK — a leitura é sempre pela loja. Sem carimbos e sem trigger: import puro.

## Armadilha de leitura

**Não existe FK para `item_catalog`, e isso é decisão, não esquecimento:** o insumo de loja cita item que
o catálogo do cliente não tem — 22 casos, um deles negativo (`H`, pendência em
`migrations/0019-content-world.md` §9). O `CHECK (item_index > 0)` recusa o negativo, e o seed de `0020`
reporta o que falta em vez de o lote falhar na carga.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019-content-world.md` §8.

## Migration

Forma (dona): `migrations/0019-content-world.md` — arquivo `0019_content_world.sql`.

> Pergunta: "o que `content.map_definition` guarda de um mapa e por que o retângulo da região fica em outra tabela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.24; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Nome e atributos de cada mapa do cliente. `map_id` é o id que o `STRUCT_MOB` já carrega, então é chave
natural (`P-04`), não identidade gerada. A região não vira coluna aqui: `region_id` aponta para
`content.region` (`0019`), que é dona única do retângulo — assim mapa e região não podem divergir. Não
guarda conteúdo de mapa (NPC, item no chão, teleporte): cada um tem a sua tabela.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `map_id` | `SMALLINT` | não | — | id do mapa no cliente (`STRUCT_MOB`) |
| `name` | `TEXT` | não | — | nome exibido |
| `region_id` | `BIGINT` | sim | — | região que recorta o mapa; nula enquanto o mapa não tiver retângulo |
| `is_pvp` | `BOOLEAN` | não | `false` | PvP habilitado no mapa |
| `is_safe` | `BOOLEAN` | não | `false` | zona segura, sem combate |
| `is_dungeon` | `BOOLEAN` | não | `false` | instância/dungeon |

## Chave primária

`pk_map_definition (map_id)` — a chave natural do cliente, sem identidade própria (`P-04`). A faixa é
`>= 0`: o teto 255 da ficha do legado não tem fonte no planejamento novo.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_map_definition_region`: `region_id → content.region (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — região com mapa apontando não é apagada.
- **Entrada, `RESTRICT`:** `fk_teleport_route_origem` e `fk_teleport_route_destino` (`0024`,
  `from_map_id`/`to_map_id`) e `fk_quest_step_mapa` (`0026`, `quest_step.map_id`).
- **Sem FK de `region` para cá:** a seta é sempre mapa → região; a região não sabe quais mapas recorta.

## Unicidade

Só a PK — nenhum único adicional; dois mapas podem ter o mesmo nome.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_map_definition_id` | `map_id` negativo |
| `ck_map_definition_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_map_definition_region (region_id) WHERE region_id IS NOT NULL` | FK nullable (`P-15`) e leitura dos mapas de uma região |

Sem carimbos e sem trigger: tabela base de seed, read-only em runtime.

## Armadilha de leitura

Quais mapas têm cada flag é `H`: o planejamento novo não tem fonte para o valor por mapa
(`migrations/0024_content_maps.md` §10), então `false` significa "não preenchido", não "provado que o
mapa não é PvP". O `map_id` também não tem teto, pelo mesmo motivo: a lista de mapas do 7662 segue
UNVERIFIED e o 255 da ficha não foi reverificado.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0024_content_maps.md` §7.

## Migration

Forma (dona): `migrations/0024_content_maps.md` — arquivo `0024_content_maps.sql`.

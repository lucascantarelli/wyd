> Pergunta: "o que `game.character_item` guarda por posição e o que o schema exige da gema?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.12; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Item na posição do personagem (0–79: 0–15 equipamento, 16–79 mochila), com quantidade, refino,
marca `ancient` e gema.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `slot` | `SMALLINT` | não | — | posição 0–79 |
| `item_index` | `INTEGER` | não | — | índice de item; sem FK |
| `qty` | `INTEGER` | não | `1` | empilhamento, `>= 1` |
| `flags` | `INTEGER` | não | `0` | valor inteiro; semântica a reverificar |
| `refine` | `SMALLINT` | não | `0` | refino 0–15 |
| `ancient` | `BOOLEAN` | não | `false` | marca `ancient` |
| `gem_effect` | `SMALLINT` | não | `0` | efeito de gema 0–4 |
| `gem_value` | `SMALLINT` | não | `0` | valor da gema, `>= 0` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_character_item (character_id, slot)` — duas linhas na mesma posição são impossíveis; o item não
tem id próprio.

## FKs de saída e de entrada

- **Saída:** `fk_character_item_character`: `character_id → game.character (id)`,
  `CASCADE`/`NO ACTION`.
- **Entrada:** `refine_pity` (`fk_refine_pity_item`), com `CASCADE` — piedade sem item no slot é estado
  impossível.
- **Sem FK para conteúdo:** `item_index` não aponta para `content.item_catalog` (`0014`), arquivo
  posterior da série.

## Unicidade

Só a PK: `(character_id, slot)` já é a unicidade da posição.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_item_slot` | posição fora de 0–79 |
| `ck_character_item_qtd` | `qty` menor que 1 |
| `ck_character_item_refino` | refino fora de 0–15 |
| `ck_character_item_gema` | `gem_effect` fora de 0–4 ou `gem_value` negativo |
| `ck_character_item_gema_par` | efeito e valor de gema discordando (`(gem_effect = 0) = (gem_value = 0)`) |
| `ck_character_item_gema_refino` | gema em item comum com refino abaixo de 10 |

Gema exige refino ≥ 10, exceto em item `ancient`.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_character_item_indice (item_index)` | lado filho: itens por índice no inventário |

Trigger `trg_character_item_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).
- **Administra:** `wyd_admin` — `SELECT, INSERT, DELETE` concedidos por `0032`: a custódia (`ADM-10`)
  tira o item da mochila e o devolve na mesma transação. É a única tabela com `DELETE` para o papel.

## Migration

Forma (dona): `migrations/0009_game_inventory.md` — arquivo `0009_game_inventory.sql`.

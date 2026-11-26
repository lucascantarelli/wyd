> Pergunta: "o que `game.vault_item` guarda no baú da conta e por que a chave é da conta, não do personagem?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.12; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Item guardado no baú da conta, em 128 posições (0–127), com as mesmas colunas de item de
`character_item`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | PK e FK para `account` |
| `slot` | `SMALLINT` | não | — | posição 0–127 |
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

`pk_vault_item (account_id, slot)` — uma linha por posição do baú; a conta é a dona da chave.

## FKs de saída e de entrada

- **Saída:** `fk_vault_item_account`: `account_id → game.account (id)`, `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.
- **Sem FK para conteúdo:** `item_index` não aponta para `content.item_catalog` (`0014`).

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_vault_item_slot` | posição fora de 0–127 |
| `ck_vault_item_qtd` | `qty` menor que 1 |
| `ck_vault_item_refino` | refino fora de 0–15 |
| `ck_vault_item_gema` | `gem_effect` fora de 0–4 ou `gem_value` negativo |
| `ck_vault_item_gema_par` | efeito e valor de gema discordando |
| `ck_vault_item_gema_refino` | gema em item comum com refino abaixo de 10 |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_vault_item_indice (item_index)` | lado filho: itens por índice no baú |

Trigger `trg_vault_item_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0009_game_inventory.md` — arquivo `0009_game_inventory.sql`.

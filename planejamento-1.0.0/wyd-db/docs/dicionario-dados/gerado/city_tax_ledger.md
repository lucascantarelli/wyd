> Pergunta: "o que `game.city_tax_ledger` registra e como o motivo fecha a lista?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Ledger do imposto arrecadado por cidade: cada recolhimento vira uma linha, com motivo e, quando houver,
a loja e o personagem que recolheu.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `city_code` | `TEXT` | não | — | cidade que arrecadou; `RESTRICT` |
| `shop_id` | `BIGINT` | sim | — | loja de origem; `SET NULL` |
| `amount` | `BIGINT` | não | — | valor recolhido, `>= 0` |
| `reason` | `TEXT` | não | — | `shop_sale`, `npc_sale`, `portal` ou `collection` |
| `collected_by` | `BIGINT` | sim | — | personagem que recolheu; `SET NULL` |
| `occurred_at` | `TIMESTAMPTZ` | não | `now()` | instante do recolhimento |

## Chave primária

`pk_city_tax_ledger (id)` — cada recolhimento é uma linha; não há chave natural.

## FKs de saída e de entrada

- **Saída:** `fk_city_tax_ledger_city`: `city_code → game.city (code)`, `RESTRICT`/`NO ACTION` — cidade com
  imposto registrado não é apagada; `fk_city_tax_ledger_shop`: `shop_id → game.personal_shop (id)`,
  `SET NULL`/`NO ACTION`; `fk_city_tax_ledger_quem`: `collected_by → game.character (id)`,
  `SET NULL`/`NO ACTION`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_city_tax_ledger_valor` | valor negativo |
| `ck_city_tax_ledger_motivo` | motivo fora dos quatro valores |

O planejamento não documenta o significado de cada motivo.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_city_tax_ledger_cidade (city_code, occurred_at DESC)` | imposto recente de uma cidade |

Sem `updated_at` nem trigger: o carimbo é só `occurred_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011-game-economy.sql`.

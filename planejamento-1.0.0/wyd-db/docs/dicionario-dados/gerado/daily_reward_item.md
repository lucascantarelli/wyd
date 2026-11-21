> Pergunta: "o que `portal.daily_reward_item` guarda e por que a recompensa exige item ou ouro?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Definição de recompensa do bônus diário: o que cada dia entrega — item, ouro ou os dois —, e se o
resgate é por conta.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `day` | `SMALLINT` | sim | — | dia 1–7; nulo = fora do ciclo |
| `item_index` | `INTEGER` | sim | — | item entregue; sem FK |
| `gold` | `BIGINT` | não | `0` | ouro entregue, `>= 0` |
| `qty` | `SMALLINT` | não | `1` | quantidade do item, `> 0` |
| `account_wide` | `BOOLEAN` | não | `true` | resgate por conta |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da recompensa |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_daily_reward_item (id)` — cada recompensa é uma linha; não há unicidade por dia.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** `daily_reward_claim` (`fk_daily_reward_claim_reward`), com `RESTRICT` — apagar recompensa
  já resgatada falha alto.
- **Sem FK para conteúdo:** `item_index` é de `content.item_catalog` (`0014`, posterior a `0011`).

## Unicidade

Só a PK. O `day` é dado, não chave: mais de uma recompensa pode apontar para o mesmo dia.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_daily_reward_item_dia` | dia preenchido e fora de 1–7 |
| `ck_daily_reward_item_ouro` | ouro negativo |
| `ck_daily_reward_item_qtd` | quantidade menor ou igual a zero |
| `ck_daily_reward_item_conteudo` | recompensa sem item e sem ouro (`item_index IS NULL AND gold = 0`) |

## Índices

Nenhum além da PK. Trigger `trg_daily_reward_item_set_updated_at`.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`, `wyd_read`.
- **Não lê:** `wyd_game` (sem `USAGE` em `portal`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011_game_economy.sql`.

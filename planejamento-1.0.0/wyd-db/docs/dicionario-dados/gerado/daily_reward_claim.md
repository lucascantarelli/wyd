> Pergunta: "o que `portal.daily_reward_claim` registra e como o banco impede dois resgates no mesmo dia?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Registro de resgate do bônus diário: uma linha por conta e por dia UTC. O `23505` da chave única é o sinal
de "já resgatou".

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | conta que resgatou; sem FK |
| `reward_item_id` | `BIGINT` | sim | — | recompensa resgatada; `RESTRICT` |
| `claim_date` | `DATE` | não | — | dia UTC do resgate |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do resgate |

## Chave primária

`pk_daily_reward_claim (id)` — cada resgate é uma linha; o par conta/dia é único por constraint à parte.

## FKs de saída e de entrada

- **Saída:** `fk_daily_reward_claim_reward`: `reward_item_id → portal.daily_reward_item (id)`,
  `RESTRICT`/`NO ACTION`.
- **Entrada:** nenhuma.
- **Sem FK para `game.account`:** `account_id` é número por decisão de fronteira (`P-17`, `A19`); a conta
  é validada na borda do BFF.

## Unicidade

`uq_daily_reward_claim_dia (account_id, claim_date)` — unique **total**: um resgate por conta e dia.

## `CHECK`s

Nenhum. A chave única é a regra; não há faixa de valor a validar.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `uq_daily_reward_claim_dia (account_id, claim_date)` | resgate do dia por conta (última e regra de um só) |

Sem `updated_at` nem trigger: o carimbo é só `created_at`.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`, `wyd_read`.
- **Não lê:** `wyd_game` (sem `USAGE` em `portal`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011-game-economy.sql`.

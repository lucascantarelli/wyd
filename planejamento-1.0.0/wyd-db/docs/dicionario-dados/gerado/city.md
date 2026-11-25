> Pergunta: "o que `game.city` guarda e qual faixa `map_id` aceita?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Cidade do jogo: código, nome, mapa opcional, marca de capital e alíquota de imposto em pontos-base.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `code` | `TEXT` | não | — | PK; `snake_case` de 3–16 caracteres |
| `name` | `TEXT` | não | — | nome de 1–32 caracteres após aparar |
| `map_id` | `SMALLINT` | sim | — | mapa 0–255 quando preenchido |
| `is_capital` | `BOOLEAN` | não | `false` | marca de capital |
| `tax_rate_bps` | `INTEGER` | não | `1000` | alíquota em bps, 0–10000 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_city (code)` — o código textual é a chave; não há id numérico de cidade.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada, `RESTRICT`:** `personal_shop`, `city_tax_ledger` e `castle_state` — cidade com loja, imposto
  ou castelo não é apagada.
- **Sem FK para conteúdo:** `map_id` não referencia `content.map_definition` (`0024`).

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_city_code` | código fora de `^[a-z][a-z0-9_]{2,15}$` |
| `ck_city_nome` | nome vazio ou acima de 32 caracteres |
| `ck_city_mapa` | `map_id` fora de 0–255 quando preenchido |
| `ck_city_imposto` | alíquota fora de 0–10000 bps |

## Índices

Nenhum além da PK. Trigger `trg_city_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).
- **Administra:** `wyd_admin` — `SELECT, UPDATE` concedidos por `0032`; sem `INSERT`/`DELETE`.

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011-game-economy.sql`.

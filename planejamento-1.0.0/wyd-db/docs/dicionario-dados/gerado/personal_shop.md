> Pergunta: "o que `game.personal_shop` guarda e como o banco garante uma loja aberta por personagem?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Loja pessoal de um personagem em uma cidade, com estado aberta/fechada, alíquota e imposto pago. Fechar
não apaga: o histórico de imposto fica no ledger da cidade.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `character_id` | `BIGINT` | não | — | dono da loja; `CASCADE` |
| `city_code` | `TEXT` | não | — | cidade que sedia; `RESTRICT` |
| `name` | `TEXT` | não | — | nome de 1–24 caracteres após aparar |
| `is_open` | `BOOLEAN` | não | `true` | aberta ou fechada |
| `tax_bps` | `INTEGER` | não | `1000` | alíquota em bps, 0–10000 |
| `tax_paid` | `BIGINT` | não | `0` | imposto já pago, `>= 0` |
| `opened_at` | `TIMESTAMPTZ` | não | `now()` | abertura |
| `closed_at` | `TIMESTAMPTZ` | sim | — | fechamento; nulo = aberta |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_personal_shop (id)` — cada abertura é uma linha; a loja fechada continua com histórico.

## FKs de saída e de entrada

- **Saída:** `fk_personal_shop_character`: `character_id → game.character (id)`, `CASCADE`/`NO ACTION`;
  `fk_personal_shop_city`: `city_code → game.city (code)`, `RESTRICT`/`NO ACTION` — cidade com loja não
  é apagada.
- **Entrada:** `city_tax_ledger` (`fk_city_tax_ledger_shop`), com `SET NULL` — apagar a loja preserva o
  imposto, sem a referência.

## Unicidade

- `ux_personal_shop_aberta (character_id) WHERE is_open` — único **parcial**: no máximo uma loja
  **aberta** por personagem. Fechadas podem se acumular.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_personal_shop_nome` | nome vazio ou acima de 24 caracteres |
| `ck_personal_shop_imposto` | alíquota fora de 0–10000 bps |
| `ck_personal_shop_pago` | imposto pago negativo |
| `ck_personal_shop_fechamento` | fechamento anterior à abertura |
| `ck_personal_shop_estado` | `is_open` e `closed_at` discordando (`is_open = (closed_at IS NULL)`) |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_personal_shop_aberta (character_id) WHERE is_open` | loja aberta do personagem (e regra de uma só) |
| `ix_personal_shop_cidade (city_code) WHERE is_open` | lojas abertas de uma cidade |

Trigger `trg_personal_shop_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011-game-economy.sql`.

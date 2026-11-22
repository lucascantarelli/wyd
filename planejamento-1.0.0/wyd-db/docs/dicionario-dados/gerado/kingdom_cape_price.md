> Pergunta: "o que `portal.kingdom_cape_price` guarda e por que é linha única?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.14; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Tabela de preço da capa de reino: custo nas duas moedas e versão da tabela. Estado único, como
`content.content_version` e `guild_tower`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | `1` | PK; `CHECK (id = 1)` |
| `version` | `BIGINT` | não | `1` | versão da tabela de preço, `>= 1` |
| `hekalotia_cost` | `INTEGER` | não | `8` | custo em hekalotia, 4–32 |
| `akelonia_cost` | `INTEGER` | não | `8` | custo em akelonia, 4–32 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_kingdom_cape_price (id)` com `ck_kingdom_cape_price_singleton (id = 1)` — uma linha só; o preço não é
entidade.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** nenhuma.

## Unicidade

Só a PK, restrita a uma linha pelo `CHECK`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_kingdom_cape_price_singleton` | qualquer `id` diferente de 1 |
| `ck_kingdom_cape_price_versao` | versão menor que 1 |
| `ck_kingdom_cape_price_hekalotia` | custo em hekalotia fora de 4–32 |
| `ck_kingdom_cape_price_akelonia` | custo em akelonia fora de 4–32 |

`version` não é a versão do conteúdo: é a versão da própria tabela de preço.

## Índices

Nenhum além da PK. Trigger `trg_kingdom_cape_price_set_updated_at`.

## Papéis

- **Escreve:** `wyd_portal`.
- **Lê:** `wyd_portal`, `wyd_read`.
- **Não lê:** `wyd_game` (sem `USAGE` em `portal`).

## Migration

Forma (dona): `migrations/0011-game-economy.md` — arquivo `0011_game_economy.sql`.

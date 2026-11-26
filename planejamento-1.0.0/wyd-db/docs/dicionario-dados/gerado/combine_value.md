> Pergunta: "o que `content.combine_value` guarda por família e chave e por que cada linha tem só uma unidade?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.22; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Valor base de composição por família e chave, com **uma** unidade por linha: taxa em pontos-base
(`rate_bps`) **ou** custo em moeda (`coin`), nunca as duas e nunca nenhuma. Tabela seed-only e
read-only em runtime; a alteração por operação é `content.combine_value_override` (`0022`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `family` | `TEXT` | não | — | família da regra, 1–64 caracteres após aparar; PK com `key` |
| `key` | `TEXT` | não | — | chave estável dentro da família, 1–96 caracteres após aparar |
| `rate_bps` | `INTEGER` | sim | — | taxa, presente quando a unidade é percentual (0–10000) |
| `coin` | `BIGINT` | sim | — | custo em moeda na menor unidade, presente quando a unidade é moeda |

Sem `created_at`/`updated_at` e sem trigger: base de seed, sem escrita do staff em runtime
(`migrations/0021_content_rules.md` §7).

## Chave primária

`pk_combine_value (family, key)` — o par é a identidade (`P-04`); não há coluna de número nem `id`.

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada (1 FK):** `fk_combine_value_override_base` em `content.combine_value_override` (`0022`),
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE`. Apagar a base apaga o override.
- **Não há FK para item nem para família:** `family`/`key` são texto, e o significado de cada família é
  `H` — não existe tabela de família nesta série.

## Unicidade

Só a PK: a mesma família não repete chave.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_combine_value_family` | `family` vazia, só com espaço, ou acima de 64 caracteres |
| `ck_combine_value_key` | `key` vazia, só com espaço, ou acima de 96 caracteres |
| `ck_combine_value_one_unit` | as duas colunas de valor preenchidas, nenhuma preenchida, `rate_bps` fora de 0–10000 ou `coin` negativo |

A constraint de unidade é uma disjunção: ou `rate_bps` preenchido, `coin` nulo e taxa na faixa, ou
`coin` preenchido, `rate_bps` nulo e moeda não negativa.

## Índices

Nenhum além do índice da PK, que cobre a busca por `(family, key)` e a FK do override.

## Armadilha de leitura

Uma linha com `rate_bps` **e** `coin` não existe por construção, então "somar taxa e custo" é leitura
errada: são unidades alternativas, e o tipo de valor é decidido por qual coluna está preenchida. `coin`
está na menor unidade da moeda — a conversão é do runtime. E família/chave não são enums: o conjunto é
da ficha do legado (`H`), sem verificação no cliente.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0023`), que está `bloqueada`; nenhum `GRANT` de escrita em
  runtime (`migrations/0021_content_rules.md` §7).
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0021_content_rules.md` — arquivo `0021_content_rules.sql`.

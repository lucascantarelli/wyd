> Pergunta: "o que `portal.market_reputation` agrega por conta e por que soma e contagem de notas andam juntas?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Agregado de mercado por conta: vendas concluídas, vendas disputadas, disputas perdidas e a soma/contagem de
notas que dá a média. Uma linha por conta, mantida por `UPDATE` a cada evento — não é visão nem cálculo em
consulta.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | conta agregada; PK e sem FK |
| `sales_completed` | `INTEGER` | não | `0` | vendas liquidadas |
| `sales_disputed` | `INTEGER` | não | `0` | vendas com disputa aberta |
| `disputes_lost` | `INTEGER` | não | `0` | disputas decididas contra o vendedor |
| `rating_sum` | `BIGINT` | não | `0` | soma das notas recebidas |
| `rating_count` | `INTEGER` | não | `0` | quantidade de notas |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_market_reputation (account_id)` — a conta é a chave; não há linha sem conta nem `id` artificial.

## FKs de saída e de entrada

- **Saída:** nenhuma. `account_id` é número de `game.account` e `portal` não tem `USAGE` em `game`
  (`P-17`).
- **Entrada:** nenhuma; ninguém aponta para o agregado.

## Unicidade

Só a PK: uma linha por conta. A média é derivada (`rating_sum / rating_count`), não coluna.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_reputation_contadores` | qualquer contador negativo (`sales_completed`, `sales_disputed`, `disputes_lost`, `rating_count`) |
| `ck_market_reputation_notas` | `rating_count = 0` com `rating_sum <> 0`, ou o contrário |

`ck_market_reputation_notas` é equivalência nas duas direções: média de zero avaliação não existe, e soma
sem contagem é resto de escrita errada.

## Índices

Nenhum além da PK. A leitura é sempre por conta; ranking por média, se existir, é consulta nova e pede
índice novo.

## Armadilha de leitura

`rating_sum` **não tem piso próprio** no DDL: o que impede soma negativa é a nota ser 1–5 em
`market_review`, e escrita que subtraia sem a linha de origem não é barrada aqui. `rating_count = 0` com
`rating_sum = 0` é conta sem nota, não reputação zero — a média não é 0, é inexistente, e dividir por
`rating_count` sem testar zero quebra o cálculo. `sales_completed` e `sales_disputed` não são exclusivos:
uma venda pode ter sido concluída e depois disputada. O agregado é **cache**: reconstruí-lo a partir de
`market_transaction`, `market_dispute` e `market_review` é possível, e divergência é defeito de escrita.
`trg_market_reputation_set_updated_at` mantém `updated_at`.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`,
  pelo BFF.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034_portal_market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

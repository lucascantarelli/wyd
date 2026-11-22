> Pergunta: "o que `portal.market_fee_rule` guarda da taxa da casa e por que a vigência é a chave do histórico?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Tabela de **taxa da casa** em pontos-base, versionada por instante de vigência. O anúncio congela a taxa
que valeu na publicação (`market_listing.fee_bps`) e a liquidação congela de novo
(`market_transaction.fee_bps`), então esta tabela é configuração histórica, não o valor corrente lido pelo
cálculo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `fee_bps` | `INTEGER` | não | — | taxa em pontos-base, 0–10000 |
| `effective_from` | `TIMESTAMPTZ` | não | — | início da vigência; único |
| `created_by_person_id` | `BIGINT` | sim | — | pessoa que registrou a regra; sem FK |
| `note` | `TEXT` | não | `''` | observação livre |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |

## Chave primária

`pk_market_fee_rule (id)` — identidade sintética; a chave de negócio é a vigência, que tem único próprio.

## FKs de saída e de entrada

- **Saída:** nenhuma, inclusive para `portal.person`: `created_by_person_id` guarda o número e o DDL do
  lote não declara referência. O autor da regra é rastro, e fechar pessoa (`A12`, soft-delete com
  anonimização) não pode barrar a configuração.
- **Entrada:** nenhuma; anúncio e transação copiam `fee_bps`, sem apontar para a regra.

## Unicidade

`uq_market_fee_rule_vigencia UNIQUE (effective_from)` — **uma** regra por instante de vigência. Duas
regras começando no mesmo segundo seriam empate sem desempate; a nova taxa entra com vigência nova, nunca
sobrescrevendo a anterior.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_fee_rule_bps` | `fee_bps` fora de 0–10000 |

O teto de 10000 é 100% — a `CHECK` impede taxa acima do valor da venda; não impede taxa abusiva, que é
decisão de operação.

## Índices

Nenhum além da PK e do único de vigência: a consulta "taxa vigente" é
`ORDER BY effective_from DESC LIMIT 1` e usa `uq_market_fee_rule_vigencia`. A tabela não tem `updated_at`
nem trigger — regra errada se corrige com vigência nova, não com `UPDATE`.

## Armadilha de leitura

"Taxa vigente" **não** é a linha de maior `id`: é a de maior `effective_from` que já começou
(`effective_from <= now()`), e uma regra futura pode existir antes da hora. `market_listing.fee_bps` e
`market_transaction.fee_bps` são **cópias congeladas**, e comparar com esta tabela é auditoria, não
recomposição do preço. `note` vazio é o normal, não ausência de decisão.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`;
  na prática o BFF só insere.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

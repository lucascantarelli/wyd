> Pergunta: "o que `portal.market_review` guarda da avaliação e por que moderação oculta em vez de apagar?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Avaliação do comprador sobre uma transação liquidada: nota de 1 a 5, comentário opcional e estado de
visibilidade. É a fonte do agregado de `market_reputation`, e **uma** por transação.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `transaction_id` | `BIGINT` | não | — | transação avaliada; FK `RESTRICT`, única |
| `reviewer_account_id` | `BIGINT` | não | — | conta que avalia; sem FK |
| `seller_account_id` | `BIGINT` | não | — | conta avaliada; sem FK |
| `rating` | `SMALLINT` | não | — | nota de 1 a 5 |
| `comment` | `TEXT` | sim | — | comentário livre, até 500 caracteres |
| `state` | `TEXT` | não | `'visivel'` | visibilidade editorial |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |

## Chave primária

`pk_market_review (id)` — identidade sintética; a unicidade de negócio é a transação, e tem único próprio.

## FKs de saída e de entrada

- **Saída (1):** `fk_market_review_transaction`: `transaction_id → portal.market_transaction (id)`,
  `ON DELETE RESTRICT`, `ON UPDATE NO ACTION` — avaliação não sobrevive à transação que a originou, e a
  transação não é apagada por causa dela.
- **Entrada:** nenhuma.

## Unicidade

`uq_market_review_transaction UNIQUE (transaction_id)` — **uma** avaliação por transação. Não há
`UNIQUE (transaction_id, reviewer_account_id)` porque o par seria redundante: a transação já tem um
comprador.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_review_nota` | `rating` fora de 1–5 |
| `ck_market_review_comentario` | `comment` acima de 500 caracteres |
| `ck_market_review_estado` | `state` fora de `visivel` e `oculto` |

`comment` vazio é aceito: a `CHECK` limita tamanho, não exige conteúdo.

## Índices

Nenhum além da PK e do único de transação. A leitura por vendedor (`seller_account_id`) e a média por conta
saem do agregado em `market_reputation`, não de varredura aqui.

## Armadilha de leitura

`state = 'oculto'` é **moderação**, não remoção: a linha continua contando para a prova da transação, e
`market_reputation.rating_sum`/`rating_count` **não** são recalculados ao ocultar — média que ignora
ocultação é decisão de produto ainda não tomada. `reviewer_account_id` e `seller_account_id` são cópias da
transação, não FKs; divergência entre os três é defeito de escrita. E nota não se edita: mudar a avaliação
seria decisão nova, e hoje o schema só permite mudar `state`.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`;
  a moderação usa `UPDATE` de `state`.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.

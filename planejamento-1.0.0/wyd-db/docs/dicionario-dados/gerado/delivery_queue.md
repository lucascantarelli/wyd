> Pergunta: "o que `ops.delivery_queue` guarda de um pedido da web ao jogo e o que impede a entrega dupla?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.16; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Fila única pela qual a web pede algo ao jogo — item, ouro ou nível —, sem que o portal escreva em
`game`: o portal enfileira e acompanha, o servidor drena e aplica. Guarda o pedido e o ponto do ciclo em
que ele está, não o efeito: a concessão é do `wyd-server`/`wyd-admin`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | identidade da entrega; é também a ordem FIFO |
| `idempotency_key` | `UUID` | não | — | chave do pedido gerada pelo BFF (`M17`) |
| `kind` | `TEXT` | não | — | `item`, `gold` ou `level` |
| `account_id` | `BIGINT` | sim | — | conta destino de `gold` (FK) |
| `character_id` | `BIGINT` | sim | — | personagem destino de `item` e `level` (FK) |
| `item_index` | `INTEGER` | sim | — | item da concessão; catálogo em `0014`, sem FK |
| `qty` | `INTEGER` | sim | — | quantidade da concessão de item |
| `gold_amount` | `BIGINT` | sim | — | ouro creditado na carteira da conta |
| `level_delta` | `INTEGER` | sim | — | níveis somados ao personagem |
| `status` | `TEXT` | não | `'pending'` | `pending` → `claimed` → `applied`/`failed` |
| `attempt_count` | `SMALLINT` | não | `0` | quantas vezes o dreno reivindicou a linha |
| `claimed_at` | `TIMESTAMPTZ` | sim | — | instante em que o dreno tomou a linha |
| `closed_at` | `TIMESTAMPTZ` | sim | — | instante em que a linha deixou a fila |
| `failure_reason` | `TEXT` | sim | — | motivo curto da falha; só existe em `failed` |
| `created_by_person_id` | `BIGINT` | sim | — | staff que pediu pela web (FK) |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | chegada do pedido |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbo pelo trigger |

## Chave primária

`pk_delivery_queue (id)` — identidade da entrega, e a ordem que o dreno usa sem `Sort`.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (`ON UPDATE NO ACTION`):** `fk_delivery_queue_account`
  (`account_id → game.account (id)`), `fk_delivery_queue_character`
  (`character_id → game.character (id)`) e `fk_delivery_queue_autor`
  (`created_by_person_id → portal.person (id)`) — conta, personagem ou pessoa com concessão pendente
  não é apagada.
- **Entrada:** nenhuma.
- **Sem FK para `content.item_catalog`:** o catálogo é `0014`, posterior na série, e a FK seria migração
  aditiva (`0014-content-item.md` §10). O `item_index` fica solto e quem o valida é o dreno.

## Unicidade

`uq_delivery_queue_idem (idempotency_key)` — reenvio do navegador ou retry do BFF devolve a mesma
entrega em vez de enfileirar duas (`M17`).

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_delivery_queue_tipo` | `kind` fora de `item`/`gold`/`level` |
| `ck_delivery_queue_estado` | `status` fora dos quatro do ciclo |
| `ck_delivery_queue_campos` | concessão sem o destino do seu tipo, ou com campo de outro tipo preenchido |
| `ck_delivery_queue_tentativas` | `attempt_count` negativo |
| `ck_delivery_queue_reivindicacao` | `pending` com `claimed_at`, ou linha fora de `pending` sem ele |
| `ck_delivery_queue_fechamento` | `applied`/`failed` sem `closed_at`, ou linha aberta com carimbo |
| `ck_delivery_queue_motivo` | `failed` sem motivo, ou motivo em linha que não falhou |
| `ck_delivery_queue_motivo_tamanho` | `failure_reason` fora de 1–200 caracteres |
| `ck_delivery_queue_ordem` | `closed_at` anterior a `claimed_at` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_delivery_queue_pendentes (id) WHERE status = 'pending'` | dreno lê os pendentes em ordem, sem `Sort` |
| `ix_delivery_queue_conta (account_id) WHERE account_id IS NOT NULL` | FK do lado filho (`P-15`); nulo não entra no índice |
| `ix_delivery_queue_personagem (character_id) WHERE character_id IS NOT NULL` | FK do lado filho (`P-15`); nulo não entra no índice |
| `ix_delivery_queue_autor (created_by_person_id) WHERE created_by_person_id IS NOT NULL` | FK do lado filho (`P-15`); nulo não entra no índice |
| `ix_delivery_queue_fechadas (closed_at) WHERE closed_at IS NOT NULL` | expurgo por retenção varre só o que fechou |
| `ix_delivery_queue_status (status, created_at DESC)` | acompanhamento por estado, mais recente primeiro |

Trigger `trg_delivery_queue_set_updated_at`.

## Armadilha de leitura

O banco garante a chave única e a coerência de estado, **não** que a entrega seja aplicada uma vez no
jogo: quem prova é o dreno com `SELECT … FOR UPDATE SKIP LOCKED` em transação curta e o teste com a
mesma linha reivindicada por dois processos (`M25`). `attempt_count` registra tentativas; o teto é do
dreno, não do schema. E `DELETE` não existe em runtime: o expurgo de `applied`/`failed` é rotina de
operação, 30 dias após `closed_at`, e `pending` nunca é expurgada.

## Papéis

Esta é a única tabela do dicionário com `GRANT` explícito de escrita para papel de runtime.

- **Enfileira:** `wyd_portal` (`SELECT, INSERT`) — o jogo não enfileira.
- **Drena:** `wyd_game` (`SELECT, UPDATE`) — marca `claimed`, `applied` ou `failed`.
- **Lê:** `wyd_read` (`SELECT` por default privilege de `ops`, `0001-bootstrap.md` §7).
- **Não escreve:** `wyd_admin`; `0032` concede `INSERT` e `SELECT (id)` só em `ops.audit_log`. Dono da
  permissão: `migrations/0012-ops-delivery.md` §8.

## Migration

Forma (dona): `migrations/0012-ops-delivery.md` — arquivo `0012-ops-delivery.sql`.

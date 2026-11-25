> Pergunta: "o que `ops.idempotency_record` guarda por chave e por que `in_progress` nunca é commitado?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.33; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Registro durável de idempotência do contrato C4 do `wyd-admin`: para cada escopo `(caller, method, route,
key)`, guarda o hash do corpo, o estado e — quando concluída — status e corpo da resposta. A linha nasce e
fecha **na mesma transação da ação**; `in_progress` não é commitado (`M30`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `caller` | `TEXT` | não | — | identidade do serviço chamador; PK |
| `method` | `TEXT` | não | — | `POST` ou `PUT`; PK |
| `route` | `TEXT` | não | — | rota-modelo com o parâmetro literal, 1–128; PK |
| `key` | `TEXT` | não | — | chave do chamador, 16–64 de `[A-Za-z0-9_-]`; PK |
| `body_sha256` | `TEXT` | não | — | sha256 hex do JSON canônico do corpo |
| `state` | `TEXT` | não | — | `in_progress`, `completed` ou `failed` |
| `status_code` | `SMALLINT` | sim | — | status HTTP devolvido; só em `completed` |
| `response_body` | `JSONB` | sim | — | corpo devolvido; só em `completed` |
| `audit_id` | `BIGINT` | sim | — | linha de auditoria da mesma transação, **sem FK** |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação; base do expurgo |
| `closed_at` | `TIMESTAMPTZ` | sim | — | fechamento; nulo **só** em `in_progress` |

## Chave primária

`pk_idempotency_record (caller, method, route, key)` — o escopo da chave do C4 §2 é a própria PK. Não há
`id`: duas chamadas com a mesma chave são a mesma linha, e a PK é o que faz a segunda `INSERT` esperar o
`COMMIT` da primeira.

## FKs de saída e de entrada

- **Saída:** nenhuma — **não há FK para `ops.audit_log`**, por desenho: como o ator da auditoria, o
  registro sobrevive ao alvo e não bloqueia escrita na auditoria append-only.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o registro.

## Unicidade

Só a PK, e ela **é** a unicidade de negócio: não existe segundo único de chave, porque o escopo completo
já é a identidade. Chave repetida no mesmo escopo não cria linha nova — encontra a existente ou esbarra na
`INSERT` em andamento.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_idempotency_record_metodo` | `method` fora de `POST` e `PUT` |
| `ck_idempotency_record_chave` | `key` fora de `^[A-Za-z0-9_-]{16,64}$` |
| `ck_idempotency_record_corpo` | `body_sha256` fora de 64 hex minúsculos |
| `ck_idempotency_record_rota` | `route` vazia ou acima de 128 caracteres |
| `ck_idempotency_record_estado` | `state` fora de `in_progress`, `completed` e `failed` |
| `ck_idempotency_record_resultado` | `completed` sem `status_code` **ou** sem `response_body`, ou não-`completed` com algum deles |
| `ck_idempotency_record_fechamento` | `in_progress` com `closed_at`, ou não-`in_progress` sem `closed_at` |

`failed` também não guarda status nem corpo: o `CHECK` de resultado exige os dois **só** em `completed`, e
a linha falha fecha com `closed_at` preenchido e resposta vazia.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_idempotency_record_criado (created_at)` | expurgo por idade (`DELETE … WHERE created_at < now() - interval '7 days'`), que varre por retenção e não por estado |

## Armadilha de leitura

`in_progress` existe no DDL, mas **nunca é visível**: a linha só é commitada já fechada, e o `409
ADM_IDEMPOTENCY_IN_PROGRESS` vem do bloqueio da `INSERT` duplicada na PK, não de uma linha lida em
andamento. Não há chave presa nem lease a expirar — transação abortada leva a linha junto. `state =
'failed'` permite repetir a ação e a repetição reusa a linha; `completed` devolve o corpo gravado, e corpo
diferente para a mesma chave é `409`, não replay. `audit_id` sem FK pode apontar para linha de auditoria
que já foi expurgada (730 dias) — o registro não é a trilha, é o cache de resposta. E `route` é a
rota-modelo com parâmetro literal: a URL concreta não entra na chave.

## Papéis

- **Escreve:** `wyd_admin` — `SELECT, INSERT, UPDATE` concedidos pelo próprio `0033`; `DELETE` **não** é
  de runtime, e o expurgo é rotina de operação com papel de manutenção (`M25` é o precedente).
- **Lê:** `wyd_read` — `SELECT` herdado do default privilege de `ops` (`0001-bootstrap.md` §7).
- **Não escrevem:** `wyd_game` e `wyd_portal` têm `USAGE` em `ops` mas nenhum privilégio de tabela aqui
  (`42501`).

## Migration

Forma (dona): `migrations/0033-ops-idempotency.md` — arquivo `0033-ops-idempotency.sql`; DDL em
`ddl-confirmado.md` §5.33.

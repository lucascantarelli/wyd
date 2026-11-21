> Pergunta: "qual é a forma exata do contrato C4 — rotas, payload, idempotência, códigos de erro e prazos?"
> Público: Desenvolvedor do `wyd-admin` e do BFF do portal
> Dono: `wyd-admin` (forma), arquitetura (direção)
> Base: `wyd-portal/docs/arquitetura/fronteira.md` §3 (C4); `LAC-POR-01` (2026-11-20); `ddl-confirmado.md` §5.2/§5.5/§5.14
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — a direção do C4 está decidida; esta forma aguarda aprovação.

## 1. Base e cabeçalhos

- Base: `https://<admin-interno>/internal/v1` — nunca exposta à internet.
- Corpo e resposta: `application/json; charset=utf-8`.
- `Authorization: Bearer <token-de-serviço>` — obrigatório em toda rota.
- `X-Staff-Person-Id: <int>` — pessoa autenticada pelo portal; vira autor em `ops.audit_log`.
- `X-Trace-Id: <uuid>` — obrigatório; repetido na resposta e no log. Sem ele, `400`.
- `Idempotency-Key: <16–64 chars [A-Za-z0-9_-]>` — obrigatório em `POST`/`PUT`.

O admin **não** autentica pessoa: confia no token de serviço e no `X-Staff-Person-Id` assinado pelo
portal. O mecanismo do token/assinatura é `LAC-ADM-04`.

## 2. Idempotência

| Regra | Efeito |
|---|---|
| Escopo | `(chamador, método, rota-modelo, chave)` — a mesma chave em rotas diferentes não colide |
| Chave ausente em mutação | `428 ADM_IDEMPOTENCY_REQUIRED` |
| Chave malformada | `400 ADM_INVALID_HEADER` |
| Mesma chave, mesmo corpo, concluída | devolve **status e corpo gravados**, com `Idempotency-Replayed: true` |
| Mesma chave, corpo diferente | `409 ADM_IDEMPOTENCY_KEY_REUSED` |
| Mesma chave, ainda em execução | `409 ADM_IDEMPOTENCY_IN_PROGRESS`, `Retry-After: 1` |
| Retenção do registro | 24 h (proposta; `LAC-ADM-06`) |

O corpo é comparado por `sha256` do JSON canônico (chaves ordenadas, sem espaço). O registro precisa ser
**durável e transacional com a ação**; sem tabela aprovada, isso é `LAC-ADM-02` e bloqueia a
implementação da idempotência — não a do contrato.

`GET /actions/{key}` permite reconciliação depois de timeout; não é replay e não escreve nada.

## 3. Erro tipado

Forma única:

```json
{ "error": { "code": "ADM_STATE_MISMATCH", "message": "conta já está bloqueada",
             "details": { "current": true }, "trace_id": "..." } }
```

| HTTP | `code` | Quando |
|---:|---|---|
| 400 | `ADM_INVALID_BODY` | JSON inválido ou campo fora do tipo |
| 400 | `ADM_INVALID_HEADER` | `X-Trace-Id`/`Idempotency-Key` ausente ou malformado |
| 401 | `ADM_UNAUTHENTICATED` | token de serviço ausente/inválido |
| 403 | `ADM_FORBIDDEN` | papel do staff insuficiente para a ação |
| 404 | `ADM_NOT_FOUND` | conta, sessão, canal, cidade, item ou chave de ação inexistente |
| 409 | `ADM_STATE_MISMATCH` | `expected_*` diverge do estado atual |
| 409 | `ADM_IDEMPOTENCY_KEY_REUSED` | mesma chave com corpo diferente |
| 409 | `ADM_IDEMPOTENCY_IN_PROGRESS` | mesma chave ainda executando |
| 409 | `ADM_SESSION_NOT_ALIVE` | kick de sessão já liberada |
| 422 | `ADM_RULE_VIOLATION` | regra de domínio: saldo fora da faixa, canal 0, taxa fora de 0–10.000 |
| 428 | `ADM_IDEMPOTENCY_REQUIRED` | mutação sem `Idempotency-Key` |
| 500 | `ADM_INTERNAL` | falha não classificada; nada foi gravado |
| 503 | `ADM_DB_UNAVAILABLE` | banco indisponível; nada foi gravado |
| 503 | `ADM_SERVER_UNAVAILABLE` | `wyd-server` indisponível; nada foi gravado |
| 504 | `ADM_UPSTREAM_TIMEOUT` | `wyd-server` não respondeu no prazo; estado é incerto, consulte a chave |

Nunca vaza `SQLSTATE`, nome de constraint, DSN ou PII. `23505`/`23514`/`23503` são traduzidos para
`409`/`422` antes de sair. Erro **não** vira escrita local compensatória (C4).

## 4. Rotas de ação

Todas exigem os cabeçalhos de §1 e, sendo mutação, `Idempotency-Key`. Sucesso é `200` com o estado
resultante.

| # | Método e rota | Papel mínimo |
|---|---|---|
| 1 | `POST /accounts/{account_id}/moderation` | `moderator` |
| 2 | `POST /accounts/{account_id}/wallet-adjustments` | `admin` |
| 3 | `PUT /channels/{group_id}/{channel}` | `admin` |
| 4 | `POST /sessions/{session_id}/kick` | `moderator` |
| 5 | `POST /broadcasts` | `moderator` |
| 6 | `PUT /cities/{code}/tax` | `admin` |
| 7 | `PUT /items/{item_index}/prices/{npc_id}` | `admin` |

### 4.1 `POST /accounts/{account_id}/moderation`

```json
{ "blocked": true, "reason": "multibox confirmado", "expected_blocked": false }
```

- `reason`: 1–500 caracteres, obrigatório quando `blocked = true`; ignorado quando `false`.
- `expected_blocked`: obrigatório; evita sobrescrever decisão de outro staff.
- Efeito: `game.account_moderation` (upsert) + `ops.audit_log`, na mesma transação.
- `200`: `{ "account_id": 42, "blocked": true, "blocked_at": "…", "audit_id": 9001 }`.
- Erros: `404`, `409 ADM_STATE_MISMATCH`, `422 ADM_RULE_VIOLATION` (motivo vazio).

### 4.2 `POST /accounts/{account_id}/wallet-adjustments`

```json
{ "delta": -1500, "reason": "estorno manual", "expected_coin": 4000 }
```

- `delta` ≠ 0; `reason` 1–64; `expected_coin` obrigatório.
- Efeito: trava a carteira, confere `expected_coin`, grava `account_wallet.coin` e uma linha em
  `account_ledger` com `balance_after`; audita.
- `200`: `{ "account_id": 42, "coin": 2500, "ledger_id": 771, "audit_id": 9002 }`.
- Erros: `404 ADM_NOT_FOUND` (conta ou carteira), `409 ADM_STATE_MISMATCH`, `422 ADM_RULE_VIOLATION`
  (saldo resultante fora de `0..2147483647`).

### 4.3 `PUT /channels/{group_id}/{channel}`

```json
{ "max_players": 1000, "maintenance": false, "role": "normal", "enabled": true }
```

- `channel` 1–10; `channel = 0` é a linha de status e responde `422 ADM_RULE_VIOLATION`.
- `role`: `normal` ou `verde`; `max_players` ≥ 0.
- Efeito: `game.server_channel` + auditoria.
- `200`: o estado gravado. Erros: `404` (canal ausente), `422` (faixa/papel).

### 4.4 `POST /sessions/{session_id}/kick`

```json
{ "note": "encerrando por manutenção" }
```

- `note` opcional, até 200 caracteres; vai para o log do admin, não para o banco.
- Efeito: pede o encerramento ao `wyd-server`; **só depois do ack** grava `released_at` e
  `released_reason = 'admin'`; audita.
- `200`: `{ "session_id": 88, "released_at": "…", "audit_id": 9003 }`.
- Erros: `404`, `409 ADM_SESSION_NOT_ALIVE`, `503 ADM_SERVER_UNAVAILABLE`, `504 ADM_UPSTREAM_TIMEOUT`
  (estado incerto: consulte `GET /actions/{key}`).

### 4.5 `POST /broadcasts`

```json
{ "message": "Servidor reinicia em 5 minutos", "scope": { "group_id": 1, "channel": 2 } }
```

- `message`: 1–200 caracteres, texto puro (sem HTML).
- `scope`: `{group_id, channel}` ou `{ "all": true }`.
- Efeito: envia ao `wyd-server` e audita; **não** grava tabela de jogo.
- `200`: `{ "delivered_to": 1, "audit_id": 9004 }`.
- Erros: `503`, `504`, `422` (escopo inválido).

### 4.6 `PUT /cities/{code}/tax`

```json
{ "tax_rate_bps": 500, "expected_tax_rate_bps": 1000 }
```

- `code`: PK de `game.city` (`TEXT`, `^[a-z][a-z0-9_]{2,15}$`).
- `tax_rate_bps`: inteiro 0–10.000; `expected_*` obrigatório.
- Efeito: `game.city` + auditoria. `200` devolve o valor gravado. Erros: `404`, `409
  ADM_STATE_MISMATCH`, `422`.

### 4.7 `PUT /items/{item_index}/prices/{npc_id}`

```json
{ "buy_price": 1200, "sell_price": 800, "expected_buy_price": 1000, "expected_sell_price": 700 }
```

- Chave composta de `game.item_price`: `(item_index, npc_id)`; ambos os preços ≥ 0 e os `expected_*`
  obrigatórios.
- Efeito: `game.item_price` + auditoria. Erros: `404` (par ausente), `409 ADM_STATE_MISMATCH`, `422`.

### 4.8 Limite: preço em `portal`

`portal.kingdom_cape_price`, `donate_shop_item` e `daily_reward_item` são tabelas de `portal`: o BFF
as escreve direto (C2) e **não** passam pelo admin. Se um dia o jogo precisar escrevê-las, é contrato
novo — hoje a fronteira de §7 da [fronteira](fronteira.md) vale.

## 5. Rotas de saúde e reconciliação

| Método e rota | Resposta |
|---|---|
| `GET /health` | `200 {"status":"ok"}` — processo vivo; não toca banco |
| `GET /ready` | `200` quando banco e armazenamento de idempotência respondem; `503` com `ADM_DB_UNAVAILABLE` |
| `GET /actions/{key}` | `200 {"state":"completed","status":200,"body":{…}}` ou `{"state":"in_progress"}`; `404` se expirou |

## 6. Prazos e repetição

| Trecho | Prazo | Repetição |
|---|---|---|
| portal → admin | 5 s total | só com a mesma `Idempotency-Key` |
| admin → banco | 2 s por transação | nenhuma automática; erro 500/503 |
| admin → `wyd-server` | 1,5 s | nenhuma; 504 e reconciliação pela chave |

`GET` pode ser repetido livremente. `POST`/`PUT` só se repete com a mesma chave e o mesmo corpo.

## 7. O que este contrato não decide

| Assunto | Onde fecha |
|---|---|
| Papel de banco e `GRANT` de escrita em `game` | `LAC-ADM-01` |
| Onde o registro de idempotência mora | `LAC-ADM-02` |
| Canal real com o `wyd-server` (formato, porta, ack) | `LAC-ADM-03` |
| Token de serviço, mTLS e assinatura do ator | `LAC-ADM-04` |
| Dono da escrita de `account_moderation` | **fechado em 2026-11-21** (`LAC-ADM-05`): é este contrato, §4.1; o `wyd-portal` corrigiu `consumo-do-banco.md` §3 e só lê |

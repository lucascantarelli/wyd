> Pergunta: "qual é a forma exata do contrato C4 — rotas, payload, idempotência, códigos de erro e prazos?"
> Público: Desenvolvedor do `wyd-admin` e do BFF do portal
> Dono: `wyd-admin` (forma), arquitetura (direção)
> Base: `wyd-portal/docs/arquitetura/fronteira.md` §3 (C4); `LAC-POR-01` (2026-11-20); `ddl-confirmado.md` §5.2/§5.5/§5.14; `wyd-db/armaria-e-mercado.md` §5 (custódia de item)
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); custódia de item acrescentada em 2026-11-22 (§4.9–§4.12). A
direção do C4 está decidida; esta forma aguarda aprovação.

## 1. Base e cabeçalhos

- Base: `https://<admin-interno>/internal/v1` — nunca exposta à internet.
- Corpo e resposta: `application/json; charset=utf-8`.
- `Authorization: Bearer <token-de-serviço>` — obrigatório em toda rota.
- `X-Staff-Person-Id: <int>` — pessoa autenticada pelo portal; vira autor em `ops.audit_log`.
- `X-Trace-Id: <uuid>` — obrigatório; repetido na resposta e no log. Sem ele, `400`.
- `Idempotency-Key: <16–64 chars [A-Za-z0-9_-]>` — obrigatório em `POST`/`PUT`.
- `X-Actor-Account-Id: <int>` — obrigatório **somente** nas rotas de custódia (§4.9–§4.12): é a conta
do jogador em nome de quem o BFF chama. `X-Staff-Person-Id` nessas rotas devolve `403 ADM_FORBIDDEN`;
staff não opera custódia de jogador.

O admin **não** autentica pessoa: confia no token de serviço e no `X-Staff-Person-Id` afirmado pelo
portal, que é a autoridade de quem é o staff. O mecanismo do token foi **decidido em 2026-11-22**
(`LAC-ADM-04`, opção A): **token de serviço por ambiente**, em `Authorization: Bearer` sobre TLS, com
dois valores aceitos durante a rotação e comparação em tempo constante
([autenticação e controle](autenticacao-e-controle.md) §2). Nas rotas de custódia a confiança é a mesma —
token de serviço do portal — mas o ator declarado é a **conta do jogador**, e o admin confere, na mesma
transação, que o personagem citado pertence a essa conta (`game.character.account_id`).

## 2. Idempotência

| Regra | Efeito |
|---|---|
| Escopo | `(chamador, método, rota-modelo, chave)` — a mesma chave em rotas diferentes não colide |
| Chave ausente em mutação | `428 ADM_IDEMPOTENCY_REQUIRED` |
| Chave malformada | `400 ADM_INVALID_HEADER` |
| Mesma chave, mesmo corpo, concluída | devolve **status e corpo gravados**, com `Idempotency-Replayed: true` |
| Mesma chave, corpo diferente | `409 ADM_IDEMPOTENCY_KEY_REUSED` |
| Mesma chave, ainda em execução | `409 ADM_IDEMPOTENCY_IN_PROGRESS`, `Retry-After: 1` |
| Retenção do registro | **7 dias** após a criação (`M30`, 2026-11-22; `LAC-ADM-06` fechada) |

O corpo é comparado por `sha256` do JSON canônico (chaves ordenadas, sem espaço). O registro é **durável e
transacional com a ação**: mora em `ops.idempotency_record` (`0033`, `ddl-confirmado.md` §5.33; `LAC-ADM-02`
fechada em 2026-11-22). A linha nasce `in_progress` **dentro da transação da ação** e é fechada nela como
`completed` ou `failed`; o `COMMIT` nunca publica `in_progress`. Duas consequências que a implementação
assume:

1. **`409 ADM_IDEMPOTENCY_IN_PROGRESS` vem do bloqueio da chave**, não de um estado lido: a segunda
   `INSERT` espera o `COMMIT` alheio e estoura o prazo de 2 s da transação. `in_progress` não é
   observável por outra sessão, e não existe chave presa nem lease a expirar.
2. **`failed` permite repetir** com a mesma chave, reusando a linha; `completed` devolve status e corpo
   gravados.

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
| 8 | `POST /characters/{character_id}/escrow` | titular da conta |
| 9 | `POST /escrows/{escrow_id}/deliver` | titular da conta compradora |
| 10 | `POST /escrows/{escrow_id}/return` | titular da conta vendedora |
| 11 | `POST /escrows/{escrow_id}/cancel` | titular da conta vendedora |

As rotas 1–7 são de staff e usam `X-Staff-Person-Id`. As rotas 8–11 são do **mercado**: o ator é a
conta do jogador (`X-Actor-Account-Id`), a única autoridade sobre o item é o banco e nenhuma delas
existe para staff. Elas não chamam o `wyd-server`.

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
- Efeito: pede o encerramento ao `wyd-server` por `POST /internal/v1/control` no processo do canal
  ([autenticação e controle](autenticacao-e-controle.md) §3); **só depois do ack** grava `released_at` e
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
- Efeito: envia ao `wyd-server` pelo mesmo controle do §4.4 e audita; **não** grava tabela de jogo.
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

### 4.9 `POST /characters/{character_id}/escrow` — custodiar

```json
{ "slot": 12, "expected_item_index": 1234, "listing_ref": 5001 }
```

- `slot`: 0–79 (0–15 equipado, 16–79 mochila), o mesmo intervalo de `ck_character_item_slot`.
- `expected_item_index`: obrigatório; se o slot já não tiver esse item, `409` e nada é escrito.
- `listing_ref`: id do anúncio em `portal.market_listing`; sem FK entre schemas, gravado para rastro.
- Efeito: remove a linha de `game.character_item` e cria `game.market_escrow` em `retido`, na **mesma
transação**; audita `admin.escrow.hold`.
- `200`: `{ "escrow_id": 91, "state": "retido", "character_id": 7, "item_index": 1234, "qty": 1,
  "refine": 9, "ancient": false, "gem_effect": 0, "gem_value": 0, "audit_id": 9101 }`.
- Erros: `403 ADM_FORBIDDEN` (personagem de outra conta), `404` (personagem ou slot vazio), `409
  ADM_STATE_MISMATCH` (`expected_item_index` divergiu), `409 ADM_ITEM_ALREADY_HELD` (o mesmo
  `listing_ref` já tem custódia `retido`), `422 ADM_RULE_VIOLATION` (slot fora de 0–79).

### 4.10 `POST /escrows/{escrow_id}/deliver` — entregar

```json
{ "character_id": 77, "expected_state": "retido" }
```

- `character_id`: personagem do comprador; precisa pertencer a `X-Actor-Account-Id`, senão `403`.
- Efeito: insere a instância em `game.character_item` no **primeiro slot livre de 16–79** e passa a
custódia para `entregue`; audita `admin.escrow.deliver`.
- `200`: `{ "escrow_id": 91, "state": "entregue", "slot": 16, "audit_id": 9102 }`.
- Erros: `403 ADM_FORBIDDEN` (personagem de outra conta), `404` (custódia ou personagem), `409
  ADM_ESCROW_STATE` (não está `retido`), `409 ADM_INVENTORY_FULL` (sem slot livre: nada é escrito e a
  custódia continua `retido`).

### 4.11 `POST /escrows/{escrow_id}/return` — devolver

```json
{ "expected_state": "retido", "reason": "expirado" }
```

- `reason`: `expirado` (leilão sem lance) ou `disputa_vendedor` (desfecho a favor do vendedor).
- Efeito: item volta ao personagem de origem e a custódia vira `devolvido`; audita
`admin.escrow.return` com o motivo.
- `200`: `{ "escrow_id": 91, "state": "devolvido", "slot": 16, "audit_id": 9103 }`.
- Erros: `404`, `409 ADM_ESCROW_STATE`, `409 ADM_INVENTORY_FULL` (o item **permanece** `retido`; nunca é
  descartado), `422` (motivo fora da lista).

### 4.12 `POST /escrows/{escrow_id}/cancel` — cancelar

```json
{ "expected_state": "retido" }
```

- Efeito idêntico ao de §4.11 em estado (`retido → devolvido`), com ação de auditoria própria:
`admin.escrow.cancel`.
- **Por que rota separada:** a missão e `armaria-e-mercado.md` §5 nomeiam quatro operações, e a
trilha precisa distinguir **desistência do anúncio** de **fim de leilão ou disputa**. A rota não
consulta lances: `market_bid` vive em `portal`, e é o BFF que só chama `cancel` quando não há lance
ativo. Alternativa recusada: uma rota única com `reason = cancelado`, que apagaria a distinção na
auditoria do admin.
- `200`: `{ "escrow_id": 91, "state": "devolvido", "slot": 16, "audit_id": 9104 }`.
- Erros: `404`, `409 ADM_ESCROW_STATE`, `409 ADM_INVENTORY_FULL`.

### 4.13 Invariantes das rotas de custódia

1. O item existe em **um** lugar: `character_item`, `market_escrow` em `retido`, ou já entregue. As duas
   escritas de cada operação acontecem na mesma transação; não há passo intermediário visível.
2. Custódia não é apagada: `retido → entregue` ou `retido → devolvido`; estado terminal não volta.
3. A pilha é custodiada inteira (`qty` copiado). Dividir pilha fica fora da `1.0.0`.
4. Falha por mochila cheia não perde item: a custódia permanece `retido` e a operação pode ser repetida
   com a mesma chave depois de liberar espaço.

## 5. Rotas de saúde e reconciliação

| Método e rota | Resposta |
|---|---|
| `GET /health` | `200 {"status":"ok"}` — processo vivo; não toca banco |
| `GET /ready` | `200` quando banco e armazenamento de idempotência respondem; `503` com `ADM_DB_UNAVAILABLE` |
| `GET /actions/{key}` | `200 {"state":"completed","status":200,"body":{…}}` ou `{"state":"failed","error":{…}}`; `404` quando não há registro — nunca executada, ou já expurgada pelos 7 dias. `in_progress` não é devolvido: ele nunca é commitado |

## 6. Prazos e repetição

| Trecho | Prazo | Repetição |
|---|---|---|
| portal → admin | 5 s total | só com a mesma `Idempotency-Key` |
| admin → banco | 2 s por transação | nenhuma automática; erro 500/503 |
| admin → `wyd-server` | 1,5 s | nenhuma; 504 e reconciliação pela chave |

`GET` pode ser repetido livremente. `POST`/`PUT` só se repete com a mesma chave e o mesmo corpo.

As rotas de custódia não têm perna com o `wyd-server`: o prazo útil delas é o de portal → admin (5 s) e
o da transação (2 s). Timeout no meio é estado **incerto**: reconcilie por `GET /actions/{key}`, nunca
repetindo com chave nova.

## 7. O que este contrato não decide

| Assunto | Onde fecha |
|---|---|
| Papel de banco e `GRANT` de escrita em `game` | `LAC-ADM-01` |
| Onde o registro de idempotência mora | **fechado em 2026-11-22** (`LAC-ADM-02` e `LAC-ADM-06`): `ops.idempotency_record` (`0033`, `ddl-confirmado.md` §5.33), com retenção de 7 dias |
| Canal real com o `wyd-server` (formato, porta, ack) | **decidido em 2026-11-22** (`LAC-ADM-03`, opção A): HTTP interno síncrono com ack — `autenticacao-e-controle.md` §3; o formato do endpoint é do plano do `wyd-server` |
| Token de serviço, mTLS e assinatura do ator | **decidido em 2026-11-22** (`LAC-ADM-04`, opção A): token de serviço por ambiente com rotação de dois valores — `autenticacao-e-controle.md` §2 |
| Dono da escrita de `account_moderation` | **fechado em 2026-11-21** (`LAC-ADM-05`): é este contrato, §4.1; o `wyd-portal` corrigiu `consumo-do-banco.md` §3 e só lê |
| Schema de `game.market_escrow` e das tabelas `portal.market_*` | **fechado em 2026-11-22** (`LAC-ADM-07`): lote `0034_portal_market.sql`, onze tabelas (`ddl-confirmado.md` §5.34), com os `GRANT` da custódia no `0032` emendado |
| Verificação de que o personagem pertence à conta declarada | **fechado nesta proposta:** `game.character.account_id` lido na mesma transação; divergência é `403` |

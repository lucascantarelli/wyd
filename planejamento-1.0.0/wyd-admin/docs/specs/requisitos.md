> Pergunta: "quais requisitos o `wyd-admin` precisa cumprir, com que critério verificável?"
> Público: Produto, arquitetura e desenvolvedor
> Dono: equipe do wyd-admin
> Base: [contrato C4](../arquitetura/contrato-c4.md); [ações](acoes.md); `LAC-POR-01`
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21). Requisito sem critério de aceitação não entra.

## Funcionais

### RF-ADM-001 — Bloquear e desbloquear conta

- **Entrada:** `account_id`, `blocked`, `reason`, `expected_blocked`, staff `moderator`+.
- **Saída:** estado de `game.account_moderation` e `audit_id`.
- **Aceitação:** bloquear grava `blocked = true`, `blocked_at`, `blocked_by`; desbloquear grava
  `blocked = false` e `blocked_at = NULL`; `expected_blocked` divergente devolve `409` e **nenhuma**
  linha muda.
- **Contrato:** C4 §4.1. **Entrega:** `ADM-04`.

### RF-ADM-002 — Ajustar saldo com lançamento

- **Entrada:** `account_id`, `delta` ≠ 0, `reason`, `expected_coin`, staff `admin`.
- **Saída:** `coin` resultante, `ledger_id`, `audit_id`.
- **Aceitação:** toda variação tem exatamente uma linha em `account_ledger` com `balance_after` igual
  ao saldo gravado; saldo fora de `0..2147483647` devolve `422` e nada é gravado.
- **Contrato:** C4 §4.2. **Entrega:** `ADM-05`.

### RF-ADM-003 — Configurar canal jogável

- **Entrada:** `group_id`, `channel` 1–10, `max_players`, `maintenance`, `role`, `enabled`.
- **Saída:** estado gravado em `game.server_channel`.
- **Aceitação:** canal 0 devolve `422`; canal inexistente devolve `404`; capacidade negativa devolve
  `422`; nenhum `INSERT` de canal é feito pela API.
- **Contrato:** C4 §4.3. **Entrega:** `ADM-06`.

### RF-ADM-004 — Expulsar sessão

- **Entrada:** `session_id`, `note` opcional, staff `moderator`+.
- **Saída:** `released_at` e `audit_id`.
- **Aceitação:** sessão viva é liberada com `released_reason = 'admin'` **só depois** do ack do
  `wyd-server`; sessão já liberada devolve `409`; servidor fora devolve `503` sem escrita.
- **Contrato:** C4 §4.4; elo de `LAC-ADM-03`, **decidido em 2026-11-22** (HTTP interno síncrono com
  ack: `../arquitetura/autenticacao-e-controle.md` §3). **Entrega:** `ADM-08`.

### RF-ADM-005 — Enviar broadcast

- **Entrada:** `message` 1–200, escopo por canal ou `all`, staff `moderator`+.
- **Saída:** contagem de destinos e `audit_id`.
- **Aceitação:** mensagem vazia ou acima de 200 devolve `400`; servidor fora devolve `503`; nenhuma
  tabela de jogo é escrita.
- **Contrato:** C4 §4.5; `LAC-ADM-03` (decidida em 2026-11-22). **Entrega:** `ADM-08`.

### RF-ADM-006 — Alterar imposto de cidade

- **Entrada:** `code`, `tax_rate_bps` 0–10.000, `expected_tax_rate_bps`, staff `admin`.
- **Saída:** valor gravado em `game.city`.
- **Aceitação:** fora da faixa devolve `422`; `code` inexistente `404`; esperado divergente `409`;
  valor gravado é o devolvido.
- **Contrato:** C4 §4.6. **Entrega:** `ADM-07`.

### RF-ADM-007 — Alterar preço de item

- **Entrada:** `item_index`, `npc_id`, `buy_price`, `sell_price`, `expected_*`, staff `admin`.
- **Saída:** preços gravados em `game.item_price`.
- **Aceitação:** qualquer `expected_*` divergente devolve `409` sem escrita; valor negativo devolve
  `422`; par `(item_index, npc_id)` ausente devolve `404`.
- **Contrato:** C4 §4.7. **Entrega:** `ADM-07`.

### RF-ADM-008 — Idempotência durável

- **Entrada:** `Idempotency-Key` + mutação.
- **Saída:** replay do status/corpo gravado, ou erro de conflito.
- **Aceitação:** repetir a mesma requisição com a mesma chave **não** cria segunda linha em
  `account_ledger` nem segundo evento em `audit_log`; corpo diferente devolve
  `409 ADM_IDEMPOTENCY_KEY_REUSED`; chave em execução devolve `409 ADM_IDEMPOTENCY_IN_PROGRESS`.
- **Contrato:** C4 §2; tabela em `ops.idempotency_record` (`ddl-confirmado.md` §5.33; `LAC-ADM-02` fechada em 2026-11-22). **Entrega:** `ADM-03`.

### RF-ADM-009 — Auditar toda ação

- **Entrada:** qualquer mutação.
- **Saída:** uma linha em `ops.audit_log`.
- **Aceitação:** `actor_kind = 'person'`, `actor_id` = staff, `action` da lista fechada de
  [ações](acoes.md), `target` com o identificador; rollback da ação remove a linha de auditoria.
- **Contrato:** C4 §1/§3. **Entrega:** `ADM-09`.

### RF-ADM-010 — Saúde e prontidão

- **Entrada:** `GET /health`, `GET /ready`.
- **Saída:** `200` ou `503` com código tipado.
- **Aceitação:** `/health` responde sem tocar banco; `/ready` responde `503 ADM_DB_UNAVAILABLE` com o
  banco parado e `200` com banco e idempotência disponíveis.
- **Contrato:** C4 §5. **Entrega:** `ADM-01`.

### RF-ADM-011 — Consultar ação por chave

- **Entrada:** `GET /actions/{key}`.
- **Saída:** estado fechado (`completed`/`failed`) e corpo gravado; `404` quando a chave não existe —
  nunca executada ou já expurgada. `in_progress` não é devolvido, porque não é commitado.
- **Aceitação:** após timeout de kick, a consulta devolve o desfecho real; chave expirada (7 dias,
  `LAC-ADM-06`) devolve `404`; a consulta não escreve nada.
- **Contrato:** C4 §5. **Entrega:** `ADM-03`.

### RF-ADM-012 — Custodiar item do personagem

- **Entrada:** `character_id`, `slot` 0–79, `expected_item_index`, `listing_ref` e a conta do ator
  (`X-Actor-Account-Id`).
- **Saída:** `escrow_id` em `retido` com o retrato do item e `audit_id`.
- **Aceitação:** a linha sai de `game.character_item` e nasce em `game.market_escrow` na **mesma**
  transação; slot vazio devolve `404`; `expected_item_index` divergente devolve `409` sem escrita;
  repetir o mesmo `listing_ref` devolve `409 ADM_ITEM_ALREADY_HELD`; personagem de outra conta devolve
  `403`; repetir a chamada com a mesma chave não cria segunda custódia.
- **Contrato:** C4 §4.9. **Entrega:** `ADM-10`. **Depende:** `game.market_escrow` (`0034`,
  `ddl-confirmado.md` §5.34; `LAC-ADM-07` fechada em 2026-11-22).

### RF-ADM-013 — Entregar item ao comprador

- **Entrada:** `escrow_id`, `character_id` do comprador, `expected_state`.
- **Saída:** custódia em `entregue`, slot de destino e `audit_id`.
- **Aceitação:** o item entra no primeiro slot livre de 16–79 e existe em **um** lugar só; sem slot livre
  devolve `409 ADM_INVENTORY_FULL` sem escrita e a custódia continua `retido`; `state` diferente de
  `retido` devolve `409 ADM_ESCROW_STATE`; personagem de outra conta devolve `403`; a mesma chave não
  entrega duas instâncias.
- **Contrato:** C4 §4.10. **Entrega:** `ADM-10`.

### RF-ADM-014 — Devolver item ao vendedor

- **Entrada:** `escrow_id`, `expected_state = retido`, `reason` (`expirado`/`disputa_vendedor`).
- **Saída:** custódia em `devolvido`, slot de destino e `audit_id`.
- **Aceitação:** o item volta ao personagem de origem; mochila cheia devolve `409` e **não** perde o
  item; motivo fora da lista devolve `422`; repetição idempotente não duplica a instância.
- **Contrato:** C4 §4.11. **Entrega:** `ADM-10`.

### RF-ADM-015 — Cancelar custódia de anúncio

- **Entrada:** `escrow_id`, `expected_state = retido`.
- **Saída:** custódia em `devolvido`, slot de destino e `audit_id`, com ação `admin.escrow.cancel`.
- **Aceitação:** efeito de estado idêntico ao de `RF-ADM-014`, com ação de auditoria **distinta**;
  o admin não consulta `portal.market_bid`; a pré-condição “sem lance ativo” é do portal e a chamada com
  lance ativo **não** é detectada aqui (fica registrada como decisão de quem chama).
- **Contrato:** C4 §4.12. **Entrega:** `ADM-10`.

## Regras de negócio

| ID | Regra | Verificação |
|---|---|---|
| `RN-ADM-001` | Papel mínimo por rota: `moderator` para moderação/kick/broadcast; `admin` para carteira, canal, imposto e preço; nas rotas de custódia o papel é o **titular da conta** (`X-Actor-Account-Id`), nunca staff | teste de contrato 403 por papel e por conta |
| `RN-ADM-002` | Toda escrita tem autor e `trace_id` — `X-Staff-Person-Id` para staff, `X-Actor-Account-Id` para custódia; sem eles, `400`/`401` | teste sem cabeçalho |
| `RN-ADM-008` | Item em custódia não existe em inventário: `character_item` e `market_escrow` nunca contêm a mesma instância | teste conta as duas tabelas antes e depois de cada operação |
| `RN-ADM-003` | Uma ação = uma transação; escrita e auditoria commitam juntas | teste que força rollback e confere ausência das duas linhas |
| `RN-ADM-004` | Nenhuma ação apaga linha; estado é `UPDATE`/`upsert`. Exceção única: `custodiar` move a instância — sai de `character_item` e entra em `market_escrow` na mesma transação | revisão de SQL + teste de contagem nas duas tabelas |
| `RN-ADM-005` | Payload, log e resposta não contêm hash, token, segredo TOTP nem PII além de id | teste de log e de resposta |
| `RN-ADM-006` | Erro do admin não é compensado por escrita local no portal | teste de contrato com 503 |
| `RN-ADM-007` | O admin não escreve em `portal`; preço de capa, donate e recompensa diária são do BFF | revisão de schema + teste de credencial |

## Não funcionais

| ID | Requisito | Critério |
|---|---|---|
| `RNF-ADM-001` | Prazo: 5 s portal→admin, 2 s por transação, 1,5 s admin→server | teste de timeout com dublê lento |
| `RNF-ADM-002` | Autenticação de serviço obrigatória; sem token, `401` | teste de contrato; mecanismo decidido em 2026-11-22 (`LAC-ADM-04`): token por ambiente, dois aceitos na rotação |
| `RNF-ADM-003` | Cobertura mínima de 85% no gate | `cargo llvm-cov` no alvo |
| `RNF-ADM-004` | Log estruturado com `trace_id`, ação e resultado; sem corpo sensível | inspeção de log no teste |
| `RNF-ADM-005` | Concorrência otimista obrigatória em carteira e preço | dois clientes simultâneos: um `200`, um `409` |

## O que ainda não é requisito

`kick` e `broadcast` deixaram de ser requisito bloqueado: o elo com o `wyd-server` foi decidido em
2026-11-22 (`LAC-ADM-03`, HTTP interno síncrono com ack), e o que falta é o endpoint no plano do
`wyd-server` — obrigação dele, não lacuna deste sistema.

A custódia (`RF-ADM-012` a `RF-ADM-015`) tem forma, contrato e schema fechados: `game.market_escrow` entrou
no lote `0034` em 2026-11-22 (`LAC-ADM-07`), e os `GRANT` que ela exige estão no `0032` emendado. Nada
mais a bloqueia além da ordem de fatias do roadmap.

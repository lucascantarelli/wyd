> Pergunta: "o que exatamente cada ação do admin grava em `game` e sob quais constraints?"
> Público: Desenvolvedor do `wyd-admin` e revisor de schema
> Dono: `wyd-admin` + `wyd-db`
> Base: `ddl-confirmado.md` §5.2, §5.5, §5.14; `plano-migrations.md` §3; `padrao-banco.md` §11; `wyd-db/armaria-e-mercado.md` §3.11 e §5 (custódia de item)
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); custódia de item acrescentada em 2026-11-22 (§8). Nenhum SQL foi
escrito; a forma é a de `ddl-confirmado.md`.

## Regra comum

Toda ação roda em **uma transação** com isolamento `READ COMMITTED`:

1. valida cabeçalhos, papel (ou titular, nas rotas de §8) e corpo — nada toca o banco antes disso;
2. abre transação, registra a chave de idempotência como `in_progress`;
3. executa a escrita com conferência otimista (`expected_*`) quando a rota exige;
4. grava `ops.audit_log` com o mesmo `trace_id`;
5. marca a chave como `completed` com o corpo de resposta;
6. `COMMIT`. Falha em qualquer passo = `ROLLBACK` e chave `failed` (permite repetir).

Sem o passo 2 não há idempotência; ele depende de `LAC-ADM-02`.

## 1. Moderação de conta

- **Tabela:** `game.account_moderation` (PK `account_id`, FK `RESTRICT` para `account` e para
  `person` em `blocked_by`).
- **Sequência:** `SELECT … FOR UPDATE` da linha; bloqueia se `expected_blocked` divergir; `INSERT … ON
  CONFLICT (account_id) DO UPDATE` com `blocked`, `reason`, `blocked_by = staff`, `blocked_at = now()`
  quando bloqueia, `blocked_at = NULL` quando desbloqueia.
- **Constraints que podem recusar:** `ck_account_moderation_motivo`, `ck_account_moderation_instante`,
  `ck_account_moderation_autor`.
- **Desbloqueio:** `reason` e `blocked_by` **permanecem** (o DDL só exige `blocked_at` nulo); a
  resposta informa o motivo histórico. O `H` de `wyd-db/docs/dicionario-dados/dominio-moderacao.md`
  não muda por causa desta rota.
- **Auditoria:** `action = 'admin.account.moderate'`, `target = 'account:<id>'`, detalhe com
  `before/after` de `blocked`.

## 2. Ajuste de carteira

- **Tabelas:** `game.account_wallet` (PK `account_id`) e `game.account_ledger`.
- **Sequência:** `SELECT coin FROM account_wallet WHERE account_id = $1 FOR UPDATE`; 404 se não
  existir; 409 se `coin <> expected_coin`; calcula `novo = coin + delta`; 422 se `novo < 0` ou
  `> 2147483647`; `UPDATE account_wallet`; `INSERT account_ledger (delta, balance_after, reason)`.
- **Constraints:** `ck_account_wallet_coin`, `ck_account_ledger_delta` (`delta <> 0`),
  `ck_account_ledger_balance`, `ck_account_ledger_reason` (1–64).
- **Regra:** não existe `UPDATE` de carteira sem linha no ledger (`M16`).
- **Auditoria:** `action = 'admin.wallet.adjust'`, `target = 'account:<id>'`, detalhe com `delta`,
  `before`, `after`, `ledger_id`; **nunca** o motivo com PII.

## 3. Configuração de canal

- **Tabela:** `game.server_channel` (PK `(group_id, channel)`).
- **Sequência:** 422 se `channel = 0` (linha de status); `UPDATE … WHERE group_id = $1 AND channel =
  $2`; 404 se nenhuma linha; confere `max_players >= 0` e `role IN ('normal','verde')`.
- **Constraints:** `ck_server_channel_jogavel`, `ck_server_channel_max`, `ck_server_channel_role`.
- **Sem `INSERT`:** canal novo é dado de instalação, não ação de portal.
- **Auditoria:** `action = 'admin.channel.update'`, `target = 'channel:<group>:<channel>'`.

## 4. Kick de sessão

- **Tabelas:** `game.session` (PK `id`; FK `RESTRICT`).
- **Sequência:** lê a sessão; 404 se não existir; 409 se `released_at` não nulo; chama o `wyd-server`;
  **após o ack**, `UPDATE game.session SET released_at = now(), released_reason = 'admin'`.
- **Constraint:** `ck_session_motivo` exige o par `released_at`/`released_reason` coerente.
- **Falha do servidor:** nenhuma escrita; `503`/`504` (o estado fica incerto só no timeout).
- **Auditoria:** `action = 'admin.session.kick'`, `target = 'session:<id>'`.

## 5. Broadcast

- **Tabela:** nenhuma.
- **Sequência:** valida `message` (1–200) e escopo; envia ao `wyd-server`; audita o envio.
- **Auditoria:** `action = 'admin.broadcast'`, `target = 'channel:<group>:<channel>'` ou `'all'`.

## 6. Imposto de cidade

- **Tabela:** `game.city`, PK `code TEXT` e coluna `tax_rate_bps` (DDL em `0011`).
- **Sequência:** `SELECT tax_rate_bps … FOR UPDATE`; 404 se o `code` não existir; 409 se divergir do
  `expected_tax_rate_bps`; `UPDATE game.city SET tax_rate_bps = $2 WHERE code = $1`; 422 fora de
  0–10.000 (`ck_city_imposto`).
- **Auditoria:** `action = 'admin.city.tax'`, `target = 'city:<code>'`.

## 7. Preço de item

- **Tabela:** `game.item_price`, PK composta `(item_index, npc_id)` e colunas `buy_price`/`sell_price`
  (DDL em `0011`).
- **Sequência:** `SELECT buy_price, sell_price … FOR UPDATE`; 404 se o par não existir; 409 se
  qualquer preço divergir do `expected_*`; `UPDATE`; 422 se negativo.
- **Auditoria:** `action = 'admin.item.price'`, `target = 'item:<item_index>:<npc_id>'`.

## 8. Custódia de item do mercado

Rotas 8–11 de §4.9–§4.12 do [contrato C4](../arquitetura/contrato-c4.md). O ator é a conta do jogador
(`X-Actor-Account-Id`), não staff, e nenhuma delas chama o `wyd-server`. A tabela `game.market_escrow`
ainda **não tem DDL aprovado** — forma em `wyd-db/armaria-e-mercado.md` §3.11 e lote a abrir no
`wyd-db` (`LAC-ADM-07`).

### 8.1 Custodiar (`POST /characters/{character_id}/escrow`)

- **Tabelas:** `game.character_item` (PK `(character_id, slot)`) e `game.market_escrow`.
- **Sequência:** `422` se `slot` fora de 0–79; `SELECT … FROM character_item WHERE character_id = $1
  AND slot = $2 FOR UPDATE`; `404` se não existir; `409 ADM_STATE_MISMATCH` se `item_index <>
  expected_item_index`; lê `game.character.account_id` e recusa `403` se não for a conta do ator;
  `409 ADM_ITEM_ALREADY_HELD` se já houver escrow `retido` com o mesmo `listing_ref`; `DELETE` da linha;
  `INSERT market_escrow` com o retrato e `state = 'retido'`.
- **Constraints que podem recusar:** `uq_market_escrow_idempotency`, `ck_market_escrow_estado`.
- **Pilha:** `qty` vai inteiro para a custódia; dividir pilha fica fora da `1.0.0`.
- **Auditoria:** `admin.escrow.hold`, `target = 'character:<id>:slot:<slot>'`, detalhe com
  `item_index`, `qty`, `listing_ref` e `escrow_id`.

### 8.2 Entregar (`POST /escrows/{escrow_id}/deliver`)

- **Tabelas:** `game.market_escrow`, `game.character`, `game.character_item` (destino).
- **Sequência:** `SELECT … FROM market_escrow WHERE id = $1 FOR UPDATE`; `404`; `409 ADM_ESCROW_STATE`
  se `state <> 'retido'`; confere que o personagem de destino pertence à conta do ator (`403` se não);
  calcula o menor slot livre em 16–79; sem slot livre devolve `409 ADM_INVENTORY_FULL` **sem escrita**;
  `INSERT character_item` com o retrato; `UPDATE market_escrow SET state = 'entregue'`.
- **Constraints:** `pk_character_item` impede colisão de slot; `ck_character_item_qtd`, `_refino` e
  `_gema*` aceitam o retrato porque ele veio de uma linha válida na origem.
- **Auditoria:** `admin.escrow.deliver`, `target = 'escrow:<id>'`, detalhe com `character_id` e `slot`.

### 8.3 Devolver (`POST /escrows/{escrow_id}/return`)

- **Sequência:** igual a 8.2, com o personagem de **origem**; `409 ADM_INVENTORY_FULL` mantém a custódia
  em `retido` (o item nunca é descartado); `UPDATE market_escrow SET state = 'devolvido'`.
- **Auditoria:** `admin.escrow.return`, `target = 'escrow:<id>'`, detalhe com `reason`
  (`expirado`/`disputa_vendedor`) e `slot`.

### 8.4 Cancelar (`POST /escrows/{escrow_id}/cancel`)

- **Sequência:** igual a 8.3 em efeito (`retido → devolvido`); a pré-condição “sem lance ativo” é do
  **portal**, porque `market_bid` é tabela dele; o admin não consulta `portal` (`RN-ADM-007`).
- **Auditoria:** `admin.escrow.cancel`, mesmo `target`, sem `reason` de domínio.

### Invariantes das quatro

1. O item existe em um lugar só: `character_item`, escrow `retido` ou já entregue. As duas escritas de
   cada operação acontecem na mesma transação.
2. `market_escrow` não é apagada: `retido → entregue` ou `retido → devolvido`, estado terminal não volta.
3. Mochila cheia não perde item: `409` sem escrita, repetível com a mesma chave depois de liberar espaço.
4. Nenhuma delas compensa erro com fila ou escrita posterior; `ops.delivery_queue` **não** é usada para
   custódia.

## O que nenhuma ação faz

- Não escreve em `portal`: moderação de pessoa, sessão de portal, CMS e `portal.kingdom_cape_price`
  são do BFF.
- Não apaga linha: bloqueio, kick e preço são estado, não `DELETE`. **Exceção única:** `custodiar`
  (§8.1) remove a linha de `game.character_item` porque a instância passa a `game.market_escrow` na
  mesma transação — é movimentação de item, não descarte de dado.
- Não grava `ops.audit_log` em transação separada: se a ação não commita, a trilha não existe.
- Não compensa erro com escrita local, fila ou “tentar de novo depois”.

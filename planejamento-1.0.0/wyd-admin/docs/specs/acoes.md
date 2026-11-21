> Pergunta: "o que exatamente cada ação do admin grava em `game` e sob quais constraints?"
> Público: Desenvolvedor do `wyd-admin` e revisor de schema
> Dono: `wyd-admin` + `wyd-db`
> Base: `ddl-confirmado.md` §5.2, §5.5, §5.14; `plano-migrations.md` §3; `padrao-banco.md` §11
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21). Nenhum SQL foi escrito; a forma é a de `ddl-confirmado.md`.

## Regra comum

Toda ação roda em **uma transação** com isolamento `READ COMMITTED`:

1. valida cabeçalhos, papel e corpo (nada toca o banco antes disso);
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

## O que nenhuma ação faz

- Não escreve em `portal`: moderação de pessoa, sessão de portal, CMS e `portal.kingdom_cape_price`
  são do BFF.
- Não apaga linha: bloqueio, kick e preço são estado, não `DELETE`.
- Não grava `ops.audit_log` em transação separada: se a ação não commita, a trilha não existe.
- Não compensa erro com escrita local, fila ou “tentar de novo depois”.

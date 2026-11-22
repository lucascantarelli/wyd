> Pergunta: "em que ordem o `wyd-admin` é construído e como sei que cada fatia fechou?"
> Público: Produto, arquitetura e desenvolvedor
> Dono: equipe do wyd-admin
> Base: [requisitos](../specs/requisitos.md); [contrato C4](../arquitetura/contrato-c4.md); `AGENTS.md` §7
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21). **Nenhuma issue foi criada e o board não foi tocado**; estas são
fatias de planejamento, não itens de GitHub.

| ID | Fase | Pri. | Entregável | DoD verificável | Depende de |
|---|---|---|---|---|---|
| `ADM-01` | P0 | alta | Esqueleto HTTP, config, `/health` e `/ready` | `cargo test -p wyd-admin contrato::saude` verde; `/ready` com banco parado devolve `503 ADM_DB_UNAVAILABLE` | — |
| `ADM-02` | P0 | alta | Autenticação de serviço, papel mínimo e `trace_id` | testes `401` sem token, `403` para `member`, `400` sem `X-Trace-Id`; rotação com dois tokens aceitos durante a janela | — (decisão de 2026-11-22) |
| `ADM-03` | P0 | alta | Registro de idempotência, replay e `GET /actions/{key}` | duas chamadas com a mesma chave = **1** linha em `account_ledger`; corpo diferente = `409`; consulta devolve o corpo gravado | schema pronto (`0033`); nada bloqueia |
| `ADM-04` | P1 | alta | `POST /accounts/{id}/moderation` | teste de integração bloqueia/desbloqueia e confere `blocked_at`, `blocked_by` e `audit_log` | `ADM-03`, `LAC-ADM-01` |
| `ADM-05` | P1 | alta | `POST /accounts/{id}/wallet-adjustments` | saldo + ledger coerentes; `expected_coin` divergente = `409` sem escrita; rollback sem linha | `ADM-03`, `LAC-ADM-01` |
| `ADM-06` | P1 | média | `PUT /channels/{g}/{c}` | canal 0 = `422`; atualização grava e audita; nenhum `INSERT` | `ADM-03`, `LAC-ADM-01` |
| `ADM-07` | P2 | média | `PUT /cities/{code}/tax` e `PUT /items/{item_index}/prices/{npc_id}` | faixa e concorrência otimista provadas; `404` para alvo ausente | `ADM-03`, `LAC-ADM-01` |
| `ADM-08` | P2 | média | `POST /sessions/{id}/kick` e `POST /broadcasts` | teste com dublê: sem ack não há escrita; `200` do controle grava `released_at` e audita; processo parado devolve `503` sem escrita | `ADM-03`; endpoint do `wyd-server` é obrigação do plano dele (decisão de 2026-11-22) |
| `ADM-09` | P2 | alta | Auditoria, log estruturado e gate de 85% | toda ação tem `audit_id`; log sem PII; `cargo llvm-cov … --fail-under-lines 85` exit 0 | `ADM-04`–`ADM-07` |
| `ADM-10` | P1 | alta | Custódia de item do mercado: `POST /characters/{id}/escrow`, `POST /escrows/{id}/deliver`, `/return`, `/cancel` | duas chamadas com a mesma chave = **1** linha em `market_escrow`; o item nunca está em `character_item` e `market_escrow` ao mesmo tempo; entrega com mochila cheia = `409` sem perda e custódia continua `retido`; as quatro ações gravam `audit_log` | `ADM-03` e `LAC-ADM-01`; schema pronto (`0034` + emenda do `0032`) |

## Fora de escopo da `1.0.0`

- Tela ou HTML: a administração visual é do `wyd-portal`.
- Fila durável, retry agendado ou “ação para depois”.
- Escrita em `portal` (CMS, preço de capa, donate, recompensa diária).
- Regra de jogo: dano, drop, progressão e economia são de `wyd-sim`/`wyd-core`.
- Autenticação de pessoa e segundo fator: `wyd-portal` (`LAC-POR-05`).

## Riscos

| Risco | Efeito | Mitigação |
|---|---|---|
| Papel `wyd_admin` não existe | escrita impossível ou feita com credencial errada | `LAC-ADM-01` antes de `ADM-04` |
| Idempotência sem tabela | retry do portal duplica lançamento | **encerrado em 2026-11-22:** `ops.idempotency_record` (`0033`) fecha `LAC-ADM-02` e `LAC-ADM-06` |
| Contrato com `wyd-server` inexistente | kick/broadcast não saem do papel | **encerrado por decisão em 2026-11-22** (`LAC-ADM-03`): HTTP interno síncrono com ack; o admin é escrito contra o contrato e testado com dublê, e o endpoint é obrigação nomeada do plano do `wyd-server` |
| Escrita duplicada de `account_moderation` | duas implementações para a mesma escrita | mitigado: `LAC-ADM-05` fechada em 2026-11-21 e doc do portal corrigido; resta impedir SQL direto em revisão |
| Item em dois lugares na custódia | duplicação de item real e mercado sem lastro | as duas escritas na mesma transação (`RN-ADM-008`); mochila cheia devolve `409` e mantém `retido` |
| Schema de custódia ausente | `ADM-10` não implementa | **encerrado em 2026-11-22:** `game.market_escrow` no lote `0034` (`LAC-ADM-07`); nenhuma tabela improvisada no admin |

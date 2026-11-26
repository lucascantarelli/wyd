> Pergunta: "em que ordem o `wyd-admin` é construído e como sei que cada fatia fechou?"
> Público: Produto, arquitetura e desenvolvedor
> Dono: equipe do wyd-admin
> Base: [requisitos](../specs/requisitos.md); [contrato C4](../arquitetura/contrato-c4.md); `AGENTS.md` §7; `planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md`
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-21; páginas por issue em 2026-11-26). **Nenhuma issue foi criada e o
board não foi tocado**; estas são fatias de planejamento, não itens de GitHub.

## 1. Regras do fatiamento

1. Uma issue, uma página; a página é dona de fase, prioridade, entregável, DoD, fora de escopo,
   dependência, skills e evidência.
2. Fatia só entra se o DoD tiver comando, arquivo, status HTTP ou teste observável.
3. O que depende de schema, lacuna ou decisão fica em `## Depende de` da página; o roadmap não repete.
4. O número da issue nasce na criação; a chave local `ADM-nn` não antecipa número do GitHub.

## 2. Fatias

A ordem abaixo é a do roadmap. A prova de término é o `## DoD` da página citada; o roadmap não repete
fase, prioridade, entregável, dependência, DoD nem fora de escopo.

| Ordem | Issue | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | `ADM-01` — esqueleto HTTP, config e saúde | [`ADM-01`](WYDADM-01.md) §Skills |
| 2 | `ADM-02` — autenticação de serviço e trace_id | [`ADM-02`](WYDADM-02.md) §Skills |
| 3 | `ADM-03` — idempotência, replay e consulta de ação | [`ADM-03`](WYDADM-03.md) §Skills |
| 4 | `ADM-04` — moderação de conta | [`ADM-04`](WYDADM-04.md) §Skills |
| 5 | `ADM-05` — ajuste de carteira | [`ADM-05`](WYDADM-05.md) §Skills |
| 6 | `ADM-06` — configuração de canal | [`ADM-06`](WYDADM-06.md) §Skills |
| 7 | `ADM-07` — imposto e preço | [`ADM-07`](WYDADM-07.md) §Skills |
| 8 | `ADM-08` — kick e broadcast | [`ADM-08`](WYDADM-08.md) §Skills |
| 9 | `ADM-09` — auditoria, log e cobertura | [`ADM-09`](WYDADM-09.md) §Skills |
| 10 | `ADM-10` — custódia de item do mercado | [`ADM-10`](WYDADM-10.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2. O que cada issue exige antes de começar — outra issue,
lacuna, schema ou decisão — é o campo `## Depende de` da página dela; esta página não repete aresta
nem grafo.

## 4. Fora de escopo da `1.0.0`

- Tela ou HTML: a administração visual é do `wyd-portal`.
- Fila durável, retry agendado ou “ação para depois”.
- Escrita em `portal` (CMS, preço de capa, donate, recompensa diária).
- Regra de jogo: dano, drop, progressão e economia são de `wyd-sim`/`wyd-core`.
- Autenticação de pessoa e segundo fator: `wyd-portal` (`LAC-POR-05`).

## 5. Riscos

| Risco | Efeito | Mitigação |
|---|---|---|
| Papel `wyd_admin` não existe | escrita impossível ou feita com credencial errada | `LAC-ADM-01` antes de `ADM-04` |
| Idempotência sem tabela | retry do portal duplica lançamento | **encerrado em 2026-11-22:** `ops.idempotency_record` (`0033`) fecha `LAC-ADM-02` e `LAC-ADM-06` |
| Contrato com `wyd-server` inexistente | kick/broadcast não saem do papel | **encerrado por decisão em 2026-11-22** (`LAC-ADM-03`): HTTP interno síncrono com ack; o admin é escrito contra o contrato e testado com dublê, e o endpoint é obrigação nomeada do plano do `wyd-server` |
| Escrita duplicada de `account_moderation` | duas implementações para a mesma escrita | mitigado: `LAC-ADM-05` fechada em 2026-11-21 e doc do portal corrigido; resta impedir SQL direto em revisão |
| Item em dois lugares na custódia | duplicação de item real e mercado sem lastro | as duas escritas na mesma transação (`RN-ADM-008`); mochila cheia devolve `409` e mantém `retido` |
| Schema de custódia ausente | `ADM-10` não implementa | **encerrado em 2026-11-22:** `game.market_escrow` no lote `0034` (`LAC-ADM-07`); nenhuma tabela improvisada no admin |

## 6. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
- Não repete o detalhe da fatia; a página da issue é dona.

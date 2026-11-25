> Pergunta: "em que ordem o portal é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: produto
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/rotas-e-permissoes.md`; `M21`
> Revisado em: 2026-11-22

**Estado:** aceito (2026-11-20); F7 e F9 alinhadas às decisões de 2026-11-22

Fatias de entrega; o DoD executável de cada uma está na página citada em §2. **Nenhuma issue foi criada**: o roadmap planeja
ordem e prova; abrir issue, milestone ou board é ato separado e não pedido nesta rodada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Fatia bloqueada por lacuna (`lacunas.md`) não começa antes de a lacuna fechar.
3. Toda fatia administrativa inclui auditoria e teste de permissão negada.
4. Dependência de DDL é explícita: sem a migration aplicada, a fatia não fecha.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Fundação do portal (auth e sessão) | [`WYDPOR-F0`](WYDPOR-F0.md) §Skills |
| 2 | F1 — Conta e personagens | [`WYDPOR-F1`](WYDPOR-F1.md) §Skills |
| 3 | F2 — Portal público: home, notícias e rankings | [`WYDPOR-F2`](WYDPOR-F2.md) §Skills |
| 4 | F3 — WYDpedia e atlas | [`WYDPOR-F3`](WYDPOR-F3.md) §Skills |
| 5 | F4 — Admin: NOC, canais, logs e configuração | [`WYDPOR-F4`](WYDPOR-F4.md) §Skills |
| 6 | F5 — Admin: jogadores, contas e economia | [`WYDPOR-F5`](WYDPOR-F5.md) §Skills |
| 7 | F6 — CMS e launcher | [`WYDPOR-F6`](WYDPOR-F6.md) §Skills |
| 8 | F7 — Armaria read-only | [`WYDPOR-F7`](WYDPOR-F7.md) §Skills |
| 9 | F8 — Mercado: carteira e venda direta | [`WYDPOR-F8`](WYDPOR-F8.md) §Skills |
| 10 | F9 — Mercado: leilão | [`WYDPOR-F9`](WYDPOR-F9.md) §Skills |
| 11 | F10 — Mercado: reputação e notificação | [`WYDPOR-F10`](WYDPOR-F10.md) §Skills |
| 12 | F11 — Mercado: disputa, antifraude e fechamento | [`WYDPOR-F11`](WYDPOR-F11.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | sessão de portal confundida com sessão de jogo | teste E2E explícito de que o jogo não depende do portal |
| F2 | ranking caro em tabela quente | `LAC-POR-04`: cache e instante declarado |
| F3 | conteúdo sem seed suficiente | nada de “dado autêntico” inventado; seed é do `wyd-db` |
| F4 | escrita em `game` bloqueada por permissão | ação só pelo `wyd-admin`, nunca SQL local |
| F4 | API de métricas indisponível | card mostra “não medido” e aponta o Grafana; ocupação e capacidade continuam vindo do banco |
| F4 | Loki ou Grafana fora | cada tela degrada isolada (`monitoramento.md` §8); presença e motor continuam e a tela diz qual fonte falhou |
| F4 | telemetria do launcher sem histórico no banco | agregado em processo + log no Loki; a tela declara a janela “desde o boot” e não inventa número |
| F6 | tela sem tabela | não começa antes do lote de schema no `wyd-db` |
| F7 | estatística de arma inexistente no banco | `LAC-POR-08` fechada: a ficha publica só o provado; captura de dano/alcance em `P-POR-08` |
| F8 | item duplicado entre inventário e anúncio | custódia na mesma transação (`armaria-e-mercado.md` §3.11); anúncio só fica `ativo` com `escrow_ref` |
| F8 | schema de custódia não aplicado | `LAC-ADM-07` fechada: contrato do `wyd-admin` escrito (`LAC-POR-07`) e lote `0034` aprovado; falta aplicar a série; `delivery_queue` foi recusada para custódia |
| F9 | leilão fechando duas vezes | job idempotente + `uq_market_transaction_listing`; `LAC-POR-09` = A exige dreno `SKIP LOCKED` |
| F9 | leilão sem comprador deixa item preso | expiração devolve a custódia ao vendedor no mesmo job |
| F9 | fechamento parado ou atrasado | `LAC-POR-09` = A: métrica de atraso e alerta; o lote é reentrante e tem teto de tempo |
| F10 | avaliação usada como retaliação | uma por transação, só pelo comprador, com moderação por estado `oculto` |
| F11 | taxa alimentando inflação | taxa é sumidouro: linha `taxa` no ledger não é creditada a ninguém |

## 5. O que este roadmap não faz

- Não estima prazo em dias: não há medição de velocidade do time.
- Não cria issue, milestone ou board.
- Não fixa biblioteca de UI: isso é da fatia, com registro de decisão se mudar fronteira.

*Página de planejamento. Nenhuma fatia foi iniciada.*

## Páginas de issue por fatia

Medido em 2026-11-25: **12** páginas, uma por fatia, no formato de `WYDDB-nnn`
(dependências, DoD observável e bloco de skills); o estado de cada uma é o `**Estado:**` da própria página. O número sequencial de cada issue é
atribuído na criação pelo `github-mapping.md`; a chave aqui é a fatia.

| Issue | Título |
|---|---|
| [`WYDPOR-F0`](WYDPOR-F0.md) | Fundação do portal (auth e sessão) |
| [`WYDPOR-F1`](WYDPOR-F1.md) | Conta e personagens |
| [`WYDPOR-F10`](WYDPOR-F10.md) | Mercado: reputação e notificação |
| [`WYDPOR-F11`](WYDPOR-F11.md) | Mercado: disputa, antifraude e fechamento |
| [`WYDPOR-F2`](WYDPOR-F2.md) | Portal público: home, notícias e rankings |
| [`WYDPOR-F3`](WYDPOR-F3.md) | WYDpedia e atlas |
| [`WYDPOR-F4`](WYDPOR-F4.md) | Admin: NOC, canais, logs e configuração |
| [`WYDPOR-F5`](WYDPOR-F5.md) | Admin: jogadores, contas e economia |
| [`WYDPOR-F6`](WYDPOR-F6.md) | CMS e launcher |
| [`WYDPOR-F7`](WYDPOR-F7.md) | Armaria read-only |
| [`WYDPOR-F8`](WYDPOR-F8.md) | Mercado: carteira e venda direta |
| [`WYDPOR-F9`](WYDPOR-F9.md) | Mercado: leilão |

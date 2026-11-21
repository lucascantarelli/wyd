> Pergunta: "de onde vêm CPU, RAM, latência, logs, painéis e telemetria do launcher que o staff vê no portal, e o que acontece quando a fonte falha?"
> Público: Desenvolvedor novo, operação e arquitetura
> Dono: arquitetura + `wyd-ops`
> Base: `LAC-POR-06` (2026-11-20); `deprecated/wyd-portal/src/server/monitoring.ts` (referência); `deprecated/docs/rust/06-observabilidade-runbooks.md` (referência); `docs/arquitetura/consumo-do-banco.md`
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

O portal legado já operava um conjunto de telas de monitoramento e o planejamento da `1.0.0` **não pode
entregar menos**: cada tela, número e ação do legado tem rota, fonte e degradação declaradas aqui. O que
muda é a fronteira — o BFF do portal consulta as fontes por HTTP, e **nenhuma série de métrica, log ou
evento de launcher vira tabela** (`LAC-POR-06`, reconfirmada em 2026-11-20).

## 1. As seis telas

Rotas 38 a 43 de `rotas-e-permissoes.md`; todas exigem `staff` e são somente leitura.

| # | Rota | O que mostra | Fonte |
|---:|---|---|---|
| 38 | `/admin/monitoramento` | Resumo: estado das quatro fontes, jogadores online, canais no ar, tick P99 e orçamento, entidades, latência do banco, tabela de motor por canal, alertas em disparo, volume de log em 15 min e atalhos para as subpáginas | Prometheus, Loki, Grafana e banco (presença/saúde) |
| 39 | `/admin/monitoramento/servidor` | Por canal: `up`, handshakes, sessões abertas, ticks/s, P99 do tick, entidades, jogadores, mobs estimados e RSS; agregado de host (CPU, RAM, load 1, rede RX/TX); presença por canal lida do banco; alertas do motor | Prometheus (`wyd_*`), `game.v_live_session` |
| 40 | `/admin/monitoramento/prometheus` | Saúde do coletor, alvos `up`/`down` (canais e demais jobs) e alertas avaliados (disparando e pendentes, com severidade, resumo e instante) | Prometheus `/api/v1/query`, `/api/v1/alerts` |
| 41 | `/admin/monitoramento/grafana` | Saúde do Grafana, catálogo de dashboards provisionados e o painel escolhido embutido em `iframe` (kiosk), com link para abrir fora do portal | Grafana `/api/health`, `/api/search` e `/d/<uid>` |
| 42 | `/admin/monitoramento/logs` | Linhas, erros e avisos por serviço nos últimos 15 min; filtros de serviço e nível; download NDJSON da última hora (até 1.000 linhas) | Loki `/loki/api/v1/query` e `query_range` |
| 43 | `/admin/monitoramento/launcher` | Eventos anônimos do launcher agregados por versão e operação: eventos, falhas, taxa de falha, duração média e máxima, último evento, chaves e descartes | `POST /api/launcher/telemetria` (público) e log estruturado no Loki |

O resumo (38) é índice e painel ao mesmo tempo: cada cartão agregado leva à subpágina com o filtro já
aplicado, como manda `RF-POR-010`.

## 2. De onde vem cada número

| Número | Consulta/fonte | Quando falta |
|---|---|---|
| `up` por canal | `up{job="wyd-server"}` com o rótulo `canal` | canal sai da tabela e o contador “canais no ar” mostra `—` |
| Ticks/s, P99, entidades | `rate(wyd_ticks_total[1m])`, `histogram_quantile(0.99, …)`, `sum by (canal) (wyd_entities_active)` | campo `—`, nunca `0` |
| Orçamento do tick | `wyd_tick_budget_microseconds` (`1e6 / WYD_TICK_HZ`) | cai no default de 20 Hz, declarado na tela |
| Handshakes, sessões, RSS | `wyd_handshakes_total`, `wyd_sessions_open`, `process_resident_memory_bytes` | campo `—` |
| Jogadores por canal | `game.v_live_session` (view de `0029`) | presença inteira marcada como indisponível |
| Mobs estimados | entidades menos jogadores | `—` enquanto faltar uma das duas fontes |
| CPU, RAM, load, rede do host | `node_exporter` via Prometheus | cartão de host “não medido” |
| Alertas | `/api/v1/alerts` (disparando e pendentes) | lista vazia com aviso de coletor fora |
| Logs por serviço | LogQL `count_over_time` por `container` em 15 min | painel de logs marcado “fora” |
| Painéis | `/api/health` e `/api/search?type=dash-db` | catálogo vazio com o erro do Grafana |
| Telemetria do launcher | agregado em processo por `versão + operação` | “nenhum evento ainda”, com a janela desde o boot |

`V` (medido em 2026-11-20): as consultas e os nomes de métrica acima são os do portal legado, em
`deprecated/wyd-portal/src/server/monitoring.ts`, `loki.ts` e `grafana.ts`, e batem com o contrato de
`deprecated/docs/arquitetura/adr/0018-telemetria-tracing.md`. `P`: na `1.0.0` eles são reimplementados
no BFF novo, sem copiar código.

## 3. Telemetria do launcher (pública, anônima)

O launcher não tem sessão; o evento chega sem autenticação e o contrato é **fechado**: versão, build do
cliente, operação (`check`, `config`, `patch`, `launch`), sucesso, duração em ms, número de arquivos e
até dez falhas com nome de arquivo e motivo enumerado. Corpo limitado, sem texto livre, sem login, IP ou
caminho — o `H` do legado que serve de referência é `deprecated/wyd-portal/src/server/launcher-telemetria.ts`.

- A resposta é `204` sem corpo: o launcher **nunca** depende dela para abrir o jogo.
- A agregação vive no processo do BFF (mapa limitado, descartes contados); cada evento aceito também
  vira uma linha estruturada, e **essa linha no Loki é a série histórica**.
- Nenhuma tabela, nenhum vínculo com jogador. A tela 43 é `staff`; a ingestão é pública e limitada.

## 4. Variáveis de ambiente

Nomes herdados do legado (`V`, `deprecated/wyd-portal/README.md`), para a operação não reaprender:

| Variável | Uso | Segredo |
|---|---|---|
| `PROMETHEUS_URL` | consulta de métricas e alertas, server-side | não |
| `PROMETHEUS_PUBLIC_URL` | link “abrir no Prometheus” | não |
| `LOKI_URL` | consulta e exportação de logs, server-side | não |
| `GRAFANA_URL` | saúde e catálogo de dashboards, server-side | não |
| `GRAFANA_PUBLIC_URL` | `iframe` e link “abrir no Grafana” | não |

Credencial de banco não entra aqui: saúde e presença usam o adapter do `wyd-db` e a view de `0029`.
Se a API de métricas exigir autenticação, o valor é segredo do `wyd-ops` no ambiente do portal — nunca
em `docs/`, nunca no navegador, nunca em log (`AGENTS.md` §5.7).

## 5. Segurança e fronteira

1. **Somente `staff`.** As seis rotas validam sessão e papel no servidor; `member` recebe `403`
   (`RNF-POR-001`).
2. **Somente o BFF fala com Prometheus, Loki e Grafana.** O navegador recebe apenas a URL pública do
   `iframe`/link; nenhuma consulta sai do browser.
3. **Allowlist, nunca LogQL do usuário.** Serviço e nível são listas fechadas; texto livre vira `|=`
   literal, com metacaractere recusado e teto de 64 caracteres.
4. **Exportação limitada.** NDJSON, última hora, no máximo 1.000 linhas, `staff`; resposta sem cache.
5. **`trace_id` por ação.** O BFF envia `x-trace-id` ao `wyd-admin` e o repete no log; o id não vira
   rótulo de métrica (cardenalidade por requisição estouraria a série).

## 6. Retenção e histórico

| Fonte | Retenção | Dono |
|---|---|---|
| Prometheus | 7 dias (`--storage.tsdb.retention.time=7d`) | `wyd-ops` |
| Loki | 14 dias (`retention_period: 336h`) | `wyd-ops` |
| Alertmanager | 120 h de dados | `wyd-ops` |
| Telemetria do launcher | agregado em processo (desde o boot) + linhas no Loki por 14 dias | `wyd-ops` |
| Portal | **nenhuma** série, nenhum expurgo: o portal não guarda métrica, log ou evento | — |

`V` (medido em 2026-11-20): `deprecated/docker-compose.yml` (retenção do Prometheus) e
`deprecated/docker/monitoring/loki/loki.yml` (14 dias). Os valores são da pilha de referência; a
retenção definitiva é contrato do `wyd-ops`.

## 7. Paridade com o legado

Toda função do legado tem destino na `1.0.0`; nada foi descartado por conveniência.

| Função do legado (`V`, 2026-11-20) | Na `1.0.0` |
|---|---|
| Resumo com estado das quatro fontes e motor por canal | rota 38 |
| Detalhe por canal com handshakes, sessões, RSS e orçamento | rota 39 |
| Presença por canal lida do banco (sobrevive a restart) | rota 39, por `game.v_live_session` |
| Alvos e alertas do Prometheus, incluindo “outros alvos” | rota 40 |
| Catálogo e `embed` dos dashboards do Grafana | rota 41 |
| Volume/erros/avisos por serviço e exportação NDJSON | rota 42 |
| Telemetria anônima do launcher por versão e operação | rotas 43 e `POST /api/launcher/telemetria` |
| Serviços do resumo do painel (`/painel`) | rotas 38 e 43; o NOC (23) mantém os cartões |

**O que a `1.0.0` acrescenta** (`P`): janela de série por cartão (`query_range`), cartão de pacotes
recusados por motivo, estado “não medido” explícito em cada fonte e o mesmo `trace_id` ligando a ação do
staff ao log do `wyd-admin`.

## 8. Quando a fonte falha

| Falha | Efeito |
|---|---|
| Prometheus fora | cartões de motor e host em `—`, aviso “métricas indisponíveis”; presença, logs e painéis continuam |
| Loki fora | painel de logs “fora”; o resto da tela 38 segue |
| Grafana fora | catálogo vazio com o motivo; link direto continua exibido |
| Banco fora | presença e latência “indisponível”; métricas da pilha continuam |
| Launcher sem eventos | estado vazio com a explicação, nunca número inventado |

Regra única: **falha de uma fonte nunca zera número de outra, e ausência de medição nunca vira `0`.**

## 9. O que esta página não decide

| Assunto | Onde fecha |
|---|---|
| URL, credencial, intervalo de coleta, retenção e alertas definitivos | planejamento do `wyd-ops` |
| Forma do contrato HTTP do `wyd-admin` (incluindo `x-trace-id`) | planejamento do `wyd-admin` |
| Quais dashboards são provisionados no Grafana | `wyd-ops` |
| Série histórica de negócio (contas novas, retenção D1/D7/D30) | consulta ao banco/`wyd-ops`, não esta página |
| Substituir o Grafana por gráficos próprios | fora do escopo: o portal é porta de entrada e resumo |

## 10. Prova de implementação

1. `curl -s localhost:3000/admin/monitoramento` sem sessão → redireciona ao login; com `member` → `403`.
2. Prometheus parado: as seis telas respondem, com “não medido” nos cartões de métrica e presença viva.
3. `GET /api/logs/export?limit=1001` → `422`; `limit=500` → NDJSON com `content-disposition`.
4. `POST /api/launcher/telemetria` com campo extra ou corpo acima do limite → `400`/`413`, sem gravar.
5. Nenhuma resposta contém URL interna de Prometheus/Loki/Grafana nem credencial.

*Página de planejamento. Nenhuma tela, consulta ou integração foi implementada.*

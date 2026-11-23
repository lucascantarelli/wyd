# wyd-server 1.0.0 — abertura do planejamento

> Pergunta: "como o processo do servidor é composto, o que ele expõe para operação e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-server`
> Dono: equipe do `wyd-server`
> Base: `wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §3 (`LAC-ADM-03`); `wyd-portal/docs/arquitetura/fronteira.md` §3; `AGENTS.md` §6; `docs/specs/endpoint-de-captura.md`
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código** e nenhum requisito numerado (`RF-SRV`). O plano nasce com **uma obrigação já
decidida por outro sistema** — o endpoint de controle — e cinco lacunas: quatro fechadas em 2026-11-23
e uma aberta, o peer de captura A1 (`LSRV-05`).

## 1. O que ele faz

Compõe o processo do servidor e roda **um processo por canal**: abre o bind do CPSock, monta o
`wyd-net` sobre o `wyd-core`, publica saúde e prontidão para `wyd-ops` e recebe comando
administrativo. É o dono da topologia — quem sobe, em que porta, com que canal — e do ciclo de vida
do processo, não da regra de jogo.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| codec, opcode e layout do fio | `wyd-protocol` |
| socket, timeout e rate-limit | `wyd-net` |
| tick, combate, cena e IA | `wyd-core` / `wyd-sim` |
| SQL, migrations e transação | `wyd-db` |
| escrita administrativa e auditoria | `wyd-admin` |
| patch e manifesto do cliente | `wyd-launcher` / `wyd-client` |
| deploy, imagem e monitoramento | `wyd-ops` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Dono | Estado |
|---|---|---|---|---|---|---|
| S1 | controle administrativo (`kick`, `broadcast`) | server ← `wyd-admin` | `POST /internal/v1/control`, `200` com `{delivered_to}` | **sem ack, sem escrita** no admin; `503` se o processo caiu | `wyd-server` (forma), `wyd-admin` (elo) | **decidido** (`LAC-ADM-03`) |
| S2 | autenticação do elo de controle | server verifica | segredo próprio `WYD_SERVER_CONTROL_TOKEN` | `401` sem ele; segredo distinto do portal → admin | `wyd-ops` (emissão) | **decidido** (`LAC-ADM-03` §3.3) |
| S3 | saúde e prontidão | `wyd-ops` ← server | `GET /health`, `GET /ready` | `200` só com socket + banco + tick; `503` sem detalhe interno | `wyd-ops` | **decidido** (`LSRV-03`) |
| S4 | métricas para o portal (NOC) | `wyd-portal` ← `/metrics` | Prometheus, coletado por `wyd-ops` | amostra com instante; sem série no banco | `wyd-ops` | **decidido** (`LAC-POR-06`) |
| S5 | composição net + core | server monta | um binário, um processo por canal | uma porta por canal, sem estado global entre canais | `wyd-server` | **decidido** (`LSRV-01`, `LSRV-02`) |
| S6 | mapa canal → URL de controle | configuração de deploy | `WYD_SERVER_CONTROL_URLS` | muda com a topologia, não com o jogador | `wyd-ops` | **decidido** (`LAC-ADM-03` §3.3) |
| S7 | captura local (`ECAP`) | CLI de `wyd-client` → endpoint local | TCP em loopback, uma conexão, bruto `.wydcap`; separado de `EPROD` | sem produção; peer de A1 em `Q-SRV-5` | `wyd-server` (endpoint), `wyd-client` (CLI) | **proposto** (`docs/specs/endpoint-de-captura.md`) |

## 4. Estado medido (`V`, 2026-11-22; páginas recontadas em 2026-11-23)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 6 | `find planejamento-1.0.0/wyd-server -name '*.md'` (2026-11-23, com o esqueleto e a spec do ECAP) |
| Arquivos de código | 0 | `find crates/wyd-server -type f` (só `.gitkeep`) |
| Endpoints implementados | 0 | nenhum código existe; `EPROD` e `ECAP` são propostas |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

Uma lacuna aberta: `LSRV-05`/`Q-SRV-5`, o peer que completa o handshake em A1. O `ECAP` está
especificado em [`docs/specs/endpoint-de-captura.md`](docs/specs/endpoint-de-captura.md), mas sem peer
A1 grava bytes e não é conclusiva. `LSRV-03` e `LSRV-04` fecharam em 2026-11-23, junto de `LSRV-01` e
`LSRV-02`; o que falta nelas é implementação, não decisão.

## 6. Gate para sair do papel

1. `LSRV-01`–`LSRV-04` decididas em 2026-11-23: dois canais, `ready` com socket + banco + tick e
   shutdown drenando/persistindo; falta implementar o processo, o teste de isolamento e o shutdown.
2. Endpoint de controle especificado com o formato do pacote de broadcast (`wyd-protocol`).
3. `Q-SRV-5` respondida e o peer de A1 registrado; sem ele A1 não é conclusiva.
4. Spec do `ECAP` aceita e DoD de §8 executável; `EPROD` continua separado e não implementado.
5. `RF-SRV` numerados com critério verificável, incluindo `503` sem escrita.
6. Árvore de `src/` e `tests/` planejada; teste de dois canais isolados.
7. Plano de encerramento: `SIGTERM` drena e persiste, sem perda silenciosa.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/arquitetura/topologia-de-canais.md`,
`docs/specs/endpoint-de-captura.md` (escrita) e `docs/roadmap/` com DoD e a issue do `ECAP`.
O peer de A1 continua na decisão aberta `Q-SRV-5`.

A fatia mínima que serve de gate à onda C — login, entrada no mundo e um passo — já está escrita como proposta
em [`docs/planos/esqueleto-de-ponta-a-ponta.md`](docs/planos/esqueleto-de-ponta-a-ponta.md) (`P`, 2026-11-23):
7 mensagens mínimas, 5 cenários/6 capturas, 7 goldens e os critérios C1–C7. Ela não muda o gate deste
plano; `LSRV-01`–`LSRV-04` estão decididas, e o passo 0 depende da implementação do processo, do `ECAP`,
do peer `Q-SRV-5` e de `LPRT-03`.

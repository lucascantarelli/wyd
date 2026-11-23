# wyd-server 1.0.0 — abertura do planejamento

> Pergunta: "como o processo do servidor é composto, o que ele expõe para operação e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-server`
> Dono: equipe do `wyd-server`
> Base: `wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §3 (`LAC-ADM-03`); `wyd-portal/docs/arquitetura/fronteira.md` §3; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código** e nenhum requisito numerado (`RF-SRV`). O plano nasce com **uma obrigação já
decidida por outro sistema** — o endpoint de controle — e cinco perguntas abertas.

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
| S3 | saúde e prontidão | `wyd-ops` ← server | `GET /health`, `GET /ready` | `200`/`503` sem detalhe interno | `wyd-ops` | **proposto** |
| S4 | métricas para o portal (NOC) | `wyd-portal` ← `/metrics` | Prometheus, coletado por `wyd-ops` | amostra com instante; sem série no banco | `wyd-ops` | **decidido** (`LAC-POR-06`) |
| S5 | composição net + core | server monta | a definir | uma porta por canal, sem estado global entre canais | `wyd-server` | **lacuna** `LSRV-01` |
| S6 | mapa canal → URL de controle | configuração de deploy | `WYD_SERVER_CONTROL_URLS` | muda com a topologia, não com o jogador | `wyd-ops` | **decidido** (`LAC-ADM-03` §3.3) |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-server -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-server -type f` (só `.gitkeep`) |
| Endpoints implementados | 0 | nenhum código existe |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

`lacunas.md`: `LSRV-01` (composição dos canais e fronteira com o core), `LSRV-02` (quantos canais e
qual a porta de cada um), `LSRV-03` (o que `ready` significa de verdade) e `LSRV-04` (encerramento
gracioso com fila e save).

## 6. Gate para sair do papel

1. `LSRV-01` e `LSRV-02` decididas: sem topologia não há teste de dois canais.
2. Endpoint de controle especificado com o formato do pacote de broadcast (`wyd-protocol`).
3. `RF-SRV` numerados com critério verificável, incluindo `503` sem escrita.
4. Árvore de `src/` e `tests/` planejada; teste de dois canais isolados.
5. Plano de encerramento: `SIGTERM` drena e persiste, sem perda silenciosa.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/arquitetura/topologia-de-canais.md`,
`docs/specs/` com os `RF-SRV` e `docs/roadmap/` com DoD.

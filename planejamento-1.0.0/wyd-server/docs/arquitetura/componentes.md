> Pergunta: "quais unidades de código o `wyd-server` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-server`
> Dono: equipe do wyd-server
> Base: `wyd-server-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `composition`

- **Responsabilidade:** compor `net` + `core` + dados em um processo; `main` só compõe, lê configuração e inicia o canal.
- **API:** `compose(config) -> Server`, `run`.
- **Invariantes:** um binário; um processo por canal; nenhum crate vira binário próprio; sem estado global entre canais.
- **Depende de:** `wyd-net`, `wyd-core`, `wyd-db` (adapter), `config`.
- **Testes:** composição de dois canais isolados; configuração inválida recusa.
- **Risco:** alto — é a topologia.

## `channel`

- **Responsabilidade:** abrir bind do CPSock, montar net sobre core, expor porta por canal.
- **API:** `open(channel_id)`, `close`.
- **Invariantes:** uma porta por canal; canal inexistente não abre; sem estado global entre canais.
- **Depende de:** `composition`, `wyd-net`.
- **Testes:** dois canais isolados; bind em porta ocupada recusa.
- **Risco:** alto.

## `health`

- **Responsabilidade:** `GET /health` e `GET /ready` com semântica de prontidão.
- **API:** `health() -> 200`, `ready() -> 200|503`.
- **Invariantes:** `ready` só 200 com socket aceito + banco alcançável + tick rodando; senão 503 sem detalhe interno.
- **Depende de:** `wyd-net`, `wyd-db` (ping), `wyd-core` (tick).
- **Testes:** ready 503 sem banco; ready 200 com tudo.
- **Risco:** médio.

## `control`

- **Responsabilidade:** endpoint `POST /internal/v1/control` para kick/broadcast do `wyd-admin`.
- **API:** `control(req) -> 200 {delivered_to} | 401 | 503`.
- **Invariantes:** segredo `WYD_SERVER_CONTROL_TOKEN`; 401 sem ele; 503 se o processo caiu; sem ack, sem escrita no admin.
- **Depende de:** `wyd-net`, `wyd-core`, `auth`.
- **Testes:** 401 sem token; 503 processo caído; 200 com delivered_to.
- **Risco:** alto — superfície administrativa.

## `auth`

- **Responsabilidade:** verificar segredo do elo de controle; distinto do portal → admin.
- **API:** `verify(token) -> bool`.
- **Invariantes:** segredo próprio; nunca reutilizado.
- **Depende de:** config (`WYD_SERVER_CONTROL_TOKEN`).
- **Testes:** token válido/inválido.
- **Risco:** alto.

## `metrics`

- **Responsabilidade:** expor `/metrics` Prometheus para `wyd-ops` coletar.
- **API:** `metrics() -> text`.
- **Invariantes:** amostra com instante; sem série no banco; sem PII.
- **Depende de:** `wyd-net`, `wyd-core`, `observability`.
- **Testes:** métricas presentes.
- **Risco:** baixo.

## `lifecycle`

- **Responsabilidade:** startup, shutdown drenando, persistindo, fechando fila.
- **API:** `start`, `shutdown(signal)`.
- **Invariantes:** `SIGTERM` drena e persiste, sem perda silenciosa; `WYDDB-021` dona da sequência.
- **Depende de:** `composition`, `wyd-db`.
- **Testes:** shutdown drena; sem perda.
- **Risco:** alto.

## `config`

- **Responsabilidade:** ler configuração (canais, URLs de controle, tokens).
- **API:** `load() -> Config`.
- **Invariantes:** muda com topologia, não com jogador; `WYD_SERVER_CONTROL_URLS`.
- **Depende de:** ambiente.
- **Testes:** config inválida recusa.
- **Risco:** baixo.

## `capture` (ECAP)

- **Responsabilidade:** endpoint local de captura `.wydcap` em loopback, separado de `EPROD`.
- **API:** `capture(buf) -> file`.
- **Invariantes:** sem produção; peer de A1; handshake mínimo local.
- **Depende de:** `wyd-protocol`, `wyd-net`.
- **Testes:** captura local; não exposto externamente.
- **Risco:** médio.

## `observability`

- **Responsabilidade:** métricas e logs de metadados; sem payload/PII.
- **API:** `metrics`, `log_meta`.
- **Invariantes:** nunca payload/PII em log.
- **Depende de:** todas.
- **Testes:** log sem payload.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-server/ (raiz) ou crates/wyd-server/
├── composition/          # compõe net+core+dados
├── channel/              # bind por canal
├── health/               # /health, /ready
├── control/              # endpoint de controle
├── auth/                 # segredo do elo
├── metrics/              # /metrics
├── lifecycle/            # startup/shutdown
├── config/               # configuração
├── capture/              # ECAP local
└── observability/        # métricas/logs
tests/
├── unitario/            # config, auth, health
├── contrato/            # controle, composição
├── integracao/          # dois canais
├── sistema/             # shutdown, ready
├── e2e/                 # ponta a ponta
└── propriedade/         # fuzz de config
```

*Página de planejamento. Nenhuma unidade foi implementada.*
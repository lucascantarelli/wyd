> Pergunta: "quais unidades de código o `wyd-net` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-net`
> Dono: equipe do wyd-net
> Base: `wyd-net-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `transport`

- **Responsabilidade:** aceitar conexão TCP, manter a sessão por socket, ler/escrever bytes por conexão, fechar.
- **API:** `accept`, `read`, `write`, `close`; timeout ocioso e limite por conexão aplicados aqui.
- **Invariantes:** uma conexão = uma sessão; sem paralelismo de despacho por conexão; nenhuma mensagem vaza para outra conexão.
- **Depende de:** `session`, `observability`.
- **Testes:** conexão aceita/fechada; timeout ocioso; limite por conexão (`LNET-01`).
- **Risco:** alto — é a superfície de entrada do fio.

## `session`

- **Responsabilidade:** identidade por conexão, uma sessão viva por conta, recusa da segunda conexão, estado de versão de protocolo.
- **API:** `open`, `identify`, `check_version`, `close`; erro tipado para build não suportado.
- **Invariantes:** build-alvo 7662 imposto no handshake; segunda conexão da mesma conta recusada; `R6`/`D9`/`ux_session_viva_por_conta`.
- **Depende de:** `transport`, `error`.
- **Testes:** segunda conexão recusada; build fora do alvo recusado; sessão fechada libera conta.
- **Risco:** alto — sessão é identidade.

## `protocol-bridge`

- **Responsabilidade:** entregar ao codec do `wyd-protocol` o buffer de cada conexão e devolver ao `wyd-core` a mensagem tipada.
- **API:** `decode_frame(buf) -> Frame`, `encode(frame) -> bytes`, `dispatch(frame)`.
- **Invariantes:** buffer completo por conexão; bytes crus não saem do crate (`LPRT-18`); nenhum offset cru atravessa para o `wyd-core`.
- **Depende de:** `wyd-protocol`, `error`.
- **Testes:** roundtrip por conexão; buffer parcial não despacha; bytes crus não vazam.
- **Risco:** alto — é o ponto de tradução fio ↔ core.

## `routing`

- **Responsabilidade:** rotear mensagem tipada para o handler do `wyd-core` por `Type`, preservando ordem.
- **API:** `route(msg)`, fila por conexão FIFO.
- **Invariantes:** ordem por conexão sem paralelismo de despacho; `Type` desconhecido não causa queda por si só.
- **Depende de:** `protocol-bridge`, `wyd-core` (contrato), `error`.
- **Testes:** ordem preservada; `UnknownType` logado/descartado; sem reordenação.
- **Risco:** médio — ordem é requisito de combate/inventário.

## `validation`

- **Responsabilidade:** validar limites por conexão, IP, shard, timeout e rate-limit; recusa tipada + contador + fechamento.
- **API:** `check(conn, ip, shard) -> Ok | Refused(reason)`.
- **Invariantes:** valores medidos, não prometidos (`LNET-01` aberta); forma fechada por `Q-NET-1` = A; cenário `Q-BOT-3` para perfil de teste.
- **Depende de:** `transport`, `observability`, `error`.
- **Testes:** recusa tipada; contador incrementado; fechamento; medição no CI efêmero.
- **Risco:** médio — limites sem medição viram opinião.

## `backpressure`

- **Responsabilidade:** controlar fluxo de saída por conexão, evitar acúmulo de bytes, fechar em overflow.
- **API:** `enqueue`, `drain`, `overflow`.
- **Invariantes:** sem bloqueio indefinido; overflow fecha a conexão com erro tipado.
- **Depende de:** `transport`, `observability`.
- **Testes:** overflow fecha; drenagem ordenada.
- **Risco:** médio.

## `reconnect`

- **Responsabilidade:** política de reconexão do cliente (lado servidor: detectar queda, limpar sessão, permitir nova conexão).
- **API:** `on_disconnect`, `cleanup`, `allow_reopen`.
- **Invariantes:** sessão fechada libera conta; reabertura obedece `R6`/`D9`.
- **Depende de:** `session`, `wyd-db` (contrato).
- **Testes:** queda libera conta; reabertura permitida após cleanup.
- **Risco:** médio.

## `error`

- **Responsabilidade:** taxonomia de erro do transporte (conexão, sessão, codec, validação) com classe recuperável/fatal.
- **API:** enum tipado; mapeamento para reação sugerida; `HeaderError`/`TransformError`/`PayloadError` consumidos do codec.
- **Invariantes:** classificação é do codec, reação é da conexão (`LPRT-13`); nenhuma duplicação de taxonomia.
- **Depende de:** nenhum módulo de negócio.
- **Testes:** mapeamento de cada variante; classe recuperável/fatal.
- **Risco:** médio.

## `observability`

- **Responsabilidade:** expor `/metrics` Prometheus (conexões, erros por tipo, bytes, latência) e logs só de metadados.
- **API:** `metrics`, `log_meta`.
- **Invariantes:** nunca payload/PII em log (`AGENTS.md` §5, item 7); fonte externa `LAC-POR-06`; nomes de métrica são implementação.
- **Depende de:** `transport`, `session`, `error`.
- **Testes:** métricas presentes; log sem payload.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-net/ (raiz) ou crates/wyd-net/
├── transport/            # aceite, leitura, escrita, fechamento, timeout
├── session/              # identidade, versão, uma sessão por conta
├── protocol-bridge/      # codec + dispatch
├── routing/              # roteamento por Type, ordem FIFO
├── validation/           # limites por conexão/IP/shard/timeout
├── backpressure/         # fluxo de saída, overflow
├── reconnect/            # queda, cleanup, reabertura
├── error/                # taxonomia do transporte
└── observability/        # /metrics, logs de metadados
tests/
├── unitario/            # transport, session, error (Rust)
├── contrato/            # contrato com wyd-protocol e wyd-core
├── integracao/          # conexão real, buffer parcial, timeout
├── sistema/             # portas falsas, backpressure
├── e2e/                 # ponta a ponta com codec real
└── propriedade/         # fuzz de buffer
```

*Página de planejamento. Nenhuma unidade foi implementada.*

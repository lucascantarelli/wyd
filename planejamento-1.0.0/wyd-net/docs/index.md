# wyd-net — visão geral

> Pergunta: "o que o `wyd-net` é e por onde começo a ler?"
> Público: Desenvolvedor novo e arquitetura
> Dono: equipe do wyd-net
> Base: `wyd-protocol-1.0.0.md` §1.4; `LPRT-08`, `LPRT-12`, `LPRT-13`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-22)

O `wyd-net` é o transporte TCP do CPSock. Ele aceita a conexão, mantém o socket, entrega ao codec
do `wyd-protocol` o buffer daquela conexão e devolve ao `wyd-core` a mensagem já tipada. Decide o
que acontece com a **conexão** — timeout, limite, ordem, reação a erro — e não decide o que a
mensagem significa.

## O que ele não faz

| Fora | Dono |
|---|---|
| cabeçalho, ofuscação, checksum, catálogo de `Type` | `wyd-protocol` |
| tick, combate, cena, IA | `wyd-core` / `wyd-sim` |
| processo por canal e topologia | `wyd-server` |
| sessão persistida | `wyd-db` (`game.session`) |

## Contratos que ele já herda

| Contrato | Estado | Onde foi decidido |
|---|---|---|
| reação a erro de codec é da conexão, não do codec | decidido | `LPRT-13` |
| `Type` desconhecido é `UnknownType` tipado; a reação é daqui | decidido | `LPRT-08` |
| ordem por conexão é garantida aqui | decidido | `LPRT-12` |
| mensagem cruza tipada e validada, nunca por offset | decidido | `LPRT-18` |

## O que falta

A política de limites e rate-limit (`Q-NET-1` = A) foi decidida em 2026-11-23, mas `LNET-01` continua
**bloqueada como medição**, porque não há conexão, latência nem recusa medidas. A política de versão
(`LNET-02`) foi decidida em 2026-11-23 (`Q-NET-2` = A: impor o build-alvo 7662 no handshake, com erro
tipado para build não suportado) e o modelo de sessão (`LNET-03`) está decidido por
`R6`/`D9`/`ux_session_viva_por_conta`; a observabilidade (`LNET-04`) fechou na mesma data por **regra
vigente** (`X10`, `LAC-POR-06` e `AGENTS.md` §5, item 7). Nessas três só falta o transporte implementar. A
aberta não tem número: esta página não afirma limite nem timeout.

## Limite desta documentação

- Não descreve byte, offset ou opcode: dono é `wyd-protocol`.
- Não descreve regra de jogo: dono é `wyd-core` e `wyd-sim`.
- Não afirma capacidade, carga ou latência: nada foi medido.

## Decisões registradas

As quatro perguntas do sistema foram respondidas ou derivadas: `Q-NET-1`, `Q-NET-2` e `Q-NET-4` = A, e
`LNET-03` pela regra de sessão por conta. `LNET-01` **continua bloqueada como medição**, sem número
inventado; `LNET-02`, `LNET-03` e `LNET-04` fecharam. O estado de cada lacuna está em `lacunas.md`; o
briefing com a ordem das respostas é `../../briefing-de-decisoes-1.0.0.md`.

## Mapa de páginas

Este mapa garante que toda página de `docs/` seja alcançável a partir deste índice; a pasta dona continua sendo a fonte do conteúdo.

### raiz

- [Decisões do `wyd-net` — limites, versão e observabilidade](proposta-decisoes-net.md)

### arquitetura

- [componentes](arquitetura/componentes.md)

### arquitetura/adr

- [ADR-NET-0001 — Política de limites (Q-NET-1 = A)](arquitetura/adr/adr-net-0001-politica-de-limites.md)
- [ADRs — `wyd-net`](arquitetura/adr/index.md)

### planos

- [plano-de-testes](planos/plano-de-testes.md)

### roadmap

- [index](roadmap/index.md)
- [roadmap](roadmap/roadmap.md)

### specs

- [requisitos](specs/requisitos.md)

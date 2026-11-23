# wyd-net — visão geral

> Pergunta: "o que o `wyd-net` é e por onde começo a ler?"
> Público: Desenvolvedor novo e arquitetura
> Dono: equipe do wyd-net
> Base: `wyd-protocol-1.0.0.md` §1.4; `LPRT-08`, `LPRT-12`, `LPRT-13`
> Revisado em: 2026-11-23

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

Uma pergunta aberta — limites e rate-limit — está no registro de lacunas do sistema (`lacunas.md`,
planejamento). A política de versão (`LNET-02`) foi decidida em 2026-11-23 (`Q-NET-2` = A: impor o
build-alvo 7662 no handshake, com erro tipado para build não suportado) e o modelo de sessão (`LNET-03`)
está decidido por `R6`/`D9`/`ux_session_viva_por_conta`; a observabilidade (`LNET-04`) fechou na mesma
data por **regra vigente** (`X10`, `LAC-POR-06` e `AGENTS.md` §5.7). Nessas três só falta o transporte
implementar. A aberta não tem resposta: esta página não afirma número de limite nem timeout.

## Limite desta documentação

- Não descreve byte, offset ou opcode: dono é `wyd-protocol`.
- Não descreve regra de jogo: dono é `wyd-core` e `wyd-sim`.
- Não afirma capacidade, carga ou latência: nada foi medido.

## Decisões em aberto

A pergunta que ainda espera o dono — `LNET-01` — está em [Decisões do `wyd-net`](proposta-decisoes-net.md); `LNET-02` foi decidida em 2026-11-23 (opção A), `LNET-03` está resolvida pela regra de sessão por conta e `LNET-04` fechou por regra vigente (métrica Prometheus, log só de metadados), e nenhuma delas é reaberta. O estado de cada lacuna continua em `lacunas.md`; o briefing com a ordem das respostas é `../../briefing-de-decisoes-1.0.0.md`.

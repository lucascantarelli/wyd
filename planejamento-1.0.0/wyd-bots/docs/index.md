# wyd-bots — visão geral

> Pergunta: "para que servem os bots e o que cada um prova?"
> Público: QA, arquitetura e operação
> Dono: equipe do wyd-bots
> Base: `wyd-protocol-1.0.0.md` §1.4; `AGENTS.md` §8
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-bots` usa o mesmo codec do servidor em modo cliente para três papéis: **QA de fio** (fluxo
ponta a ponta), **carga** (N conexões com perfil declarado) e **convivência** (presença sintética
controlada). É o nível mais alto e mais caro da pirâmide; não substitui teste unitário nem de
contrato.

## O que ele não faz

| Fora | Dono |
|---|---|
| codec e layout do fio | `wyd-protocol` |
| socket, sessão, rate-limit | `wyd-net` |
| regra de jogo | `wyd-core` / `wyd-sim` |
| ambiente e infraestrutura | `wyd-ops` |
| métrica de produção | `wyd-ops` |

## O que falta

Quatro perguntas abertas — ambiente alvo, conta sintética, limiar de carga e artefato de falha —
estão em `lacunas.md` (planejamento). Nenhum cenário existe e nenhuma carga foi medida: esta página
não afirma conexões simultâneas, latência nem taxa de sucesso.

## Limite desta documentação

- Não promete número de bots: limiar é medido, não estimado.
- Não autoriza bot em produção: falta decisão de ambiente (`LBOT-01`).
## Decisões em aberto

As perguntas que ainda esperam o dono — `LBOT-01`–`LBOT-04` — estão em [Decisões do `wyd-bots`](proposta-decisoes-bots.md). O estado de cada lacuna continua em `lacunas.md`.

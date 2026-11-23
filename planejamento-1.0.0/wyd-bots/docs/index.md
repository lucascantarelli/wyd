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

Duas perguntas abertas — conta sintética e limiar de carga — estão em `lacunas.md` (planejamento). O
ambiente alvo (`LBOT-01`) fechou em 2026-11-23 pela resposta do dono (`Q-BOT-1` = A): CI efêmero em
runner hospedado, com servidor descartável por execução. O artefato de falha (`LBOT-04`) fechou na
mesma data por regra vigente, porque o gate já exigia log + captura e `AGENTS.md` §5.7/`P-22` fixam a
higiene. Nenhum cenário existe e nenhuma carga foi medida: esta página não afirma conexões simultâneas,
latência nem taxa de sucesso.

## Limite desta documentação

- Não promete número de bots: limiar é medido, não estimado.
- Não autoriza bot em produção: o ambiente é CI efêmero (`Q-BOT-1` = A), e produção continua fora.
## Decisões em aberto

As perguntas que ainda esperam o dono — `LBOT-02` e `LBOT-03` — estão em [Decisões do `wyd-bots`](proposta-decisoes-bots.md); `LBOT-01` fechou por resposta do dono (`Q-BOT-1` = A) e `LBOT-04` por regra vigente, e nenhuma das duas é reaberta. O estado de cada lacuna continua em `lacunas.md`; o briefing com a ordem das respostas é `../../briefing-de-decisoes-1.0.0.md`.

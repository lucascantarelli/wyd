# wyd-bots — visão geral

> Pergunta: "para que servem os bots e o que cada um prova?"
> Público: QA, arquitetura e operação
> Dono: equipe do wyd-bots
> Base: `wyd-protocol-1.0.0.md` §1.4; `AGENTS.md` §8
> Revisado em: 2026-11-25

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

A política do limiar de carga (`Q-BOT-3` = A) foi decidida em 2026-11-23, mas a lacuna `LBOT-03`
continua **bloqueada como medição**, porque ainda não há número, conexão nem latência medidos. O ambiente alvo
(`LBOT-01`) fechou em 2026-11-23 pela resposta do dono (`Q-BOT-1` = A): CI efêmero em runner hospedado,
com servidor descartável por execução. A conta e o personagem sintéticos (`LBOT-02`) fecharam na mesma
data (`Q-BOT-2` = A): seed declarativo em `crates/wyd-db/seeds/dev.sql`, aplicado no banco recriado por
execução. O artefato de falha (`LBOT-04`) fechou na mesma data por regra vigente, porque o gate já exigia
log + captura e `AGENTS.md` §5, item 7/`P-22` fixam a higiene. Nenhum cenário existe e nenhuma carga foi
medida: esta página não afirma conexões simultâneas, latência nem taxa de sucesso.

## Limite desta documentação

- Não promete número de bots: limiar é medido, não estimado.
- Não autoriza bot em produção: o ambiente é CI efêmero (`Q-BOT-1` = A), e produção continua fora.
## Decisões registradas

As quatro perguntas do sistema foram respondidas ou derivadas: `Q-BOT-1` e `Q-BOT-2` = A, `Q-BOT-3` =
A (política de medição) e `Q-BOT-4` por regra vigente. `LBOT-01`, `LBOT-02` e `LBOT-04` fecharam;
`LBOT-03` **continua bloqueada como medição**, sem número inventado. O estado de cada lacuna está em
`lacunas.md`; o briefing com a ordem das respostas é `../../briefing-de-decisoes-1.0.0.md`.

## Mapa de páginas

Este mapa garante que toda página de `docs/` seja alcançável a partir deste índice; a pasta dona continua sendo a fonte do conteúdo.

### raiz

- [Módulos do `wyd-bots` — detalhamento](modulos.md)
- [Decisões do `wyd-bots` — ambiente, dado, limiar e artefato](proposta-decisoes-bots.md)

### arquitetura

- [componentes](arquitetura/componentes.md)
- [adr/index](arquitetura/adr/index.md)

### planos

- [plano-de-testes](planos/plano-de-testes.md)

### roadmap

- [index](roadmap/index.md)
- [roadmap](roadmap/roadmap.md)

### specs

- [requisitos](specs/requisitos.md)

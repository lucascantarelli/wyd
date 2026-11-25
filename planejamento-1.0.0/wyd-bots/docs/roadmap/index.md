> Pergunta: "qual é a próxima fatia do `wyd-bots`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-bots`
> Base: `wyd-bots-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

As lacunas `LBOT-01`, `LBOT-02` e `LBOT-04` fecharam em 2026-11-23; `LBOT-03` tem política
decidida (`Q-BOT-3` = A) e só fecha com a primeira execução. A fila tem **5 fatias** em 3 fases.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDBOT-001`–`002` | baseline e ambiente efêmero |
| 2 | `WYDBOT-003`–`004` | bots de QA, carga e convivência |
| 3 | `WYDBOT-005` | suíte, anti-skip e medição |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| medição de carga | `WYDBOT-003`–`004` | `LBOT-03` só fecha com a primeira execução |
| ambiente efêmero | `WYDBOT-001` | `LBOT-01`/`LBOT-02` (Postgres + servidor descartáveis) |
| CI real | `WYDBOT-005` | workflow stub |

## 3. Como ler uma fatia

Cada `WYDBOT-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
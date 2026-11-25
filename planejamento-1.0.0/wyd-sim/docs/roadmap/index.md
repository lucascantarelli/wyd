> Pergunta: "qual é a próxima fatia do `wyd-sim`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-sim`
> Base: `wyd-sim-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

A fila tem **5 fatias** em 3 fases. Todas as 4 lacunas (`LSIM-01`–`04`) fecharam em 2026-11-23 por
decisão explícita do dono; nenhuma fórmula ou fixture escrita.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDSIM-001`–`002` | baseline do crate e contrato de fórmula |
| 2 | `WYDSIM-003`–`004` | curvas e simulações determinísticas |
| 3 | `WYDSIM-005` | suíte, fixtures e anti-skip |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| dado de origem | `WYDSIM-003`–`004` | fórmulas dependem de captura do cliente 7662 e do catálogo funcional |
| CI real | `WYDSIM-005` | workflow stub |

## 3. Como ler uma fatia

Cada `WYDSIM-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
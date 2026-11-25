> Pergunta: "qual é a próxima fatia do `wyd-launcher`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-launcher`
> Base: `wyd-launcher-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

As 4 lacunas (`LLCH-01`–`04`) fecharam em 2026-11-23 por decisão do dono. A fila tem **5 fatias**
em 3 fases.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDLCH-001`–`002` | baseline Tauri e manifesto |
| 2 | `WYDLCH-003`–`004` | patch, serverlist/itemlist |
| 3 | `WYDLCH-005` | suíte, E2E e anti-skip |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| manifesto íntegro | `WYDLCH-003` | depende de `wyd-ops` (geração de patch/upload) |
| CI real | `WYDLCH-005` | workflow stub |

## 3. Como ler uma fatia

Cada `WYDLCH-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
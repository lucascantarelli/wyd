> Pergunta: "qual é a próxima fatia do `wyd-ops`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-ops`
> Base: `wyd-ops/docs/index.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada. `wyd-ops` não tinha
dashboard próprio até esta rodada.

A fila tem **6 fatias** em 4 fases.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDOPS-001`–`002` | CI e imagem |
| 2 | `WYDOPS-003`–`004` | ambiente e monitoramento |
| 3 | `WYDOPS-005` | segredos e deploy |
| 4 | `WYDOPS-006` | gate e fechamento |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| imagem nunca publicada | `WYDOPS-001` | `.github/workflows/docs-image.yml` versionado mas nunca rodou |
| CI real | `WYDOPS-006` | `full-workflow.yml` stub; cobertura 85% no gate do sistema |
| ambiente efêmero | `WYDOPS-003` | `LBOT-01`/`LBOT-02` |

## 3. Como ler uma fatia

Cada `WYDOPS-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
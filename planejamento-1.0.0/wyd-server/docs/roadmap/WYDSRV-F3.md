# WYDSRV-F3 — Observabilidade e suíte

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-server`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (`RF-SRV-007` e todos); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — Observabilidade e suíte
**Fase:** 4


## Contexto

A fatia F3 do `wyd-server` cobre spans, métricas, logs de metadados, suíte, anti-skip e existe para
cumprir `RF-SRV-007` e todos sem antecipar a fatia seguinte.

## Entregável

spans, métricas, logs de metadados, suíte, anti-skip

## DoD

- [ ] incidente simulado diagnosticado.
- [ ] alvos de `plano-de-testes.md` §2 sem `SKIP`.
- [ ] cobertura ≥ 85%.

## Fora de escopo

migrations

## Depende de

F0–F2; CI real

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F3 concluída em `docs/roadmap/roadmap.md`.

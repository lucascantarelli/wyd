# WYDCORE-F3 — IA e capacidades

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-core`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (`RF-CORE-006`, `RF-CORE-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — IA e capacidades
**Fase:** 3


## Contexto

A fatia F3 do `wyd-core` cobre iA de mob, skills, classes, quests (modelo) e existe para
cumprir `RF-CORE-006`, `RF-CORE-007` sem antecipar a fatia seguinte.

## Entregável

IA de mob, skills, classes, quests (modelo)

## DoD

- [ ] comportamento por classe coberto por teste.
- [ ] catálogo fechado (sem placeholder).

## Fora de escopo

implementação de conteúdo

## Depende de

F1; catálogo funcional

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F3 concluída em `docs/roadmap/roadmap.md`.

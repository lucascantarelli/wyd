# WYDDOC-F0 — Montagem e tema

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-docs`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-DOC-001`, `RF-DOC-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F0 — Montagem e tema
**Fase:** 1

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F0 do `wyd-docs` cobre `preparar.py`, tema Jinja, navegação e existe para
cumprir `RF-DOC-001`, `RF-DOC-007` sem antecipar a fatia seguinte.

## Entregável

`preparar.py`, tema Jinja, navegação

## DoD

- [ ] build gera `.build/docs/` com caminho relativo preservado.
- [ ] sem conteúdo interno.

## Fora de escopo

gate

## Depende de

`planejamento-1.0.0/<sistema>/docs/**/*.md`

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `frontend-design` |
| `skills_complementares` | `documentation-writer` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

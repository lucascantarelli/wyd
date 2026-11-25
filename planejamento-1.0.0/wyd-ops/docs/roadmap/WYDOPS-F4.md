# WYDOPS-F4 — Segredos e deploy

> Pergunta: "o que a fatia F4 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F4; `docs/specs/requisitos.md` (`RF-OPS-005`, `RF-OPS-006`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F4 — Segredos e deploy
**Fase:** 3

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F4 do `wyd-ops` cobre secret store, deploy contínuo, rollback e existe para
cumprir `RF-OPS-005`, `RF-OPS-006` sem antecipar a fatia seguinte.

## Entregável

secret store, deploy contínuo, rollback

## DoD

- [ ] segredo nunca em código.
- [ ] deploy em staging.
- [ ] rollback validado.

## Fora de escopo

gate

## Depende de

F0–F3

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F4 concluída em `docs/roadmap/roadmap.md`.

# WYDPOR-F0 — Fundação do portal (auth e sessão)

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-POR-001` a `RF-POR-003`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F0 — Fundação do portal (auth e sessão)
**Fase:** 1

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F0 do `wyd-portal` cobre app Next.js, login, cadastro, recuperação, middleware de sessão, layout dos dois temas e existe para
cumprir `RF-POR-001` a `RF-POR-003` sem antecipar a fatia seguinte.

## Entregável

app Next.js, login, cadastro, recuperação, middleware de sessão, layout dos dois temas

## DoD

- [ ] E2E de cadastro → verificação → login → logout verde.
- [ ] sessão revogada não acessa `/conta`.
- [ ] `npm run build` e `tsc --noEmit` limpos.

## Fora de escopo

área administrativa, WYDpedia, atlas

## Depende de

`0001`–`0004` aplicadas

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

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

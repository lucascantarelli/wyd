# WYDPOR-F6 — CMS e launcher

> Pergunta: "o que a fatia F6 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F6; `docs/specs/requisitos.md` (`RF-POR-006` (edição)); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F6 — CMS e launcher
**Fase:** 5

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F6 do `wyd-portal` cobre `/admin/conteudo/noticias`, `/admin/conteudo/wydpedia`, `/admin/launcher` e existe para
cumprir `RF-POR-006` (edição) sem antecipar a fatia seguinte.

## Entregável

`/admin/conteudo/noticias`, `/admin/conteudo/wydpedia`, `/admin/launcher`

## DoD

- [ ] publicar/despublicar notícia reflete no público em até um ciclo de revalidação.
- [ ] versão de launcher só publica manifesto íntegro.

## Fora de escopo

geração de patch e upload de binário (`wyd-ops`)

## Depende de

— (`0030` aprovado em 2026-11-20: `news`, `banner`, `guide`, `launcher_release`)

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

Saída do comando do DoD no PR, com a fatia F6 concluída em `docs/roadmap/roadmap.md`.

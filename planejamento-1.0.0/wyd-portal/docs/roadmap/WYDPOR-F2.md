# WYDPOR-F2 — Portal público: home, notícias e rankings

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-POR-005`, `RF-POR-006`, `RF-POR-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F2 — Portal público: home, notícias e rankings
**Fase:** 3

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F2 do `wyd-portal` cobre home, `/noticias*`, `/rankings`, `/download` e existe para
cumprir `RF-POR-005`, `RF-POR-006`, `RF-POR-007` sem antecipar a fatia seguinte.

## Entregável

home, `/noticias*`, `/rankings`, `/download`

## DoD

- [ ] home com banco vazio mostra `0` e hora da medição.
- [ ] ranking exclui `deleted_at`.
- [ ] rascunho de notícia não aparece.

## Fora de escopo

WYDpedia e atlas

## Depende de

F0; views de `0029`; lote das tabelas `portal.news`/`banner` para notícias

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F2 concluída em `docs/roadmap/roadmap.md`.

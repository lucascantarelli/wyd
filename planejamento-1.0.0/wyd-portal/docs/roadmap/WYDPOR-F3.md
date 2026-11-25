# WYDPOR-F3 — WYDpedia e atlas

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (`RF-POR-008`, `RF-POR-009`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F3 — WYDpedia e atlas
**Fase:** 3

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F3 do `wyd-portal` cobre `/wydpedia*`, `/atlas*`, busca e tooltip e existe para
cumprir `RF-POR-008`, `RF-POR-009` sem antecipar a fatia seguinte.

## Entregável

`/wydpedia*`, `/atlas*`, busca e tooltip

## DoD

- [ ] 404 para item inexistente.
- [ ] busca ignora caixa.
- [ ] mapa sem pins mostra estado vazio.
- [ ] link cruzado mínimo por ficha.

## Fora de escopo

simulador de build e calculadora de dano

## Depende de

F2; `0013`–`0019`, `0024` e seus seeds

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

Saída do comando do DoD no PR, com a fatia F3 concluída em `docs/roadmap/roadmap.md`.

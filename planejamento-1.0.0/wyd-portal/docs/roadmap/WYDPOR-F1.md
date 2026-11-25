# WYDPOR-F1 — Conta e personagens

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-POR-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F1 — Conta e personagens
**Fase:** 2

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F1 do `wyd-portal` cobre `/conta`, `/conta/personagens`, `/conta/seguranca`, `/conta/consentimentos`; criação de conta de jogo e existe para
cumprir `RF-POR-004` sem antecipar a fatia seguinte.

## Entregável

`/conta`, `/conta/personagens`, `/conta/seguranca`, `/conta/consentimentos`; criação de conta de jogo

## DoD

- [ ] sexta conta recusada com mensagem de vaga.
- [ ] personagem apagado não aparece.
- [ ] revogar sessão derruba a lista.

## Fora de escopo

edição de personagem e inventário

## Depende de

F0; `0005`/`0006` aplicadas; view de `0029`

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

Saída do comando do DoD no PR, com a fatia F1 concluída em `docs/roadmap/roadmap.md`.

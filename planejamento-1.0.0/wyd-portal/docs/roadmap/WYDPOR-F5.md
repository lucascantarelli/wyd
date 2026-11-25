# WYDPOR-F5 — Admin: jogadores, contas e economia

> Pergunta: "o que a fatia F5 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F5; `docs/specs/requisitos.md` (`RF-POR-011`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F5 — Admin: jogadores, contas e economia
**Fase:** 4

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F5 do `wyd-portal` cobre `/admin/jogadores*`, `/admin/contas*`, `/admin/economia*`, `/admin/seguranca` e existe para
cumprir `RF-POR-011` sem antecipar a fatia seguinte.

## Entregável

`/admin/jogadores*`, `/admin/contas*`, `/admin/economia*`, `/admin/seguranca`

## DoD

- [ ] ficha não devolve hash.
- [ ] bloqueio sem motivo recusado.
- [ ] ajuste de moeda vira linha no ledger.
- [ ] toda ação com autor em `audit_log`.

## Fora de escopo

alteração de inventário item a item (precisa de decisão de operação)

## Depende de

F4; `0005`, `0006`, `0008`–`0011`; `wyd-admin` para ação no processo

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

Saída do comando do DoD no PR, com a fatia F5 concluída em `docs/roadmap/roadmap.md`.

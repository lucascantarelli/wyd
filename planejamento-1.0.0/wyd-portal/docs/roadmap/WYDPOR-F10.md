# WYDPOR-F10 — Mercado: reputação e notificação

> Pergunta: "o que a fatia F10 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F10; `docs/specs/requisitos.md` (`RF-POR-029`, `RF-POR-030`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F10 — Mercado: reputação e notificação
**Fase:** 6

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F10 do `wyd-portal` cobre avaliação do comprador, agregado público do vendedor, central de avisos com contador e existe para
cumprir `RF-POR-029`, `RF-POR-030` sem antecipar a fatia seguinte.

## Entregável

avaliação do comprador, agregado público do vendedor, central de avisos com contador

## DoD

- [ ] uma avaliação por transação e só pelo comprador.
- [ ] agregado confere com as transações concluídas.
- [ ] avaliação oculta não soma.
- [ ] cada evento gera um aviso por destinatário.
- [ ] contador zera ao ler.

## Fora de escopo

comentário com resposta do vendedor e moderação editorial de texto livre

## Depende de

F8 e F9 (transação concluída é a base da avaliação)

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

Saída do comando do DoD no PR, com a fatia F10 concluída em `docs/roadmap/roadmap.md`.

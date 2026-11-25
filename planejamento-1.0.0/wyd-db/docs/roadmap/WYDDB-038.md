# WYDDB-038 — escrever 0008 a 0012 do estado de jogo

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 7 — migrations e evolução de schema

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

O estado de jogo restante e a entrega completam o schema quente; a fila `delivery_queue` é a ponte de escrita do portal.

## Entregável

Escrever `0008`–`0012` conforme `plano-migrations.md` §3, com tabelas, constraints, índices e a fila de entrega; `down` espelhado.

## DoD

- [ ] Os cinco arquivos aplicam depois do corte vertical.
- [ ] `ops.delivery_queue` existe com a chave de idempotência definida no DDL.
- [ ] O contrato não encontra FK para arquivo posterior.
- [ ] As páginas `migrations/0008`–`0012` existem.

## Fora de escopo

Não implementar consumidor, save ou grants; são das issues 17–20.

## Risco

Uma tabela sem DDL confirmado não entra; a issue deve declarar a lacuna em vez de inventar coluna.

## Depende de

WYDDB-037.

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

Arquivos, contrato e páginas de migration.

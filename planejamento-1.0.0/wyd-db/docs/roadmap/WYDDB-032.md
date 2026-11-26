# WYDDB-032 — testar stack de capacidades

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

A matriz `CAP-nn` liga requisito a alvo; sem um alvo que a execute, a cobertura é declaração sem prova.

## Entregável

Criar `sistema_stack_caps` para executar os cenários da matriz `CAP-nn`, um por capacidade, e falhar quando uma capacidade não tiver cenário.

## DoD

- [ ] Cada linha `CAP-nn` tem cenário e alvo declarados.
- [ ] Uma capacidade sem cenário reprova o alvo, não passa vazio.
- [ ] O teste usa banco real e dado isolado.
- [ ] A saída lista capacidade, cenário e resultado.

## Fora de escopo

Não inventar capacidades, não definir cobertura percentual e não substituir os alvos de contrato/sistema.

## Risco

A matriz existe desde 2026-11-24 (`tests/specs/capacidades.md`, **23** capacidades e **0** cobertas); o risco
agora é o inverso: cenário escrito sem capacidade, ou capacidade declarada coberta sem cenário executado.

## Depende de

WYDDB-030 e WYDDB-031; a matriz `CAP-nn` está escrita em `tests/specs/capacidades.md` desde 2026-11-24.

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

Matriz versionada e saída do alvo.

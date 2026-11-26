# WYDDB-033 — criar fixtures e factories isoladas

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

Sem factory, cada teste inventa id e dado; sem limpeza por chave, o paralelismo contamina o resultado.

## Entregável

Criar `tests/common/` com factory por agregado, fixture de conteúdo, limpeza por chave e proibição de estado global; documentar o inventário.

## DoD

- [ ] Nenhum teste usa id mágico; a factory devolve o identificador.
- [ ] Cada teste limpa o que criou e pode rodar em paralelo.
- [ ] Fixture de conteúdo tem versão própria e dado fictício.
- [ ] `tests/fixtures-e-factories.md` lista o inventário real.

## Fora de escopo

Não criar conta de produção, não usar PII real e não definir catálogo de contas (depende de `M19`).

## Risco

Uma factory que compartilha estado global economiza linha e cria flaky; o teste de paralelismo deve pegar.

## Depende de

WYDDB-030.

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

Testes paralelos verdes e página de fixtures.

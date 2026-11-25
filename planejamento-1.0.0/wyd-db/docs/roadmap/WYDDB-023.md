# WYDDB-023 — emitir relatório do provisionamento sem segredo

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 5 — observabilidade e diagnóstico

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P2

## Contexto

O provisionamento informa o que criou ou deixou divergente; sem relatório, o operador não distingue `apply` de `check` e pode achar que um drift foi corrigido.

## Entregável

Emitir `Report` de provisionamento com passos aplicados, drift de `login` e resultado de `check`, sem imprimir SQL de senha ou DSN.

## DoD

- [ ] Teste cobre `Render`, `Plan`, `Apply` e `Check` com saída distinta.
- [ ] Nenhuma senha, DSN ou script inteiro aparece no relatório.
- [ ] O código de saída continua sendo do binário.
- [ ] Drift de `login` é listado por papel, não mascarado.

## Fora de escopo

Não criar telemetria, métrica ou dashboard; é relatório de comando.

## Risco

Relatório que imprime o script pode expor credencial; o teste de redação é obrigatório.

## Depende de

WYDDB-009, WYDDB-015.

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

Teste de relatório e página `src/provision/app/runner.md`.

# WYDDB-014 — ordenar preflight, lock e apply no runner

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 3 — decomposição de portas e separação de SQL

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P1

## Contexto

A ordem do runner é o contrato de segurança: verificar papel/servidor, pegar lock, preparar ledger, calcular plano e só então executar. Inverter o lock e o prepare cria corrida.

## Entregável

Implementar `app/runner.rs` com a ordem `preflight → lock → prepare → plan → apply/down → Report`, usando apenas `Source`, `Ledger`, `MigrationExecutor` e `AdvisoryLock`.

## DoD

- [ ] Teste com portas falsas registra a ordem das chamadas.
- [ ] `status` e `apply --dry-run` não chamam `prepare` nem pegam lock.
- [ ] `down` executa em ordem inversa e não chama `apply`.
- [ ] O primeiro erro interrompe a série; nenhum arquivo seguinte é tentado.

## Fora de escopo

Não implementar CLI, logging ou retry; a issue é a orquestração e o teste de ordem.

## Risco

Uma ordem “equivalente” pode parecer certa e criar janela entre lock e prepare; o teste deve fixar a sequência.

## Depende de

WYDDB-011, WYDDB-012, WYDDB-013.

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

Teste de ordem com portas falsas e página `src/migrate/app/runner.md`.

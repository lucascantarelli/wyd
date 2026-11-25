# WYDDB-012 — implementar Ledger e MigrationExecutor

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

O ledger e a transação de cada arquivo precisam commitar juntos; o `app` não pode montar `BEGIN`/`COMMIT` nem consultar `schema_migrations` diretamente.

## Entregável

Implementar `Ledger`, `MigrationExecutor` e `MigrationTxn` no adapter PostgreSQL, incluindo `prepare`, `begin`, `run`, `record`, `forget` e `commit`.

## DoD

- [ ] Teste de contrato prova um commit por arquivo e rollback em falha.
- [ ] `app/runner` só chama as portas; não contém SQL de controle.
- [ ] Ledger com coluna a menos é recusado por forma, não tratado como vazio.
- [ ] `record` e `run` usam a mesma transação.

## Fora de escopo

Não escrever a série, as migrations ou o comando `down` completo; a issue é a porta e a transação.

## Risco

Um `commit` chamado duas vezes ou esquecido deixa estado parcial; o teste de contrato deve cobrir os dois.

## Depende de

WYDDB-011.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Teste de contrato com `FakeExecutor` e teste de integração do adapter quando o banco existir.

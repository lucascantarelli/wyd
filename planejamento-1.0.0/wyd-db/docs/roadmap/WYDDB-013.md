# WYDDB-013 — implementar AdvisoryLock e timeout

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

Duas migrações concorrentes precisam serializar; um lock de transação não atravessa a série porque cada arquivo tem sua transação.

## Entregável

Implementar `AdvisoryLock` com `pg_advisory_lock`, chave `MIGRATION_LOCK_KEY`, conexão dedicada e timeout padrão de 10 s; estouro devolve `LockBusy` código 3.

## DoD

- [ ] Teste com duas conexões prova que a segunda espera e falha com `LockBusy` no timeout.
- [ ] O lock não é `xact` e não expõe `unlock`.
- [ ] A conexão do lock é separada da conexão de aplicação.
- [ ] O código 3 é o único para lock ocupado.

## Fora de escopo

Não mudar o valor da chave, não criar modo alternativo e não definir timeout por ambiente fora de `MigrateConfig`.

## Risco

Matar a sessão dona do lock pode interromper uma migration no meio; o teste não deve simular isso como correção.

## Depende de

WYDDB-012.

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

Teste de concorrência contra PG17 e teste unitário de timeout.

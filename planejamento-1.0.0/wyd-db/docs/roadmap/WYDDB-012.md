# WYDDB-012 — implementar Ledger e MigrationExecutor

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 3 — decomposição de portas e separação de SQL


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

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Teste de contrato com `FakeExecutor` e teste de integração do adapter quando o banco existir.

# WYDDB-012 — implementar Ledger e MigrationExecutor

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 3 — decomposição de portas e separação de SQL
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

## Evidência ao fechar

Teste de contrato com `FakeExecutor` e teste de integração do adapter quando o banco existir.

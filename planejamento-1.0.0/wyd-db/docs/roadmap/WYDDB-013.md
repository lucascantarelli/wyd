# WYDDB-013 — implementar AdvisoryLock e timeout

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 3 — decomposição de portas e separação de SQL
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

## Evidência ao fechar

Teste de concorrência contra PG17 e teste unitário de timeout.

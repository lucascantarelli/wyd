# WYDDB-015 — separar Catalog e Script no provisionamento

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 3 — decomposição de portas e separação de SQL
**Prioridade:** P1

## Contexto

Ler o cluster e aplicar o script são operações diferentes; juntá-las numa porta gorda impede testar o plano sem executar SQL.

## Entregável

Implementar `provision/port/catalog.rs` e `script.rs` e o `PgProvisionAdapter` que implementa as duas, com `Catalog` só leitura e `Script` aplicando o texto de `render()`.

## DoD

- [ ] `plan`/`check` usam `Catalog` e nunca `Script`.
- [ ] `apply` chama `render()` e entrega o texto ao `Script`.
- [ ] O adapter compartilha o pool, mas não expõe `sqlx` às camadas de cima.
- [ ] Teste com `FakeCatalog`/`FakeScript` cobre os quatro comandos.

## Fora de escopo

Não gerar o golden de `roles.sql`, não executar em cluster e não decidir RLS ou `pg_hba.conf`.

## Risco

O adapter pode ser tentado a fazer parse do SQL; `render` é a única fonte do texto.

## Depende de

WYDDB-003, WYDDB-007.

## Evidência ao fechar

Teste de contrato do provisionamento e página `src/provision/port/mod.md`.

# WYDDB-016 — concentrar SQL de repositório em repo/pg.rs

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 3 — decomposição de portas e separação de SQL
**Prioridade:** P2

## Contexto

As operações de repositório estão planejadas em módulos por agregado, mas o SQL precisa de um dono único para não se espalhar por `people`, `characters` e `sessions`.

## Entregável

Definir o contrato de `repo/pg.rs` como ponto de SQL e mover as consultas dos módulos de repositório para lá, sem criar porta nova sem desenho.

## DoD

- [ ] `rg` mostra `sqlx::query` apenas em `repo/pg.rs` e no adapter de migrations/provision.
- [ ] Os módulos `repo/*.rs` chamam funções de `repo/pg.rs`, não montam SQL.
- [ ] Nenhuma assinatura pública muda sem página de `src/repo-*` atualizada.
- [ ] O contrato de cada operação continua na página do módulo, não no `pg.rs`.

## Fora de escopo

Não redesenhar as operações de repositório, não criar cache e não implementar CRUD que não tenha contrato.

## Risco

Concentrar SQL sem contrato pode virar um módulo gigante; a issue limita o escopo ao movimento, não a novas operações.

## Depende de

WYDDB-002, WYDDB-003.

## Evidência ao fechar

Saída do `rg`, diff dos módulos e páginas de `src/repo-*` sincronizadas.

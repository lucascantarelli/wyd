# WYDDB-006 — fechar Command e Report do runner

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 2 — contratos e erros tipados


## Contexto

O runner precisa de uma decisão pura entre aplicar, não fazer nada e parar por drift; o `Report` é o dado que o operador lê no fim.

## Entregável

Implementar `Command`, `DownTo`, `Plan`, `Drift` e `Report` conforme `src/migrate/spec.md` §8 e §12, com `plan()` puro sobre `MigrationSet` × `AppliedMigration`.

## DoD

- [ ] Teste cobre `UpToDate`, `Apply` em ordem e `Drift` com todos os achados.
- [ ] `down --to 0` é `DownTo::Empty`, não `Version(0)`.
- [ ] `Report` contém `applied`, `skipped`, `pending` e `findings`.
- [ ] Nenhum `bool` de modo entra em `Command` além de `check`/`dry_run`.

## Fora de escopo

Não executar SQL, não pegar lock e não montar CLI; a decisão é pura.

## Risco

Drift parcial pode esconder o segundo achado; o teste exige a lista completa.

## Depende de

WYDDB-002.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Teste unitário de `plan` e página `src/migrate/migration/plan.md`.

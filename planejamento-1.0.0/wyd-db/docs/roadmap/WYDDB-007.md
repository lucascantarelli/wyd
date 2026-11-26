# WYDDB-007 — fechar Command e Report do provisionamento

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 2 — contratos e erros tipados


## Contexto

`render`, `plan`, `check` e `apply` respondem perguntas diferentes; misturar as saídas faria o binário comparar texto com drift de papel.

## Entregável

Implementar `Command`, `Plan`, `Step`, `Drift` e `Report` do provisionamento com `ROLES` como estado desejado, sem I/O no domínio.

## DoD

- [ ] `render` é determinístico e não contém senha.
- [ ] `plan` separa `Apply` de `Drift` de `login`.
- [ ] `check` não chama `render()` nem `Script`.
- [ ] `Report::Rendered` carrega texto; o código de saída fica no binário.

## Fora de escopo

Não criar SQL de `GRANT`/`REVOKE`, não aplicar em cluster e não decidir `pg_hba.conf`.

## Risco

O estado desejado pode divergir do DDL de `public`; a issue cita `ADR-DB-0001` e não inventa papel.

## Depende de

WYDDB-003.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Teste de `render_is_stable`/`plan` e página `src/provision/domain/plan.md`.

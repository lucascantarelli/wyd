# WYDDB-026 — documentar diagnóstico de lock e ledger

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 5 — observabilidade e diagnóstico


## Contexto

O diagnóstico de migration travada e ledger divergente precisa de passos que não matem sessão nem editem o ledger; a página existe, mas o runbook ainda não foi validado.

## Entregável

Revisar `guia-administrador/diagnostico.md` contra os códigos 3/4, a chave de lock e o checksum, e registrar a evidência de um ensaio local.

## DoD

- [ ] A página cita `pg_locks` e a chave `MIGRATION_LOCK_KEY` corretamente.
- [ ] Um ensaio local reproduz lock ocupado e drift, com saída registrada.
- [ ] A ação não recomenda `pg_terminate_backend` às cegas nem editar ledger.
- [ ] O que não tem medição (pool, disco) continua declarado como lacuna.

## Fora de escopo

Não definir threshold de pool, disco ou latência; não criar dashboard.

## Risco

O ensaio pode não cobrir a sessão dona real; a evidência precisa dizer o que foi simulado.

## Depende de

WYDDB-013, WYDDB-025.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `documentation-writer` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Ensaio registrado e diff da página de diagnóstico.

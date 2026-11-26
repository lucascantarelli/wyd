# WYDDB-022 — instrumentar spans e resumo do runner

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 5 — observabilidade e diagnóstico


## Contexto

O operador precisa saber qual versão está aplicando e quanto demorou, sem ler o SQL. O runner já define os campos; falta emitir o span e o resumo.

## Entregável

Instrumentar `migration.apply` com `version`, `slug` e `elapsed_ms` e emitir o resumo de `Report` (`applied`, `skipped`, `pending`, `findings`, `elapsed_ms`).

## DoD

- [ ] Teste de log captura um span por arquivo e um resumo final.
- [ ] O log não contém DSN, senha, SQL ou PII.
- [ ] `Report` continua dado; a formatação é do binário.
- [ ] O resumo inclui drift sem duplicar a lista.

## Fora de escopo

Não definir backend de tracing, amostragem, retenção ou dashboard.

## Risco

Instrumentação sem teste de redação pode vazar a DSN que o erro tentou esconder.

## Depende de

WYDDB-008, WYDDB-014.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Teste de captura de log e página `src/migrate/app/runner.md`.

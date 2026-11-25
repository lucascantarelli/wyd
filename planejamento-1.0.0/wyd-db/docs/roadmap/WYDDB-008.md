# WYDDB-008 — mapear MigrateError para códigos de saída

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 2 — contratos e erros tipados

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P1

## Contexto

O binário não pode traduzir erro por `match` solto: a taxonomia e os códigos 0/2/3/4/5 são contrato de operação.

## Entregável

Implementar `MigrateError` e uma função de código de saída fechada, com `Display` que nomeia versão, arquivo ou variável sem imprimir SQL/DSN.

## DoD

- [ ] Teste percorre cada variante e confere o código 2, 3, 4 ou 5.
- [ ] `Drift` deriva do `Plan` já calculado, sem reler disco/ledger.
- [ ] Mensagem de configuração nomeia a variável ausente.
- [ ] Nenhuma variante nova entra sem atualização da tabela de código.

## Fora de escopo

Não criar `anyhow`, `catch-all` ou código novo; não implementar retry.

## Risco

A mensagem do driver pode trazer a URL; o `Display` deve substituir, não repassar.

## Depende de

WYDDB-006.

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

Teste de tabela de códigos e página `src/migrate/error.md`.

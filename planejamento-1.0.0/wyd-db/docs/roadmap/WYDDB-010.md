# WYDDB-010 — provar cadeia de erro sem segredo

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

Erros encadeados são úteis, mas uma mensagem de conexão pode carregar a DSN inteira. O crate precisa de um teste que varra a saída de todas as variantes.

## Entregável

Adicionar teste de redação que percorre `MigrateError` e `ProvisionError`, incluindo `source`, e falha se a saída contiver senha, token, `postgres://` ou texto de SQL.

## DoD

- [ ] Teste usa uma DSN fictícia com senha e confirma ausência do valor em `Display` e `Debug`.
- [ ] Erro de SQL não inclui o `up`/`down` inteiro.
- [ ] Nenhuma mensagem imprime PIN, hash ou token.
- [ ] O teste roda sem banco e sem variável de ambiente.

## Fora de escopo

Não implementar logger, collector ou destino de log; isso é da fase 5.

## Risco

Um `#[derive(Debug)]` novo pode furar a redação; o teste precisa cobrir os tipos públicos.

## Depende de

WYDDB-005, WYDDB-008, WYDDB-009.

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

Teste de redação verde e revisão do checklist de PR.

# WYDDB-005 — implementar Dsn com Debug redigido

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

A DSN carrega a senha e seria impressa por `Debug` herdado. O crate precisa de um tipo único que parseie, valide e redija antes de qualquer conexão.

## Entregável

Implementar `src/dsn.rs` com parse, erro tipado e `Debug` que substitui a senha; nenhum `DATABASE_URL` e nenhuma leitura de ambiente na biblioteca.

## DoD

- [ ] Teste de parse aceita DSN válida e recusa forma inválida com erro específico.
- [ ] `format!("{:?}", dsn)` não contém a senha nem a URL completa.
- [ ] `rg -n "DATABASE_URL" crates/wyd-db/src` não encontra leitura.
- [ ] A página `src/dsn.md` documenta a garantia e o erro.

## Fora de escopo

Não escolher driver, pool ou política de conexão; esta issue é o tipo e a redação.

## Risco

Um erro de parse pode vazar a entrada; o `Display` também precisa ser testado.

## Depende de

WYDDB-001.

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

Testes de parse/redação e saída do `rg` no PR.

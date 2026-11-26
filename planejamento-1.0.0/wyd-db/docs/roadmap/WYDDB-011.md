# WYDDB-011 — implementar Source e FsSource

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 3 — decomposição de portas e separação de SQL

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P1

## Contexto

O runner não deve importar `std::fs`; a leitura da série precisa entrar por uma porta que entregue arquivos carregados e possa ser falsificada.

## Entregável

Implementar `port/source.rs` (`Source`, `LoadedFile`) e `adapter/fs.rs` (`FsSource`), com leitura de diretório sem parser de SQL no adapter.

## DoD

- [ ] `FsSource` entrega os arquivos em ordem de versão e não monta SQL.
- [ ] `FakeSource` no teste devolve a mesma forma e permite testar erro de leitura.
- [ ] `app/` não menciona `std::fs`; `rg` confirma.
- [ ] Teste cobre diretório ausente/ilegível como `SourceError`, não `MigrateError` genérico.

## Fora de escopo

Não escrever parser, checksum ou transação; a issue é a porta e o adapter de leitura.

## Risco

Um adapter que ordena por nome pode divergir do `MigrationSet`; a ordenação final é do modelo puro.

## Depende de

WYDDB-002, WYDDB-006.

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

Teste de `Source` com fixture e saída do `rg` de `std::fs` em `app/`.

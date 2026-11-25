# WYDDB-027 — testar tipos puros de migration

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

Os tipos puros são a primeira barreira contra SQL inválido e versão zero; sem testes de tabela, o parser aceita o que o banco vai recusar depois.

## Entregável

Criar testes unitários para `Version`, `Slug`, `Checksum`, `Sql` e `plan`, com tabela de válidos/inválidos e normalização `CRLF`→`LF`.

## DoD

- [ ] Tabela cobre vazio, zero, nome com acento, `COMMIT`, marcador interno e checksum de um byte.
- [ ] `plan` cobre `UpToDate`, `Apply` e cada `DriftKind`.
- [ ] Nenhum teste lê arquivo, ambiente ou relógio.
- [ ] Os testes rodam sem PostgreSQL.

## Fora de escopo

Não criar fixture de banco, não testar adapter e não medir cobertura percentual.

## Risco

Um teste de tabela grande pode virar lista de casos sem intenção; cada linha precisa dizer o que recusa.

## Depende de

WYDDB-002, WYDDB-006.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída de `cargo test -p wyd-db` e tabela de casos no PR.

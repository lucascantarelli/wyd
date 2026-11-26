# WYDDB-001 — fixar lint e documentação pública do crate

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 1 — higiene e baseline

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P1

## Contexto

O crate ainda não existe, mas o contrato de qualidade precisa nascer junto com ele. Sem `missing_docs` e sem baseline de lint, cada arquivo novo entra sem contrato e a revisão vira opinião.

## Entregável

Habilitar `#![deny(missing_docs)]` em `crates/wyd-db/src/lib.rs`, documentar os itens públicos e fixar o baseline de `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` no guia do desenvolvedor.

## DoD

- [ ] `cargo doc -p wyd-db --no-deps` termina sem warning de documentação faltante.
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` passam no commit de referência.
- [ ] `guia-desenvolvedor/index.md` §3 deixa de dizer que os três comandos não existem.
- [ ] Nenhum `unsafe` novo entra sem bloco de segurança justificado.

## Fora de escopo

Não criar CI real, não definir versão mínima de Rust e não documentar módulo que ainda não tem contrato.

## Risco

O baseline pode virar gate decorativo se só rodar no PR inicial; o checklist de PR precisa cobrar a repetição.

## Depende de

Nenhuma; é a primeira issue da fase 1.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `documentation-writer` |
| `skills_complementares` | `writing-plans` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída dos três comandos, diff do `lib.rs` e página do guia atualizada no mesmo PR.

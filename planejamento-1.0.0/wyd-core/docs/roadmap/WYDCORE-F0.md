# WYDCORE-F0 — Fundação do crate

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-core`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-CORE-001`, `RF-CORE-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F0 — Fundação do crate
**Fase:** 1

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F0 do `wyd-core` cobre `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline fmt/clippy e existe para
cumprir `RF-CORE-001`, `RF-CORE-002` sem antecipar a fatia seguinte.

## Entregável

`Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline fmt/clippy

## DoD

- [ ] `cargo doc` sem warning.
- [ ] fmt/clippy verdes.

## Fora de escopo

CI real

## Depende de

crate existente

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

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

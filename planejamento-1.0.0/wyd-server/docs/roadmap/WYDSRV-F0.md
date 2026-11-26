# WYDSRV-F0 — Fundação do crate

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-server`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-SRV-001`, `RF-SRV-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Fundação do crate
**Fase:** 1


## Contexto

A fatia F0 do `wyd-server` cobre `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline fmt/clippy e existe para
cumprir `RF-SRV-001`, `RF-SRV-002` sem antecipar a fatia seguinte.

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

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

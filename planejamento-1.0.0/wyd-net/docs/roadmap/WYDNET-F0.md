# WYDNET-F0 — Fundação do crate (baseline)

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-net`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-NET-001` a `RF-NET-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26)
**Aceite:** o dono autorizou a execução e o aceite desta página em 2026-11-26, nesta sessão; a evidência está em §Evidência de execução.

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Fundação do crate (baseline)
**Fase:** 1


## Contexto

A fatia F0 do `wyd-net` cobre `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline de `cargo fmt`/`clippy` e existe para
cumprir `RF-NET-001` a `RF-NET-002` sem antecipar a fatia seguinte.

## Entregável

`Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline de `cargo fmt`/`clippy`

## DoD

- [x] `cargo doc -p wyd-net --no-deps` sem warning.
- [x] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` verdes.

## Fora de escopo

CI real, versão mínima de Rust

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

## Evidência de execução (2026-11-26)

Executado em `crates/wyd-net` com Rust 1.100.0:

```text
$ cargo doc -p wyd-net --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s

$ cargo fmt --check
(sem saída; exit 0)

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
```

`lib.rs` recebeu `#![deny(missing_docs)]` sobre uma árvore que já tinha `Cargo.toml`, `src/lib.rs` e os módulos reservados; `cargo test --lib` → 0 passed (esqueleto, sem teste ainda).

//! Crate do `wyd-db`: runner de migrations e provisionamento de papéis do WYD 1.0.0.
//!
//! Só existe aqui o que tem página `aceita` no planejamento — invariante §5.1 de
//! `AGENTS.md`. Hoje: `dsn` (página `src/dsn.md`, aceita em 2026-11-24),
//! `migrate` e a fachada de `provision` (páginas aceitas em 2026-11-24);
//! a árvore completa está em `docs/guia-desenvolvedor/mapa-do-crate.md`.

#![deny(missing_docs)]

pub mod dsn;
pub mod migrate;
pub mod provision;

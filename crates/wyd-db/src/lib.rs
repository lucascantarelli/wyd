//! Crate do `wyd-db`: runner de migrations e provisionamento de papéis do WYD 1.0.0.
//!
//! Só existe aqui o que tem página `aceita` no planejamento — invariante §5.1 de
//! `AGENTS.md`. Neste primeiro slice, `dsn` (página `src/dsn.md`, aceita em 2026-11-24);
//! a árvore completa está em `docs/guia-desenvolvedor/mapa-do-crate.md`.

pub mod dsn;
pub mod migrate;

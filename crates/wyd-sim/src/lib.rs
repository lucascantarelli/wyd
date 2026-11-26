//! `wyd-sim` — fórmulas puras: dano, acerto, drop, economia e movimento.
//!
//! Estrutura inicial criada em 2026-11-26. A fronteira é do plano em
//! `planejamento-1.0.0/wyd-sim/`; nenhuma fórmula foi escrita e nenhuma fixture
//! existe. Os módulos são reservas do plano e ganham código com a página dona aceita
//! (`AGENTS.md` §5, item 1).

#![forbid(unsafe_code)]

pub mod accuracy;
pub mod arithmetic;
pub mod damage;
pub mod drop;
pub mod economy;
pub mod fixtures;
pub mod movement;
pub mod version;

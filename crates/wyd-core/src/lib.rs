//! `wyd-core` — tick, entidades, cena, combate, IA e persistência.
//!
//! Estrutura inicial criada em 2026-11-26. A fronteira é do plano em
//! `planejamento-1.0.0/wyd-core/`; nenhuma regra de jogo está implementada. Os
//! módulos são reservas do plano e ganham código com a página dona aceita
//! (`AGENTS.md` §5, item 1).

#![forbid(unsafe_code)]

pub mod ai;
pub mod combat;
pub mod entities;
pub mod events;
pub mod observability;
pub mod persistence;
pub mod scene;
pub mod tick;
pub mod validation;

//! `wyd-bots` — bots de QA de fio, carga e convivência.
//!
//! Estrutura inicial criada em 2026-11-26. A fronteira é do plano em
//! `planejamento-1.0.0/wyd-bots/`; nenhum bot ou cenário está implementado, e não há
//! binário enquanto o protocolo do cliente não existir. Os módulos são reservas do plano
//! e ganham código com a página dona aceita (`AGENTS.md` §5, item 1).

#![forbid(unsafe_code)]

pub mod coexistence_bot;
pub mod config;
pub mod environment;
pub mod load_bot;
pub mod qa_bot;
pub mod reporting;
pub mod scenario_runner;

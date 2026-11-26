//! `wyd-server` — composição de canal, health, controle e captura (ECAP).
//!
//! Estrutura inicial criada em 2026-11-26. A fronteira é do plano em
//! `planejamento-1.0.0/wyd-server/` e da spec do endpoint de captura. O binário desta
//! entrega sobe apenas as sondas de processo; `EPROD`, o `ECAP` e o handshake não estão
//! implementados. Os módulos são reservas do plano e ganham código com a página dona
//! aceita (`AGENTS.md` §5, item 1).

#![forbid(unsafe_code)]

pub mod auth;
pub mod capture;
pub mod channel;
pub mod composition;
pub mod config;
pub mod control;
pub mod health;
pub mod lifecycle;
pub mod metrics;
pub mod observability;

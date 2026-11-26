//! `wyd-net` — transporte, sessão, roteamento, validação e backpressure.
//!
//! Estrutura inicial criada em 2026-11-26. A fronteira é do plano em
//! `planejamento-1.0.0/wyd-net/`; nenhum listener, timeout ou limite por IP está
//! implementado. Os módulos são reservas do plano e ganham código com a página dona
//! aceita (`AGENTS.md` §5, item 1).

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod backpressure;
pub mod error;
pub mod observability;
pub mod protocol_bridge;
pub mod reconnect;
pub mod routing;
pub mod session;
pub mod transport;
pub mod validation;

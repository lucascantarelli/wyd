//! `wyd-admin` — administração por HTTP/JSON interno (contrato C4).
//!
//! Estrutura inicial criada em 2026-11-26. A fronteira é do plano em
//! `planejamento-1.0.0/wyd-admin/`; o contrato C4, a autenticação de serviço, a
//! idempotência e as ações continuam não implementados. O binário desta entrega sobe
//! apenas as sondas de processo. Os módulos são reservas do plano e ganham código com a
//! página dona aceita (`AGENTS.md` §5, item 1).

#![forbid(unsafe_code)]

pub mod acoes;
pub mod auth;
pub mod banco;
pub mod http;
pub mod idempotencia;
pub mod server;

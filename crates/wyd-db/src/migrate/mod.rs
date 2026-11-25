//! Runner de migrations da 1.0.0 — fachada pública.
//!
//! O contrato completo está em `planejamento-1.0.0/wyd-db/src/migrate/spec.md`.
//! Nenhuma leitura de ambiente ou construção de adapter mora aqui: o binário
//! monta [`config::MigrateConfig`] e injeta as portas.

pub mod adapter;
pub mod app;
pub mod config;
pub mod error;
pub mod migration;
pub mod port;

use error::MigrateError;
use migration::plan::Drift;
use migration::version::Version;
use port::{AdvisoryLock, Ledger, MigrationExecutor, Source};

/// Comando aceito pelo runner.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Status { check: bool },
    Apply { dry_run: bool },
    Down { to: DownTo },
}

/// Alvo do `down`; `Empty` é o reset de desenvolvimento.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownTo {
    Empty,
    Version(Version),
}

/// Resultado de uma execução.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub applied: Vec<Version>,
    pub skipped: usize,
    pub pending: usize,
    pub findings: Vec<Drift>,
}

/// Fachada do runner; tudo é injetado.
pub async fn run<B, S>(command: Command, backend: &B, source: &S) -> Result<Report, MigrateError>
where
    B: Ledger + MigrationExecutor + AdvisoryLock,
    S: Source,
{
    app::runner::run(command, backend, source).await
}

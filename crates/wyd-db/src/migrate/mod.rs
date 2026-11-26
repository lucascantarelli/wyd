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
    /// Relata o estado da série; com `check`, drift vira falha.
    Status {
        /// Se `true`, qualquer drift faz o comando terminar em erro.
        check: bool,
    },
    /// Aplica as migrations pendentes; com `dry_run`, só relata.
    Apply {
        /// Se `true`, não executa SQL nem grava no ledger.
        dry_run: bool,
    },
    /// Reverte migrations até o alvo indicado.
    Down {
        /// Onde a reversão deve parar.
        to: DownTo,
    },
}

/// Alvo do `down`; `Empty` é o reset de desenvolvimento.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownTo {
    /// Remove todas as migrations aplicadas (reset de desenvolvimento).
    Empty,
    /// Para na versão indicada, inclusive.
    Version(Version),
}

/// Resultado de uma execução.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Versões aplicadas nesta execução, em ordem crescente.
    pub applied: Vec<Version>,
    /// Quantas já estavam aplicadas e foram puladas.
    pub skipped: usize,
    /// Quantas continuam pendentes ao fim da execução.
    pub pending: usize,
    /// Drifts encontrados, em ordem crescente de versão.
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

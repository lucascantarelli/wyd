//! Porta somente leitura do ledger.

use crate::migrate::migration::applied::AppliedMigration;
use thiserror::Error;

/// Consulta o que já foi aplicado.
pub trait Ledger: Send + Sync {
    /// Devolve as linhas aplicadas; tabela ausente é `Ok(vec![])`.
    async fn applied(&self) -> Result<Vec<AppliedMigration>, LedgerError>;
}

/// Falha de leitura ou de forma do ledger.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LedgerError {
    #[error("ledger com forma divergente: {detail}")]
    Shape { detail: String },
    #[error("falha ao consultar o ledger: {detail}")]
    Query { detail: String },
}

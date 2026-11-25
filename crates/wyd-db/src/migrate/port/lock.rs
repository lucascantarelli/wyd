//! Porta do advisory lock de sessão.

use std::time::Duration;
use thiserror::Error;

/// Chave estável do lock da série 1.0.0; não reutiliza a chave herdada.
pub const MIGRATION_LOCK_KEY: i64 = 0x5759_4404;

/// Serializa a aplicação entre processos.
pub trait AdvisoryLock: Send + Sync {
    /// Espera até `timeout`; estourou → [`LockError::Busy`].
    async fn acquire(&self, timeout: Duration) -> Result<(), LockError>;
}

/// Falha ao obter o lock.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LockError {
    #[error("outro processo está migrando (esperei {waited:?}); verifique pg_locks")]
    Busy { waited: Duration },
    #[error("falha ao consultar o lock: {detail}")]
    Query { detail: String },
}

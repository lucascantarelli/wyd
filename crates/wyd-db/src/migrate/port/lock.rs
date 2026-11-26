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
    /// O lock continua com outro processo depois do timeout.
    #[error("outro processo está migrando (esperei {waited:?}); verifique pg_locks")]
    Busy {
        /// Tempo esperado antes de desistir.
        waited: Duration,
    },
    /// A consulta ao advisory lock falhou.
    #[error("falha ao consultar o lock: {detail}")]
    Query {
        /// Detalhe do erro do driver.
        detail: String,
    },
}

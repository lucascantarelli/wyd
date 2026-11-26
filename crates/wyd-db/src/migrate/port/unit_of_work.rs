//! Porta de execução transacional e do ledger.

use crate::migrate::migration::checksum::Checksum;
use crate::migrate::migration::file::RawMigration;
use crate::migrate::migration::sql::Sql;
use crate::migrate::migration::version::Version;
use crate::migrate::port::ledger::LedgerError;
use std::fmt;

/// Retrato mínimo da sessão, lido antes de qualquer `BEGIN`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerInfo {
    /// Papel conectado, como o PostgreSQL o reporta.
    pub current_user: String,
    /// Versão numérica do servidor (`server_version_num`).
    pub server_version_num: i32,
}

/// Unidade de trabalho de um arquivo. `commit` consome a transação.
pub trait MigrationTxn: Send {
    /// Executa o SQL de uma migration dentro da transação.
    async fn run(&mut self, sql: &Sql) -> Result<(), SqlError>;
    /// Registra a migration aplicada no ledger.
    async fn record(
        &mut self,
        migration: &RawMigration,
        checksum: Checksum,
    ) -> Result<(), SqlError>;
    /// Remove do ledger a versão revertida.
    async fn forget(&mut self, version: Version) -> Result<(), SqlError>;
    /// Confirma a transação; consome a unidade de trabalho.
    async fn commit(self) -> Result<(), SqlError>;
}

/// Bootstrap do ledger, fábrica de transação e preflight.
pub trait MigrationExecutor: Send + Sync {
    /// Transação concreta usada pelo executor.
    type Txn: MigrationTxn;

    /// Garante o bootstrap do ledger antes da primeira transação.
    async fn prepare(&self) -> Result<(), LedgerError>;
    /// Abre uma nova unidade de trabalho.
    async fn begin(&self) -> Result<Self::Txn, SqlError>;
    /// Lê o retrato da sessão antes de qualquer `BEGIN`.
    async fn server_info(&self) -> Result<ServerInfo, SqlError>;
    /// Se a operação aceita um papel com privilégio de superusuário.
    fn allows_superuser(&self) -> bool;
}

/// Erro de SQL de uma migration, sem expor o driver.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlError {
    /// Erro devolvido pelo servidor, sem posição no texto.
    Server {
        /// Código SQLSTATE, quando disponível.
        sqlstate: Option<String>,
        /// Mensagem do servidor.
        message: String,
    },
    /// Erro com posição no texto SQL.
    Position {
        /// Código SQLSTATE, quando disponível.
        sqlstate: Option<String>,
        /// Mensagem do servidor.
        message: String,
        /// Posição em bytes, quando o servidor a informa.
        position: Option<u32>,
    },
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Server { sqlstate, message } => {
                if let Some(code) = sqlstate {
                    write!(f, "{code}: {message}")
                } else {
                    f.write_str(message)
                }
            }
            Self::Position {
                sqlstate,
                message,
                position,
            } => {
                if let Some(code) = sqlstate {
                    write!(f, "{code}: {message}")?;
                } else {
                    f.write_str(message)?;
                }
                if let Some(position) = position {
                    write!(f, " (posição {position})")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SqlError {}

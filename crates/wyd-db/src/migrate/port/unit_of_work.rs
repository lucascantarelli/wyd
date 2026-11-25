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
    pub current_user: String,
    pub server_version_num: i32,
}

/// Unidade de trabalho de um arquivo. `commit` consome a transação.
pub trait MigrationTxn: Send {
    async fn run(&mut self, sql: &Sql) -> Result<(), SqlError>;
    async fn record(
        &mut self,
        migration: &RawMigration,
        checksum: Checksum,
    ) -> Result<(), SqlError>;
    async fn forget(&mut self, version: Version) -> Result<(), SqlError>;
    async fn commit(self) -> Result<(), SqlError>;
}

/// Bootstrap do ledger, fábrica de transação e preflight.
pub trait MigrationExecutor: Send + Sync {
    type Txn: MigrationTxn;

    async fn prepare(&self) -> Result<(), LedgerError>;
    async fn begin(&self) -> Result<Self::Txn, SqlError>;
    async fn server_info(&self) -> Result<ServerInfo, SqlError>;
    fn allows_superuser(&self) -> bool;
}

/// Erro de SQL de uma migration, sem expor o driver.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlError {
    Server {
        sqlstate: Option<String>,
        message: String,
    },
    Position {
        sqlstate: Option<String>,
        message: String,
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

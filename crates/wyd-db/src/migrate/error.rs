//! Erro único do runner e mapa de códigos de saída.

use crate::migrate::migration::plan::{Drift, DriftKind};
use crate::migrate::migration::set::SetError;
use crate::migrate::migration::version::Version;
use crate::migrate::port::ledger::LedgerError;
use crate::migrate::port::source::SourceError;
use crate::migrate::port::unit_of_work::SqlError;
use std::fmt;
use std::time::Duration;
use thiserror::Error;

/// Resultado do runner.
pub type Result<T> = std::result::Result<T, MigrateError>;

/// Falha do runner.
#[non_exhaustive]
#[derive(Debug)]
pub enum MigrateError {
    Config(ConfigError),
    Connect(ConnectError),
    Source(SourceError),
    WrongRole {
        found: String,
        expected: &'static str,
    },
    WrongServer {
        found: String,
        expected: u8,
    },
    SetInvalid(SetError),
    LockBusy {
        waited: Duration,
    },
    Drift(Vec<Drift>),
    Ledger(LedgerError),
    Sql {
        version: Version,
        source: SqlError,
    },
}

impl MigrateError {
    /// Código de saída do processo para esta falha.
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Config(_)
            | Self::Connect(_)
            | Self::Source(_)
            | Self::WrongRole { .. }
            | Self::WrongServer { .. }
            | Self::Ledger(_) => 2,
            Self::LockBusy { .. } => 3,
            Self::Drift(_) | Self::SetInvalid(_) => 4,
            Self::Sql { .. } => 5,
        }
    }
}

impl fmt::Display for MigrateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(source) => write!(f, "configuração: {source}"),
            Self::Connect(source) => write!(f, "conexão: {source}"),
            Self::Source(source) => write!(f, "série: {source}"),
            Self::WrongRole { found, expected } => {
                write!(f, "esperado {expected}, encontrado {found}")
            }
            Self::WrongServer { found, expected } => {
                write!(f, "esperado PostgreSQL {expected}, encontrado {found}")
            }
            Self::SetInvalid(source) => write!(f, "série inválida: {source}"),
            Self::LockBusy { waited } => write!(
                f,
                "outro processo está migrando (esperei {waited:?}); verifique pg_locks"
            ),
            Self::Drift(findings) => {
                f.write_str("drift entre disco e ledger:")?;
                for drift in findings {
                    write!(
                        f,
                        "\n  {} {}",
                        drift.version,
                        match drift.kind {
                            DriftKind::Checksum => "checksum",
                            DriftKind::Slug => "slug",
                            DriftKind::MissingFile => "arquivo ausente",
                        }
                    )?;
                }
                Ok(())
            }
            Self::Ledger(source) => write!(f, "ledger: {source}"),
            Self::Sql { version, source } => write!(f, "arquivo {version}: {source}"),
        }
    }
}

impl std::error::Error for MigrateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Config(source) => Some(source),
            Self::Connect(source) => Some(source),
            Self::Source(source) => Some(source),
            Self::SetInvalid(source) => Some(source),
            Self::Ledger(source) => Some(source),
            Self::Sql { source, .. } => Some(source),
            Self::WrongRole { .. }
            | Self::WrongServer { .. }
            | Self::LockBusy { .. }
            | Self::Drift(_) => None,
        }
    }
}

/// Configuração inválida.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigError {
    #[error("lock_timeout fora da faixa: {given:?}")]
    TimeoutOutOfRange { given: Duration },
    #[error("variável de ambiente ausente: {name}")]
    MissingEnv { name: &'static str },
}

/// Falha de conexão com o PostgreSQL, já sem DSN.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConnectError {
    #[error("não conectou: {detail}")]
    Refused { detail: String },
    #[error("não abriu o pool: {detail}")]
    Pool { detail: String },
}

impl From<ConfigError> for MigrateError {
    fn from(source: ConfigError) -> Self {
        Self::Config(source)
    }
}

impl From<ConnectError> for MigrateError {
    fn from(source: ConnectError) -> Self {
        Self::Connect(source)
    }
}

impl From<SourceError> for MigrateError {
    fn from(source: SourceError) -> Self {
        Self::Source(source)
    }
}

impl From<SetError> for MigrateError {
    fn from(source: SetError) -> Self {
        Self::SetInvalid(source)
    }
}

impl From<LedgerError> for MigrateError {
    fn from(source: LedgerError) -> Self {
        Self::Ledger(source)
    }
}

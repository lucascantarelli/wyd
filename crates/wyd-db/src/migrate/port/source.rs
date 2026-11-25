//! Porta de leitura da série.

use std::path::PathBuf;
use thiserror::Error;

/// De onde o runner recebe os arquivos.
pub trait Source: Send + Sync {
    /// Lê os arquivos da série; não ordena, não valida e não parseia.
    async fn load(&self) -> Result<Vec<LoadedFile>, SourceError>;
}

/// Um arquivo lido do disco, ainda cru.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedFile {
    pub path: PathBuf,
    pub content: String,
}

/// Falha de ambiente ao ler a série.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SourceError {
    #[error("diretório da série não existe: {path}")]
    Missing { path: PathBuf },
    #[error("caminho da série não é diretório: {path}")]
    NotADirectory { path: PathBuf },
    #[error("falha ao ler {path}: {detail}")]
    Read { path: PathBuf, detail: String },
}

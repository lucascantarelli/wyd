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
    /// Caminho de origem do arquivo.
    pub path: PathBuf,
    /// Conteúdo cru, sem parse nem validação.
    pub content: String,
}

/// Falha de ambiente ao ler a série.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SourceError {
    /// O diretório configurado não existe.
    #[error("diretório da série não existe: {path}")]
    Missing {
        /// Caminho configurado.
        path: PathBuf,
    },
    /// O caminho configurado existe, mas não é diretório.
    #[error("caminho da série não é diretório: {path}")]
    NotADirectory {
        /// Caminho configurado.
        path: PathBuf,
    },
    /// Um arquivo da série não pôde ser lido.
    #[error("falha ao ler {path}: {detail}")]
    Read {
        /// Arquivo que falhou.
        path: PathBuf,
        /// Detalhe do erro de leitura.
        detail: String,
    },
}

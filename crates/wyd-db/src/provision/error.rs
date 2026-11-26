//! Erro único do provisionamento.

use crate::provision::domain::plan::Drift;
use crate::provision::port::{CatalogError, ScriptError};
use thiserror::Error;

/// Resultado do provisionamento.
pub type Result<T> = std::result::Result<T, ProvisionError>;

/// Falha do provisionamento.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ProvisionError {
    /// Falha ao ler o cluster.
    #[error("catálogo: {0}")]
    Catalog(#[from] CatalogError),
    /// Falha ao aplicar o script.
    #[error("script: {0}")]
    Script(#[from] ScriptError),
    /// Divergência que `apply` não corrige.
    #[error("drift de papéis: {0:?}")]
    Drift(Vec<Drift>),
}

impl ProvisionError {
    /// Código de saída do processo para esta falha; `3` é do `render --check` no binário.
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Drift(_) => 4,
            Self::Catalog(_) | Self::Script(_) => 2,
        }
    }
}

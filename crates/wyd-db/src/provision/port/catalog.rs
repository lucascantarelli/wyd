//! Porta de leitura do cluster.

use crate::provision::domain::observed::ObservedRole;
use thiserror::Error;

/// Leitura de `pg_roles` e `pg_auth_members`. Nunca escreve.
pub trait Catalog: Send + Sync {
    /// Devolve o retrato dos papéis; papel extra é ignorado pelo plano.
    async fn observed(&self) -> Result<Vec<ObservedRole>, CatalogError>;
}

/// Falha ao ler o cluster.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CatalogError {
    /// A conexão com o cluster falhou.
    #[error("não conectou ao cluster: {detail}")]
    Connect {
        /// Detalhe do erro, sem DSN.
        detail: String,
    },
    /// A consulta a `pg_roles`/`pg_auth_members` falhou.
    #[error("falha ao ler pg_roles: {detail}")]
    Query {
        /// Detalhe do erro, sem DSN.
        detail: String,
    },
}

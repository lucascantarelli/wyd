//! Porta de aplicação do texto de `render()`.

use thiserror::Error;

/// Aplica o script de papéis numa sessão, sem lock e sem transação explícita.
pub trait Script: Send + Sync {
    /// Envia o texto inteiro; `DO`/`GRANT` são idempotentes.
    async fn apply(&self, sql: &str) -> Result<(), ScriptError>;
}

/// Falha ao aplicar o script.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScriptError {
    /// A conexão com o cluster falhou.
    #[error("não conectou ao cluster: {detail}")]
    Connect {
        /// Detalhe do erro, sem DSN.
        detail: String,
    },
    /// O servidor recusou o texto.
    #[error("falha ao aplicar roles.sql: {detail}")]
    Query {
        /// Detalhe do erro, sem DSN.
        detail: String,
        /// Offset do erro no texto, quando o servidor informa.
        position: Option<u32>,
    },
}

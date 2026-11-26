//! Provisionamento de papéis do cluster e do schema `public` — fachada.
//!
//! Contrato completo em `planejamento-1.0.0/wyd-db/src/provision/spec.md`.
//! O adapter PostgreSQL entra em `WYDDB-015`; o golden e o CLI ficam fora desta
//! fachada.

pub mod app;
pub mod domain;
pub mod error;
pub mod port;

pub use app::runner::run;
pub use domain::plan::{Drift, Plan, Step};
pub use domain::spec::{memberships, render, RoleSpec, MIGRATOR, OWNER, ROLES};
pub use error::{ProvisionError, Result};

/// Comando aceito pelo provisionamento.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Devolve o texto de `db/roles.sql`; o binário escreve e compara.
    Render,
    /// Compara o cluster com o estado desejado.
    Plan,
    /// Aplica o texto de `render()` no cluster.
    Apply,
    /// Mesmo plano de [`Command::Plan`], sem aplicar; o binário define o código 4.
    Check,
}

/// Resultado de uma execução.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Report {
    /// O texto gerado; nenhum arquivo foi escrito.
    Rendered(String),
    /// O plano de `plan` e de `check`.
    Planned(Plan),
    /// Quantos passos o plano tinha quando o script foi aplicado.
    Applied {
        /// Passos que faltavam antes da aplicação.
        steps: usize,
    },
}

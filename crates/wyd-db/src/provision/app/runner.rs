//! Orquestração do provisionamento: render | plan | apply | check.

use crate::provision::domain::plan::{self, Plan};
use crate::provision::domain::spec::{render, ROLES};
use crate::provision::error::{ProvisionError, Result};
use crate::provision::port::{Catalog, Script};
use crate::provision::{Command, Report};

/// Executa um comando do provisionamento.
///
/// `render` não toca o backend; `plan` lê o catálogo; `check` falha com drift;
/// `apply` bloqueia com drift, não executa plano vazio e envia `render()` uma vez.
pub async fn run<B>(command: Command, backend: &B) -> Result<Report>
where
    B: Catalog + Script,
{
    match command {
        Command::Render => Ok(Report::Rendered(render())),
        Command::Plan => {
            let observed = backend.observed().await?;
            Ok(Report::Planned(plan::plan(ROLES, &observed)))
        }
        Command::Check => {
            let observed = backend.observed().await?;
            let plano = plan::plan(ROLES, &observed);
            if plano.has_drift() {
                return Err(ProvisionError::Drift(plano.drifts));
            }
            Ok(Report::Planned(plano))
        }
        Command::Apply => {
            let observed = backend.observed().await?;
            let Plan { steps, drifts } = plan::plan(ROLES, &observed);
            if !drifts.is_empty() {
                return Err(ProvisionError::Drift(drifts));
            }
            if steps.is_empty() {
                return Ok(Report::Applied { steps: 0 });
            }
            let steps = steps.len();
            backend.apply(&render()).await?;
            Ok(Report::Applied { steps })
        }
    }
}

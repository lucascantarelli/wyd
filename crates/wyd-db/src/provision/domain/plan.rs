//! Decisão entre o cluster observado e o estado desejado.

use std::collections::HashMap;

use super::observed::ObservedRole;
use super::spec::RoleSpec;

/// Um passo que `apply` pode executar.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Cria o papel com o atributo desejado.
    CreateRole {
        /// Nome do papel.
        name: &'static str,
        /// Atributo `LOGIN` desejado.
        login: bool,
    },
    /// Concede a membership direta que falta.
    GrantMembership {
        /// Papel dono.
        owner: &'static str,
        /// Papel membro.
        member: &'static str,
    },
}

/// Divergência que `apply` não corrige.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftKind {
    /// `LOGIN`/`NOLOGIN` diferente do desejado.
    Login {
        /// Atributo desejado.
        expected: bool,
        /// Atributo encontrado.
        found: bool,
    },
    /// Membership desejada que o `assert!` de `spec.rs` já impede; reservada ao `match`.
    MissingMembership {
        /// Papel dono ausente.
        owner: &'static str,
    },
}

/// Uma divergência de papel.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drift {
    /// Papel cujo estado diverge.
    pub role: &'static str,
    /// Tipo da divergência.
    pub kind: DriftKind,
}

/// O que fazer com o cluster observado.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// Passos executáveis: todas as criações antes dos grants.
    pub steps: Vec<Step>,
    /// Divergências que `apply` não corrige.
    pub drifts: Vec<Drift>,
}

impl Plan {
    /// `true` quando não há passo a executar.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// `true` quando alguma divergência impede `apply`.
    #[must_use]
    pub fn has_drift(&self) -> bool {
        !self.drifts.is_empty()
    }
}

/// Compara o estado desejado com o observado. Papel extra é ignorado.
#[must_use]
pub fn plan(desired: &[RoleSpec], observed: &[ObservedRole]) -> Plan {
    let by_name: HashMap<&str, &ObservedRole> = observed
        .iter()
        .map(|role| (role.name.as_str(), role))
        .collect();
    let mut steps = Vec::new();
    let mut drifts = Vec::new();

    for expected in desired {
        match by_name.get(expected.name) {
            None => steps.push(Step::CreateRole {
                name: expected.name,
                login: expected.login,
            }),
            Some(found) => {
                if found.login != expected.login {
                    drifts.push(Drift {
                        role: expected.name,
                        kind: DriftKind::Login {
                            expected: expected.login,
                            found: found.login,
                        },
                    });
                }
            }
        }
    }

    for expected in desired {
        let found = by_name
            .get(expected.name)
            .map(|role| role.member_of.as_slice());
        for owner in expected.member_of {
            let already_member =
                found.is_some_and(|members| members.iter().any(|name| name == owner));
            if !already_member {
                steps.push(Step::GrantMembership {
                    owner,
                    member: expected.name,
                });
            }
        }
    }

    Plan { steps, drifts }
}

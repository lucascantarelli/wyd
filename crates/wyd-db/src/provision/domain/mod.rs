//! Modelo puro do provisionamento: estado desejado, observado e plano.

pub mod observed;
pub mod plan;
pub mod spec;

pub use observed::ObservedRole;
pub use plan::{plan, Drift, Plan, Step};
pub use spec::{memberships, render, RoleSpec, MIGRATOR, OWNER, ROLES};

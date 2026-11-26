//! Retrato do cluster lido pelo `Catalog`, sem I/O.

/// O que o `Catalog` encontrou em `pg_roles` e `pg_auth_members`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedRole {
    /// Nome do papel.
    pub name: String,
    /// Atributo `LOGIN` lido do cluster.
    pub login: bool,
    /// Membership direta encontrada.
    pub member_of: Vec<String>,
}

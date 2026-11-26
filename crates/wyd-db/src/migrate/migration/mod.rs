//! Camada pura do runner: versões, checksum, SQL, arquivo, série e plano.

pub mod applied;
pub mod checksum;
pub mod file;
pub mod plan;
pub mod set;
pub mod sql;
pub mod version;

pub use applied::AppliedMigration;
pub use checksum::{Checksum, ChecksumError};
pub use file::{parse, FileError, RawMigration};
pub use plan::{plan, Drift, DriftKind, Plan};
pub use set::{MigrationSet, SetError};
pub use sql::{Sql, SqlTextError};
pub use version::{Slug, SlugError, Version, VersionError};

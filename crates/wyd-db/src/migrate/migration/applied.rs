//! Linha aplicada do ledger, sem banco e sem data.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/applied.md`.

use crate::migrate::migration::checksum::Checksum;
use crate::migrate::migration::version::{Slug, Version};

/// O que o ledger devolve por linha.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedMigration {
    pub version: Version,
    pub slug: Slug,
    pub checksum: Checksum,
}

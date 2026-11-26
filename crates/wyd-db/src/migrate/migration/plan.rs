//! Decisão pura entre aplicar, nada fazer ou parar por drift.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/plan.md`.

use crate::migrate::migration::applied::AppliedMigration;
use crate::migrate::migration::set::MigrationSet;
use crate::migrate::migration::version::Version;
use std::collections::HashMap;

/// O que o runner deve fazer.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// Disco e ledger coincidem; nada a fazer.
    UpToDate,
    /// Pendentes, em ordem crescente.
    Apply(Vec<Version>),
    /// Todos os achados, em ordem crescente de versão.
    Drift(Vec<Drift>),
}

/// Uma divergência entre disco e ledger.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drift {
    /// Versão afetada pela divergência.
    pub version: Version,
    /// Causa da divergência.
    pub kind: DriftKind,
}

/// As três causas de drift.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftKind {
    /// O conteúdo do arquivo mudou depois de aplicado.
    Checksum,
    /// O slug do arquivo mudou depois de aplicado.
    Slug,
    /// O ledger tem uma versão que não existe mais no disco.
    MissingFile,
}

/// Compara a série do disco com as linhas aplicadas.
#[must_use]
pub fn plan(set: &MigrationSet, applied: &[AppliedMigration]) -> Plan {
    let por_versao: HashMap<Version, &AppliedMigration> = applied
        .iter()
        .map(|migration| (migration.version, migration))
        .collect();
    let mut pendentes = Vec::new();
    let mut findings = Vec::new();

    for entry in set.iter() {
        match por_versao.get(&entry.raw.version) {
            None => pendentes.push(entry.raw.version),
            Some(aplicada) if aplicada.checksum != entry.checksum => {
                findings.push(Drift {
                    version: entry.raw.version,
                    kind: DriftKind::Checksum,
                });
            }
            Some(aplicada) if aplicada.slug != entry.raw.slug => {
                findings.push(Drift {
                    version: entry.raw.version,
                    kind: DriftKind::Slug,
                });
            }
            Some(_) => {}
        }
    }

    for aplicada in applied {
        if set.get(aplicada.version).is_none() {
            findings.push(Drift {
                version: aplicada.version,
                kind: DriftKind::MissingFile,
            });
        }
    }

    findings.sort_by_key(|drift| drift.version);
    if !findings.is_empty() {
        Plan::Drift(findings)
    } else if !pendentes.is_empty() {
        Plan::Apply(pendentes)
    } else {
        Plan::UpToDate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrate::migration::checksum::Checksum;
    use crate::migrate::migration::set::MigrationSet;
    use crate::migrate::migration::version::{Slug, Version};

    fn set() -> MigrationSet {
        let sql = "-- migrate:up\nSELECT 1;\n-- migrate:down\nSELECT 2;\n";
        MigrationSet::build(vec![
            ("0001_first".into(), sql.into()),
            ("0002_second".into(), sql.into()),
            ("0003_third".into(), sql.into()),
        ])
        .expect("set")
    }

    fn applied(version: u32, slug: &str, content: &[u8]) -> AppliedMigration {
        AppliedMigration {
            version: Version::new(version).expect("version"),
            slug: Slug::new(slug).expect("slug"),
            checksum: Checksum::of(content),
        }
    }

    const SQL: &[u8] = b"-- migrate:up\nSELECT 1;\n-- migrate:down\nSELECT 2;\n";

    #[test]
    fn empty_ledger_applies_all() {
        assert_eq!(
            plan(&set(), &[]),
            Plan::Apply(vec![
                Version::new(1).expect("1"),
                Version::new(2).expect("2"),
                Version::new(3).expect("3")
            ])
        );
    }

    #[test]
    fn full_ledger_is_up_to_date() {
        let applied = vec![
            applied(1, "first", SQL),
            applied(2, "second", SQL),
            applied(3, "third", SQL),
        ];
        assert_eq!(plan(&set(), &applied), Plan::UpToDate);
    }

    #[test]
    fn partial_applies_only_missing() {
        let applied = vec![applied(1, "first", SQL), applied(3, "third", SQL)];
        assert_eq!(
            plan(&set(), &applied),
            Plan::Apply(vec![Version::new(2).expect("2")])
        );
    }

    #[test]
    fn checksum_slug_and_missing_file_are_all_reported() {
        let applied = vec![
            applied(1, "renamed", SQL),
            applied(2, "second", b"outro"),
            applied(4, "quarta", b"x"),
        ];
        match plan(&set(), &applied) {
            Plan::Drift(findings) => {
                assert_eq!(findings.len(), 3);
                assert_eq!(findings[0].kind, DriftKind::Slug);
                assert_eq!(findings[1].kind, DriftKind::Checksum);
                assert_eq!(findings[2].kind, DriftKind::MissingFile);
            }
            outro => panic!("esperava Drift; veio {outro:?}"),
        }
    }
}

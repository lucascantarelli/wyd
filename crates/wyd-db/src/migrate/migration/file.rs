//! Parser dos marcadores `-- migrate:up` / `-- migrate:down`.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/file.md`.

use crate::migrate::migration::sql::{Sql, SqlTextError};
use crate::migrate::migration::version::{Slug, SlugError, Version, VersionError};
use std::fmt;

/// Uma migration já parseada, ainda sem banco e sem caminho.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawMigration {
    pub version: Version,
    pub slug: Slug,
    pub up: Sql,
    pub down: Sql,
}

/// Transforma `(stem, conteúdo)` numa migration.
pub fn parse(stem: &str, content: &str) -> Result<RawMigration, FileError> {
    let (prefixo, sufixo) = match stem.split_once('_').or_else(|| stem.split_once('-')) {
        Some(par) => par,
        None => {
            return Err(FileError::BadName {
                stem: stem.to_owned(),
                reason: NameReason::NoSeparator,
            })
        }
    };
    if prefixo.len() != 4 || !prefixo.bytes().all(|b| b.is_ascii_digit()) {
        return Err(FileError::BadName {
            stem: stem.to_owned(),
            reason: NameReason::BadVersionDigits,
        });
    }
    if sufixo.is_empty() {
        return Err(FileError::BadName {
            stem: stem.to_owned(),
            reason: NameReason::MissingSlug,
        });
    }
    let version: Version = prefixo.parse().map_err(FileError::BadVersion)?;
    let slug = Slug::new(sufixo).map_err(FileError::BadSlug)?;

    let mut before_up = true;
    let mut up_seen = false;
    let mut down_seen = false;
    let mut up_body = String::new();
    let mut down_body = String::new();

    for (index, raw_line) in content.split_inclusive('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\n').unwrap_or(raw_line);
        let line = line.strip_suffix('\r').unwrap_or(line);

        if line.trim_start().starts_with("-- migrate:") {
            if line.chars().next().is_some_and(char::is_whitespace) {
                return Err(FileError::MarkerNotAlone { line: line_number });
            }
            match line {
                "-- migrate:up" => {
                    if up_seen {
                        return Err(FileError::DuplicateMarker {
                            marker: "-- migrate:up",
                        });
                    }
                    if down_seen {
                        return Err(FileError::MarkerOutOfOrder);
                    }
                    before_up = false;
                    up_seen = true;
                }
                "-- migrate:down" => {
                    if !up_seen {
                        return Err(FileError::MarkerOutOfOrder);
                    }
                    if down_seen {
                        return Err(FileError::DuplicateMarker {
                            marker: "-- migrate:down",
                        });
                    }
                    down_seen = true;
                }
                _ => {
                    return Err(FileError::MarkerNotAlone { line: line_number });
                }
            }
            continue;
        }

        if before_up {
            if !line.trim().is_empty() && !line.starts_with("--") {
                return Err(FileError::HeaderNotComment { line: line_number });
            }
            continue;
        }

        if down_seen {
            down_body.push_str(raw_line);
        } else {
            up_body.push_str(raw_line);
        }
    }

    if !up_seen {
        return Err(FileError::MissingUp);
    }
    if !down_seen {
        return Err(FileError::MissingDown);
    }
    if up_body.trim().is_empty() {
        return Err(FileError::EmptySql { part: "up" });
    }
    if down_body.trim().is_empty() {
        return Err(FileError::EmptySql { part: "down" });
    }

    let up = up_body
        .parse::<Sql>()
        .map_err(|source| FileError::Sql { part: "up", source })?;
    let down = down_body.parse::<Sql>().map_err(|source| FileError::Sql {
        part: "down",
        source,
    })?;

    Ok(RawMigration {
        version,
        slug,
        up,
        down,
    })
}

/// Por que um arquivo não entrou na série.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileError {
    BadName {
        stem: String,
        reason: NameReason,
    },
    MissingUp,
    MissingDown,
    MarkerOutOfOrder,
    DuplicateMarker {
        marker: &'static str,
    },
    MarkerNotAlone {
        line: usize,
    },
    HeaderNotComment {
        line: usize,
    },
    EmptySql {
        part: &'static str,
    },
    Sql {
        part: &'static str,
        source: SqlTextError,
    },
    BadVersion(VersionError),
    BadSlug(SlugError),
}

/// A parte do nome que não segue `NNNN_slug`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameReason {
    NoSeparator,
    BadVersionDigits,
    MissingSlug,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadName { stem, reason } => write!(f, "nome inválido {stem:?}: {reason:?}"),
            Self::MissingUp => f.write_str("falta `-- migrate:up`"),
            Self::MissingDown => f.write_str("falta `-- migrate:down`"),
            Self::MarkerOutOfOrder => f.write_str("marcadores fora de ordem"),
            Self::DuplicateMarker { marker } => write!(f, "marcador duplicado {marker}"),
            Self::MarkerNotAlone { line } => write!(f, "marcador não está sozinho na linha {line}"),
            Self::HeaderNotComment { line } => {
                write!(f, "instrução antes do `up` na linha {line}")
            }
            Self::EmptySql { part } => write!(f, "corpo `{part}` vazio"),
            Self::Sql { part, source } => write!(f, "corpo `{part}`: {source}"),
            Self::BadVersion(source) => write!(f, "versão inválida: {source}"),
            Self::BadSlug(source) => write!(f, "slug inválido: {source}"),
        }
    }
}

impl std::error::Error for FileError {}

#[cfg(test)]
mod tests {
    use super::*;

    const OK: &str =
        "-- header\n\n-- migrate:up\nCREATE TABLE x (id INT);\n-- migrate:down\nDROP TABLE x;\n";

    #[test]
    fn parses_minimal_file() {
        let raw = parse("0001_bootstrap", OK).expect("migration");
        assert_eq!(raw.version.get(), 1);
        assert_eq!(raw.slug.as_str(), "bootstrap");
        assert_eq!(raw.up.as_ref(), "CREATE TABLE x (id INT);\n");
        assert_eq!(raw.down.as_ref(), "DROP TABLE x;\n");

        let legado = parse("0001-bootstrap", OK).expect("migration com separador legado");
        assert_eq!(legado.version.get(), 1);
        assert_eq!(legado.slug.as_str(), "bootstrap");
    }

    #[test]
    fn strips_nothing_from_body() {
        let content = "-- migrate:up\n\nSELECT 1;\n\n-- migrate:down\n\nSELECT 2;\n\n";
        let raw = parse("0001_x", content).expect("migration");
        assert_eq!(raw.up.as_ref(), "\nSELECT 1;\n\n");
        assert_eq!(raw.down.as_ref(), "\nSELECT 2;\n\n");
    }

    #[test]
    fn rejects_missing_markers() {
        assert_eq!(parse("0001_x", "-- header\n"), Err(FileError::MissingUp));
        assert_eq!(
            parse("0001_x", "-- migrate:down\nDROP TABLE x;"),
            Err(FileError::MarkerOutOfOrder)
        );
        assert_eq!(
            parse("0001_x", "-- migrate:up\nCREATE TABLE x (id INT);"),
            Err(FileError::MissingDown)
        );
    }

    #[test]
    fn rejects_order_and_duplicates() {
        assert_eq!(
            parse("0001_x", "-- migrate:down\n-- migrate:up\nSELECT 1;"),
            Err(FileError::MarkerOutOfOrder)
        );
        assert_eq!(
            parse(
                "0001_x",
                "-- migrate:up\nSELECT 1;\n-- migrate:up\nSELECT 2;\n-- migrate:down\nSELECT 3;"
            ),
            Err(FileError::DuplicateMarker {
                marker: "-- migrate:up"
            })
        );
    }

    #[test]
    fn rejects_indented_marker_and_sql_in_header() {
        assert_eq!(
            parse(
                "0001_x",
                "  -- migrate:up\nSELECT 1;\n-- migrate:down\nSELECT 2;"
            ),
            Err(FileError::MarkerNotAlone { line: 1 })
        );
        assert_eq!(
            parse(
                "0001_x",
                "SELECT 1;\n-- migrate:up\nSELECT 1;\n-- migrate:down\nSELECT 2;"
            ),
            Err(FileError::HeaderNotComment { line: 1 })
        );
    }

    #[test]
    fn rejects_empty_down() {
        assert_eq!(
            parse("0001_x", "-- migrate:up\nSELECT 1;\n-- migrate:down\n\n"),
            Err(FileError::EmptySql { part: "down" })
        );
    }

    #[test]
    fn rejects_bad_stem() {
        for (stem, reason) in [
            ("1_bootstrap", NameReason::BadVersionDigits),
            ("00001_x", NameReason::BadVersionDigits),
            ("0001", NameReason::NoSeparator),
        ] {
            assert!(matches!(
                parse(stem, OK),
                Err(FileError::BadName { reason: actual, .. }) if actual == reason
            ));
        }
    }

    #[test]
    fn maps_sql_error_to_part() {
        assert!(matches!(
            parse(
                "0001_x",
                "-- migrate:up\nCOMMIT;\n-- migrate:down\nSELECT 1;"
            ),
            Err(FileError::Sql { part: "up", .. })
        ));
    }
}

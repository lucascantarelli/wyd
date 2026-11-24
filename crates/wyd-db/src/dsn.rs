//! `Dsn` — o tipo único de string de conexão do crate.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/dsn.md` (aceita em 2026-11-24).
//! `migrate` e `provision` usam este tipo e nenhum dos dois define o seu.

use std::fmt;
use std::str::FromStr;

/// String de conexão validada. Campo privado: só o parse constrói.
#[derive(Clone)]
pub struct Dsn(String);

impl FromStr for Dsn {
    type Err = DsnError;

    /// Aceita `postgres://` e `postgresql://`. Vazia ou só espaço é [`DsnError::Empty`].
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.trim().is_empty() {
            return Err(DsnError::Empty);
        }
        if raw.starts_with("postgres://") || raw.starts_with("postgresql://") {
            Ok(Self(raw.to_owned()))
        } else {
            Err(DsnError::Scheme)
        }
    }
}

impl Dsn {
    /// A string crua, para quem abre pool — nunca para log.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Dsn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Dsn(<redacted>)")
    }
}

impl fmt::Display for Dsn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Dsn(<redacted>)")
    }
}

/// Único erro deste arquivo. Não carrega a entrada: ela pode conter senha.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DsnError {
    /// Vazia ou só espaço.
    Empty,
    /// Nem `postgres://` nem `postgresql://`.
    Scheme,
}

impl fmt::Display for DsnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("DSN vazia"),
            Self::Scheme => f.write_str("esquema não aceito: use postgres:// ou postgresql://"),
        }
    }
}

impl std::error::Error for DsnError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok_and_redacted() {
        let dsn: Dsn = "postgres://u:segredo@h:5432/db".parse().expect("parse");
        let debug = format!("{dsn:?}");
        let display = format!("{dsn}");
        assert!(debug.contains("<redacted>"));
        assert!(display.contains("<redacted>"));
        assert!(!debug.contains("segredo"));
        assert!(!display.contains("segredo"));
    }

    #[test]
    fn postgresql_scheme_is_accepted() {
        assert!("postgresql://u@h/db".parse::<Dsn>().is_ok());
    }

    #[test]
    fn empty_and_blank_are_empty_error() {
        assert_eq!("".parse::<Dsn>().unwrap_err(), DsnError::Empty);
        assert_eq!("   ".parse::<Dsn>().unwrap_err(), DsnError::Empty);
    }

    #[test]
    fn other_scheme_is_scheme_error() {
        assert_eq!(
            "mysql://u@h/db".parse::<Dsn>().unwrap_err(),
            DsnError::Scheme
        );
    }

    #[test]
    fn debug_is_identical_for_different_passwords() {
        let a: Dsn = "postgres://u:um@h/db".parse().expect("parse");
        let b: Dsn = "postgres://u:outro@h/db".parse().expect("parse");
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }

    #[test]
    fn as_str_returns_the_input_verbatim() {
        let raw = "postgres://u:p@h:5432/db?sslmode=disable";
        let dsn: Dsn = raw.parse().expect("parse");
        assert_eq!(dsn.as_str(), raw);
    }
}

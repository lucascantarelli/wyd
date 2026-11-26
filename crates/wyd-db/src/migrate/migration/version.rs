//! `Version` e `Slug` — os dois nomes de uma migration.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/version.md`.

use std::fmt;
use std::num::NonZeroU32;
use std::str::FromStr;

/// Versão numérica de uma migration. Invariante: maior que zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(NonZeroU32);

impl Version {
    /// Constrói uma versão; zero é recusado.
    pub fn new(value: u32) -> Result<Self, VersionError> {
        NonZeroU32::new(value).map(Self).ok_or(VersionError::Zero)
    }

    /// O valor numérico.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0.get()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0.get())
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
            return Err(VersionError::NotANumber);
        }
        if raw.len() > 9 {
            return Err(VersionError::TooLarge);
        }
        let value = raw.parse::<u32>().map_err(|_| VersionError::TooLarge)?;
        Self::new(value)
    }
}

/// Por que um texto não é uma versão.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionError {
    /// `0` não é uma versão válida.
    Zero,
    /// O texto não é composto só de dígitos.
    NotANumber,
    /// O valor não cabe em `u32`.
    TooLarge,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("versão zero não existe"),
            Self::NotANumber => f.write_str("versão precisa ser um inteiro positivo"),
            Self::TooLarge => f.write_str("versão maior que u32"),
        }
    }
}

impl std::error::Error for VersionError {}

/// Nome lógico da migration, sem o prefixo numérico.
///
/// Invariante: `^[a-z][a-z0-9_]*$`, de 1 a 60 caracteres, sem `__` e sem `_` final.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug(String);

impl Slug {
    /// Constrói um slug válido.
    pub fn new(raw: &str) -> Result<Self, SlugError> {
        if raw.is_empty() {
            return Err(SlugError::Empty);
        }
        if raw.len() > 60 {
            return Err(SlugError::TooLong {
                len: raw.len(),
                max: 60,
            });
        }
        if raw.as_bytes()[0].is_ascii_digit() {
            return Err(SlugError::LeadingDigit);
        }
        if !raw.as_bytes()[0].is_ascii_lowercase() {
            return Err(SlugError::InvalidChar {
                ch: raw.chars().next().expect("não vazio"),
                index: 0,
            });
        }
        let mut anterior_underscore = false;
        for (index, ch) in raw.char_indices() {
            let valido = ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_';
            if !valido {
                return Err(SlugError::InvalidChar { ch, index });
            }
            if ch == '_' && anterior_underscore {
                return Err(SlugError::DoubleUnderscore { index: index - 1 });
            }
            anterior_underscore = ch == '_';
        }
        if raw.ends_with('_') {
            return Err(SlugError::TrailingUnderscore);
        }
        Ok(Self(raw.to_owned()))
    }

    /// O texto do slug.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Slug {
    type Err = SlugError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::new(raw)
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Por que um texto não é um slug.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlugError {
    /// Texto vazio.
    Empty,
    /// Passou do limite de 60 caracteres.
    TooLong {
        /// Comprimento recebido.
        len: usize,
        /// Limite permitido.
        max: usize,
    },
    /// O primeiro caractere é um dígito.
    LeadingDigit,
    /// Caractere fora de `[a-z0-9_]`.
    InvalidChar {
        /// Caractere recusado.
        ch: char,
        /// Posição em bytes do caractere.
        index: usize,
    },
    /// Dois underscores seguidos.
    DoubleUnderscore {
        /// Posição do primeiro underscore do par.
        index: usize,
    },
    /// Slug terminando em `_`.
    TrailingUnderscore,
}

impl fmt::Display for SlugError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("slug vazio"),
            Self::TooLong { len, max } => write!(f, "slug com {len} caracteres; máximo {max}"),
            Self::LeadingDigit => f.write_str("slug não pode começar por dígito"),
            Self::InvalidChar { ch, index } => {
                write!(f, "caractere inválido {ch:?} na posição {index}")
            }
            Self::DoubleUnderscore { index } => {
                write!(f, "underscore duplicado na posição {index}")
            }
            Self::TrailingUnderscore => f.write_str("slug não pode terminar em underscore"),
        }
    }
}

impl std::error::Error for SlugError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_rejects_zero() {
        assert_eq!(Version::new(0), Err(VersionError::Zero));
    }

    #[test]
    fn version_display_pads_four() {
        assert_eq!(Version::new(7).expect("versão").to_string(), "0007");
        assert_eq!(Version::new(1234).expect("versão").to_string(), "1234");
    }

    #[test]
    fn version_parse_accepts_leading_zeros_and_counts_numerically() {
        assert_eq!("0007".parse::<Version>().expect("versão").get(), 7);
        assert_eq!("7".parse::<Version>().expect("versão").get(), 7);
        assert!(Version::new(9).expect("9") < Version::new(10).expect("10"));
    }

    #[test]
    fn version_rejects_invalid_text() {
        assert_eq!("0".parse::<Version>(), Err(VersionError::Zero));
        assert_eq!("-1".parse::<Version>(), Err(VersionError::NotANumber));
        assert_eq!(" 7".parse::<Version>(), Err(VersionError::NotANumber));
        assert_eq!("".parse::<Version>(), Err(VersionError::NotANumber));
        assert_eq!("1234567890".parse::<Version>(), Err(VersionError::TooLarge));
    }

    #[test]
    fn slug_accepts_table_names() {
        for raw in ["ops", "server_channel", "character_learned_skill"] {
            assert_eq!(Slug::new(raw).expect("slug").as_str(), raw);
        }
    }

    #[test]
    fn slug_rejects_each_rule() {
        assert_eq!(Slug::new(""), Err(SlugError::Empty));
        assert_eq!(
            Slug::new("Game_Session"),
            Err(SlugError::InvalidChar { ch: 'G', index: 0 })
        );
        assert_eq!(
            Slug::new("game-session"),
            Err(SlugError::InvalidChar { ch: '-', index: 4 })
        );
        assert_eq!(
            Slug::new("game__session"),
            Err(SlugError::DoubleUnderscore { index: 4 })
        );
        assert_eq!(Slug::new("game_"), Err(SlugError::TrailingUnderscore));
        assert_eq!(Slug::new("1game"), Err(SlugError::LeadingDigit));
        assert_eq!(
            Slug::new("_game"),
            Err(SlugError::InvalidChar { ch: '_', index: 0 })
        );
    }

    #[test]
    fn slug_length_boundary() {
        assert!(Slug::new(&"a".repeat(60)).is_ok());
        assert_eq!(
            Slug::new(&"a".repeat(61)),
            Err(SlugError::TooLong { len: 61, max: 60 })
        );
    }
}

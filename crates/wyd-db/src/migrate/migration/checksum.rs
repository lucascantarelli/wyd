//! SHA-256 do arquivo inteiro, com EOL normalizado.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/checksum.md`.

use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;

/// SHA-256 do conteúdo normalizado. Invariante: sempre 32 bytes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Checksum([u8; 32]);

impl Checksum {
    /// Único construtor: normaliza EOL e hasheia.
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        let mut normalizado = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == b'\r' && index + 1 < bytes.len() && bytes[index + 1] == b'\n' {
                normalizado.push(b'\n');
                index += 2;
            } else {
                normalizado.push(bytes[index]);
                index += 1;
            }
        }
        if normalizado.last() == Some(&b'\n') {
            normalizado.pop();
        }
        let digest = Sha256::digest(&normalizado);
        let mut output = [0_u8; 32];
        output.copy_from_slice(&digest);
        Self(output)
    }

    /// Os 32 bytes do hash.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = self.to_string();
        write!(f, "Checksum({}…)", &hex[..8])
    }
}

impl AsRef<[u8]> for Checksum {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromStr for Checksum {
    type Err = ChecksumError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() != 64 {
            return Err(ChecksumError::Length { found: raw.len() });
        }
        let mut output = [0_u8; 32];
        for (index, pair) in raw.as_bytes().as_chunks::<2>().0.iter().enumerate() {
            let high = hex_nibble(pair[0], index * 2)?;
            let low = hex_nibble(pair[1], index * 2 + 1)?;
            output[index] = (high << 4) | low;
        }
        Ok(Self(output))
    }
}

fn hex_nibble(byte: u8, index: usize) -> Result<u8, ChecksumError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Err(ChecksumError::NotLowercaseHex { index }),
        _ => Err(ChecksumError::NotHex { index }),
    }
}

/// Por que um texto não é um checksum armazenado.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumError {
    /// Não tem 64 caracteres.
    Length { found: usize },
    /// Tem uma letra maiúscula.
    NotLowercaseHex { index: usize },
    /// Tem um caractere fora de hexadecimal.
    NotHex { index: usize },
}

impl fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length { found } => write!(f, "checksum com {found} caracteres; esperado 64"),
            Self::NotLowercaseHex { index } => {
                write!(f, "checksum com maiúscula na posição {index}")
            }
            Self::NotHex { index } => write!(
                f,
                "checksum com caractere não hexadecimal na posição {index}"
            ),
        }
    }
}

impl std::error::Error for ChecksumError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crlf_equals_lf() {
        assert_eq!(Checksum::of(b"a\r\n"), Checksum::of(b"a\n"));
    }

    #[test]
    fn trailing_newline_ignored_once() {
        assert_eq!(Checksum::of(b"a\n"), Checksum::of(b"a"));
    }

    #[test]
    fn double_newline_is_content() {
        assert_ne!(Checksum::of(b"a\n\n"), Checksum::of(b"a\n"));
    }

    #[test]
    fn lone_cr_is_content() {
        assert_ne!(Checksum::of(b"a\r"), Checksum::of(b"a"));
    }

    #[test]
    fn empty_is_not_a_panic() {
        let checksum = Checksum::of(b"");
        assert_eq!(checksum.to_string().len(), 64);
    }

    #[test]
    fn display_is_64_lowercase() {
        let checksum = Checksum::of(b"x").to_string();
        assert_eq!(checksum.len(), 64);
        assert!(checksum
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    }

    #[test]
    fn parse_roundtrip_and_errors() {
        let original = Checksum::of(b"x");
        let parsed: Checksum = original.to_string().parse().expect("checksum");
        assert_eq!(parsed, original);
        assert_eq!(
            "ab".parse::<Checksum>(),
            Err(ChecksumError::Length { found: 2 })
        );
        assert_eq!(
            format!("{}A", &original.to_string()[..63]).parse::<Checksum>(),
            Err(ChecksumError::NotLowercaseHex { index: 63 })
        );
        assert_eq!(
            format!("{}z", &original.to_string()[..63]).parse::<Checksum>(),
            Err(ChecksumError::NotHex { index: 63 })
        );
    }
}

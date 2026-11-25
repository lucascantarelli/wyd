//! Comparação do manifesto com o disco (`LCLI-02`).
//!
//! Regra única: o arquivo só é aceito quando tamanho **e** `sha256` batem com o declarado.
//! Divergência é resultado, não erro de execução: o relatório lista o que divergiu para que
//! o launcher recuse o patch inteiro e diga por quê.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::manifest::Manifest;

/// Uma divergência entre o manifesto e o arquivo no disco.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Divergence {
    /// O arquivo declarado não existe na raiz do cliente.
    Missing { path: String },
    /// O tamanho medido difere do declarado.
    Size {
        path: String,
        expected: u64,
        actual: u64,
    },
    /// O `sha256` medido difere do declarado.
    Hash {
        path: String,
        expected: String,
        actual: String,
    },
}

impl Divergence {
    /// Caminho relativo do arquivo que divergiu.
    pub fn path(&self) -> &str {
        match self {
            Self::Missing { path } | Self::Size { path, .. } | Self::Hash { path, .. } => path,
        }
    }
}

/// Resultado da verificação: quantos arquivos foram checados e o que divergiu.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Report {
    pub checked: usize,
    pub divergences: Vec<Divergence>,
}

impl Report {
    /// `true` quando nenhum arquivo divergiu.
    pub fn is_ok(&self) -> bool {
        self.divergences.is_empty()
    }
}

/// Verifica cada arquivo do manifesto sob `root`, sem escrever nada.
pub fn verify(manifest: &Manifest, root: &Path) -> io::Result<Report> {
    let mut report = Report::default();
    for arquivo in &manifest.files {
        let caminho = root.join(&arquivo.path);
        report.checked += 1;
        match medir(&caminho)? {
            None => report.divergences.push(Divergence::Missing {
                path: arquivo.path.clone(),
            }),
            Some((tamanho, sha256)) => {
                if tamanho != arquivo.size {
                    report.divergences.push(Divergence::Size {
                        path: arquivo.path.clone(),
                        expected: arquivo.size,
                        actual: tamanho,
                    });
                } else if sha256 != arquivo.sha256 {
                    report.divergences.push(Divergence::Hash {
                        path: arquivo.path.clone(),
                        expected: arquivo.sha256.clone(),
                        actual: sha256,
                    });
                }
            }
        }
    }
    Ok(report)
}

/// Tamanho e `sha256` do arquivo; `None` quando ele não existe.
fn medir(caminho: &Path) -> io::Result<Option<(u64, String)>> {
    let mut arquivo = match File::open(caminho) {
        Ok(arquivo) => arquivo,
        Err(erro) if erro.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(erro) => return Err(erro),
    };
    let mut hasher = Sha256::new();
    let mut tamanho = 0_u64;
    let mut buffer = [0_u8; 8 * 1024];
    loop {
        let lidos = arquivo.read(&mut buffer)?;
        if lidos == 0 {
            break;
        }
        tamanho += lidos as u64;
        hasher.update(&buffer[..lidos]);
    }
    Ok(Some((tamanho, hex(&hasher.finalize()))))
}

/// Converte o digest em 64 dígitos hexadecimais minúsculos.
fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;

    let mut saida = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(saida, "{byte:02x}");
    }
    saida
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_do_digest_tem_64_digitos() {
        let digest = Sha256::digest(b"wyd");
        assert_eq!(hex(&digest).len(), 64);
        assert_eq!(
            hex(&Sha256::digest(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}

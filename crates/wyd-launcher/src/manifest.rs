//! Leitura e validação de forma do manifesto (`ADR-LCH-0001`, `LCLI-02`).
//!
//! A validação é de **forma**, não de conteúdo jurídico: o manifesto diz quais arquivos o
//! patch entrega, com tamanho e `sha256`; se um caminho escapa da raiz, se dois caminhos
//! repetem ou se um hash não é SHA-256, o manifesto é recusado antes de tocar no disco.

use std::collections::HashSet;

use serde::Deserialize;

/// Versão do formato aceita por este crate. Valor novo exige código novo, não fallback.
pub const SCHEMA: u32 = 1;

/// Canais de publicação registrados em `portal.launcher_release.channel`.
const CHANNELS: [&str; 2] = ["stable", "beta"];

/// Manifesto do patch, como declarado no `ADR-LCH-0001`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    pub channel: String,
    pub version: String,
    pub client_build: String,
    pub min_version: String,
    #[serde(default)]
    pub files: Vec<FileEntry>,
}

/// Um arquivo entregue pelo patch.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FileEntry {
    /// Caminho relativo à raiz do cliente, com `/` como separador.
    pub path: String,
    pub size: u64,
    /// SHA-256 em 64 dígitos hexadecimais minúsculos.
    pub sha256: String,
    /// Onde o arquivo é servido. Ausente enquanto não há host de release (`P`).
    #[serde(default)]
    pub url: Option<String>,
}

impl Manifest {
    /// Lê e valida o manifesto. Não toca no disco.
    pub fn from_json(texto: &str) -> Result<Self, ManifestError> {
        let manifesto: Self =
            serde_json::from_str(texto).map_err(|erro| ManifestError::Json(erro.to_string()))?;
        manifesto.validate()?;
        Ok(manifesto)
    }

    /// Valida a forma do manifesto já desserializado.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.schema != SCHEMA {
            return Err(ManifestError::Schema(self.schema));
        }
        if !CHANNELS.contains(&self.channel.as_str()) {
            return Err(ManifestError::Channel(self.channel.clone()));
        }
        require_non_empty(&self.version, "version")?;
        require_non_empty(&self.min_version, "min_version")?;
        if !is_sha256(&self.client_build) {
            return Err(ManifestError::ClientBuild(self.client_build.clone()));
        }
        if self.files.is_empty() {
            return Err(ManifestError::NoFiles);
        }

        let mut vistos: HashSet<&str> = HashSet::new();
        for arquivo in &self.files {
            if !is_relative_path(&arquivo.path) {
                return Err(ManifestError::Path(arquivo.path.clone()));
            }
            if !vistos.insert(arquivo.path.as_str()) {
                return Err(ManifestError::DuplicatePath(arquivo.path.clone()));
            }
            if !is_sha256(&arquivo.sha256) {
                return Err(ManifestError::Sha256 {
                    path: arquivo.path.clone(),
                });
            }
            if let Some(url) = &arquivo.url {
                if !url.starts_with("https://") {
                    return Err(ManifestError::Url(arquivo.path.clone()));
                }
            }
        }
        Ok(())
    }
}

/// Falha de forma do manifesto — sempre anterior a qualquer leitura de arquivo.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ManifestError {
    #[error("JSON inválido: {0}")]
    Json(String),
    #[error("schema {0} não suportado (esperado {SCHEMA})")]
    Schema(u32),
    #[error("canal desconhecido: {0}")]
    Channel(String),
    #[error("campo vazio: {0}")]
    Empty(&'static str),
    #[error("`client_build` não é SHA-256: {0}")]
    ClientBuild(String),
    #[error("manifesto sem arquivos")]
    NoFiles,
    #[error("caminho inválido no manifesto: {0}")]
    Path(String),
    #[error("caminho repetido no manifesto: {0}")]
    DuplicatePath(String),
    #[error("`sha256` inválido para {path}: esperado 64 hex minúsculos")]
    Sha256 { path: String },
    #[error("`url` não é https para {0}")]
    Url(String),
}

fn require_non_empty(valor: &str, campo: &'static str) -> Result<(), ManifestError> {
    if valor.trim().is_empty() {
        return Err(ManifestError::Empty(campo));
    }
    Ok(())
}

/// `true` para 64 dígitos hexadecimais minúsculos.
pub fn is_sha256(valor: &str) -> bool {
    valor.len() == 64
        && valor
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// `true` para caminho relativo com `/`, sem `..`, raiz absoluta, letra de unidade ou `\`.
fn is_relative_path(caminho: &str) -> bool {
    if caminho.is_empty() || caminho.starts_with('/') || caminho.contains('\\') {
        return false;
    }
    if caminho.contains(':') {
        return false;
    }
    caminho
        .split('/')
        .all(|parte| !parte.is_empty() && parte != "." && parte != "..")
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "26a6140b9a31258b018e17c53ca47df17c532e3ae362d73ba6c764d38686a81f";

    fn manifesto(files: &str) -> String {
        format!(
            r#"{{"schema":1,"channel":"stable","version":"1.0.0","client_build":"{HASH}","min_version":"1.0.0","files":[{files}]}}"#
        )
    }

    fn arquivo(path: &str, sha256: &str) -> String {
        // O caminho entra escapado: uma barra invertida literal no JSON é `\\`, não `\`.
        let path = serde_json::to_string(path).expect("escapar caminho");
        format!(r#"{{"path":{path},"size":1,"sha256":"{sha256}"}}"#)
    }

    #[test]
    fn aceita_manifesto_valido() {
        let texto = manifesto(&arquivo("client/UI/strdef.bin", HASH));
        let lido = Manifest::from_json(&texto).expect("manifesto válido");
        assert_eq!(lido.files.len(), 1);
    }

    #[test]
    fn recusa_schema_desconhecido() {
        let texto = manifesto(&arquivo("a.bin", HASH)).replace("\"schema\":1", "\"schema\":2");
        assert_eq!(Manifest::from_json(&texto), Err(ManifestError::Schema(2)));
    }

    #[test]
    fn recusa_caminho_que_escapa_da_raiz() {
        for caminho in [
            "../fora.bin",
            "/absoluto.bin",
            "C:/windows.bin",
            "a\\b.bin",
            "a//b.bin",
        ] {
            let texto = manifesto(&arquivo(caminho, HASH));
            assert!(
                matches!(Manifest::from_json(&texto), Err(ManifestError::Path(_))),
                "aceitou {caminho}"
            );
        }
    }

    #[test]
    fn recusa_caminho_repetido() {
        let texto = manifesto(&format!(
            "{},{}",
            arquivo("a.bin", HASH),
            arquivo("a.bin", HASH)
        ));
        assert_eq!(
            Manifest::from_json(&texto),
            Err(ManifestError::DuplicatePath("a.bin".to_string()))
        );
    }

    #[test]
    fn recusa_hash_fora_do_formato() {
        for hash in ["abc", &HASH.to_uppercase(), &HASH[..63]] {
            let texto = manifesto(&arquivo("a.bin", hash));
            assert!(
                matches!(
                    Manifest::from_json(&texto),
                    Err(ManifestError::Sha256 { .. })
                ),
                "aceitou {hash}"
            );
        }
    }

    #[test]
    fn recusa_manifesto_sem_arquivos() {
        assert_eq!(
            Manifest::from_json(&manifesto("")),
            Err(ManifestError::NoFiles)
        );
    }

    #[test]
    fn recusa_url_que_nao_e_https() {
        let texto = manifesto(&format!(
            r#"{{"path":"a.bin","size":1,"sha256":"{HASH}","url":"http://x"}}"#
        ));
        assert!(matches!(
            Manifest::from_json(&texto),
            Err(ManifestError::Url(_))
        ));
    }

    #[test]
    fn reconhece_sha256_canonico() {
        assert!(is_sha256(HASH));
        assert!(!is_sha256(&HASH.to_uppercase()));
        assert!(!is_sha256(""));
    }
}

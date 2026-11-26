//! Verificação da assinatura Ed25519 destacada (`LLCH-03`, `ADR-LCH-0001` §3).
//!
//! A assinatura cobre os **bytes canônicos** do manifesto (RFC 8785, ver [`crate::canon`]) e vive no
//! arquivo irmão `manifest.json.sig`. Este módulo só verifica: assinar é do `wyd-ops`, com a chave
//! privada que nunca entra no repositório nem no binário.
//!
//! **Encoding fixado aqui** (o ADR deixou para a primeira implementação): `manifest.json.sig` guarda
//! os 64 bytes da assinatura em **hexadecimal minúsculo** (128 caracteres), com espaço ou quebra de
//! linha nas pontas tolerados e nada mais. A chave pública embutida no launcher usa a mesma forma,
//! 64 caracteres hexadecimais minúsculos para 32 bytes. Forma diferente é recusada, não adivinhada.

use ed25519_dalek::{Signature, VerifyingKey};

use crate::canon::{canonicalize_str, CanonError};

/// Identificador de chave e chave pública que o launcher confia.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    /// `key_id` publicado no manifesto de release; é o que seleciona a chave.
    pub key_id: String,
    /// 32 bytes da chave pública Ed25519 em 64 dígitos hexadecimais minúsculos.
    pub hex: String,
}

/// Conjunto de chaves confiadas. Rotação = lista nova no launcher, sem migration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrustStore {
    keys: Vec<PublicKey>,
}

impl TrustStore {
    /// Monta o conjunto na ordem dada; `key_id` repetido é permitido e a primeira ocorrência vence.
    pub fn new(keys: impl IntoIterator<Item = PublicKey>) -> Self {
        Self {
            keys: keys.into_iter().collect(),
        }
    }

    /// Chaves confiadas, na ordem de inserção.
    pub fn keys(&self) -> &[PublicKey] {
        &self.keys
    }

    /// Verifica `manifest.json.sig` sobre a forma canônica de `manifest_json`.
    pub fn verify(
        &self,
        key_id: &str,
        manifest_json: &str,
        signature_file: &str,
    ) -> Result<(), SignatureError> {
        let chave = self
            .keys
            .iter()
            .find(|chave| chave.key_id == key_id)
            .ok_or_else(|| SignatureError::UnknownKey(key_id.to_string()))?;
        verificar(manifest_json, signature_file, chave)
    }
}

/// Verifica uma assinatura destacada contra uma chave conhecida.
pub fn verificar(
    manifest_json: &str,
    signature_file: &str,
    chave: &PublicKey,
) -> Result<(), SignatureError> {
    let canonico = canonicalize_str(manifest_json)?;
    let assinatura =
        decodificar_hex(signature_file, 64).map_err(SignatureError::SignatureEncoding)?;
    let bytes_chave = decodificar_hex(&chave.hex, 32).map_err(SignatureError::KeyEncoding)?;

    let assinatura = Signature::from_slice(&assinatura)
        .map_err(|_| SignatureError::SignatureEncoding(assinatura.len()))?;
    let bytes_chave: [u8; 32] = bytes_chave
        .try_into()
        .map_err(|_| SignatureError::KeyEncoding(0))?;
    let verificador = VerifyingKey::from_bytes(&bytes_chave)
        .map_err(|erro| SignatureError::PublicKey(erro.to_string()))?;

    verificador
        .verify_strict(canonico.as_bytes(), &assinatura)
        .map_err(|_| SignatureError::Invalid {
            key_id: chave.key_id.clone(),
        })
}

/// `Ok(bytes)` só para hexadecimal minúsculo do tamanho exato, com espaço nas pontas tolerado.
fn decodificar_hex(texto: &str, bytes_esperados: usize) -> Result<Vec<u8>, usize> {
    let limpo = texto.trim();
    if limpo.len() != bytes_esperados * 2 {
        return Err(limpo.len());
    }
    let mut saida = Vec::with_capacity(bytes_esperados);
    for par in limpo.as_bytes().chunks(2) {
        let (alto, baixo) = (valor_hex(par[0]), valor_hex(par[1]));
        match (alto, baixo) {
            (Some(alto), Some(baixo)) => saida.push(alto << 4 | baixo),
            _ => return Err(limpo.len()),
        }
    }
    Ok(saida)
}

fn valor_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

/// Falha de verificação — sempre antes de qualquer leitura de arquivo do patch.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SignatureError {
    #[error("`manifest.json.sig` tem {0} caracteres; esperado 128 hex minúsculos")]
    SignatureEncoding(usize),
    #[error("chave pública tem {0} caracteres; esperado 64 hex minúsculos")]
    KeyEncoding(usize),
    #[error("chave desconhecida: {0}")]
    UnknownKey(String),
    #[error("chave pública inválida: {0}")]
    PublicKey(String),
    #[error("assinatura não confere com a chave {key_id}")]
    Invalid { key_id: String },
    #[error(transparent)]
    Canon(#[from] CanonError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    const MANIFESTO: &str = r#"{"schema":1,"channel":"stable","version":"1.0.0","client_build":"04c9d2a1235cba57f7b588347fae935a2466824ddf9d7d92e541e0cc8a27ddf8","min_version":"1.0.0","files":[{"path":"client/UI/tabela.bin","size":64,"sha256":"c1d8bb944d06dd641f1c13d75e1d4d0bbca3bf906a15264eeb0119c9ab8c810e"}]}"#;

    fn hexadecimal(bytes: &[u8]) -> String {
        use std::fmt::Write as _;
        let mut saida = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            let _ = write!(saida, "{byte:02x}");
        }
        saida
    }

    fn par(seed: u8) -> (SigningKey, PublicKey) {
        let assinador = SigningKey::from_bytes(&[seed; 32]);
        let chave = PublicKey {
            key_id: format!("teste-{seed}"),
            hex: hexadecimal(assinador.verifying_key().as_bytes()),
        };
        (assinador, chave)
    }

    fn assinar(assinador: &SigningKey, manifesto: &str) -> String {
        let canonico = canonicalize_str(manifesto).expect("canonizar");
        hexadecimal(&assinador.sign(canonico.as_bytes()).to_bytes())
    }

    #[test]
    fn aceita_assinatura_valida() {
        let (assinador, chave) = par(7);
        let sig = assinar(&assinador, MANIFESTO);
        assert_eq!(super::verificar(MANIFESTO, &sig, &chave), Ok(()));
    }

    #[test]
    fn aceita_manifesto_reindentado() {
        let (assinador, chave) = par(7);
        let sig = assinar(&assinador, MANIFESTO);
        let indentado = MANIFESTO.replace(",", ",\n  ");
        assert_eq!(super::verificar(&indentado, &sig, &chave), Ok(()));
    }

    #[test]
    fn recusa_assinatura_de_outra_chave() {
        let (assinador, _) = par(7);
        let (_, outra) = par(9);
        let sig = assinar(&assinador, MANIFESTO);
        assert!(matches!(
            super::verificar(MANIFESTO, &sig, &outra),
            Err(SignatureError::Invalid { .. })
        ));
    }

    #[test]
    fn recusa_manifesto_alterado() {
        let (assinador, chave) = par(7);
        let sig = assinar(&assinador, MANIFESTO);
        let alterado = MANIFESTO.replace("\"1.0.0\"", "\"1.0.1\"");
        assert!(matches!(
            super::verificar(&alterado, &sig, &chave),
            Err(SignatureError::Invalid { .. })
        ));
    }

    #[test]
    fn recusa_assinatura_fora_do_formato() {
        let (_, chave) = par(7);
        for sig in ["", "00", &"a".repeat(128).to_uppercase()] {
            assert!(matches!(
                super::verificar(MANIFESTO, sig, &chave),
                Err(SignatureError::SignatureEncoding(_))
            ));
        }
    }

    #[test]
    fn recusa_chave_fora_do_formato() {
        let (assinador, _) = par(7);
        let sig = assinar(&assinador, MANIFESTO);
        let chave = PublicKey {
            key_id: "quebrada".to_string(),
            hex: "abc".to_string(),
        };
        assert!(matches!(
            super::verificar(MANIFESTO, &sig, &chave),
            Err(SignatureError::KeyEncoding(_))
        ));
    }

    #[test]
    fn trust_store_recusa_chave_desconhecida() {
        let (assinador, chave) = par(7);
        let sig = assinar(&assinador, MANIFESTO);
        let store = TrustStore::new([chave]);
        assert_eq!(
            store.verify("nao-existe", MANIFESTO, &sig),
            Err(SignatureError::UnknownKey("nao-existe".to_string()))
        );
        assert_eq!(store.keys().len(), 1);
    }

    #[test]
    fn tolera_quebra_de_linha_nas_pontas() {
        let (assinador, chave) = par(7);
        let sig = format!("{}\n", assinar(&assinador, MANIFESTO));
        assert_eq!(super::verificar(MANIFESTO, &sig, &chave), Ok(()));
    }
}

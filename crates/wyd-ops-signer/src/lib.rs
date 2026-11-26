//! Assinatura Ed25519 do manifesto do patch (`LLCH-03`, `ADR-LCH-0001`).
//!
//! A operação de release assina os **bytes canônicos RFC 8785** do manifesto e grava o arquivo irmão
//! `manifest.json.sig` no encoding fixado pelo verificador: 64 bytes em hexadecimal minúsculo, sem
//! nada mais. A canonização vem de [`wyd_launcher::canonicalize_str`] — este crate não tem cópia
//! dela, e é justamente isso que faz o teste de ciclo entre `wyd-ops` e `wyd-launcher` valer.
//!
//! A chave privada nunca é argumento de linha de comando por padrão: vem do ambiente
//! ([`VAR_CHAVE`]) ou de arquivo lido localmente, e nunca é impressa.

use std::path::Path;

use ed25519_dalek::{Signer, SigningKey};
use wyd_launcher::canonicalize_str;

/// Variável de ambiente de onde o release lê a chave privada (64 hex minúsculos).
pub const VAR_CHAVE: &str = "WYD_LAUNCHER_SIGNING_KEY";

/// Falha ao assinar — antes de qualquer arquivo ser publicado.
#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("chave privada inválida: esperado 64 dígitos hexadecimais minúsculos")]
    ChaveInvalida,
    #[error("manifesto inválido para canonização: {0}")]
    Canon(String),
    #[error("falha de I/O em {caminho}: {erro}")]
    Io {
        caminho: String,
        erro: std::io::Error,
    },
}

/// Lê a chave de 32 bytes a partir de 64 dígitos hexadecimais minúsculos (espaço nas pontas ok).
pub fn ler_chave(texto: &str) -> Result<SigningKey, SignError> {
    let limpo = texto.trim();
    if limpo.len() != 64 {
        return Err(SignError::ChaveInvalida);
    }
    let mut semente = [0_u8; 32];
    for (indice, par) in limpo.as_bytes().chunks(2).enumerate() {
        let alto = valor_hex(par[0]).ok_or(SignError::ChaveInvalida)?;
        let baixo = valor_hex(par[1]).ok_or(SignError::ChaveInvalida)?;
        semente[indice] = alto << 4 | baixo;
    }
    Ok(SigningKey::from_bytes(&semente))
}

/// Assina o manifesto e devolve o conteúdo do `.sig` (128 hex minúsculos).
pub fn assinar(manifesto_json: &str, chave: &SigningKey) -> Result<String, SignError> {
    let canonico =
        canonicalize_str(manifesto_json).map_err(|erro| SignError::Canon(erro.to_string()))?;
    Ok(hexadecimal(&chave.sign(canonico.as_bytes()).to_bytes()))
}

/// Assina lendo a chave em hexadecimal; é a forma que o CLI usa.
pub fn assinar_texto(manifesto_json: &str, chave_hex: &str) -> Result<String, SignError> {
    assinar(manifesto_json, &ler_chave(chave_hex)?)
}

/// Assina `manifesto` e grava `saida` com os 128 caracteres, sem quebra de linha.
pub fn assinar_arquivo(manifesto: &Path, chave_hex: &str, saida: &Path) -> Result<(), SignError> {
    let texto = std::fs::read_to_string(manifesto).map_err(|erro| SignError::Io {
        caminho: manifesto.display().to_string(),
        erro,
    })?;
    let assinatura = assinar_texto(&texto, chave_hex)?;
    std::fs::write(saida, assinatura).map_err(|erro| SignError::Io {
        caminho: saida.display().to_string(),
        erro,
    })
}

fn hexadecimal(bytes: &[u8]) -> String {
    use std::fmt::Write as _;

    let mut saida = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(saida, "{byte:02x}");
    }
    saida
}

fn valor_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Verifier;

    const MANIFESTO: &str = r#"{"schema":1,"channel":"stable","version":"1.0.0","client_build":"04c9d2a1235cba57f7b588347fae935a2466824ddf9d7d92e541e0cc8a27ddf8","min_version":"1.0.0","files":[{"path":"client/UI/tabela.bin","size":64,"sha256":"c1d8bb944d06dd641f1c13d75e1d4d0bbca3bf906a15264eeb0119c9ab8c810e"}]}"#;
    const CHAVE_TESTE: &str = "4242424242424242424242424242424242424242424242424242424242424242";

    #[test]
    fn assinatura_tem_128_hex_minusculos() {
        let assinatura = assinar_texto(MANIFESTO, CHAVE_TESTE).expect("assinar");
        assert_eq!(assinatura.len(), 128);
        assert!(assinatura
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    }

    #[test]
    fn assinatura_verifica_com_a_propria_chave() {
        let chave = ler_chave(CHAVE_TESTE).unwrap();
        let assinatura = assinar_texto(MANIFESTO, CHAVE_TESTE).unwrap();
        let bytes: Vec<u8> = (0..64)
            .map(|i| u8::from_str_radix(&assinatura[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        let assinatura = ed25519_dalek::Signature::from_slice(&bytes).unwrap();
        let canonico = canonicalize_str(MANIFESTO).unwrap();
        chave
            .verifying_key()
            .verify(canonico.as_bytes(), &assinatura)
            .expect("assinatura válida");
    }

    #[test]
    fn chave_fora_do_formato_reprova() {
        for chave in ["", "abc", &"A".repeat(64)] {
            assert!(matches!(ler_chave(chave), Err(SignError::ChaveInvalida)));
        }
    }

    #[test]
    fn manifesto_invalido_reprova() {
        assert!(matches!(
            assinar_texto("{não é json}", CHAVE_TESTE),
            Err(SignError::Canon(_))
        ));
    }
}

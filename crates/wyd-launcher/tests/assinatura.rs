//! Assinatura destacada sobre o manifesto — fixture real em `tests/fixtures/assinatura/`.
//!
//! Fixture gerada em 2026-11-25 com par de chaves de teste (seed fixa), manifesto de
//! `tests/fixtures/manifesto-valido.json` e assinatura sobre a forma canônica RFC 8785.

use std::path::{Path, PathBuf};

use wyd_launcher::{verificar, PublicKey, SignatureError, TrustStore};

const KEY_ID: &str = "wyd-launcher-teste";

fn caminho(nome: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/assinatura")
        .join(nome)
}

fn ler(nome: &str) -> String {
    std::fs::read_to_string(caminho(nome)).unwrap_or_else(|erro| panic!("{nome}: {erro}"))
}

fn chave(hex: &str) -> PublicKey {
    PublicKey {
        key_id: KEY_ID.to_string(),
        hex: hex.to_string(),
    }
}

#[test]
fn aceita_assinatura_do_fixture() {
    let manifesto = ler("manifest.json");
    let assinatura = ler("manifest.json.sig");
    let publica = chave(&ler("chave-publica.hex"));
    assert_eq!(verificar(&manifesto, &assinatura, &publica), Ok(()));
}

#[test]
fn recusa_manifesto_alterado() {
    let manifesto = ler("manifest.json").replace("\"1.0.0\"", "\"1.0.1\"");
    let assinatura = ler("manifest.json.sig");
    let publica = chave(&ler("chave-publica.hex"));
    assert!(matches!(
        verificar(&manifesto, &assinatura, &publica),
        Err(SignatureError::Invalid { .. })
    ));
}

#[test]
fn recusa_assinatura_corrompida() {
    let manifesto = ler("manifest.json");
    let original = ler("manifest.json.sig");
    let mut corrompida = original.clone();
    let primeiro = corrompida.remove(0);
    let trocado = if primeiro == '0' { '1' } else { '0' };
    corrompida.insert(0, trocado);
    let publica = chave(&ler("chave-publica.hex"));
    assert!(matches!(
        verificar(&manifesto, &corrompida, &publica),
        Err(SignatureError::Invalid { .. })
    ));
}

#[test]
fn recusa_assinatura_maiuscula() {
    let manifesto = ler("manifest.json");
    let assinatura = ler("manifest.json.sig").to_uppercase();
    let publica = chave(&ler("chave-publica.hex"));
    assert!(matches!(
        verificar(&manifesto, &assinatura, &publica),
        Err(SignatureError::SignatureEncoding(_))
    ));
}

#[test]
fn recusa_chave_errada() {
    let manifesto = ler("manifest.json");
    let assinatura = ler("manifest.json.sig");
    let errada = chave(&ler("chave-errada.hex"));
    assert!(matches!(
        verificar(&manifesto, &assinatura, &errada),
        Err(SignatureError::Invalid { .. })
    ));
}

#[test]
fn trust_store_recusa_chave_desconhecida() {
    let manifesto = ler("manifest.json");
    let assinatura = ler("manifest.json.sig");
    let store = TrustStore::new([chave(&ler("chave-publica.hex"))]);
    assert_eq!(store.verify(KEY_ID, &manifesto, &assinatura), Ok(()));
    assert_eq!(
        store.verify("wyd-launcher-antiga", &manifesto, &assinatura),
        Err(SignatureError::UnknownKey(
            "wyd-launcher-antiga".to_string()
        ))
    );
}

#[test]
fn assinatura_cobre_bytes_canonicos_e_nao_o_arquivo() {
    // Reindentar o manifesto não invalida a assinatura: a cobertura é a forma canônica.
    let manifesto = ler("manifest.json").replace(",\n", ",\n\n  ");
    let assinatura = ler("manifest.json.sig");
    let publica = chave(&ler("chave-publica.hex"));
    assert_eq!(verificar(&manifesto, &assinatura, &publica), Ok(()));
}

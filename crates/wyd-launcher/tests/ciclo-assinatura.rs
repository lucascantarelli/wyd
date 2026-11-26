//! Ciclo completo: o `wyd-ops` assina, o `wyd-launcher` verifica.
//!
//! O teste usa o assinador de verdade (`wyd_ops_signer`) sobre o manifesto da fixture e verifica com
//! o `TrustStore` do launcher. Como o assinador não tem canonização própria e o verificador exige o
//! encoding fixado, qualquer divergência entre os dois lados reprova aqui.
//!
//! A chave usada é **de teste**, declarada abaixo, e não abre nada: é a mesma semente fixa do
//! fixture `tests/fixtures/assinatura/`.

use std::path::{Path, PathBuf};

use wyd_launcher::{PublicKey, SignatureError, TrustStore};

/// Semente de teste (0x42 repetido). Nunca foi e nunca será usada em release.
const CHAVE_TESTE: &str = "4242424242424242424242424242424242424242424242424242424242424242";

fn fixture(nome: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/assinatura")
        .join(nome)
}

fn ler(nome: &str) -> String {
    std::fs::read_to_string(fixture(nome)).unwrap_or_else(|erro| panic!("{nome}: {erro}"))
}

/// Chave pública que corresponde à semente de teste, derivada pelo assinador.
fn chave_publica() -> PublicKey {
    let chave = wyd_ops_signer::ler_chave(CHAVE_TESTE).expect("chave de teste");
    let hex = chave
        .verifying_key()
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    PublicKey {
        key_id: "wyd-launcher-teste".to_string(),
        hex,
    }
}

#[test]
fn assinador_do_wyd_ops_alimenta_o_trust_store_do_launcher() {
    let manifesto = ler("manifest.json");
    let assinatura = wyd_ops_signer::assinar_texto(&manifesto, CHAVE_TESTE).expect("assinar");
    assert_eq!(assinatura.len(), 128);

    let store = TrustStore::new([chave_publica()]);
    assert_eq!(
        store.verify("wyd-launcher-teste", &manifesto, &assinatura),
        Ok(())
    );
}

#[test]
fn assinatura_do_assinador_reprova_manifesto_alterado() {
    let manifesto = ler("manifest.json");
    let assinatura = wyd_ops_signer::assinar_texto(&manifesto, CHAVE_TESTE).expect("assinar");
    let alterado = manifesto.replace("\"1.0.0\"", "\"1.0.1\"");
    let store = TrustStore::new([chave_publica()]);
    assert!(matches!(
        store.verify("wyd-launcher-teste", &alterado, &assinatura),
        Err(SignatureError::Invalid { .. })
    ));
}

#[test]
fn assinatura_do_assinador_reprova_chave_errada() {
    let manifesto = ler("manifest.json");
    let assinatura = wyd_ops_signer::assinar_texto(&manifesto, CHAVE_TESTE).expect("assinar");
    let errada = PublicKey {
        key_id: "wyd-launcher-teste".to_string(),
        hex: ler("chave-errada.hex"),
    };
    let store = TrustStore::new([errada]);
    assert!(matches!(
        store.verify("wyd-launcher-teste", &manifesto, &assinatura),
        Err(SignatureError::Invalid { .. })
    ));
}

#[test]
fn assinatura_gerada_em_arquivo_e_a_mesma_do_texto() {
    let manifesto = ler("manifest.json");
    let esperada = wyd_ops_signer::assinar_texto(&manifesto, CHAVE_TESTE).expect("assinar");
    let diretorio = tempfile::tempdir().expect("tmp");
    let saida = diretorio.path().join("manifest.json.sig");
    wyd_ops_signer::assinar_arquivo(&fixture("manifest.json"), CHAVE_TESTE, &saida)
        .expect("assinar arquivo");
    let em_arquivo = std::fs::read_to_string(&saida).expect("ler .sig");
    assert_eq!(em_arquivo, esperada);
    // Sem quebra de linha: o verificador não depende de normalização do arquivo.
    assert!(!em_arquivo.contains('\n'));
}

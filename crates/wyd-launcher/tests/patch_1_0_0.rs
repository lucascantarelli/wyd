//! Verificação do manifesto real do patch da `1.0.0` — `LCLI-02` (2026-11-25, opção A).
//!
//! O manifesto medido fica em `planejamento-1.0.0/wyd-launcher/docs/specs/patch-1.0.0.json`
//! e aponta para os três arquivos de tabela de strings do cliente. A árvore do cliente é
//! ignorada pelo git (`wyd-client/files/`), então o teste é **condicional**: onde a árvore
//! não existe — CI, máquina limpa — ele imprime o motivo e passa sem medir; onde existe,
//! ele mede os bytes reais. O `sha256` do manifesto foi medido em 2026-11-25.

use std::fs;
use std::path::{Path, PathBuf};

use wyd_launcher::manifest::Manifest;
use wyd_launcher::verify;

fn raiz_do_repositorio() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn manifesto_do_patch() -> PathBuf {
    raiz_do_repositorio().join("planejamento-1.0.0/wyd-launcher/docs/specs/patch-1.0.0.json")
}

fn arvore_do_cliente() -> PathBuf {
    raiz_do_repositorio().join("wyd-client/files")
}

#[test]
fn manifesto_do_patch_e_valido() {
    let texto = fs::read_to_string(manifesto_do_patch()).expect("ler manifesto do patch");
    let manifesto = Manifest::from_json(&texto).expect("manifesto do patch válido");

    assert_eq!(manifesto.version, "1.0.0");
    assert_eq!(manifesto.channel, "stable");
    assert_eq!(manifesto.files.len(), 3);
}

#[test]
fn manifesto_do_patch_confere_com_a_arvore_local() {
    let raiz = arvore_do_cliente();
    if !raiz.is_dir() {
        eprintln!("wyd-client/files ausente: verificação do patch real não medida nesta máquina");
        return;
    }

    let texto = fs::read_to_string(manifesto_do_patch()).expect("ler manifesto do patch");
    let manifesto = Manifest::from_json(&texto).expect("manifesto do patch válido");

    let relatorio = verify(&manifesto, &raiz).expect("verificar patch");

    assert_eq!(relatorio.checked, 3);
    assert!(
        relatorio.is_ok(),
        "divergências: {:?}",
        relatorio.divergences
    );
}

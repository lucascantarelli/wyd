//! Testes do verificador do manifesto — `LCLI-02` (2026-11-25, opção A).
//!
//! Os fixtures ficam centralizados em `tests/fixtures/`: um manifesto válido e uma tabela
//! de exemplo. Cada teste copia o fixture para um diretório temporário antes de alterar,
//! para que nenhum teste dependa da ordem nem do estado deixado por outro.

use std::fs;
use std::path::{Path, PathBuf};

use wyd_launcher::manifest::Manifest;
use wyd_launcher::{verify, Divergence};

const MANIFESTO: &str = include_str!("fixtures/manifesto-valido.json");

fn fixture(nome: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(nome)
}

/// Copia `tests/fixtures/dados/` para um temporário e devolve a raiz.
fn raiz_temporaria() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("temporário");
    let raiz = dir.path().join("cliente");
    fs::create_dir_all(raiz.join("client/UI")).expect("criar árvore");
    fs::copy(
        fixture("dados/tabela.bin"),
        raiz.join("client/UI/tabela.bin"),
    )
    .expect("copiar fixture");
    (dir, raiz)
}

fn manifesto() -> Manifest {
    Manifest::from_json(MANIFESTO).expect("fixture válido")
}

#[test]
fn aceita_arquivo_com_hash_e_tamanho_declarados() {
    let (_dir, raiz) = raiz_temporaria();
    let relatorio = verify(&manifesto(), &raiz).expect("verificação");

    assert_eq!(relatorio.checked, 1);
    assert!(relatorio.is_ok(), "{:?}", relatorio.divergences);
}

#[test]
fn recusa_hash_divergente() {
    let (_dir, raiz) = raiz_temporaria();
    fs::write(raiz.join("client/UI/tabela.bin"), b"conteudo adulterado").expect("adulterar");

    let relatorio = verify(&manifesto(), &raiz).expect("verificação");

    assert_eq!(relatorio.divergences.len(), 1);
    assert!(matches!(
        &relatorio.divergences[0],
        Divergence::Size { path, .. } if path == "client/UI/tabela.bin"
    ));
}

#[test]
fn recusa_hash_divergente_com_mesmo_tamanho() {
    let (_dir, raiz) = raiz_temporaria();
    let original = fs::read(fixture("dados/tabela.bin")).expect("ler fixture");
    let mut trocado = original.clone();
    trocado[0] ^= 0xff;
    fs::write(raiz.join("client/UI/tabela.bin"), trocado).expect("trocar byte");

    let relatorio = verify(&manifesto(), &raiz).expect("verificação");

    match relatorio.divergences.as_slice() {
        [Divergence::Hash {
            path,
            expected,
            actual,
        }] => {
            assert_eq!(path, "client/UI/tabela.bin");
            assert_ne!(expected, actual);
        }
        outro => panic!("esperava divergência de hash, veio {outro:?}"),
    }
}

#[test]
fn recusa_arquivo_ausente() {
    let (_dir, raiz) = raiz_temporaria();
    fs::remove_file(raiz.join("client/UI/tabela.bin")).expect("remover");

    let relatorio = verify(&manifesto(), &raiz).expect("verificação");

    assert_eq!(
        relatorio.divergences,
        vec![Divergence::Missing {
            path: "client/UI/tabela.bin".to_string()
        }]
    );
    assert!(!relatorio.is_ok());
}

#[test]
fn relatorio_de_sucesso_nao_tem_divergencia() {
    let (_dir, raiz) = raiz_temporaria();
    let relatorio = verify(&manifesto(), &raiz).expect("verificação");
    assert_eq!(relatorio.divergences, Vec::new());
}

//! CLI do assinador: `wyd-ops-sign sign --manifest <arq> --out <arq.sig>`.
//!
//! A chave vem de `--key-file <arq>` ou da variável `WYD_LAUNCHER_SIGNING_KEY`; sem uma das duas, o
//! comando falha com mensagem explícita — é o mesmo comportamento que o `release.yml` exige.
//! Nenhum caminho imprime a chave.

use std::path::PathBuf;
use std::process::ExitCode;

use wyd_ops_signer::{assinar_arquivo, VAR_CHAVE};

const USO: &str = "uso: wyd-ops-sign sign --manifest <manifest.json> --out <manifest.json.sig> \\
[--key-file <arquivo>]

A chave privada (64 hex minúsculos) vem de --key-file ou da variável de ambiente \
WYD_LAUNCHER_SIGNING_KEY.";

fn main() -> ExitCode {
    match executar(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(erro) => {
            eprintln!("erro: {erro}");
            ExitCode::FAILURE
        }
    }
}

fn executar(mut args: Vec<String>) -> Result<(), String> {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("{USO}");
        return Ok(());
    }
    if args[0] == "sign" {
        args.remove(0);
    }

    let mut manifesto: Option<PathBuf> = None;
    let mut saida: Option<PathBuf> = None;
    let mut arquivo_chave: Option<PathBuf> = None;
    let mut indice = 0;
    while indice < args.len() {
        let atual = args[indice].as_str();
        let valor = |indice: usize| -> Result<&String, String> {
            args.get(indice + 1)
                .ok_or_else(|| format!("falta valor para {atual}"))
        };
        match atual {
            "--manifest" => {
                manifesto = Some(PathBuf::from(valor(indice)?));
                indice += 2;
            }
            "--out" => {
                saida = Some(PathBuf::from(valor(indice)?));
                indice += 2;
            }
            "--key-file" => {
                arquivo_chave = Some(PathBuf::from(valor(indice)?));
                indice += 2;
            }
            outro => return Err(format!("argumento desconhecido: {outro}\n\n{USO}")),
        }
    }

    let manifesto = manifesto.ok_or_else(|| format!("falta --manifest\n\n{USO}"))?;
    let saida = saida.ok_or_else(|| format!("falta --out\n\n{USO}"))?;
    let chave = match arquivo_chave {
        Some(caminho) => std::fs::read_to_string(&caminho)
            .map_err(|erro| format!("{}: {erro}", caminho.display()))?,
        None => std::env::var(VAR_CHAVE)
            .map_err(|_| format!("chave ausente: use --key-file ou a variável {VAR_CHAVE}"))?,
    };
    if chave.trim().is_empty() {
        return Err(format!("chave vazia em {VAR_CHAVE}"));
    }

    assinar_arquivo(&manifesto, &chave, &saida).map_err(|erro| erro.to_string())?;
    println!(
        "assinado: {} -> {} ({} caracteres)",
        manifesto.display(),
        saida.display(),
        128
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sem_manifesto_reprova() {
        let erro = executar(vec!["sign".to_string()]).unwrap_err();
        assert!(erro.contains("--manifest"), "{erro}");
    }

    #[test]
    fn argumento_desconhecido_reprova() {
        let erro = executar(vec!["--nada".to_string()]).unwrap_err();
        assert!(erro.contains("desconhecido"), "{erro}");
    }

    #[test]
    fn ajuda_nao_reprova() {
        assert!(executar(vec!["--help".to_string()]).is_ok());
    }
}

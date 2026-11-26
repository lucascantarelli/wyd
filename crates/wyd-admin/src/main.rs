//! Composition root mínimo do `wyd-admin`.
//!
//! Sobe apenas as sondas de processo (`http`) para o cenário de desenvolvimento. O
//! contrato C4, a autenticação de serviço e o acesso ao banco não estão implementados.
//! Endereço por `WYD_ADMIN_BIND` (padrão de desenvolvimento `127.0.0.1:7600`).

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let endereco = env::var("WYD_ADMIN_BIND").unwrap_or_else(|_| String::from("127.0.0.1:7600"));
    // `--health` é a sonda do `HEALTHCHECK` do container: consulta o próprio processo
    // e encerra com 0/1, sem depender de `curl` na imagem.
    if env::args().any(|argumento| argumento == "--health") {
        return match wyd_admin::http::sondar(&endereco) {
            Ok(true) => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE,
        };
    }
    println!("event=start modo=esqueleto bind={endereco}");
    if let Err(erro) = wyd_admin::http::servir(&endereco, true) {
        eprintln!("event=fatal motivo=bind endereco={endereco} erro={erro}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

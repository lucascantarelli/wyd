//! Composition root mínimo do `wyd-server`.
//!
//! Sobe apenas as sondas de processo (`health`) para o cenário de desenvolvimento.
//! `EPROD`, canais, protocolo e ECAP não estão implementados: este binário não fala
//! com o cliente do jogo. Endereço por `WYD_SERVER_HEALTH_BIND` (padrão de
//! desenvolvimento `127.0.0.1:7100`).

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let endereco =
        env::var("WYD_SERVER_HEALTH_BIND").unwrap_or_else(|_| String::from("127.0.0.1:7100"));
    // `--health` é a sonda do `HEALTHCHECK` do container: consulta o próprio processo
    // e encerra com 0/1, sem depender de `curl` na imagem.
    if env::args().any(|argumento| argumento == "--health") {
        return match wyd_server::health::sondar(&endereco) {
            Ok(true) => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE,
        };
    }
    println!("event=start modo=esqueleto bind={endereco}");
    if let Err(erro) = wyd_server::health::servir(&endereco, true) {
        eprintln!("event=fatal motivo=bind endereco={endereco} erro={erro}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

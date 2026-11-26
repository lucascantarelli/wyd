//! `/health` e `/ready` do processo — módulo `health` do plano do `wyd-server`.
//!
//! Escopo desta estrutura inicial: provar que o processo sobe, responde e é
//! encerrável. Não é o `EPROD`: não abre o CPSock, não fala com o cliente do jogo e
//! não conhece conta, canal ou protocolo.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// Resposta HTTP mínima de uma sonda: estado numérico e corpo JSON fixo.
#[derive(Debug, PartialEq, Eq)]
pub struct Resposta {
    pub status: u16,
    pub corpo: &'static str,
}

impl Resposta {
    fn razao(&self) -> &'static str {
        match self.status {
            200 => "OK",
            404 => "Not Found",
            405 => "Method Not Allowed",
            503 => "Service Unavailable",
            _ => "Internal Server Error",
        }
    }

    /// Serializa a resposta HTTP/1.1 com `Connection: close`.
    pub fn para_wire(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            self.status,
            self.razao(),
            self.corpo.len(),
            self.corpo
        )
    }
}

/// Interpreta a linha de requisição e devolve a resposta da sonda.
///
/// `/health` responde sempre 200 (processo vivo); `/ready` responde 200 só quando
/// `pronto`; qualquer outro caminho é 404 e qualquer método diferente de `GET` é 405.
pub fn avaliar(requisicao: &str, pronto: bool) -> Resposta {
    let mut partes = requisicao.split_whitespace();
    let metodo = partes.next().unwrap_or("");
    let caminho = partes.next().unwrap_or("");
    if metodo != "GET" {
        return Resposta {
            status: 405,
            corpo: "{\"erro\":\"metodo\"}",
        };
    }
    match caminho {
        "/health" => Resposta {
            status: 200,
            corpo: "{\"ok\":true}",
        },
        "/ready" if pronto => Resposta {
            status: 200,
            corpo: "{\"pronto\":true}",
        },
        "/ready" => Resposta {
            status: 503,
            corpo: "{\"pronto\":false}",
        },
        _ => Resposta {
            status: 404,
            corpo: "{\"erro\":\"rota\"}",
        },
    }
}

fn atender(mut conexao: TcpStream, pronto: bool) -> std::io::Result<()> {
    let mut leitor = BufReader::new(&conexao);
    let mut linha = String::new();
    leitor.read_line(&mut linha)?;
    let resposta = avaliar(linha.trim_end(), pronto);
    conexao.write_all(resposta.para_wire().as_bytes())?;
    conexao.flush()
}

/// Verdadeiro quando a linha de status HTTP é 200.
pub fn status_ok(linha: &str) -> bool {
    linha.starts_with("HTTP/1.1 200") || linha.starts_with("HTTP/1.0 200")
}

/// Sonda o próprio processo por TCP — usada pelo `HEALTHCHECK` do container.
pub fn sondar(endereco: &str) -> std::io::Result<bool> {
    let mut conexao = TcpStream::connect(endereco)?;
    conexao.write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")?;
    let mut leitor = BufReader::new(conexao);
    let mut linha = String::new();
    leitor.read_line(&mut linha)?;
    Ok(status_ok(&linha))
}

/// Escuta `endereco` e atende as sondas até o processo ser encerrado.
pub fn servir(endereco: &str, pronto: bool) -> std::io::Result<()> {
    let listener = TcpListener::bind(endereco)?;
    for conexao in listener.incoming() {
        match conexao {
            Ok(conexao) => {
                std::thread::spawn(move || {
                    // Sonda é uma linha e fecha: erro de escrita não derruba o processo.
                    let _ = atender(conexao, pronto);
                });
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn health_responde_200() {
        let resposta = avaliar("GET /health HTTP/1.1", false);
        assert_eq!(resposta.status, 200);
        assert_eq!(resposta.corpo, "{\"ok\":true}");
    }

    #[test]
    fn ready_depende_do_estado() {
        assert_eq!(avaliar("GET /ready HTTP/1.1", true).status, 200);
        assert_eq!(avaliar("GET /ready HTTP/1.1", false).status, 503);
    }

    #[test]
    fn rota_desconhecida_e_metodo_invalido() {
        assert_eq!(avaliar("GET /admin HTTP/1.1", true).status, 404);
        assert_eq!(avaliar("POST /health HTTP/1.1", true).status, 405);
    }

    #[test]
    fn status_ok_reconhece_somente_200() {
        assert!(status_ok("HTTP/1.1 200 OK\r\n"));
        assert!(!status_ok("HTTP/1.1 503 Service Unavailable\r\n"));
        assert!(!status_ok(""));
    }

    #[test]
    fn wire_tem_cabecalho_e_tamanho() {
        let wire = avaliar("GET /health HTTP/1.1", true).para_wire();
        assert!(wire.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(wire.contains("Content-Length: 11\r\n"));
        assert!(wire.ends_with("{\"ok\":true}"));
    }
}

//! Decodifica uma sessão gravada pelo proxy do cliente (`tools/cliente-harness/proxy.mjs`)
//! usando o mesmo parser do servidor — `wyd-protocol`.
//!
//! O proxy grava JSONL com uma linha por frame (`dir`, `size`, `hex`, `init`) e uma
//! linha `FIM` por conexão. Aqui cada frame é desofuscado (`decode_in_place`),
//! o `HEADER` é lido e o opcode é resolvido pela tabela congelada; quando o layout
//! está portado, os campos são impressos.
//!
//! Uso:
//!   cargo run -p wyd-server --bin decode-captura -- tmp/cliente/primeira-sessao.jsonl
//!   cargo run -p wyd-server --bin decode-captura -- <sessao.jsonl> --hex
//!
//! Nada aqui inventa layout: opcode sem tipo portado aparece só como `0xNNNN`.

use std::env;
use std::fs;
use std::process::ExitCode;

use wyd_protocol::payload::{nul_trimmed, AccountLogin, CreateMob, MessagePanel};
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::{decode_in_place, Header, Opcode, WirePayload};
use wyd_server::personagens::Mob;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(caminho) = args.next() else {
        eprintln!("uso: decode-captura <sessao.jsonl> [--hex]");
        return ExitCode::FAILURE;
    };
    let mostrar_hex = args.any(|arg| arg == "--hex");

    match rodar(&caminho, mostrar_hex) {
        Ok((frames, falhas)) => {
            println!("\nframes={frames} falhas={falhas}");
            if frames == 0 || falhas > 0 {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(erro) => {
            eprintln!("erro: {erro}");
            ExitCode::FAILURE
        }
    }
}

/// Lê o JSONL e imprime uma linha por frame. Devolve `(frames, falhas)`.
fn rodar(caminho: &str, mostrar_hex: bool) -> Result<(usize, usize), String> {
    let conteudo = fs::read_to_string(caminho).map_err(|e| format!("{caminho}: {e}"))?;

    let mut frames = 0usize;
    let mut falhas = 0usize;
    for (indice, linha) in conteudo.lines().enumerate() {
        let linha = linha.trim();
        if linha.is_empty() || linha.contains("\"dir\":\"FIM\"") {
            continue;
        }
        let numero = indice + 1;
        let dir = campo(linha, "dir").unwrap_or("?");
        let Some(hex) = campo(linha, "hex") else {
            eprintln!("#{numero} {dir} linha sem campo hex");
            falhas += 1;
            continue;
        };
        let Some(bytes) = bytes_de_hex(hex) else {
            eprintln!("#{numero} {dir} hex inválido ({} caracteres)", hex.len());
            falhas += 1;
            continue;
        };
        frames += 1;

        if linha.contains("\"init\":true") {
            println!("#{numero} {dir} INITCODE {}", hex.to_ascii_uppercase());
            continue;
        }
        match decodificar(&bytes) {
            Ok(texto) => println!("#{numero} {dir} {texto}"),
            Err(erro) => {
                falhas += 1;
                println!("#{numero} {dir} {erro}");
                if mostrar_hex {
                    println!("    hex={}", hex.to_ascii_uppercase());
                }
            }
        }
    }
    Ok((frames, falhas))
}

/// Desofusca e interpreta um frame completo. `Err` traz o motivo legível.
fn decodificar(bytes: &[u8]) -> Result<String, String> {
    let mut frame = bytes.to_vec();
    let key_word = decode_in_place(&mut frame).map_err(|e| format!("! codec: {e}"))?;
    let header = Header::decode(&frame).map_err(|e| format!("! header: {e}"))?;

    let (nome, opcode) = match Opcode::from_wire(header.message_type) {
        Some(opcode) => (opcode.const_name(), Some(opcode)),
        None => ("desconhecido", None),
    };
    let mut linha = format!(
        "{:>5}B 0x{:04x} {nome} id={} tick={} key={} sum=0x{:02x}",
        header.size, header.message_type, header.id, header.client_tick, key_word, header.checksum
    );
    if let Some(resumo) = opcode.and_then(|op| resumo(op, &frame)) {
        linha.push(' ');
        linha.push_str(&resumo);
    }
    Ok(linha)
}

/// Campos dos opcodes já portados. `None` = sem resumo (opcode fora do contrato).
fn resumo(opcode: Opcode, frame: &[u8]) -> Option<String> {
    match opcode {
        Opcode::AccountLogin => {
            let p = AccountLogin::decode(frame).ok()?;
            Some(format!(
                "conta={} versao={} db_need_save={}",
                String::from_utf8_lossy(p.account_name_bytes()),
                p.client_version,
                p.db_need_save
            ))
        }
        Opcode::CreateMob => {
            let p = CreateMob::decode(frame).ok()?;
            Some(format!(
                "mob={} nome={} pos=({},{}) nivel={} hp={}/{} mp={}/{} for={} int={} des={} con={} chaos={}",
                p.mob_id,
                String::from_utf8_lossy(nul_trimmed(&p.mob_name)),
                p.pos_x,
                p.pos_y,
                p.level,
                p.max_hp,
                p.hp,
                p.max_mp,
                p.mp,
                p.strength,
                p.intelligence,
                p.dexterity,
                p.constitution,
                p.chaos_rate
            ))
        }
        Opcode::CNFCharacterLogin => {
            // Corpo: `PosX@0`, `PosY@2`, `STRUCT_MOB@4` (`character_login_body`).
            let mob = Mob::decode(frame.get(HEADER_SIZE + 4..)?).ok()?;
            let score = mob.current_score;
            Some(format!(
                "mob={} nivel={} hp={}/{} mp={}/{} for={} int={} des={} con={} dano={} chaos={} resist={:?} critico={}",
                String::from_utf8_lossy(nul_trimmed(&mob.name)),
                score.level,
                score.hp,
                score.max_hp,
                score.mp,
                score.max_mp,
                score.strength,
                score.intelligence,
                score.dexterity,
                score.constitution,
                score.damage,
                score.chaos_rate,
                mob.resist,
                mob.critical
            ))
        }
        Opcode::UpdateScore => {
            // Corpo: `STRUCT_SCORE`@0 (48 B), `Critical`@48, `Resist[4]`@118,
            // `CurrHp`@124, `CurrMp`@128 (`MSG_UpdateScore`, protocolo.md §6).
            let body = frame.get(HEADER_SIZE..)?;
            if body.len() < 132 {
                return None;
            }
            let i32_at = |off: usize| {
                i32::from_le_bytes([body[off], body[off + 1], body[off + 2], body[off + 3]])
            };
            let i16_at = |off: usize| i16::from_le_bytes([body[off], body[off + 1]]);
            Some(format!(
                "nivel={} hp={}/{} mp={}/{} for={} int={} des={} con={} chaos={} dano={} critico={} resist={:?} curr_hp={} curr_mp={}",
                i32_at(0),
                i32_at(24),
                i32_at(16),
                i32_at(28),
                i32_at(20),
                i16_at(32),
                i16_at(34),
                i16_at(36),
                i16_at(38),
                body[15],
                i32_at(8),
                body[48],
                [body[118], body[119], body[120], body[121]],
                i32_at(124),
                i32_at(128)
            ))
        }
        Opcode::MessagePanel => {
            let p = MessagePanel::decode(frame).ok()?;
            Some(format!(
                "texto={}",
                String::from_utf8_lossy(nul_trimmed(&p.text))
            ))
        }
        _ => None,
    }
}

/// Valor de um campo string simples (`"chave":"valor"`), sem dependência de JSON.
fn campo<'a>(linha: &'a str, chave: &str) -> Option<&'a str> {
    let alvo = format!("\"{chave}\":\"");
    let inicio = linha.find(&alvo)? + alvo.len();
    let resto = &linha[inicio..];
    Some(&resto[..resto.find('"')?])
}

/// Hex ASCII → bytes. `None` para tamanho ímpar ou caractere inválido.
fn bytes_de_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wyd_protocol::encode_in_place;

    fn frame_de_login() -> Vec<u8> {
        let mut nome = [0u8; 16];
        nome[..5].copy_from_slice(b"admin");
        let mut frame = vec![0u8; AccountLogin::WIRE_LEN];
        frame[0..2].copy_from_slice(&(AccountLogin::WIRE_LEN as u16).to_le_bytes());
        frame[4..6].copy_from_slice(&Opcode::AccountLogin.wire().to_le_bytes());
        frame[6..8].copy_from_slice(&1u16.to_le_bytes());
        AccountLogin {
            account_password: [0u8; 12],
            account_name: nome,
            zero: [0u8; 52],
            client_version: wyd_protocol::PROTOCOL_VERSION as i32,
            db_need_save: 1,
            adapter_name: [0i32; 4],
        }
        .encode_into(&mut frame)
        .expect("frame do tamanho do contrato");
        encode_in_place(&mut frame, 7).expect("frame válido");
        frame
    }

    #[test]
    fn hex_com_tamanho_impar_ou_lixo_e_recusado() {
        assert!(bytes_de_hex("0d02").is_some());
        assert!(bytes_de_hex("0d0").is_none());
        assert!(bytes_de_hex("zz").is_none());
    }

    #[test]
    fn campo_le_o_valor_sem_quebrar_no_proximo() {
        let linha = r#"{"dir":"C","size":116,"hex":"0d02"}"#;
        assert_eq!(campo(linha, "dir"), Some("C"));
        assert_eq!(campo(linha, "hex"), Some("0d02"));
        assert_eq!(campo(linha, "init"), None);
    }

    #[test]
    fn decodifica_login_sem_expor_senha() {
        let texto = decodificar(&frame_de_login()).expect("frame do próprio parser");
        assert!(texto.contains("AccountLogin"), "{texto}");
        assert!(texto.contains("conta=admin"), "{texto}");
        assert!(texto.contains("versao=12000"), "{texto}");
        assert!(!texto.contains("password"), "{texto}");
    }

    #[test]
    fn le_jsonl_do_proxy_e_conta_frames() {
        let caminho = std::env::temp_dir().join(format!(
            "captura-{}-{}.jsonl",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        let hex = frame_de_login()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let conteudo = format!(
            "{{\"dir\":\"C\",\"size\":4,\"hex\":\"1f11f311\",\"init\":true}}\n{{\"dir\":\"C\",\"size\":116,\"hex\":\"{hex}\"}}\n{{\"dir\":\"FIM\"}}\n"
        );
        std::fs::write(&caminho, conteudo).expect("grava a sessão de teste");
        let resultado = rodar(caminho.to_str().expect("caminho utf-8"), false);
        let _ = std::fs::remove_file(&caminho);
        assert_eq!(resultado.expect("sessão legível"), (2, 0));
    }

    #[test]
    fn frame_truncado_vira_erro_e_nao_panic() {
        let frame = frame_de_login();
        let texto = decodificar(&frame[..5]).expect_err("5 bytes não são frame");
        assert!(
            texto.contains("codec") || texto.contains("header"),
            "{texto}"
        );
    }
}

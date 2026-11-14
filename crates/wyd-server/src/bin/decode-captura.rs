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
//!   cargo run -p wyd-server --bin decode-captura -- <sessao.jsonl> \
//!     --dsn postgres://admin:admin123@127.0.0.1:5432/wyd
//!
//! Com `--dsn` (ou `WYD_TEST_DSN`/`DATABASE_URL`), o personagem do
//! `_MSG_CNFCharacterLogin` é comparado com `characters` + `character_scores`
//! (`Progression`) e cada divergência de nível/MP/atributos falha o processo.
//!
//! Nada aqui inventa layout: opcode sem tipo portado aparece só como `0xNNNN`.

use std::env;
use std::fs;
use std::process::ExitCode;

use wyd_db::{Accounts, Characters, PgBackend, PoolConfig};
use wyd_protocol::payload::{nul_trimmed, AccountLogin, CreateMob, MessagePanel};
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::{decode_in_place, Header, Opcode, WirePayload};
use wyd_server::personagens::{wire_level, Mob};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(caminho) = args.next() else {
        eprintln!("uso: decode-captura <sessao.jsonl> [--hex] [--dsn <url>]");
        return ExitCode::FAILURE;
    };
    let mut mostrar_hex = false;
    let mut dsn: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--hex" => mostrar_hex = true,
            "--dsn" => dsn = args.next(),
            _ => {}
        }
    }
    let dsn = dsn
        .or_else(|| env::var("WYD_TEST_DSN").ok())
        .or_else(|| env::var("DATABASE_URL").ok())
        .filter(|valor| !valor.trim().is_empty());

    match rodar(&caminho, mostrar_hex) {
        Ok((frames, falhas, achados)) => {
            println!("\nframes={frames} falhas={falhas}");
            let mut ok = frames > 0 && falhas == 0;
            if let Some(dsn) = dsn.as_deref() {
                match comparar(&achados, dsn) {
                    Ok(0) => {}
                    Ok(divergencias) => {
                        println!("divergencias={divergencias}");
                        ok = false;
                    }
                    Err(erro) => {
                        eprintln!("comparação com o banco: {erro}");
                        ok = false;
                    }
                }
            }
            if ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(erro) => {
            eprintln!("erro: {erro}");
            ExitCode::FAILURE
        }
    }
}

/// O que a captura diz sobre o personagem e a conta (para comparar com o banco).
#[derive(Default)]
struct Achados {
    conta: Option<String>,
    personagem: Option<String>,
    slot: Option<u16>,
    score: Option<Score>,
}

/// Campos de `STRUCT_SCORE` que interessam ao painel do cliente.
#[derive(Default, Clone, Copy)]
struct Score {
    nivel: i32,
    hp: i32,
    max_hp: i32,
    mp: i32,
    max_mp: i32,
    forca: i16,
    inteligencia: i16,
    destreza: i16,
    constituicao: i16,
    chaos: u8,
    dano: i32,
    critico: u8,
    resist: [u8; 4],
    curr_hp: i32,
    curr_mp: i32,
}

/// Lê o JSONL e imprime uma linha por frame. Devolve `(frames, falhas)`.
fn rodar(caminho: &str, mostrar_hex: bool) -> Result<(usize, usize, Achados), String> {
    let conteudo = fs::read_to_string(caminho).map_err(|e| format!("{caminho}: {e}"))?;

    let mut frames = 0usize;
    let mut falhas = 0usize;
    let mut achados = Achados::default();
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
        match decodificar(&bytes, &mut achados) {
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
    Ok((frames, falhas, achados))
}

/// Desofusca e interpreta um frame completo. `Err` traz o motivo legível.
fn decodificar(bytes: &[u8], achados: &mut Achados) -> Result<String, String> {
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
    if let Some(opcode) = opcode {
        preencher(opcode, &frame, achados);
        if let Some(resumo) = resumo(opcode, &frame) {
            linha.push(' ');
            linha.push_str(&resumo);
        }
    }
    Ok(linha)
}

/// Guarda o que a captura diz de conta/personagem/`Score` para a comparação com o banco.
fn preencher(opcode: Opcode, frame: &[u8], achados: &mut Achados) {
    match opcode {
        Opcode::AccountLogin => {
            if let Ok(login) = AccountLogin::decode(frame) {
                achados.conta =
                    Some(String::from_utf8_lossy(login.account_name_bytes()).to_string());
            }
        }
        Opcode::CNFCharacterLogin => {
            let Some(mob) = Mob::decode(frame.get(HEADER_SIZE + 4..).unwrap_or_default()).ok()
            else {
                return;
            };
            achados.personagem = Some(String::from_utf8_lossy(nul_trimmed(&mob.name)).to_string());
            achados.slot = frame
                .get(HEADER_SIZE + 1028..HEADER_SIZE + 1030)
                .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]));
            achados.score = Some(score_do_mob(&mob));
        }
        Opcode::UpdateScore => {
            if let Some(score) = score_do_update(frame) {
                achados.score = Some(score);
            }
        }
        _ => {}
    }
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
            Some(format!(
                "mob={} {}",
                String::from_utf8_lossy(nul_trimmed(&mob.name)),
                score_texto(&score_do_mob(&mob))
            ))
        }
        Opcode::UpdateScore => Some(score_texto(&score_do_update(frame)?)),
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

/// `Score` do `STRUCT_MOB` do `CNFCharacterLogin` (crítico/resist vêm da cauda do mob).
fn score_do_mob(mob: &Mob) -> Score {
    let s = mob.current_score;
    Score {
        nivel: s.level,
        hp: s.hp,
        max_hp: s.max_hp,
        mp: s.mp,
        max_mp: s.max_mp,
        forca: s.strength,
        inteligencia: s.intelligence,
        destreza: s.dexterity,
        constituicao: s.constitution,
        chaos: s.chaos_rate,
        dano: s.damage,
        critico: mob.critical,
        resist: [
            mob.resist[0] as u8,
            mob.resist[1] as u8,
            mob.resist[2] as u8,
            mob.resist[3] as u8,
        ],
        curr_hp: s.hp,
        curr_mp: s.mp,
    }
}

/// `STRUCT_SCORE` do `MSG_UpdateScore` (protocolo.md §6: Score@0, Critical@48, Resist@118,
/// `CurrHp`@124, `CurrMp`@128).
fn score_do_update(frame: &[u8]) -> Option<Score> {
    let body = frame.get(HEADER_SIZE..)?;
    if body.len() < 132 {
        return None;
    }
    let i32_at =
        |off: usize| i32::from_le_bytes([body[off], body[off + 1], body[off + 2], body[off + 3]]);
    let i16_at = |off: usize| i16::from_le_bytes([body[off], body[off + 1]]);
    Some(Score {
        nivel: i32_at(0),
        max_hp: i32_at(16),
        max_mp: i32_at(20),
        hp: i32_at(24),
        mp: i32_at(28),
        forca: i16_at(32),
        inteligencia: i16_at(34),
        destreza: i16_at(36),
        constituicao: i16_at(38),
        dano: i32_at(8),
        chaos: body[15],
        critico: body[48],
        resist: [body[118], body[119], body[120], body[121]],
        curr_hp: i32_at(124),
        curr_mp: i32_at(128),
    })
}

fn score_texto(s: &Score) -> String {
    format!(
        "nivel={} hp={}/{} mp={}/{} for={} int={} des={} con={} dano={} chaos={} critico={} resist={:?} curr_hp={} curr_mp={}",
        s.nivel,
        s.hp,
        s.max_hp,
        s.mp,
        s.max_mp,
        s.forca,
        s.inteligencia,
        s.destreza,
        s.constituicao,
        s.dano,
        s.chaos,
        s.critico,
        s.resist,
        s.curr_hp,
        s.curr_mp
    )
}

/// Compara o personagem da captura com `characters` + `character_scores`.
///
/// Devolve o número de divergências; cada uma sai como `campo: captura=… banco=… DIVERGENTE`.
/// O nível do banco é 1-based e vai para o fio como `level - 1` (`wire_level`).
fn comparar(achados: &Achados, dsn: &str) -> Result<usize, String> {
    let Some(personagem) = achados.personagem.as_deref() else {
        println!("comparação: captura sem _MSG_CNFCharacterLogin");
        return Ok(0);
    };
    let Some(score) = achados.score else {
        return Err("captura sem STRUCT_SCORE".to_string());
    };
    let Some(conta) = achados.conta.as_deref() else {
        return Err("captura sem _MSG_AccountLogin".to_string());
    };
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    let backend = rt
        .block_on(PgBackend::connect(dsn, PoolConfig::local()))
        .map_err(|e| format!("conectar no banco: {e}"))?;
    let conta = rt
        .block_on(backend.find_by_login(conta))
        .map_err(|e| format!("conta: {e}"))?;
    let personagens = rt
        .block_on(backend.list_by_account(conta.id))
        .map_err(|e| format!("roster: {e}"))?;
    let slot = achados.slot.map(i64::from);
    let Some(alvo) = personagens
        .iter()
        .find(|c| c.name == personagem && slot.is_none_or(|s| c.slot == s))
    else {
        return Err(format!(
            "personagem {personagem} (slot {:?}) não existe na conta {}",
            achados.slot, conta.login
        ));
    };
    let progressao = rt
        .block_on(backend.progression(alvo.id))
        .map_err(|e| format!("progressão: {e}"))?;
    let esperado = Score {
        nivel: wire_level(progressao.level),
        hp: i32::try_from(progressao.hp).unwrap_or(i32::MAX),
        max_hp: i32::try_from(progressao.hp_max).unwrap_or(i32::MAX),
        mp: i32::try_from(progressao.mp).unwrap_or(i32::MAX),
        max_mp: i32::try_from(progressao.mp_max).unwrap_or(i32::MAX),
        forca: i16::try_from(progressao.strength).unwrap_or(i16::MAX),
        inteligencia: i16::try_from(progressao.intelligence).unwrap_or(i16::MAX),
        destreza: i16::try_from(progressao.dexterity).unwrap_or(i16::MAX),
        constituicao: i16::try_from(progressao.constitution).unwrap_or(i16::MAX),
        chaos: u8::try_from((alvo.cp + 75).clamp(0, 255)).unwrap_or(0),
        ..score
    };

    println!(
        "banco: {} id={} slot={} nivel={} (fio {}) hp={}/{} mp={}/{} for={} int={} des={} con={} cp={}",
        alvo.name,
        alvo.id,
        alvo.slot,
        progressao.level,
        esperado.nivel,
        esperado.hp,
        esperado.max_hp,
        esperado.mp,
        esperado.max_mp,
        esperado.forca,
        esperado.inteligencia,
        esperado.destreza,
        esperado.constituicao,
        alvo.cp
    );

    let mut divergencias = 0usize;
    let mut checar = |campo: &str, captura: i64, banco: i64| {
        let ok = captura == banco;
        println!(
            "{campo}: captura={captura} banco={banco} {}",
            if ok { "OK" } else { "DIVERGENTE" }
        );
        if !ok {
            divergencias += 1;
        }
    };
    checar("nivel", i64::from(score.nivel), i64::from(esperado.nivel));
    checar("hp", i64::from(score.hp), i64::from(esperado.hp));
    checar(
        "hp_max",
        i64::from(score.max_hp),
        i64::from(esperado.max_hp),
    );
    checar("mp", i64::from(score.mp), i64::from(esperado.mp));
    checar(
        "mp_max",
        i64::from(score.max_mp),
        i64::from(esperado.max_mp),
    );
    checar("forca", i64::from(score.forca), i64::from(esperado.forca));
    checar(
        "inteligencia",
        i64::from(score.inteligencia),
        i64::from(esperado.inteligencia),
    );
    checar(
        "destreza",
        i64::from(score.destreza),
        i64::from(esperado.destreza),
    );
    checar(
        "constituicao",
        i64::from(score.constituicao),
        i64::from(esperado.constituicao),
    );
    checar("chaos", i64::from(score.chaos), i64::from(esperado.chaos));
    Ok(divergencias)
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
        let texto = decodificar(&frame_de_login(), &mut Achados::default())
            .expect("frame do próprio parser");
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
        let (frames, falhas, achados) = resultado.expect("sessão legível");
        assert_eq!((frames, falhas), (2, 0));
        assert_eq!(achados.conta.as_deref(), Some("admin"));
    }

    #[test]
    fn update_score_le_os_offsets_do_send_score() {
        let mut frame = vec![0u8; 152];
        frame[0..2].copy_from_slice(&152u16.to_le_bytes());
        frame[4..6].copy_from_slice(&Opcode::UpdateScore.wire().to_le_bytes());
        // Corpo: Score@0, Critical@48, Resist@118, CurrHp@124, CurrMp@128.
        frame[12..16].copy_from_slice(&42i32.to_le_bytes()); // level
        frame[12 + 16..12 + 20].copy_from_slice(&500i32.to_le_bytes()); // max_hp
        frame[12 + 20..12 + 24].copy_from_slice(&200i32.to_le_bytes()); // max_mp
        frame[12 + 24..12 + 28].copy_from_slice(&480i32.to_le_bytes()); // hp
        frame[12 + 28..12 + 32].copy_from_slice(&150i32.to_le_bytes()); // mp
        frame[12 + 32..12 + 34].copy_from_slice(&31i16.to_le_bytes()); // strength
        frame[12 + 34..12 + 36].copy_from_slice(&12i16.to_le_bytes()); // intelligence
        frame[12 + 36..12 + 38].copy_from_slice(&17i16.to_le_bytes()); // dexterity
        frame[12 + 38..12 + 40].copy_from_slice(&23i16.to_le_bytes()); // constitution
        frame[12 + 15] = 80; // chaos
        frame[12 + 48] = 7; // critical
        frame[12 + 118..12 + 122].copy_from_slice(&[1, 2, 3, 4]); // resist
        frame[12 + 124..12 + 128].copy_from_slice(&470i32.to_le_bytes()); // curr_hp
        frame[12 + 128..12 + 132].copy_from_slice(&140i32.to_le_bytes()); // curr_mp

        let score = score_do_update(&frame).expect("corpo de 140 B");
        assert_eq!(score.nivel, 42);
        assert_eq!((score.hp, score.max_hp), (480, 500));
        assert_eq!((score.mp, score.max_mp), (150, 200));
        assert_eq!(
            (
                score.forca,
                score.inteligencia,
                score.destreza,
                score.constituicao
            ),
            (31, 12, 17, 23)
        );
        assert_eq!(score.chaos, 80);
        assert_eq!(score.critico, 7);
        assert_eq!(score.resist, [1, 2, 3, 4]);
        assert_eq!((score.curr_hp, score.curr_mp), (470, 140));
    }

    #[test]
    fn frame_truncado_vira_erro_e_nao_panic() {
        let frame = frame_de_login();
        let texto =
            decodificar(&frame[..5], &mut Achados::default()).expect_err("5 bytes não são frame");
        assert!(
            texto.contains("codec") || texto.contains("header"),
            "{texto}"
        );
    }
}

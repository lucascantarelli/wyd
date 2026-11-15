//! Comparação medida entre o painel de recusa que o jogador **vê** (exclusão de personagem com
//! senha errada) e o painel da recusa de login — no canal real (`127.0.0.1:8281`).
//!
//! Ponto de partida do relato: "usuário/senha errada não informa nada no cliente", enquanto a
//! senha errada **na exclusão** aparece. Se o painel é o mesmo objeto de fio (`0x0101`, 140 B,
//! `ID_SCENE`), a diferença que explica o silêncio está na sequência/lifecycle — e é isso que este
//! teste mede e trava:
//!
//! | campo | referência visível (delete) | login falho (alvo) |
//! |---|---|---|
//! | `Size@0` | 140 | 140 |
//! | `Type@4` | `0x0101` | `0x0101` |
//! | `id@8` | 30000 | 30000 |
//! | texto@12 | frase NUL-padded | frase NUL-padded |
//! | frame anterior | **um** `*Fail` da operação (`0x0113`) | **um** `*Fail` (`0x0119`) |
//! | depois do painel | conexão **aberta** (laço do canal) | aberta por até 2 s (não FIN imediato) |
//!
//! Gated por `WYD_LIVE_CPSOCK` + DSN.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use wyd_db::{Accounts, Characters, PgBackend, PoolConfig};
use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{
    AccountLogin, AccountSecure, CreateCharacter, DeleteCharacter, Header, WirePayload,
};

const SENHA: &str = "qa12345678";
const PIN: &str = "2468";
const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/painel-login");

fn runtime() -> Arc<tokio::runtime::Runtime> {
    Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("runtime"),
    )
}

/// Nome de conta/campo de 16 B: prefixo + 8 dígitos.
fn sufixo() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{:08}", nanos % 100_000_000)
}

fn quadro(opcode: Opcode, tamanho: usize, tick: u32) -> Vec<u8> {
    let mut frame = vec![0u8; tamanho];
    Header {
        size: tamanho as u16,
        key_word: 11,
        checksum: 0,
        message_type: opcode.wire(),
        id: 1,
        client_tick: tick,
    }
    .encode(&mut frame)
    .expect("header");
    frame
}

fn fechar(mut frame: Vec<u8>) -> Vec<u8> {
    encode_in_place(&mut frame, 11).expect("ofuscação");
    frame
}

fn frame_login(conta: &str, senha: &str) -> Vec<u8> {
    let mut nome = [0u8; 16];
    let n = conta.len().min(16);
    nome[..n].copy_from_slice(&conta.as_bytes()[..n]);
    let mut senha_bytes = [0u8; 12];
    let s = senha.len().min(12);
    senha_bytes[..s].copy_from_slice(&senha.as_bytes()[..s]);
    let mut frame = quadro(Opcode::AccountLogin, AccountLogin::WIRE_LEN, 1);
    AccountLogin {
        account_password: senha_bytes,
        account_name: nome,
        zero: [0u8; 52],
        client_version: 12_000,
        db_need_save: 1,
        adapter_name: [0; 4],
    }
    .encode_into(&mut frame)
    .expect("corpo do login");
    fechar(frame)
}

fn frame_pin(pin: &str) -> Vec<u8> {
    let mut token = [0u8; 6];
    let b = pin.len().min(6);
    token[..b].copy_from_slice(&pin.as_bytes()[..b]);
    let mut frame = quadro(Opcode::AccountSecure, AccountSecure::WIRE_LEN, 2);
    AccountSecure {
        numeric_token: token,
        unknown: [0u8; 10],
        change_numeric: 0,
    }
    .encode_into(&mut frame)
    .expect("corpo do PIN");
    fechar(frame)
}

fn frame_criacao(slot: i32, nome: &str) -> Vec<u8> {
    let mut campo = [0u8; 16];
    let n = nome.len().min(16);
    campo[..n].copy_from_slice(&nome.as_bytes()[..n]);
    let mut frame = quadro(Opcode::CreateCharacter, CreateCharacter::WIRE_LEN, 3);
    CreateCharacter {
        slot,
        mob_name: campo,
        mob_class: 0,
    }
    .encode_into(&mut frame)
    .expect("corpo da criação");
    fechar(frame)
}

fn frame_delete(slot: i32, nome: &str, senha: &str) -> Vec<u8> {
    let mut campo = [0u8; 16];
    let n = nome.len().min(16);
    campo[..n].copy_from_slice(&nome.as_bytes()[..n]);
    let mut segredo = [0u8; 12];
    let s = senha.len().min(12);
    segredo[..s].copy_from_slice(&senha.as_bytes()[..s]);
    let mut frame = quadro(Opcode::DeleteCharacter, DeleteCharacter::WIRE_LEN, 4);
    DeleteCharacter {
        slot,
        mob_name: campo,
        password: segredo,
    }
    .encode_into(&mut frame)
    .expect("corpo do delete");
    fechar(frame)
}

fn conectar(endereco: &str) -> TcpStream {
    let stream = TcpStream::connect(endereco).expect("conecta no canal");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("timeout");
    stream
}

/// Devolve `(opcode, frame)`; `None` quando o canal fecha ou cala.
fn ler(stream: &mut TcpStream) -> Option<(Opcode, Vec<u8>)> {
    let mut head = [0u8; HEADER_SIZE];
    stream.read_exact(&mut head).ok()?;
    let size = usize::from(u16::from_le_bytes([head[0], head[1]]));
    let mut frame = vec![0u8; size];
    frame[..HEADER_SIZE].copy_from_slice(&head);
    if size > HEADER_SIZE {
        stream.read_exact(&mut frame[HEADER_SIZE..]).ok()?;
    }
    decode_in_place(&mut frame).ok()?;
    let header = Header::decode(&frame).ok()?;
    Opcode::from_wire(header.message_type).map(|opcode| (opcode, frame))
}

fn hex(frame: &[u8]) -> String {
    frame.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Campos medidos de um painel de 140 B já desofuscado.
struct Painel {
    size: u16,
    tipo: u16,
    id: u16,
    texto: String,
    texto_offset: usize,
    pad_ate_o_fim: usize,
}

fn medir_painel(frame: &[u8]) -> Painel {
    let header = Header::decode(frame).expect("header do painel");
    let corpo = &frame[HEADER_SIZE..];
    let fim_texto = corpo
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(corpo.len());
    Painel {
        size: header.size,
        tipo: header.message_type,
        id: header.id,
        texto: String::from_utf8_lossy(&corpo[..fim_texto]).to_string(),
        texto_offset: HEADER_SIZE,
        pad_ate_o_fim: corpo.len() - fim_texto,
    }
}

fn despejar(nome: &str, linhas: &[String]) {
    std::fs::create_dir_all(RELATORIO).expect("tmp/painel-login");
    std::fs::write(format!("{RELATORIO}/{nome}.txt"), linhas.join("\n") + "\n").expect("dump");
}

/// O painel do canal continua aberto depois do texto? `true` = sem FIN imediato.
fn conexao_continua_aberta(stream: &mut TcpStream) -> bool {
    stream
        .set_read_timeout(Some(Duration::from_millis(600)))
        .expect("timeout curto");
    let mut lixo = [0u8; 256];
    match stream.read(&mut lixo) {
        Ok(0) => false,
        Ok(_) => true,
        Err(_) => true,
    }
}

#[test]
fn painel_do_login_tem_a_mesma_forma_do_painel_da_exclusao() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("painel no canal") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );

    // Conta com senha + PIN + um personagem no slot 1.
    let sufixo = sufixo();
    // Conta **fixa** de QA (reuso, não conta nova por execução): `accounts` fica igual entre
    // rodadas; o preparo zera roster, senha e PIN — ver `common::preparar_conta_de_teste`.
    let conta = "qapainellive".to_string();
    let nome = "PnLive".to_string();
    common::preparar_conta_de_teste(&rt, &backend, &conta, SENHA, PIN);

    // PIN cadastrado pela primeira conexão (fluxo real).
    let mut sessao = conectar(&endereco);
    sessao.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    sessao
        .write_all(&frame_login(&conta, SENHA))
        .expect("login");
    assert_eq!(
        ler(&mut sessao).expect("CNF do login").0,
        Opcode::CNFAccountLogin
    );
    sessao.write_all(&frame_pin(PIN)).expect("PIN");
    assert_eq!(
        ler(&mut sessao).expect("aceite do PIN").0,
        Opcode::AccountSecure,
        "PIN inicial cadastrado"
    );
    sessao.write_all(&frame_criacao(1, &nome)).expect("criação");
    assert_eq!(
        ler(&mut sessao).expect("CNFNewCharacter").0,
        Opcode::CNFNewCharacter
    );

    // ---- Referência: exclusão com senha errada (o jogador vê este painel) ----
    let delete_c2s = frame_delete(1, &nome, "senha-errada");
    sessao.write_all(&delete_c2s).expect("delete");
    let (opcode_fail_delete, fail_delete) = ler(&mut sessao).expect("`*Fail` do delete");
    assert_eq!(
        opcode_fail_delete,
        Opcode::DeleteCharacterFail,
        "um `*Fail` da operação antes do painel"
    );
    let (opcode_painel_delete, painel_delete) = ler(&mut sessao).expect("painel do delete");
    assert_eq!(opcode_painel_delete, Opcode::MessagePanel);
    let referencia = medir_painel(&painel_delete);
    let continua_no_delete = conexao_continua_aberta(&mut sessao);
    drop(sessao);

    // ---- Login falho: conta inexistente (mesma senha de forma) ----
    let inexistente = format!("qanx{}", sufixo);
    let mut s = conectar(&endereco);
    s.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    s.write_all(&frame_login(&inexistente, SENHA))
        .expect("login");
    let mut sequencia: Vec<(Opcode, Vec<u8>)> = Vec::new();
    while let Some(item) = ler(&mut s) {
        sequencia.push(item);
    }
    // Dump **antes** das asserções: o registro bruto é a prova, mesmo quando a sequência diverge.
    despejar(
        "comparacao",
        &[
            format!("conta={conta} personagem={nome}"),
            format!(
                "referencia_delete c2s size={} hex={}",
                delete_c2s.len(),
                hex(&delete_c2s)
            ),
            format!(
                "referencia_delete fail opcode=0x{:04x} size={} hex={}",
                Opcode::DeleteCharacterFail.wire(),
                fail_delete.len(),
                hex(&fail_delete)
            ),
            format!(
                "referencia_delete painel size={} tipo=0x{:04x} id={} texto_offset={} pad={} texto={:?} hex={}",
                referencia.size,
                referencia.tipo,
                referencia.id,
                referencia.texto_offset,
                referencia.pad_ate_o_fim,
                referencia.texto,
                hex(&painel_delete)
            ),
            format!(
                "login_falho sequencia={:?}",
                sequencia
                    .iter()
                    .map(|(opcode, frame)| format!("0x{:04x}({}B)", opcode.wire(), frame.len()))
                    .collect::<Vec<_>>()
            ),
        ],
    );
    // A recusa de credencial é **um** frame: o único que a etapa de login do 7662 desenha
    // (`0x4b4082` → `push 0xfa0; push 0x109aab8; call 0x404109`). Nada de roster nem
    // `CNFAccountLogin` — a variante com CNF de roster vazio foi medida e rejeitada.
    assert_eq!(
        sequencia.len(),
        1,
        "a recusa é um único frame (sem roster e sem CNFAccountLogin)"
    );
    let (opcode_recusa, recusa) = &sequencia[0];
    assert_eq!(
        *opcode_recusa,
        Opcode::AlreadyPlaying,
        "0x011c: braço de mensagem da etapa de login"
    );
    assert_eq!(recusa.len(), HEADER_SIZE, "header-only (12 B)");
    assert_eq!(
        referencia.size, 140,
        "referência visível: painel de 140 B na exclusão"
    );
    assert!(!referencia.texto.is_empty(), "painel do delete tem texto");
    assert_eq!(
        referencia.texto.len() + referencia.pad_ate_o_fim,
        128,
        "campo de texto do delete = 128 B NUL-padded"
    );

    // ---- Lifecycle: o caso visível não fecha a conexão no painel ----
    assert!(
        continua_no_delete,
        "referência visível mantém a conexão aberta depois do painel"
    );
    let mut s2 = conectar(&endereco);
    s2.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    s2.write_all(&frame_login(&inexistente, SENHA))
        .expect("login");
    let _ = ler(&mut s2).expect("mensagem de recusa");
    assert!(
        conexao_continua_aberta(&mut s2),
        "login falho não pode fechar o soquete no instante do painel (o caso visível não fecha)"
    );

    drop(s2);

    // Devolve o banco ao estado inicial: o personagem de prova sai pelo `_MSG_DeleteCharacter`
    // (a conexão da sessão já foi fechada acima) — duas execuções seguidas deixam as mesmas
    // contagens de sessões e personagens.
    assert_eq!(
        common::remover_personagem(&endereco, &conta, SENHA, PIN, 1, &nome),
        Some(Opcode::CNFDeleteCharacter),
        "personagem de prova removido pelo protocolo"
    );
    let conta_id = rt
        .block_on(backend.find_by_login(&conta))
        .expect("conta da limpeza")
        .id;
    let roster = rt
        .block_on(backend.list_by_account(conta_id))
        .expect("roster depois da limpeza");
    assert!(roster.is_empty(), "roster volta a zero: {roster:?}");
    eprintln!(
        "PASS: recusa = [0x011c header-only] ({} frame, sem roster e sem CNF); referência visível (exclusão) mantém 0x011b + painel de 140 B; conexão aberta | limpeza: CNFDeleteCharacter roster=0",
        sequencia.len()
    );
}

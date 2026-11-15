//! Autenticação completa no CPSock, com banco e socket reais.
//!
//! Cobre, na ordem do cliente 7662: senha da conta (recusa com painel), PIN cadastrado
//! (recusa com painel e aceite), e exclusão de personagem exigindo a senha da conta.
//!
//! Gate duplo: `WYD_LIVE_CPSOCK` (canal) e DSN (`WYD_TEST_DSN`/`WYD_TEST_DATABASE_URL`/
//! `DATABASE_URL`). Sem qualquer um deles o teste **pula** com `SKIP:` explícito; com
//! `WYD_TEST_REQUIRED=1` a ausência de DSN falha, como nos demais testes de sistema.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use wyd_admin::auth::hash_password;
use wyd_db::{Accounts, Character, Characters, PgBackend, PoolConfig, Role};
use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
use wyd_protocol::payload::{nul_trimmed, AccountLogin, AccountSecure, DeleteCharacter};
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{Header, MessagePanel, WirePayload};
use wyd_server::auth::{Authenticator, PinOutcome};

const SENHA: &str = "qa12345678";
const PIN: &str = "2468";

fn runtime() -> Arc<tokio::runtime::Runtime> {
    Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("runtime"),
    )
}

fn sufixo() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{}", nanos % 100_000_000_000)
}

/// Frame completo com corpo escrito por `preencher` e ofuscação do cliente.
fn frame(opcode: Opcode, size: usize, key: u8, preencher: impl FnOnce(&mut [u8])) -> Vec<u8> {
    let mut frame = vec![0u8; size];
    Header {
        size: u16::try_from(size).expect("frame cabe em u16"),
        key_word: key,
        checksum: 0,
        message_type: opcode.wire(),
        id: 1,
        client_tick: 7,
    }
    .encode(&mut frame)
    .expect("cabeçalho cabe");
    preencher(&mut frame);
    encode_in_place(&mut frame, key).expect("ofuscação");
    frame
}

fn conectar(endereco: &str) -> TcpStream {
    let stream = TcpStream::connect(endereco).expect("canal aceita conexão");
    stream
        .set_read_timeout(Some(Duration::from_secs(8)))
        .expect("timeout");
    stream
}

/// Envia INITCODE + `_MSG_AccountLogin`; devolve o opcode da primeira resposta.
fn login(stream: &mut TcpStream, conta: &str, senha: &str) -> Option<Opcode> {
    let mut nome = [0u8; 16];
    let bytes = conta.as_bytes();
    let n = bytes.len().min(nome.len());
    nome[..n].copy_from_slice(&bytes[..n]);
    let mut segredo = [0u8; 12];
    let senha = senha.as_bytes();
    let n = senha.len().min(segredo.len());
    segredo[..n].copy_from_slice(&senha[..n]);
    let pacote = frame(Opcode::AccountLogin, AccountLogin::WIRE_LEN, 11, |buf| {
        AccountLogin {
            account_password: segredo,
            account_name: nome,
            zero: [0u8; 52],
            client_version: wyd_protocol::PROTOCOL_VERSION as i32,
            db_need_save: 1,
            adapter_name: [0; 4],
        }
        .encode_into(buf)
        .expect("corpo do login cabe");
    });
    stream.write_all(&INITCODE.to_le_bytes()).expect("INITCODE");
    stream.write_all(&pacote).expect("login");
    ler(stream).map(|(opcode, _)| opcode)
}

/// Lê um frame completo e o devolve já desofuscado. `None` = conexão fechada/timeout.
fn ler(stream: &mut TcpStream) -> Option<(Opcode, Vec<u8>)> {
    let mut head = [0u8; HEADER_SIZE];
    stream.read_exact(&mut head).ok()?;
    let header = Header::decode(&head).ok()?;
    let declarado = header.frame_len().ok()?;
    let mut frame = vec![0u8; declarado];
    frame[..HEADER_SIZE].copy_from_slice(&head);
    stream.read_exact(&mut frame[HEADER_SIZE..]).ok()?;
    decode_in_place(&mut frame).ok()?;
    let header = Header::decode(&frame).ok()?;
    let opcode = Opcode::from_wire(header.message_type)?;
    Some((opcode, frame))
}

fn texto_do_painel(frame: &[u8]) -> String {
    MessagePanel::decode(frame)
        .map(|painel| String::from_utf8_lossy(nul_trimmed(&painel.text)).to_string())
        .unwrap_or_default()
}

fn enviar_pin(stream: &mut TcpStream, pin: &str, troca: i32) {
    let bytes = pin.as_bytes();
    let mut token = [0u8; 6];
    token[..bytes.len().min(6)].copy_from_slice(&bytes[..bytes.len().min(6)]);
    let pacote = frame(Opcode::AccountSecure, AccountSecure::WIRE_LEN, 12, |buf| {
        AccountSecure {
            numeric_token: token,
            unknown: [0u8; 10],
            change_numeric: troca,
        }
        .encode_into(buf)
        .expect("corpo do PIN cabe");
    });
    stream.write_all(&pacote).expect("PIN");
}

fn enviar_delete(stream: &mut TcpStream, slot: i32, nome: &str, senha: &str) {
    let mut nome_wire = [0u8; 16];
    let bytes = nome.as_bytes();
    let n = bytes.len().min(nome_wire.len());
    nome_wire[..n].copy_from_slice(&bytes[..n]);
    let mut segredo = [0u8; 12];
    let bytes = senha.as_bytes();
    let n = bytes.len().min(segredo.len());
    segredo[..n].copy_from_slice(&bytes[..n]);
    let pacote = frame(
        Opcode::DeleteCharacter,
        DeleteCharacter::WIRE_LEN,
        13,
        |buf| {
            DeleteCharacter {
                slot,
                mob_name: nome_wire,
                password: segredo,
            }
            .encode_into(buf)
            .expect("corpo do delete cabe");
        },
    );
    stream.write_all(&pacote).expect("delete");
}

fn endereco() -> Option<String> {
    match std::env::var("WYD_LIVE_CPSOCK") {
        Ok(valor) if !valor.trim().is_empty() => Some(valor),
        _ => {
            eprintln!("SKIP: WYD_LIVE_CPSOCK ausente — autenticação no CPSock não testada");
            None
        }
    }
}

#[test]
fn autenticacao_completa_no_cpsock() {
    let Some(endereco) = endereco() else {
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("autenticação no CPSock") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );

    // Conta descartável com senha, personagem no slot 0 e PIN cadastrado.
    let sufixo = sufixo();
    let conta_login = format!("aa{sufixo}");
    let nome = format!("QAA{sufixo}");
    rt.block_on(backend.create_account(&conta_login, Role::Player))
        .expect("conta");
    let hash = hash_password(SENHA).expect("argon2id");
    rt.block_on(backend.set_credentials(&conta_login, &hash, Role::Player))
        .expect("senha");
    let conta = rt
        .block_on(backend.find_by_login(&conta_login))
        .expect("conta");
    let personagem = Character {
        id: 0,
        account_id: conta.id,
        slot: 0,
        name: nome.clone(),
        class: 0,
        level: 5,
        hp: 100,
        mp: 50,
        hp_max: 100,
        mp_max: 50,
        ..Character::default()
    };
    rt.block_on(backend.create_character(&personagem))
        .expect("personagem");
    let auth = Authenticator::new(Arc::clone(&backend), Arc::clone(&rt));
    assert_eq!(
        auth.secure_pin(&conta_login, PIN.as_bytes())
            .expect("cadastro"),
        PinOutcome::Registered,
        "primeiro PIN da conta é cadastrado"
    );

    // 1) Senha errada: **um** `_MSG_AlreadyPlaying` header-only — a forma que a etapa de login
    // do 7662 desenha (`0x4b4082`), sem roster e sem `CNFAccountLogin`
    // (`docs/rust/evidencias/login-tela-handler-2026-11-14.md`).
    let mut sessao = conectar(&endereco);
    assert_eq!(
        login(&mut sessao, &conta_login, "senha-errada"),
        Some(Opcode::AlreadyPlaying),
        "a recusa manda a mensagem que a tela de login desenha"
    );
    drop(sessao);

    // 2) Senha certa, PIN errado: `AccountSecureFail` + painel.
    let mut sessao = conectar(&endereco);
    assert_eq!(
        login(&mut sessao, &conta_login, SENHA),
        Some(Opcode::CNFAccountLogin),
        "senha certa confirma o login"
    );
    enviar_pin(&mut sessao, "0000", 0);
    let (opcode, _) = ler(&mut sessao).expect("recusa do PIN");
    assert_eq!(opcode, Opcode::AccountSecureFail);
    let (opcode, painel) = ler(&mut sessao).expect("painel da recusa de PIN");
    assert_eq!(opcode, Opcode::MessagePanel);
    let texto = texto_do_painel(&painel);
    assert!(
        texto.to_lowercase().contains("pin"),
        "painel precisa citar o PIN: {texto:?}"
    );
    drop(sessao);

    // 3) Senha certa + PIN certo: sessão aberta.
    let mut sessao = conectar(&endereco);
    assert_eq!(
        login(&mut sessao, &conta_login, SENHA),
        Some(Opcode::CNFAccountLogin)
    );
    enviar_pin(&mut sessao, PIN, 0);
    let (opcode, _) = ler(&mut sessao).expect("aceite do PIN");
    assert_eq!(opcode, Opcode::AccountSecure, "PIN certo abre a sessão");

    // 4) Exclusão com senha errada: recusa + painel; personagem continua vivo.
    enviar_delete(&mut sessao, 0, &nome, "senha-errada");
    let (opcode, _) = ler(&mut sessao).expect("recusa do delete");
    assert_eq!(
        opcode,
        Opcode::DeleteCharacterFail,
        "senha errada não apaga personagem"
    );
    let (opcode, painel) = ler(&mut sessao).expect("painel do delete");
    assert_eq!(opcode, Opcode::MessagePanel);
    assert!(
        !texto_do_painel(&painel).is_empty(),
        "recusa de exclusão precisa de texto"
    );
    let vivos = rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster");
    assert_eq!(vivos.len(), 1, "personagem não pode sumir com senha errada");

    // 5) Exclusão com a senha da conta: roster volta vazio.
    enviar_delete(&mut sessao, 0, &nome, SENHA);
    let (opcode, _) = ler(&mut sessao).expect("confirmação do delete");
    assert_eq!(opcode, Opcode::CNFDeleteCharacter);
    let vivos = rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster depois");
    assert!(vivos.is_empty(), "personagem excluído sai do roster");
}

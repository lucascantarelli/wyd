//! Criação de personagem no canal real: confirmação da segunda senha e nível de fronteira.
//!
//! Duas perguntas do relato, medidas no canal 1 vivo (`127.0.0.1:8281`):
//!
//! 1. **Segunda senha na criação**: a confirmação divergente tem de ser recusada com painel e
//!    **nada** pode ser gravado. O ponto de comparação no servidor é o `_MSG_AccountSecure`
//!    (`0x0FDE`) que o cliente manda na etapa de seleção: até esta rodada esse braço respondia
//!    `0x0FDE` sem comparar o token com `accounts.pin_hash` — era a raiz de "aceita qualquer
//!    coisa". Agora ele passa pelo mesmo `secure_pin` do login e a divergência encerra a conexão
//!    antes de qualquer `_MSG_CreateCharacter` ser processado.
//!
//! 2. **Nível**: o banco é 1-based e o fio é 0-based (`wire_level = banco − 1`), em todos os
//!    portadores: `Selchar` no `CNFAccountLogin` (`0x010A`, `sel@20`, `Score[0].Level@80`),
//!    `Selchar` no `CNFNewCharacter` (`0x0110`, `sel@4`) e `STRUCT_MOB` no `CNFCharacterLogin`
//!    (`0x0114`, `mob@4`, `BaseScore@44`, `CurrentScore@92`). Este teste mede as duas pontas nos
//!    níveis de fronteira 1, 2, 399 e 400.
//!
//! Offsets vêm do legado (`Basedef.h`, ver `nivel_wire.rs`); o codec do porte não é usado como
//! prova. Gated por `WYD_LIVE_CPSOCK` + `WYD_TEST_DSN`/`DATABASE_URL`.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use wyd_admin::auth::hash_secret;
use wyd_db::{Accounts, Character, Characters, PgBackend, PoolConfig, Role};
use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{
    motivo::CodigoRecusa, AccountLogin, AccountSecure, CharacterLogin, CreateCharacter, Header,
    MessagePanel, WirePayload,
};

const SENHA: &str = "qa12345678";
const PIN: &str = "2468";
const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/criacao-canal");
/// `STRUCT_SELCHAR.Score[4]` e `STRUCT_SCORE.Level` (`Basedef.h:1002`, `:524`).
const SELCHAR_SCORE: usize = 80;
/// `MSG_DBCNFAccountLogin.sel` (`Basedef.h:1413`).
const SEL_AT_ACCOUNT: usize = 20;
/// `MSG_CNFNewCharacter.sel` (cliente lê `frame+0x10`, `docs/cliente/pacotes.md`).
const SEL_AT_NEW: usize = 4;
/// `MSG_CNFCharacterLogin.mob` (`short PosX, PosY` antes).
const MOB_AT_LOGIN: usize = 4;
/// `STRUCT_MOB`: `BaseScore`/`CurrentScore`.
const MOB_BASE_SCORE: usize = 44;
const MOB_CURRENT_SCORE: usize = 92;

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
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{:06}", nanos % 1_000_000)
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
        mob_class: 1,
    }
    .encode_into(&mut frame)
    .expect("corpo da criação");
    fechar(frame)
}

fn frame_selecao(slot: i32) -> Vec<u8> {
    let mut frame = quadro(Opcode::CharacterLogin, CharacterLogin::WIRE_LEN, 4);
    CharacterLogin { slot, force: 0 }
        .encode_into(&mut frame)
        .expect("corpo da seleção");
    fechar(frame)
}

fn conectar(endereco: &str) -> TcpStream {
    let stream = TcpStream::connect(endereco).expect("conecta no canal");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("timeout");
    stream
}

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

fn i32_em(frame: &[u8], offset: usize) -> i32 {
    let inicio = HEADER_SIZE + offset;
    i32::from_le_bytes([
        frame[inicio],
        frame[inicio + 1],
        frame[inicio + 2],
        frame[inicio + 3],
    ])
}

fn hex(frame: &[u8]) -> String {
    frame.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn despejar(nome: &str, linhas: &[String]) {
    std::fs::create_dir_all(RELATORIO).expect("tmp/criacao-canal");
    std::fs::write(format!("{RELATORIO}/{nome}.txt"), linhas.join("\n") + "\n").expect("dump");
}

fn nova_conta(rt: &tokio::runtime::Runtime, backend: &PgBackend, conta: &str) -> i64 {
    rt.block_on(backend.create_account(conta, Role::Player))
        .expect("conta");
    let hash = hash_secret(SENHA).expect("argon2id");
    rt.block_on(backend.set_credentials(conta, &hash, Role::Player))
        .expect("credencial");
    rt.block_on(backend.find_by_login(conta)).expect("conta").id
}

/// Conecta, faz o login e o PIN; devolve o stream pronto para a seleção.
fn entrar(endereco: &str, conta: &str, pin: &str) -> TcpStream {
    let mut s = conectar(endereco);
    s.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    s.write_all(&frame_login(conta, SENHA)).expect("login");
    assert_eq!(
        ler(&mut s).expect("CNF do login").0,
        Opcode::CNFAccountLogin,
        "login com senha correta"
    );
    s.write_all(&frame_pin(pin)).expect("PIN");
    assert_eq!(
        ler(&mut s).expect("aceite do PIN").0,
        Opcode::AccountSecure,
        "PIN correto abre a seleção"
    );
    s
}

#[test]
fn criacao_por_protocolo_confere_pin_e_nivel() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("criação no canal") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );
    let sufixo = sufixo();
    let mut linhas: Vec<String> = Vec::new();

    // ---- 1. Confirmação divergente: recusa com painel e nada gravado ----
    let conta = format!("qacr{}", sufixo);
    let conta_id = nova_conta(&rt, &backend, &conta);
    let mut s = entrar(&endereco, &conta, PIN); // cadastra o PIN 2468
    let divergente = frame_pin("0000");
    s.write_all(&divergente).expect("confirmação divergente");
    let (opcode, recusa) = ler(&mut s).expect("`0x0FDF` da confirmação");
    assert_eq!(
        opcode,
        Opcode::AccountSecureFail,
        "confirmação divergente recebe `_MSG_AccountSecureFail`"
    );
    assert_eq!(
        CodigoRecusa::from_byte(recusa.get(HEADER_SIZE).copied()),
        CodigoRecusa::PinIncorreto,
        "motivo `PinIncorreto` no trailer"
    );
    let (opcode, painel) = ler(&mut s).expect("painel da confirmação");
    assert_eq!(opcode, Opcode::MessagePanel);
    let texto = String::from_utf8_lossy(wyd_protocol::nul_trimmed(
        &MessagePanel::decode(&painel).unwrap().text,
    ))
    .to_string();
    assert!(
        texto.to_lowercase().contains("senhas") || texto.to_lowercase().contains("pin"),
        "painel explica a divergência: {texto:?}"
    );
    // Criação enviada depois da recusa não pode ser processada: a conexão já foi encerrada.
    let nome_divergente = format!("Pd{}", &sufixo[..4]);
    let _ = s.write_all(&frame_criacao(2, &nome_divergente));
    let mut resto = Vec::new();
    let _ = s.read(&mut resto);
    let roster = rt
        .block_on(backend.list_by_account(conta_id))
        .expect("roster");
    assert!(
        roster.is_empty(),
        "confirmação divergente não pode gravar personagem: {roster:?}"
    );
    eprintln!(
        "PASS(pin_divergente): AccountSecureFail {} [{}] | MessagePanel {} | roster={}",
        recusa.len(),
        hex(&recusa),
        painel.len(),
        roster.len()
    );
    linhas.extend([
        format!("caso=pin_divergente conta={conta}"),
        format!("c2s size={} hex={}", divergente.len(), hex(&divergente)),
        format!("s2c size={} hex={}", recusa.len(), hex(&recusa)),
        format!("s2c size={} hex={}", painel.len(), hex(&painel)),
        format!("texto={texto}"),
        format!("roster_depois={}", roster.len()),
    ]);

    // ---- 2. Confirmação igual: cria e grava, nível do banco = 1 ----
    let mut s = entrar(&endereco, &conta, PIN);
    let nome = format!("Pc{}", &sufixo[..4]);
    s.write_all(&frame_criacao(2, &nome)).expect("criação");
    let (opcode, criado) = ler(&mut s).expect("CNFNewCharacter");
    assert_eq!(opcode, Opcode::CNFNewCharacter, "criação confirmada");
    let fio_criacao = i32_em(&criado, SEL_AT_NEW + SELCHAR_SCORE);
    let roster = rt
        .block_on(backend.list_by_account(conta_id))
        .expect("roster");
    let banco = roster
        .iter()
        .find(|personagem| personagem.name == nome)
        .expect("personagem no banco");
    assert_eq!(banco.level, 1, "banco nasce nível 1");
    assert_eq!(
        fio_criacao, 0,
        "`CNFNewCharacter` manda `Level` 0-based (banco 1 → fio 0)"
    );
    eprintln!(
        "PASS(pin_igual): CNFNewCharacter {} B Level@{}={} | banco level={}",
        criado.len(),
        HEADER_SIZE + SEL_AT_NEW + SELCHAR_SCORE,
        fio_criacao,
        banco.level
    );
    linhas.extend([
        format!("caso=pin_igual conta={conta} personagem={nome}"),
        format!(
            "c2s size={} hex={}",
            CreateCharacter::WIRE_LEN,
            hex(&frame_criacao(2, &nome))
        ),
        format!(
            "s2c size={} hex={}",
            criado.len(),
            hex(&criado[..HEADER_SIZE + 100])
        ),
        format!("banco_level={} fio_level={fio_criacao}", banco.level),
    ]);
    drop(s);
    std::thread::sleep(Duration::from_millis(500));

    // ---- 3. Fronteiras de nível: banco 2, 399 e 400 ----
    let conta_borda = format!("qab{}", sufixo);
    let conta_borda_id = nova_conta(&rt, &backend, &conta_borda);
    for (slot, nivel) in [(0_i64, 400_i64), (1, 399), (2, 2)] {
        let personagem = Character {
            id: 0,
            account_id: conta_borda_id,
            slot,
            name: format!("Qb{slot}{}", &sufixo[..3]),
            class: 1,
            level: nivel,
            hp: 900,
            mp: 300,
            hp_max: 900,
            mp_max: 300,
            pos_x: 2096,
            pos_y: 2096,
            ..Character::default()
        };
        rt.block_on(backend.create_character(&personagem))
            .expect("personagem de borda");
    }

    // Login: o roster (`0x010A`) carrega o `Selchar` dos três slots.
    let mut s = conectar(&endereco);
    s.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    s.write_all(&frame_login(&conta_borda, SENHA))
        .expect("login");
    let (opcode, cnf) = ler(&mut s).expect("CNFAccountLogin");
    assert_eq!(opcode, Opcode::CNFAccountLogin);
    let fio_roster: Vec<i32> = (0..3)
        .map(|slot| i32_em(&cnf, SEL_AT_ACCOUNT + SELCHAR_SCORE + slot * 48))
        .collect();
    assert_eq!(
        fio_roster,
        vec![399, 398, 1],
        "roster 0-based: banco 400→399, 399→398, 2→1"
    );
    s.write_all(&frame_pin(PIN)).expect("PIN");
    assert_eq!(ler(&mut s).expect("PIN aceito").0, Opcode::AccountSecure);

    // Seleção do personagem de nível 400: `STRUCT_MOB` do `0x0114`.
    s.write_all(&frame_selecao(0)).expect("seleção");
    let (opcode, login_char) = ler(&mut s).expect("CNFCharacterLogin");
    assert_eq!(opcode, Opcode::CNFCharacterLogin, "seleção aceita");
    let base = i32_em(&login_char, MOB_AT_LOGIN + MOB_BASE_SCORE);
    let atual = i32_em(&login_char, MOB_AT_LOGIN + MOB_CURRENT_SCORE);
    assert_eq!(base, 399, "`BaseScore.Level` 0-based (banco 400 → 399)");
    assert_eq!(atual, 399, "`CurrentScore.Level` 0-based (banco 400 → 399)");
    eprintln!(
        "PASS(nivel_bordas): roster fio={fio_roster:?} banco=[400, 399, 2] | CNFCharacterLogin BaseScore@{}={base} CurrentScore@{}={atual}",
        HEADER_SIZE + MOB_AT_LOGIN + MOB_BASE_SCORE,
        HEADER_SIZE + MOB_AT_LOGIN + MOB_CURRENT_SCORE
    );
    linhas.extend([
        format!("caso=nivel_bordas conta={conta_borda} banco=[400,399,2]"),
        format!("s2c size={} roster_fio={fio_roster:?}", cnf.len()),
        format!(
            "s2c size={} base_score_level={base} current_score_level={atual}",
            login_char.len()
        ),
        format!("hex_cnf={}", hex(&cnf[..HEADER_SIZE + 140])),
        format!(
            "hex_login_char={}",
            hex(&login_char[..HEADER_SIZE + MOB_AT_LOGIN + MOB_CURRENT_SCORE + 8])
        ),
    ]);
    despejar("criacao", &linhas);
}

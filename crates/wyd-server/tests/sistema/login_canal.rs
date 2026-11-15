//! Recusas de login **no canal real** (`127.0.0.1:8281`), medidas nos bytes crus.
//!
//! Complementa `sistema_login` (que sobe um `TcpListener` em processo): aqui o cliente TCP é
//! um soquete comum falando com o canal que o jogador usa, com a mesma conta/senha do banco.
//!
//! O que é medido e afirmado:
//!
//! 1. senha errada → **um** `_MSG_AlreadyPlaying` (`0x011C`, header-only 12 B) — o único braço
//!    que a etapa de login do 7662 desenha (`0x4b4082` → `push 0xfa0; push 0x109aab8; call
//!    0x404109`), sem roster, sem `CNFAccountLogin` e com a conexão aberta até o cliente fechar
//!    (`docs/rust/evidencias/login-tela-handler-2026-11-14.md`);
//! 2. conta inexistente → a **mesma** sequência (não revela se o login existe);
//! 3. PIN errado em conta com PIN → `_MSG_AccountSecureFail` (`0x0FDF`) + painel, e nada é
//!    gravado (roster igual);
//! 4. PIN correto → `_MSG_AccountSecure` (`0x0FDE`) e a criação de personagem conclui, com o
//!    personagem no banco;
//! 5. seleção antes do PIN em conta **com** PIN → `0x0FDF` + painel e nenhum personagem criado.
//!
//! Gated por `WYD_LIVE_CPSOCK` + `WYD_TEST_DSN`/`DATABASE_URL`. Os frames lidos são gravados em
//! `tmp/login-canal/*.txt` para o parser independente.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use wyd_admin::auth::hash_secret;
use wyd_db::{Accounts, Characters, PgBackend, PoolConfig, Role};
use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{AccountLogin, AccountSecure, CreateCharacter, Header, WirePayload};

const SENHA: &str = "qa12345678";
const PIN: &str = "2468";
const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/login-canal");

/// Conta única por execução — o roster tem de começar vazio.
fn conta_unica(prefixo: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    // `_MSG_AccountLogin` carrega o nome em 16 B: prefixo + 8 dígitos tem de caber, senão o
    // servidor recebe um login truncado e a recusa medida seria `conta inexistente` por acidente.
    format!("{prefixo}{:08}", nanos % 100_000_000)
}

fn runtime() -> Arc<tokio::runtime::Runtime> {
    Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("runtime"),
    )
}

fn frame_login(conta: &str, senha: &str) -> Vec<u8> {
    let mut nome = [0u8; 16];
    let n = conta.len().min(16);
    nome[..n].copy_from_slice(&conta.as_bytes()[..n]);
    let mut senha_bytes = [0u8; 12];
    let s = senha.len().min(12);
    senha_bytes[..s].copy_from_slice(&senha.as_bytes()[..s]);
    let mut frame = vec![0u8; AccountLogin::WIRE_LEN];
    Header {
        size: AccountLogin::WIRE_LEN as u16,
        key_word: 11,
        checksum: 0,
        message_type: Opcode::AccountLogin.wire(),
        id: 1,
        client_tick: 1,
    }
    .encode(&mut frame)
    .expect("header");
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
    encode_in_place(&mut frame, 11).expect("ofuscação");
    frame
}

fn frame_pin(pin: &str) -> Vec<u8> {
    let mut token = [0u8; 6];
    let b = pin.len().min(6);
    token[..b].copy_from_slice(&pin.as_bytes()[..b]);
    let mut frame = vec![0u8; AccountSecure::WIRE_LEN];
    Header {
        size: AccountSecure::WIRE_LEN as u16,
        key_word: 11,
        checksum: 0,
        message_type: Opcode::AccountSecure.wire(),
        id: 1,
        client_tick: 2,
    }
    .encode(&mut frame)
    .expect("header");
    AccountSecure {
        numeric_token: token,
        unknown: [0u8; 10],
        change_numeric: 0,
    }
    .encode_into(&mut frame)
    .expect("corpo do PIN");
    encode_in_place(&mut frame, 11).expect("ofuscação");
    frame
}

fn frame_criacao(slot: i32, nome: &str) -> Vec<u8> {
    let mut campo = [0u8; 16];
    let n = nome.len().min(16);
    campo[..n].copy_from_slice(&nome.as_bytes()[..n]);
    let mut frame = vec![0u8; CreateCharacter::WIRE_LEN];
    Header {
        size: CreateCharacter::WIRE_LEN as u16,
        key_word: 11,
        checksum: 0,
        message_type: Opcode::CreateCharacter.wire(),
        id: 1,
        client_tick: 3,
    }
    .encode(&mut frame)
    .expect("header");
    CreateCharacter {
        slot,
        mob_name: campo,
        mob_class: 0,
    }
    .encode_into(&mut frame)
    .expect("corpo da criação");
    encode_in_place(&mut frame, 11).expect("ofuscação");
    frame
}

fn conectar(endereco: &str) -> TcpStream {
    let stream = TcpStream::connect(endereco).expect("conecta no canal");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("timeout");
    stream
}

/// Lê um frame inteiro e devolve `(opcode, bytes de-ofuscados)`.
fn ler_frame(stream: &mut TcpStream) -> Option<(Opcode, Vec<u8>)> {
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

fn despejar(nome: &str, linhas: &[String]) {
    std::fs::create_dir_all(RELATORIO).expect("tmp/login-canal");
    let mut saida = String::new();
    for linha in linhas {
        saida.push_str(linha);
        saida.push('\n');
    }
    std::fs::write(format!("{RELATORIO}/{nome}.txt"), saida).expect("dump");
}

/// Confere que, depois do painel, a conexão **continua aberta** e sem bytes novos — o caso
/// visível (senha errada na exclusão) fica no laço do canal, sem FIN. Um FIN no instante do
/// painel é o que apaga a janela de erro do cliente antes de ela ser desenhada.
fn sem_bytes_e_sem_fin(stream: &mut TcpStream) -> bool {
    stream
        .set_read_timeout(Some(Duration::from_millis(1_200)))
        .expect("timeout curto");
    let mut resto = [0u8; 64];
    match stream.read(&mut resto) {
        Ok(0) => false,
        Ok(n) => panic!("bytes inesperados depois do painel: {n}"),
        Err(erro) => matches!(
            erro.kind(),
            std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
        ),
    }
}

/// Confere a recusa de senha (um frame) e devolve as linhas cruas para o dump.
fn conferir_recusa_de_senha(endereco: &str, conta: &str, senha: &str, caso: &str) -> Vec<String> {
    let mut s = conectar(endereco);
    s.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    s.write_all(&frame_login(conta, senha)).expect("login");

    // A etapa de login do 7662 (`0x4b3e83`) não compara `0x0119` nem `0x011b` e trata `0x0101`
    // só como visibilidade (`0x4b3ec1`, sem `0x404109`). O único braço de mensagem é
    // `0x011C`/`0x011D` (`0x4b4082`/`0x4b4071` → `push 0xfa0; push 0x109aab8; call 0x404109`).
    // A recusa manda exatamente isso: **um** `_MSG_AlreadyPlaying` header-only, sem roster e sem
    // `CNFAccountLogin`. Prova: `docs/rust/evidencias/login-tela-handler-2026-11-14.md`.
    let (opcode, recusa) = ler_frame(&mut s).expect("`_MSG_AlreadyPlaying`");
    assert_eq!(
        opcode,
        Opcode::AlreadyPlaying,
        "{caso}: forma que a etapa de login desenha (0x011c em 0x4b4082)"
    );
    assert_eq!(
        recusa.len(),
        HEADER_SIZE,
        "{caso}: header-only (12 B), sem roster"
    );

    assert!(
        sem_bytes_e_sem_fin(&mut s),
        "{caso}: nada depois da mensagem e a conexão segue aberta, sem FIN"
    );

    eprintln!(
        "PASS({caso}): AlreadyPlaying size={} [{}] | sem roster, sem CNFAccountLogin",
        recusa.len(),
        hex(&recusa)
    );
    vec![
        format!("caso={caso} conta={conta}"),
        format!(
            "c2s size={} hex={}",
            AccountLogin::WIRE_LEN,
            hex(&frame_login(conta, senha))
        ),
        format!("s2c size={} hex={}", recusa.len(), hex(&recusa)),
    ]
}

fn nova_conta(rt: &tokio::runtime::Runtime, backend: &PgBackend, conta: &str) {
    rt.block_on(backend.create_account(conta, Role::Player))
        .expect("conta criada");
    let hash = hash_secret(SENHA).expect("argon2id");
    rt.block_on(backend.set_credentials(conta, &hash, Role::Player))
        .expect("credencial");
}

#[test]
fn recusas_de_login_no_canal_real() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("login no canal") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );
    let mut linhas: Vec<String> = Vec::new();

    // 1. senha errada numa conta que existe.
    let conta = conta_unica("qasenha");
    nova_conta(&rt, &backend, &conta);
    linhas.extend(conferir_recusa_de_senha(
        &endereco,
        &conta,
        "senha-errada",
        "senha_errada",
    ));

    // 1b. conta bloqueada (banida pelo portal): mesma forma de recusa, mesmo com senha certa.
    let conta_db = rt
        .block_on(backend.find_by_login(&conta))
        .expect("conta do bloqueio");
    rt.block_on(backend.set_blocked(&conta, true, "canal: teste", conta_db.id))
        .expect("bloqueio");
    linhas.extend(conferir_recusa_de_senha(
        &endereco,
        &conta,
        SENHA,
        "conta_bloqueada",
    ));
    rt.block_on(backend.set_blocked(&conta, false, "", conta_db.id))
        .expect("desbloqueio");

    // 2. conta inexistente: mesma forma, sem revelar existência.
    let inexistente = conta_unica("qanada");
    linhas.extend(conferir_recusa_de_senha(
        &endereco,
        &inexistente,
        SENHA,
        "conta_inexistente",
    ));
    despejar("recusas", &linhas);

    // 3. PIN errado → 0x0FDF + painel, roster intacto.
    let conta_pin = conta_unica("qapin");
    nova_conta(&rt, &backend, &conta_pin);
    let conta_db = rt
        .block_on(backend.find_by_login(&conta_pin))
        .expect("conta do PIN");

    let mut primeiro = conectar(&endereco);
    primeiro
        .write_all(&INITCODE.to_le_bytes())
        .expect("initcode");
    primeiro
        .write_all(&frame_login(&conta_pin, SENHA))
        .expect("login");
    let (opcode, _) = ler_frame(&mut primeiro).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin, "login da conta do PIN");
    primeiro
        .write_all(&frame_pin(PIN))
        .expect("cadastro do PIN");
    let (opcode, _aceito) = ler_frame(&mut primeiro).expect("PIN cadastrado");
    assert_eq!(opcode, Opcode::AccountSecure, "PIN inicial aceito");
    drop(primeiro);
    std::thread::sleep(Duration::from_millis(800));

    let hash_antes = rt
        .block_on(backend.pin_hash(&conta_pin))
        .expect("pin_hash antes")
        .expect("PIN gravado");

    let mut errado = conectar(&endereco);
    errado.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    errado
        .write_all(&frame_login(&conta_pin, SENHA))
        .expect("login");
    let (opcode, _) = ler_frame(&mut errado).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin);
    errado.write_all(&frame_pin("0000")).expect("PIN errado");
    let (opcode, recusa) = ler_frame(&mut errado).expect("0x0FDF");
    assert_eq!(
        opcode,
        Opcode::AccountSecureFail,
        "PIN errado recebe `_MSG_AccountSecureFail` no canal real"
    );
    let (opcode, painel_pin) = ler_frame(&mut errado).expect("painel do PIN");
    assert_eq!(opcode, Opcode::MessagePanel, "painel do PIN errado");
    let roster = rt
        .block_on(backend.list_by_account(conta_db.id))
        .expect("roster");
    assert!(
        roster.is_empty(),
        "PIN errado não pode criar nada: {roster:?}"
    );
    let hash_depois = rt
        .block_on(backend.pin_hash(&conta_pin))
        .expect("pin_hash depois")
        .expect("PIN continua gravado");
    assert_eq!(
        hash_antes, hash_depois,
        "PIN errado não pode reescrever `accounts.pin_hash`"
    );
    eprintln!(
        "PASS(pin_errado): AccountSecureFail size={} [{}] | MessagePanel size={} | roster={} personagens",
        recusa.len(),
        hex(&recusa),
        painel_pin.len(),
        roster.len()
    );
    linhas = vec![
        format!("caso=pin_errado conta={conta_pin}"),
        format!(
            "c2s size={} hex={}",
            AccountSecure::WIRE_LEN,
            hex(&frame_pin("0000"))
        ),
        format!("s2c size={} hex={}", recusa.len(), hex(&recusa)),
        format!("s2c size={} hex={}", painel_pin.len(), hex(&painel_pin)),
        format!("roster_depois={}", roster.len()),
    ];

    // 4. PIN correto → `0x0FDE` e criação conclui no banco.
    let mut certo = conectar(&endereco);
    certo.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    certo
        .write_all(&frame_login(&conta_pin, SENHA))
        .expect("login");
    let (opcode, _) = ler_frame(&mut certo).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin);
    certo.write_all(&frame_pin(PIN)).expect("PIN correto");
    let (opcode, ok_pin) = ler_frame(&mut certo).expect("0x0FDE");
    assert_eq!(opcode, Opcode::AccountSecure, "PIN correto é aceito");
    let nome = format!("Pc{}", &conta_pin[2..8]);
    certo
        .write_all(&frame_criacao(1, &nome))
        .expect("criação depois do PIN");
    let (opcode, criado) = ler_frame(&mut certo).expect("CNFNewCharacter");
    assert_eq!(
        opcode,
        Opcode::CNFNewCharacter,
        "PIN correto prossegue até a seleção"
    );
    let roster = rt
        .block_on(backend.list_by_account(conta_db.id))
        .expect("roster");
    assert!(
        roster.iter().any(|p| p.name == nome),
        "personagem `{nome}` precisa existir no banco: {roster:?}"
    );
    eprintln!(
        "PASS(pin_correto): AccountSecure size={} [{}] | CNFNewCharacter size={} | banco={}",
        ok_pin.len(),
        hex(&ok_pin),
        criado.len(),
        roster.len()
    );
    linhas.push(format!("caso=pin_correto conta={conta_pin}"));
    linhas.push(format!(
        "c2s size={} hex={}",
        AccountSecure::WIRE_LEN,
        hex(&frame_pin(PIN))
    ));
    linhas.push(format!("s2c size={} hex={}", ok_pin.len(), hex(&ok_pin)));
    linhas.push(format!("s2c size={} hex={}", criado.len(), hex(&criado)));
    linhas.push(format!("personagem_criado={nome}"));
    despejar("pin", &linhas);

    // 5. seleção antes do PIN numa conta com PIN → recusa e nada gravado.
    let mut pulou = conectar(&endereco);
    pulou.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    pulou
        .write_all(&frame_login(&conta_pin, SENHA))
        .expect("login");
    let (opcode, _) = ler_frame(&mut pulou).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin);
    let outro = format!("Pp{}", &conta_pin[2..8]);
    pulou
        .write_all(&frame_criacao(2, &outro))
        .expect("seleção sem PIN");
    let (opcode, sem_pin) = ler_frame(&mut pulou).expect("0x0FDF da seleção");
    assert_eq!(
        opcode,
        Opcode::AccountSecureFail,
        "seleção antes do PIN é recusada com `0x0FDF`"
    );
    let roster = rt
        .block_on(backend.list_by_account(conta_db.id))
        .expect("roster");
    assert!(
        !roster.iter().any(|p| p.name == outro),
        "seleção recusada não pode criar `{outro}`: {roster:?}"
    );
    eprintln!(
        "PASS(selecao_sem_pin): AccountSecureFail size={} [{}] | banco sem `{outro}`",
        sem_pin.len(),
        hex(&sem_pin)
    );
    let mut linhas_pulo = vec![
        format!("caso=selecao_sem_pin conta={conta_pin}"),
        format!(
            "c2s size={} hex={}",
            CreateCharacter::WIRE_LEN,
            hex(&frame_criacao(2, &outro))
        ),
        format!("s2c size={} hex={}", sem_pin.len(), hex(&sem_pin)),
        format!("personagem_ausente={outro}"),
    ];
    despejar("selecao_sem_pin", &linhas_pulo);
    linhas_pulo.clear();
}

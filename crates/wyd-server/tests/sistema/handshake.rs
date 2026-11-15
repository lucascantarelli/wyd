//! Handshake CPSock de verdade: socket, `INITCODE`, frame ofuscado e gate de versão.
//!
//! O teste sobe o listener em porta efêmera, conecta como o cliente faz e verifica os dois
//! lados do gate: versão do cliente oficial (12 000) recebe `_MSG_CNFAccountLogin`; versão
//! histórica (7 640) é recusada e a conexão fecha sem resposta.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use wyd_db::{PgBackend, PoolConfig, ServerChannel};
use wyd_net::VersionGate;
use wyd_protocol::codec::keyword::encode_in_place;
use wyd_protocol::payload::WirePayload;
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{AccountLogin, Header};
use wyd_server::channels::World;
use wyd_server::cpsock;

fn canal(channel: i64, host: &str) -> ServerChannel {
    ServerChannel {
        group_id: 1,
        channel,
        host: host.to_string(),
        status_url: if channel == 0 {
            "http://192.168.100.3/serv00.htm".to_string()
        } else {
            String::new()
        },
        enabled: true,
    }
}

fn world() -> Arc<World> {
    Arc::new(
        World::new(vec![
            canal(0, "192.168.100.3"),
            canal(1, "192.168.100.3"),
            canal(2, "192.168.100.4"),
        ])
        .expect("foto com dois canais"),
    )
}

/// Frame de login ofuscado, exatamente como o cliente o envia.
///
/// `conta` é única por chamada porque o cânone do log cobre uma sessão por conta: dois testes
/// em paralelo com a mesma conta disputariam a cerca (`character_session`).
fn login_frame(version: i32, key_word: u8, conta: &str, senha: &str) -> Vec<u8> {
    let mut nome_conta = [0u8; 16];
    let bytes = conta.as_bytes();
    let n = bytes.len().min(nome_conta.len());
    nome_conta[..n].copy_from_slice(&bytes[..n]);
    let mut senha_bytes = [0u8; 12];
    let s = senha.as_bytes();
    senha_bytes[..s.len().min(12)].copy_from_slice(&s[..s.len().min(12)]);
    let mut frame = vec![0u8; AccountLogin::WIRE_LEN];
    Header {
        size: AccountLogin::WIRE_LEN as u16,
        key_word,
        checksum: 0,
        message_type: Opcode::AccountLogin.wire(),
        id: 1,
        client_tick: 42,
    }
    .encode(&mut frame)
    .expect("cabeçalho cabe no frame");
    AccountLogin {
        account_password: senha_bytes,
        account_name: nome_conta,
        zero: [0u8; 52],
        client_version: version,
        db_need_save: 1,
        adapter_name: [0; 4],
    }
    .encode_into(&mut frame)
    .expect("corpo cabe no frame");
    encode_in_place(&mut frame, key_word).expect("ofuscação válida");
    frame
}

/// Listener em processo, **sem** cerca de sessão (o foco destes casos é o framing/gate).
fn serve() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("porta efêmera");
    let port = listener.local_addr().expect("endereço").port();
    let world = world();
    std::thread::spawn(move || {
        cpsock::serve_listener(listener, world, VersionGate::default(), None, None, None)
    });
    port
}

/// Conta única por chamada (16 bytes no fio): a cerca é por conta.
fn conta_unica(prefixo: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let nome = format!("{prefixo}{nanos}");
    nome.chars().take(16).collect()
}

#[test]
fn versao_do_cliente_oficial_completa_o_handshake() {
    let port = serve();
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("conecta");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("timeout");
    stream.write_all(&INITCODE.to_le_bytes()).expect("INITCODE");
    stream
        .write_all(&login_frame(12_000, 7, &conta_unica("h"), ""))
        .expect("login");

    let mut head = [0u8; HEADER_SIZE];
    stream.read_exact(&mut head).expect("resposta do servidor");
    let size = usize::from(u16::from_le_bytes([head[0], head[1]]));
    let mut frame = vec![0u8; size];
    frame[..HEADER_SIZE].copy_from_slice(&head);
    if size > HEADER_SIZE {
        stream.read_exact(&mut frame[HEADER_SIZE..]).expect("corpo");
    }
    let computed = wyd_protocol::codec::keyword::decode_in_place(&mut frame).expect("desofusca");
    assert_eq!(computed, frame[3], "checksum da resposta confere");
    let header = Header::decode(&frame).expect("cabeçalho da resposta");
    assert_eq!(
        Opcode::from_wire(header.message_type),
        Some(Opcode::CNFAccountLogin),
        "servidor confirma o login com _MSG_CNFAccountLogin"
    );
    assert!(header.size as usize >= HEADER_SIZE, "frame com cabeçalho");
}

#[test]
fn versao_fora_do_gate_e_recusada_com_conexao_fechada() {
    let port = serve();
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("conecta");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("timeout");
    stream.write_all(&INITCODE.to_le_bytes()).expect("INITCODE");
    stream
        .write_all(&login_frame(7_640, 3, &conta_unica("v"), ""))
        .expect("login histórico");

    let mut buf = Vec::new();
    let lido = stream.read_to_end(&mut buf).unwrap_or(0);
    assert_eq!(lido, 0, "sem confirmação: o servidor fecha a conexão");
    assert!(buf.is_empty());
}

/// Prova contra os containers de verdade: `WYD_LIVE_CPSOCK=192.168.100.3:8281`.
///
/// Sem a variável o teste não roda (o CI não tem a stack), com ela o mesmo frame do cliente
/// atravessa o Docker e exige `_MSG_CNFAccountLogin` — handshake real, não in-process.
#[test]
fn handshake_contra_servidor_vivo() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("handshake autenticado contra o container") else {
        return;
    };
    // O campo do `_MSG_AccountLogin` tem 12 bytes: a senha do teste cabe nele.
    const SENHA: &str = "Senha123456";
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("runtime");
    let backend = rt
        .block_on(PgBackend::connect(&dsn, PoolConfig::local()))
        .expect("banco do handshake");
    // Conta **própria** e fixa deste teste (`qahandlive`), não a `WYD_LIVE_ACCOUNT` do harness.
    //
    // Antes ele pegava a conta do env e **reescrevia a senha** para `Senha123456`: como a suíte
    // inteira aponta para a mesma conta (`qapinraw0001`), o teste de gauge logo depois levava
    // `senha de login não confere` — medido no log do canal em 2026-11-15. Reusar conta de outro
    // teste assim não é autossuficiente; cada um normaliza a sua.
    let conta = "qahandlive".to_string();
    common::preparar_conta_de_teste(&rt, &backend, &conta, SENHA, "2468");

    let mut stream = TcpStream::connect(&endereco).expect("conecta no container");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("timeout");
    stream.write_all(&INITCODE.to_le_bytes()).expect("INITCODE");
    stream
        .write_all(&login_frame(12_000, 11, &conta, SENHA))
        .expect("login");
    let mut reply = [0u8; HEADER_SIZE];
    stream
        .read_exact(&mut reply)
        .expect("resposta do container");
    let size = usize::from(u16::from_le_bytes([reply[0], reply[1]]));
    let mut frame = vec![0u8; size];
    frame[..HEADER_SIZE].copy_from_slice(&reply);
    if size > HEADER_SIZE {
        stream
            .read_exact(&mut frame[HEADER_SIZE..])
            .expect("corpo da resposta");
    }
    wyd_protocol::codec::keyword::decode_in_place(&mut frame).expect("desofusca");
    let header = Header::decode(&frame).expect("cabeçalho");
    assert_eq!(
        Opcode::from_wire(header.message_type),
        Some(Opcode::CNFAccountLogin),
        "{endereco} confirmou o handshake com senha"
    );
}

/// Negativo vivo: `WYD_LIVE_CPSOCK_BAD=192.168.100.4:8281` envia 7 640 e exige conexão fechada.
#[test]
fn versao_historica_e_recusada_pelo_servidor_vivo() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK_BAD") else {
        return;
    };
    let mut stream = TcpStream::connect(&endereco).expect("conecta no container");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("timeout");
    stream.write_all(&INITCODE.to_le_bytes()).expect("INITCODE");
    stream
        .write_all(&login_frame(7_640, 5, &conta_unica("b"), ""))
        .expect("login");
    let mut buf = Vec::new();
    let lido = std::io::Read::read_to_end(&mut stream, &mut buf).unwrap_or(1);
    assert_eq!(lido, 0, "{endereco} recusou 7640 e fechou sem resposta");
}

#[test]
fn initcode_errado_nao_entra_no_handshake() {
    let port = serve();
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("conecta");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("timeout");
    // Lixo no lugar do INITCODE **e** um frame de login bem formado logo depois: se o gate de
    // INITCODE sumir, o servidor processa o frame e responde `CNFAccountLogin` — é isso que a
    // asserção precisa pegar (só o lixo de 4 B fecharia por timeout, não por validação).
    let mut lixo = Vec::from(0xDEAD_BEEFu32.to_le_bytes());
    lixo.extend_from_slice(&login_frame(12_000, 1, &conta_unica("i"), ""));
    stream.write_all(&lixo).expect("lixo + login");
    let mut buf = Vec::new();
    let lido = stream.read_to_end(&mut buf).unwrap_or(0);
    assert_eq!(
        lido, 0,
        "conexão sem INITCODE correto é descartada sem resposta"
    );
}

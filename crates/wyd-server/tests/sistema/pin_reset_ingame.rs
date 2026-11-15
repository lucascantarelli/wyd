//! Reset da segunda senha **pelo portal** e a volta do jogador **pelo protocolo de jogo**.
//!
//! A conta sai do reset com `pin_hash IS NULL` (o portal chama o mesmo `clear_pin_hash`). Este
//! teste começa exatamente nesse estado e conversa com o canal como o cliente 7662, sem atalho
//! de banco para gravar PIN:
//!
//! 1. `_MSG_AccountLogin` → `_MSG_CNFAccountLogin` e o canal **pede o cadastro** (`0x0101`
//!    "Cadastre a segunda senha no dialogo do cliente."), o sinal de que a conta está em
//!    primeira entrada depois do reset;
//! 2. `_MSG_AccountSecure` (`0x0FDE`, 32 B) com o corpo **capturado do botão Alterar** do
//!    cliente (`numeric_token[6]` = `432143`, o valor digitado duas vezes), `change_numeric = 0`:
//!    o servidor cadastra, responde `0x0FDE` e grava `accounts.pin_registered` na trilha;
//! 3. o personagem criado por `_MSG_CreateCharacter` entra por `_MSG_CharacterLogin` com o PIN
//!    novo (`CNFCharacterLogin`) — sem chamada HTTP no meio;
//! 4. a **troca** usa o mesmo corpo: passo 1 (`change_numeric = 1`, `432143`) prova o PIN atual,
//!    passo 2 (`change_numeric = 2`, `864286`) grava o novo; hash novo confere e o antigo não,
//!    com `accounts.pin_changed` na trilha;
//! 5. o PIN **antigo** agora é recusado (`0x0FDF` + painel) sem zerar o hash nem abrir sessão, e
//!    o jogador continua entrando com o PIN novo.
//!
//! Gated por `WYD_TEST_DSN`/`DATABASE_URL` (canal em processo, banco real). Frames crus em
//! `tmp/pin-reset-ingame/`.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use wyd_admin::{hash_password, verify_password};
use wyd_db::{
    Accounts, AuditTrail, Characters, PgBackend, PoolConfig, Role, ServerChannel, Sessions,
};
use wyd_net::VersionGate;
use wyd_protocol::codec::keyword::decode_in_place;
use wyd_protocol::payload::WirePayload;
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{AccountLogin, AccountSecure, CharacterLogin, CreateCharacter, Header};
use wyd_server::auth::Authenticator;
use wyd_server::channels::World;
use wyd_server::characters::SelectionStore;
use wyd_server::cpsock;
use wyd_server::session::SessionGate;

const SENHA: &str = "SenhaForte1";
const CONTA: &str = "qapinreset1";
const PERSONAGEM: &str = "QAReset";
/// Corpo do Alterar capturado no proxy para o PIN `4321` (valor digitado duas vezes, cortado em 6).
const CORPO_ALTERAR_4321: &[u8; 6] = b"432143";
/// Mesmo corpo para o PIN novo `8642` na troca.
const CORPO_ALTERAR_8642: &[u8; 6] = b"864286";
const PIN_NOVO: &str = "8642";
const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/pin-reset-ingame");

fn runtime() -> Arc<tokio::runtime::Runtime> {
    Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("runtime"),
    )
}

/// Canal em processo com autenticador, cerca de sessão e armazém de personagens (banco real).
fn servidor(
    backend: Arc<PgBackend>,
    rt: Arc<tokio::runtime::Runtime>,
) -> (u16, Arc<SelectionStore>) {
    let auth = Arc::new(Authenticator::new(Arc::clone(&backend), Arc::clone(&rt)));
    let cerca = Arc::new(SessionGate::new(
        Arc::clone(&backend),
        Arc::clone(&rt),
        1,
        1,
        Duration::from_secs(120),
        Duration::from_secs(180),
    ));
    let selection = Arc::new(SelectionStore::new(Arc::clone(&backend), Arc::clone(&rt)));
    let world = Arc::new(
        World::new(vec![ServerChannel {
            group_id: 1,
            channel: 1,
            host: "127.0.0.1".to_string(),
            status_url: String::new(),
            enabled: true,
        }])
        .expect("canal"),
    );
    let listener = TcpListener::bind("127.0.0.1:0").expect("porta");
    let porta = listener.local_addr().expect("endereço").port();
    let para_servidor = Arc::clone(&selection);
    std::thread::spawn(move || {
        cpsock::serve_listener(
            listener,
            world,
            VersionGate::default(),
            Some(auth),
            Some(cerca),
            Some(para_servidor),
        )
    });
    (porta, selection)
}

/// Conta fixa da suíte: cria se faltar, reescreve a senha e limpa o roster — **sem** mexer no
/// `pin_hash`, que é justamente o estado sob prova (o reset do portal é quem o apaga).
fn preparar_conta_sem_tocar_no_pin(rt: &tokio::runtime::Runtime, backend: &PgBackend) -> i64 {
    let id = match rt.block_on(backend.find_by_login(CONTA)) {
        Ok(conta) => conta.id,
        Err(_) => {
            rt.block_on(backend.create_account(CONTA, Role::Player))
                .expect("conta de QA criada");
            rt.block_on(backend.find_by_login(CONTA))
                .expect("conta de QA")
                .id
        }
    };
    let hash = hash_password(SENHA).expect("argon2id da senha de QA");
    rt.block_on(backend.set_credentials(CONTA, &hash, Role::Player))
        .expect("senha da conta de QA");
    for personagem in rt
        .block_on(backend.list_by_account(id))
        .expect("roster da conta de QA")
    {
        rt.block_on(backend.soft_delete(personagem.id))
            .expect("personagem de execução anterior removido");
    }
    id
}

fn quadro(opcode: Opcode, len: usize, tick: u32) -> Vec<u8> {
    let mut frame = vec![0u8; len];
    Header {
        size: u16::try_from(len).expect("frame"),
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
    wyd_protocol::codec::keyword::encode_in_place(&mut frame, 11).expect("ofuscação");
    frame
}

fn frame_login(conta: &str) -> Vec<u8> {
    let mut nome = [0u8; 16];
    let bytes = conta.as_bytes();
    nome[..bytes.len()].copy_from_slice(bytes);
    let mut senha = [0u8; 12];
    senha[..SENHA.len()].copy_from_slice(SENHA.as_bytes());
    let mut frame = quadro(Opcode::AccountLogin, AccountLogin::WIRE_LEN, 1);
    AccountLogin {
        account_password: senha,
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

/// `_MSG_AccountSecure` (0x0FDE) com o corpo cru do cliente: `numeric_token[6]` + `change`.
fn frame_pin(token: &[u8; 6], change_numeric: i32) -> Vec<u8> {
    let mut frame = quadro(Opcode::AccountSecure, AccountSecure::WIRE_LEN, 2);
    AccountSecure {
        numeric_token: *token,
        unknown: [0u8; 10],
        change_numeric,
    }
    .encode_into(&mut frame)
    .expect("corpo do PIN");
    fechar(frame)
}

fn frame_pin_digitos(pin: &str) -> Vec<u8> {
    let mut token = [0u8; 6];
    token[..pin.len()].copy_from_slice(pin.as_bytes());
    frame_pin(&token, 0)
}

fn frame_criacao(slot: i32) -> Vec<u8> {
    let mut nome = [0u8; 16];
    nome[..PERSONAGEM.len()].copy_from_slice(PERSONAGEM.as_bytes());
    let mut frame = quadro(Opcode::CreateCharacter, CreateCharacter::WIRE_LEN, 3);
    CreateCharacter {
        slot,
        mob_name: nome,
        mob_class: 0,
    }
    .encode_into(&mut frame)
    .expect("corpo da criação");
    fechar(frame)
}

fn frame_entrar(slot: i32) -> Vec<u8> {
    let mut frame = quadro(Opcode::CharacterLogin, CharacterLogin::WIRE_LEN, 4);
    CharacterLogin { slot, force: 1 }
        .encode_into(&mut frame)
        .expect("corpo do CharacterLogin");
    fechar(frame)
}

fn ler_frame(stream: &mut TcpStream) -> Vec<u8> {
    let mut head = [0u8; HEADER_SIZE];
    stream.read_exact(&mut head).expect("frame");
    let size = usize::from(u16::from_le_bytes([head[0], head[1]]));
    let mut frame = vec![0u8; size];
    frame[..HEADER_SIZE].copy_from_slice(&head);
    if size > HEADER_SIZE {
        stream.read_exact(&mut frame[HEADER_SIZE..]).expect("corpo");
    }
    frame
}

fn abrir(frame: &[u8]) -> (Opcode, Vec<u8>) {
    let mut limpo = frame.to_vec();
    let computado = decode_in_place(&mut limpo).expect("desofusca");
    assert_eq!(computado, limpo[3], "checksum");
    let header = Header::decode(&limpo[..HEADER_SIZE]).expect("header");
    (
        Opcode::from_wire(header.message_type).expect("opcode"),
        limpo[HEADER_SIZE..].to_vec(),
    )
}

fn hex(frame: &[u8]) -> String {
    frame.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Abre o soquete, manda `INITCODE` + login e exige `_MSG_CNFAccountLogin`.
fn conecta(porta: u16) -> TcpStream {
    let mut stream = TcpStream::connect(("127.0.0.1", porta)).expect("conecta");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("timeout");
    stream.write_all(&INITCODE.to_le_bytes()).expect("init");
    stream.write_all(&frame_login(CONTA)).expect("login");
    let (opcode, _) = abrir(&ler_frame(&mut stream));
    assert_eq!(opcode, Opcode::CNFAccountLogin, "login confirma a conta");
    stream
}

fn responde(stream: &mut TcpStream, frame: Vec<u8>) -> (Opcode, Vec<u8>, Vec<u8>) {
    stream.write_all(&frame).expect("envio");
    let bruto = ler_frame(stream);
    let (opcode, corpo) = abrir(&bruto);
    (opcode, corpo, bruto)
}

/// Espera a cerca de sessão da conta ficar livre (o fechamento da conexão libera).
fn esperar_sem_sessao(rt: &tokio::runtime::Runtime, backend: &PgBackend) {
    let liberou = common::esperar_ate(Duration::from_secs(10), || {
        rt.block_on(backend.live_session_of(CONTA))
            .expect("sessão")
            .is_none()
    });
    assert!(liberou, "sessão liberada no fechamento da conexão");
}

/// Texto do painel (`0x0101`, texto no `+12`).
fn texto_do_painel(corpo: &[u8]) -> String {
    let mut frame = vec![0u8; HEADER_SIZE];
    frame.extend_from_slice(corpo);
    String::from_utf8_lossy(wyd_protocol::nul_trimmed(&frame[HEADER_SIZE..])).to_string()
}

/// Quantas linhas desta ação e desta conta existem na trilha (a tabela é append-only, então o
/// teste compara **contagens** antes/depois em vez de procurar uma linha qualquer).
fn contar(rt: &tokio::runtime::Runtime, backend: &PgBackend, acao: &str) -> usize {
    let alvo = format!("account:{}", id_conta(rt, backend));
    rt.block_on(backend.recent(60))
        .expect("audit_log")
        .iter()
        .filter(|linha| linha.action == acao && linha.target == alvo)
        .count()
}

fn trilha(rt: &tokio::runtime::Runtime, backend: &PgBackend, acao: &str) -> bool {
    contar(rt, backend, acao) > 0
}

fn id_conta(rt: &tokio::runtime::Runtime, backend: &PgBackend) -> i64 {
    rt.block_on(backend.find_by_login(CONTA)).expect("conta").id
}

fn despejar(nome: &str, linhas: &[String]) {
    let dir = std::path::Path::new(RELATORIO);
    let _ = std::fs::create_dir_all(dir);
    let _ = std::fs::write(dir.join(format!("{nome}.txt")), linhas.join("\n"));
}

#[test]
fn reset_do_portal_volta_pelo_pacote_real_do_cliente() {
    let Some(dsn) = common::dsn_ou_skip("reset da segunda senha no jogo") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );
    // Conta fixa de QA com senha e roster limpo, **sem tocar no `pin_hash`**: o estado que
    // interessa é o que o reset do portal deixou. Numa conta nova (primeira execução) o teste
    // aplica o mesmo `clear_pin_hash` do reset e diz isso na evidência; quando o reset do portal
    // rodou antes, o estado é herdado e nenhuma escrita de PIN acontece aqui.
    let conta_id = preparar_conta_sem_tocar_no_pin(&rt, &backend);
    let herdou_reset = rt
        .block_on(backend.pin_hash(CONTA))
        .expect("pin_hash")
        .is_none();
    if !herdou_reset {
        rt.block_on(backend.clear_pin_hash(CONTA))
            .expect("PIN apagado (mesmo clear_pin_hash do reset)");
    }
    assert_eq!(
        rt.block_on(backend.pin_hash(CONTA)).expect("pin_hash"),
        None,
        "conta precisa estar em primeira entrada (reset)"
    );
    let (porta, _selection) = servidor(Arc::clone(&backend), Arc::clone(&rt));
    let endereco = format!("127.0.0.1:{porta}");
    let mut linhas: Vec<String> = vec![
        format!("conta={CONTA} id={conta_id}"),
        format!(
            "estado_inicial=pin_hash NULL origem={}",
            if herdou_reset {
                "reset do portal (estado herdado, sem escrita do teste)"
            } else {
                "clear_pin_hash do teste (mesma rotina do reset)"
            }
        ),
    ];

    // 1. Conecta sem PIN: o canal pede o cadastro antes de qualquer coisa.
    let mut primeira = conecta(porta);
    let pedido = ler_frame(&mut primeira);
    let (opcode, corpo) = abrir(&pedido);
    assert_eq!(opcode, Opcode::MessagePanel, "canal pede o cadastro do PIN");
    let texto = texto_do_painel(&corpo);
    assert!(
        texto.to_lowercase().contains("cadastre") && texto.to_lowercase().contains("senha"),
        "pedido precisa dizer que a segunda senha deve ser cadastrada: {texto:?}"
    );
    eprintln!("PASS(pedido): MessagePanel {} B \"{texto}\"", pedido.len());
    linhas.push(format!("s2c size={} hex={}", pedido.len(), hex(&pedido)));
    linhas.push(format!("s2c_pedido={texto}"));

    // 2. O corpo do Alterar (`432143`) cadastra o PIN.
    let corpo_alterar = frame_pin(CORPO_ALTERAR_4321, 0);
    let (opcode, corpo, bruto) = responde(&mut primeira, corpo_alterar.clone());
    assert_eq!(opcode, Opcode::AccountSecure, "0x0FDE do Alterar é aceito");
    assert_eq!(bruto.len(), HEADER_SIZE, "aceite é header-only");
    assert_eq!(corpo.len(), 0, "sem corpo no aceite");
    let gravado = rt
        .block_on(backend.pin_hash(CONTA))
        .expect("pin_hash")
        .expect("PIN gravado");
    assert!(
        verify_password("4321", &gravado),
        "PIN resolvido do campo duplicado é 4321"
    );
    assert!(
        !verify_password("432143", &gravado),
        "o blob de 6 dígitos nunca é o PIN"
    );
    assert!(
        trilha(&rt, &backend, "accounts.pin_registered"),
        "cadastro no cliente entra na trilha"
    );
    eprintln!(
        "PASS(cadastro): c2s {} B hex={} | s2c {} B hex={} | accounts.pin_registered",
        corpo_alterar.len(),
        hex(&corpo_alterar),
        bruto.len(),
        hex(&bruto)
    );
    linhas.push(format!(
        "c2s size={} hex={}",
        corpo_alterar.len(),
        hex(&corpo_alterar)
    ));
    linhas.push(format!("s2c size={} hex={}", bruto.len(), hex(&bruto)));
    linhas.push("db_pin_hash=argon2id(4321) pin_failures=0 trilha=accounts.pin_registered".into());
    drop(primeira);
    esperar_sem_sessao(&rt, &backend);

    // 2b. Com PIN gravado, o **mesmo** corpo do Alterar (`432143`) só verifica e entra.
    let trocas_antes = contar(&rt, &backend, "accounts.pin_changed");
    let mut confere = conecta(porta);
    let (opcode, _, bruto) = responde(&mut confere, frame_pin(CORPO_ALTERAR_4321, 0));
    assert_eq!(
        opcode,
        Opcode::AccountSecure,
        "Alterar com PIN já gravado confere pelo mesmo corpo"
    );
    assert_eq!(
        bruto.len(),
        HEADER_SIZE,
        "aceite da conferência é header-only"
    );
    assert_eq!(
        contar(&rt, &backend, "accounts.pin_changed"),
        trocas_antes,
        "conferir não é trocar: nenhuma linha nova de troca na trilha"
    );
    eprintln!("PASS(conferencia): mesmo corpo 432143 aceito como verificação ({bruto:?})");
    linhas.push("conferencia=mesmo_corpo_432143_aceito_modo_Verified".to_string());
    drop(confere);
    esperar_sem_sessao(&rt, &backend);

    // 3. Personagem entra com o PIN novo.
    let mut entrada = conecta(porta);
    let (opcode, _, _) = responde(&mut entrada, frame_pin_digitos("4321"));
    assert_eq!(opcode, Opcode::AccountSecure, "PIN novo aceito no login");
    let (opcode, _, criado) = responde(&mut entrada, frame_criacao(0));
    assert_eq!(opcode, Opcode::CNFNewCharacter, "personagem criado");
    let (opcode, _, login) = responde(&mut entrada, frame_entrar(0));
    assert_eq!(
        opcode,
        Opcode::CNFCharacterLogin,
        "personagem entra no mundo com o PIN cadastrado pelo corpo do cliente"
    );
    let viva = rt.block_on(backend.live_session_of(CONTA)).expect("sessão");
    assert!(viva.is_some(), "sessão viva depois de entrar no mundo");
    eprintln!(
        "PASS(entrada): CNFNewCharacter {} B | CNFCharacterLogin {} B | sessao={viva:?}",
        criado.len(),
        login.len()
    );
    linhas.push(format!("s2c_entrada_criacao={} B", criado.len()));
    linhas.push(format!("s2c_entrada_login={} B", login.len()));
    drop(entrada);
    esperar_sem_sessao(&rt, &backend);

    // 4. Troca com o **mesmo** corpo: passo 1 prova o atual, passo 2 grava o novo.
    let mut troca = conecta(porta);
    let passo1 = frame_pin(CORPO_ALTERAR_4321, 1);
    let (opcode, _, bruto) = responde(&mut troca, passo1.clone());
    assert_eq!(opcode, Opcode::AccountSecure, "passo 1 confere o PIN atual");
    let passo2 = frame_pin(CORPO_ALTERAR_8642, 2);
    let (opcode, _, bruto2) = responde(&mut troca, passo2.clone());
    assert_eq!(opcode, Opcode::AccountSecure, "passo 2 grava o PIN novo");
    let novo = rt
        .block_on(backend.pin_hash(CONTA))
        .expect("pin_hash")
        .expect("PIN gravado");
    assert!(verify_password(PIN_NOVO, &novo), "PIN novo vale");
    assert!(!verify_password("4321", &novo), "PIN antigo não vale");
    assert!(
        trilha(&rt, &backend, "accounts.pin_changed"),
        "troca no cliente entra na trilha"
    );
    eprintln!(
        "PASS(troca): c2s {} B hex={} | c2s {} B hex={} | s2c {} B + {} B | db=argon2id(8642) trilha=accounts.pin_changed",
        passo1.len(),
        hex(&passo1),
        passo2.len(),
        hex(&passo2),
        bruto.len(),
        bruto2.len()
    );
    linhas.push(format!(
        "c2s_troca1 size={} hex={}",
        passo1.len(),
        hex(&passo1)
    ));
    linhas.push(format!(
        "c2s_troca2 size={} hex={}",
        passo2.len(),
        hex(&passo2)
    ));
    linhas
        .push("db_pin_hash=argon2id(8642) pin_antigo=recusado trilha=accounts.pin_changed".into());
    drop(troca);
    esperar_sem_sessao(&rt, &backend);

    // 5. PIN antigo errado: recusa honesta, hash intacto, nenhuma sessão.
    let mut errado = conecta(porta);
    let (opcode, corpo, bruto) = responde(&mut errado, frame_pin_digitos("4321"));
    assert_eq!(opcode, Opcode::AccountSecureFail, "PIN antigo é recusado");
    let motivo = corpo.first().copied().unwrap_or(0);
    assert_eq!(motivo, 0x02, "motivo PinIncorreto@12");
    let (opcode, painel, _) = responde(&mut errado, Vec::new());
    assert_eq!(opcode, Opcode::MessagePanel, "recusa vem com painel");
    let texto = texto_do_painel(&painel);
    assert!(
        texto.to_uppercase().contains("PIN"),
        "painel da recusa precisa citar o PIN: {texto:?}"
    );
    let (falhas, segundos) = rt.block_on(backend.pin_guard(CONTA)).expect("falhas");
    assert_eq!(falhas, 1, "a tentativa errada conta");
    assert_eq!(segundos, 0, "uma falha ainda não bloqueia");
    let intacto = rt
        .block_on(backend.pin_hash(CONTA))
        .expect("pin_hash")
        .expect("hash continua gravado");
    assert!(
        verify_password(PIN_NOVO, &intacto),
        "PIN antigo errado não zera nem troca o hash"
    );
    assert!(
        rt.block_on(backend.live_session_of(CONTA))
            .expect("sessão")
            .is_none(),
        "recusa não abre sessão"
    );
    eprintln!(
        "PASS(pin_antigo): 0x0FDF motivo=0x{motivo:02x} {} B + painel \"{texto}\" | pin_failures={falhas} sessao=None hash=argon2id(8642)",
        bruto.len()
    );
    linhas.push(format!(
        "c2s_errado size={} hex={}",
        32,
        hex(&frame_pin_digitos("4321"))
    ));
    linhas.push(format!(
        "s2c size={} hex={} motivo=0x{motivo:02x}",
        bruto.len(),
        hex(&bruto)
    ));
    linhas.push(format!("s2c_painel={texto}"));
    linhas.push(format!(
        "db_pin_failures={falhas} pin_locked={segundos} hash_intacto=true"
    ));
    drop(errado);

    // 6. Entra com o PIN novo depois da recusa.
    let mut final_ = conecta(porta);
    let (opcode, _, _) = responde(&mut final_, frame_pin_digitos(PIN_NOVO));
    assert_eq!(opcode, Opcode::AccountSecure, "PIN novo continua aceito");
    let (opcode, _, login) = responde(&mut final_, frame_entrar(0));
    assert_eq!(opcode, Opcode::CNFCharacterLogin, "entra no mundo");
    eprintln!(
        "PASS(final): PIN novo aceito e CNFCharacterLogin {} B",
        login.len()
    );
    linhas.push(format!("s2c_final_login={} B", login.len()));
    drop(final_);
    esperar_sem_sessao(&rt, &backend);

    // Limpeza: o personagem de prova sai pelo protocolo do cliente.
    let removido = common::remover_personagem(&endereco, CONTA, SENHA, PIN_NOVO, 0, PERSONAGEM);
    assert_eq!(
        removido,
        Some(Opcode::CNFDeleteCharacter),
        "personagem de prova removido pelo protocolo"
    );
    let roster = rt
        .block_on(backend.list_by_account(conta_id))
        .expect("roster");
    assert!(roster.is_empty(), "roster volta vazio");
    linhas.push("limpeza=CNFDeleteCharacter roster=0".to_string());
    despejar("pin-reset-ingame", &linhas);
}

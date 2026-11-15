//! PIN (segunda senha) **no canal real** (`127.0.0.1:8281`), medido em bytes e no banco.
//!
//! Fluxo mapeado no protocolo (e no porte):
//!
//! | etapa | opcode | tamanho | campos |
//! |---|---:|---:|---|
//! | login | `_MSG_AccountLogin` 0x020D | 116 | senha[12]@12, conta[16]@24, versão@92 |
//! | confirmação | `_MSG_CNFAccountLogin` 0x010A | 1996+12 | roster/`Selchar`@20 |
//! | **PIN** | `_MSG_AccountSecure` 0x0FDE | 32 | `numeric_token[6]`@12, `unknown[10]`@18, `change_numeric` i32@28 |
//! | recusa do PIN | `_MSG_AccountSecureFail` 0x0FDF | 12/13 | motivo@12 (`0x02` = `PinIncorreto`) |
//! | texto | `_MSG_MessagePanel` 0x0101 | 140 | texto@12 |
//!
//! Criação (`0x0203`, 36 B: slot@12, nome@16, classe@32), `_MSG_CharacterLogin` (0x0202, 36 B
//! do 7662: slot@12, force@16) e exclusão (`0x0204`, 44 B) **não carregam o PIN** — a exclusão
//! carrega a senha da conta em `password[12]@32` (`selection::delete_character`). O PIN é exigido
//! **uma vez**, na etapa de login, e é essa a porta que estes casos exercitam: com PIN gravado,
//! qualquer pacote de seleção antes do `0x0FDE` é recusado.
//!
//! Casos (cada um depois de gravar o PIN `2468` na conta):
//! 1. PIN errado (`0000`) → `0x0FDF` motivo 2 + painel; banco intacto;
//! 2. PIN vazio (6 B NUL) → `0x0FDF` motivo 2 + painel; banco intacto (nem `pin_failures` anda,
//!    porque token fora de 4..=6 dígitos nem chega a ser contado);
//! 3. PIN ausente (cliente mudo) → `0x0FDF` + painel **antes de fechar** (era FIN seco); banco
//!    intacto;
//! 4. PIN correto → `0x0FDE`, sessão viva no banco, criação conclui e o personagem aparece.
//!
//! Gated por `WYD_LIVE_CPSOCK` + `WYD_TEST_DSN`/`DATABASE_URL`; frames crus em `tmp/pin-canal/`.

#[path = "../common/mod.rs"]
mod common;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use wyd_db::{Accounts, Characters, PgBackend, PoolConfig, Sessions};
use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
use wyd_protocol::wire::frame::INITCODE;
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;
use wyd_protocol::{
    AccountLogin, AccountSecure, CreateCharacter, Header, MessagePanel, WirePayload,
};

const SENHA: &str = "qa12345678";
const PIN: &str = "2468";
const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/pin-canal");

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

/// `_MSG_AccountSecure` (0x0FDE) de 32 B: token[6]@12, unknown[10]@18, `change_numeric`@28.
fn frame_pin_bytes(token: &[u8; 6], change_numeric: i32) -> Vec<u8> {
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
        numeric_token: *token,
        unknown: [0u8; 10],
        change_numeric,
    }
    .encode_into(&mut frame)
    .expect("corpo do PIN");
    encode_in_place(&mut frame, 11).expect("ofuscação");
    frame
}

fn frame_pin(pin: &str) -> Vec<u8> {
    let mut token = [0u8; 6];
    let b = pin.len().min(6);
    token[..b].copy_from_slice(&pin.as_bytes()[..b]);
    frame_pin_bytes(&token, 0)
}

fn frame_pin_vazio() -> Vec<u8> {
    frame_pin_bytes(&[0u8; 6], 0)
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
        .set_read_timeout(Some(Duration::from_secs(20)))
        .expect("timeout");
    stream
}

/// Último frame **como veio do fio** (antes do XOR da palavra-chave), para o dump da
/// evidência mostrar byte a byte o que o canal escreveu, não só o decodificado.
static CRU: std::sync::OnceLock<std::sync::Mutex<Vec<u8>>> = std::sync::OnceLock::new();

fn cru() -> String {
    let guarda = CRU
        .get_or_init(|| std::sync::Mutex::new(Vec::new()))
        .lock()
        .expect("trava do frame cru");
    guarda.iter().map(|b| format!("{b:02x}")).collect()
}

fn ler_frame(stream: &mut TcpStream) -> Option<(Opcode, Vec<u8>)> {
    let mut head = [0u8; HEADER_SIZE];
    stream.read_exact(&mut head).ok()?;
    let size = usize::from(u16::from_le_bytes([head[0], head[1]]));
    let mut frame = vec![0u8; size];
    frame[..HEADER_SIZE].copy_from_slice(&head);
    if size > HEADER_SIZE {
        stream.read_exact(&mut frame[HEADER_SIZE..]).ok()?;
    }
    *CRU.get_or_init(|| std::sync::Mutex::new(Vec::new()))
        .lock()
        .expect("trava do frame cru") = frame.clone();
    decode_in_place(&mut frame).ok()?;
    let header = Header::decode(&frame).ok()?;
    Opcode::from_wire(header.message_type).map(|opcode| (opcode, frame))
}

fn hex(frame: &[u8]) -> String {
    frame.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn despejar(nome: &str, linhas: &[String]) {
    std::fs::create_dir_all(RELATORIO).expect("tmp/pin-canal");
    let mut saida = String::new();
    for linha in linhas {
        saida.push_str(linha);
        saida.push('\n');
    }
    std::fs::write(format!("{RELATORIO}/{nome}.txt"), saida).expect("dump");
}

/// Foto do banco antes/depois de cada tentativa: hash do PIN, roster e sessão viva da conta.
struct Foto {
    pin_hash: String,
    personagens: Vec<String>,
    sessao_viva: Option<i16>,
}

fn foto(rt: &tokio::runtime::Runtime, backend: &PgBackend, conta: &str) -> Foto {
    let pin_hash = rt
        .block_on(backend.pin_hash(conta))
        .expect("pin_hash")
        .expect("PIN gravado");
    let registro = rt.block_on(backend.find_by_login(conta)).expect("conta");
    let mut personagens: Vec<String> = rt
        .block_on(backend.list_by_account(registro.id))
        .expect("roster")
        .into_iter()
        .map(|p| p.name)
        .collect();
    personagens.sort();
    let sessao_viva = rt.block_on(backend.live_session_of(conta)).expect("sessão");
    Foto {
        pin_hash,
        personagens,
        sessao_viva,
    }
}

/// Texto do painel (`0x0101`, texto@12), recusando frame que não seja o painel.
fn texto_do_painel(painel: &[u8]) -> String {
    String::from_utf8_lossy(wyd_protocol::nul_trimmed(
        &MessagePanel::decode(painel).expect("painel").text,
    ))
    .to_string()
}

/// Medição crua do caminho "cliente mudo" (PIN ausente), que não falha: imprime quantos bytes
/// o canal manda antes de fechar. Serve para o antes/depois do conserto do FIN seco.
///
/// `DATABASE_URL=… WYD_LIVE_CPSOCK=127.0.0.1:8281 cargo test -p wyd-server --test
/// sistema_pin_canal -- --ignored --nocapture medir_cliente_mudo`
#[test]
#[ignore = "medição manual do caminho mudo (escreve em tmp/pin-canal/mudo.txt)"]
fn medir_cliente_mudo() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("PIN mudo") else {
        return;
    };
    let rt = runtime();
    let backend = rt
        .block_on(PgBackend::connect(&dsn, PoolConfig::local()))
        .expect("banco");
    let conta = "qamudoprobe".to_string();
    common::preparar_conta_de_teste(&rt, &backend, &conta, SENHA, PIN);

    let mut s = conectar(&endereco);
    s.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    s.write_all(&frame_login(&conta, SENHA)).expect("login");
    let (opcode, _) = ler_frame(&mut s).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin);
    s.write_all(&frame_pin(PIN)).expect("cadastro do PIN");
    assert_eq!(
        ler_frame(&mut s).map(|(o, _)| o),
        Some(Opcode::AccountSecure)
    );
    drop(s);
    std::thread::sleep(Duration::from_millis(600));

    // Agora entra de novo e **fica mudo** depois do `CNFAccountLogin`.
    let mut mudo = conectar(&endereco);
    mudo.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    mudo.write_all(&frame_login(&conta, SENHA)).expect("login");
    assert_eq!(
        ler_frame(&mut mudo).map(|(o, _)| o),
        Some(Opcode::CNFAccountLogin)
    );
    let inicio = std::time::Instant::now();
    let mut recebido: Vec<u8> = Vec::new();
    let mut buffer = [0u8; 512];
    loop {
        match mudo.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => recebido.extend_from_slice(&buffer[..n]),
            Err(_) => break,
        }
    }
    let segundos = inicio.elapsed().as_secs_f32();
    let linhas = vec![format!(
        "conta={conta} silencio_s={segundos:.1} bytes={} hex={}",
        recebido.len(),
        hex(&recebido)
    )];
    despejar("mudo", &linhas);
    eprintln!(
        "MUDO: {segundos:.1}s de espera, {} B recebidos antes do fechamento: {}",
        recebido.len(),
        hex(&recebido)
    );
}

#[test]
fn pin_no_canal_real_recusa_errado_vazio_e_ausente_e_aceita_o_certo() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("PIN no canal") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );
    // Conta **fixa** de QA: reuso em vez de conta nova por execução (senão `accounts` cresce e
    // o banco não termina igual ao início). O preparo zera roster e grava PIN `2468` — a
    // primeira conexão abaixo passa a ser verificação em vez de cadastro, com o mesmo `0x0FDE`.
    let conta = "qapinprobe".to_string();
    common::preparar_conta_de_teste(&rt, &backend, &conta, SENHA, PIN);
    let mut linhas: Vec<String> = vec![format!("conta={conta}")];

    // 0. Grava o PIN (primeiro `0x0FDE` cadastra) e fecha.
    let mut primeiro = conectar(&endereco);
    primeiro
        .write_all(&INITCODE.to_le_bytes())
        .expect("initcode");
    primeiro
        .write_all(&frame_login(&conta, SENHA))
        .expect("login");
    let (opcode, _) = ler_frame(&mut primeiro).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin, "login aceito");
    primeiro
        .write_all(&frame_pin(PIN))
        .expect("cadastro do PIN");
    let (opcode, aceito) = ler_frame(&mut primeiro).expect("0x0FDE do cadastro");
    assert_eq!(opcode, Opcode::AccountSecure, "PIN inicial gravado");
    eprintln!("PASS(pin_cadastrado): AccountSecure [{}]", hex(&aceito));
    linhas.push(format!(
        "c2s size={} hex={}",
        AccountSecure::WIRE_LEN,
        hex(&frame_pin(PIN))
    ));
    linhas.push(format!("s2c size={} hex={}", aceito.len(), hex(&aceito)));
    drop(primeiro);
    std::thread::sleep(Duration::from_millis(600));

    let base = foto(&rt, &backend, &conta);
    assert!(
        base.sessao_viva.is_none(),
        "nenhuma sessão viva antes da prova"
    );
    eprintln!(
        "BD(antes): pin_hash={} personagens={:?} sessao={:?}",
        &base.pin_hash[..12],
        base.personagens,
        base.sessao_viva
    );

    // 1..3: três recusas, cada uma numa conexão nova. O banco tem de ficar idêntico.
    let recusas: [(&str, Vec<u8>); 3] = [
        ("pin_errado", frame_pin("0000")),
        ("pin_vazio", frame_pin_vazio()),
        // `pin_ausente` não envia frame nenhum: fica mudo depois do `CNFAccountLogin`.
        ("pin_ausente", Vec::new()),
    ];
    for (caso, envio) in recusas {
        let mut s = conectar(&endereco);
        s.write_all(&INITCODE.to_le_bytes()).expect("initcode");
        s.write_all(&frame_login(&conta, SENHA)).expect("login");
        let (opcode, _) = ler_frame(&mut s).expect("CNF do login");
        assert_eq!(opcode, Opcode::CNFAccountLogin, "{caso}: login aceito");
        if !envio.is_empty() {
            s.write_all(&envio).expect("PIN");
        } else {
            // Cliente **vivo** com o diálogo aberto não pode ser recusado pelo prazo curto: o
            // jogador real leva mais que os 10 s do handshake para digitar a segunda senha
            // (medido em 2026-11-15, conta `admin`: `reason=pin_ausente kind=WouldBlock`).
            // Aqui: 11 s de silêncio **sem** recusa, e só o FIN recebe `0x0FDF` + painel.
            std::thread::sleep(Duration::from_secs(11));
            s.set_read_timeout(Some(Duration::from_millis(300)))
                .expect("timeout de sondagem");
            let mut sonda = [0u8; 1];
            match s.read(&mut sonda) {
                Err(erro)
                    if matches!(
                        erro.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                outro => panic!("{caso}: canal recusou antes do FIN ({outro:?})"),
            }
            s.set_read_timeout(Some(Duration::from_secs(20)))
                .expect("timeout de leitura");
            let _ = s.shutdown(std::net::Shutdown::Write);
        }

        // Primeiro frame da etapa de PIN: `0x0FDF` — nunca `CNFAccountLogin` de novo, nunca
        // `CNFNewCharacter`, nunca FIN seco.
        let (opcode, recusa) = ler_frame(&mut s).unwrap_or_else(|| panic!("{caso}: sem frame"));
        assert_eq!(
            opcode,
            Opcode::AccountSecureFail,
            "{caso}: recusa de PIN tem de ser visível (`0x0FDF`)"
        );
        assert_eq!(
            u16::from_le_bytes([recusa[4], recusa[5]]),
            Opcode::AccountSecureFail.wire(),
            "{caso}: Type@4"
        );
        assert_eq!(recusa.len(), 13, "{caso}: `0x0FDF` com motivo no byte 12");
        assert_eq!(recusa[12], 0x02, "{caso}: motivo@12 = PinIncorreto");

        let (opcode, painel) = ler_frame(&mut s).expect("painel da recusa");
        assert_eq!(opcode, Opcode::MessagePanel, "{caso}: painel `0x0101`");
        assert_eq!(painel.len(), 140, "{caso}: painel de 140 B");
        let texto = texto_do_painel(&painel);
        assert!(!texto.is_empty(), "{caso}: painel sem texto");

        // Depois do painel a conexão encerra (o PIN não abre sessão).
        let mut resto = [0u8; 1];
        let fechou = matches!(s.read(&mut resto), Ok(0) | Err(_));
        assert!(fechou, "{caso}: servidor tem de encerrar depois da recusa");

        let depois = foto(&rt, &backend, &conta);
        assert_eq!(
            base.pin_hash, depois.pin_hash,
            "{caso}: PIN errado/vazio/ausente não pode reescrever `accounts.pin_hash`"
        );
        assert_eq!(
            base.personagens, depois.personagens,
            "{caso}: nenhuma escrita em `characters`"
        );
        assert!(
            depois.sessao_viva.is_none(),
            "{caso}: nenhuma sessão aberta (`character_session`)"
        );
        eprintln!(
            "PASS({caso}): AccountSecureFail [{}] motivo@12=0x{:02x} | painel {} B \"{}\" | banco intacto (pin_hash, personagens={}, sessao=None)",
            hex(&recusa),
            recusa[12],
            painel.len(),
            texto,
            depois.personagens.len()
        );
        linhas.push(format!("caso={caso}"));
        if envio.is_empty() {
            linhas.push("c2s size=0 (cliente mudo: PIN ausente)".to_string());
        } else {
            linhas.push(format!("c2s size={} hex={}", envio.len(), hex(&envio)));
        }
        linhas.push(format!("s2c size={} hex={}", recusa.len(), hex(&recusa)));
        linhas.push(format!("s2c fio={}", cru()));
        linhas.push(format!("s2c size={} hex={}", painel.len(), hex(&painel)));
        linhas.push(format!("s2c fio={}", cru()));
        linhas.push(format!("painel_texto={texto}"));
        linhas.push("banco=inalterado".to_string());
        drop(s);
        std::thread::sleep(Duration::from_millis(400));
    }

    // 4. PIN correto: `0x0FDE`, sessão viva e criação concluída.
    let mut certo = conectar(&endereco);
    certo.write_all(&INITCODE.to_le_bytes()).expect("initcode");
    certo.write_all(&frame_login(&conta, SENHA)).expect("login");
    let (opcode, _) = ler_frame(&mut certo).expect("CNF do login");
    assert_eq!(opcode, Opcode::CNFAccountLogin);
    certo.write_all(&frame_pin(PIN)).expect("PIN correto");
    let (opcode, ok_pin) = ler_frame(&mut certo).expect("0x0FDE");
    assert_eq!(opcode, Opcode::AccountSecure, "PIN correto é aceito");
    assert_eq!(ok_pin.len(), HEADER_SIZE, "`0x0FDE` header-only (12 B)");
    let sessao = foto(&rt, &backend, &conta);
    assert!(
        sessao.sessao_viva.is_some(),
        "PIN correto abre sessão (`character_session`), e só ele"
    );

    let nome = format!("Pp{}", &conta[4..10]);
    certo
        .write_all(&frame_criacao(1, &nome))
        .expect("criação depois do PIN");
    let (opcode, criado) = ler_frame(&mut certo).expect("CNFNewCharacter");
    assert_eq!(opcode, Opcode::CNFNewCharacter, "operação conclui");
    let depois = foto(&rt, &backend, &conta);
    assert!(
        depois.personagens.iter().any(|p| p == &nome),
        "personagem `{nome}` no banco: {:?}",
        depois.personagens
    );
    eprintln!(
        "PASS(pin_correto): AccountSecure [{}] | sessao={:?} | CNFNewCharacter {} B | banco={:?}",
        hex(&ok_pin),
        sessao.sessao_viva,
        criado.len(),
        depois.personagens
    );
    linhas.push("caso=pin_correto".to_string());
    linhas.push(format!(
        "c2s size={} hex={}",
        AccountSecure::WIRE_LEN,
        hex(&frame_pin(PIN))
    ));
    linhas.push(format!("s2c size={} hex={}", ok_pin.len(), hex(&ok_pin)));
    linhas.push(format!("s2c size={} hex={}", criado.len(), hex(&criado)));
    linhas.push(format!("sessao_viva={:?}", sessao.sessao_viva));
    linhas.push(format!("banco_personagens={:?}", depois.personagens));
    despejar("pin", &linhas);

    drop(certo);
    let liberou = common::esperar_ate(Duration::from_secs(10), || {
        rt.block_on(backend.live_session_of(&conta))
            .expect("sessão")
            .is_none()
    });
    assert!(liberou, "sessão liberada no fechamento da conexão");

    // Devolve o banco ao estado inicial: o personagem de prova sai pelo protocolo (senha da
    // conta em `password@32`) e a conta volta a zero — duas execuções seguidas deixam as mesmas
    // contagens de sessões e personagens.
    let remocao = common::remover_personagem(&endereco, &conta, SENHA, PIN, 1, &nome);
    assert_eq!(
        remocao,
        Some(Opcode::CNFDeleteCharacter),
        "personagem de prova removido pelo protocolo"
    );
    let limpo = foto(&rt, &backend, &conta);
    assert!(
        limpo.personagens.is_empty(),
        "roster volta a zero: {:?}",
        limpo.personagens
    );
    assert!(limpo.sessao_viva.is_none(), "nenhuma sessão viva no fim");
    eprintln!(
        "PASS(limpeza): CNFDeleteCharacter | roster={:?} sessao={:?}",
        limpo.personagens, limpo.sessao_viva
    );
    linhas.push("limpeza=CNFDeleteCharacter roster=0 sessao=None".to_string());
    despejar("pin", &linhas);
}

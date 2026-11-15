//! Helpers compartilhados dos testes que precisam de Postgres.
//!
//! O gate é único por crate: qualquer teste de sistema usa `dsn_ou_skip` e recebe
//! a mesma mensagem `SKIP:` com todas as variáveis aceitas.

#![allow(dead_code)]

use std::time::{Duration, Instant};

/// DSN de teste, na ordem de precedência do repositório.
pub fn dsn() -> Option<String> {
    ["WYD_TEST_DSN", "WYD_TEST_DATABASE_URL", "DATABASE_URL"]
        .iter()
        .filter_map(|var| std::env::var(var).ok())
        .map(|valor| valor.trim().to_string())
        .find(|valor| !valor.is_empty())
}

/// Devolve o DSN ou imprime o `SKIP:` explícito e devolve `None`.
///
/// Com `WYD_TEST_REQUIRED=1` (ligado no job `sistema` do CI, DEVOPS-07) a ausência de DSN vira
/// **falha**: teste que precisa de banco não pode passar por omissão — era assim que o CI ficava
/// verde com zero cobertura dos caminhos mais frágeis (login, entrada no mundo, cena).
///
/// # Panics
///
/// Se `WYD_TEST_REQUIRED` estiver ligado e nenhum DSN for encontrado.
pub fn dsn_ou_skip(contexto: &str) -> Option<String> {
    if let Some(dsn) = dsn() {
        return Some(dsn);
    }
    if obrigatorio_sem_skip() {
        panic!(
            "WYD_TEST_REQUIRED=1 sem DSN — {contexto} não pode ser pulado; defina \
             WYD_TEST_DSN/WYD_TEST_DATABASE_URL/DATABASE_URL"
        );
    }
    eprintln!(
        "SKIP: WYD_TEST_DSN/WYD_TEST_DATABASE_URL/DATABASE_URL ausente — {contexto} não executado"
    );
    None
}

/// `WYD_TEST_REQUIRED` ligado? (qualquer valor diferente de vazio/`0`).
fn obrigatorio_sem_skip() -> bool {
    std::env::var("WYD_TEST_REQUIRED")
        .map(|valor| {
            let valor = valor.trim();
            !valor.is_empty() && valor != "0"
        })
        .unwrap_or(false)
}

/// Prepara uma conta **fixa** de QA para a suíte viva e devolve o id dela.
///
/// A suíte criava uma conta nova por execução (`conta_unica`) e `accounts` crescia a cada
/// rodada — banco não terminava igual ao início. Aqui a conta é reutilizada: cria só se faltar,
/// reescreve a credencial (argon2id), **normaliza o PIN** para o valor da suíte, zera falhas /
/// bloqueio de PIN e limpa o roster de qualquer personagem deixado por uma execução anterior
/// (inclusive interrompida). Assim cada execução começa exatamente com o mesmo estado de conta.
///
/// # Panics
///
/// Se o banco não responder ou uma escrita falhar: preparo de conta não pode ser pulado em
/// silêncio.
pub fn preparar_conta_de_teste(
    rt: &tokio::runtime::Runtime,
    backend: &wyd_db::PgBackend,
    login: &str,
    senha: &str,
    pin: &str,
) -> i64 {
    use wyd_db::{Accounts, Characters, Role};

    let id = match rt.block_on(backend.find_by_login(login)) {
        Ok(conta) => conta.id,
        Err(_) => {
            rt.block_on(backend.create_account(login, Role::Player))
                .expect("conta de QA criada");
            rt.block_on(backend.find_by_login(login))
                .expect("conta de QA criada")
                .id
        }
    };
    let hash = wyd_admin::auth::hash_secret(senha).expect("argon2id da senha de QA");
    rt.block_on(backend.set_credentials(login, &hash, Role::Player))
        .expect("senha da conta de QA");
    let hash_pin = wyd_admin::auth::hash_secret(pin).expect("argon2id do PIN de QA");
    rt.block_on(backend.set_pin_hash(login, &hash_pin))
        .expect("PIN da conta de QA");
    rt.block_on(backend.pin_success(login))
        .expect("tentativas de PIN zeradas");

    let antigos = rt
        .block_on(backend.list_by_account(id))
        .expect("roster da conta de QA");
    for personagem in antigos {
        rt.block_on(backend.soft_delete(personagem.id))
            .expect("personagem de execução anterior removido");
    }
    id
}

/// Igual a [`preparar_conta_de_teste`], mas deixa a conta **sem PIN** (primeira entrada).
///
/// Usado pelo teste de autenticação, que assere o caminho de **cadastro** do PIN (`mode=
/// Registered`) e precisa disso de forma repetível — a conta fixa de QA nunca pode acumular
/// estado entre execuções.
///
/// # Panics
///
/// Se o banco não responder ou a limpeza do PIN falhar.
pub fn preparar_conta_de_teste_sem_pin(
    rt: &tokio::runtime::Runtime,
    backend: &wyd_db::PgBackend,
    login: &str,
    senha: &str,
) -> i64 {
    use wyd_db::Accounts;

    let id = preparar_conta_de_teste(rt, backend, login, senha, "2468");
    rt.block_on(backend.clear_pin_hash(login))
        .expect("PIN limpo para a primeira entrada");
    id
}

/// Remove um personagem de prova **pelo protocolo do cliente**, sem `panic!`.
///
/// Serve para os testes vivos deixarem o banco como o encontraram: conneta, faz
/// `_MSG_AccountLogin` + `_MSG_AccountSecure` e manda `_MSG_DeleteCharacter` com a senha da
/// conta em `password[12]@32` (o canal confere o Argon2id antes do soft-delete). Devolve o
/// opcode da resposta (`CNFDeleteCharacter` quando removeu) ou `None` se a conversa não
/// chegou ao fim — por isso pode ser chamado também de um `Drop`, onde pânico abortaria o
/// processo.
pub fn remover_personagem(
    endereco: &str,
    conta: &str,
    senha: &str,
    pin: &str,
    slot: i32,
    nome: &str,
) -> Option<wyd_protocol::wire::opcode::Opcode> {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
    use wyd_protocol::payload::WirePayload;
    use wyd_protocol::wire::header::HEADER_SIZE;
    use wyd_protocol::{AccountLogin, AccountSecure, DeleteCharacter, Header};

    let mut texto = [0u8; 16];
    let bytes = conta.trim().as_bytes();
    let limite = bytes.len().min(texto.len());
    texto[..limite].copy_from_slice(&bytes[..limite]);
    let mut senha_bytes = [0u8; 12];
    let bytes = senha.as_bytes();
    let limite = bytes.len().min(senha_bytes.len());
    senha_bytes[..limite].copy_from_slice(&bytes[..limite]);

    let quadro = |opcode: wyd_protocol::wire::opcode::Opcode, len: usize| {
        let mut frame = vec![0u8; len];
        Header {
            size: u16::try_from(len).ok()?,
            key_word: 11,
            checksum: 0,
            message_type: opcode.wire(),
            id: 1,
            client_tick: 1,
        }
        .encode(&mut frame)
        .ok()?;
        Some(frame)
    };
    let escrever = |stream: &mut TcpStream, frame: &mut Vec<u8>| -> Option<()> {
        encode_in_place(frame, 11).ok()?;
        stream.write_all(frame).ok()
    };
    let ler = |stream: &mut TcpStream| -> Option<wyd_protocol::wire::opcode::Opcode> {
        let mut head = [0u8; HEADER_SIZE];
        stream.read_exact(&mut head).ok()?;
        let size = usize::from(u16::from_le_bytes([head[0], head[1]]));
        let mut frame = vec![0u8; size];
        frame[..HEADER_SIZE].copy_from_slice(&head);
        if size > HEADER_SIZE {
            stream.read_exact(&mut frame[HEADER_SIZE..]).ok()?;
        }
        let _ = decode_in_place(&mut frame);
        let header = Header::decode(&frame).ok()?;
        wyd_protocol::wire::opcode::Opcode::from_wire(header.message_type)
    };

    let mut stream = TcpStream::connect(endereco).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .ok()?;
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .ok()?;
    stream
        .write_all(&wyd_protocol::wire::frame::INITCODE.to_le_bytes())
        .ok()?;

    let mut login = quadro(
        wyd_protocol::wire::opcode::Opcode::AccountLogin,
        AccountLogin::WIRE_LEN,
    )?;
    AccountLogin {
        account_password: senha_bytes,
        account_name: texto,
        zero: [0u8; 52],
        client_version: 12_000,
        db_need_save: 1,
        adapter_name: [0; 4],
    }
    .encode_into(&mut login)
    .ok()?;
    escrever(&mut stream, &mut login)?;
    if ler(&mut stream)? != wyd_protocol::wire::opcode::Opcode::CNFAccountLogin {
        return None;
    }

    let mut token = [0u8; 6];
    let bytes = pin.as_bytes();
    let limite = bytes.len().min(token.len());
    token[..limite].copy_from_slice(&bytes[..limite]);
    let mut pin_frame = quadro(
        wyd_protocol::wire::opcode::Opcode::AccountSecure,
        AccountSecure::WIRE_LEN,
    )?;
    AccountSecure {
        numeric_token: token,
        unknown: [0u8; 10],
        change_numeric: 0,
    }
    .encode_into(&mut pin_frame)
    .ok()?;
    escrever(&mut stream, &mut pin_frame)?;
    if ler(&mut stream)? != wyd_protocol::wire::opcode::Opcode::AccountSecure {
        return None;
    }

    let mut nome_bytes = [0u8; 16];
    let bytes = nome.as_bytes();
    let limite = bytes.len().min(nome_bytes.len());
    nome_bytes[..limite].copy_from_slice(&bytes[..limite]);
    let mut remocao = quadro(
        wyd_protocol::wire::opcode::Opcode::DeleteCharacter,
        DeleteCharacter::WIRE_LEN,
    )?;
    DeleteCharacter {
        slot,
        mob_name: nome_bytes,
        password: senha_bytes,
    }
    .encode_into(&mut remocao)
    .ok()?;
    escrever(&mut stream, &mut remocao)?;
    ler(&mut stream)
}

/// Espera `cond` virar verdadeira até o prazo, amostrando a cada 25 ms.
pub fn esperar_ate(prazo: Duration, cond: impl FnMut() -> bool) -> bool {
    esperar_ate_a_cada(prazo, Duration::from_millis(25), cond)
}

/// Espera `cond` com um intervalo próprio (ex.: scrape do Prometheus a cada 1 s).
///
/// Substitui `sleep` fixo de sincronização: o teste só espera o tempo necessário
/// e falha com o prazo esgotado quando a condição nunca acontece.
pub fn esperar_ate_a_cada(
    prazo: Duration,
    intervalo: Duration,
    mut cond: impl FnMut() -> bool,
) -> bool {
    let limite = Instant::now() + prazo;
    loop {
        if cond() {
            return true;
        }
        if Instant::now() >= limite {
            return false;
        }
        std::thread::sleep(intervalo);
    }
}

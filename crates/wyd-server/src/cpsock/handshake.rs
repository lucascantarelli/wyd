//! Submódulo de `cpsock` (ADR-0026, etapa 2).

use super::*;

/// Resultado de um handshake aceito (o que ficou provado sobre a conexão).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandshakeOk {
    /// Versão declarada pelo cliente no login.
    pub client_version: i32,
    /// Chave de ofuscação usada na resposta (a mesma do frame de entrada).
    pub key_word: u8,
    /// Sessão aberta no banco (`None` quando o processo roda sem cerca, ex.: teste unitário).
    pub session_id: Option<i64>,
}

/// Falhas de handshake — todas viram log estruturado + fechamento da conexão.
#[derive(Debug)]
pub enum HandshakeError {
    /// Primeiros 4 bytes diferentes de [`INITCODE`].
    InitCode { received: u32 },
    /// `Size` fora de `[12, 8192]` ou frame incompleto.
    Frame(String),
    /// Checksum estrito divergiu.
    Checksum { computed: u8, declared: u8 },
    /// Opcode que este processo ainda não atende.
    OpcodeNotServed { opcode: Opcode },
    /// Versão de cliente recusada pelo gate (ADR-0020).
    Version { received: i32, expected: u16 },
    /// Cerca de sessão recusou (conta já em jogo) ou o banco não respondeu.
    Session(String),
    /// Senha de login recusada (ou banco indisponível): sem `CNF`, sem sessão.
    Auth(String),
    /// O cliente não enviou `_MSG_AccountSecure` no prazo: conexão fechada sem sessão.
    PinTimeout,
    /// PIN recusado: `_MSG_AccountSecureFail` enviado e conexão fechada sem sessão.
    Pin(String),
    /// I/O: cliente saiu no meio do handshake.
    Io(String),
}

impl core::fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InitCode { received } => {
                write!(f, "INITCODE divergente: {received:#010x}")
            }
            Self::Frame(motivo) => write!(f, "frame inválido: {motivo}"),
            Self::Checksum { computed, declared } => {
                write!(
                    f,
                    "checksum divergiu: calculado {computed}, declarado {declared}"
                )
            }
            Self::OpcodeNotServed { opcode } => {
                write!(f, "opcode sem handler: {}", opcode.const_name())
            }
            Self::Version { received, expected } => {
                write!(
                    f,
                    "versão de cliente {received} fora do gate (esperado {expected})"
                )
            }
            Self::Session(motivo) => write!(f, "sessão: {motivo}"),
            Self::Auth(motivo) => write!(f, "autenticação: {motivo}"),
            Self::PinTimeout => write!(f, "cliente não enviou a senha numérica"),
            Self::Pin(motivo) => write!(f, "senha numérica: {motivo}"),
            Self::Io(motivo) => write!(f, "io: {motivo}"),
        }
    }
}

impl std::error::Error for HandshakeError {}

impl From<io::Error> for HandshakeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

pub(super) fn handshake(
    stream: &mut TcpStream,
    world: &World,
    gate: VersionGate,
    auth: Option<&Authenticator>,
    selection: Option<&SelectionStore>,
) -> Result<(HandshakeOk, String), HandshakeError> {
    let mut init = [0u8; 4];
    stream.read_exact(&mut init)?;
    let received = u32::from_le_bytes(init);
    if received != INITCODE {
        NET.packet_rejected("ip");
        return Err(HandshakeError::InitCode { received });
    }

    let mut head = [0u8; HEADER_SIZE];
    stream.read_exact(&mut head)?;
    let header = Header::decode(&head).map_err(|error| HandshakeError::Frame(error.to_string()))?;
    // Só o HEADER chegou até aqui: o tamanho vem do próprio cabeçalho (validado), e
    // `split_frame` exige o frame inteiro — por isso `frame_len`, não `split_frame`.
    let declared = header
        .frame_len()
        .map_err(|error| HandshakeError::Frame(error.to_string()))?;
    let mut frame = vec![0u8; declared];
    frame[..HEADER_SIZE].copy_from_slice(&head);
    stream.read_exact(&mut frame[HEADER_SIZE..])?;

    let computed =
        decode_in_place(&mut frame).map_err(|error| HandshakeError::Frame(error.to_string()))?;
    checksum::verify(computed, frame[3], ChecksumPolicy::Strict).map_err(|_| {
        HandshakeError::Checksum {
            computed,
            declared: frame[3],
        }
    })?;

    let valid = inspect(&frame).map_err(|error| HandshakeError::Frame(error.to_string()))?;
    if valid.opcode != Opcode::AccountLogin {
        NET.packet_rejected("timeout");
        return Err(HandshakeError::OpcodeNotServed {
            opcode: valid.opcode,
        });
    }

    let login =
        AccountLogin::decode(&frame).map_err(|error| HandshakeError::Frame(error.to_string()))?;
    gate.check(login.client_version, &NET)
        .map_err(|_| HandshakeError::Version {
            received: login.client_version,
            expected: gate.expected(),
        })?;
    let conta = String::from_utf8_lossy(login.account_name_bytes()).to_string();
    // Senha da conta **antes** de qualquer confirmação: senha errada não abre sessão, não grava
    // linha e não recebe roster nem `CNFAccountLogin`. Sem porta de autenticação (teste de
    // framing/gate) o passo é pulado.
    if let Some(auth) = auth {
        if let Err(error) = auth.verify_password(&conta, &login.account_password) {
            NET.packet_rejected("auth");
            log(
                "warn",
                "login",
                &format!("result=refused account={conta} reason={error}"),
            );
            // Provado no binário (`docs/rust/evidencias/login-tela-handler-2026-11-14.md`): a
            // função da etapa de login (`0x4b3e83`) compara **só** `0x0101` (visibilidade,
            // `0x4b3ec1`), `0x010A` (`0x4b3f45`), `0x011D` (`0x4b4071`) e `0x011C` (`0x4b4082`);
            // não existe `cmp ...,0x119` nem `0x11b` ali, e o `0x0101` não chama `0x404109`.
            //
            // A recusa **não** manda `CNFAccountLogin` nem roster: a variante com CNF de roster
            // vazio foi medida no canal vivo e rejeitada — ela tira o cliente da tela de login e
            // o faz progredir para a tela de personagens (ver
            // `docs/rust/evidencias/login-tela-handler-2026-11-14.md`, seção "Retratação").
            //
            // Na etapa de login (`0x4b3e83`) o único braço que chega à rotina de mensagem
            // `0x404109` é o de `0x011D`/`0x011C` (`0x4b4071`/`0x4b4082`): `push 0xfa0` (4000 ms)
            // + `push 0x109aab8` (id de mensagem do cliente) — o frame **não** é passado, então o
            // texto vem do binário, nunca do payload. `0x0119` não tem `cmp` nessa função.
            let _ = reply(stream, header.key_word, Opcode::AlreadyPlaying);
            manter_conexao_aberta_apos_recusa(stream);
            return Err(HandshakeError::Auth(error.to_string()));
        }
    }

    NET.packet_accepted();
    NET.handshake_opened();
    // Linha única e auto-descritiva do login aceito (a sessão abre só depois do PIN).
    log(
        "info",
        "login",
        &format!(
            "result=password_ok account={conta} version={} channels={}",
            login.client_version,
            world.game_channel_count()
        ),
    );

    // Corpo com os personagens da conta (`STRUCT_SELCHAR`), o cofre (`Cargo[128]`) e o
    // `Coin`: tudo do banco. Falha de leitura devolve lista vazia/zero — o cliente vê "sem
    // personagem", nunca dado inventado.
    let snapshot = match selection.map(|store| store.login_snapshot(&conta)) {
        Some(Ok(snapshot)) => snapshot,
        Some(Err(error)) => {
            log(
                "warn",
                "login",
                &format!("result=empty_roster account={conta} reason={error}"),
            );
            LoginSnapshot::default()
        }
        None => LoginSnapshot::default(),
    };
    let cargo = cargo_items(&snapshot.vault);
    let coin = i32::try_from(snapshot.coin).unwrap_or(i32::MAX);
    let corpo = account_login_body(&snapshot.roster, &cargo, coin, &conta);
    reply_with_body(
        stream,
        header.key_word,
        ID_SELCHAR,
        Opcode::CNFAccountLogin,
        &corpo,
    )
    .then_some(())
    .ok_or_else(|| HandshakeError::Io("falha ao escrever _MSG_CNFAccountLogin".to_string()))?;

    Ok((
        HandshakeOk {
            client_version: login.client_version,
            key_word: header.key_word,
            session_id: None,
        },
        conta,
    ))
}

/// Mantém o soquete aberto depois do painel de recusa de senha.
///
/// O caminho que o jogador **vê** (senha errada na exclusão de personagem) roda dentro do laço do
/// canal: o painel chega e a conexão **continua viva**, sem FIN. A recusa de login fechava o
/// soquete no mesmo instante do painel, e um FIN imediato é justamente o que apaga a janela de
/// erro do cliente antes de ela ser desenhada. Aqui a conexão é drenada até o cliente fechar, no
/// máximo `SEGUNDOS_DE_JANELA` — o mesmo comportamento do caso visível, sem sessão aberta.
fn manter_conexao_aberta_apos_recusa(stream: &mut TcpStream) {
    use std::time::{Duration, Instant};

    const SEGUNDOS_DE_JANELA: u64 = 30;
    let _ = stream.flush();
    let _ = stream.set_read_timeout(Some(Duration::from_millis(250)));
    let prazo = Instant::now() + Duration::from_secs(SEGUNDOS_DE_JANELA);
    let mut lixo = [0u8; 512];
    while Instant::now() < prazo {
        match stream.read(&mut lixo) {
            Ok(0) => break,
            Ok(_) => {}
            Err(erro)
                if matches!(
                    erro.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};

    fn mundo() -> World {
        World::new(vec![wyd_db::ServerChannel {
            group_id: 1,
            channel: 1,
            host: "127.0.0.1".to_string(),
            status_url: String::new(),
            enabled: true,
        }])
        .expect("foto de canais válida")
    }

    /// O gate de INITCODE tem de recusar **antes** de ler o frame: com 4 B errados e nada
    /// mais, o `read_exact` do frame nunca chega, então o erro precisa ser `InitCode`.
    #[test]
    fn initcode_divergente_e_recusado_antes_de_ler_o_frame() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("porta efêmera");
        let endereco = listener.local_addr().expect("endereço");
        let cliente = std::thread::spawn(move || {
            let mut stream = TcpStream::connect(endereco).expect("conecta");
            stream
                .write_all(&0xDEAD_BEEFu32.to_le_bytes())
                .expect("initcode errado");
        });
        let (mut servidor, _) = listener.accept().expect("aceita");
        servidor
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .expect("timeout");
        let erro = match handshake(&mut servidor, &mundo(), VersionGate::default(), None, None) {
            Ok(_) => panic!("INITCODE divergente foi aceito"),
            Err(erro) => erro,
        };
        assert!(
            matches!(erro, HandshakeError::InitCode { received } if received == 0xDEAD_BEEF),
            "erro: {erro:?}"
        );
        cliente.join().expect("thread do cliente");
    }
}

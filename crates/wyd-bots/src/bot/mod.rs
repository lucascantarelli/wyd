//! Cliente do canal usado pelos bots do Campo de Treinamento.
//!
//! ADR-0027: `config` (raça/conta), `cena` (leitura de `CreateMob`), `campanha`
//! (quest/etapas), `comercio`, `combate`, `movimento` e `recusa`; este arquivo
//! guarda o socket, a autenticação e os helpers de frame.

pub(crate) use std::io::{Error, ErrorKind, Read, Write};
pub(crate) use std::net::{TcpStream, ToSocketAddrs};
pub(crate) use std::time::Duration;
pub(crate) use wyd_protocol::codec::keyword::{decode_in_place, encode_in_place};
pub(crate) use wyd_protocol::payload::combat::{Attack, TargetDamage};
pub(crate) use wyd_protocol::payload::movement::Action;
pub(crate) use wyd_protocol::payload::quest::Quest;
pub(crate) use wyd_protocol::payload::scene::CreateMob;
pub(crate) use wyd_protocol::payload::WirePayload;
pub(crate) use wyd_protocol::wire::frame::INITCODE;
pub(crate) use wyd_protocol::wire::header::{Header, HEADER_SIZE};
pub(crate) use wyd_protocol::wire::opcode::Opcode;
pub(crate) use wyd_protocol::{
    AccountLogin, AccountSecure, CharacterLogin, CreateCharacter, MessagePanel,
};

mod campanha;
mod cena;
mod combate;
mod comercio;
mod config;
mod movimento;
mod recusa;

pub use campanha::parse_etapa;
pub use cena::{nome_do_campo, SceneMob};
pub use config::{BotConfig, Race};
pub use recusa::motivo_da_recusa;

/// Cliente do canal para um bot.
pub struct Bot {
    stream: TcpStream,
    key_word: u8,
    race: Race,
    /// Corpo cru do último `_MSG_CNFAccountLogin`/`_MSG_CNFNewCharacter` (o `Selchar`).
    /// É o pacote que o cliente 7662 usa para desenhar o nível na tela de seleção.
    roster: Option<Vec<u8>>,
    /// Corpo cru do último `_MSG_CNFCharacterLogin` (`STRUCT_MOB` a partir do offset 4).
    login: Option<Vec<u8>>,
    /// Frames crus (header + corpo, já de-ofuscados) dos pacotes que carregam nível:
    /// `CNFAccountLogin`, `CNFNewCharacter`, `CNFCharacterLogin` e `CreateMob`. É o byte que
    /// o cliente 7662 recebe; serve para um parser externo medir o offset do legado.
    raw: Vec<Vec<u8>>,
}

impl Bot {
    /// Abre o socket e faz o handshake completo; devolve o bot pronto para o `_MSG_Quest`.
    pub fn connect(config: &BotConfig, race: Race) -> std::io::Result<Self> {
        let endereco = config
            .addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error::new(ErrorKind::AddrNotAvailable, "endereco do canal"))?;
        let stream = TcpStream::connect_timeout(&endereco, Duration::from_secs(5))?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        let mut bot = Self {
            stream,
            key_word: 11,
            race,
            roster: None,
            login: None,
            raw: Vec::new(),
        };
        // Servidor que encerra sem responder (ex.: fail só de cabeçalho já consumido, ou
        // socket fechado na recusa) não pode virar "failed to fill whole buffer": a tela
        // traduz o EOF para uma frase acionável, como qualquer outra recusa.
        bot.handshake(config).map_err(|erro| {
            if matches!(
                erro.kind(),
                ErrorKind::UnexpectedEof | ErrorKind::ConnectionReset | ErrorKind::BrokenPipe
            ) {
                Error::new(
                    ErrorKind::ConnectionAborted,
                    "Conexão encerrada pelo servidor antes da resposta — confira a senha, o PIN \
                     e o slot do personagem e tente novamente.",
                )
            } else {
                erro
            }
        })?;
        Ok(bot)
    }

    /// Raça deste bot.
    #[must_use]
    pub fn race(&self) -> Race {
        self.race
    }

    fn autenticar(&mut self, config: &BotConfig) -> std::io::Result<()> {
        self.stream.write_all(&INITCODE.to_le_bytes())?;

        let mut nome = [0u8; 16];
        let bytes = config.account.as_bytes();
        if bytes.len() > nome.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "conta acima de 16 bytes",
            ));
        }
        nome[..bytes.len()].copy_from_slice(bytes);
        let mut senha = [0u8; 12];
        let bytes = config.password.as_bytes();
        if bytes.len() > senha.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "senha acima de 12 bytes",
            ));
        }
        senha[..bytes.len()].copy_from_slice(bytes);
        self.send(Opcode::AccountLogin, AccountLogin::WIRE_LEN, |frame| {
            AccountLogin {
                account_password: senha,
                account_name: nome,
                zero: [0u8; 52],
                client_version: 12_000,
                db_need_save: 1,
                adapter_name: [0; 4],
            }
            .encode_into(frame)
            .expect("corpo do AccountLogin");
        })?;
        self.roster = Some(self.expect_body(Opcode::CNFAccountLogin)?);

        let mut token = [0u8; 6];
        for (indice, digito) in config.pin.bytes().take(6).enumerate() {
            token[indice] = digito;
        }
        self.send(Opcode::AccountSecure, AccountSecure::WIRE_LEN, |frame| {
            AccountSecure {
                numeric_token: token,
                unknown: [0u8; 10],
                change_numeric: 1,
            }
            .encode_into(frame)
            .expect("corpo do AccountSecure");
        })?;
        // O canal confirma o PIN ecoando `_MSG_AccountSecure` (0x0FDE), não um `CNF`.
        self.expect(Opcode::AccountSecure)?;
        Ok(())
    }

    fn entrar(&mut self, config: &BotConfig) -> std::io::Result<()> {
        let slot = config.slot;
        self.send(Opcode::CharacterLogin, CharacterLogin::WIRE_LEN, |frame| {
            CharacterLogin { slot, force: 1 }
                .encode_into(frame)
                .expect("corpo do CharacterLogin");
        })?;
        self.login = Some(self.expect_body(Opcode::CNFCharacterLogin)?);
        Ok(())
    }

    fn handshake(&mut self, config: &BotConfig) -> std::io::Result<()> {
        self.autenticar(config)?;
        self.entrar(config)
    }

    /// Caminho de personagem **novo**: autentica, cria o char no slot e entra com ele.
    ///
    /// É o `_MSG_CreateCharacter` real (36 B) + `_MSG_CharacterLogin`, sem atalho de banco:
    /// o teste vivo prova o mesmo caminho que o cliente 7662 faz na tela de criação.
    pub fn connect_novo(
        config: &BotConfig,
        race: Race,
        nome: &str,
        class: i32,
    ) -> std::io::Result<Self> {
        let endereco = config
            .addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error::new(ErrorKind::AddrNotAvailable, "endereco do canal"))?;
        let stream = TcpStream::connect_timeout(&endereco, Duration::from_secs(5))?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        let mut bot = Self {
            stream,
            key_word: 11,
            race,
            roster: None,
            login: None,
            raw: Vec::new(),
        };
        bot.autenticar(config)?;
        bot.create_character(config.slot, nome, class)?;
        bot.entrar(config)?;
        Ok(bot)
    }

    /// `_MSG_CreateCharacter` (0x0203) e a confirmação `CNFNewCharacter`.
    pub fn create_character(&mut self, slot: i32, nome: &str, class: i32) -> std::io::Result<()> {
        let mut mob_name = [0u8; 16];
        let bytes = nome.as_bytes();
        if bytes.len() > mob_name.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "nome acima de 16 bytes",
            ));
        }
        mob_name[..bytes.len()].copy_from_slice(bytes);
        self.send(
            Opcode::CreateCharacter,
            CreateCharacter::WIRE_LEN,
            |frame| {
                CreateCharacter {
                    slot,
                    mob_name,
                    mob_class: class,
                }
                .encode_into(frame)
                .expect("corpo do CreateCharacter");
            },
        )?;
        self.roster = Some(self.expect_body(Opcode::CNFNewCharacter)?);
        Ok(())
    }

    /// Corpo cru do último roster recebido (`_MSG_CNFAccountLogin` ou
    /// `_MSG_CNFNewCharacter`): o teste decodifica o `Selchar` daqui, não de uma conta
    /// paralela — é o byte que o cliente recebeu.
    #[must_use]
    pub fn roster_body(&self) -> Option<&[u8]> {
        self.roster.as_deref()
    }

    /// Corpo cru do último `_MSG_CNFCharacterLogin` (`STRUCT_MOB` começa no offset 4).
    #[must_use]
    pub fn login_body(&self) -> Option<&[u8]> {
        self.login.as_deref()
    }

    /// Lê a cena por até `millis` e devolve as criaturas dos `_MSG_CreateMob` recebidos.
    ///
    /// O resto dos pacotes (score, items, painéis) é descartado: o bot só precisa do id,
    /// do nome e do tile para escolher alvo e portão, tudo dado pelo servidor.
    pub fn read_scene(&mut self, millis: u64) -> std::io::Result<Vec<SceneMob>> {
        let anterior = self
            .stream
            .read_timeout()?
            .unwrap_or(Duration::from_secs(5));
        self.stream
            .set_read_timeout(Some(Duration::from_millis(millis)))?;
        let mut vistos = Vec::new();
        let resultado = loop {
            match self.recv() {
                Ok((header, frame)) => {
                    if Opcode::from_wire(header.message_type) == Some(Opcode::CreateMob) {
                        match CreateMob::decode(&frame) {
                            Ok(mob) => vistos.push(SceneMob {
                                id: mob.mob_id,
                                name: nome_do_campo(&mob.mob_name),
                                x: i32::from(mob.pos_x),
                                y: i32::from(mob.pos_y),
                                level: u16::try_from(mob.level).unwrap_or(0),
                                hp: u32::try_from(mob.hp).unwrap_or(0),
                                max_hp: u32::try_from(mob.max_hp).unwrap_or(0),
                                merchant: mob.merchant,
                            }),
                            Err(erro) => {
                                break Err(Error::new(ErrorKind::InvalidData, erro.to_string()))
                            }
                        }
                    }
                }
                Err(erro)
                    if erro.kind() == ErrorKind::WouldBlock
                        || erro.kind() == ErrorKind::TimedOut =>
                {
                    break Ok(vistos)
                }
                Err(erro) => break Err(erro),
            }
        };
        self.stream.set_read_timeout(Some(anterior))?;
        resultado
    }

    /// Espera o próximo `_MSG_MessagePanel` da conexão.
    fn painel(&mut self) -> std::io::Result<String> {
        let limite = std::time::Instant::now() + Duration::from_secs(5);
        while std::time::Instant::now() < limite {
            let (header, frame) = self.recv()?;
            if Opcode::from_wire(header.message_type) == Some(Opcode::MessagePanel) {
                let painel = MessagePanel::decode(&frame)
                    .map_err(|erro| Error::new(ErrorKind::InvalidData, erro.to_string()))?;
                return Ok(String::from_utf8_lossy(painel.text_bytes()).to_string());
            }
        }
        Err(Error::new(ErrorKind::TimedOut, "sem MessagePanel"))
    }

    fn send(
        &mut self,
        opcode: Opcode,
        len: usize,
        corpo: impl FnOnce(&mut [u8]),
    ) -> std::io::Result<()> {
        let mut frame = vec![0u8; len];
        let size = u16::try_from(len).map_err(|_| Error::new(ErrorKind::InvalidInput, "frame"))?;
        Header {
            size,
            key_word: self.key_word,
            checksum: 0,
            message_type: opcode.wire(),
            id: 1,
            client_tick: 7,
        }
        .encode(&mut frame)
        .map_err(|erro| Error::new(ErrorKind::InvalidData, erro.to_string()))?;
        corpo(&mut frame);
        encode_in_place(&mut frame, self.key_word)
            .map_err(|erro| Error::new(ErrorKind::InvalidData, erro.to_string()))?;
        self.stream.write_all(&frame)
    }

    fn recv(&mut self) -> std::io::Result<(Header, Vec<u8>)> {
        let mut head = [0u8; HEADER_SIZE];
        self.stream.read_exact(&mut head)?;
        let size = usize::from(u16::from_le_bytes([head[0], head[1]]));
        if !(HEADER_SIZE..=8192).contains(&size) {
            return Err(Error::new(ErrorKind::InvalidData, "Size fora de 12..8192"));
        }
        let mut frame = vec![0u8; size];
        frame[..HEADER_SIZE].copy_from_slice(&head);
        self.stream.read_exact(&mut frame[HEADER_SIZE..])?;
        decode_in_place(&mut frame)
            .map_err(|erro| Error::new(ErrorKind::InvalidData, erro.to_string()))?;
        let header = Header::decode(&frame)
            .map_err(|erro| Error::new(ErrorKind::InvalidData, erro.to_string()))?;
        if matches!(
            Opcode::from_wire(header.message_type),
            Some(
                Opcode::CNFAccountLogin
                    | Opcode::CNFNewCharacter
                    | Opcode::CNFCharacterLogin
                    | Opcode::CreateMob
                    | Opcode::UpdateScore
                    | Opcode::UpdateEtc
            )
        ) {
            self.raw.push(frame.clone());
        }
        Ok((header, frame))
    }

    /// Frames crus (header + corpo) dos pacotes que carregam nível, na ordem de chegada.
    #[must_use]
    pub fn raw_frames(&self) -> &[Vec<u8>] {
        &self.raw
    }

    fn expect(&mut self, opcode: Opcode) -> std::io::Result<()> {
        self.expect_body(opcode).map(|_| ())
    }

    /// Igual a [`Bot::expect`], mas devolve o corpo (sem o header) do frame recebido.
    fn expect_body(&mut self, opcode: Opcode) -> std::io::Result<Vec<u8>> {
        let (header, frame) = self.recv()?;
        // Trailer opcional de motivo: só existe se o servidor mandou o byte extra.
        let motivo = wyd_protocol::motivo::CodigoRecusa::from_byte(frame.get(HEADER_SIZE).copied());
        match Opcode::from_wire(header.message_type) {
            Some(visto) if visto == opcode => Ok(frame[HEADER_SIZE..].to_vec()),
            Some(visto) => Err(Error::new(
                ErrorKind::PermissionDenied,
                format!(
                    "recusa:{}:{}:{}",
                    visto.wire(),
                    motivo.byte(),
                    motivo_da_recusa(visto)
                ),
            )),
            None => Err(Error::new(
                ErrorKind::InvalidData,
                format!("esperado {opcode:?}, recebido um opcode fora do catálogo"),
            )),
        }
    }
}

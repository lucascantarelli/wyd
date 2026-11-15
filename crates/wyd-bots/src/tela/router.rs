//! **Modelo derivado — não é prova do cliente.** Roteador de recebimento do 7662 reconstruído a
//! partir do desassemble de `client/WYD.exe`; ele serve para discutir/regredir a forma de fio, mas
//! a prova do comportamento do cliente é o assembly citado em
//! `docs/rust/evidencias/login-tela-handler-2026-11-14.md`, não este código.
//!
//! Este módulo existe para responder, dentro do repositório, *quais opcodes a tela do cliente
//! realmente processa em cada etapa* — e portanto qual resposta de falha ela consegue desenhar.
//! Cada regra cita o endereço virtual da comparação (`objdump -d -M intel client/WYD.exe`,
//! despejo em `tmp/wyd.asm`).
//!
//! # Duas etapas, dois roteadores
//!
//! - **Etapa de login** (função em `0x4b3e83`, antes de haver roster):
//!   - `0x4b3ec1` `cmp ecx,0x101` → só `call [obj+0x60](1)`: **visibilidade**, não desenha texto;
//!   - `0x4b3f45` `cmp edx,0x10a` → copia `0x348` B de `frame+0x20` (roster) e avança de etapa;
//!   - `0x4b4071` `cmp ecx,0x11d` / `0x4b4082` `cmp eax,0x11c` → `push 5000; push 0x109aab8;
//!     call 0x404109` + ação `[obj+0x88](1,1)`: **mensagem nativa** do cliente;
//!   - qualquer outro opcode: `xor eax,eax` (`0x4b4126`) → **ignorado**.
//! - **Etapa de seleção/mundo** (função que trata `0x4aea52`):
//!   - `0x4aea5b` `cmp ecx,0x11b` → `push 2000; push 0x109aeb8; call 0x404109` → mensagem nativa;
//!   - `0x4aed63` `cmp ecx,0x119` → `push 2000; push 0x109af38; call 0x404109` → mensagem nativa;
//!   - `0x4aeab5` `cmp ecx,0x114` → `0x400` B de `frame+0x10` para o `STRUCT_MOB`.
//!
//! Consequência medida: **`0x0119` e o texto do `0x0101` não têm handler na etapa de login**. É
//! por isso que a recusa de credencial não aparece na tela, enquanto a recusa de exclusão
//! (`0x011b`, etapa de seleção) aparece.

use wyd_protocol::wire::header::{Header, HEADER_SIZE};
use wyd_protocol::wire::opcode::Opcode;

/// Etapa em que a tela do cliente está.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Estagio {
    /// Antes do `0x010A`: login, senha numérica e recusas nativas.
    Login,
    /// Depois do `0x010A`: roster carregado, seleção e mundo.
    Selecao,
}

/// Efeito de um frame na tela do cliente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Acao {
    /// Opcode fora do conjunto daquela etapa (`xor eax,eax`): nada muda.
    Ignorado,
    /// `0x0101` na etapa de login: só alterna visibilidade, **não** desenha o texto.
    Visibilidade,
    /// `0x0101` na etapa de seleção: painel com o texto de `frame+12`.
    PainelTexto(String),
    /// `_MSG_CNFAccountLogin`: roster copiado e etapa passa a seleção.
    Roster,
    /// `*Fail` desenhado pelo próprio cliente, com o id de mensagem do binário.
    MensagemNativa(&'static str),
}

/// Id de mensagem usado por `0x011c`/`0x011d` na etapa de login.
pub const MSG_JA_JOGANDO: &str = "0x109aab8";
/// Id de mensagem da recusa de exclusão (o caminho que o jogador confirma visível).
pub const MSG_RECUSA_EXCLUSAO: &str = "0x109aeb8";
/// Id de mensagem da recusa de login de personagem: desenhado **só** na etapa de seleção.
pub const MSG_RECUSA_LOGIN_CHAR: &str = "0x109af38";

/// Estado da tela + roteamento por etapa.
#[derive(Debug)]
pub struct Router7662 {
    /// Painel de texto visível (só a etapa de seleção alimenta isto).
    pub painel: Option<String>,
    /// Última mensagem nativa desenhada pelo cliente.
    pub mensagem: Option<&'static str>,
    /// Quantos bytes de roster o cliente copiou.
    pub roster: usize,
    estagio: Estagio,
}

impl Default for Router7662 {
    fn default() -> Self {
        Self {
            painel: None,
            mensagem: None,
            roster: 0,
            estagio: Estagio::Login,
        }
    }
}

impl Router7662 {
    /// Tela recém-conectada: etapa de login.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Etapa corrente da tela.
    #[must_use]
    pub fn estagio(&self) -> Estagio {
        self.estagio
    }

    /// Processa um frame s→c já desofuscado, como o roteador do binário faria.
    pub fn ao_receber(&mut self, frame: &[u8]) -> Acao {
        let Ok(header) = Header::decode(frame) else {
            return Acao::Ignorado;
        };
        let Some(opcode) = Opcode::from_wire(header.message_type) else {
            return Acao::Ignorado;
        };
        match (self.estagio, opcode) {
            // 0x4b3ec1: `0x0101` na etapa de login só mexe em visibilidade.
            (Estagio::Login, Opcode::MessagePanel) => Acao::Visibilidade,
            // 0x4b3f45: roster e troca de etapa.
            (Estagio::Login, Opcode::CNFAccountLogin) => {
                self.roster = frame.len().saturating_sub(HEADER_SIZE);
                self.estagio = Estagio::Selecao;
                Acao::Roster
            }
            // 0x4b4071 / 0x4b4082: únicas mensagens que a tela de login desenha.
            (Estagio::Login, Opcode::AlreadyPlaying | Opcode::StillPlaying) => {
                self.mensagem = Some(MSG_JA_JOGANDO);
                Acao::MensagemNativa(MSG_JA_JOGANDO)
            }
            // 0x4aea5b: recusa de exclusão — o caminho visível.
            (Estagio::Selecao, Opcode::DeleteCharacterFail) => {
                self.mensagem = Some(MSG_RECUSA_EXCLUSAO);
                Acao::MensagemNativa(MSG_RECUSA_EXCLUSAO)
            }
            // 0x4aed63: recusa de login de personagem.
            (Estagio::Selecao, Opcode::CharacterLoginFail) => {
                self.mensagem = Some(MSG_RECUSA_LOGIN_CHAR);
                Acao::MensagemNativa(MSG_RECUSA_LOGIN_CHAR)
            }
            // Etapa de seleção desenha o painel de texto livre (`frame+12`).
            (Estagio::Selecao, Opcode::MessagePanel) => {
                let corpo = &frame[HEADER_SIZE..];
                let fim = corpo
                    .iter()
                    .position(|byte| *byte == 0)
                    .unwrap_or(corpo.len());
                let texto = String::from_utf8_lossy(&corpo[..fim]).to_string();
                self.painel = Some(texto.clone());
                Acao::PainelTexto(texto)
            }
            _ => Acao::Ignorado,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Frames s→c reais, capturados no canal vivo (`127.0.0.1:8281`, imagem de
    /// 2026-11-14T23:01:10Z) pelo teste `sistema_login_canal`; despejo em
    /// `tmp/login-canal/recusas.txt` (senha errada / conta bloqueada / conta inexistente).
    const RECUSA_0X0119: &str = "0c000bac1901010000000000";
    const RECUSA_0X011B: &str = "0c000bac1b01010000000000";
    const PAINEL_SENHA: &str = "8c000b5a010130750000000053656e6861206f7520636f6e746120696e636f72726574610000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    fn bytes(hex: &str) -> Vec<u8> {
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("hex"))
            .collect()
    }

    /// `_MSG_CNFAccountLogin` do jeito que o servidor manda (2008 B), montado aqui.
    fn roster_0x010a() -> Vec<u8> {
        let mut frame = vec![0u8; 2008];
        frame[0..2].copy_from_slice(&(2008u16).to_le_bytes());
        frame[4..6].copy_from_slice(&Opcode::CNFAccountLogin.wire().to_le_bytes());
        frame
    }

    /// O ponto do defeito: na tela de login, `0x0119` e o texto do `0x0101` não produzem nada.
    #[test]
    fn tela_de_login_ignora_0x0119_e_o_texto_do_painel() {
        let mut tela = Router7662::new();
        assert_eq!(tela.estagio(), Estagio::Login);
        assert_eq!(tela.ao_receber(&bytes(RECUSA_0X0119)), Acao::Ignorado);
        assert_eq!(tela.ao_receber(&bytes(PAINEL_SENHA)), Acao::Visibilidade);
        assert!(tela.painel.is_none(), "login nao desenha texto de painel");
        assert!(tela.mensagem.is_none(), "login nao desenha 0x0119");
        assert_eq!(tela.roster, 0, "nada de roster na recusa");
    }

    /// Na etapa de seleção — a do caminho que o jogador vê — os mesmos frames desenham.
    #[test]
    fn tela_de_selecao_desenha_recusa_e_painel() {
        let mut tela = Router7662::new();
        assert_eq!(tela.ao_receber(&roster_0x010a()), Acao::Roster);
        assert_eq!(tela.estagio(), Estagio::Selecao);
        assert_eq!(
            tela.ao_receber(&bytes(RECUSA_0X011B)),
            Acao::MensagemNativa(MSG_RECUSA_EXCLUSAO),
            "recusa de exclusao: o caminho que o usuario confirma visivel"
        );
        assert_eq!(
            tela.ao_receber(&bytes(RECUSA_0X0119)),
            Acao::MensagemNativa(MSG_RECUSA_LOGIN_CHAR)
        );
        assert_eq!(
            tela.ao_receber(&bytes(PAINEL_SENHA)),
            Acao::PainelTexto("Senha ou conta incorreta".to_string())
        );
        assert_eq!(tela.painel.as_deref(), Some("Senha ou conta incorreta"));
    }

    /// A forma que o servidor passou a emitir na recusa (CNF vazio, `0x0119`, `0x0101`) faz o
    /// cliente **desenhar** o painel, porque o `0x010A` move a tela para a etapa de seleção.
    #[test]
    fn sequencia_de_recusa_do_servidor_desenha_o_painel_no_cliente() {
        // `0x010A` com roster todo zero = nenhum `STRUCT_SELCHAR` (nenhum personagem).
        let mut tela = Router7662::new();
        assert_eq!(tela.ao_receber(&roster_0x010a()), Acao::Roster);
        assert_eq!(tela.estagio(), Estagio::Selecao);
        assert_eq!(
            tela.ao_receber(&bytes(RECUSA_0X0119)),
            Acao::MensagemNativa(MSG_RECUSA_LOGIN_CHAR)
        );
        assert_eq!(
            tela.ao_receber(&bytes(PAINEL_SENHA)),
            Acao::PainelTexto("Senha ou conta incorreta".to_string())
        );
        assert_eq!(
            tela.painel.as_deref(),
            Some("Senha ou conta incorreta"),
            "painel visivel na tela de selecao aberta pela recusa"
        );
    }

    /// A única mensagem que a tela de login sabe desenhar vem de `0x011c`/`0x011d`.
    #[test]
    fn tela_de_login_desenha_mensagem_nativa_de_0x011c() {
        let mut frame = vec![0u8; HEADER_SIZE];
        frame[0] = HEADER_SIZE as u8;
        frame[4..6].copy_from_slice(&Opcode::AlreadyPlaying.wire().to_le_bytes());
        let mut tela = Router7662::new();
        assert_eq!(
            tela.ao_receber(&frame),
            Acao::MensagemNativa(MSG_JA_JOGANDO)
        );
    }
}

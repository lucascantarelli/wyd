//! Submódulo de `bot` (ADR-0027).

use super::*;

/// Tamanho do frame de ataque que o cliente 7662 manda de fato: o corpo do `MSG_Attack` de
/// 13 alvos (164 B) mais 4 B de trailer, sob o tipo `_MSG_AttackOne`.
pub const CLIENT_ATTACK_LEN: usize = Attack::<13>::WIRE_LEN + 4;

impl Bot {
    /// Envia `_MSG_AttackOne` no tile do alvo (o servidor resolve o alvo pela posição).
    pub fn attack_at(&mut self, x: i32, y: i32) -> std::io::Result<()> {
        let coord = |valor: i32| u16::try_from(valor).unwrap_or(0);
        let (target_x, target_y) = (coord(x), coord(y));
        self.send(Opcode::AttackOne, Attack::<1>::WIRE_LEN, |frame| {
            Attack::<1> {
                unk_1: [0; 4],
                current_hp: 0,
                unk_2: [0; 4],
                current_exp: 0,
                unk_0: 0,
                pos_x: target_x,
                pos_y: target_y,
                target_x,
                target_y,
                attacker_id: 0,
                progress: 0,
                motion: 1,
                skill_parm: 0,
                double_critical: 0,
                flag_local: 0,
                rsv: 0,
                current_mp: 0,
                skill_index: 0,
                req_mp: 0,
                dam: [TargetDamage {
                    target_id: 0,
                    damage: 0,
                }],
            }
            .encode_into(frame)
            .expect("corpo do AttackOne");
        })
    }

    /// `_MSG_Attack` (0x0367) com 13 alvos — o pacote cheio do cliente 7662 (164 B), com o
    /// alvo no primeiro `STRUCT_DAM`. É o formato que o cliente usa no golpe básico; o
    /// `AttackOne` acima só existe para as respostas de skill.
    pub fn attack_full_at(&mut self, x: i32, y: i32, target_id: i32) -> std::io::Result<()> {
        let coord = |valor: i32| u16::try_from(valor).unwrap_or(0);
        let (target_x, target_y) = (coord(x), coord(y));
        let mut dam = [TargetDamage::default(); 13];
        dam[0] = TargetDamage {
            target_id,
            damage: 0,
        };
        self.send(Opcode::Attack, Attack::<13>::WIRE_LEN, |frame| {
            Attack::<13> {
                unk_1: [0; 4],
                current_hp: 0,
                unk_2: [0; 4],
                current_exp: 0,
                unk_0: 0,
                pos_x: target_x,
                pos_y: target_y,
                target_x,
                target_y,
                attacker_id: 0,
                progress: 0,
                motion: 1,
                skill_parm: 0,
                double_critical: 0,
                flag_local: 0,
                rsv: 0,
                current_mp: 0,
                skill_index: 0,
                req_mp: 0,
                dam,
            }
            .encode_into(frame)
            .expect("corpo do Attack");
        })
    }

    /// Frame **do cliente 7662**: tipo `_MSG_AttackOne` (`0x039D`) com 168 B — o corpo do
    /// `MSG_Attack` de 13 alvos (164 B) mais 4 B de trailer —, com o alvo em `Dam[0]`.
    ///
    /// É o tamanho medido no canal real (`event=attack result=refused reason=layout:
    /// tamanho de pacote inválido: esperado 68, recebido 168`); antes o `check_len` exigia os
    /// 68 B do `MSG_AttackOne` e recusava todo golpe do jogador.
    pub fn attack_client_at(&mut self, x: i32, y: i32, target_id: i32) -> std::io::Result<()> {
        let coord = |valor: i32| u16::try_from(valor).unwrap_or(0);
        let (target_x, target_y) = (coord(x), coord(y));
        let mut dam = [TargetDamage::default(); 13];
        dam[0] = TargetDamage {
            target_id,
            damage: 0,
        };
        let mut corpo = vec![0u8; Attack::<13>::WIRE_LEN];
        Attack::<13> {
            unk_1: [0; 4],
            current_hp: 0,
            unk_2: [0; 4],
            current_exp: 0,
            unk_0: 0,
            pos_x: target_x,
            pos_y: target_y,
            target_x,
            target_y,
            attacker_id: 0,
            progress: 0,
            motion: 1,
            skill_parm: 0,
            double_critical: 0,
            flag_local: 0,
            rsv: 0,
            current_mp: 0,
            skill_index: 0,
            req_mp: 0,
            dam,
        }
        .encode_into(&mut corpo)
        .expect("corpo do Attack");
        self.send(Opcode::AttackOne, CLIENT_ATTACK_LEN, |frame| {
            frame[HEADER_SIZE..Attack::<13>::WIRE_LEN]
                .copy_from_slice(&corpo[HEADER_SIZE..Attack::<13>::WIRE_LEN]);
        })
    }

    /// Envia **um** golpe só com o tile (sem `Dam[0].TargetID`) e devolve o `MobID` que o
    /// servidor usou no eco `_MSG_AttackOne` — é assim que o cliente descobre o id de uma
    /// criatura que ainda não veio em `_MSG_CreateMob`.
    pub fn attack_echo_target(
        &mut self,
        x: i32,
        y: i32,
        millis: u64,
    ) -> std::io::Result<Option<u16>> {
        self.attack_client_at(x, y, 0)?;
        let anterior = self
            .stream
            .read_timeout()?
            .unwrap_or(Duration::from_secs(5));
        self.stream
            .set_read_timeout(Some(Duration::from_millis(millis)))?;
        let mut alvo = None;
        let limite = std::time::Instant::now() + Duration::from_millis(millis);
        while std::time::Instant::now() < limite && alvo.is_none() {
            match self.recv() {
                Ok((header, frame)) => {
                    if Opcode::from_wire(header.message_type) == Some(Opcode::AttackOne) {
                        if let Ok(eco) = Attack::<1>::decode(&frame) {
                            let id = eco.dam[0].target_id;
                            if id > 0 {
                                alvo = u16::try_from(id).ok();
                            }
                        }
                    }
                }
                Err(erro)
                    if erro.kind() == ErrorKind::WouldBlock
                        || erro.kind() == ErrorKind::TimedOut =>
                {
                    break
                }
                Err(_erro) => break,
            }
        }
        self.stream.set_read_timeout(Some(anterior))?;
        Ok(alvo)
    }

    /// Igual a [`Bot::kill_full_at`], mas com o frame do cliente (0x039D, 168 B) e no ritmo
    /// do canal: um golpe por ciclo e o resto do ciclo **lendo** o socket (o cliente também
    /// não para de ler). Sem drenar, o servidor fica preso nas escritas de cena e o próximo
    /// golpe só é aceito segundos depois.
    pub fn kill_client_at(&mut self, id: u16, x: i32, y: i32) -> std::io::Result<()> {
        let limite = std::time::Instant::now() + Duration::from_secs(25);
        while std::time::Instant::now() < limite {
            self.attack_client_at(x, y, i32::from(id))?;
            let janela = std::time::Instant::now() + Duration::from_millis(820);
            while std::time::Instant::now() < janela {
                match self.recv() {
                    Ok((header, _)) => {
                        if Opcode::from_wire(header.message_type) == Some(Opcode::RemoveMob)
                            && header.id == id
                        {
                            return Ok(());
                        }
                    }
                    Err(erro)
                        if erro.kind() == ErrorKind::WouldBlock
                            || erro.kind() == ErrorKind::TimedOut =>
                    {
                        break
                    }
                    Err(erro) => return Err(erro),
                }
            }
        }
        Err(Error::new(
            ErrorKind::TimedOut,
            format!("{}: alvo {id} em ({x},{y}) não morreu", self.race.nome()),
        ))
    }

    /// Igual a [`Bot::kill_at`], mas com o `_MSG_Attack` cheio (`0x0367`).
    pub fn kill_full_at(&mut self, id: u16, x: i32, y: i32) -> std::io::Result<()> {
        let limite = std::time::Instant::now() + Duration::from_secs(20);
        while std::time::Instant::now() < limite {
            self.attack_full_at(x, y, i32::from(id))?;
            let fim = std::time::Instant::now() + Duration::from_millis(600);
            while std::time::Instant::now() < fim {
                match self.recv() {
                    Ok((header, _)) => {
                        if Opcode::from_wire(header.message_type) == Some(Opcode::RemoveMob)
                            && header.id == id
                        {
                            return Ok(());
                        }
                    }
                    Err(erro)
                        if erro.kind() == ErrorKind::WouldBlock
                            || erro.kind() == ErrorKind::TimedOut =>
                    {
                        break
                    }
                    Err(erro) => return Err(erro),
                }
            }
        }
        Err(Error::new(
            ErrorKind::TimedOut,
            format!("{}: alvo {id} em ({x},{y}) não morreu", self.race.nome()),
        ))
    }

    /// Mata a criatura `nome` vista na cena, atacando até o `_MSG_RemoveMob` dela.
    ///
    /// Só serve para um alvo que ainda vai aparecer: o `_MSG_CreateMob` é consumido na leitura
    /// da cena, então quem já foi lido deve usar [`Bot::kill_at`] com id/posição.
    pub fn kill(&mut self, nome: &str) -> std::io::Result<(i32, i32)> {
        let limite = std::time::Instant::now() + Duration::from_secs(10);
        let mut alvo = None;
        while alvo.is_none() && std::time::Instant::now() < limite {
            alvo = self
                .read_scene(700)?
                .into_iter()
                .find(|mob| mob.name == nome);
        }
        let alvo = alvo.ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                format!("{}: {nome} ausente da cena", self.race.nome()),
            )
        })?;
        self.kill_at(alvo.id, alvo.x, alvo.y)?;
        Ok((alvo.x, alvo.y))
    }

    /// Ataca o alvo já conhecido (`MobID`/tile vistos na cena) até o `_MSG_RemoveMob` dele.
    pub fn kill_at(&mut self, id: u16, x: i32, y: i32) -> std::io::Result<()> {
        let limite = std::time::Instant::now() + Duration::from_secs(20);
        while std::time::Instant::now() < limite {
            self.attack_at(x, y)?;
            let fim = std::time::Instant::now() + Duration::from_millis(600);
            while std::time::Instant::now() < fim {
                match self.recv() {
                    Ok((header, _)) => {
                        if Opcode::from_wire(header.message_type) == Some(Opcode::RemoveMob)
                            && header.id == id
                        {
                            return Ok(());
                        }
                    }
                    Err(erro)
                        if erro.kind() == ErrorKind::WouldBlock
                            || erro.kind() == ErrorKind::TimedOut =>
                    {
                        break
                    }
                    Err(erro) => return Err(erro),
                }
            }
        }
        Err(Error::new(
            ErrorKind::TimedOut,
            format!("{}: alvo {id} em ({x},{y}) não morreu", self.race.nome()),
        ))
    }
}

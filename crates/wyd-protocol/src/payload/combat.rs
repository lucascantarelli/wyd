//! Combate (`_MSG_Attack`/`_MSG_AttackOne`/`_MSG_AttackTwo` — mesmo corpo, N alvos).
//!
//! `N` = 13 (`MSG_Attack`, `MAX_TARGET`), 1 (`AttackOne`) ou 2 (`AttackTwo`);
//! o tamanho do pacote varia com o número de alvos (`protocol-spec` §3.5).

use super::{PayloadError, WireField, WirePayload};
use crate::wire::opcode::Opcode;

/// Um alvo de dano de `MSG_Attack` (`STRUCT_DAM`, 8 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TargetDamage {
    /// Índice da conexão do alvo.
    pub target_id: i32,
    /// Dano aplicado.
    pub damage: i32,
}

impl TargetDamage {
    /// Tamanho em bytes (`STRUCT_DAM`).
    pub const WIRE_LEN: usize = 8;
}

/// Ataque genérico sobre `N` alvos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attack<const N: usize> {
    pub unk_1: [u8; 4],
    pub current_hp: i32,
    pub unk_2: [u8; 4],
    pub current_exp: i64,
    pub unk_0: i16,
    pub pos_x: u16,
    pub pos_y: u16,
    pub target_x: u16,
    pub target_y: u16,
    pub attacker_id: u16,
    pub progress: u16,
    pub motion: u8,
    pub skill_parm: u8,
    pub double_critical: u8,
    pub flag_local: u8,
    pub rsv: i16,
    pub current_mp: i32,
    pub skill_index: i16,
    pub req_mp: i16,
    /// `Dam[N]` — `STRUCT_DAM` por alvo.
    pub dam: [TargetDamage; N],
}

/// Opcode de `MSG_Attack` para a aridade `N` (13 alvos = pacote cheio).
const fn attack_opcode<const N: usize>() -> Opcode {
    match N {
        1 => Opcode::AttackOne,
        2 => Opcode::AttackTwo,
        _ => Opcode::Attack,
    }
}

impl<const N: usize> WirePayload for Attack<N> {
    const OPCODE: Opcode = attack_opcode::<N>();
    const WIRE_LEN: usize = ATTACK_BODY_LEN + TargetDamage::WIRE_LEN * N;

    fn decode(packet: &[u8]) -> Result<Self, PayloadError> {
        // Tolerante a frame **maior**: o cliente 7662 manda `_MSG_AttackOne` (0x039D) com
        // 168 B — o corpo do `MSG_Attack` de 13 alvos (164 B) mais 4 B de trailer —, e o
        // legado lê os `Dam[]` pelos mesmos offsets. Frame menor que o layout continua erro.
        if packet.len() < Self::WIRE_LEN {
            return Err(PayloadError::BadLength {
                expected: Self::WIRE_LEN,
                got: packet.len(),
            });
        }
        let mut dam = [TargetDamage::default(); N];
        for (index, slot) in dam.iter_mut().enumerate() {
            let off = ATTACK_BODY_LEN + index * TargetDamage::WIRE_LEN;
            *slot = TargetDamage {
                target_id: i32::read(packet, off),
                damage: i32::read(packet, off + 4),
            };
        }
        Ok(Self {
            unk_1: <[u8; 4]>::read(packet, 12),
            current_hp: i32::read(packet, 16),
            unk_2: <[u8; 4]>::read(packet, 20),
            current_exp: i64::read(packet, 24),
            unk_0: i16::read(packet, 32),
            pos_x: u16::read(packet, 34),
            pos_y: u16::read(packet, 36),
            target_x: u16::read(packet, 38),
            target_y: u16::read(packet, 40),
            attacker_id: u16::read(packet, 42),
            progress: u16::read(packet, 44),
            motion: u8::read(packet, 46),
            skill_parm: u8::read(packet, 47),
            double_critical: u8::read(packet, 48),
            flag_local: u8::read(packet, 49),
            rsv: i16::read(packet, 50),
            current_mp: i32::read(packet, 52),
            skill_index: i16::read(packet, 56),
            req_mp: i16::read(packet, 58),
            dam,
        })
    }

    fn encode_into(&self, packet: &mut [u8]) -> Result<(), PayloadError> {
        super::check_len(Self::WIRE_LEN, packet.len())?;
        <[u8; 4]>::write(self.unk_1, packet, 12);
        i32::write(self.current_hp, packet, 16);
        <[u8; 4]>::write(self.unk_2, packet, 20);
        i64::write(self.current_exp, packet, 24);
        i16::write(self.unk_0, packet, 32);
        u16::write(self.pos_x, packet, 34);
        u16::write(self.pos_y, packet, 36);
        u16::write(self.target_x, packet, 38);
        u16::write(self.target_y, packet, 40);
        u16::write(self.attacker_id, packet, 42);
        u16::write(self.progress, packet, 44);
        u8::write(self.motion, packet, 46);
        u8::write(self.skill_parm, packet, 47);
        u8::write(self.double_critical, packet, 48);
        u8::write(self.flag_local, packet, 49);
        i16::write(self.rsv, packet, 50);
        i32::write(self.current_mp, packet, 52);
        i16::write(self.skill_index, packet, 56);
        i16::write(self.req_mp, packet, 58);
        for (index, target) in self.dam.iter().enumerate() {
            let off = ATTACK_BODY_LEN + index * TargetDamage::WIRE_LEN;
            i32::write(target.target_id, packet, off);
            i32::write(target.damage, packet, off + 4);
        }
        Ok(())
    }
}

/// Corpo fixo de `MSG_Attack` antes de `Dam[]` (offset do primeiro `STRUCT_DAM`).
pub const ATTACK_BODY_LEN: usize = 60;

/// `_MSG_Attack` com 13 alvos (`MAX_TARGET`).
pub type AttackFull = Attack<13>;
/// `_MSG_AttackOne` com 1 alvo.
pub type AttackOne = Attack<1>;
/// `_MSG_AttackTwo` com 2 alvos.
pub type AttackTwo = Attack<2>;

#[cfg(test)]
mod tests {
    use super::super::round_trip;
    use super::*;

    macro_rules! attack {
        ($ty:ident, $dam:expr) => {
            $ty {
                unk_1: [1, 2, 3, 4],
                current_hp: 500,
                unk_2: [5, 6, 7, 8],
                current_exp: 9_000_000,
                unk_0: -1,
                pos_x: 100,
                pos_y: 200,
                target_x: 110,
                target_y: 210,
                attacker_id: 7,
                progress: 3,
                motion: 1,
                skill_parm: 2,
                double_critical: 1,
                flag_local: 0,
                rsv: 0,
                current_mp: 300,
                skill_index: 42,
                req_mp: 10,
                dam: $dam,
            }
        };
    }

    #[test]
    fn attack_variants_round_trip_with_their_target_counts() {
        let full = attack!(
            AttackFull,
            std::array::from_fn(|index| TargetDamage {
                target_id: index as i32 + 1,
                damage: 100 + index as i32,
            })
        );
        assert_eq!(AttackFull::WIRE_LEN, 164);
        assert_eq!(AttackFull::OPCODE.wire(), 871);
        let decoded = round_trip(full);
        assert_eq!(decoded, full);
        assert_eq!(decoded.dam[12].target_id, 13);

        let one = attack!(
            AttackOne,
            [TargetDamage {
                target_id: 9,
                damage: 77,
            }]
        );
        assert_eq!(AttackOne::WIRE_LEN, 68);
        assert_eq!(AttackOne::OPCODE.wire(), 925);
        let decoded = round_trip(one);
        assert_eq!(decoded.dam[0].target_id, 9);
        assert_eq!(decoded.dam[0].damage, 77);

        let two = attack!(
            AttackTwo,
            [
                TargetDamage {
                    target_id: 9,
                    damage: 77,
                },
                TargetDamage {
                    target_id: 10,
                    damage: 88,
                },
            ]
        );
        assert_eq!(AttackTwo::WIRE_LEN, 76);
        let decoded = round_trip(two);
        assert_eq!(decoded.dam[1].damage, 88);
    }

    /// Trava os offsets do legado (`Basedef.h:2399-2432`) e o frame **real** do cliente
    /// 7662: `0x039D` com 168 B (o `MSG_Attack` de 13 alvos + 4 B de trailer). Se alguém
    /// mexer na ordem dos campos, o alvo (`TargetX@38`, `Dam[0].TargetID@60`) muda e o
    /// ataque passa a acertar lugar nenhum — este teste falha antes.
    #[test]
    fn frame_do_cliente_7662_le_alvo_e_dano_nos_offsets_do_legado() {
        assert_eq!(ATTACK_BODY_LEN, 60, "Dam[] começa no 60 do frame");
        assert_eq!(AttackFull::WIRE_LEN, 164);

        let mut frame = vec![0u8; AttackFull::WIRE_LEN];
        // Campos nos offsets do `MSG_Attack` do legado, escritos à mão.
        frame[38..40].copy_from_slice(&2110_u16.to_le_bytes());
        frame[40..42].copy_from_slice(&2046_u16.to_le_bytes());
        frame[42..44].copy_from_slice(&7_u16.to_le_bytes());
        frame[46] = 1;
        frame[60..64].copy_from_slice(&3654_i32.to_le_bytes());
        frame[64..68].copy_from_slice(&0_i32.to_le_bytes());
        frame.extend_from_slice(&[0u8; 4]); // trailer do cliente

        let decoded = AttackFull::decode(&frame).expect("168 B decodificam");
        assert_eq!(decoded.target_x, 2110, "TargetX no offset 38");
        assert_eq!(decoded.target_y, 2046, "TargetY no offset 40");
        assert_eq!(decoded.attacker_id, 7);
        assert_eq!(decoded.motion, 1);
        assert_eq!(
            decoded.dam[0].target_id, 3654,
            "Dam[0].TargetID no offset 60"
        );
        assert_eq!(decoded.dam[0].damage, 0, "Dam[0].Damage no offset 64");

        // Frame menor que o layout continua recusado (nada de ler além do buffer).
        let curto = &frame[..AttackFull::WIRE_LEN - 1];
        assert!(matches!(
            AttackFull::decode(curto),
            Err(PayloadError::BadLength {
                expected: 164,
                got: 163
            })
        ));
    }
}

// Parser independente do `_MSG_Attack` (0x0367) e do eco `_MSG_AttackOne` (0x039D).
//
// Lê o dump do teste `sistema_combate_pacote` (`tmp/combate-pacote/*.txt`, linhas
// `c2s|s2c size=N hex=...`) e extrai os campos nos offsets do legado
// (`Source/Code/Basedef.h:2399-2432`, `MSG_Attack` com alinhamento natural do MSVC x86):
//
//   16 CurrentHp (int)      24 CurrentExp (int64)   34 PosX   36 PosY
//   38 TargetX   40 TargetY   42 AttackerID   44 Progress
//   46 Motion   47 SkillParm   48 DoubleCritical   49 FlagLocal   50 Rsv (short)
//   52 CurrentMp (int)   56 SkillIndex   58 ReqMp   60 Dam[0] { int TargetID; int Damage; }
//
// Nada aqui usa o crate Rust: tamanho, opcode e offsets são literais, para que uma
// divergência do porte apareça como erro deste script.
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

const DIR = process.argv[2] ?? 'tmp/combate-pacote';
const OFFSETS = {
  current_hp: 16,
  current_exp: 24,
  pos_x: 34,
  pos_y: 36,
  target_x: 38,
  target_y: 40,
  attacker_id: 42,
  progress: 44,
  motion: 46,
  skill_parm: 47,
  double_critical: 48,
  flag_local: 49,
  rsv: 50,
  current_mp: 52,
  skill_index: 56,
  req_mp: 58,
  dam: 60,
};
const DAM_LEN = 8;
// Tamanhos aceitos por tipo. O cliente 7662 manda `0x039D` com **168 B** (o corpo do
// `MSG_Attack` de 13 alvos + 4 B de trailer), medido em `event=attack result=refused
// reason=layout:tamanho de pacote inválido: esperado 68, recebido 168`.
const TAMANHOS = { 0x0367: [164], 0x039d: [68, 168], 0x039e: [76] };

const u16 = (b, o) => b.readUInt16LE(o);
const i16 = (b, o) => b.readInt16LE(o);
const i32 = (b, o) => b.readInt32LE(o);
const i64 = (b, o) => Number(b.readBigInt64LE(o));

let falhas = 0;
const arquivos = readdirSync(DIR).filter((nome) => nome.endsWith('.txt')).sort();
if (arquivos.length === 0) {
  console.error(`sem dumps em ${DIR} (rode o teste sistema_combate_pacote)`);
  process.exit(1);
}

for (const arquivo of arquivos) {
  const linhas = readFileSync(join(DIR, arquivo), 'utf8').split('\n');
  for (const linha of linhas) {
    const m = /^(c2s|s2c) size=(\d+) hex=([0-9a-f]+)$/.exec(linha.trim());
    if (!m) continue;
    const [, direcao, tamanhoTexto, hex] = m;
    const bytes = Buffer.from(hex, 'hex');
    const type = u16(bytes, 4);
    if (!(type in TAMANHOS)) continue;
    const esperados = TAMANHOS[type];
    if (!esperados.includes(bytes.length)) {
      falhas += 1;
      console.log(
        `FALHA ${arquivo} ${direcao} type=0x${type.toString(16)} size=${bytes.length} fora de [${esperados}]`,
      );
      continue;
    }
    const nome = type === 0x0367 ? 'Attack' : type === 0x039d ? 'AttackOne' : 'AttackTwo';
    const alvos = Math.floor((bytes.length - OFFSETS.dam) / DAM_LEN);
    const dam = [];
    for (let i = 0; i < alvos; i += 1) {
      const off = OFFSETS.dam + i * DAM_LEN;
      dam.push({ target_id: i32(bytes, off), damage: i32(bytes, off + 4) });
    }
    const preenchidos = dam.filter((d) => d.target_id !== 0 || d.damage !== 0);
    console.log(
      [
        `${arquivo} ${direcao} ${nome} size=${bytes.length}`,
        `CurrentHp@16=${i32(bytes, OFFSETS.current_hp)}`,
        `CurrentExp@24=${i64(bytes, OFFSETS.current_exp)}`,
        `Pos@34/36=(${u16(bytes, OFFSETS.pos_x)},${u16(bytes, OFFSETS.pos_y)})`,
        `Target@38/40=(${u16(bytes, OFFSETS.target_x)},${u16(bytes, OFFSETS.target_y)})`,
        `AttackerID@42=${u16(bytes, OFFSETS.attacker_id)}`,
        `Progress@44=${u16(bytes, OFFSETS.progress)}`,
        `Motion@46=${bytes[OFFSETS.motion]}`,
        `SkillParm@47=${bytes[OFFSETS.skill_parm]}`,
        `DoubleCritical@48=${bytes[OFFSETS.double_critical]}`,
        `FlagLocal@49=${bytes[OFFSETS.flag_local]}`,
        `Rsv@50=${i16(bytes, OFFSETS.rsv)}`,
        `CurrentMp@52=${i32(bytes, OFFSETS.current_mp)}`,
        `SkillIndex@56=${i16(bytes, OFFSETS.skill_index)}`,
        `ReqMp@58=${i16(bytes, OFFSETS.req_mp)}`,
        `Dam@60=[${preenchidos.map((d) => `${d.target_id}/${d.damage}`).join(', ')}]`,
      ].join(' '),
    );
  }
}

if (falhas > 0) {
  console.error(`${falhas} frame(s) fora do layout do legado`);
  process.exit(1);
}
console.log('OK: todo frame de ataque bate com o layout do legado');

#!/usr/bin/env node
// Parser **independente** do codec do porte: lê o dump hexadecimal dos frames capturados
// (`tmp/nivel-wire/*.txt`) e extrai o nível no offset calculado das structs do legado.
//
// Fontes do layout (Source/Code, commit f39aed3):
//   STRUCT_SCORE   Basedef.h:524   -- int Level @0, 48 B
//   STRUCT_MOB     Basedef.h:556   -- BaseScore @44, CurrentScore @92, 816 B
//   STRUCT_SELCHAR Basedef.h:1002  -- Score[4] @80, 48 B por slot
//   MSG_DBCNFAccountLogin Basedef.h:1413 -- HashKeyTable[16] + int -> sel @20
//   MSG_CNFNewCharacter   Basedef.h:1639 -- sel após o _MSG; o cliente 7662 copia 840 B a
//                                           partir de frame+0x10 (docs/cliente/pacotes.md)
//   MSG_CNFCharacterLogin Basedef.h:1683 -- short PosX, PosY -> mob @4
//   MSG_CreateMob         Basedef.h:1918 -- Score @136 no frame (= 124 no corpo)
//   _MSG_UpdateScore      protocolo.md §5.2 -- STRUCT_SCORE @0 no corpo
//
// Nada aqui importa crate nem struct do servidor: só a tabela acima e os bytes.

const SCORE_LEN = 48;
const SCORE_LEVEL = 0;
const MOB_BASE_SCORE = 44;
const MOB_CURRENT_SCORE = 92;
const SELCHAR_SCORE = 80;
const SEL_AT_ACCOUNT = 20;
const SEL_AT_NEW = 4;
const MOB_AT_LOGIN = 4;
const CREATE_MOB_SCORE = 124;
const HEADER = 12;

const OPCODES = {
  0x010a: "CNFAccountLogin",
  0x0110: "CNFNewCharacter",
  0x0114: "CNFCharacterLogin",
  0x0364: "CreateMob",
  0x0336: "UpdateScore",
};

function lerInt32(bytes, offset) {
  if (offset + 4 > bytes.length) {
    throw new Error(`offset ${offset} fora do frame (${bytes.length} B)`);
  }
  return bytes.readInt32LE(offset);
}

function hex(bytes, offset, antes = 4, depois = 8) {
  const inicio = Math.max(0, offset - antes);
  const fim = Math.min(bytes.length, offset + depois);
  return bytes.subarray(inicio, fim).toString("hex").replace(/(..)/g, "$1 ").trim();
}

/** Offset do `Level` dentro do corpo, por opcode, ou null quando não há nível. */
function offsetDoNivel(tipo, slot) {
  switch (tipo) {
    case 0x010a:
      return SEL_AT_ACCOUNT + SELCHAR_SCORE + slot * SCORE_LEN + SCORE_LEVEL;
    case 0x0110:
      return SEL_AT_NEW + SELCHAR_SCORE + slot * SCORE_LEN + SCORE_LEVEL;
    case 0x0114:
      return MOB_AT_LOGIN + MOB_CURRENT_SCORE + SCORE_LEVEL;
    case 0x0364:
      return CREATE_MOB_SCORE + SCORE_LEVEL;
    case 0x0336:
      return SCORE_LEVEL;
    default:
      return null;
  }
}

/** Nível em `CurrentScore`/`BaseScore` do `STRUCT_MOB` do `CNFCharacterLogin`. */
function offsetDoBaseScore() {
  return MOB_AT_LOGIN + MOB_BASE_SCORE + SCORE_LEVEL;
}

function nomeDoMob(corpo) {
  const bruto = corpo.subarray(6, 22);
  const fim = bruto.indexOf(0);
  return bruto
    .subarray(0, fim === -1 ? 16 : fim)
    .toString("latin1")
    .trim();
}

function principal() {
  const args = process.argv.slice(2);
  const arquivo = args[0];
  const slot = Number(
    args.find((arg) => arg.startsWith("--slot="))?.slice("--slot=".length) ?? "0",
  );
  if (!arquivo) {
    console.error("uso: node tools/client/nivel-do-legado.mjs <dump.txt> [--slot=N]");
    process.exit(2);
  }
  const linhas = require("node:fs")
    .readFileSync(arquivo, "utf8")
    .split(/\r?\n/)
    .filter(Boolean);

  console.log(`arquivo=${arquivo} slot=${slot} frames=${linhas.length}`);
  for (const linha of linhas) {
    const campos = Object.fromEntries(
      linha.split(" ").map((par) => {
        const [chave, ...resto] = par.split("=");
        return [chave, resto.join("=")];
      }),
    );
    const tipo = Number(campos.type);
    const bytes = Buffer.from(campos.hex ?? "", "hex");
    const nome = OPCODES[tipo] ?? `0x${tipo.toString(16)}`;
    const offset = offsetDoNivel(tipo, slot);
    if (offset === null) {
      console.log(`  ${nome}: sem campo de nível`);
      continue;
    }
    const envelope = offset < HEADER ? "corpo" : "frame";
    const nivel = lerInt32(bytes, HEADER + offset);
    const extra =
      tipo === 0x0364
        ? ` mob=${nomeDoMob(bytes.subarray(HEADER))}`
        : "";
    console.log(
      `  ${nome}: Level@${envelope}+${offset} (frame+${HEADER + offset}) ` +
        `bytes[${hex(bytes, HEADER + offset)}] = ${nivel} -> desenhado ${nivel + 1}${extra}`,
    );
    if (tipo === 0x0114) {
      const base = lerInt32(bytes, HEADER + offsetDoBaseScore());
      console.log(
        `      BaseScore.Level@corpo+${offsetDoBaseScore()} = ${base} -> desenhado ${base + 1}`,
      );
    }
  }
}

principal();

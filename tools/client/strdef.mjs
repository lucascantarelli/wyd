#!/usr/bin/env node
/**
 * Resolve os ids de mensagem do cliente 7662 a partir de `client/UI/strdef.bin`.
 *
 * Como o binário lê a tabela (desassemble em `tmp/wyd.asm`):
 * - base `0x109a4b8`, registros de **128 B**, `2000` registros (`0x3e800` B) — função
 *   `0x53a837` (`fopen("UI/strdef.bin","rb")`, `fread(0x109a4b8, 1, 0x3e800)`, checksum
 *   `0x53a55b`; a contraparte de escrita é `0x53a758`);
 * - o braço `0x4b4071`/`0x4b4082` (etapa de login) faz `push 0x109aab8`, isso é
 *   `base + 12 * 128` = registro **12**;
 * - texto em **cp1252** (não UTF-8), NUL-padded.
 *
 * Uso: node tools/client/strdef.mjs            # ids conhecidos
 *      node tools/client/strdef.mjs 0x109aab8  # um VA qualquer
 *      node tools/client/strdef.mjs --index 12
 */
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

export const BASE = 0x109a4b8;
export const RECORD = 128;
const ARQUIVO = "client/UI/strdef.bin";

/** Ids de mensagem citados no desassemble e em que opcode aparecem. */
export const IDS = {
  "0x011b DeleteCharacterFail": 0x109aeb8,
  "0x0119 CharacterLoginFail": 0x109af38,
  "0x011c AlreadyPlaying / 0x011d StillPlaying": 0x109aab8,
};

const lerTabela = () => readFileSync(ARQUIVO);

/** Devolve `{ index, offset, texto, hex }` para um VA da tabela. */
export function registroPorVa(va, tabela = lerTabela()) {
  const deslocamento = va - BASE;
  if (deslocamento < 0 || deslocamento % RECORD !== 0) {
    throw new Error(`VA 0x${va.toString(16)} não é múltiplo de ${RECORD} a partir da base`);
  }
  return registroPorIndice(deslocamento / RECORD, tabela);
}

/** Devolve `{ index, offset, texto, hex }` para um índice da tabela. */
export function registroPorIndice(index, tabela = lerTabela()) {
  const offset = index * RECORD;
  const bruto = tabela.subarray(offset, offset + RECORD);
  if (bruto.length === 0) throw new Error(`índice ${index} fora da tabela`);
  const fim = bruto.indexOf(0);
  const texto = bruto.subarray(0, fim === -1 ? RECORD : fim).toString("latin1");
  return { index, offset, texto, hex: bruto.subarray(0, 24).toString("hex") };
}

const principal = () => {
  const [arg] = process.argv.slice(2);
  if (arg === "--index") {
    const { index, offset, texto } = registroPorIndice(Number(process.argv[3]));
    console.log(`índice=${index} file_off=0x${offset.toString(16)} cp1252=${JSON.stringify(texto)}`);
    return;
  }
  if (arg) {
    const { index, offset, texto, hex } = registroPorVa(Number(arg));
    console.log(
      `VA=${arg} índice=${index} file_off=0x${offset.toString(16)} hex=${hex} cp1252=${JSON.stringify(texto)}`,
    );
    return;
  }
  for (const [nome, va] of Object.entries(IDS)) {
    const { index, offset, texto, hex } = registroPorVa(va);
    console.log(
      `${nome} VA=0x${va.toString(16)} índice=${index} file_off=0x${offset.toString(16)} ` +
        `hex=${hex} cp1252=${JSON.stringify(texto)}`,
    );
  }
};

const executadoDireto =
  process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href;
if (executadoDireto) principal();

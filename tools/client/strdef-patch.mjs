#!/usr/bin/env node
/**
 * Reescreve um registro de `client/UI/strdef.bin` (tabela de mensagens do 7662).
 *
 * Formato medido no carregador do binário (`0x53a837`, desassemble em `tmp/wyd.asm`):
 * - a tabela tem `0x3e800` (256.000) bytes = 2000 registros de **128** bytes, base `0x109a4b8`;
 * - o arquivo em disco é lido com `fread(buf, 1, 0x3e800, f)` e **depois** com
 *   `fread(&valor, 1, 4, f)` — os 4 bytes do fim seriam o checksum esperado;
 * - `computed = checksum(buf, 0x3e800)` (`0x53a55b`, a rotina replicada em `checksum()` abaixo);
 * - se `computed == valor`, o buffer é decodificado com `xor 0x5a` (`0x53a8f5`); se **não**,
 *   a função devolve 0 **sem decodificar** — e quem a chama (`0x5494ea`) **ignora o retorno**:
 *   a próxima instrução é `push 0x622cd4` / `call 0x438310`. Ou seja, a tabela fica como foi lida.
 * - o gravador `0x53a758` (re-codifica e anexa o checksum) **não tem chamador** no binário.
 *
 * O `client/UI/strdef.bin` deste repositório tem 255.992 B (8 bytes menos que `0x3e800`) e
 * **não** tem os 4 bytes de checksum no fim: o segundo `fread` devolve 0, `valor` fica 0 e
 * `computed` (medido: 10.705.530) nunca fecha — logo o `xor 0x5a` nunca é aplicado e o texto é
 * lido em claro. É por isso que reescrever um registro em cp1252 aparece na tela **sem**
 * atualizar checksum nenhum: o caminho de decodificação já está desligado neste arquivo.
 *
 * Uso:
 *   node tools/client/strdef-patch.mjs --index 12 --texto "Usuário ou senha incorretos."
 *   ... --dry-run                 # mede e mostra, não grava
 *   ... --backup tmp/strdef.bin.orig-2026-11-15
 *   ... --json                    # saída só em JSON
 *
 * Recusa (exit 1) se o texto não couber no registro (127 B + NUL) ou sair do cp1252 — nunca
 * trunca em silêncio.
 */
import { copyFileSync, readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

export const BASE = 0x109a4b8;
export const RECORD = 128;
export const TABELA = 0x3e800;
const ARQUIVO = "client/UI/strdef.bin";

/** Classe de recusa do patch (nunca trunca em silêncio). */
export class PatchError extends Error {}

/**
 * Checksum do carregador (`0x53a55b`): acumula com uma operação diferente por `índice % 7`,
 * sobre **byte com sinal** (`movsx`), em 32 bits com sinal.
 */
export function checksum(bytes, len = bytes.length) {
  let acc = 0;
  for (let i = 0; i < len; i++) {
    const v = bytes[i] < 128 ? bytes[i] : bytes[i] - 256;
    const m = i % 7;
    if (m === 0) acc = (acc + Math.trunc(v / 2)) | 0;
    else if (m === 1) acc = (acc + (v ^ 0xff)) | 0;
    else if (m === 2) acc = (acc + v * 3) | 0;
    else if (m === 3) acc = (acc + v * 2) | 0;
    else if (m === 4) acc = (acc - Math.trunc(v / 7)) | 0;
    else if (m === 5) acc = (acc - v) | 0;
    else acc = (acc + Math.trunc(v / 3)) | 0;
  }
  return acc;
}

/** Texto → bytes cp1252 (o cliente lê latin1/cp1252; UTF-8 sairia mojibake). */
export function paraCp1252(texto) {
  const bytes = Buffer.alloc(texto.length);
  for (let i = 0; i < texto.length; i++) {
    const cp = texto.charCodeAt(i);
    if (cp > 0xff) {
      throw new PatchError(
        `caractere ${JSON.stringify(texto[i])} (U+${cp.toString(16).toUpperCase()}) não existe em cp1252`,
      );
    }
    bytes[i] = cp;
  }
  return bytes;
}

/** Aplica o texto no registro `index` de uma cópia de `tabela`; devolve `{ antes, depois }`. */
export function gravarRegistro(tabela, index, texto) {
  const offset = index * RECORD;
  if (offset + RECORD > tabela.length) {
    throw new PatchError(`registro ${index} fora da tabela (offset 0x${offset.toString(16)})`);
  }
  const bytes = paraCp1252(texto);
  if (bytes.length > RECORD - 1) {
    throw new PatchError(
      `${bytes.length} B não cabem no slot de ${RECORD} B (máximo ${RECORD - 1} + NUL)`,
    );
  }
  const antes = Buffer.from(tabela.subarray(offset, offset + RECORD));
  const depois = Buffer.alloc(RECORD);
  bytes.copy(depois, 0);
  bytes.copy(tabela, offset);
  tabela.fill(0, offset + bytes.length, offset + RECORD);
  return { offset, antes, depois: tabela.subarray(offset, offset + RECORD), bytes: bytes.length };
}

const argumento = (nome) => {
  const i = process.argv.indexOf(nome);
  return i === -1 ? undefined : process.argv[i + 1];
};

const principal = () => {
  const index = Number(argumento("--index"));
  const texto = argumento("--texto");
  if (!Number.isInteger(index) || index < 0 || texto === undefined) {
    console.error(
      'uso: node tools/client/strdef-patch.mjs --index N --texto "..." [--dry-run] [--backup CAMINHO] [--json]',
    );
    return 1;
  }
  const original = readFileSync(ARQUIVO);
  const tabela = Buffer.from(original);
  let resultado;
  try {
    resultado = gravarRegistro(tabela, index, texto);
  } catch (error) {
    if (error instanceof PatchError) {
      console.error(`recusado: ${error.message}`);
      return 1;
    }
    throw error;
  }

  const backup = argumento("--backup");
  if (backup) copyFileSync(ARQUIVO, backup);

  const seco = process.argv.includes("--dry-run");
  if (!seco) writeFileSync(ARQUIVO, tabela);

  const alterados = [];
  for (let i = 0; i < Math.max(original.length, tabela.length); i++) {
    if (original[i] !== tabela[i]) alterados.push(i);
  }
  // Primeiro/último byte alterado, quando existe algum: a lista vazia acontece no `--dry-run`
  // repetido (o registro já está com o texto pedido) e não pode derrubar a impressão.
  const primeiro = alterados.length > 0 ? `0x${alterados[0].toString(16)}` : "-";
  const ultimo = alterados.length > 0 ? `0x${alterados.at(-1).toString(16)}` : "-";
  const dadosChecksum = Buffer.alloc(TABELA);
  tabela.copy(dadosChecksum, 0, 0, Math.min(tabela.length, TABELA));
  const antesChecksum = Buffer.alloc(TABELA);
  original.copy(antesChecksum, 0, 0, Math.min(original.length, TABELA));

  const saida = {
    arquivo: ARQUIVO,
    modo: seco ? "dry-run" : "gravado",
    backup: backup ?? null,
    tamanho: { antes: original.length, depois: tabela.length },
    registro: {
      index,
      offset: `0x${resultado.offset.toString(16)}`,
      bytes_usados: resultado.bytes,
      limite_bytes: RECORD - 1,
      hex_antes: resultado.antes.subarray(0, resultado.bytes + 8).toString("hex"),
      hex_depois: resultado.depois.subarray(0, resultado.bytes + 8).toString("hex"),
      texto_antes: resultado.antes.subarray(0, resultado.antes.indexOf(0)).toString("latin1"),
      texto_depois: resultado.depois.subarray(0, resultado.depois.indexOf(0)).toString("latin1"),
    },
    bytes_alterados: { total: alterados.length, primeiro, ultimo },
    checksum: {
      // 4 bytes de checksum do fim do arquivo, quando existem
      declarado_no_arquivo:
        original.length > TABELA ? original.readInt32LE(TABELA) : "ausente (arquivo menor que 0x3e800)",
      calculado_antes: checksum(antesChecksum, TABELA),
      calculado_depois: checksum(dadosChecksum, TABELA),
      decodifica_xor: checksum(dadosChecksum, TABELA) === (original.length > TABELA ? original.readInt32LE(TABELA) : 0),
    },
  };

  if (process.argv.includes("--json")) {
    console.log(JSON.stringify(saida));
  } else {
    console.log(`arquivo=${saida.arquivo} modo=${saida.modo} backup=${saida.backup ?? "-"}`);
    console.log(`registro=${index} offset=${saida.registro.offset} bytes=${resultado.bytes}/${RECORD - 1}`);
    console.log(`antes : ${JSON.stringify(saida.registro.texto_antes)}`);
    console.log(`depois: ${JSON.stringify(saida.registro.texto_depois)}`);
    console.log(`hex_antes : ${saida.registro.hex_antes}`);
    console.log(`hex_depois: ${saida.registro.hex_depois}`);
    console.log(
      `bytes_alterados=${saida.bytes_alterados.total} (${saida.bytes_alterados.primeiro}..${saida.bytes_alterados.ultimo})`,
    );
    console.log(
      `checksum declarado=${saida.checksum.declarado_no_arquivo} calculado_antes=${saida.checksum.calculado_antes} calculado_depois=${saida.checksum.calculado_depois} decodifica_xor=${saida.checksum.decodifica_xor}`,
    );
  }
  return 0;
};

const executadoDireto =
  process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href;
if (executadoDireto) process.exit(principal());

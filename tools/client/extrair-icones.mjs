#!/usr/bin/env node
// Extrai os ícones de item do cliente 7662 para o portal.
//
// ENTRADA (não versionada, `client/` é gitignored):
//   client/itemicon.bin            6.500 × int32 LE — item index → icon id (`-1` = sem ícone)
//   client/UI/itemicon01..10.wyt   atlas de ícones (350×350, 32 bpp BGRA)
//
// SAÍDA (versionada, servida por `public/`):
//   wyd-portal/public/game-assets/itens/atlas-01.png … atlas-10.png
//   wyd-portal/public/game-assets/itens/icones.json
//
// POR QUE `itemicon01..10` E NÃO `itemicon.wyt`:
//   `itemicon.wyt` (400×3550) parece uma grade de 35 px, mas as células são **idênticas** —
//   é uma textura de slot vazio que se repete, não um atlas. Os arquivos numerados são
//   350×350 = **10×10 células de 35 px**, todas distintas, e 10 arquivos dão **1.000 slots**
//   para os **941** ids usados. Conferido visualmente e por categoria: `itemicon01` traz
//   anéis/amuletos/poções, `itemicon02` traz armas de haste e arcos.
//
// MAPEAMENTO (confiança declarada no JSON):
//   icon id → arquivo = 1 + floor(id / 100), célula = id % 100, coluna = célula % 10,
//   linha = floor(célula / 10). A ordem é **fortemente indicada** (não provada por captura):
//   os `kind` do catálogo caem nas faixas que a arte mostra — `accessory` 45/46 e
//   `consumable` 36 em `0000s` (itemicon01), `weapon` 238 em `0100s` (itemicon02).
//
// Uso:
//   node tools/client/extrair-icones.mjs
//   node tools/client/extrair-icones.mjs --amostra 1,150,400 tmp/icones
import fs from 'node:fs';
import path from 'node:path';

import { parseWyt, toPng } from './wyt.mjs';

const RAIZ = path.resolve(import.meta.dirname, '..', '..');
const PASTA_UI = path.join(RAIZ, 'client/UI');
const TABELA = path.join(RAIZ, 'client/itemicon.bin');
const SAIDA = path.join(RAIZ, 'wyd-portal/public/game-assets/itens');

const LADO = 35; // célula
const COLUNAS = 10;
const LINHAS = 10;
const POR_ARQUIVO = COLUNAS * LINHAS;
const ARQUIVOS = 10;
const SLOTS = ARQUIVOS * POR_ARQUIVO;

function main(argv) {
  if (!fs.existsSync(TABELA)) {
    console.error('ausente: client/itemicon.bin (client/ não é versionado)');
    return 1;
  }
  const buf = fs.readFileSync(TABELA);
  if (buf.length % 4 !== 0) {
    console.error(`itemicon.bin: ${buf.length} B não é múltiplo de 4`);
    return 1;
  }
  const porItem = [];
  for (let i = 0; i < buf.length; i += 4) porItem.push(buf.readInt32LE(i));

  const ids = porItem.filter((v) => v >= 0);
  const maxId = Math.max(...ids);
  if (maxId >= SLOTS) {
    console.error(`ícone ${maxId} fora dos ${SLOTS} slots de ${ARQUIVOS} atlas`);
    return 1;
  }

  fs.mkdirSync(SAIDA, { recursive: true });
  const atlas = [];
  for (let n = 1; n <= ARQUIVOS; n += 1) {
    const nome = `itemicon${String(n).padStart(2, '0')}.wyt`;
    const origem = path.join(PASTA_UI, nome);
    if (!fs.existsSync(origem)) {
      console.error(`ausente: client/UI/${nome}`);
      return 1;
    }
    const wyt = parseWyt(fs.readFileSync(origem), `client/UI/${nome}`);
    if (wyt.largura !== LADO * COLUNAS || wyt.altura !== LADO * LINHAS) {
      console.error(`${nome}: ${wyt.largura}x${wyt.altura} (esperado ${LADO * COLUNAS}²)`);
      return 1;
    }
    const arquivo = `atlas-${String(n).padStart(2, '0')}.png`;
    fs.writeFileSync(path.join(SAIDA, arquivo), toPng(wyt));
    atlas.push({ arquivo: `/game-assets/itens/${arquivo}`, largura: wyt.largura, altura: wyt.altura });
  }

  const semIcone = porItem.filter((v) => v < 0).length;
  const saida = {
    versao: 1,
    fonte: { tabela: 'client/itemicon.bin', atlas: 'client/UI/itemicon01..10.wyt' },
    grade: { lado: LADO, colunas: COLUNAS, linhas: LINHAS, porArquivo: POR_ARQUIVO },
    atlas,
    confianca: {
      grade: 'confirmado',
      ordem: 'fortemente indicado',
      nota:
        'id → arquivo 1+floor(id/100), célula id%100. Conferir uma amostra contra o cliente ' +
        'antes de tratar como contrato (--amostra).',
    },
    resumo: { itens: porItem.length, comIcone: porItem.length - semIcone, semIcone, idsDistintos: new Set(ids).size, maxId },
    // Uma linha por item index (6.500): id do ícone ou -1.
    itens: porItem,
  };
  fs.writeFileSync(path.join(SAIDA, 'icones.json'), `${JSON.stringify(saida)}\n`);

  console.log(`atlas ${ARQUIVOS} × ${LADO * COLUNAS}² (${SLOTS} slots)`);
  console.log(
    `itens ${porItem.length} · com ícone ${porItem.length - semIcone} · sem ícone ${semIcone} · ids distintos ${new Set(ids).size} (máx ${maxId})`,
  );
  console.log(`saída: ${path.relative(RAIZ, SAIDA)}`);

  // Conferência visual de ids específicos.
  const iAmostra = argv.indexOf('--amostra');
  if (iAmostra >= 0) {
    const lista = (argv[iAmostra + 1] ?? '0,1,150,400').split(',').map(Number);
    const destino = argv[iAmostra + 2] ?? 'tmp/icones';
    fs.mkdirSync(destino, { recursive: true });
    for (const id of lista) {
      if (!Number.isInteger(id) || id < 0 || id >= SLOTS) {
        console.error(`id inválido: ${id}`);
        continue;
      }
      const n = 1 + Math.floor(id / POR_ARQUIVO);
      const celula = id % POR_ARQUIVO;
      const x = (celula % COLUNAS) * LADO;
      const y = Math.floor(celula / COLUNAS) * LADO;
      const wyt = parseWyt(
        fs.readFileSync(path.join(PASTA_UI, `itemicon${String(n).padStart(2, '0')}.wyt`)),
      );
      const recorte = Buffer.alloc(LADO * LADO * wyt.bpp);
      for (let ly = 0; ly < LADO; ly += 1) {
        wyt.pixels.copy(
          recorte,
          ly * LADO * wyt.bpp,
          ((y + ly) * wyt.largura + x) * wyt.bpp,
          ((y + ly) * wyt.largura + x + LADO) * wyt.bpp,
        );
      }
      const png = toPng({ largura: LADO, altura: LADO, bpp: wyt.bpp, pixels: recorte });
      fs.writeFileSync(path.join(destino, `icone-${String(id).padStart(3, '0')}.png`), png);
      console.log(`  id ${id} → atlas-${String(n).padStart(2, '0')} célula ${celula} (${x},${y})`);
    }
    console.log(`amostra em ${destino}`);
  }

  return 0;
}

process.exitCode = main(process.argv);

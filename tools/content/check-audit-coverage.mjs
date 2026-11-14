#!/usr/bin/env node
/**
 * Gate de cobertura da auditoria (AUD-19…AUD-22): todo item proposto em `docs/audit/` precisa
 * aparecer em alguma issue escrita de `roadmap-rust/issues/`, e toda entrega do roadmap da
 * auditoria (`docs/audit/17`, itens 0.1…4.5) precisa estar citada no índice ou numa issue.
 *
 * Uso: node tools/content/check-audit-coverage.mjs
 * Saída: uma linha por item sem dono; exit 1 se houver item órfão.
 */
import { readFileSync, readdirSync } from "node:fs";

const leitura = (p) => readFileSync(p, "utf8");
const problemas = [];

const issues = readdirSync("roadmap-rust/issues")
  .filter((f) => f.endsWith(".md") && !/^FASE-\d\d\.md$/.test(f))
  .map((f) => leitura(`roadmap-rust/issues/${f}`))
  .join("\n");
const indice = leitura("roadmap-rust/index.md");
const tudo = `${indice}\n${issues}`;

const audit18 = leitura("docs/audit/18-unknown-features.md");
const audit15 = leitura("docs/audit/15-bugs-and-divergences.md");
const audit17 = leitura("docs/audit/17-priorities-and-roadmap.md");

const unks = [...new Set(audit18.match(/`(UNK-\d{2})`/g) ?? [])].map((s) => s.replaceAll("`", ""));
const divs = [...new Set(audit15.match(/`(DIV-\d{2})`/g) ?? [])].map((s) => s.replaceAll("`", ""));
const ns = [...new Set((audit15.match(/^\| (N-\d+) \|/gm) ?? []).map((l) => l.split("|")[1].trim()))];
const entregas = [
  ...new Set((audit17.match(/^\| (\d\.\d) \|/gm) ?? []).map((l) => l.split("|")[1].trim())),
];

if (unks.length < 30) problemas.push(`audit/18: só ${unks.length} UNK lidas — parser quebrado?`);
if (divs.length < 10) problemas.push(`audit/15: só ${divs.length} DIV lidas — parser quebrado?`);
if (ns.length < 10) problemas.push(`audit/15: só ${ns.length} achados N-* lidos — parser quebrado?`);
if (entregas.length < 20) problemas.push(`audit/17: só ${entregas.length} entregas lidas — parser quebrado?`);

for (const id of [...unks, ...divs, ...ns]) {
  if (!tudo.includes(id)) problemas.push(`${id}: nenhuma issue de roadmap-rust/issues cita o item`);
}
// A tabela "Cobertura item a item do §17" é o contrato: item → issue, checada linha a linha.
const donos = new Map();
for (const linha of indice.split("\n")) {
  for (const m of linha.matchAll(/\| (\d\.\d) \| `([A-Z]+-\d+)`/g)) donos.set(m[1], m[2]);
}
const changelog = leitura("roadmap-rust/changelog.md");
for (const entrega of entregas) {
  const dono = donos.get(entrega);
  if (!dono) {
    problemas.push(`audit/17 ${entrega}: sem linha na tabela de cobertura do índice`);
    continue;
  }
  const arquivo = readdirSync("roadmap-rust/issues").some((f) => f.startsWith(`${dono}-`));
  if (!arquivo && !changelog.includes(dono)) {
    problemas.push(`audit/17 ${entrega}: issue ${dono} não existe nem está no changelog`);
  }
}

console.log(
  `audit-coverage: ${unks.length} UNK, ${divs.length} DIV, ${ns.length} N-*, ${entregas.length} entregas do §17`,
);
if (problemas.length) {
  for (const p of problemas) console.log(`  ${p}`);
  process.exit(1);
}
console.log("ok: todo item da auditoria tem issue rastreável");

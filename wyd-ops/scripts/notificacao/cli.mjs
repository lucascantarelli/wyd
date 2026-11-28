#!/usr/bin/env node
// Entrada do relatório de CI: junta coleta, modelo e render; escreve o resumo,
// sobe nada (o artefato é da action) e posta no Slack quando há webhook.
//
// Fora do Actions dá para rodar contra um run real, que é como a paridade com o
// aviso antigo foi provada:
//
//   node wyd-ops/scripts/notificacao/cli.mjs --run 46138711791 \
//     --repo lucascantarelli/wyd --dry-run
//
// `--dry-run` valida o payload e imprime o JSON em vez de postar.

import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseArgs } from "node:util";

import { coletar } from "./coleta.mjs";
import { montarModelo } from "./modelo.mjs";
import { linhaDoAviso, markdown } from "./texto.mjs";
import { payloadSlack, validarPayload } from "./slack.mjs";

const { values } = parseArgs({
  options: {
    run: { type: "string" },
    repo: { type: "string" },
    workflow: { type: "string" },
    saida: { type: "string" },
    "dry-run": { type: "boolean", default: false },
    "sem-slack": { type: "boolean", default: false },
    "sem-cobertura": { type: "boolean", default: false },
    json: { type: "boolean", default: false },
    ajuda: { type: "boolean", default: false },
  },
  allowPositionals: false,
});

if (values.ajuda) {
  console.log(
    [
      "uso: node cli.mjs [--run ID] [--repo dono/nome] [--workflow nome] [--saida arquivo]",
      "                   [--dry-run] [--sem-slack] [--sem-cobertura] [--json]",
      "",
      "Sem --run/--repo usa GITHUB_RUN_ID e GITHUB_REPOSITORY. Em CI, escreve o resumo em",
      "$RUNNER_TEMP/resumo.md, anexa em $GITHUB_STEP_SUMMARY e posta em $SLACK_WEBHOOK_URL.",
    ].join("\n"),
  );
  process.exit(0);
}

const repo = values.repo ?? process.env.GITHUB_REPOSITORY ?? "";
const runId = values.run ?? process.env.GITHUB_RUN_ID ?? "";
const workflow = values.workflow ?? process.env.WORKFLOW ?? "";

const { dados, avisos } = coletar({
  repo,
  runId,
  workflow,
  baixarCobertura: !values["sem-cobertura"],
});

for (const aviso of avisos) console.log(`::warning::${aviso}`);

const modelo = montarModelo(dados);
const resumo = markdown(modelo);

const destino = values.saida ?? join(process.env.RUNNER_TEMP ?? process.cwd(), "resumo.md");
mkdirSync(dirname(destino), { recursive: true });
writeFileSync(destino, resumo, "utf8");

if (process.env.GITHUB_STEP_SUMMARY) {
  appendFileSync(process.env.GITHUB_STEP_SUMMARY, resumo, "utf8");
}

console.log(resumo);
console.log(`resumo escrito em ${destino}`);

const payload = validarPayload(payloadSlack(modelo));
console.log(`aviso montado: ${linhaDoAviso(modelo)}`);

if (values.json) console.log(JSON.stringify(modelo, null, 2));

if (values["dry-run"]) {
  console.log(JSON.stringify(payload, null, 2));
  console.log("dry-run: payload validado e não enviado.");
  process.exit(0);
}

const webhook = process.env.SLACK_WEBHOOK_URL;
if (!webhook || values["sem-slack"]) {
  console.log("SLACK_WEBHOOK_URL ausente (ou --sem-slack): aviso no Slack ignorado.");
  process.exit(0);
}

const resposta = await fetch(webhook, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(payload),
});

if (!resposta.ok) {
  const corpo = (await resposta.text()).slice(0, 300);
  console.error(`::error::Slack respondeu ${resposta.status}: ${corpo}`);
  process.exit(1);
}

console.log("aviso enviado ao Slack");

#!/usr/bin/env node
// Metadados do pull request: label de área por caminho, label de tamanho pelo diff e
// milestone herdado da issue que o PR fecha. Idempotente — roda em opened,
// ready_for_review, reopened e synchronize, e só escreve o que falta.
//
// GitHub-hosted action não é usada aqui de propósito: a allowlist de actions do
// repositório aceita só docker/*, dtolnay/*, Swatinem/* e taiki-e/*.

const AGENTE = "sync-pr-metadata";

const AREAS = [
  { nome: "area/db", cor: "1f6feb", descricao: "crates/wyd-db, migrations e planejamento do wyd-db", prefixos: ["crates/wyd-db/", "planejamento-1.0.0/wyd-db/"] },
  { nome: "area/docs", cor: "0e8a16", descricao: "site MkDocs e planejamento publicado", prefixos: ["wyd-docs/", "planejamento-1.0.0/"] },
  { nome: "area/ci", cor: "5319e7", descricao: "workflows, actions e scripts de CI", prefixos: [".github/", "wyd-ops/scripts/ci/"] },
  { nome: "area/ops", cor: "fbca04", descricao: "imagens, Compose e operação", prefixos: ["wyd-ops/", "docker-compose"] },
  { nome: "area/portal", cor: "d93f0b", descricao: "portal Next.js", prefixos: ["wyd-portal/"] },
  { nome: "area/launcher", cor: "006b75", descricao: "launcher Tauri", prefixos: ["crates/wyd-launcher/", "planejamento-1.0.0/wyd-launcher/"] },
  { nome: "area/cliente", cor: "bfd4f2", descricao: "cliente e registro do cliente", prefixos: ["wyd-client/", "planejamento-1.0.0/wyd-client/"] },
];

const TAMANHOS = [
  { nome: "size/XS", limite: 20, cor: "3cbf00", descricao: "até 20 linhas" },
  { nome: "size/S", limite: 100, cor: "5d9801", descricao: "21 a 100 linhas" },
  { nome: "size/M", limite: 400, cor: "7f7203", descricao: "101 a 400 linhas" },
  { nome: "size/L", limite: 1000, cor: "a14c09", descricao: "401 a 1000 linhas" },
  { nome: "size/XL", limite: Infinity, cor: "b60205", descricao: "mais de 1000 linhas" },
];

function exigir(nome) {
  const valor = process.env[nome];
  if (!valor) {
    console.error(`${AGENTE}: variável ${nome} ausente`);
    process.exit(1);
  }
  return valor;
}

async function api(caminho, init = {}) {
  const resposta = await fetch(`https://api.github.com${caminho}`, {
    ...init,
    headers: {
      Accept: "application/vnd.github+json",
      Authorization: `Bearer ${exigir("GITHUB_TOKEN")}`,
      "X-GitHub-Api-Version": "2022-11-28",
      ...(init.headers ?? {}),
    },
  });
  if (!resposta.ok) {
    const corpo = await resposta.text();
    throw new Error(`${init.method ?? "GET"} ${caminho} → ${resposta.status}: ${corpo.slice(0, 200)}`);
  }
  return resposta.status === 204 ? null : resposta.json();
}

function areasDe(arquivos) {
  return AREAS.filter((area) => arquivos.some((arquivo) => area.prefixos.some((prefixo) => arquivo.startsWith(prefixo))));
}

function tamanhoDe(linhas) {
  return TAMANHOS.find((t) => linhas <= t.limite);
}

function issueFechada(corpo) {
  const achado = /(?:closes|fixes|resolve[sd]?)\s+#(\d+)/i.exec(corpo ?? "");
  return achado ? Number(achado[1]) : null;
}

async function garantirLabel(repo, { nome, cor, descricao }) {
  try {
    await api(`/repos/${repo}/labels`, {
      method: "POST",
      body: JSON.stringify({ name: nome, color: cor, description: descricao }),
    });
  } catch (erro) {
    if (!String(erro.message).includes("422")) throw erro;
  }
}

const repo = exigir("REPOSITORIO");
const numero = Number(exigir("PR_NUMERO"));

const pr = await api(`/repos/${repo}/pulls/${numero}`);
const arquivos = await api(`/repos/${repo}/pulls/${numero}/files?per_page=100`);
const caminhos = arquivos.map((a) => a.filename);
const linhas = arquivos.reduce((soma, a) => soma + a.additions + a.deletions, 0);

const desejadas = [...areasDe(caminhos).map((a) => a.nome), tamanhoDe(linhas).nome];
for (const area of areasDe(caminhos)) await garantirLabel(repo, area);
await garantirLabel(repo, tamanhoDe(linhas));

const atuais = new Set((pr.labels ?? []).map((l) => l.name));
const adicionar = desejadas.filter((nome) => !atuais.has(nome));
if (adicionar.length > 0) {
  await api(`/repos/${repo}/issues/${numero}/labels`, {
    method: "POST",
    body: JSON.stringify({ labels: adicionar }),
  });
}

// Milestone: o PR herda o do item que ele fecha. Não inventa marco quando a issue não
// tem nenhum, e não sobrescreve milestone já definido à mão.
let marco = null;
if (!pr.milestone) {
  const fechada = issueFechada(pr.body);
  if (fechada) {
    const issue = await api(`/repos/${repo}/issues/${fechada}`);
    if (issue.milestone) {
      marco = issue.milestone.number;
      await api(`/repos/${repo}/issues/${numero}`, {
        method: "PATCH",
        body: JSON.stringify({ milestone: marco }),
      });
    }
  }
}

console.log(
  `${AGENTE}: #${numero} ${caminhos.length} arquivo(s), ${linhas} linha(s) → ` +
    `${adicionar.length ? adicionar.join(", ") : "labels já presentes"}` +
    `${marco ? `, milestone ${marco}` : ""}`,
);

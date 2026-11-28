#!/usr/bin/env node
// Coleta do relatório de CI — a parte impura: fala com a API e lê o disco.
//
// Usa o `gh` já autenticado pelo job (`GH_TOKEN`), como as outras automações do
// repositório: nenhum token é lido, impresso ou passado como argumento. Toda
// leitura que falha vira aviso e o campo fica ausente — API sem permissão degrada
// o relatório, não derruba o run.
//
// O que sai daqui é dado cru normalizado; cálculo e texto ficam em `modelo.mjs`,
// `texto.mjs` e `slack.mjs`, que são puros e testados sem rede.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";

import { motivosPulados } from "./workflow.mjs";

/** Crates cujo job publica o artefato `cobertura-<crate>`. */
export const CRATES_COBERTURA = ["wyd-db", "wyd-launcher"];

function gh(args) {
  return execFileSync("gh", args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
}

/** `gh api` + JSON.parse, uma chamada por página. */
export function ghJson(args) {
  return JSON.parse(gh(args));
}

function tentar(rotulo, avisos, fn) {
  try {
    return fn();
  } catch (erro) {
    const detalhe = String(erro?.stderr ?? erro?.message ?? erro).split("\n").find((l) => l.trim()) ?? "falhou";
    avisos.push(`${rotulo}: ${detalhe.trim()}`);
    return null;
  }
}

/** Jobs do run, percorrendo as páginas enquanto o `total_count` não fechar. */
export function jobsDoRun(repo, runId) {
  const todos = [];
  for (let pagina = 1; pagina <= 10; pagina += 1) {
    const corpo = ghJson([
      "api",
      `repos/${repo}/actions/runs/${runId}/jobs?per_page=100&page=${pagina}`,
    ]);
    const jobs = corpo.jobs ?? [];
    todos.push(...jobs);
    if (!jobs.length || todos.length >= (corpo.total_count ?? 0)) break;
  }
  return todos;
}

/** Commit do run: autor, data e primeira linha da mensagem. */
export function commitDo(repo, sha) {
  const corpo = ghJson(["api", `repos/${repo}/commits/${sha}`]);
  return {
    sha: corpo.sha ?? sha,
    autor: corpo.author?.login ?? corpo.commit?.author?.name ?? null,
    data: corpo.commit?.committer?.date ?? corpo.commit?.author?.date ?? null,
    assunto: String(corpo.commit?.message ?? "").split("\n")[0],
    url: `https://github.com/${repo}/commit/${corpo.sha ?? sha}`,
  };
}

/** PR aberto cuja head é a branch do run — caminho usado fora do evento de PR. */
export function prDaBranch(repo, branch) {
  const dono = repo.split("/")[0];
  const lista = ghJson([
    "api",
    `repos/${repo}/pulls?head=${dono}:${encodeURIComponent(branch)}&state=all&per_page=1`,
  ]);
  const primeiro = Array.isArray(lista) ? lista[0] : null;
  if (!primeiro) return null;
  return {
    numero: primeiro.number,
    titulo: primeiro.title,
    url: primeiro.html_url,
    headSha: primeiro.head?.sha ?? null,
    autor: primeiro.user?.login ?? null,
    corpo: primeiro.body ?? "",
  };
}

/**
 * Issue fechada pelo PR. `closingIssuesReferences` é GraphQL e enxerga o vínculo
 * criado pela interface e pelo corpo; sem permissão, cai no `Closes #N` do corpo.
 */
export function issueFechadaPeloPR(repo, pr) {
  const [dono, nome] = repo.split("/");
  try {
    const resposta = ghJson([
      "api",
      "graphql",
      "-f",
      "query=query($dono:String!,$nome:String!,$numero:Int!){repository(owner:$dono,name:$nome){pullRequest(number:$numero){closingIssuesReferences(first:5){nodes{number title url state}}}}}",
      "-F",
      `dono=${dono}`,
      "-F",
      `nome=${nome}`,
      "-F",
      `numero=${pr.numero}`,
    ]);
    const no = resposta?.data?.repository?.pullRequest?.closingIssuesReferences?.nodes?.[0];
    if (no) {
      return { numero: no.number, titulo: no.title, url: no.url, estado: no.state, fonte: "closingIssuesReferences" };
    }
  } catch {
    // Sem GraphQL o vínculo ainda pode estar escrito no corpo do PR.
  }

  const achado = /(?:closes|fixes|resolves?)\s+#(\d+)/i.exec(pr.corpo ?? "");
  if (!achado) return null;
  const issue = ghJson(["api", `repos/${repo}/issues/${achado[1]}`]);
  return {
    numero: issue.number,
    titulo: issue.title,
    url: issue.html_url,
    estado: issue.state,
    fonte: "corpo do PR",
  };
}

/** Últimos runs concluídos do workflow na branch — base da taxa de falha e do MTTR. */
export function amostraDaBranch(repo, workflowId, branch, limite = 30) {
  const corpo = ghJson([
    "api",
    `repos/${repo}/actions/workflows/${workflowId}/runs?branch=${encodeURIComponent(branch)}&status=completed&per_page=${limite}`,
  ]);
  return (corpo.workflow_runs ?? []).map((run) => ({ conclusao: run.conclusion, fim: run.updated_at }));
}

/** Percentual de linha de cada crate, lido do `coverage-summary.txt` do artefato. */
export function coberturaDoRun(repo, runId, destino, crates = CRATES_COBERTURA) {
  const encontrados = [];
  for (const crate of crates) {
    const pasta = join(destino, `cob-${crate}`);
    rmSync(pasta, { recursive: true, force: true });
    mkdirSync(pasta, { recursive: true });
    try {
      gh(["run", "download", String(runId), "-n", `cobertura-${crate}`, "-D", pasta, "--repo", repo]);
    } catch {
      continue; // crate sem cobertura publicada: ausência, não 0%
    }
    const arquivo = join(pasta, "coverage-summary.txt");
    if (!existsSync(arquivo)) continue;
    const total = readFileSync(arquivo, "utf8")
      .split(/\r?\n/)
      .find((linha) => linha.startsWith("TOTAL"));
    if (!total) continue;
    const campos = total.trim().split(/\s+/);
    const pct = Number.parseFloat(String(campos[9] ?? "").replace("%", ""));
    if (Number.isFinite(pct)) encontrados.push({ crate, pct });
  }
  return encontrados;
}

/** Conteúdo do workflow do run, para explicar job pulado. */
export function lerWorkflowDoAmbiente(env = process.env) {
  const referencia = env.GITHUB_WORKFLOW_REF ?? "";
  const caminho = referencia.split("@")[0].split("/").slice(2).join("/");
  const base = env.GITHUB_WORKSPACE ?? process.cwd();
  const alvo = caminho ? join(base, caminho) : join(base, ".github/workflows/ci.yml");
  return existsSync(alvo) ? readFileSync(alvo, "utf8") : null;
}

/** Evento cru do GitHub (`GITHUB_EVENT_PATH`), quando existe. */
export function lerEvento(caminho = process.env.GITHUB_EVENT_PATH) {
  if (!caminho || !existsSync(caminho)) return null;
  try {
    return JSON.parse(readFileSync(caminho, "utf8"));
  } catch {
    return null;
  }
}

function normalizarRun(bruto, workflow) {
  return {
    id: bruto.id,
    numero: bruto.run_number ?? null,
    tentativa: bruto.run_attempt ?? 1,
    evento: bruto.event ?? "",
    ref: bruto.head_branch ?? "",
    sha: bruto.head_sha ?? null,
    url: bruto.html_url ?? null,
    criadoEm: bruto.created_at ?? null,
    atualizadoEm: bruto.updated_at ?? null,
    workflowId: bruto.workflow_id ?? null,
    workflowName: workflow,
  };
}

function normalizarJob(bruto) {
  return {
    nome: bruto.name,
    conclusao: bruto.conclusion ?? null,
    inicio: bruto.started_at ?? null,
    fim: bruto.completed_at ?? null,
    url: bruto.html_url ?? null,
  };
}

/**
 * Junta tudo que o relatório precisa.
 *
 * @returns {{dados: object, avisos: string[]}}
 */
export function coletar({ repo, runId, workflow, workflowYaml = null, evento = null, baixarCobertura = true, crates = CRATES_COBERTURA } = {}) {
  const avisos = [];
  if (!repo) throw new Error("repositório ausente (use --repo ou GITHUB_REPOSITORY)");
  if (!runId) throw new Error("run ausente (use --run ou GITHUB_RUN_ID)");

  const bruto = tentar("run", avisos, () => ghJson(["api", `repos/${repo}/actions/runs/${runId}`]));
  const run = bruto ? normalizarRun(bruto, workflow ?? (bruto.name ?? "").trim()) : { id: runId, workflowName: workflow ?? "" };

  const jobsBrutos = tentar("jobs do run", avisos, () => jobsDoRun(repo, runId)) ?? [];
  const jobs = jobsBrutos.map(normalizarJob);

  const yaml = workflowYaml ?? lerWorkflowDoAmbiente();
  const pulados = jobs.filter((job) => job.conclusao === "skipped").map((job) => job.nome);
  const motivos = motivosPulados(yaml, pulados);
  for (const job of jobs) {
    if (job.conclusao === "skipped") job.motivo = motivos.get(job.nome) ?? null;
  }

  // O payload do evento traz o PR completo (título, head, corpo); fora do evento
  // de PR — execução local com `--run` — o PR sai da branch do run.
  const doEvento = evento ?? lerEvento();
  const doPayload = doEvento?.pull_request ?? null;
  let pr = null;
  if (doPayload) {
    pr = {
      numero: doPayload.number,
      titulo: doPayload.title ?? null,
      url: doPayload.html_url ?? null,
      headSha: doPayload.head?.sha ?? null,
      autor: doPayload.user?.login ?? null,
      corpo: doPayload.body ?? "",
    };
  } else if (run.evento === "pull_request" && run.ref) {
    pr = tentar("PR da branch", avisos, () => prDaBranch(repo, run.ref));
  }

  const issue = pr ? tentar("issue do PR", avisos, () => issueFechadaPeloPR(repo, pr)) : null;

  const shaDoCommit = pr?.headSha ?? run.sha;
  const commit = shaDoCommit ? tentar("commit", avisos, () => commitDo(repo, shaDoCommit)) : null;

  const amostra = run.workflowId && run.ref
    ? tentar("amostra de runs", avisos, () => amostraDaBranch(repo, run.workflowId, run.ref)) ?? []
    : [];

  const destino = process.env.RUNNER_TEMP ?? process.cwd();
  const cobertura = baixarCobertura
    ? tentar("cobertura", avisos, () => coberturaDoRun(repo, runId, destino, crates)) ?? []
    : [];

  return {
    dados: {
      run,
      jobs,
      pr,
      issue,
      commit,
      amostra,
      cobertura,
      workflow: workflow ?? run.workflowName ?? "",
      // Ator: quem disparou o evento; sem payload (execução local) o run traz o autor.
      autor: doEvento?.sender?.login ?? bruto?.actor?.login ?? null,
    },
    avisos,
  };
}

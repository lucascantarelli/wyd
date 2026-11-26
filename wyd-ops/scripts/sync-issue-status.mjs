#!/usr/bin/env node
// Transição de estado de issue — fonte única da sequência.
//
// Labels são a fonte de verdade no repositório; o Project (Status) é espelho.
// A sequência é: proposta → aceita → em-andamento → em-revisao → Done.
// `bloqueada` e `aguardando-decisao` são desvios laterais; ao sair deles, a
// issue volta para `proposta` (reapply `aceita` se já tinha aceite).
//
// Duas invariantes evitam o vaivém de label:
//
//   1. **Troca atômica.** O estado é aplicado com uma única requisição
//      `PUT /repos/{owner}/{repo}/issues/{n}/labels` contendo o conjunto final.
//      Remover a label antiga e adicionar a nova em duas chamadas gerava um
//      evento `unlabeled` que devolvia a issue para `proposta` logo depois do
//      `labeled` — a entrega inteira terminava na fila.
//   2. **Evento do próprio script é ignorado.** A troca de label é feita com o
//      `GITHUB_TOKEN`, então o GitHub a atribui a `github-actions[bot]`. Esse
//      eco não é decisão de ninguém: quem manda é o evento original.
//
// O estado corrente é sempre lido da API, nunca do payload do evento, que já
// pode estar desatualizado quando a execução começa (o grupo de concorrência
// enfileira execuções do mesmo número).
//
// O token do workflow escreve labels. O Project exige `PROJECT_TOKEN` (PAT com
// escopo `project`); sem ele, o espelho é ignorado e só as labels mudam.

import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const {
  GITHUB_EVENT_NAME: eventName,
  GITHUB_EVENT_PATH: eventPath,
  GITHUB_REPOSITORY: repository,
  GITHUB_TOKEN: repoToken,
  PROJECT_TOKEN: projectToken,
  PROJECT_OWNER: projectOwnerEnv,
  PROJECT_NUMBER: projectNumberEnv = "10",
} = process.env;

const [owner, repo] = (repository || "/").split("/");
const projectOwner = projectOwnerEnv || owner;
const projectNumber = Number(projectNumberEnv);

const STATE = {
  proposta: "estado/proposta",
  aceita: "estado/aceita",
  andamento: "estado/em-andamento",
  revisao: "estado/em-revisao",
  bloqueada: "estado/bloqueada",
  decisao: "estado/aguardando-decisao",
};

const ORDER = [STATE.proposta, STATE.aceita, STATE.andamento, STATE.revisao];
const STATE_LABELS = new Set([...ORDER, STATE.bloqueada, STATE.decisao]);
const PROJECT_STATUS = {
  [STATE.proposta]: "Backlog",
  [STATE.aceita]: "Pronto",
  [STATE.andamento]: "Em andamento",
  [STATE.revisao]: "Em revisão",
  [STATE.bloqueada]: "Bloqueado",
  [STATE.decisao]: "Bloqueado",
};
const RANK = new Map(ORDER.map((label, index) => [label, index]));

/** Identidade com que o `GITHUB_TOKEN` assina as mudanças que ele mesmo faz. */
const SELF_LOGINS = new Set(["github-actions[bot]", "github-actions"]);

/** Ações de issue em que o próprio script pode ser o autor da mudança. */
const SELF_ACTIONS = new Set(["labeled", "unlabeled"]);

export function currentState(issue) {
  const labels = (issue.labels || []).map((label) => label.name);
  return [...STATE_LABELS].find((label) => labels.includes(label)) || null;
}

export function canAdvance(current, desired) {
  if (!current || !desired) return true;
  if (current === STATE.bloqueada || current === STATE.decisao) return true;
  if (desired === STATE.bloqueada || desired === STATE.decisao) return true;
  return RANK.get(desired) >= RANK.get(current);
}

/**
 * Evento causado pela troca de label que o próprio script fez. Reprocessá-lo
 * reverte a transição: o `unlabeled` da label antiga chega depois do `labeled`
 * da nova e devolvia a issue para `proposta`.
 */
export function isSelfEvent(event) {
  const login = event?.sender?.login;
  if (!login || !SELF_LOGINS.has(login)) return false;
  return SELF_ACTIONS.has(event.action);
}

/**
 * Conjunto final de labels para `desired`: as labels que não são de estado são
 * preservadas e no máximo uma `estado/*` sobra. `changed` falso significa que a
 * API não precisa ser chamada.
 */
export function nextLabelSet(labels, desired) {
  const names = (labels || []).map((label) => (typeof label === "string" ? label : label.name));
  const keep = names.filter((name) => !STATE_LABELS.has(name));
  const estados = names.filter((name) => STATE_LABELS.has(name));
  const changed = !(estados.length === 1 && estados[0] === desired);
  return { labels: [...keep, desired], changed };
}

/**
 * Destino de um `unlabeled` de estado: se ainda há outra label de estado, ela
 * manda; só a remoção da última volta para a fila.
 */
export function planUnlabeled(issue) {
  const current = currentState(issue);
  if (current) return { desired: null, keep: current };
  return { desired: STATE.proposta, keep: null };
}

async function rest(method, path, body) {
  const response = await fetch(`https://api.github.com${path}`, {
    method,
    headers: {
      Authorization: `Bearer ${repoToken}`,
      Accept: "application/vnd.github+json",
      "X-GitHub-Api-Version": "2022-11-28",
      "User-Agent": "wyd-issue-status",
    },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  if (!response.ok) {
    throw new Error(`${method} ${path}: ${response.status} ${await response.text()}`);
  }
  return response.status === 204 ? null : response.json();
}

async function repoGraphql(query, variables) {
  return graphql(query, variables, repoToken);
}

async function graphql(query, variables, token) {
  const response = await fetch("https://api.github.com/graphql", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
      "User-Agent": "wyd-issue-status",
    },
    body: JSON.stringify({ query, variables }),
  });
  const text = await response.text();
  if (!response.ok) throw new Error(`GraphQL: ${response.status} ${text}`);
  const payload = JSON.parse(text);
  if (payload.errors?.length) throw new Error(`GraphQL: ${JSON.stringify(payload.errors)}`);
  return payload.data;
}

async function getIssue(number) {
  return rest("GET", `/repos/${owner}/${repo}/issues/${number}`);
}

async function setState(issue, desired, force = false) {
  if (!desired) return null;
  // O payload do evento pode ter sido montado antes da última troca de label.
  const fresh = await getIssue(issue.number);
  const current = currentState(fresh);
  if (!force && !canAdvance(current, desired)) {
    console.log(`#${fresh.number}: mantém ${current} (não regride para ${desired})`);
    return current;
  }

  const { labels, changed } = nextLabelSet(fresh.labels, desired);
  if (!changed) {
    console.log(`#${fresh.number}: já está em ${desired}; nenhuma label alterada`);
    return desired;
  }

  // Uma requisição: o GitHub calcula a diferença e não há janela intermediária
  // em que a issue fique sem estado.
  await rest("PUT", `/repos/${owner}/${repo}/issues/${fresh.number}/labels`, { labels });
  console.log(`#${fresh.number}: ${current || "sem estado"} → ${desired} (troca atômica)`);
  return desired;
}

async function linkedIssues(number) {
  const data = await repoGraphql(
    `query($owner: String!, $repo: String!, $number: Int!) {
       repository(owner: $owner, name: $repo) {
         pullRequest(number: $number) {
           closingIssuesReferences(first: 20) { nodes { number } }
         }
       }
     }`,
    { owner, repo, number },
  );
  return data.repository.pullRequest.closingIssuesReferences.nodes.map((node) => node.number);
}

const PROJECT_QUERY = `query($owner: String!, $number: Int!, $cursor: String) {
  user(login: $owner) {
    projectV2(number: $number) {
      id
      field(name: "Status") {
        ... on ProjectV2SingleSelectField { id options { id name } }
      }
      items(first: 100, after: $cursor) {
        pageInfo { hasNextPage endCursor }
        nodes { id content { ... on Issue { number } } }
      }
    }
  }
}`;

const ADD_ITEM = `mutation($project: ID!, $content: ID!) {
  addProjectV2ItemById(input: { projectId: $project, contentId: $content }) {
    item { id }
  }
}`;

const SET_STATUS = `mutation($project: ID!, $item: ID!, $field: ID!, $option: String!) {
  updateProjectV2ItemFieldValue(input: {
    projectId: $project, itemId: $item, fieldId: $field,
    value: { singleSelectOptionId: $option }
  }) { projectV2Item { id } }
}`;

async function syncProject(issue, statusName) {
  if (!statusName) return;
  if (!projectToken) {
    console.log("::notice::PROJECT_TOKEN ausente; Status do Project não sincronizado.");
    return;
  }

  try {
    let cursor = null;
    let project = null;
    const items = [];
    do {
      const data = await graphql(PROJECT_QUERY, { owner: projectOwner, number: projectNumber, cursor }, projectToken);
      project = data.user?.projectV2;
      if (!project) throw new Error(`Project ${projectOwner}#${projectNumber} não encontrado`);
      items.push(...project.items.nodes);
      cursor = project.items.pageInfo.hasNextPage ? project.items.pageInfo.endCursor : null;
    } while (cursor);
    project.items.nodes = items;

    const field = project.field;
    const option = field?.options?.find((candidate) => candidate.name === statusName);
    if (!field || !option) throw new Error(`Status "${statusName}" não existe no Project`);

    let item = project.items.nodes.find((candidate) => candidate.content?.number === issue.number);
    if (!item) {
      const added = await graphql(ADD_ITEM, { project: project.id, content: issue.node_id }, projectToken);
      item = { id: added.addProjectV2ItemById.item.id };
    }
    await graphql(
      SET_STATUS,
      { project: project.id, item: item.id, field: field.id, option: option.id },
      projectToken,
    );
    console.log(`#${issue.number}: Project → ${statusName}`);
  } catch (error) {
    // O Project é espelho; a label não pode ficar refém de um PAT ausente ou sem escopo.
    console.log(`::warning::Project não sincronizado para #${issue.number}: ${error.message}`);
  }
}

async function transition(issue, desired, { force = false, status = null } = {}) {
  const aplicado = await setState(issue, desired, force);
  // O Status segue a label efetiva, não a desejada: um evento repetido pode
  // manter `em-revisao` e não deve regredir o Project para `Em andamento`.
  await syncProject(issue, status || PROJECT_STATUS[aplicado] || null);
}

async function handleIssue(event) {
  const issue = event.issue;
  const action = event.action;

  if (isSelfEvent(event)) {
    console.log(`#${issue.number}: ${action} por ${event.sender?.login} ignorado (eco do próprio script)`);
    return;
  }

  if (action === "closed") {
    // A label fica como está; Done é o estado do Project e o fechamento é o fato.
    await syncProject(issue, "Done");
    return;
  }

  if (action === "opened") {
    await transition(issue, STATE.proposta, { status: "Backlog" });
    return;
  }

  if (action === "reopened") {
    // A label não é removida no fechamento; reabrir retoma o último estado.
    const fresh = await getIssue(issue.number);
    const current = currentState(fresh);
    const accepted = (fresh.labels || []).some((label) => label.name === STATE.aceita);
    const retomado = current && ![STATE.bloqueada, STATE.decisao].includes(current)
      ? current
      : accepted
        ? STATE.aceita
        : STATE.proposta;
    await transition(fresh, retomado, { force: true });
    return;
  }

  if (action === "labeled") {
    if (!STATE_LABELS.has(event.label.name)) return;
    await transition(issue, event.label.name);
    return;
  }

  if (action === "unlabeled") {
    if (!STATE_LABELS.has(event.label.name)) return;
    // Só a remoção da última label de estado devolve a issue para a fila; se
    // outra label de estado ainda está presente, ela é o estado vigente.
    const fresh = await getIssue(issue.number);
    const { desired, keep } = planUnlabeled(fresh);
    if (!desired) {
      console.log(`#${fresh.number}: mantém ${keep} (outra label de estado presente)`);
      await syncProject(fresh, PROJECT_STATUS[keep] || null);
      return;
    }
    await transition(fresh, desired, { status: "Backlog" });
  }
}

async function handlePullRequest(event) {
  const pull = event.pull_request;
  const action = event.action;
  const numbers = await linkedIssues(pull.number);

  for (const number of numbers) {
    const issue = await getIssue(number);
    if (issue.state === "closed" && action !== "closed") continue;

    if (action === "opened") {
      await transition(issue, STATE.andamento);
    } else if (action === "ready_for_review") {
      await transition(issue, STATE.revisao);
    } else if (action === "reopened") {
      await transition(issue, STATE.andamento, { force: true, status: "Em andamento" });
    } else if (action === "closed") {
      if (pull.merged) {
        await syncProject(issue, "Done");
      } else {
        await transition(issue, STATE.aceita, { force: true, status: "Pronto" });
      }
    }
  }
}

async function handleDispatch(event) {
  const number = Number(event.inputs?.issue);
  if (!Number.isInteger(number) || number <= 0) throw new Error("input issue inválido");
  const issue = await getIssue(number);
  const desired = currentState(issue) || STATE.proposta;
  await transition(issue, desired, { force: true });
}

async function main() {
  if (!repository || !repoToken) {
    console.error("GITHUB_REPOSITORY e GITHUB_TOKEN são obrigatórios");
    process.exit(2);
  }
  if (!eventPath) {
    console.error("GITHUB_EVENT_PATH é obrigatório");
    process.exit(2);
  }

  const event = JSON.parse(readFileSync(eventPath, "utf8"));

  if (eventName === "issues") {
    await handleIssue(event);
  } else if (eventName === "pull_request_target") {
    await handlePullRequest(event);
  } else if (eventName === "workflow_dispatch") {
    await handleDispatch(event);
  } else {
    console.log(`evento ignorado: ${eventName}`);
  }
}

// Só executa quando chamado como script; importado (teste), exporta as regras.
if (process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url) {
  await main();
}

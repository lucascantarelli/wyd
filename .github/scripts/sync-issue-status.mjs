#!/usr/bin/env node
// Transição de estado de issue — fonte única da sequência.
//
// Labels são a fonte de verdade no repositório; o Project (Status) é espelho.
// A sequência é: proposta → aceita → em-andamento → em-revisao → Done.
// `bloqueada` e `aguardando-decisao` são desvios laterais; ao sair deles, a
// issue volta para `proposta` (reapply `aceita` se já tinha aceite).
//
// O token do workflow escreve labels. O Project exige `PROJECT_TOKEN` (PAT com
// escopo `project`); sem ele, o espelho é ignorado e só as labels mudam.

import { readFileSync } from "node:fs";

const {
  GITHUB_EVENT_NAME: eventName,
  GITHUB_EVENT_PATH: eventPath,
  GITHUB_REPOSITORY: repository,
  GITHUB_TOKEN: repoToken,
  PROJECT_TOKEN: projectToken,
  PROJECT_OWNER: projectOwnerEnv,
  PROJECT_NUMBER: projectNumberEnv = "10",
} = process.env;

if (!repository || !repoToken) {
  console.error("GITHUB_REPOSITORY e GITHUB_TOKEN são obrigatórios");
  process.exit(2);
}

const [owner, repo] = repository.split("/");
const event = JSON.parse(readFileSync(eventPath, "utf8"));
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

function currentState(issue) {
  const labels = (issue.labels || []).map((label) => label.name);
  return [...STATE_LABELS].find((label) => labels.includes(label)) || null;
}

function canAdvance(current, desired) {
  if (!current || !desired) return true;
  if (current === STATE.bloqueada || current === STATE.decisao) return true;
  if (desired === STATE.bloqueada || desired === STATE.decisao) return true;
  return RANK.get(desired) >= RANK.get(current);
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
  if (!desired) return;
  const current = currentState(issue);
  if (!force && !canAdvance(current, desired)) {
    console.log(`#${issue.number}: mantém ${current} (não regride para ${desired})`);
    return;
  }

  const labels = (issue.labels || []).map((label) => label.name);
  const remove = [...STATE_LABELS].filter((label) => label !== desired && labels.includes(label));
  for (const label of remove) {
    await rest("DELETE", `/repos/${owner}/${repo}/issues/${issue.number}/labels/${encodeURIComponent(label)}`);
  }
  if (!labels.includes(desired)) {
    await rest("POST", `/repos/${owner}/${repo}/issues/${issue.number}/labels`, { labels: [desired] });
  }
  console.log(`#${issue.number}: ${current || "sem estado"} → ${desired}`);
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
  if (desired) await setState(issue, desired, force);
  await syncProject(issue, status || PROJECT_STATUS[desired] || null);
}

async function handleIssue() {
  const issue = event.issue;
  const action = event.action;

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
    const current = currentState(issue);
    const accepted = (issue.labels || []).some((label) => label.name === STATE.aceita);
    const retomado = current && ![STATE.bloqueada, STATE.decisao].includes(current)
      ? current
      : accepted
        ? STATE.aceita
        : STATE.proposta;
    await transition(issue, retomado, { force: true });
    return;
  }

  if (action === "labeled") {
    if (!STATE_LABELS.has(event.label.name)) return;
    await transition(issue, event.label.name);
    return;
  }

  if (action === "unlabeled") {
    if (!STATE_LABELS.has(event.label.name)) return;
    // Sem histórico de qual estado foi substituído, o seguro é voltar à fila.
    await transition(issue, STATE.proposta, { force: true, status: "Backlog" });
  }
}

async function handlePullRequest() {
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

async function handleDispatch() {
  const number = Number(event.inputs?.issue);
  if (!Number.isInteger(number) || number <= 0) throw new Error("input issue inválido");
  const issue = await getIssue(number);
  const desired = currentState(issue) || STATE.proposta;
  await transition(issue, desired, { force: true });
}

if (eventName === "issues") {
  await handleIssue();
} else if (eventName === "pull_request_target") {
  await handlePullRequest();
} else if (eventName === "workflow_dispatch") {
  await handleDispatch();
} else {
  console.log(`evento ignorado: ${eventName}`);
}

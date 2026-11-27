// Regressão da transição de estado: a troca de label tem de sobreviver ao eco
// do próprio script. Sem isso, o `unlabeled` da label antiga (autor
// `github-actions[bot]`) chegava depois do `labeled` da nova e devolvia toda
// issue entregue para `estado/proposta`.
//
// Dado é montado em memória, sem id mágico, `sleep` ou estado global: as regras
// exportadas são puras e o teste não fala com o GitHub.

import assert from "node:assert/strict";
import { test } from "node:test";

import {
  canAdvance,
  currentState,
  isClosedIssue,
  isSelfEvent,
  nextLabelSet,
  planUnlabeled,
} from "../../scripts/sync-issue-status.mjs";

const comLabel = (...nomes) => ({
  number: 1,
  labels: nomes.map((name) => ({ name })),
});

test("troca de estado vai numa lista final só, sem a label anterior", () => {
  const { labels, changed } = nextLabelSet(
    ["area/wyd-ops", "estado/proposta", "fase/5"],
    "estado/aceita",
  );

  assert.equal(changed, true);
  assert.deepEqual(labels, ["area/wyd-ops", "fase/5", "estado/aceita"]);
  assert.ok(!labels.includes("estado/proposta"));
});

test("estado já aplicado não gera chamada de label", () => {
  const { labels, changed } = nextLabelSet(["area/wyd-ops", "estado/aceita"], "estado/aceita");

  assert.equal(changed, false);
  assert.deepEqual(labels, ["area/wyd-ops", "estado/aceita"]);
});

test("avançar remove a label lateral, mantendo só a nova", () => {
  const { labels } = nextLabelSet(["estado/bloqueada", "estado/aceita"], "estado/em-andamento");

  assert.deepEqual(labels, ["estado/em-andamento"]);
});

test("eco do próprio script é ignorado", () => {
  const eco = { action: "unlabeled", label: { name: "estado/proposta" }, sender: { login: "github-actions[bot]" } };

  assert.equal(isSelfEvent(eco), true);
  assert.equal(isSelfEvent({ ...eco, action: "labeled" }), true);
});

test("evento humano e evento de outra ação não são eco", () => {
  const humano = { action: "labeled", label: { name: "estado/aceita" }, sender: { login: "lucascantarelli" } };

  assert.equal(isSelfEvent(humano), false);
  assert.equal(isSelfEvent({ ...humano, sender: { login: "github-actions[bot]" }, action: "opened" }), false);
  assert.equal(isSelfEvent({ action: "unlabeled" }), false);
});

test("unlabeled com outra label de estado presente não devolve para a fila", () => {
  const plano = planUnlabeled(comLabel("estado/em-revisao"));

  assert.deepEqual(plano, { desired: null, keep: "estado/em-revisao" });
});

test("unlabeled da última label devolve para a fila", () => {
  assert.deepEqual(planUnlabeled(comLabel()), { desired: "estado/proposta", keep: null });
});

test("issue fechada é terminal: label e Status não são reescritos", () => {
  assert.equal(isClosedIssue({ state: "closed" }), true);
  assert.equal(isClosedIssue({ state: "open" }), false);
  assert.equal(isClosedIssue({}), false);
});

test("a transição aceita sobrevive ao eco do unlabeled", () => {
  const antes = comLabel("area/wyd-ops", "estado/proposta");

  // Evento humano aplicando o aceite.
  const aceite = { action: "labeled", label: { name: "estado/aceita" }, sender: { login: "lucascantarelli" } };
  assert.equal(isSelfEvent(aceite), false);
  assert.equal(canAdvance(currentState(antes), aceite.label.name), true);

  const { labels } = nextLabelSet(antes.labels, aceite.label.name);
  const depois = comLabel(...labels);
  assert.equal(currentState(depois), "estado/aceita");

  // Eco: o GitHub reporta a remoção de `estado/proposta` feita pelo script.
  const eco = {
    action: "unlabeled",
    label: { name: "estado/proposta" },
    sender: { login: "github-actions[bot]" },
    issue: depois,
  };

  assert.equal(isSelfEvent(eco), true);
  assert.equal(currentState(eco.issue), "estado/aceita");
});

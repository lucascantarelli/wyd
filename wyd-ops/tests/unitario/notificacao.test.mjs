// Regressão do relatório de CI: as duas grandezas de tempo, os rótulos que
// enganavam e os limites do Block Kit.
//
// Dado montado em memória, sem id mágico, `sleep` ou estado global: o modelo, o
// render e o payload são puros e o teste não fala com o GitHub nem com o Slack.

import assert from "node:assert/strict";
import { test } from "node:test";

import { JOB_DE_AVISO, dominioDoJob, duracaoSegundos, montarModelo } from "../../scripts/notificacao/modelo.mjs";
import { celula, formatarDuracao, linhaDoAviso, markdown } from "../../scripts/notificacao/texto.mjs";
import { LIMITES, escapar, payloadSlack, textoDoAviso, validarPayload } from "../../scripts/notificacao/slack.mjs";
import { motivoDoJob, motivosPulados } from "../../scripts/notificacao/workflow.mjs";

const AGORA = new Date("2026-11-28T11:15:00Z");
const SHA = "f8e51d946887165390067766ea245f5e74853dc3";

function job(nome, conclusao, inicio, fim, extra = {}) {
  return { nome, conclusao, inicio, fim, url: `https://exemplo.test/job/${encodeURIComponent(nome)}`, ...extra };
}

function dados(overrides = {}) {
  return {
    workflow: "ci",
    run: {
      numero: 99,
      tentativa: 1,
      evento: "pull_request",
      ref: "chore/ci-centralizacao-proposta",
      sha: SHA,
      url: "https://exemplo.test/run/99",
      workflowId: 407278205,
      criadoEm: "2026-11-28T11:11:37Z",
      atualizadoEm: "2026-11-28T11:13:19Z",
    },
    // 74 s de relógio (primeiro início → último fim) contra 206 s de soma:
    // é a diferença que o aviso antigo escondia atrás de "Σ Soma dos jobs".
    jobs: [
      job("pg / runner", "success", "2026-11-28T11:11:37Z", "2026-11-28T11:12:51Z"),
      job("wyd-portal", "success", "2026-11-28T11:11:37Z", "2026-11-28T11:12:42Z"),
      job("pg / schema", "success", "2026-11-28T11:11:37Z", "2026-11-28T11:12:24Z"),
      job("cobertura", "success", "2026-11-28T11:11:40Z", "2026-11-28T11:12:00Z"),
      job("publicar no GitHub Pages", "skipped", null, null, { motivo: "github.event_name == 'push'" }),
      job(JOB_DE_AVISO, null, "2026-11-28T11:12:55Z", null),
    ],
    commit: {
      sha: SHA,
      autor: "lucascantarelli",
      assunto: "fix(ci): aviso | legível",
      data: "2026-11-28T11:10:00Z",
      url: `https://exemplo.test/commit/${SHA}`,
    },
    pr: { numero: 242, titulo: "chore(ci): relatório único por run", url: "https://exemplo.test/pr/242", autor: "lucascantarelli" },
    issue: { numero: 7, titulo: "Notificação do CI", url: "https://exemplo.test/issue/7", estado: "OPEN" },
    amostra: [
      { conclusao: "success", fim: "2026-11-27T10:00:00Z" },
      { conclusao: "failure", fim: "2026-11-28T10:00:00Z" },
      { conclusao: "cancelled", fim: "2026-11-28T10:10:00Z" },
      { conclusao: "success", fim: "2026-11-28T10:20:00Z" },
    ],
    cobertura: [
      { crate: "wyd-db", pct: 67.77 },
      { crate: "wyd-launcher", pct: 0 },
    ],
    autor: "lucascantarelli",
    ...overrides,
  };
}

const modelo = (overrides, opcoes) => montarModelo(dados(overrides), { agora: AGORA, ...opcoes });

test("duração ignora ponta ausente e não devolve negativo", () => {
  assert.equal(duracaoSegundos("2026-11-28T11:11:37Z", "2026-11-28T11:12:51Z"), 74);
  assert.equal(duracaoSegundos(null, "2026-11-28T11:12:51Z"), null);
  assert.equal(duracaoSegundos("2026-11-28T11:13:00Z", "2026-11-28T11:12:00Z"), 0);
});

test("tempo de pipeline e consumo de runner são grandezas diferentes", () => {
  const m = modelo();
  assert.equal(m.tempo.pipelineSegundos, 74, "parede: primeiro início → último fim");
  assert.equal(m.tempo.consumoSegundos, 206, "soma das durações dos jobs");
  assert.notEqual(m.tempo.pipelineSegundos, m.tempo.consumoSegundos);
  assert.equal(m.tempo.paralelismo, 2.8, "consumo ÷ pipeline, uma casa decimal");
});

test("o job de aviso não entra na contagem nem no veredito", () => {
  const m = modelo();
  assert.equal(m.jobs.total, 5);
  assert.equal(m.jobs.ok, 4);
  assert.equal(m.jobs.pulado, 1);
  assert.equal(m.jobs.falha, 0);
  assert.equal(m.jobs.semVeredito, 1);
  assert.equal(m.veredito, "sucesso");
});

test("job pulado aparece uma vez só, na seção 3, com o motivo", () => {
  const m = modelo();
  const publicacao = m.jobs.grupos.find((grupo) => grupo.nome === "Publicação");
  assert.deepEqual(publicacao.jobs.map((job) => job.nome), ["publicar no GitHub Pages"]);
  assert.equal(publicacao.jobs[0].motivo, "github.event_name == 'push'");
  assert.equal(publicacao.pulado, 1);
  assert.match(markdown(m), /publicar no GitHub Pages \| ⏭️ skipped \| — \| pulado — `github\.event_name == 'push'`/);
});

test("domínios agrupam todos os jobs, na ordem do dashboard", () => {
  const m = modelo();
  assert.deepEqual(m.jobs.grupos.map((grupo) => grupo.nome), ["Código", "Banco", "Imagem", "Publicação"]);
  assert.equal(
    m.jobs.grupos.reduce((soma, grupo) => soma + grupo.jobs.length, 0),
    m.jobs.total,
    "nenhum job some e nenhum é contado duas vezes",
  );
  assert.equal(dominioDoJob("gate-security"), "Segurança");
  assert.equal(dominioDoJob("construir e publicar"), "Imagem");
  assert.equal(dominioDoJob("job desconhecido"), "Outros");
});

// Conta o nome no texto **visível**: o caminho do job aparece dentro da URL do log
// (`/job/wyd-portal`), e isso não é o job sendo listado duas vezes.
function ocorrencias(texto, agulha) {
  return texto.replace(/https?:\/\/[^\s|)>]+/g, "").split(agulha).length - 1;
}

function textosDoSlack(payload) {
  return payload.attachments[0].blocks
    .flatMap((bloco) => [
      bloco.text?.text,
      ...(bloco.fields ?? []).map((campo) => campo.text),
      ...(bloco.elements ?? []).map((elemento) => elemento.text ?? ""),
    ])
    .filter(Boolean)
    .join("\n");
}

test("cada job aparece uma vez por canal", () => {
  const m = modelo();
  const nomes = ["pg / runner", "wyd-portal", "pg / schema", "cobertura", "publicar no GitHub Pages"];
  const canais = { markdown: markdown(m), slack: textosDoSlack(payloadSlack(m)) };
  for (const [canal, texto] of Object.entries(canais)) {
    for (const nome of nomes) {
      assert.equal(ocorrencias(texto, nome), 1, `${canal} repete \`${nome}\``);
    }
  }
});

test("falha muda o veredito e entra na lista de falhas", () => {
  const m = modelo({
    jobs: [
      job("pg / runner", "failure", "2026-11-28T11:11:37Z", "2026-11-28T11:12:51Z"),
      job(JOB_DE_AVISO, null, "2026-11-28T11:12:55Z", null),
    ],
  });
  assert.equal(m.veredito, "falha");
  assert.deepEqual(m.jobs.falhas.map((f) => f.nome), ["pg / runner"]);
});

test("DORA usa o N real da amostra e ignora run cancelado", () => {
  const m = modelo();
  assert.equal(m.dora.amostra, 4, "não é 30: é o que a branch devolveu");
  assert.equal(m.dora.falhas, 1, "cancelled não é incidente");
  assert.equal(m.dora.taxa, 25);
  assert.equal(m.dora.mttrSegundos, 1200, "falha 10:00 → sucesso seguinte 10:20");
  assert.equal(m.dora.verdes7, 2);
});

test("lead time de pull_request vem do commit do head e é marcado como proxy", () => {
  const pr = modelo();
  assert.equal(pr.dora.leadSegundos, 300);
  assert.equal(pr.dora.leadProxy, true);
  assert.equal(pr.dora.leadCommit, "f8e51d9");

  const push = modelo({ run: { ...dados().run, evento: "push" } });
  assert.equal(push.dora.leadProxy, false);
});

test("amostra da branch padrão é declarada, não confundida com a branch do run", () => {
  const m = modelo({ amostraEscopo: "padrão", amostraBranch: "develop" });
  assert.match(markdown(m), /runs concluídos deste workflow em `develop` \(esta branch não tem histórico\)/);
  assert.match(JSON.stringify(payloadSlack(m)), /amostra: develop/);
});

test("cobertura 0 vira ausência, não 0%", () => {
  const m = modelo();
  assert.deepEqual(m.cobertura, [{ crate: "wyd-db", pct: 67.77 }]);

  const vazia = modelo({ cobertura: [{ crate: "wyd-db", pct: 0 }] });
  assert.deepEqual(vazia.cobertura, []);
  assert.match(markdown(vazia), /\| Cobertura \| — \|/);
});

test("markdown nomeia as duas grandezas e diz que a soma não é tempo decorrido", () => {
  const texto = markdown(modelo());
  assert.match(texto, /### ✅ ci — sucesso/);
  assert.match(texto, /\| Tempo de pipeline \| 1m 14s \|/);
  assert.match(texto, /\| Consumo de runner \(soma dos jobs\) \| 3m 26s \|/);
  assert.match(texto, /minuto de runner gasto, não tempo decorrido/);
  assert.match(texto, /\| Paralelismo \| 2,8× \|/);
  assert.match(texto, /o job de aviso não entra, porque é ele que publica este relatório/);
  assert.match(texto, /\| Falhas na amostra \| 1 de 4 \(25%\) \|/);
  assert.match(texto, /commit do PR → aviso \(proxy\)/);
  assert.match(texto, /qualquer run concluído com sucesso na amostra, não só deploy/);
});

test("markdown mostra PR, issue e autor quando existem e omite quando não", () => {
  const texto = markdown(modelo());
  assert.match(texto, /\[#242\]\(https:\/\/exemplo\.test\/pr\/242\)/);
  assert.match(texto, /\[#7\]\(https:\/\/exemplo\.test\/issue\/7\)/);
  assert.match(texto, /Autor do commit \| `lucascantarelli`/);

  const sem = markdown(modelo({ pr: null, issue: null, commit: null }));
  assert.doesNotMatch(sem, /\| Issue \|/);
  assert.doesNotMatch(sem, /\| PR \|/);
  assert.doesNotMatch(sem, /Autor do commit/);
});

test("célula de tabela escapa pipe e quebra de linha", () => {
  assert.equal(celula("a | b\nc"), "a \\| b c");
  assert.match(markdown(modelo()), /fix\(ci\): aviso \\\| legível/);
});

test("linha do log resume pipeline e consumo", () => {
  assert.equal(
    linhaDoAviso(modelo()),
    "✅ ci sucesso · chore/ci-centralizacao-proposta (5 jobs, pipeline 1m 14s, consumo 3m 26s, 0 falha(s), amostra 1/4)",
  );
});

test("payload do Slack tem quebra de linha real, não barra invertida", () => {
  const textos = textosDoSlack(payloadSlack(modelo()));
  assert.match(textos, /\*1 · Resultado\*\n/);
  assert.match(textos, /\*Ref\* `chore\/ci-centralizacao-proposta`/);
  assert.ok(!textos.includes("\\n"), "o `\\n` literal era o defeito do aviso antigo");
});

test("payload do Slack escapa markup e separa as duas grandezas de tempo", () => {
  const payload = payloadSlack(modelo({ commit: { ...dados().commit, assunto: "fix: A & B <c>" } }));
  const textos = textosDoSlack(payload);
  assert.match(textos, /A &amp; B &lt;c&gt;/);
  assert.match(textos, /\*Tempo de pipeline\* 1m 14s/);
  assert.match(textos, /\*Consumo de runner\* 3m 26s/);
});

test("payload do Slack mostra PR, issue e motivo do job pulado", () => {
  const textos = textosDoSlack(payloadSlack(modelo()));
  assert.match(textos, /PR\* <https:\/\/exemplo\.test\/pr\/242\|#242>/);
  assert.match(textos, /Issue\* <https:\/\/exemplo\.test\/issue\/7\|#7>/);
  assert.match(textos, /pulado por `github\.event_name == 'push'`/);
});

test("fallback do Slack usa Unicode, nunca código :emoji:", () => {
  const texto = textoDoAviso(modelo());
  assert.match(texto, /^✅ ci passou/);
  assert.doesNotMatch(texto, /:[a-z_]+:/);
  assert.match(texto, /tempo de pipeline 1m 14s/);
  assert.match(texto, /consumo de runner 3m 26s/);
  assert.match(texto, /Jobs 5 \(✅ 4 · ❌ 0 · ⏭️ 1\)/);
  assert.doesNotMatch(texto, /pg \/ runner/, "o fallback conta, não lista job");
});

test("validador aceita o payload normal e recusa o que o Block Kit não aceita", () => {
  const payload = payloadSlack(modelo());
  assert.equal(validarPayload(payload), payload);

  const muitosBlocos = structuredClone(payload);
  muitosBlocos.attachments[0].blocks = Array.from({ length: LIMITES.blocos + 1 }, () => ({ type: "divider" }));
  assert.throws(() => validarPayload(muitosBlocos), /limite 50/);

  const textao = structuredClone(payload);
  const secaoLonga = textao.attachments[0].blocks.find((bloco) => bloco.type === "section" && bloco.text);
  secaoLonga.text.text = "x".repeat(LIMITES.texto + 1);
  assert.throws(() => validarPayload(textao), /limite 3000/);

  const cabecalho = structuredClone(payload);
  cabecalho.attachments[0].blocks[0].text.text = "x".repeat(LIMITES.cabecalho + 1);
  assert.throws(() => validarPayload(cabecalho), /cabeçalho/);

  const semFallback = structuredClone(payload);
  delete semFallback.text;
  assert.throws(() => validarPayload(semFallback), /fallback/);
});

test("assunto enorme estoura o limite e o job falha em vez de postar torto", () => {
  const m = modelo({ commit: { ...dados().commit, assunto: "y".repeat(3100) } });
  assert.throws(() => validarPayload(payloadSlack(m)), /limite 3000/);
});

test("escapar cobre os três caracteres de markup", () => {
  assert.equal(escapar("a & b < c > d"), "a &amp; b &lt; c &gt; d");
});

test("formatarDuracao usa segundos abaixo de um minuto", () => {
  assert.equal(formatarDuracao(0), "0s");
  assert.equal(formatarDuracao(59), "59s");
  assert.equal(formatarDuracao(60), "1m 00s");
  assert.equal(formatarDuracao(832), "13m 52s");
  assert.equal(formatarDuracao(null), "—");
});

const WORKFLOW = `name: ci
on:
  pull_request:
jobs:
  publicar:
    name: publicar no GitHub Pages
    if: github.event_name == 'push'
    runs-on: ubuntu-latest
  build:
    name: construir e publicar
    runs-on: ubuntu-latest
`;

test("motivo do job pulado sai do `if:` do próprio workflow", () => {
  assert.equal(motivoDoJob(WORKFLOW, "publicar no GitHub Pages"), "github.event_name == 'push'");
  assert.equal(motivoDoJob(WORKFLOW, "construir e publicar"), null, "job sem `if:` não tem motivo");
  assert.equal(motivoDoJob(WORKFLOW, "job inexistente"), null);
  assert.equal(motivoDoJob(null, "publicar no GitHub Pages"), null);
});

test("motivosPulados monta o mapa dos nomes pedidos", () => {
  const mapa = motivosPulados(WORKFLOW, ["publicar no GitHub Pages", "construir e publicar"]);
  assert.equal(mapa.get("publicar no GitHub Pages"), "github.event_name == 'push'");
  assert.equal(mapa.get("construir e publicar"), null);
});

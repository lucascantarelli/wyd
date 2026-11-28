// Modelo do relatório de CI — puro de propósito.
//
// Recebe dados já coletados e devolve números e listas; não chama rede, não lê
// arquivo e não olha o relógio (o instante entra por parâmetro). É esta separação
// que permite provar o cálculo com `node --test`, sem GitHub e sem webhook.
//
// Duas grandezas que o aviso antigo confundia:
//
//   - **tempo de pipeline** (`pipelineSegundos`): relógio do run, do primeiro job
//     iniciado ao último concluído. É o tempo que a pessoa esperou.
//   - **consumo de runner** (`consumoSegundos`): soma das durações dos jobs. Com
//     matriz em paralelo a soma fica ~9× maior que o relógio; é minuto de runner
//     gasto, não tempo decorrido. Fica exibida como consumo, com o paralelismo.

/** Nome do job que publica o próprio aviso: ele nunca entra no veredito. */
export const JOB_DE_AVISO = "notificar resultado";

const JANELA_VERDE_SEGUNDOS = 7 * 24 * 60 * 60;
const CONCLUSAO_COM_FALHA = new Set(["failure", "timed_out"]);

function paraData(iso) {
  if (!iso) return null;
  const data = new Date(iso);
  return Number.isNaN(data.getTime()) ? null : data;
}

/** Duração em segundos entre dois instantes ISO; `null` quando falta um deles. */
export function duracaoSegundos(inicio, fim) {
  const a = paraData(inicio);
  const b = paraData(fim);
  if (!a || !b) return null;
  const segundos = Math.round((b.getTime() - a.getTime()) / 1000);
  return segundos < 0 ? 0 : segundos;
}

function menorInicio(jobs) {
  return jobs.reduce((menor, job) => (job.inicio < menor ? job.inicio : menor), jobs[0].inicio);
}

function maiorFim(jobs) {
  return jobs.reduce((maior, job) => (job.fim > maior ? job.fim : maior), jobs[0].fim);
}

function arredondar(valor, casas = 1) {
  const fator = 10 ** casas;
  return Math.round(valor * fator) / fator;
}

/**
 * Monta o modelo do relatório.
 *
 * @param {object} dados saída de `coleta.mjs` (run, jobs, evento, commit, amostra, cobertura)
 * @param {{agora?: Date, jobDeAviso?: string}} opcoes
 */
export function montarModelo(dados, { agora = new Date(), jobDeAviso = JOB_DE_AVISO } = {}) {
  const run = dados.run ?? {};
  const todos = dados.jobs ?? [];
  const executados = todos.filter((job) => job.nome !== jobDeAviso);
  const doAviso = todos.filter((job) => job.nome === jobDeAviso);
  const comTempo = executados
    .map((job) => ({ ...job, segundos: duracaoSegundos(job.inicio, job.fim) }))
    .filter((job) => job.segundos !== null);

  const pipelineSegundos = comTempo.length
    ? duracaoSegundos(menorInicio(comTempo), maiorFim(comTempo)) ?? 0
    : 0;
  const consumoSegundos = comTempo.reduce((soma, job) => soma + job.segundos, 0);
  const paralelismo = pipelineSegundos > 0 ? arredondar(consumoSegundos / pipelineSegundos) : null;

  const ok = executados.filter((job) => job.conclusao === "success");
  const pulados = executados.filter((job) => job.conclusao === "skipped");
  const falhas = executados.filter(
    (job) => job.conclusao && job.conclusao !== "success" && job.conclusao !== "skipped",
  );
  const semVeredito = executados.filter((job) => !job.conclusao).length + doAviso.length;

  // `todos` alimenta a tabela do resumo (job a job, maior primeiro); `lentos` é o
  // recorte de cinco que cabe no Slack.
  const ordenados = [...comTempo]
    .sort((a, b) => b.segundos - a.segundos)
    .map((job) => ({
      nome: job.nome,
      segundos: job.segundos,
      url: job.url ?? null,
      conclusao: job.conclusao,
    }));
  const lentos = ordenados.slice(0, 5);

  const amostra = dados.amostra ?? [];
  const falhasNaAmostra = amostra.filter((r) => CONCLUSAO_COM_FALHA.has(r.conclusao)).length;
  const taxa = amostra.length ? Math.round((falhasNaAmostra / amostra.length) * 100) : null;
  const limiteVerde = new Date(agora.getTime() - JANELA_VERDE_SEGUNDOS * 1000);
  const verdes7 = amostra.filter((r) => {
    const fim = paraData(r.fim);
    return r.conclusao === "success" && fim !== null && fim >= limiteVerde;
  }).length;

  // MTTR é a última recuperação da amostra: falha seguida do primeiro sucesso
  // posterior. Run cancelado não é incidente e fica de fora dos dois lados.
  const ordenada = [...amostra].sort((a, b) => String(a.fim).localeCompare(String(b.fim)));
  let mttrSegundos = null;
  ordenada.forEach((runAmostra, indice) => {
    if (!CONCLUSAO_COM_FALHA.has(runAmostra.conclusao)) return;
    const proximo = ordenada.slice(indice + 1).find((r) => r.conclusao === "success");
    if (proximo) mttrSegundos = duracaoSegundos(runAmostra.fim, proximo.fim);
  });

  const commit = dados.commit ?? null;
  const dataCommit = paraData(commit?.data);
  const leadSegundos = dataCommit ? Math.max(0, Math.round((agora.getTime() - dataCommit.getTime()) / 1000)) : null;
  const leadProxy = run.evento === "pull_request";

  const cobertura = (dados.cobertura ?? []).filter(
    (item) => Number.isFinite(item.pct) && item.pct > 0,
  );

  return {
    veredito: falhas.length > 0 ? "falha" : "sucesso",
    workflow: dados.workflow ?? run.workflowName ?? "",
    run: {
      numero: run.numero ?? null,
      tentativa: run.tentativa ?? 1,
      url: run.url ?? null,
      evento: run.evento ?? "",
      ref: run.ref ?? "",
      sha: run.sha ?? null,
      criadoEm: run.criadoEm ?? null,
      atualizadoEm: run.atualizadoEm ?? null,
    },
    commit,
    pr: dados.pr ?? null,
    issue: dados.issue ?? null,
    autorDoEvento: dados.autor ?? null,
    tempo: {
      pipelineSegundos,
      consumoSegundos,
      paralelismo,
      inicio: comTempo.length ? menorInicio(comTempo) : null,
      fim: comTempo.length ? maiorFim(comTempo) : null,
    },
    jobs: {
      total: executados.length,
      ok: ok.length,
      falha: falhas.length,
      pulado: pulados.length,
      semVeredito,
      todos: ordenados,
      lentos,
      falhas: falhas.map((job) => ({ nome: job.nome, conclusao: job.conclusao, url: job.url ?? null })),
      pulados: pulados.map((job) => ({ nome: job.nome, motivo: job.motivo ?? null })),
    },
    dora: {
      amostra: amostra.length,
      falhas: falhasNaAmostra,
      taxa,
      mttrSegundos,
      verdes7,
      leadSegundos,
      leadProxy,
      leadCommit: commit?.sha ? commit.sha.slice(0, 7) : null,
    },
    cobertura,
    geradoEm: agora.toISOString(),
  };
}

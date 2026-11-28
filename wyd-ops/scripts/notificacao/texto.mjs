// Render em Markdown do relatório — puro: recebe o modelo, devolve texto.
// É o que vai para `$GITHUB_STEP_SUMMARY`, para o log do job e para o artefato.

/** Duração legível: abaixo de um minuto em segundos, acima em `Xm YYs`. */
export function formatarDuracao(segundos) {
  if (segundos === null || segundos === undefined) return "—";
  if (segundos < 60) return `${segundos}s`;
  const minutos = Math.floor(segundos / 60);
  return `${minutos}m ${String(segundos % 60).padStart(2, "0")}s`;
}

/** Percentual com vírgula decimal, como o resto da documentação em português. */
export function formatarPct(valor) {
  if (valor === null || valor === undefined) return "—";
  return `${String(valor).replace(".", ",")}%`;
}

/** Célula de tabela Markdown: `|` e quebra de linha não podem vazar. */
export function celula(texto) {
  return String(texto ?? "").replace(/\|/g, "\\|").replace(/\r?\n/g, " ");
}

const SELO = { sucesso: "✅", falha: "❌" };
const ESTADO = { sucesso: "sucesso", falha: "falha" };

function link(url, texto) {
  return url ? `[${texto}](${url})` : texto;
}

function linhaDoCampo(rotulo, valor) {
  return `| ${rotulo} | ${celula(valor)} |`;
}

function linhaDeMetrica(rotulo, valor, significado) {
  return `| ${rotulo} | ${celula(valor)} | ${celula(significado)} |`;
}

function linhaDoJob(job) {
  const selo = job.conclusao === "success" ? "✅" : job.conclusao === "skipped" ? "⏭️" : "❌";
  const log = job.url ? `[log](${job.url})` : "—";
  return `| ${celula(job.nome)} | ${selo} ${job.conclusao} | ${formatarDuracao(job.segundos)} | ${log} |`;
}

function linhaDePulado(pulado) {
  return pulado.motivo ? `${pulado.nome} — pulado por \`${pulado.motivo}\`` : `${pulado.nome} — pulado`;
}

function coberturaEmTexto(cobertura) {
  if (!cobertura.length) return "—";
  return cobertura.map((item) => `${item.crate} ${formatarPct(item.pct)}`).join(" · ");
}

/**
 * Markdown completo do run.
 *
 * @param {ReturnType<import("./modelo.mjs").montarModelo>} modelo
 */
export function markdown(modelo) {
  const { run, commit, pr, issue, tempo, jobs, dora, cobertura } = modelo;
  const linhas = [];

  linhas.push(`### ${SELO[modelo.veredito]} ${modelo.workflow} — ${ESTADO[modelo.veredito]}`);
  linhas.push("");
  linhas.push("| Campo | Valor |");
  linhas.push("|---|---|");
  if (run.url || run.numero) {
    const tentativa = run.tentativa > 1 ? ` (tentativa ${run.tentativa})` : "";
    linhas.push(linhaDoCampo("Run", `${link(run.url, `#${run.numero ?? "?"}`)}${tentativa}`));
  }
  if (commit) {
    linhas.push(
      linhaDoCampo("Commit", `\`${(commit.sha ?? "").slice(0, 7)}\`${commit.assunto ? ` — ${commit.assunto}` : ""}`),
    );
    if (commit.autor) linhas.push(linhaDoCampo("Autor do commit", `\`${commit.autor}\``));
  }
  if (pr) linhas.push(linhaDoCampo("PR", `${link(pr.url, `#${pr.numero}`)}${pr.titulo ? ` — ${pr.titulo}` : ""}`));
  if (issue) {
    const estado = issue.estado ? ` · \`${String(issue.estado).toLowerCase()}\`` : "";
    linhas.push(linhaDoCampo("Issue", `${link(issue.url, `#${issue.numero}`)}${issue.titulo ? ` — ${issue.titulo}` : ""}${estado}`));
  }
  if (run.ref) linhas.push(linhaDoCampo("Ref", `\`${run.ref}\``));
  linhas.push(linhaDoCampo("Evento", `\`${run.evento}\``));
  if (modelo.autorDoEvento) linhas.push(linhaDoCampo("Ator", `\`${modelo.autorDoEvento}\``));
  linhas.push(linhaDoCampo("Gerado em", `${modelo.geradoEm.replace("T", " ").slice(0, 16)} UTC`));
  linhas.push("");

  linhas.push("| Métrica | Valor | O que conta |");
  linhas.push("|---|---|---|");
  linhas.push(
    linhaDeMetrica(
      "Tempo de pipeline",
      formatarDuracao(tempo.pipelineSegundos),
      "do primeiro job iniciado ao último concluído — o tempo que o run levou",
    ),
  );
  linhas.push(
    linhaDeMetrica(
      "Consumo de runner (soma dos jobs)",
      formatarDuracao(tempo.consumoSegundos),
      "soma das durações dos jobs: minuto de runner gasto, não tempo decorrido",
    ),
  );
  linhas.push(
    linhaDeMetrica(
      "Paralelismo",
      tempo.paralelismo === null ? "—" : `${String(tempo.paralelismo).replace(".", ",")}×`,
      "consumo de runner ÷ tempo de pipeline",
    ),
  );
  linhas.push(
    linhaDeMetrica(
      "Jobs",
      `${jobs.total} (✅ ${jobs.ok} · ❌ ${jobs.falha} · ⏭️ ${jobs.pulado})`,
      "sem o job de aviso, que publica este relatório",
    ),
  );
  if (dora.leadSegundos !== null) {
    const rotulo = dora.leadProxy ? "commit do PR → aviso (proxy)" : "commit → aviso";
    linhas.push(
      linhaDeMetrica("Lead time", `${formatarDuracao(dora.leadSegundos)} · \`${dora.leadCommit ?? "?"}\``, rotulo),
    );
  }
  linhas.push(
    linhaDeMetrica(
      "Falhas na amostra",
      dora.amostra ? `${dora.falhas} de ${dora.amostra} (${formatarPct(dora.taxa)})` : "—",
      "runs concluídos deste workflow na mesma branch",
    ),
  );
  linhas.push(
    linhaDeMetrica("MTTR", formatarDuracao(dora.mttrSegundos), "falha → primeiro sucesso seguinte (última recuperação)"),
  );
  linhas.push(
    linhaDeMetrica(
      "Runs verdes (7 dias)",
      dora.amostra ? String(dora.verdes7) : "—",
      "qualquer run concluído com sucesso, não só deploy",
    ),
  );
  linhas.push(
    linhaDeMetrica("Cobertura", coberturaEmTexto(cobertura), "linhas por crate; `—` quando o artefato não existe"),
  );
  linhas.push("");

  if (jobs.pulados.length) {
    linhas.push("**Pulados:**");
    for (const pulado of jobs.pulados) linhas.push(`- ${linhaDePulado(pulado)}`);
    linhas.push("");
  }

  if (jobs.falhas.length) {
    linhas.push("**Falhas:**");
    for (const falha of jobs.falhas) {
      linhas.push(`- ${link(falha.url, falha.nome)} — \`${falha.conclusao}\``);
    }
    linhas.push("");
  }

  if (jobs.todos.length) {
    linhas.push("<details><summary>Job, status, tempo e log (maior primeiro)</summary>");
    linhas.push("");
    linhas.push("| Job | Status | Tempo | Log |");
    linhas.push("|---|---|---|---|");
    for (const job of jobs.todos) linhas.push(linhaDoJob(job));
    linhas.push("");
    linhas.push("</details>");
  }

  return `${linhas.join("\n").trimEnd()}\n`;
}

/** Linha única para o log do job: resume o que foi montado. */
export function linhaDoAviso(modelo) {
  const { tempo, jobs, dora } = modelo;
  const partes = [
    `${jobs.total} jobs`,
    `pipeline ${formatarDuracao(tempo.pipelineSegundos)}`,
    `consumo ${formatarDuracao(tempo.consumoSegundos)}`,
    `${jobs.falha} falha(s)`,
  ];
  if (dora.amostra) partes.push(`amostra ${dora.falhas}/${dora.amostra}`);
  return `${SELO[modelo.veredito]} ${modelo.workflow} ${ESTADO[modelo.veredito]} · ${modelo.run.ref} (${partes.join(", ")})`;
}

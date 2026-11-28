// Render em Markdown do relatório — puro: recebe o modelo, devolve texto.
// É o que vai para `$GITHUB_STEP_SUMMARY`, para o log do job e para o artefato.
//
// O resumo segue o padrão das nossas páginas de documentação: título, subtítulos
// curtos e tabela por assunto — nada de "1 ·", "2 ·". A ordem é a de leitura de
// quem chega no run: Resumo, Tempo, Entrega, Jobs, Ação.
//
// A regra do bloco Jobs é a que mata a duplicidade: antes o mesmo job aparecia em
// "mais lentos", em "falhas" e em "pulados". Agora ele aparece uma vez, na tabela
// do seu domínio, com status, tempo e — quando pulado — o motivo.

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
const SELO_JOB = { success: "✅", skipped: "⏭️" };

function link(url, texto) {
  return url ? `[${texto}](${url})` : texto;
}

function linhaDoCampo(rotulo, valor) {
  return `| ${rotulo} | ${celula(valor)} |`;
}

function linhaDeMetrica(rotulo, valor, significado) {
  return `| ${rotulo} | ${celula(valor)} | ${celula(significado)} |`;
}

function pluralJobs(total) {
  return total === 1 ? "1 job" : `${total} jobs`;
}

function seloDoJob(job) {
  return SELO_JOB[job.conclusao] ?? "❌";
}

/** Linha única do job: status, tempo e — no pulo — o motivo, no lugar do log. */
function linhaDoJob(job) {
  const detalhe =
    job.conclusao === "skipped"
      ? job.motivo
        ? `pulado — \`${job.motivo}\``
        : "pulado"
      : job.url
        ? `[log](${job.url})`
        : "—";
  return `| ${celula(job.nome)} | ${seloDoJob(job)} ${job.conclusao ?? "—"} | ${formatarDuracao(job.segundos)} | ${celula(detalhe)} |`;
}

function cabecalhoDoGrupo(grupo) {
  const partes = [`${pluralJobs(grupo.jobs.length)}`, formatarDuracao(grupo.segundos)];
  if (grupo.ok) partes.push(`✅ ${grupo.ok}`);
  if (grupo.falha) partes.push(`❌ ${grupo.falha}`);
  if (grupo.pulado) partes.push(`⏭️ ${grupo.pulado}`);
  return partes.join(" · ");
}

function escopoDaAmostra(dora) {
  if (dora.amostraEscopo === "padrão" && dora.amostraBranch) {
    return `runs concluídos deste workflow em \`${dora.amostraBranch}\` (esta branch não tem histórico)`;
  }
  return "runs concluídos deste workflow na mesma branch";
}

function coberturaEmTexto(cobertura) {
  if (!cobertura.length) return "—";
  return cobertura.map((item) => `${item.crate} ${formatarPct(item.pct)}`).join(" · ");
}

function acaoEmTexto(jobs, run) {
  if (jobs.falha) {
    const runLink = run.url || run.numero ? ` e o run em ${link(run.url, `#${run.numero ?? "?"}`)}` : "";
    return `❌ **${jobs.falha} job(s) falharam.** Abra o \`log\` do job marcado com ❌ em Jobs${runLink}.`;
  }
  if (jobs.pulado) {
    return `✅ **Nada a fazer.** ${jobs.pulado} job(s) pulado(s) por gatilho, com o motivo na tabela de Jobs.`;
  }
  return `✅ **Nada a fazer.** Todos os ${jobs.ok} jobs passaram.`;
}

/**
 * Markdown completo do run.
 *
 * @param {ReturnType<import("./modelo.mjs").montarModelo>} modelo
 */
export function markdown(modelo) {
  const { run, commit, pr, issue, tempo, jobs, dora, cobertura } = modelo;
  const linhas = [];

  const nome = modelo.workflow ? `${modelo.workflow} — ` : "";
  const referencia = run.ref ? ` · ${run.ref}` : "";
  linhas.push(`### ${SELO[modelo.veredito]} ${nome}${ESTADO[modelo.veredito]}${referencia}`);
  linhas.push("");

  // Resumo
  linhas.push("#### Resumo");
  linhas.push("");
  linhas.push("| Campo | Valor |");
  linhas.push("|---|---|");
  if (run.url || run.numero) {
    const tentativa = run.tentativa > 1 ? ` (tentativa ${run.tentativa})` : "";
    linhas.push(linhaDoCampo("Run", `${link(run.url, `#${run.numero ?? "?"}`)}${tentativa}`));
  }
  linhas.push(linhaDoCampo("Evento", `\`${run.evento}\``));
  if (modelo.autorDoEvento) linhas.push(linhaDoCampo("Ator", `\`${modelo.autorDoEvento}\``));
  // A ref está no título do relatório: repetir como campo era a mesma duplicidade do Slack.
  if (pr) linhas.push(linhaDoCampo("PR", `${link(pr.url, `#${pr.numero}`)}${pr.titulo ? ` — ${pr.titulo}` : ""}`));
  if (issue) {
    const estado = issue.estado ? ` · \`${String(issue.estado).toLowerCase()}\`` : "";
    linhas.push(linhaDoCampo("Issue", `${link(issue.url, `#${issue.numero}`)}${issue.titulo ? ` — ${issue.titulo}` : ""}${estado}`));
  }
  if (commit) {
    linhas.push(
      linhaDoCampo("Commit", `\`${(commit.sha ?? "").slice(0, 7)}\`${commit.assunto ? ` — ${commit.assunto}` : ""}`),
    );
    if (commit.autor) linhas.push(linhaDoCampo("Autor do commit", `\`${commit.autor}\``));
  }
  linhas.push(
    linhaDoCampo("Jobs", `${jobs.total} (✅ ${jobs.ok} · ❌ ${jobs.falha} · ⏭️ ${jobs.pulado})`),
  );
  linhas.push("");

  // Tempo
  linhas.push("#### Tempo");
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
  linhas.push("");

  // Entrega
  linhas.push("#### Entrega");
  linhas.push("");
  linhas.push("| Métrica | Valor | O que conta |");
  linhas.push("|---|---|---|");
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
      escopoDaAmostra(dora),
    ),
  );
  linhas.push(
    linhaDeMetrica("MTTR", formatarDuracao(dora.mttrSegundos), "falha → primeiro sucesso seguinte (última recuperação)"),
  );
  linhas.push(
    linhaDeMetrica(
      "Runs verdes (7 dias)",
      dora.amostra ? String(dora.verdes7) : "—",
      "qualquer run concluído com sucesso na amostra, não só deploy",
    ),
  );
  linhas.push(
    linhaDeMetrica("Cobertura", coberturaEmTexto(cobertura), "linhas por crate; `—` quando o artefato não existe"),
  );
  linhas.push("");

  // Jobs — uma tabela por domínio, cada job uma única vez.
  linhas.push("#### Jobs");
  linhas.push("");
  linhas.push("Cada job aparece uma vez; o job de aviso não entra.");
  linhas.push("");
  for (const grupo of jobs.grupos) {
    linhas.push(`**${celula(grupo.nome)}** — ${cabecalhoDoGrupo(grupo)}`);
    linhas.push("");
    linhas.push("| Job | Status | Tempo | Log |");
    linhas.push("|---|---|---|---|");
    for (const job of grupo.jobs) linhas.push(linhaDoJob(job));
    linhas.push("");
  }

  // Ação
  linhas.push("#### Ação");
  linhas.push("");
  linhas.push(acaoEmTexto(jobs, run));
  linhas.push("");
  linhas.push(`_Gerado em ${modelo.geradoEm.replace("T", " ").slice(0, 16)} UTC._`);

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
  const veredito = `${SELO[modelo.veredito]} ${[modelo.workflow, ESTADO[modelo.veredito]].filter(Boolean).join(" ")}`;
  return `${veredito} · ${modelo.run.ref} (${partes.join(", ")})`;
}

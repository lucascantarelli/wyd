// Payload do Slack — puro: recebe o modelo, devolve o JSON do Block Kit.
//
// Regras que o aviso antigo quebrava e que aqui viram teste:
//
//   1. **quebra de linha de verdade.** O `"*Ref*\n"` em aspas duplas do bash
//      entregava barra invertida + `n`, e o Slack mostrava `*Ref*\n` literal.
//      Aqui o `\n` é newline no JSON, como o Slack espera.
//   2. **escape.** `&`, `<` e `>` são markup do Slack: assunto de commit com `|`
//      dentro de `<url|texto>` também escaparia errado, então título e assunto
//      vão como texto, com link só onde o texto é gerado aqui.
//   3. **limites.** 50 blocos, 3000 caracteres por texto, 150 no título,
//      10 campos por seção. Payload fora do limite é erro, não aviso torto. O
//      bloco de jobs é dividido por tamanho, não por domínio, porque a matriz
//      `esqueleto` cresce e um domínio sozinho já chega perto do teto.
//   4. **título uma vez só.** O `text` da mensagem é o título e o Slack o mostra
//      acima do anexo: por isso não existe bloco `header` com o mesmo texto, e o
//      `text` não repete contagem nem cobertura — antes a mensagem abria com um
//      resumo e logo abaixo o dashboard com os mesmos números.
//   5. **um job por canal.** Cada job aparece uma vez, no seu domínio, com tempo,
//      link do log e — quando pulado — o motivo. Nada de "mais lentos" e
//      "falhas" listando o mesmo job duas vezes.

import { formatarDuracao, formatarPct } from "./texto.mjs";

export const LIMITES = { blocos: 50, texto: 3000, cabecalho: 150, campos: 10, blocoSeguro: 2800 };

const SELO = { sucesso: "✅", falha: "❌" };
const ESTADO = { sucesso: "passou", falha: "falhou" };
const COR = { sucesso: "#2eb886", falha: "#cc0000" };

/** Escapa o que o Slack interpreta como markup. */
export function escapar(texto) {
  return String(texto ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function secao(texto) {
  return { type: "section", text: { type: "mrkdwn", text: texto } };
}

function pluralJobs(total) {
  return total === 1 ? "1 job" : `${total} jobs`;
}

function pctSeguro(valor) {
  return valor === null || valor === undefined ? "—" : formatarPct(valor);
}

function coberturaEmTexto(cobertura) {
  if (!cobertura.length) return "—";
  return cobertura.map((item) => `${item.crate} ${formatarPct(item.pct)}`).join(" · ");
}

function paralelismoEmTexto(valor) {
  return valor === null ? "—" : `${String(valor).replace(".", ",")}×`;
}

/** Cabeçalho do grupo: domínio, quantidade e tempo somado. */
function cabecalhoDoGrupo(grupo) {
  const partes = [`*${grupo.nome}*`, `${pluralJobs(grupo.jobs.length)}`, formatarDuracao(grupo.segundos)];
  if (grupo.falha) partes.push(`${grupo.falha} falha(s)`);
  if (grupo.pulado) partes.push(`${grupo.pulado} pulado(s)`);
  return partes.join(" · ");
}

/**
 * Linha do job: nome, tempo e — só quando foge do normal — a marca. Sucesso não
 * ganha selo: 32 ✅ iguais não informam nada e escondem a falha no meio.
 */
function linhaDoJob(job) {
  const nome = job.url ? `<${job.url}|${escapar(job.nome)}>` : escapar(job.nome);
  if (job.conclusao === "skipped") {
    return `• ${nome} — pulado${job.motivo ? ` por \`${escapar(job.motivo)}\`` : ""}`;
  }
  const marca = job.conclusao === "success" ? "" : ` — ❌ ${job.conclusao}`;
  return `• ${nome} — ${formatarDuracao(job.segundos)}${marca}`;
}

/** Divide o domínio em blocos que cabem nos 3000 caracteres por texto. */
function secoesDoGrupo(grupo) {
  const blocos = [];
  let atual = cabecalhoDoGrupo(grupo);
  for (const job of grupo.jobs) {
    const linha = linhaDoJob(job);
    if (atual.length + 1 + linha.length > LIMITES.blocoSeguro) {
      blocos.push(secao(atual));
      atual = `*${grupo.nome} (continuação)*`;
    }
    atual += `\n${linha}`;
  }
  blocos.push(secao(atual));
  return blocos;
}

function acaoEmTexto(jobs) {
  if (jobs.falha) return `❌ ${jobs.falha} job(s) falharam — abra o log do job marcado com ❌ em Jobs.`;
  if (jobs.pulado) return `✅ Nada a fazer. ${jobs.pulado} job(s) pulado(s) por gatilho, com o motivo em Jobs.`;
  return `✅ Nada a fazer. Todos os ${jobs.ok} jobs passaram.`;
}

/**
 * Título da mensagem — é o `text` do webhook, e é a única linha fora do anexo.
 *
 * Fica curto de propósito: o Slack usa este texto na notificação do celular, na
 * busca e na cópia, e qualquer contagem aqui apareceria de novo dentro do
 * dashboard logo abaixo.
 */
export function textoDoAviso(modelo) {
  const veredito = `${SELO[modelo.veredito]} ${[modelo.workflow, ESTADO[modelo.veredito]].filter(Boolean).join(" ")}`;
  const partes = [veredito];
  if (modelo.run.ref) partes.push(modelo.run.ref);
  return partes.join(" · ").slice(0, LIMITES.cabecalho);
}

/**
 * Payload completo do webhook: título + seções do dashboard.
 *
 * @param {ReturnType<import("./modelo.mjs").montarModelo>} modelo
 */
export function payloadSlack(modelo) {
  const { run, commit, pr, issue, tempo, jobs, dora, cobertura } = modelo;
  const blocos = [];

  // Resumo — identidade do run, sem repetir o título.
  const resumo = ["*Resumo*"];
  const identificacao = [];
  if (run.url || run.numero) identificacao.push(`*Run* <${run.url}|#${run.numero ?? "?"}>`);
  if (run.tentativa > 1) identificacao.push(`tentativa ${run.tentativa}`);
  identificacao.push(`\`${escapar(run.evento)}\``);
  if (modelo.autorDoEvento) identificacao.push(`\`${escapar(modelo.autorDoEvento)}\``);
  resumo.push(identificacao.join(" · "));
  // A ref está no título: repetir aqui era a duplicidade que abria a mensagem duas vezes.
  if (pr) resumo.push(`*PR* <${pr.url}|#${pr.numero}>${pr.titulo ? ` — ${escapar(pr.titulo)}` : ""}`);
  if (issue) {
    const estado = issue.estado ? ` · \`${String(issue.estado).toLowerCase()}\`` : "";
    resumo.push(`*Issue* <${issue.url}|#${issue.numero}>${issue.titulo ? ` — ${escapar(issue.titulo)}` : ""}${estado}`);
  }
  if (commit) {
    const assunto = commit.assunto ? ` — ${escapar(commit.assunto)}` : "";
    const autor = commit.autor ? ` · \`${escapar(commit.autor)}\`` : "";
    resumo.push(`*Commit* <${commit.url}|${escapar((commit.sha ?? "").slice(0, 7))}>${assunto}${autor}`);
  }
  resumo.push(`*Jobs* ${pluralJobs(jobs.total)} · ${jobs.ok} ok · ${jobs.falha} falha(s) · ${jobs.pulado} pulado(s)`);
  blocos.push(secao(resumo.join("\n")));

  // Tempo — duas grandezas que não se somam.
  blocos.push(
    secao(
      [
        "*Tempo*",
        `*Pipeline* ${formatarDuracao(tempo.pipelineSegundos)} · *Consumo de runner* ${formatarDuracao(tempo.consumoSegundos)} · *Paralelismo* ${paralelismoEmTexto(tempo.paralelismo)}`,
        "_Pipeline é o relógio do run; consumo é a soma das durações dos jobs._",
      ].join("\n"),
    ),
  );

  // Entrega — DORA e cobertura, com o N da amostra visível do lado do número.
  const entrega = ["*Entrega*"];
  if (dora.leadSegundos !== null) {
    entrega.push(
      `*Lead time* ${formatarDuracao(dora.leadSegundos)} · \`${dora.leadCommit ?? "?"}\`${dora.leadProxy ? " (proxy: commit do PR → aviso)" : ""}`,
    );
  }
  entrega.push(
    `*Falhas na amostra* ${dora.amostra ? `${dora.falhas} de ${dora.amostra} (${pctSeguro(dora.taxa)})` : "—"} · *MTTR* ${formatarDuracao(dora.mttrSegundos)} · *Verdes 7d* ${dora.amostra ? dora.verdes7 : "—"}`,
  );
  entrega.push(`*Cobertura* ${coberturaEmTexto(cobertura)}`);
  blocos.push(secao(entrega.join("\n")));

  // Jobs — a lista única, por domínio.
  blocos.push({ type: "divider" });
  blocos.push(secao("*Jobs*"));
  for (const grupo of jobs.grupos) blocos.push(...secoesDoGrupo(grupo));
  if (!jobs.grupos.length) blocos.push(secao("—"));

  // Ação — o que fazer, sem nomear job (o nome é do bloco Jobs).
  blocos.push({ type: "divider" });
  blocos.push(secao(`*Ação*\n${acaoEmTexto(jobs)}`));
  blocos.push({
    type: "context",
    elements: [
      {
        type: "mrkdwn",
        text: `${modelo.workflow} · tentativa ${run.tentativa}${dora.amostraEscopo === "padrão" ? ` · amostra: ${dora.amostraBranch}` : ""} · ${modelo.geradoEm.replace("T", " ").slice(0, 16)} UTC`,
      },
    ],
  });
  blocos.push({
    type: "actions",
    elements: [
      {
        type: "button",
        text: { type: "plain_text", text: "Abrir o run", emoji: true },
        url: run.url,
        style: "primary",
      },
    ],
  });

  return {
    text: textoDoAviso(modelo),
    attachments: [{ color: COR[modelo.veredito], blocks: blocos }],
  };
}

/**
 * Valida o payload contra os limites do Block Kit.
 * Lança `Error` — o job precisa falhar ruidosamente, não postar aviso truncado.
 */
export function validarPayload(payload) {
  const blocos = payload?.attachments?.[0]?.blocks;
  if (!Array.isArray(blocos)) throw new Error("payload sem blocos");
  if (blocos.length > LIMITES.blocos) {
    throw new Error(`payload com ${blocos.length} blocos (limite ${LIMITES.blocos})`);
  }

  const conferirTexto = (texto, onde) => {
    if (typeof texto !== "string") throw new Error(`${onde}: texto ausente`);
    if (texto.length > LIMITES.texto) {
      throw new Error(`${onde}: ${texto.length} caracteres (limite ${LIMITES.texto})`);
    }
  };

  blocos.forEach((bloco, indice) => {
    const onde = `bloco ${indice} (${bloco.type})`;
    if (bloco.type === "header") {
      if (bloco.text.text.length > LIMITES.cabecalho) {
        throw new Error(`${onde}: cabeçalho com ${bloco.text.text.length} caracteres (limite ${LIMITES.cabecalho})`);
      }
      return;
    }
    if (bloco.type === "section") {
      if (bloco.fields) {
        if (bloco.fields.length > LIMITES.campos) {
          throw new Error(`${onde}: ${bloco.fields.length} campos (limite ${LIMITES.campos})`);
        }
        bloco.fields.forEach((item, posicao) => conferirTexto(item.text, `${onde}, campo ${posicao}`));
      }
      if (bloco.text) conferirTexto(bloco.text.text, onde);
      return;
    }
    if (bloco.type === "context") {
      bloco.elements.forEach((item, posicao) => conferirTexto(item.text, `${onde}, elemento ${posicao}`));
      return;
    }
    if (bloco.type === "actions") {
      if (!bloco.elements.length) throw new Error(`${onde}: sem botão`);
      bloco.elements.forEach((item, posicao) => {
        if (!item.url) throw new Error(`${onde}, botão ${posicao}: sem url`);
      });
    }
  });

  if (!payload.text) throw new Error("payload sem texto de fallback");
  if (payload.text.length > LIMITES.cabecalho) {
    throw new Error(`título com ${payload.text.length} caracteres (limite ${LIMITES.cabecalho})`);
  }
  return payload;
}

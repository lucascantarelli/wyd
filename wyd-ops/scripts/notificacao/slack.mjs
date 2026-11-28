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
//   3. **limites.** 50 blocos, 3000 caracteres por texto, 150 no cabeçalho,
//      10 campos por seção. Payload fora do limite é erro, não aviso torto.
//   4. **fallback em Unicode.** O `text` da mensagem (o que aparece na
//      notificação do celular e ao copiar) usa ✅ e ⏱, nunca `:white_check_mark:`.
//   5. **um job por canal.** O dashboard repete a estrutura do resumo em cinco
//      seções; cada job aparece uma vez só, na seção 3, com status, tempo, link
//      do log ou o motivo do pulo. Nada de aparecer em "mais lentos" e "falhas"
//      ao mesmo tempo.

import { formatarDuracao, formatarPct } from "./texto.mjs";

export const LIMITES = { blocos: 50, texto: 3000, cabecalho: 150, campos: 10 };

const SELO = { sucesso: "✅", falha: "❌" };
const ESTADO = { sucesso: "passou", falha: "falhou" };
const COR = { sucesso: "#2eb886", falha: "#cc0000" };
const SELO_JOB = { success: "✅", skipped: "⏭️" };

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

function contagemDoGrupo(grupo) {
  const partes = [`${grupo.jobs.length} job(s)`, formatarDuracao(grupo.segundos)];
  if (grupo.ok) partes.push(`✅ ${grupo.ok}`);
  if (grupo.falha) partes.push(`❌ ${grupo.falha}`);
  if (grupo.pulado) partes.push(`⏭️ ${grupo.pulado}`);
  return partes.join(" · ");
}

/** Seção 3: um bloco por domínio, com cada job em uma linha só. */
function textoDoGrupo(grupo) {
  const itens = grupo.jobs.map((job) => {
    const nome = job.url && job.conclusao !== "skipped" ? `<${job.url}|${escapar(job.nome)}>` : escapar(job.nome);
    if (job.conclusao === "skipped") {
      return `• ${nome} — ⏭️ pulado${job.motivo ? ` por \`${escapar(job.motivo)}\`` : ""}`;
    }
    return `• ${nome} — ${formatarDuracao(job.segundos)} ${SELO_JOB[job.conclusao] ?? "❌"}`;
  });
  return `*${grupo.nome}* — ${contagemDoGrupo(grupo)}\n${itens.join("\n")}`;
}

function acaoEmTexto(jobs) {
  if (jobs.falha) return `❌ *${jobs.falha} job(s) falharam* — abra o job marcado com ❌ na seção 3.`;
  if (jobs.pulado) return `✅ *Nada a fazer.* ${jobs.pulado} job(s) pulado(s) por gatilho, com o motivo na seção 3.`;
  return `✅ *Nada a fazer.* Todos os ${jobs.ok} jobs passaram.`;
}

/**
 * Texto de fallback: o que o Slack mostra fora do cliente rico (notificação,
 * cópia, busca). Sem códigos `:emoji:` — o Slack só os converte no render.
 *
 * Este texto é resumo, não dashboard: leva contagem, não o nome de cada job —
 * senão a mesma falha apareceria duas vezes no mesmo canal.
 */
export function textoDoAviso(modelo) {
  const { tempo, jobs, dora, run } = modelo;
  const linhas = [
    `${SELO[modelo.veredito]} ${modelo.workflow} ${ESTADO[modelo.veredito]}${run.ref ? ` · ${run.ref}` : ""}`,
    `Run #${run.numero ?? "?"} (tentativa ${run.tentativa}) · ${run.evento}${modelo.pr ? ` · PR #${modelo.pr.numero}` : ""}`,
    `Jobs ${jobs.total} (✅ ${jobs.ok} · ❌ ${jobs.falha} · ⏭️ ${jobs.pulado}) · tempo de pipeline ${formatarDuracao(tempo.pipelineSegundos)} · consumo de runner ${formatarDuracao(tempo.consumoSegundos)}`,
  ];
  if (dora.amostra) {
    linhas.push(
      `Falhas na amostra ${dora.falhas}/${dora.amostra} (${pctSeguro(dora.taxa)}) · MTTR ${formatarDuracao(dora.mttrSegundos)} · verdes 7d ${dora.verdes7}`,
    );
  }
  if (modelo.cobertura.length) linhas.push(`Cobertura ${coberturaEmTexto(modelo.cobertura)}`);
  return linhas.join("\n");
}

/**
 * Payload completo do webhook: dashboard em cinco seções.
 *
 * @param {ReturnType<import("./modelo.mjs").montarModelo>} modelo
 */
export function payloadSlack(modelo) {
  const { run, commit, pr, issue, tempo, jobs, dora, cobertura } = modelo;
  const titulo = `${SELO[modelo.veredito]} ${modelo.workflow} ${ESTADO[modelo.veredito]}${run.ref ? ` · ${run.ref}` : ""}`.slice(
    0,
    LIMITES.cabecalho,
  );

  // 1 · Resultado — identidade do run, sem nome de job.
  const resultado = [];
  if (pr) {
    resultado.push(`🔗 *PR* <${pr.url}|#${pr.numero}>${pr.titulo ? ` — ${escapar(pr.titulo)}` : ""}`);
  }
  if (issue) {
    const estado = issue.estado ? ` · \`${String(issue.estado).toLowerCase()}\`` : "";
    resultado.push(
      `🎯 *Issue* <${issue.url}|#${issue.numero}>${issue.titulo ? ` — ${escapar(issue.titulo)}` : ""}${estado}`,
    );
  }
  if (commit) {
    const assunto = commit.assunto ? ` — ${escapar(commit.assunto)}` : "";
    const autor = commit.autor ? ` · \`${escapar(commit.autor)}\`` : "";
    resultado.push(`📌 *Commit* <${commit.url}|${escapar((commit.sha ?? "").slice(0, 7))}>${assunto}${autor}`);
  }
  resultado.push(
    `🌿 *Ref* \`${escapar(run.ref)}\` · *Evento* \`${escapar(run.evento)}\`${modelo.autorDoEvento ? ` · *Ator* \`${escapar(modelo.autorDoEvento)}\`` : ""}`,
  );
  resultado.push(`🧩 *Jobs* ${jobs.total} · ✅ ${jobs.ok} · ❌ ${jobs.falha} · ⏭️ ${jobs.pulado}`);

  const blocos = [
    { type: "header", text: { type: "plain_text", text: titulo, emoji: true } },
    secao(`*1 · Resultado*\n${resultado.join("\n")}`),
    secao(
      [
        "*2 · Tempo*",
        `⏱ *Tempo de pipeline* ${formatarDuracao(tempo.pipelineSegundos)} · 🔋 *Consumo de runner* ${formatarDuracao(tempo.consumoSegundos)} · ⚡ *Paralelismo* ${paralelismoEmTexto(tempo.paralelismo)}`,
        "_Pipeline é o relógio do run; consumo é a soma das durações dos jobs. Não são a mesma grandeza._",
      ].join("\n"),
    ),
    {
      type: "context",
      elements: [
        {
          type: "mrkdwn",
          text: "*3 · Jobs por domínio* — cada job aparece uma vez; o job de aviso não entra, porque é ele que publica este aviso.",
        },
      ],
    },
  ];

  for (const grupo of jobs.grupos) blocos.push(secao(textoDoGrupo(grupo)));
  if (!jobs.grupos.length) blocos.push(secao("*3 · Jobs por domínio*\n—"));

  // 4 · Entrega — DORA e cobertura, sem repetir job.
  const entrega = ["*4 · Entrega (DORA)*"];
  if (dora.leadSegundos !== null) {
    entrega.push(
      `🧭 *Lead time* ${formatarDuracao(dora.leadSegundos)} · \`${dora.leadCommit ?? "?"}\`${dora.leadProxy ? " _(proxy: commit do PR → aviso)_" : ""}`,
    );
  }
  entrega.push(
    `📉 *Falhas na amostra* ${dora.amostra ? `${dora.falhas} de ${dora.amostra} (${pctSeguro(dora.taxa)})` : "—"} · 🛠 *MTTR* ${formatarDuracao(dora.mttrSegundos)} · 🚀 *Runs verdes (7d)* ${dora.amostra ? dora.verdes7 : "—"}`,
  );
  entrega.push(`🧮 *Cobertura* ${coberturaEmTexto(cobertura)}`);

  blocos.push({ type: "divider" });
  blocos.push(secao(entrega.join("\n")));

  // 5 · Ação — o que fazer, sem nomear job (a seção 3 é a dona do nome).
  blocos.push(secao(`*5 · Ação*\n${acaoEmTexto(jobs)}`));
  blocos.push({
    type: "context",
    elements: [
      {
        type: "mrkdwn",
        text: `Run #${run.numero ?? "?"} · tentativa ${run.tentativa} · ${modelo.workflow}${dora.amostraEscopo === "padrão" ? ` · amostra: ${dora.amostraBranch}` : ""} · ${modelo.geradoEm.replace("T", " ").slice(0, 16)} UTC`,
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
  return payload;
}

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

import { formatarDuracao, formatarPct } from "./texto.mjs";

export const LIMITES = { blocos: 50, texto: 3000, cabecalho: 150, campos: 10 };

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

function campo(rotulo, valor) {
  return { type: "mrkdwn", text: `*${rotulo}*\n${valor}` };
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

/**
 * Texto de fallback: o que o Slack mostra fora do cliente rico (notificação,
 * cópia, busca). Sem códigos `:emoji:` — o Slack só os converte no render.
 */
export function textoDoAviso(modelo) {
  const { tempo, jobs, dora, run } = modelo;
  const linhas = [
    `${SELO[modelo.veredito]} ${modelo.workflow} ${ESTADO[modelo.veredito]}${run.ref ? ` · ${run.ref}` : ""}`,
    `Run #${run.numero ?? "?"} (tentativa ${run.tentativa}) · ${run.evento}${modelo.pr ? ` · PR #${modelo.pr.numero}` : ""}`,
    `Tempo de pipeline ${formatarDuracao(tempo.pipelineSegundos)} · consumo de runner ${formatarDuracao(tempo.consumoSegundos)} (${jobs.total} jobs)`,
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
 * Payload completo do webhook.
 *
 * @param {ReturnType<import("./modelo.mjs").montarModelo>} modelo
 */
export function payloadSlack(modelo) {
  const { run, commit, pr, issue, tempo, jobs, dora, cobertura } = modelo;
  const titulo = `${SELO[modelo.veredito]} ${modelo.workflow} ${ESTADO[modelo.veredito]}${run.ref ? ` · ${run.ref}` : ""}`.slice(
    0,
    LIMITES.cabecalho,
  );

  const blocos = [
    { type: "header", text: { type: "plain_text", text: titulo, emoji: true } },
    {
      type: "section",
      fields: [
        campo("Ref", `\`${escapar(run.ref)}\``),
        campo("Commit", commit ? `<${commit.url}|${escapar((commit.sha ?? "").slice(0, 7))}>` : "—"),
        campo("Evento", `\`${escapar(run.evento)}\``),
        campo("Ator", modelo.autorDoEvento ? `\`${escapar(modelo.autorDoEvento)}\`` : "—"),
      ],
    },
    {
      type: "section",
      fields: [
        campo("⏱ Tempo de pipeline", formatarDuracao(tempo.pipelineSegundos)),
        campo("🔋 Consumo de runner", formatarDuracao(tempo.consumoSegundos)),
        campo(
          "⚡ Paralelismo",
          tempo.paralelismo === null ? "—" : `${String(tempo.paralelismo).replace(".", ",")}×`,
        ),
        campo("🧩 Jobs", `${jobs.total} · ✅ ${jobs.ok} · ❌ ${jobs.falha} · ⏭️ ${jobs.pulado}`),
      ],
    },
    {
      type: "context",
      elements: [
        {
          type: "mrkdwn",
          text: `ℹ️ Tempo de pipeline é o relógio do run; consumo de runner é a soma das durações dos ${jobs.total} jobs (sem o job de aviso). Não são a mesma grandeza.`,
        },
      ],
    },
  ];

  const vinculos = [];
  if (pr) {
    vinculos.push(`🔗 *PR* <${pr.url}|#${pr.numero}>${pr.titulo ? ` — ${escapar(pr.titulo)}` : ""}`);
  }
  if (issue) {
    const estado = issue.estado ? ` · \`${String(issue.estado).toLowerCase()}\`` : "";
    vinculos.push(
      `🎯 *Issue* <${issue.url}|#${issue.numero}>${issue.titulo ? ` — ${escapar(issue.titulo)}` : ""}${estado}`,
    );
  }
  if (vinculos.length) blocos.push(secao(vinculos.join("\n")));

  if (commit?.assunto && commit.autor) {
    blocos.push(secao(`🗒 *Commit* de \`${escapar(commit.autor)}\`\n${escapar(commit.assunto)}`));
  } else if (commit?.assunto) {
    blocos.push(secao(`🗒 *Assunto do commit*\n${escapar(commit.assunto)}`));
  }

  blocos.push({ type: "divider" });

  const lentos = jobs.lentos.length
    ? jobs.lentos
        .map((job) =>
          job.url ? `• <${job.url}|${escapar(job.nome)}> — ${formatarDuracao(job.segundos)}` : `• ${escapar(job.nome)} — ${formatarDuracao(job.segundos)}`,
        )
        .join("\n")
    : "—";
  blocos.push(secao(`🐢 *Mais lentos*\n${lentos}`));

  if (jobs.falhas.length) {
    const falhas = jobs.falhas
      .map((falha) => `• <${falha.url}|${escapar(falha.nome)}> — \`${falha.conclusao}\``)
      .join("\n");
    blocos.push(secao(`❌ *Falhas*\n${falhas}`));
  }

  if (jobs.pulados.length) {
    const pulados = jobs.pulados
      .map((pulado) => (pulado.motivo ? `• ${escapar(pulado.nome)} — pulado por \`${escapar(pulado.motivo)}\`` : `• ${escapar(pulado.nome)} — pulado`))
      .join("\n");
    blocos.push(secao(`⏭️ *Pulados*\n${pulados}`));
  }

  const camposDora = [];
  if (dora.leadSegundos !== null) {
    camposDora.push(
      campo(
        "🧭 Lead time",
        `${formatarDuracao(dora.leadSegundos)} · \`${dora.leadCommit ?? "?"}\`${dora.leadProxy ? "\n_proxy: commit do PR → aviso_" : ""}`,
      ),
    );
  }
  camposDora.push(campo("📉 Falhas na amostra", dora.amostra ? `${dora.falhas} de ${dora.amostra} (${pctSeguro(dora.taxa)})` : "—"));
  camposDora.push(campo("🛠 MTTR", formatarDuracao(dora.mttrSegundos)));
  camposDora.push(campo("🚀 Runs verdes (7d)", dora.amostra ? String(dora.verdes7) : "—"));

  blocos.push({ type: "divider" });
  blocos.push({ type: "section", fields: camposDora });
  blocos.push({
    type: "context",
    elements: [
      {
        type: "mrkdwn",
        text: `Run #${run.numero ?? "?"} · tentativa ${run.tentativa} · ${modelo.workflow} · cobertura: ${coberturaEmTexto(cobertura)} · ${modelo.geradoEm.replace("T", " ").slice(0, 16)} UTC`,
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

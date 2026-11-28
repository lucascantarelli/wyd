// Motivo de um job pulado, lido do próprio workflow.
//
// A API do GitHub diz que o job foi `skipped`, não por quê. O `if:` está no
// arquivo do workflow, então o motivo sai de uma varredura por indentação: o
// bloco do job começa na chave de dois espaços e o `name:` do job fica em quatro.
// É leitura de uma linha só — nada de parser de YAML nem de dependência nova.

/** Tira aspas só quando elas envolvem o valor inteiro (`'x'`), não as do meio. */
function semAspas(bruto) {
  const texto = String(bruto ?? "").trim();
  const envolvido =
    texto.length >= 2 &&
    ((texto.startsWith("'") && texto.endsWith("'")) || (texto.startsWith('"') && texto.endsWith('"')));
  return envolvido ? texto.slice(1, -1) : texto;
}

/**
 * Devolve a expressão `if:` do job cujo `name:` bate, ou `null`.
 *
 * @param {string|null} yaml conteúdo do arquivo do workflow
 * @param {string} nome nome exibido do job (`jobs.<id>.name`)
 */
export function motivoDoJob(yaml, nome) {
  if (!yaml || !nome) return null;
  const linhas = yaml.split(/\r?\n/);
  const alvo = String(nome).trim();

  for (let i = 0; i < linhas.length; i += 1) {
    const nomeDoJob = /^\s{4}name:\s*(.+?)\s*$/.exec(linhas[i]);
    if (!nomeDoJob) continue;
    if (semAspas(nomeDoJob[1]) !== alvo) continue;

    for (let j = i; j < linhas.length; j += 1) {
      // Chave de dois espaços abre o próximo job: o bloco deste terminou.
      if (j > i && /^\s{2}\S/.test(linhas[j])) break;
      const condicao = /^\s{4,6}if:\s*(.+?)\s*$/.exec(linhas[j]);
      if (condicao) return semAspas(condicao[1]);
    }
    return null;
  }

  return null;
}

/** Aplica `motivoDoJob` a uma lista de nomes, devolvendo `Map nome → motivo`. */
export function motivosPulados(yaml, nomes) {
  const mapa = new Map();
  for (const nome of nomes ?? []) mapa.set(nome, motivoDoJob(yaml, nome));
  return mapa;
}

#!/usr/bin/env node
// Limpeza das contas de QA acumuladas no banco de testes (política: docs/rust/05-stack-docker.md).
//
// O que conta como "acumulada": conta cujo login termina em **5+ dígitos** — o padrão que os
// testes vivos usam para o sufixo aleatório (`qasenha12345678`, `qapin90000000`, `cr5249676200000`,
// `sc…`, `pp…`). Contas de fixture documentadas e as fixas da suíte (que não terminam em dígitos)
// ficam: `qapinraw0001`, `qapin86895000`, `qabanido0001`, `qanivel01`, `cenamob1`, além de
// `qasenhalive`, `qapinlive`, `qapainellive`, `qacrialive`, `qabordalive`, `qapinprobe`.
//
// A remoção é uma transação única no Postgres do compose:
//   1. filhos com `ON DELETE RESTRICT` em `accounts` (`delivery_queue`, `donate_credit`,
//      `donate_debit`) e `guilds` cujo mestre é personagem de conta alvo (FK sem cascade);
//   2. `character_session` das contas alvo (histórico por login, sem FK para `accounts`);
//   3. `DELETE FROM accounts` — o resto cai por `ON DELETE CASCADE` (characters, itens, cofre,
//      moderação, carteira, diária, sessões com `character_id`).
//
// Uso:
//   node tools/dev/limpar-qa.mjs              # dry-run: conta e amostra, sem remover
//   node tools/dev/limpar-qa.mjs --aplicar    # remove numa transação
//   make db-limpa-qa ARGS=--aplicar
//
// Não apaga dados de contas fora do padrão, não toca em migrations e não derruba volume.
import { spawnSync } from "node:child_process";

const CONTAINER = process.env.WYD_DB_CONTAINER ?? "wyd-db-1";
const USUARIO = process.env.WYD_DB_USER ?? "admin";
const BANCO = process.env.WYD_DB_NAME ?? "wyd";
const aplicar = process.argv.includes("--aplicar");

/** Contas que a limpeza nunca remove (fixtures de documentação e da suíte viva). */
const PRESERVAR = [
  "qapinraw0001",
  "qapin86895000",
  "qabanido0001",
  "qanivel01",
  "cenamob1",
  "qasenhalive",
  "qapinlive",
  "qapainellive",
  "qacrialive",
  "qabordalive",
  "qapinprobe",
];

const listaPreservar = PRESERVAR.map((login) => `'${login}'`).join(", ");

/** Tabela temporária com as contas alvo; some no fim da transação. */
const ALVO = `
CREATE TEMP TABLE alvo ON COMMIT DROP AS
SELECT id, login FROM accounts
WHERE login ~ '[0-9]{5,}$'
  AND login <> ALL (ARRAY[${listaPreservar}]);
`;

const CONTAGENS = `
SELECT 'contas', count(*)::text FROM alvo
UNION ALL SELECT 'personagens', count(*)::text FROM characters WHERE account_id IN (SELECT id FROM alvo)
UNION ALL SELECT 'personagens_vivos', count(*)::text FROM characters WHERE account_id IN (SELECT id FROM alvo) AND deleted_at IS NULL
UNION ALL SELECT 'sessoes', count(*)::text FROM character_session WHERE account_login IN (SELECT login FROM alvo)
UNION ALL SELECT 'sessoes_vivas', count(*)::text FROM character_session WHERE released_at IS NULL AND account_login IN (SELECT login FROM alvo)
UNION ALL SELECT 'fila_entrega', count(*)::text FROM delivery_queue WHERE account_id IN (SELECT id FROM alvo)
UNION ALL SELECT 'creditos', count(*)::text FROM donate_credit WHERE account_id IN (SELECT id FROM alvo)
UNION ALL SELECT 'debitos', count(*)::text FROM donate_debit WHERE account_id IN (SELECT id FROM alvo)
UNION ALL SELECT 'guildas', count(*)::text FROM guilds WHERE master_character_id IN (SELECT id FROM characters WHERE account_id IN (SELECT id FROM alvo))
ORDER BY 1;
`;

const REMOCAO = `
DELETE FROM delivery_queue WHERE account_id IN (SELECT id FROM alvo);
DELETE FROM donate_credit WHERE account_id IN (SELECT id FROM alvo);
DELETE FROM donate_debit WHERE account_id IN (SELECT id FROM alvo);
DELETE FROM guilds WHERE master_character_id IN
  (SELECT id FROM characters WHERE account_id IN (SELECT id FROM alvo));
DELETE FROM character_session WHERE account_login IN (SELECT login FROM alvo);
DELETE FROM accounts WHERE id IN (SELECT id FROM alvo);
`;

const AMOSTRA = `
SELECT login FROM alvo ORDER BY login LIMIT 12;
`;

/** Roda SQL no Postgres do compose; devolve as linhas (modo `-At`). */
function psql(sql) {
  const resultado = spawnSync(
    "docker",
    [
      "exec",
      "-i",
      CONTAINER,
      "psql",
      "-U",
      USUARIO,
      "-d",
      BANCO,
      "-v",
      "ON_ERROR_STOP=1",
      "-q",
      "-At",
      "-F",
      "\t",
    ],
    { input: sql, encoding: "utf8" },
  );
  if (resultado.error) {
    console.error(`erro ao chamar docker: ${resultado.error.message}`);
    process.exit(2);
  }
  if (resultado.status !== 0) {
    console.error(resultado.stderr.trim() || `psql saiu com ${resultado.status}`);
    process.exit(resultado.status ?? 1);
  }
  return resultado.stdout.trim();
}

function contagens(saida) {
  const mapa = new Map();
  for (const linha of saida.split("\n").filter(Boolean)) {
    const [chave, valor] = linha.split("\t");
    // `-q` não silencia a tag `SELECT n`; só linhas `chave\tvalor` interessam.
    if (!valor || !/^[0-9]+$/.test(valor)) continue;
    mapa.set(chave, Number(valor));
  }
  return mapa;
}

const antes = contagens(psql(`BEGIN;\n${ALVO}\n${CONTAGENS}\nROLLBACK;`));
const amostra = psql(`BEGIN;\n${ALVO}\n${AMOSTRA}\nROLLBACK;`)
  .split("\n")
  .filter((linha) => /^[a-z0-9_]+$/.test(linha));

if ((antes.get("contas") ?? 0) === 0) {
  console.log("qa: nenhuma conta acumulada no padrão (login terminando em 5+ dígitos)");
  process.exit(0);
}

console.log(
  `${aplicar ? "removendo" : "removeria"}: ${antes.get("contas")} conta(s), ` +
    `${antes.get("personagens")} personagem(ns) (${antes.get("personagens_vivos")} vivos), ` +
    `${antes.get("sessoes")} sessão(ões) (${antes.get("sessoes_vivas")} vivas)`,
);
console.log(
  `também: ${antes.get("fila_entrega")} linha(s) de delivery_queue, ` +
    `${antes.get("creditos")} crédito(s), ${antes.get("debitos")} débito(s), ` +
    `${antes.get("guildas")} guilda(s) com mestre em conta alvo`,
);
console.log(`amostra: ${amostra.join(", ")}`);
console.log(`preservadas: ${PRESERVAR.join(", ")}`);

if (!aplicar) {
  console.log("dry-run: use --aplicar para remover");
  process.exit(0);
}

psql(`BEGIN;\n${ALVO}\n${REMOCAO}\nCOMMIT;`);
const depois = contagens(
  psql(`
SELECT 'accounts', count(*)::text FROM accounts
UNION ALL SELECT 'personagens_vivos', count(*)::text FROM characters WHERE deleted_at IS NULL
UNION ALL SELECT 'sessoes_vivas', count(*)::text FROM character_session WHERE released_at IS NULL
ORDER BY 1;`),
);
console.log(
  `qa: ${antes.get("contas")} conta(s) removida(s) | agora accounts=${depois.get("accounts")} ` +
    `personagens_vivos=${depois.get("personagens_vivos")} ` +
    `sessoes_vivas=${depois.get("sessoes_vivas")}`,
);

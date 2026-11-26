> Pergunta: "que regras valem para todo teste de sistema, do preparo do banco ao isolamento?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../../docs/guia-teste/testes-de-sistema.md`; `../specs/capacidades.md` §4–§6; `../fixtures-e-factories.md` §4
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — regras escritas; **1 de 3** alvos de sistema implementado (`sistema_pg`, 13 cenários, 2026-11-24).

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre a suíte: as 18 regras valem para três alvos que ainda não
  existem em `crates/wyd-db/Cargo.toml`.
- **`H`** — dúvida aberta, dita e localizada em §6, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

Regras de `sistema_pg`, `sistema_sessoes` e `sistema_stack_caps`. O guia diz por quê; esta página fixa o
que a suíte faz em cada caso, e o que acontece quando alguém desvia.

**Alvos escritos onde roda:** `sistema_pg` (`CAP-09`–`CAP-19` e `CAP-24`–`CAP-39`), `sistema_sessoes` (`CAP-20`–`CAP-21`) e
`sistema_stack_caps` (`CAP-22`–`CAP-23`) — os três que abrem conexão ([../index.md](../index.md) §2).

## 1. Preparo do banco

| # | Regra | Violação aparece como |
|---|---|---|
| 1 | PostgreSQL **17**, mesma major da imagem de produção | comportamento diferente do ambiente real |
| 2 | banco vazio → **provisionamento real** → série real | teste verde sobre schema que não existe |
| 3 | banco descartável, criado e derrubado pela execução | estado herdado de execução anterior |
| 4 | nada de `seeds/dev.sql` | teste que só passa em máquina de desenvolvimento |
| 5 | o ledger é o do runner, nunca escrito à mão | drift mascarado |

## 2. DSN e ambiente

| # | Regra | Violação aparece como |
|---|---|---|
| 6 | `WYD_DB_URL_TEST` é a DSN do alvo (`wyd_migrator`) | papel errado esconde `permission denied` |
| 7 | sem a variável, **falha nomeando** `WYD_DB_URL_TEST` | `skipped` verde no CI |
| 8 | teste de fronteira usa a DSN **do papel testado** | fronteira não exercitada |
| 9 | nenhum binário lê `DATABASE_URL` | configuração fora da DSN por papel |

## 3. Isolamento e limpeza

| # | Regra | Violação aparece como |
|---|---|---|
| 10 | cada teste usa **chave própria** e limpa só o que criou | `TRUNCATE` global que esconde dependência |
| 11 | limpeza roda também na falha | órfão que faz o teste seguinte passar |
| 12 | filho antes do pai na ordem de `DELETE` | `DELETE` falhando por FK e virando verde falso |
| 13 | uma base por binário; paralelismo dentro do binário é por chave | corrida entre alvos |
| 14 | espera por **estado**, nunca `sleep` | teste lento e instável que esconde corrida |

## 4. Dado
**Sem arquivo no repositório:** `resources/accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| # | Regra | Violação aparece como |
|---|---|---|
| 15 | dado criado pela factory, com o id devolvido | id mágico (`1`, `42`, `7662`) |
| 16 | catálogo vem da série aplicada, não de `INSERT` de teste | fixture paralela ao seed |
| 17 | conta de teste vem de `resources/accounts.json`, como ficha | conta "que já existe" no banco |
| 18 | nenhum segredo real em código, log ou snapshot de falha | vazamento em artefato de CI |

## 5. O que o alvo de sistema não pode fazer

- **Passar sem rodar:** `#[ignore]`, `SKIP`, filtro silencioso ou `return` antecipado são falha
  (`CAP-23`).
- **Aceitar erro genérico:** o esperado é o erro nomeado da capacidade, com `SQLSTATE` quando existir.
- **Mascarar a ausência de barreira:** onde a regra diz "não há barreira" (`CAP-14`), sucesso é o
  resultado esperado e falha é constraint a mais.
- **Testar regra de jogo:** `RF-DB-015`/`RF-DB-016` são de `wyd-server`.

## 6. Dúvidas abertas
**Sem arquivo no repositório:** `db.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Dúvida | O que a resolve | Dono |
|---|---|---|
| a regra 13 (uma base por binário) basta para dois binários em paralelo? | não há medição; a primeira execução do `sistema_pg` responde, e a regra muda se colidir | autor do alvo |
| quem derruba o banco quando o processo morre no meio da execução? | `db.rs` precisa de rotina de recuperação; não há regra escrita | autor do alvo |

## 7. O que esta página não decide

- **Não lista capacidades nem cenários:** `../specs/capacidades.md` e `../alvos/`.
- **Não define a política de teste:** `../../docs/planos/plano-de-testes.md` e `../../docs/guia-teste/`.
- **Não decide CI:** não existe página de CI em `docs/planos/` do `wyd-db`; o plano do projeto é
  `../../../wyd-ops/docs/devops/index.md` (`aceito`, 2026-11-26), e `docs/planos/plano-de-testes.md` §81
  anuncia a página local para depois do primeiro código.
- **Não mede tempo:** nenhum alvo roda; tempo esperado entra com execução registrada.

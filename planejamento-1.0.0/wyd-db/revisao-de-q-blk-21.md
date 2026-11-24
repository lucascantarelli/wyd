> Pergunta: "o dono pode assinar as 12 páginas de `tests/`, e o que exatamente ele assina em cada uma?"
> Público: dono do `wyd-db` (quem assina), QA e quem revisa a leva de aceite
> Dono: equipe do `wyd-db`; **a assinatura é do dono**
> Base: `tests/` (12 páginas); `pendencias-de-fechamento.md` §6.6.23 e §6.6.25; `wyd-db-1.0.0.md` §10, item 9; medição de 2026-11-24
> Revisado em: 2026-11-24

**Estado:** cumprida (2026-11-24) — o dono aprovou `Q-BLK-21` em **A**, as correções de §4 foram aplicadas
antes da assinatura e as 12 páginas estão `aceito (2026-11-24)`. O registro está em §5.

## 1. O que a assinatura é — e o que ela não é

- **É o item 9 de §10**, "revisada por quem não escreveu". `Q-BLK-21` = A: o dono revisa e assina, e o
  registro é nome e data no lote. Esta folha é o instrumento da revisão, não a assinatura.
- **Não é prova de execução.** Os cinco alvos não existem (`crates/wyd-db/` só tem `.gitkeep`, sem
  `Cargo.toml`), `0` de `23` capacidades
  está coberta e `0` arquivo `.sql` foi escrito. Assinar diz que o **plano** está certo, não que a suíte roda.
- **Não autoriza código por si.** `Q-BLK-18` — o aceite libera o código ou ele espera a issue no GitHub? —
  segue **aberta**. Sem ela, nem a assinatura nem o aceite liberam os `.rs`.
- **Não fecha as dúvidas `H`** de cada página. O critério 4 admite `H` declarada e localizada; assinar uma
  página com dúvida é afirmar que a dúvida está no lugar certo e tem dono.

**Medido nesta passada (2026-11-24), com o comando ao lado:**

| Medida | Valor | Como |
|---|---:|---|
| páginas em `tests/` | **12** | `find tests -name '*.md'` |
| páginas com `**Estado:** proposto` / `aceito` | **12** / **0** **no pré-aceite**; hoje **0** / **12** (§5) | `grep -h '^\*\*Estado:'` |
| alvos em `crates/wyd-db/Cargo.toml` | **0 de 5** | `find crates -type f` — só `.gitkeep`, nenhum `Cargo.toml` |
| capacidades na matriz / cobertas | **23** / **0** | 23 linhas `CAP-` no arquivo, mais o cabeçalho |
| linhas da rastreabilidade (com página / sem / sem cenário) | **34** / **15** / **19** / **16** | `docs/specs/rastreabilidade.md` §1 e §3.4 |
| arquivos `.sql` da série | **0** | `find . -name '*.sql'` |
| arquivos em `tests/resources/` | **1** (só este README) | `ls tests/resources` |
| páginas autorais em `docs/` | **143** (teto 144) | `find docs -name '*.md' -not -path '*/gerado/*' -not -name 'proposta-*'` |

## 2. A folha, página por página

A tabela registra o veredito de revisão de 2026-11-24; a assinatura foi dada em seguida (§5), depois das
correções de §4. Nenhuma página foi promovida por esta tabela.

| # | Página | O que ela afirma | Evidência real que existe | O que **não** tem prova demonstrável | Veredito |
|---:|---|---|---|---|---|
| 1 | `specs/capacidades.md` | matriz de 23 `CAP`, 0 cobertas, 12 com requisito nomeado, 11 governadas por gate | 23 linhas `CAP-` no próprio arquivo; `plano-migrations.md` §7 (gates); `rastreabilidade.md` §2 (34 linhas) | nada além do que a página já declara: "coberta" é 0 por ausência de suíte | **assinar** — matriz declarada, não promessa de verde |
| 2 | `alvos/contrato-migrations.md` | 34 arquivos, 7 `CAP`, 7 cenários e 8 fixtures | `plano-migrations.md` §1 (34 arquivos, 109 tabelas); `migrations/` tem 29 páginas; `migrations-checklist.md` | **0 `.sql`** e **0 fixtures**: a "série viva" e `resources/serie/` não existem. A frase "a contagem é conferida contra a série viva e contra a fixture" está no presente sem objeto | **assinar após correção de redação** — "é conferida" → "será conferida"; os 7 cenários só rodam depois da leva de `migrations/` |
| 3 | `alvos/contrato-catalogo.md` | `CAP-08` compara catálogo × série: 109 tabelas e 2 views | `plano-migrations.md` §7 (gate `catálogo`); `ddl-confirmado.md` §1 (109); dicionário com 110 arquivos (109 + índice) | `docs/dicionario-dados/banco-catalogo.md` não existe; o leitor atual não enxerga `CREATE TABLE esquema.tabela`; **e `Q-BLK-20`** (declarar `CAP-08` pós-`1.0.0`?) está aberta | **não assinar ainda** — assinar fixa a página antes da decisão que a governa; se `Q-BLK-20` = A, assina-se como não executável |
| 4 | `alvos/sistema-pg.md` | 11 `CAP` e 11 cenários contra PG17 real | DDL das páginas da série: `0005` `ck_account_slot_range`; `0001` §9 prova 9 (`ck_content_version_linha_unica`, `id = 2` → `23514`); `0007` `ux_session_viva_por_conta`; `0011` `uq_daily_reward_claim_dia`; `0006` `fk_character_content_version`; `RNF-DB-006/007` e `RF-DB-017` | nenhum cenário rodou; sem `.sql`, "contra PG17 real" é plano | **assinar** — é o alvo que mais se apoia em DDL já fechado |
| 5 | `alvos/sistema-sessoes.md` | `CAP-20/21`: fronteira por papel, leitura por view, auditoria append-only | `seguranca.md` §"Papéis do banco" (6 papéis); `implantacao.md` §1, passo 2; `0029-views.md`; `0032-admin-grants.md` | privilégios não aplicados (0 `.sql`); §6 pergunta quem concede a auditoria (provisionamento ou `0032`) | **assinar** — a dúvida é `H` declarada, do autor do alvo |
| 6 | `alvos/sistema-stack-caps.md` | `CAP-22/23` e a conferência da matriz (23) | matriz com 23 linhas; o CI de código é stub (`AGENTS.md` §4) | nada executa; a forma da conferência (teste ou script) não está decidida (§7) | **assinar** |
| 7 | `fixtures-e-factories.md` | 4 arquivos em `common/`, 6 factories, limpeza por chave | `src/migrate/spec.md` §2 (parser único do runner) | `tests/common/` não existe; a frase "Esta página é o **inventário real**" contradiz o estado "nenhum dos quatro arquivos foi escrito"; 2 dúvidas abertas | **assinar após correção de redação** — "inventário real" → "inventário planejado" |
| 8 | `dados-de-teste.md` | 5 contas, vagas 1–5, 6ª tentativa recusada | `ck_account_slot_range` (`0005`, `ddl-confirmado.md`); `person.role` fechado; papéis do provisionamento | nenhuma conta criada; "senha ... declarada em `resources/accounts.json`" aponta arquivo ausente; **contradição interna**: §2 diz "a sexta na vaga já fechada", a tabela de §2 diz vaga 6 | **não assinar ainda** — corrigir a frase da senha e dizer qual leitura o cenário usa |
| 9 | `resources/README.md` | formato de `accounts.json` e 8 fixtures de série | `testes-de-contrato.md` §1 (`--dir`); guia de dados de teste | nenhum arquivo existe; "a senha ... está escrita aqui como fictícia" não tem valor em lugar nenhum | **assinar após correção de redação** — "está escrita" → "será escrita em `accounts.json`" |
| 10 | `specs/contrato.md` | 8 regras para os 2 alvos de contrato | `testes-de-contrato.md`; `src/migrate/spec.md` §2 | nenhuma implementação; 2 dúvidas (formato da mensagem; escolha da fixture) | **assinar** |
| 11 | `specs/sistema.md` | 18 regras para os 3 alvos de sistema | `testes-de-sistema.md`; DDL das migrations | nenhum alvo; a regra 17 depende de `accounts.json`, que não existe | **assinar** — regra é norma; o recurso é pendência declarada |
| 12 | `index.md` | árvore, 5 alvos, comandos, porta das 12 páginas | 12 arquivos em `tests/` (medido); matriz; `wyd-db-1.0.0.md` §6.17 | os comandos `cargo test` nunca rodaram (não há crate); §5 aponta `Q-BLK-17/18` abertas — a `17` fechou pela execução (a própria leva) e a `18` segue aberta | **assinar** |

**Contagem dos vereditos:** **7** prontas (`1`, `4`, `5`, `6`, `10`, `11`, `12`), **3** após correção de
redação (`2`, `7`, `9`) e **2** não assináveis hoje (`3`, `8`).

## 3. Quatro achados que valem para mais de uma página

| Achado | Onde | Correção proposta |
|---|---|---|
| o plano do CI **existe** no projeto e três páginas dizem que não | `index.md` §6, `specs/sistema.md` §7 e `alvos/sistema-stack-caps.md` §8 dizem "a página `plano-ci-real.md` está planejada e ainda não foi escrita"; `../wyd-ops/plano-ci-real.md` existe (`proposta`, `P`, 2026-11-19) | trocar por: "não existe página de CI em `docs/planos/` do `wyd-db`; o plano do projeto é `../wyd-ops/plano-ci-real.md`" |
| contagem afirmada como conferida contra artefato que não existe | `alvos/contrato-migrations.md` §1 | "é conferida" → "será conferida"; a série e a fixture nascem na leva de `migrations/` |
| valor "escrito" em arquivo ausente | `dados-de-teste.md` §4; `resources/README.md` §4, item 3 | "declarada/escrita" → "será declarada/escrita em `accounts.json`" |
| duas leituras do mesmo cenário | `dados-de-teste.md` §2: prosa e tabela divergem sobre a 6ª conta | o DDL recusa `slot` fora de 1–5 (`ck_account_slot_range`); alinhar a prosa à tabela |

**Verificado e sem defeito:** a legenda "sem marca — ... comportamento do PostgreSQL (`padrao-banco.md`
§13)" aparece nas **12** páginas. `padrao-banco.md` §13 é **"Fontes"** — 16 links da documentação do
PostgreSQL — e as páginas a usam como **bibliografia do comportamento**, não como seção de regra. A dúvida
levantada em §6.6.26 fica fechada por leitura: não há o que corrigir.

## 4. O que o dono decidiu — e o que foi corrigido antes da assinatura

| Grupo | Páginas | Decisão e efeito |
|---|---:|---|
| pronto | **7** | assinadas como estavam |
| corrigir e assinar | **3** | correções de redação aplicadas (§3) e assinadas |
| era "não assinar ainda" | **2** | tratadas antes da assinatura: `dados-de-teste.md` teve a prosa de §2 alinhada ao DDL; `alvos/contrato-catalogo.md` recebeu a nota de que **`Q-BLK-20`** decide o destino de `CAP-08` |

As cinco correções aplicadas, nenhuma delas mudando requisito, capacidade, limiar, cenário ou DDL:

1. `index.md` §6, `specs/sistema.md` §7 e `alvos/sistema-stack-caps.md` §8 passaram a citar o plano de CI
   que existe (`../wyd-ops/plano-ci-real.md`, `proposta`) em vez de dizer que ele não foi escrito;
2. `alvos/contrato-migrations.md` §1: "é conferida" → "**será** conferida … quando as duas existirem";
3. `fixtures-e-factories.md`: "inventário real" → "**inventário planejado**";
4. `resources/README.md` §4 e `dados-de-teste.md` §4: senha "escrita/declarada" → "**será** escrita/declarada
   em `accounts.json`";
5. `dados-de-teste.md` §2: a sexta tentativa passou a usar a **vaga 6**, recusada por
   `ck_account_slot_range`, e a vaga fechada ficou remetida ao cenário `CAP-10`.

**O que ainda falta depois disso:** `Q-BLK-18` (o aceite libera o código?) para os 5 alvos; a série `.sql`
(leva `migrations/`) para qualquer cenário rodar; `Q-BLK-20` para o destino de `CAP-08`; e `Q-BLK-23` para
os **16** cenários sem prova.

## 5. Assinatura e registro (2026-11-24)

- **Decisão do dono:** `Q-BLK-21` = **A** — o dono revisou e assinou as **12** páginas em 2026-11-24, por
  aprovação registrada na sessão da mesma data. O repositório não guarda nome no cabeçalho; a assinatura é
  o estado datado, e o registro fica em `pendencias-de-fechamento.md` §6.6.23.
- **Efeito:** as 12 passaram a `**Estado:** aceito (2026-11-24)`; `tests/` tem **12 `aceito`** e **0
  `proposto`**. O sistema tinha **15** páginas `aceito` naquele momento (2 ADRs + `migrations/0002` + as 12).
- **Censo:** os atos de aceite pendentes caíram em **12** (as 12 páginas de `tests/` saíram da fila) e o
  valor vigente naquele momento era **205** (hoje **168**, após o aceite das 37 de `src/`): 141 `docs/` + 37 `src/` + 27 `migrations/` + **0** `tests/`. As **23** capacidades continuam **0 cobertas** — aceite de página não
  cobre capacidade.
- **O que a assinatura não fez:** nenhum `.rs` nasceu, nenhum teste rodou, `Q-BLK-18` e `Q-BLK-20` seguem
  abertas e nenhuma página fora de `tests/` mudou de estado.

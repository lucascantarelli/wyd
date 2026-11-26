# Decisões do `wyd-bots` — ambiente, dado, limiar e artefato

> Pergunta: "em que ambiente o bot roda, como nasce o dado sintético, qual limiar reprova a carga e o
> que uma falha entrega?"
> Público: QA, arquitetura e `wyd-ops`
> Dono: equipe do `wyd-bots`
> Base: `lacunas.md` (`LBOT-01`–`LBOT-04`); `wyd-bots-1.0.0.md` §3–§6; `docs/index.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X13) e §7.6; `AGENTS.md` §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Nenhuma pergunta segue aguardando o dono: `Q-BOT-1`, `Q-BOT-2` e
`Q-BOT-3` foram respondidas (= A, §4) e `Q-BOT-4` fechou por **regra vigente** (`Q-BOT-4` = A derivada,
§4), sem escolha do usuário. `LBOT-03` continua **aberta como medição**: a política foi decidida, o
número não. Nada foi aplicado. **Não há medição de carga, conexão simultânea nem latência**; o único
número desta página é a meta de 85% de `AGENTS.md` §8, que é regra existente, não medição. Qualquer
limiar numérico continua a medir, não a inventar.

## 1. Fronteira

**Decide:** o critério de carga — `LBOT-03`. O ambiente do bot (`LBOT-01`) e a origem da conta/personagem
sintéticos (`LBOT-02`) fecharam em 2026-11-23 pelas respostas do dono (`Q-BOT-1` e `Q-BOT-2` = A), e o
artefato de falha (`LBOT-04`) fechou por regra vigente na mesma data (§4). A política de `LBOT-03` foi
decidida em `Q-BOT-3` = A, mas a lacuna **não fecha** até a primeira medição no CI efêmero.

**Não decide:** codec e fio (`wyd-protocol`), socket e rate-limit (`wyd-net`), regra de jogo
(`wyd-core`/`wyd-sim`), provisionamento e métrica de produção (`wyd-ops`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **1 aberta, 0 propostas, 3 fechadas** | `lacunas.md` §1 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-bots-1.0.0.md` §4 |
| cenários escritos | **0** | idem |
| testes de carga medidos | **0** | idem |
| contrato de codec em modo cliente | **decidido** | `wyd-bots-1.0.0.md` §3 (B1) |
| teste de dois canais / E2E | o `wyd-bots` é o próprio E2E; nenhum existe | `wyd-1.0.0.md` §7.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| codec em modo cliente | o bot usa o mesmo `encode`/`decode`, sem atalho de teste | `wyd-bots-1.0.0.md` §3 B1 | a lacuna não é de codec |
| cobertura e anti-skip | 85% por sistema, sem `SKIP` silencioso | `AGENTS.md` §8 | o relatório de falha não pode esconder cenário |

## 4. Decisões

### Q-BOT-1 — Onde o bot roda?

**Medido.** Não há ambiente de bot decidido nem servidor completo; a lacuna inclui CI efêmero,
homologação e máquina local, e o plano exige que nenhum bot rode em produção sem autorização.

**O que está em jogo.** CI efêmero é reproduzível, mas precisa de um servidor completo; homologação é
mais próxima do real, mas compartilhada; máquina local não é reproduzível.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | CI efêmero, com servidor completo descartável por execução | reproduzível e sem risco de produção; depende de o `wyd-server` existir no job, o que ainda não acontece |
| B | Homologação dedicada | disponível antes do servidor em CI; exige isolamento e limpeza, e o resultado pode variar com o ambiente |
| C | Máquina local do QA | começa sem infraestrutura; não é reproduzível nem serve de gate |

**Recomendação: A.** O bot é E2E e precisa de ambiente descartável; até o servidor caber no CI, B
pode ser paliativo declarado, não a decisão final. **Custo:** job de CI mais caro e dependência do
`wyd-server`. **Fontes:** `lacunas.md` `LBOT-01`; `wyd-bots-1.0.0.md` §3 B2 e §6; `AGENTS.md` §9
(runner hospedado, sem self-hosted em PR). **Recusa C** por não ser reproduzível; **B** por não ser o
alvo.

**Resposta do dono em 2026-11-23: A.** O ambiente alvo do bot é o **job de CI em runner hospedado**,
com Postgres e servidor descartáveis por execução e sem estado entre cenários; o contrato B2 passa a
`CI efêmero` e o replay pode ser gate de PR. Homologação fica reservada a **paliativo declarado**, com
data e dono, até a fatia do `wyd-server` caber no job; nunca vira decisão final. A decisão não implementa
o job, o servidor, a fixture nem o bot: ela fixa o alvo.

| Alternativa | Por que foi descartada |
|---|---|
| B — homologação dedicada | ambiente compartilhado varia com carga e estado, exige isolamento e reset, e o número medido lá não vale como gate de PR |
| C — máquina local do QA | não é reproduzível e não pode reprovar nada; ao trocar de ambiente, cenário e limiar são refeitos |

**Custo exato de A, que não foi implementado.** Para o gate rodar: **1 arquivo novo** em
`.github/workflows/` (job com `services: postgres`), **1 `Cargo.toml`** de workspace na raiz, código em
**5–6 crates hoje vazios** (`wyd-protocol`, `wyd-net`, `wyd-db`, `wyd-core`, `wyd-sim`, `wyd-server`),
**1 `seeds/dev.sql`** no `wyd-db`, **1 runner de cenário** em `crates/wyd-bots` e a captura A1
(`LPRT-03`/`LPRT-06`). Sem custo financeiro em repositório público (`AGENTS.md` §9).

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LBOT-01` | `lacunas.md` §1 |
| contrato B2 passa de lacuna a decidido e cumpre o item 2 do gate | `wyd-bots-1.0.0.md` §3 e §6 |
| libera `Q-BOT-2` (onde a fixture mora e como reseta) e `Q-BOT-3` (onde o limiar é medido) | §4 |
| dá ambiente ao replay dos 7 goldens em CI | `wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md` §9, etapa 5 |

**Não fecha:** `LBOT-03` (limiar, QA + `wyd-ops`, segue medição), a fatia do `wyd-server`, a captura
`LPRT-03`/`LPRT-06` e o primeiro cenário (`RF-BOT`/gate item 1); a conta sintética e o `seeds/dev.sql`
ficam em `Q-BOT-2` (§4).

### Q-BOT-2 — Como nasce a conta e o personagem sintéticos?

**Medido.** Não há fixture, seed de bot nem conta de teste; o plano exige "fixture, não conta real".
`M19` (senha de dev) está **fechada** em 2026-11-19, mas `seeds/dev.sql` ainda não existe; os seis
seeds de conteúdo seguem bloqueados.

**O que está em jogo.** Conta real ou banco compartilhado produz teste não reprodutível e risco de
dado pessoal; fixture em banco efêmero isola, mas precisa de reset e de um caminho de criação.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | Fixture/seed sintético no banco efêmero do cenário, com reset por execução | nenhum dado de jogador; reproduzível; exige um seed de bot e um reset barato |
| B | Criar conta e personagem pela API do portal | exercita o fluxo real; depende do portal e deixa estado persistido se o reset falhar |

**Recomendação: A.** O contrato B3 já diz "fixture, não conta real"; a decisão é onde a fixture mora
e como reseta. **Custo:** um seed e um reset a manter. **Fontes:** `lacunas.md` `LBOT-02`;
`wyd-bots-1.0.0.md` §3 B3; `AGENTS.md` §5, item 7; `M19` em `lacunas.md` do `wyd-db`. **Conta real não é
opção**: contraria B3. **Recusa B** por acoplar o bot ao portal.

**Resposta do dono em 2026-11-23: A.** A conta, a pessoa e o personagem de cada cenário nascem de um
**seed declarativo** em `crates/wyd-db/seeds/dev.sql`, aplicado depois das migrations no banco recriado
por execução; o reset é o próprio banco novo, sem passo de limpeza. O seed grava **5 linhas em 5
tabelas** — `portal.person` (e-mail sintético e hash Argon2id de `dev-only`), `game.account` (login,
vaga e o mesmo hash), `content.content_version` (linha única `id = 1`, porque `game.character` tem FK
para ela e o `0016` está bloqueado), `game.character` (conta, vaga, nome, classe e `content_version =
1`) e `game.server_channel` (canal de instalação, exigido pela FK de `game.session`). Não é migration: a
série de 34 arquivos e o `SCHEMA_VERSION` não mudam, e os seis seeds de conteúdo continuam bloqueados.

| Alternativa | Por que foi descartada |
|---|---|
| B — API do portal | exige portal, servidor e cliente HTTP que não existem, acopla o QA ao fluxo web e deixa estado se o reset falhar |
| C — dump/snapshot | artefato opaco, sofre drift com o schema e não é revisável linha a linha nem auditável contra `P-22` |

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LBOT-02` | `lacunas.md` §1 |
| contrato B3 passa a decidido; B5 fica decidido pelo banco descartável | `wyd-bots-1.0.0.md` §3 |
| `X13` fica decidido: ambiente e conta sintética resolvidos | `wyd-1.0.0.md` §7.3 |
| libera `Q-BOT-3`, que mede no ambiente e com o dado já decididos | §4 |

**Não fecha:** `LBOT-03` (limiar, QA + `wyd-ops`, segue medição), a fatia do `wyd-server`, a captura
`LPRT-03`/`LPRT-06`, os seis seeds de conteúdo (`0016`–`0027`, `wyd-db`), as fixtures de `wyd-sim`, o
primeiro cenário e o código do bot. O `seeds/dev.sql` é **dev-only**: não substitui o seed de conteúdo
nem entra em produção (`M19`).

### Q-BOT-3 — Qual limiar de carga reprova e o que o bot faz ao atingi-lo?

**Medido.** Não há conexões simultâneas, duração, latência nem taxa de sucesso medidas; o plano diz
que "limiar é medido, não estimado" e `AGENTS.md` §8 exige gate medido.

**O que está em jogo.** Fixar um número agora é promessa sem fonte; não ter critério nenhum deixa a
carga sem gate. A decisão é **como** o número entra, não qual é.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | Medir primeiro em ambiente controlado; nenhum limiar numérico entra no gate até a medição | nenhum número inventado; a carga fica sem reprovação até a primeira medição, que vira o gate |
| B | Adotar um teto provisório de recurso do ambiente (conexões que o processo suporta) | dá um gate imediato; o teto é de infraestrutura, não de comportamento do jogo |
| C | Fixar agora conexões/latência alvo | permite escrever o teste; o número não tem fonte e vira promessa |

**Recomendação: A.** O número é decisão, não medição; a primeira execução mede e o resultado vira o
limiar, com data. **Custo:** a carga não reprova antes da medição. **Fontes:** `lacunas.md` `LBOT-03`;
`wyd-bots-1.0.0.md` §3 B4 e §6; `AGENTS.md` §8. **Recusa C** por inventar número; **B** por medir
infraestrutura, não o cenário.

**Resposta do dono em 2026-11-23: A.** O cenário sobe em **degraus de conexões** no CI efêmero de
`Q-BOT-1`, com o seed de `Q-BOT-2` e o relatório de `Q-BOT-4`, e mede **P50/P99 de ida e volta**,
**taxa de sucesso** e **recusas por motivo**. Saturação é o **primeiro degrau em que as recusas deixam
de ser zero ou o P99 cresce sem voltar ao patamar anterior**; o valor medido é congelado com ambiente e
data e passa a ser o gate. Até a primeira medição, o cenário é **exploratório** e não reprova; depois,
ao atingir o limiar congelado, o bot **para de subir, drena e falha** com o relatório de `LBOT-04`.
Nenhum número é inventado e a lacuna não fecha nesta decisão.

| Alternativa | Por que foi descartada |
|---|---|
| B — teto de infraestrutura | mede a capacidade do processo/ambiente, não o comportamento do jogo; pode bloquear jogador legítimo e é refeito |
| C — número fixado agora | não tem fonte; vira promessa sem `V`, contra `AGENTS.md` §5, item 4 e §8 |

**Custo do que A ainda exige, não implementado.** **0 migration, 0 tabela, 0 coluna.** Na implementação:
**1 página** `docs/specs/cenarios.md`, **1 `Cargo.toml`** em `crates/wyd-bots`, **1 `src/scenario.rs`**,
**1 CLI** (`src/main.rs`), **1 `Cargo.toml`** de workspace na raiz e **1 job** em `.github/workflows/`;
**6 arquivos novos**, além do seed de `Q-BOT-2`. O gate item 1 (primeiro cenário) e `RF-BOT` continuam
pendentes; o `Q-NET-1` continua sendo pré-requisito para interpretar o teto por IP/conexão.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| política do contrato B4 decidida | `wyd-bots-1.0.0.md` §3 |
| `LBOT-03` continua aberta como medição | `lacunas.md` §1 |
| liberou `Q-NET-1` como próxima pergunta; a medição espera o `wyd-server` e o `Q-NET-1` | §4 |

**Não fecha:** `LBOT-03`, o número, o cenário, o runner, o `wyd-server`, o `Q-NET-1`, a telemetria e o
gate item 1.

### Q-BOT-4 — O que uma execução falha entrega?

**Medido.** Não há relatório, log, captura nem trace de bot; o gate do plano pede "relatório por
execução com falha reproduzível (log + captura)" e `AGENTS.md` §8 proíbe `SKIP` silencioso.

**O que está em jogo.** Sem artefato, a falha não é reproduzível e o custo do teste não se paga; log
com payload ou dado de jogador pode carregar segredo/PII e conflita com a higiene de `AGENTS.md` §5, item 7
e com a fronteira de PII do `wyd-db` (`P-22`).

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Log estruturado + trace da conexão + frame/captura anonimizada no ponto da falha | falha reproduzível; exige scrubbing e armazenamento fora do git |
| B | Só log textual | simples; não prova o estado do fio nem reproduz a falha |
| C | Nada além do exit code | mínimo; não há como diagnosticar |

**Recomendação: A.** O gate já pede log + captura; a decisão é o formato e a higiene. **Custo:**
armazenamento e revisão de PII. **Fontes:** `lacunas.md` `LBOT-04`; `wyd-bots-1.0.0.md` §6;
`AGENTS.md` §5, item 7 e §8; `wyd-db/padrao-banco.md` `P-22`. **Recusa B/C** por não reproduzirem a falha.

**Resposta derivada das regras vigentes, sem escolha do usuário (2026-11-23): A.** O item 4 do gate do
sistema já exige “relatório por execução com falha reproduzível (log + captura)” e o item 5 proíbe
segredo de jogador (`wyd-bots-1.0.0.md` §6). C (“nada além do exit code”) reprova o item 4 e B (“só log
textual”) reprova a parte “captura”; `AGENTS.md` §5, item 7 e `P-22` fixam a higiene. A execução falha entrega
**log estruturado + frame/captura anonimizada no ponto da falha**, com cenário e `ClientTick`, sem
payload, credencial, nome de personagem nem dado de jogador; trace é opcional e segue a mesma higiene. A
lacuna `LBOT-04` fechou por essa regra. A pauta em `../../wyd-ops/docs/pauta-de-decisoes-pendentes.md` e `../../lacunas.md`
registram as respostas.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LBOT-01` | `Q-BOT-1` | fechada em 2026-11-23 (A): ambiente alvo é o job de CI em runner hospedado, com Postgres e servidor descartáveis por execução; B2 passa a `CI efêmero`; homologação só como paliativo declarado. |
| `LBOT-02` | `Q-BOT-2` | fechada em 2026-11-23 (A): seed declarativo em `crates/wyd-db/seeds/dev.sql`, 5 linhas em 5 tabelas, banco recriado por execução; dev-only (`M19`); conta real não é opção (B3). |
| `LBOT-03` | `Q-BOT-3` | **aberta como medição** (2026-11-23): política A decidida — medir, congelar, parar/drenar/reportar; sem número, sem cenário e sem `Q-NET-1`, a lacuna não fecha. |
| `LBOT-04` | `Q-BOT-4` | fechada em 2026-11-23 (A): **derivada das regras vigentes, sem escolha do usuário** — log estruturado + frame anonimizado no ponto da falha, sem payload/PII; o gate do sistema já exigia log + captura. |

## 6. Registro da decisão

**Nenhuma aguarda o dono; `Q-BOT-1`–`Q-BOT-3` foram respondidas e `Q-BOT-4` derivada em 2026-11-23.**
`LBOT-03` continua `aberta` em `lacunas.md` como **medição**; `LBOT-01` e `LBOT-02` fecharam pelas
respostas do dono (`Q-BOT-1`/`Q-BOT-2` = A), `LBOT-04` pela regra do próprio gate, sem escolha do
usuário, e a política de `LBOT-03` foi fixada por `Q-BOT-3` = A. A pauta em `../../wyd-ops/docs/pauta-de-decisoes-pendentes.md` reúne a ordem das respostas.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-BOT-1` | `LBOT-01` | A — CI efêmero em runner hospedado, com servidor descartável por execução; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-BOT-2` | `LBOT-02` | A — seed declarativo em `crates/wyd-db/seeds/dev.sql`, banco recriado por execução; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-BOT-3` | `LBOT-03` | A — medir e congelar o limiar na primeira execução; ao atingi-lo, parar, drenar e reportar; resposta do dono em 2026-11-23, política registrada e lacuna **aberta como medição** |
| `Q-BOT-4` | `LBOT-04` | A — **derivada das regras vigentes, sem escolha do usuário** (2026-11-23): log estruturado + frame anonimizado no ponto da falha, sem payload/PII |

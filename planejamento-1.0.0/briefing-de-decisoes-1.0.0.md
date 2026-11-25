# Briefing de decisões da `1.0.0` — uma passada, todas as perguntas abertas

> Pergunta: "qual resposta fecha mais lacuna e libera mais frente por unidade de esforço, o que cada uma desbloqueia e o que custa a resposta oposta?"
> Público: dono do produto, arquitetura, `wyd-ops` e QA
> Dono: arquitetura (este briefing); **o foco e o ranking do projeto são da página canônica** `wyd-ops/docs/prioridade-do-projeto.md` §0.1; este briefing cobre as decisões de sistema
> Base: `wyd-1.0.0.md` §6.1–§6.3 e §7; `wyd-sim/docs/proposta-decisoes-sim.md`; `wyd-launcher/docs/proposta-decisoes-launcher.md`; `wyd-bots/docs/proposta-decisoes-bots.md`; `wyd-net/docs/proposta-decisoes-net.md`; `wyd-portal/docs/proposta-decisoes-portal.md`; `wyd-server/docs/specs/endpoint-de-captura.md` §8–§9.1; `AGENTS.md` §5, §8 e §9
> Revisado em: 2026-11-23 (respostas `Q-SIM-2`, `Q-SIM-3`, `Q-BOT-1`, `Q-BOT-2`, `Q-BOT-3`, `Q-NET-1`, `Q-LCH-1`, `Q-LCH-2`, `Q-LCH-3`, `Q-LCH-4`, `LAC-POR-09` e A1 registradas)

**Estado:** proposto (2026-11-23). Este documento **mede o que está aberto e ordena as respostas**; não
implementa, não cria issue, não altera schema e não escolhe produto. Duas lacunas (`LNET-04` e
`LBOT-04`) fecharam por regras vigentes, sem escolha do usuário, e `LSIM-02`/`LSIM-03`, `LBOT-01`,
`LBOT-02`, `LLCH-01`, `LLCH-02`, `LLCH-03` e `LLCH-04` fecharam após as respostas explícitas do dono
`Q-SIM-2` = A, `Q-SIM-3` = A, `Q-BOT-1` = A, `Q-BOT-2` = A, `Q-LCH-1` = A, `Q-LCH-2` = A, `Q-LCH-3` = A e
`Q-LCH-4` = A; `Q-BOT-3` = A fixou a política de limiar e `Q-NET-1` = A fixou a política de limites, mas
**não fecharam `LBOT-03`/`LNET-01`**, que seguem abertas como medição. `LAC-POR-09` = A fechou a última
lacuna do portal e zerou as respostas pendentes. Esses estados estão registrados nas páginas dona. Para as decisões de **sistema** cobertas aqui, o briefing mantém **0 respostas** pendentes; as decisões
de **foco** (`Q-FOCO-3`, `Q-FOCO-4`) e as **2** `Q-BLK` do piloto estão na página canônica e foram
**decididas em A em 2026-11-24** (D-01 e D-03). A1 foi
autorizada em 2026-11-23 e a implementação é a próxima ação. Nenhuma
fixture, fórmula, coeficiente, número de balanceamento, dado de jogo ou conteúdo de cliente é criado aqui.

> **Atualização de 2026-11-25:** o ponto de entrada único das decisões pendentes é a
> [pauta de decisões](wyd-ops/docs/pauta-de-decisoes-pendentes.md). Este briefing descreve o estado de
> 2026-11-23 e não é mais o índice das perguntas abertas; a frase que dizia que o `wyd-ops` não tem
> `lacunas.md` está superada pelo registro de 2026-11-25 em [lacunas.md](wyd-ops/docs/lacunas.md).

## 0. Como ler

- Marcas: `V` (verificado por arquivo ou comando, com data), `H` (hipótese, com a lacuna que a fecha),
  `P` (projetado). Presente do indicativo sem `V` é erro de escrita (`AGENTS.md` §5, item 4).
- **Derivada das regras vigentes, sem escolha do usuário** significa: mais de uma opção existia na
  proposta, mas só uma é compatível com contrato ou regra já decidida. A decisão fica registrada no
  documento dono e conta na lacuna; não é resposta de produto.
- Estados de lacuna: **aberta**, **proposta** (recomendação escrita, aguardando o dono) e **fechada**
  (com data e evidência). O dono do estado continua sendo o `lacunas.md` de cada sistema.
- Cada pergunta de §3 traz os seis itens pedidos: pergunta decidível, recomendação única, motivo medido,
  o que desbloqueia, o custo da resposta oposta e o momento de responder.
- Fontes por item: [proposta do `sim`](wyd-sim/docs/proposta-decisoes-sim.md),
  [proposta do `launcher`](wyd-launcher/docs/proposta-decisoes-launcher.md),
  [proposta do `bots`](wyd-bots/docs/proposta-decisoes-bots.md),
  [proposta do `net`](wyd-net/docs/proposta-decisoes-net.md),
  [proposta do `portal`](wyd-portal/docs/proposta-decisoes-portal.md),
  [lacunas do `net`](wyd-net/lacunas.md), [lacunas do `bots`](wyd-bots/lacunas.md),
  [spec do `ECAP`](wyd-server/docs/specs/endpoint-de-captura.md) e
  [esqueleto de ponta a ponta](wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md).

## 1. O que está aberto hoje (`V`, 2026-11-23)

Contagem lida de cada `lacunas.md` após as decisões derivadas de §2 e as respostas do dono `Q-SIM-2` = A,
`Q-SIM-3` = A, `Q-BOT-1` = A, `Q-BOT-2` = A, `Q-LCH-1` = A, `Q-LCH-2` = A, `Q-LCH-3` = A, `Q-LCH-4` =
A e `LAC-POR-09` = A:

| Sistema | Lacunas | Abertas | Propostas | Fechadas | Externas | IDs não fechados |
|---|---:|---:|---:|---:|---:|---|
| `wyd-protocol` | 20 | 2 | 0 | 18 | 0 | `LPRT-03`, `LPRT-06` |
| `wyd-client` | 5 | 2 | 0 | 3 | 0 | `LCLI-03`, `LCLI-05` |
| `wyd-net` | 4 | 1 | 0 | 3 | 0 | `LNET-01` |
| `wyd-core` | 4 | 0 | 0 | 4 | 0 | — |
| `wyd-sim` | 4 | 0 | 0 | 4 | 0 | — |
| `wyd-server` | 5 | 0 | 0 | 5 | 0 | — |
| `wyd-db` | 38 | 0 | 0 | 35 | 3 | `A11`, `A20`, `M21` (externas) |
| `wyd-admin` | 7 | 0 | 0 | 7 | 0 | — |
| `wyd-portal` | 9 | 0 | 0 | 9 | 0 | — |
| `wyd-launcher` | 4 | 0 | 0 | 4 | 0 | — |
| `wyd-bots` | 4 | 1 | 0 | 3 | 0 | `LBOT-03` |
| `wyd-docs` | 14 | 0 | 0 | 14 | 0 | — |
| **Total** | **118** | **6** | **0** | **109** | **3** | **6 não fechados** |

**Somas conferidas:** `118 = 6 + 0 + 109 + 3`; os **6 não fechados** são `6 abertos + 0 proposta`.
Desses 6, **0 têm pergunta de desenho isolada aguardando resposta** — a de `LCLI-02` foi executada em
2026-11-25 —, **2 são medições com política já
decidida** (`LNET-01`, `Q-NET-1` = A; `LBOT-03`, `Q-BOT-3` = A) e **4 não são pergunta técnica**
(`LPRT-03`, `LPRT-06`, `LCLI-03`, `LCLI-05`)
(`LPRT-03`, `LPRT-06`, `LCLI-02`, `LCLI-03`, `LCLI-05`, explicados em §4). `wyd-ops` não tem `lacunas.md` e não entra na soma,
como em `wyd-1.0.0.md` §6.1.

Duas lacunas fecharam por regra vigente (`LNET-04`, `LBOT-04`) e
`LSIM-02`/`LSIM-03`/`LBOT-01`/`LBOT-02`/`LLCH-01`/`LLCH-02`/`LLCH-03`/`LLCH-04` fecharam pelas respostas
explícitas do dono (`Q-SIM-2` = A, `Q-SIM-3` = A, `Q-BOT-1` = A, `Q-BOT-2` = A, `Q-LCH-1` = A, `Q-LCH-2` =
A, `Q-LCH-3` = A e `Q-LCH-4` = A), e `LAC-POR-09` fechou pela resposta A do dono de operação. O delta
de `LAC-POR-09` é `0 abertas / −1 proposta / +1 fechada`; o total continua 118.

## 2. Decisões derivadas das regras vigentes (sem escolha do usuário)

### 2.1 `Q-NET-4` = A — `LNET-04` fechada

**A pergunta original.** O que o transporte expõe em métrica e log?

**Regras que decidem, medidas no repositório.**

| Regra | O que já fixa | Fonte |
|---|---|---|
| `X10` | a métrica do NOC é `/metrics` no formato Prometheus, coletada pelo `wyd-ops` | `wyd-1.0.0.md` §7.3 |
| `LAC-POR-06` | a telemetria de canal vem de fonte externa; o portal consulta o Prometheus e mostra "não medido" quando falha | `wyd-portal/lacunas.md` §2 |
| `AGENTS.md` §5, item 7 | segredo, token, senha e PIN nunca aparecem em log; payload e PII não entram | `AGENTS.md` §5, item 7 |

**Decisão derivada.** O transporte expõe métricas de **conexões, erros por tipo, bytes e latência** em
`/metrics`, e grava log **só de metadados** (evento, sessão, tamanhos, caminho), nunca payload, credencial
ou PII. Nome, rótulo e unidade de cada métrica são implementação, não decisão pendente.

**Por que não há escolha.** A opção C ("sem métrica/log") contradiz `X10`, que é contrato decidido, e
`LAC-POR-06`, que já escolheu a fonte externa; log de payload contradiz `AGENTS.md` §5, item 7. Resta o
conjunto A. Não é preferência de arquitetura: é a única combinação compatível com três decisões
vigentes.

**Efeito registrado.** `LNET-04` fecha em `wyd-net/lacunas.md`; o `wyd-net` passa a
`4 lacunas / 1 aberta / 0 propostas / 3 fechadas`; `Q-NET-4` sai da fila do dono; o contrato `X10`
ganha conteúdo de implementação. `LNET-01` **não** fecha: continua aberta como medição (§3.10).

**O que teria custado a resposta oposta.** Escolher C exigiria reabrir `X10` e `LAC-POR-06`; aceitar log
de payload exigiria revogar `AGENTS.md` §5, item 7. Nenhuma das duas é decisão desta rodada, e as duas
deixariam o NOC sem a métrica que o portal já publica.

### 2.2 `Q-BOT-4` = A — `LBOT-04` fechada

**A pergunta original.** O que uma execução falha entrega: log, captura, trace ou nada?

**Regras que decidem, medidas no repositório.**

| Regra | O que já fixa | Fonte |
|---|---|---|
| gate do `wyd-bots`, item 4 | "Relatório por execução com falha reproduzível (log + captura), sem `sleep` como sincronização" | `wyd-bots/wyd-bots-1.0.0.md` §6 |
| gate do `wyd-bots`, item 5 | "Nenhum segredo de jogador em configuração de bot" | `wyd-bots/wyd-bots-1.0.0.md` §6 |
| `AGENTS.md` §5, item 7 | segredo/PII nunca em log | `AGENTS.md` §5, item 7 |
| `P-22` | PII mora só em `portal`; o jogo referencia `person_id`, nunca copia e-mail | `wyd-db/padrao-banco.md` `P-22` |

**Decisão derivada.** Uma execução falha entrega **log estruturado + frame/captura anonimizada no ponto
da falha**, com o cenário, o `ClientTick` e o passo reproduzível; o log não carrega payload, credencial,
nome de personagem nem dado de jogador. Trace da conexão é opcional e, se existir, segue a mesma
higiene.

**Por que não há escolha.** O item 4 do gate já exige log **e** captura: a opção C ("nada além do exit
code") reprova o gate, e B ("só log textual") reprova a parte "captura". O item 5 e `AGENTS.md` §5, item 7
fixam a higiene. A é o único conjunto que cumpre o gate que o próprio sistema já declarou.

**Efeito registrado.** `LBOT-04` fechou em `wyd-bots/lacunas.md`; depois de `Q-BOT-2`, o `wyd-bots` está
em `4 lacunas / 1 aberta / 0 propostas / 3 fechadas`; `Q-BOT-4` e `Q-BOT-2` saíram da fila do dono.
`LBOT-03` continua aberta (§3.9).

**O que teria custado a resposta oposta.** C deixaria a falha sem diagnóstico e reprovaria o item 4; B
não provaria o estado do fio. Em ambos os casos o gate do sistema teria de ser rebaixado — o que
contraria `AGENTS.md` §8 (nenhum `SKIP` silencioso) e `wyd-bots-1.0.0.md` §6.

## 3. Itens de decisão do dono

§3.1–§3.10 registram, para rastreabilidade, as respostas já dadas a `Q-SIM-3`, `Q-SIM-2`, `Q-BOT-1`,
`Q-BOT-2`, `Q-BOT-3`, `Q-NET-1`, `Q-LCH-1`, `Q-LCH-2`, `Q-LCH-3` e `Q-LCH-4`; §3.11 registra a resposta
`LAC-POR-09` = A e o próximo item, A1. Cada item traz a
recomendação única; as alternativas ficam registradas na página dona, não como menu.

### 3.1 `Q-SIM-3` — resposta registrada: A (`LSIM-03`, fechada)

**Pergunta decidível.** Onde ficam os vetores de entrada e saída que provam determinismo?

**Resposta do dono: A** — fixtures JSON versionadas, um arquivo por fórmula em
`tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e identificador da versão de fórmula.
`LSIM-02` fechou na sequência (`Q-SIM-2` = A) e definiu esse identificador: `formula_version` + o
número de `content_version` usado.

**Motivo medido.** O gate da onda B é "determinismo provado por vetor" (`wyd-1.0.0.md` §7.7) e o mapa de
testes registra "0 vetores (`LSIM-03`)" (`wyd-1.0.0.md` §7.6). O esqueleto exige "vetor de entrada/saída
no `sim`" e o mesmo vetor serve ao teste do `core` (`wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md`
§2.1). A proposta já registrada recomenda A e descarta B por duplicação e C por não ser auditável
(`wyd-sim/docs/proposta-decisoes-sim.md` §4, `Q-SIM-3`).

**O que desbloqueia.** Fecha a decisão `LSIM-03` sobre local/formato e fixa o contrato de fixtures para
a onda B. **Não** cria `tests/fixtures/sim/`, fórmulas nem vetores, e não satisfaz o gate de saída B: ainda
faltam implementar as cinco fórmulas da fatia e preencher os vetores JSON; o identificador da versão foi
definido depois por `LSIM-02` (`Q-SIM-2` = A).

**Custo das alternativas não escolhidas.** B (inline) duplicaria o mesmo vetor entre teste do `sim`, QA
e contrato com o `core`; C (fora do repositório) não seria versionado nem lido pelo gate. Escolher qualquer
uma exigiria refazer o armazenamento e os testes quando o `core` reutilizasse os casos.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23.** Foi uma decisão documental, sem código.
A resposta seguinte do `sim`, `Q-SIM-2`, foi registrada em §3.2.

### 3.2 `Q-SIM-2` — resposta registrada: A (`LSIM-02`, fechada)

**Resposta do dono: A** — cada fórmula carrega versão própria (`formula_version`, inteiro a partir de 1),
registrada junto do `content_version` usado no cálculo.

**Motivo medido.** `game.character` já carrega `content_version SMALLINT NOT NULL` com FK para
`content.content_version` (`wyd-db/migrations/0006-game-character.md` §6.1, linhas 70 e 77). O contrato
`X4` está decidido como função pura e determinística (`wyd-1.0.0.md` §7.3), e o fixture de `LSIM-03`
precisa de um campo de versão (`wyd-sim/docs/proposta-decisoes-sim.md` §4, `Q-SIM-3`). Sem versão
própria, mudar código sem mudar conteúdo fica invisível; sem nenhuma versão, o cálculo antigo não é
reproduzível.

**O que fechou.** `LSIM-02`, o cabeçalho do fixture e a rastreabilidade do cálculo: toda saída declara
`formula_version` + o **número** de `content_version` usado (`content.content_version.version`, não o `id`
da linha única). **1 lacuna + 1 campo do fixture + a rastreabilidade do `content_version`.**

**Custo das alternativas não escolhidas.** B mistura código e dado: uma correção de fórmula sem mudança
de conteúdo fica invisível — e a coluna que existe hoje não carrega o número, então B exigiria antes uma
migration (`game.character.content_version` tem FK para o `id`, preso a `1` por `CHECK`); C quebra a
reprodutibilidade e impede atribuir uma saída antiga a uma versão. O retrabalho seria regerar vetores e
refazer a leitura histórica sem saber qual código produziu cada número.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. O `sim`
não tem mais pergunta aberta; a resposta seguinte da rodada, `Q-BOT-1`, está em §3.3.

### 3.3 `Q-BOT-1` — resposta registrada: A (`LBOT-01`, fechada)

**Resposta do dono: A** — o ambiente alvo é o job de CI em runner hospedado, com Postgres e servidor
descartáveis por execução e sem estado entre cenários; homologação só como paliativo declarado, com data
e dono, até a fatia do `wyd-server` caber no job.

**Motivo medido.** O contrato B2 já nomeia "ambiente efêmero ou homologação" e proíbe bot em produção
sem autorização (`wyd-bots/wyd-bots-1.0.0.md` §3); o gate do sistema exige ambiente decidido (§6, item 2).
`AGENTS.md` §9 fixa runner hospedado, gratuito em repositório público, e proíbe self-hosted em PR. O bot
é o próprio E2E (`wyd-1.0.0.md` §7.6). Hoje o CI de código tem só o job `get-info`
(`.github/workflows/full-workflow.yml:27`), não há serviço Postgres no Compose e `crates/*` só tem
`.gitkeep` — a decisão fixa o alvo, não entrega o job.

**O que fechou.** `LBOT-01`, o contrato B2 e o item 2 do gate; o replay dos 7 goldens em CI (`esqueleto`
§9, etapa 5) passa a ter ambiente nomeado. **1 lacuna + 1 contrato + 1 item de gate.**

**Custo das alternativas não escolhidas.** B varia com o ambiente compartilhado, exige isolamento e
reset, e o número medido em homologação não vale como gate de PR; C não é reproduzível e não reprova
nada. O retrabalho seria refazer cenário, carga e resultado quando o ambiente mudasse.

**Custo do que A ainda exige, não implementado.** 1 arquivo novo em `.github/workflows/` com
`services: postgres`, 1 `Cargo.toml` de workspace, código em 5–6 crates vazios, 1 `seeds/dev.sql`, 1 runner
em `crates/wyd-bots` e as capturas A1/B/C (`LPRT-03`/`LPRT-06`).

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A resposta
seguinte, `Q-BOT-2`, está em §3.4.

### 3.4 `Q-BOT-2` — resposta registrada: A (`LBOT-02`, fechada)

**Resposta do dono: A** — a conta, a pessoa e o personagem de cada cenário nascem de um **seed
declarativo** em `crates/wyd-db/seeds/dev.sql`, aplicado depois das migrations no banco recriado por
execução; o reset é o próprio banco novo, sem passo de limpeza.

**Motivo medido.** O contrato B3 já proíbe conta real e dado de jogador (`wyd-bots/wyd-bots-1.0.0.md`
§3), `M19` fixou a senha de dev em 2026-11-19 e `seeds/dev.sql` continua ausente; as migrations exigem
`portal.person` com e-mail e hash Argon2id (`0003` §4), `game.account` com `person_id` e hash
(`0005` §4) e `game.character` com FK para `content.content_version` (`0006` §4), linha que o `0016`
bloqueado ainda não semeia. O seed é revisável e o banco novo é o reset; B exige portal, servidor e
cliente HTTP que não existem, e C esconde o dado num dump que sofre drift.

**O que fechou.** `LBOT-02`, o contrato B3 e o contrato B5 (banco descartável por execução); `X13`
deixa de ser lacuna porque ambiente (`LBOT-01`) e conta (`LBOT-02`) estão decididos. **1 lacuna + 2
contratos + 1 externa.**

**Custo do que A ainda exige, não implementado.** **1 arquivo** em `crates/wyd-db/seeds/dev.sql`, com
**5 INSERTs em 5 tabelas** — `portal.person`, `game.account`, `content.content_version`,
`game.character` e `game.server_channel` — e **1 hash Argon2id** de `dev-only`; **0 migrations**, **0
mudança** na série de 34 arquivos e no `SCHEMA_VERSION`. O `content_version` entra no dev seed porque o
`0016` está bloqueado; os seis seeds de conteúdo não são substituídos.

**Custo da resposta oposta.** B acoplaria o QA ao portal e deixaria estado quando o reset falhasse; C
exigiria regerar o dump a cada mudança de schema e não seria revisável. Nos dois, o retrabalho recai
sobre o runner e o reset.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A próxima
da fila é `Q-LCH-1`.

### 3.5 `Q-LCH-1` — resposta registrada: A (`LLCH-01`, fechada)

**Resposta do dono: A** — o manifesto é **JSON versionado e canônico** (UTF-8, `schema`, `channel`,
`version`, `client_build`, `min_version` e `files[{path, size, sha256, url}]`, canonização RFC 8785),
para ser auditável e assinável por inteiro.

**Motivo medido.** O contrato L1 exige arquivo publicado com hash por arquivo
(`wyd-launcher-1.0.0.md:33`) e `X11` estava em lacuna (`wyd-1.0.0.md:592`); o portal já publica
`portal.launcher_release` com `channel`, `version`, `manifest_url`, `manifest_sha256` e `min_version`
(`0030-portal-content.md:172–203`). O legado tem um manifesto de texto v1 com `build`, `protocol` e
hash/tamanho/caminho por linha (`deprecated/crates/wyd-launcher/core/src/manifest.rs:16,28–31`), mas
não tem canal, `min_version`, URL nem assinatura e não pode ser copiado (`AGENTS.md` §5, item 2).

**O que fechou.** `LLCH-01`, o contrato L1 e o item 1 do gate do launcher; `X11` passou a seguir lacuna
**apenas** por `LLCH-03`. **1 lacuna + 1 contrato + 1 item de gate.**

**Custo do que A ainda exige, não implementado.** 1 página `docs/specs/manifesto-e-patch.md`, 1 JSON
Schema, 1 módulo `crates/wyd-launcher/src/manifest.rs` (o crate precisa de `Cargo.toml`) e 1 fixture;
0 migration — `portal.launcher_release` já está em `0030`.

**Custo da resposta oposta.** B exigiria codec e inspetor próprios e perderia auditabilidade; C teria de
ser reimplementado, ainda faltaria canal/`min_version`/URL/assinatura e o formato terminaria diferente do
legado.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A próxima
da fila é `Q-LCH-3`.

### 3.6 `Q-LCH-3` — resposta registrada: A (`LLCH-03`, fechada)

**Resposta do dono: A** — assinatura **Ed25519 destacada** sobre os bytes canônicos do manifesto,
verificada antes de aplicar; chave pública e `key_id` no launcher, chave privada só no secret do
`wyd-ops`, assinatura publicada como irmã `manifest.json.sig`.

**Motivo medido.** O risco é execução de código arbitrário por patch adulterado
(`wyd-launcher/lacunas.md:27`); o contrato L5 exige que manifesto adulterado não aplique patch
(`wyd-launcher-1.0.0.md:37`), e `AGENTS.md:214` prende o segredo ao secret do GitHub. O legado já usava
Ed25519 com chave pública embutida e privada só no CI/secret
(`deprecated/crates/wyd-launcher/core/src/distribuicao.rs:4–12`), mas é referência `H`, não fonte.
`portal.launcher_release` guarda `manifest_sha256`, não assinatura nem chave (`0030:172–203`).

**O que fechou.** `LLCH-03`, o contrato L5, o item 2 do gate e **`X11`** — era a última dependente
(`LLCH-01` + `LLCH-03`). **1 lacuna + 1 contrato + 1 item de gate + 1 externa.**

**Custo do que A ainda exige, não implementado.** 1 página `docs/specs/manifesto-e-patch.md`, 1
assinador em `tools/launcher/`, 1 módulo de verificação em `crates/wyd-launcher/src/` (o crate precisa
de `Cargo.toml`), 1 chave pública embutida, 1 fixture manifesto+assinatura e 1 passo no futuro
`release.yml`; 0 migration, 0 tabela, 0 coluna.

**Custo da resposta oposta.** B não autentica a origem e, para adicionar assinatura depois, reemite
formato, launcher e release; C aceita qualquer patch e exige todo o caminho de A para corrigir.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A próxima
da fila é `Q-LCH-2`.

### 3.7 `Q-LCH-2` — resposta registrada: A (`LLCH-02`, fechada)

**Resposta do dono: A** — a lista e o estado vêm do **portal/publicação**, com **cache local** no
launcher; a rota lê `game.server_channel` (capacidade, manutenção, papel, `status_url`) e a presença de
`game.v_live_session`; o launcher gera o `serverlist.bin` a partir da última lista boa.

**Motivo medido.** O banco já tem a grade do cliente — `group_id` 0–9, `channel` 0–10, `status_url`,
`host`, `maintenance`, `enabled` (`0007-game-session.md:50–74`) — e o portal planeja `/admin/canais`
lendo/escrevendo `game.server_channel` com auditoria (`rotas-e-permissoes.md:76`; `RF-POR-010`).
`LSRV-02` fixou 2 canais e escolha por lista no launcher (`wyd-server/lacunas.md:28`). O legado já gerava
`serverlist.bin` no formato 10 × 11 × 64 B (`deprecated/crates/wyd-launcher/core/src/serverlist.rs:3–13`),
mas é referência `H`.

**O que fechou.** `LLCH-02` e o contrato L2. **1 lacuna + 1 contrato.**

**Custo do que A ainda exige, não implementado.** 1 página `docs/specs/lista-de-canais.md`, 1 rota/BFF em
`wyd-portal`, 1 módulo de leitura/cache e 1 gerador de `serverlist.bin` em `crates/wyd-launcher` e 1
fixture; 0 migration, 0 tabela, 0 coluna.

**Custo da resposta oposta.** B exige nova release para cada mudança de canal e congela o estado; C
exige conectar e provar o handshake antes da lista e depende de `LPRT-03`/`LPRT-06`.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A próxima
da fila era `Q-BOT-3`.

### 3.8 `Q-LCH-4` — resposta registrada: A (`LLCH-04`, fechada)

**Resposta do dono: A** — instalador **Tauri (NSIS)** + **updater Ed25519**, publicado em GitHub Releases
pelo `wyd-ops`; o `/download` lê `portal.launcher_release` e o updater reutiliza o manifesto e a
assinatura de L1/L5.

**Motivo medido.** O launcher é o único programa que o jogador instala e o contrato L6 exige instalador
reproduzível (`wyd-launcher-1.0.0.md:38`); o código novo está vazio (`crates/wyd-launcher/.gitkeep`) e o
`release.yml` não existe (`wyd-ops/plano-ci-real.md:69`). O legado tinha Tauri com `bundle.active: false`
e sem updater (`deprecated/crates/wyd-launcher/ui/tauri.conf.json:27–29`; `ui/Cargo.toml:17`), e o
instalador NSIS continuava pendente (`deprecated/docs/cliente/launcher.md:133–134`). L1/L5 já entregam
manifesto e assinatura; falta o continente.

**O que fechou.** `LLCH-04` e o contrato L6; o launcher fica com **0 lacunas abertas**. **1 lacuna + 1
contrato.**

**Custo do que A ainda exige, não implementado.** 1 crate Tauri em `crates/wyd-launcher` com 6 arquivos
mínimos, 1 `.github/workflows/release.yml`, 1 feed `latest.json`, 1 secret de assinatura e 1 rota
`/download` em `wyd-portal`; 0 migration. O gate ainda deve `RF-LCH` e árvore de `src/` (itens 3–4).

**Custo da resposta oposta.** B não atualiza nem confere versão e exige reinstalar a base para adotar
updater; C deixa a `1.0.0` sem artefato e adia todo o caminho de A.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A próxima
da fila era `Q-BOT-3`.

### 3.9 `Q-BOT-3` — resposta registrada: A (`LBOT-03` aberta como medição)

**Resposta do dono: A** — **medir e congelar** o limiar na primeira execução no CI efêmero; nenhum
número entra no gate antes da medição. O cenário sobe em degraus, mede P50/P99, sucesso e recusas por
motivo; saturação é o primeiro degrau em que as recusas deixam de ser zero ou o P99 cresce sem voltar ao
patamar anterior. O valor medido é congelado com ambiente/data; ao atingi-lo, o bot para de subir,
drena e falha com o relatório de `Q-BOT-4`.

**Motivo medido.** O plano diz que "limiar é medido, não estimado" (`wyd-bots/docs/index.md`) e o gate da
onda E exige "cenário de carga medido" (`wyd-1.0.0.md` §7.7). Não há conexões, latência nem taxa de
sucesso medidas (`wyd-bots/docs/proposta-decisoes-bots.md` §2). Fixar número agora seria afirmação sem
`V`, contra `AGENTS.md` §5, item 4. B mede infraestrutura, não o cenário; C inventa número.

**O que fechou.** A **política** do contrato B4; **não fechou `LBOT-03`**, que segue aberta como medição
até a primeira execução, depois da fatia do `wyd-server` e da política de `Q-NET-1`. **0 fechadas + 1 política
+ 1 medição dependente.**

**Custo do que A ainda exige, não implementado.** 0 migration, 0 tabela, 0 coluna; 1 página de spec, 3
arquivos em `crates/wyd-bots` (`Cargo.toml`, `src/scenario.rs`, CLI), 1 `Cargo.toml` de workspace na raiz
e 1 job de CI — **6 arquivos novos**, além do seed de `Q-BOT-2`.

**Se a resposta for a oposta.** B dá teto imediato, mas de infraestrutura, e pode bloquear jogador
legítimo; C vira promessa sem fonte. O retrabalho é substituir o número na primeira medição e explicar
por que o gate anterior não valia.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**; a lacuna só fecha com medição, depois da
fatia do `wyd-server`. A próxima pergunta era `Q-NET-1`.

### 3.10 `Q-NET-1` — resposta registrada: A (`LNET-01` aberta como medição)

**Resposta do dono: A** — **medir e congelar** os valores, com a **forma** do contrato N6 decidida agora:
balde por conexão, porte por IP, teto total por shard e timeout de handshake/ociosidade; ao estourar,
recusa tipada (`RateLimited`, `TooManyPerIp`, `TooManyTotal`, timeout), contador do motivo e fechamento da
conexão quando a recusa é de conexão/timeout. O cenário de `Q-BOT-3` declara o **perfil de teste** ou usa
origens distintas, para não medir o limite por IP como se fosse saturação do shard.

**Motivo medido.** Não há conexões, latência nem abuso medidos (`wyd-net/docs/proposta-decisoes-net.md`
§2), e `wyd-1.0.0.md` §7.6 registra 0 testes e 0 medições. `LNET-01` está listada como bloqueio de
ligação em `wyd-1.0.0.md` §7.5. Fixar número agora seria afirmação sem `V` (`AGENTS.md` §5, item 4). B trata
provisório como regra; C fixa número sem fonte.

**O que fechou.** A **política** do contrato N6; **não fechou `LNET-01`**, que segue aberta como medição
até a primeira execução, depois da fatia do `wyd-server`. **0 fechadas + 1 política + 1 medição
dependente.** `LBOT-03` continua medição, mas passa a ser interpretável quando o perfil de teste for
declarado.

**Custo do que A ainda exige, não implementado.** 0 migration, 0 tabela, 0 coluna; 1 spec, `Cargo.toml`
do `crates/wyd-net`, `src/lib.rs`, `src/limits/rate.rs`, `src/metrics.rs`, teste de flood e `Cargo.toml` de
workspace — **7 arquivos**, podendo chegar a 8 com `src/session/mod.rs`.

**Se a resposta for a oposta.** B pode bloquear jogador legítimo com um teto de infraestrutura; C vira
promessa sem fonte. O retrabalho é refazer o gate quando a primeira medição chegar.

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**; a lacuna fecha só com medição, depois da
fatia do `wyd-server`. A próxima pergunta era `LAC-POR-09`.

### 3.11 `LAC-POR-09` — resposta registrada: A (`LAC-POR-09`, fechada)

**Resposta do dono: A** — o **BFF do portal** fecha o leilão vencido, com rota interna autenticada,
`SKIP LOCKED`, uma transação por anúncio e chamada idempotente ao `wyd-admin`; o **`wyd-ops`** agenda e
alerta. Se a liquidação de um anúncio falhar, o lote não aborta os demais; o anúncio continua `ativo`
mesmo vencido, sem `market_transaction` e sem devolução de custódia, e o próximo ciclo o reencontra. A
`Idempotency-Key` e `uq_market_transaction_listing` impedem cobrança dupla; o atraso acumulado gera
métrica e alerta. A cadência e o limiar de atraso são medição da F9.

**Motivo medido.** `LAC-POR-01`/`P-17` fixam que só o BFF escreve `portal` e o portal não escreve
`game`; `M25` fixou o dreno com `FOR UPDATE SKIP LOCKED` em transação curta; D4 recusou fechar no
acesso; `RF-POR-025` e `RNF-POR-009` fixam o resultado e a idempotência
(`wyd-portal/docs/proposta-decisoes-portal.md` §1–§4; `wyd-portal/lacunas.md` §2). B exigiria worker com
escrita em `portal` e `GRANT` novo; C faria o admin escrever `portal`, contra `M28`; D já foi recusada.

**O que fechou.** `LAC-POR-09`, a única proposta do portal, e a última resposta pendente do dono. O
portal fica em **0 abertas, 0 propostas e 9 fechadas**. A F9 continua pendente de implementação (BFF,
agendador, alerta, `0034` aplicado e `seeds/dev.sql`); a decisão não implementa nada.

**Custo.** **0 migrations novas, 0 tabelas/colunas novas.** A implementação mínima tem **5 arquivos**:
2 no BFF, 1 teste, 1 definição de agendamento e 1 regra de alerta no `wyd-ops`; usa as 11 tabelas do
`0034` e as 4 rotas C4 já planejadas.

**Se a resposta fosse a oposta.** B reabriria a fronteira, criaria segundo escritor e exigiria `GRANT`
aprovado no `wyd-db`; C/D contrariam `LAC-POR-01`/D4. O retrabalho seria desfazer o segundo escritor e
reconciliar o estado de mercado.

**Próximo item.** A implementação de **A1**, autorizada em 2026-11-23; `LAC-POR-09` libera a F9, não a
captura de fio.

## 4. Itens fora da rodada de decisão (não são escolha do dono)

Estes cinco compõem os itens que não são pergunta técnica entre os 7 não fechados de §1. Nenhum vira
pergunta de desenho; o que falta é medição, execução ou direito.

| ID | Por que não é pergunta técnica | O que falta | O que desbloqueia |
|---|---|---|---|
| `LPRT-03` | medição de fio: origem da `KeyWord` no handshake | captura A1 datada + segunda leitura | as páginas de fio `header.md`, `enquadramento.md` e `keyword-e-checksum.md` (ainda **não escritas**); passo 1 do esqueleto |
| `LPRT-06` | medição de fio: encoding e terminador de texto | captura B/C com conta sintética (`seeds/dev.sql`) | nome de personagem e chat; passo 3 do esqueleto |
| `LCLI-02` | depende da lista do patch e de `Q-IP-1`/`Q-IP-2` (direitos) | inventário do patch e liberação de direitos | entrada do manifesto (`LLCH-01`) e patch |
| `LCLI-03` | é o **nome** do QA que assina a segunda leitura; D6 já decidiu o papel | primeira ficha assinada (QA: **Lucas Cantarelli**, dono do produto, que não é o operador; o nome do operador entra na ficha) | segunda leitura da captura |
| `LCLI-05` | jurídico (`Q-IP-1`–`Q-IP-6`), sem recomendação técnica | responsável nominal (**Lucas Cantarelli**), distribuição congelada; `Q-IP-3`–`Q-IP-6` decididas; `Q-IP-1`/`Q-IP-2` com pedido externo iniciado, inventário e pacote prontos, pedido não enviado e sem evidência; **segue proposta**; `LCLI-02` fechada em 2026-11-25 | patch, release e publicação de conteúdo do cliente |

`LPRT-03` fecha com a captura A1; `LPRT-06` depende das capturas B/C e do seed sintético. Até lá, as
duas continuam `abertas` e sem valor suposto (`wyd-1.0.0.md` §6.4).

## 5. Autorização da captura A1 — não é pergunta de desenho

**Pergunta decidível.** Autorizar a implementação do peer da opção A (passo 0 do `wyd-server`), do
`ECAP` e da CLI de captura, e a execução da captura A1?

**Recomendação única: autorizar agora, depois das respostas de §3.** A captura é a única coisa que
remove o bloqueio do fio; a autorização é uma palavra e o trabalho é de implementação.

**Motivo medido.** A spec do `ECAP` tem desenho e DoD fechados (`wyd-server/docs/specs/endpoint-de-captura.md`
§8), e a medição de 2026-11-23 reprovou a condição da opção C: `deprecated/*` não é versionado
(`git ls-files deprecated` = 0), o boot exige `MIGRATIONS_DIR` fora do default e o gate espera 12000
contra o cliente 7640/7662 (`endpoint-de-captura.md` §9.1). Por isso A1 depende da **opção A** — peer no
`wyd-server`. O esqueleto lista o bloqueio e a ordem de fechamento (`esqueleto` §7 e §9), e
`wyd-1.0.0.md` §7.5 registra que "a onda jogável não começa antes do handshake provado". `AGENTS.md`
§5.1 proíbe código antes da página aceita, e §5.9 proíbe issue/commit sem pedido: **a autorização é o
que falta**, não o desenho.

**O que desbloqueia.** Implementação do peer do passo 0, do `ECAP` e da CLI; captura A1 e os 3 goldens
do fio; fecha `LPRT-03` e dá base para `LPRT-06` (com B/C); destrava `X12`, a onda C, os offsets de
payload por `Type`, o handshake de `net`/`server` e o replay do `bots`. **1 lacuna de medição + 1
preparada (`LPRT-06`) + 4 frentes.**

**Se a resposta for "não agora".** O fio continua `H`, a onda C não começa, `LPRT-03`/`LPRT-06`
continuam abertas e `X12` continua com 0 capturas. O custo é de calendário, não de retrabalho: o desenho
já está pronto e a implementação não muda por esperar. O risco de esperar é a onda C inteira parada
atrás do mesmo bloqueio.

**Esforço e momento.** **Depois da rodada de §3**, como `wyd-1.0.0.md` §6.5 já fixou: não é a primeira
resposta porque custa um ciclo de implementação (peer, `ECAP`, CLI, captura e segunda leitura), não uma
palavra de desenho. A autorização foi dada em 2026-11-23; a implementação pode começar pelo recorte de
11 arquivos descrito acima.

**Resposta do dono em 2026-11-23: A — autorizar agora.** O recorte autorizado é peer A com handshake
mínimo local, `ECAP`, CLI e execução da captura. Nenhuma lacuna fecha nesta resposta; `LPRT-03` só fecha
após a captura completa e a segunda leitura, e `LPRT-06` continua dependente das capturas B/C e do seed
sintético. **Custo: 11 arquivos autorais** (1 workspace, 6 no `wyd-server`, 4 em `wyd-client/tools`),
0 migrations, 0 tabelas/colunas, 0 workflows e 0 mudanças de gate.

## 6. Ordem por desbloqueio ÷ esforço

Critério: quantas lacunas fecham e quantas frentes/perguntas param de esperar por unidade de esforço de
**resposta** (não de implementação). Empatados, primeiro o que é pré-requisito do outro.

| Ordem | Resposta | Fecha | Destrava | Depende de | Esforço de resposta |
|---:|---|---|---|---|---|
| — | `Q-NET-4` = A | `LNET-04` | `X10` implementável | nada | **já decidida por regra** |
| — | `Q-BOT-4` = A | `LBOT-04` | gate do `bots` cumprível | nada | **já decidida por regra** |
| concluída | `Q-SIM-3` = A | `LSIM-03` | local/formato dos vetores decidido; implementação de fixtures continua pendente | — | resposta explícita do dono |
| concluída | `Q-SIM-2` = A | `LSIM-02` | versão do fixture, reprodutibilidade | — | resposta explícita do dono |
| concluída | `Q-BOT-1` = A | `LBOT-01` | `Q-BOT-2`, `Q-BOT-3`, replay; `X13` completo com `Q-BOT-2` | — | resposta explícita do dono |
| concluída | `Q-BOT-2` = A | `LBOT-02` | seed do bot, replay, `seeds/dev.sql`, `X13` decidido | `Q-BOT-1` | resposta explícita do dono |
| concluída | `Q-LCH-1` = A | `LLCH-01` | `Q-LCH-3`, `Q-LCH-4`, `X11` parcial | — | resposta explícita do dono |
| concluída | `Q-LCH-3` = A | `LLCH-03` | `Q-LCH-4`, `X11` decidida | `Q-LCH-1` | resposta explícita do dono |
| concluída | `Q-LCH-2` = A | `LLCH-02` | serverlist, contrato portal → launcher, `Q-LCH-4` | nada | resposta explícita do dono |
| concluída | `Q-LCH-4` = A | `LLCH-04` | launcher sem lacuna; artefato de release (onda E) | `Q-LCH-3` | resposta explícita do dono |
| concluída (política) | `Q-BOT-3` = A | política; `LBOT-03` segue medição | limiar medido | `Q-BOT-1` | resposta explícita do dono |
| concluída (política) | `Q-NET-1` = A | política; `LNET-01` segue medição | limites medidos | ambiente de `Q-BOT-1` | resposta explícita do dono |
| concluída | `LAC-POR-09` = A | `LAC-POR-09` | F9 do portal | `wyd-ops` | resposta explícita do dono |
| concluída | A1 = A | — (autorização; `LPRT-03` só fecha após captura e segunda leitura) | `LPRT-03`; prepara `LPRT-06`, `X12` e a onda C | implementação | resposta explícita do dono em 2026-11-23 |

**Sequência de respostas pendentes:** **dez recomendações aceitas em 2026-11-25** — `LPRT-03`, `LPRT-06`, `A11`, `A20`, `M21`, `LBOT-03`, `LNET-01`, `LCLI-02`, `LCLI-03` e `LCLI-05`, opção A de cada bloco, com o aceite nos `lacunas.md` donos; A1 foi autorizada em 2026-11-23 e a
próxima ação é implementação, não pergunta. `Q-LCH-1` = A, `Q-LCH-2` = A, `Q-LCH-3` = A, `Q-LCH-4` = A,
`Q-BOT-1` = A, `Q-BOT-2` = A, `Q-BOT-3` = A, `Q-NET-1` = A, `Q-SIM-2` = A, `Q-SIM-3` = A, `LAC-POR-09` = A
e A1 = A já foram respondidas; `LBOT-03` e `LNET-01` seguem abertas como medição.

**Maior desbloqueio total:** a autorização de A1, já dada em 2026-11-23 — 1 lacuna de medição
(`LPRT-03`), o caminho de
`LPRT-06`, `X12`, a onda C e o handshake de `net`/`server`. `Q-SIM-2`/`Q-SIM-3` já resolveram
documentalmente a versão e o local dos vetores, e `Q-BOT-1`/`Q-BOT-2` fixaram ambiente e conta do replay;
as decisões por si não implementam fórmulas, fixtures nem job, e não provam o gate B.

**Próximo item: implementação de A1.** Não há mais pergunta de desenho pendente; A1 foi autorizada e
fecha `LPRT-03` após captura e segunda leitura, preparando `LPRT-06`, `X12` e a onda C. `LPRT-06`
continua dependente das capturas B/C e do seed sintético; `LNET-01` e `LBOT-03` continuam abertas como
medição.

**Estado após `LAC-POR-09` = A:** `118 = 6 abertas + 1 proposta + 108 fechadas + 3 externas`; os **7 não
fechados** eram `LNET-01`, `LBOT-03`, `LPRT-03`, `LPRT-06`, `LCLI-02`, `LCLI-03` e `LCLI-05` (6 abertas +
1 proposta), além das 3 externas `A11`, `A20`, `M21`. **Depois da execução de `LCLI-02` (2026-11-25):**
`118 = 6 abertas + 0 proposta + 109 fechadas + 3 externas`.

## 7. Contagens e registro

| Medida | Antes do briefing | Após Q-NET-4/Q-BOT-4 | Após Q-SIM-3 | Após Q-SIM-2 | Após Q-BOT-1 | Após Q-BOT-2 | Após Q-LCH-1 | Após Q-LCH-3 | Após Q-LCH-2 | Após Q-LCH-4 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Lacunas totais | 118 | 118 | 118 | 118 | 118 | 118 | 118 | 118 | 118 | 118 |
| Abertas | 15 | 13 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 |
| Propostas | 3 | 3 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| Fechadas | 97 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 |
| Externas (`A11`, `A20`, `M21`) | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 |

**Somas:** estado anterior `118 = 15 + 3 + 97 + 3`; após Q-NET-4/Q-BOT-4 `118 = 13 + 3 + 99 + 3`;
após Q-SIM-3 = A, `118 = 13 + 2 + 100 + 3`; após Q-SIM-2 = A, `118 = 12 + 2 + 101 + 3`; após Q-BOT-1 = A,
`118 = 11 + 2 + 102 + 3`; após Q-BOT-2 = A, `118 = 10 + 2 + 103 + 3`; após Q-LCH-1 = A,
`118 = 9 + 2 + 104 + 3`; após Q-LCH-3 = A, `118 = 8 + 2 + 105 + 3`; após Q-LCH-2 = A,
`118 = 7 + 2 + 106 + 3`; após Q-LCH-4 = A, `118 = 6 + 2 + 107 + 3`; após `LAC-POR-09` = A,
`118 = 6 + 1 + 108 + 3`; após a execução de `LCLI-02` (2026-11-25), `118 = 6 + 0 + 109 + 3`. Os 6 não
fechados são `6 abertas + 0 proposta`; `0 têm pergunta de desenho isolada pendente` (a de `LCLI-02` foi
executada), `2 são medições com política decidida` e `4 não são pergunta técnica`. **`Q-NET-1` = A não
alterou as contagens**; `LAC-POR-09` = A foi o último delta de proposta e `LCLI-02` o de fechamento
(`0 abertas / −1 proposta / +1 fechada`).

**A1 = A em 2026-11-23** não alterou nenhuma contagem: é autorização de implementação, não fechamento de
lacuna. `LPRT-03` só entra em fechadas depois da captura completa e da segunda leitura; `LPRT-06` continua
aberta até B/C com seed sintético.

**Nomeações de 2026-11-23.** O dono assumiu a segunda leitura (`LCLI-03`; **Lucas Cantarelli**, dono do
produto, que **não é o operador**; o operador entra na ficha) e a responsabilidade nominal por direitos
(`LCLI-05`), com distribuição congelada. Também decidiu as políticas de `Q-IP-3`–`Q-IP-6` e iniciou o
pedido externo de `Q-IP-1`/`Q-IP-2`; o inventário item a item (`wyd-client/docs/specs/inventario-da-arvore.md`)
e o pacote do pedido (`wyd-client/docs/specs/pedido-ao-titular.md`) existem, mas o pedido não foi enviado. Nenhuma dessas decisões altera as contagens: `LCLI-03` só fecha com a primeira ficha
assinada e `LCLI-05` só fecha com evidência do titular; `LCLI-05` continua proposta e `LCLI-02` foi executada e fechada em 2026-11-25.

**Onde cada fechamento foi registrado.**

| Lacuna | Registro | Evidência |
|---|---|---|
| `LNET-04` | `wyd-net/lacunas.md` §2; `wyd-net/docs/proposta-decisoes-net.md` §4 e §6; `wyd-net/docs/index.md` | `X10`; `LAC-POR-06`; `AGENTS.md` §5, item 7 |
| `LBOT-04` | `wyd-bots/lacunas.md` §2; `wyd-bots/docs/proposta-decisoes-bots.md` §4 e §6; `wyd-bots/docs/index.md` | `wyd-bots-1.0.0.md` §6, itens 4–5; `AGENTS.md` §5, item 7; `P-22` |
| `LSIM-03` | `wyd-sim/lacunas.md` §2; `wyd-sim/docs/proposta-decisoes-sim.md` §4 e §6; `wyd-sim/docs/index.md` | resposta explícita do dono `Q-SIM-3` = A; formato/local fechados, fixtures não implementadas |
| `LSIM-02` | `wyd-sim/lacunas.md` §2; `wyd-sim/docs/proposta-decisoes-sim.md` §4 e §6; `wyd-sim/docs/index.md`; `wyd-sim/wyd-sim-1.0.0.md` §3 | resposta explícita do dono `Q-SIM-2` = A; `M3` decidido, versão documentada e fórmula não escrita |
| `LBOT-01` | `wyd-bots/lacunas.md` §2; `wyd-bots/docs/proposta-decisoes-bots.md` §4 e §6; `wyd-bots/docs/index.md`; `wyd-bots/wyd-bots-1.0.0.md` §3 e §6 | resposta explícita do dono `Q-BOT-1` = A; B2 decidido, job de CI não implementado |
| `LBOT-02` | `wyd-bots/lacunas.md` §2; `wyd-bots/docs/proposta-decisoes-bots.md` §4 e §6; `wyd-bots/docs/index.md`; `wyd-bots/wyd-bots-1.0.0.md` §3 | resposta explícita do dono `Q-BOT-2` = A; B3 e B5 decididos, `seeds/dev.sql` não implementado |
| `LLCH-01` | `wyd-launcher/lacunas.md` §2; `wyd-launcher/docs/proposta-decisoes-launcher.md` §4 e §6; `wyd-launcher/docs/index.md`; `wyd-launcher/wyd-launcher-1.0.0.md` §3 e §6 | resposta explícita do dono `Q-LCH-1` = A; L1 decidido e item 1 do gate cumprido, manifesto não implementado |
| `LLCH-03` | `wyd-launcher/lacunas.md` §2; `wyd-launcher/docs/proposta-decisoes-launcher.md` §4 e §6; `wyd-launcher/docs/index.md`; `wyd-launcher/wyd-launcher-1.0.0.md` §3 e §6 | resposta explícita do dono `Q-LCH-3` = A; L5 decidido, item 2 do gate cumprido e `X11` decidida, assinatura não implementada |
| `LLCH-02` | `wyd-launcher/lacunas.md` §2; `wyd-launcher/docs/proposta-decisoes-launcher.md` §4 e §6; `wyd-launcher/docs/index.md`; `wyd-launcher/wyd-launcher-1.0.0.md` §3 | resposta explícita do dono `Q-LCH-2` = A; L2 decidido, rota/BFF e cache não implementados |
| `LLCH-04` | `wyd-launcher/lacunas.md` §2; `wyd-launcher/docs/proposta-decisoes-launcher.md` §4 e §6; `wyd-launcher/docs/index.md`; `wyd-launcher/wyd-launcher-1.0.0.md` §3 | resposta explícita do dono `Q-LCH-4` = A; L6 decidido, instalador/updater/release não implementados |
| `LAC-POR-09` | `wyd-portal/lacunas.md` §2 e §3; `wyd-portal/docs/proposta-decisoes-portal.md` §4–§5; `wyd-portal/docs/index.md`; `wyd-portal/armaria-e-mercado.md` D4 | resposta explícita do dono `LAC-POR-09` = A; BFF fecha, `wyd-ops` agenda/alerta e falha é reentrante |

Nenhuma lacuna sumiu: as onze fechadas nesta rodada têm data e documento dono; `LNET-04`/`LBOT-04`
foram derivadas de regras vigentes e
`LSIM-02`/`LSIM-03`/`LBOT-01`/`LBOT-02`/`LLCH-01`/`LLCH-02`/`LLCH-03`/`LLCH-04`/`LAC-POR-09` foram
fechadas pela resposta explícita do dono. Nenhuma fechada foi confundida com implementação das fixtures,
do job de CI, do manifesto, da assinatura, da lista de canais, do instalador ou do fechamento de leilão.

## 8. Registro de respostas da rodada

| Item | Recomendação registrada | Resposta / estado |
|---|---|---|
| `Q-SIM-3` | fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, entrada, saída esperada e identificador da versão de fórmula | A, respondida pelo dono em 2026-11-23 |
| `Q-SIM-2` | A — versão de fórmula própria, registrada com o `content_version` | A, respondida pelo dono em 2026-11-23 |
| `Q-BOT-1` | A — CI efêmero com servidor descartável | A, respondida pelo dono em 2026-11-23 |
| `Q-BOT-2` | A — seed declarativo em `crates/wyd-db/seeds/dev.sql`, banco recriado por execução | A, respondida pelo dono em 2026-11-23 |
| `Q-LCH-1` | A — JSON versionado e canônico, com `sha256` por arquivo | A, respondida pelo dono em 2026-11-23 |
| `Q-LCH-3` | A — assinatura Ed25519 verificada antes de aplicar | A, respondida pelo dono em 2026-11-23 |
| `Q-LCH-2` | A — portal/publicação com cache local | A, respondida pelo dono em 2026-11-23 |
| `Q-LCH-4` | A — instalador NSIS + updater Ed25519, publicado pelo `wyd-ops` | A, respondida pelo dono em 2026-11-23 |
| `Q-BOT-3` | A — medir e congelar; nenhum número antes da medição | A, respondida pelo dono em 2026-11-23; `LBOT-03` aberta como medição |
| `Q-NET-1` | A — medir e congelar; nenhum limite antes da medição | A, respondida pelo dono em 2026-11-23; `LNET-01` aberta como medição |
| `LAC-POR-09` | A — BFF fecha; `wyd-ops` agenda e alerta; falha é reentrante | A, respondida pelo dono em 2026-11-23; `LAC-POR-09` fechada |
| A1 | A — peer A com handshake mínimo local + `ECAP` + CLI + captura | A, respondida pelo dono em 2026-11-23; implementação e captura pendentes; `LPRT-03` e `LPRT-06` continuam abertas |
| `Q-IP-3`–`Q-IP-6` | A — repo público só com hash/metadado/referência; registro por item; GPL v2 com ativos excluídos; nenhuma marca/screenshot até autorização | A, respondidas pelo dono em 2026-11-23; políticas registradas na página própria; `LCLI-05` segue proposta |
| `Q-IP-1`/`Q-IP-2` | pedido de proveniência e autorização escrita ao titular **iniciado**; inventário item a item e pacote prontos, pedido **não enviado**; `Q-IP-2` = **B (árvore inteira)**, decidido em 2026-11-23 | não é resposta de evidência; sem titular, `LCLI-05` continua proposta (a lista de `LCLI-02` foi entregue em 2026-11-25) |

As duas linhas de `Q-IP` são registro da frente de direitos e **não alteram a contagem das 16 perguntas**
acima; `Q-IP-1`/`Q-IP-2` seguem sem resposta do titular, com `Q-IP-2` no escopo B (árvore inteira).

## 9. O que este briefing não decide

- Não implementa `ECAP`, CLI, peer, job, launcher, fórmula ou teste; não cria issue, ADR, commit ou
  release (`AGENTS.md` §5, itens 1 e 9).
- Não escolhe produto, escopo, balanceamento nem direito autoral; não inventa coeficiente, número de
  carga, opcode, byte ou conteúdo de cliente.
- Não substitui o `lacunas.md` de cada sistema como dono do estado, nem `wyd-1.0.0.md` §6.3 como dono da
  fila; aponta, não redefine.
- Não reabre nada decidido: `Q-NET-2`, `Q-SIM-1`, `Q-SIM-2`, `Q-SIM-3`, `Q-SIM-4`, `Q-BOT-1`,
  `Q-BOT-2`, `Q-BOT-3`, `Q-NET-1`, `Q-LCH-1`, `Q-LCH-2`, `Q-LCH-3`, `Q-LCH-4`, `Q-SRV-5` (C com condição reprovada),
  `Q-PRT-1`, `LNET-03`, `LAC-POR-01`, `LAC-POR-09`, `M25` e `D4` continuam como estão.
- Não promete data: a ordem é dependência declarada nas fontes citadas, não cronograma.

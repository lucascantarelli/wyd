# Briefing de decisões da `1.0.0` — uma passada, todas as perguntas abertas

> Pergunta: "qual resposta fecha mais lacuna e libera mais frente por unidade de esforço, o que cada uma desbloqueia e o que custa a resposta oposta?"
> Público: dono do produto, arquitetura, `wyd-ops` e QA
> Dono: arquitetura (este briefing); a fila oficial é `wyd-1.0.0.md` §6.3, que cita esta página
> Base: `wyd-1.0.0.md` §6.1–§6.3 e §7; `wyd-sim/docs/proposta-decisoes-sim.md`; `wyd-launcher/docs/proposta-decisoes-launcher.md`; `wyd-bots/docs/proposta-decisoes-bots.md`; `wyd-net/docs/proposta-decisoes-net.md`; `wyd-portal/docs/proposta-decisoes-portal.md`; `wyd-server/docs/specs/endpoint-de-captura.md` §8–§9.1; `AGENTS.md` §5, §8 e §9
> Revisado em: 2026-11-23 (respostas `Q-SIM-2`, `Q-SIM-3` e `Q-BOT-1` registradas)

**Estado:** proposto (2026-11-23). Este documento **mede o que está aberto e ordena as respostas**; não
implementa, não cria issue, não altera schema e não escolhe produto. Duas lacunas (`LNET-04` e
`LBOT-04`) fecharam por regras vigentes, sem escolha do usuário, e `LSIM-02`/`LSIM-03` e `LBOT-01`
fecharam após as respostas explícitas do dono `Q-SIM-2` = A, `Q-SIM-3` = A e `Q-BOT-1` = A; esses estados
estão registrados nas páginas dona. O briefing mantém as **8 respostas do dono** ainda pendentes e a
autorização de A1, com recomendação única cada. Nenhuma
fixture, fórmula, coeficiente, número de balanceamento, dado de jogo ou conteúdo de cliente é criado aqui.

## 0. Como ler

- Marcas: `V` (verificado por arquivo ou comando, com data), `H` (hipótese, com a lacuna que a fecha),
  `P` (projetado). Presente do indicativo sem `V` é erro de escrita (`AGENTS.md` §5.4).
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
`Q-SIM-3` = A e `Q-BOT-1` = A:

| Sistema | Lacunas | Abertas | Propostas | Fechadas | Externas | IDs não fechados |
|---|---:|---:|---:|---:|---:|---|
| `wyd-protocol` | 20 | 2 | 0 | 18 | 0 | `LPRT-03`, `LPRT-06` |
| `wyd-client` | 5 | 2 | 1 | 2 | 0 | `LCLI-02`, `LCLI-03`, `LCLI-05` |
| `wyd-net` | 4 | 1 | 0 | 3 | 0 | `LNET-01` |
| `wyd-core` | 4 | 0 | 0 | 4 | 0 | — |
| `wyd-sim` | 4 | 0 | 0 | 4 | 0 | — |
| `wyd-server` | 5 | 0 | 0 | 5 | 0 | — |
| `wyd-db` | 38 | 0 | 0 | 35 | 3 | `A11`, `A20`, `M21` (externas) |
| `wyd-admin` | 7 | 0 | 0 | 7 | 0 | — |
| `wyd-portal` | 9 | 0 | 1 | 8 | 0 | `LAC-POR-09` |
| `wyd-launcher` | 4 | 4 | 0 | 0 | 0 | `LLCH-01`–`LLCH-04` |
| `wyd-bots` | 4 | 2 | 0 | 2 | 0 | `LBOT-02`, `LBOT-03` |
| `wyd-docs` | 14 | 0 | 0 | 14 | 0 | — |
| **Total** | **118** | **11** | **2** | **102** | **3** | **13 não fechados** |

**Somas conferidas:** `118 = 11 + 2 + 102 + 3`; os **13 não fechados** são `11 abertos + 2 propostas`.
Desses 13, **8 têm pergunta e recomendação ainda aguardando resposta** (`wyd-launcher` 4, `wyd-bots` 2,
`wyd-net` 1, `wyd-portal` 1) e **5 não são pergunta técnica** (`LPRT-03`, `LPRT-06`,
`LCLI-02`, `LCLI-03`, `LCLI-05`, explicados em §4). `wyd-ops` não tem `lacunas.md` e não entra na soma,
como em `wyd-1.0.0.md` §6.1.

Duas lacunas fecharam por regra vigente (`LNET-04`, `LBOT-04`) e `LSIM-02`/`LSIM-03`/`LBOT-01` fecharam
pelas respostas explícitas do dono (`Q-SIM-2` = A, `Q-SIM-3` = A e `Q-BOT-1` = A). O delta de `Q-BOT-1`, a
última resposta, é `−1 aberta / 0 propostas / +1 fechada`; o total continua 118.

## 2. Decisões derivadas das regras vigentes (sem escolha do usuário)

### 2.1 `Q-NET-4` = A — `LNET-04` fechada

**A pergunta original.** O que o transporte expõe em métrica e log?

**Regras que decidem, medidas no repositório.**

| Regra | O que já fixa | Fonte |
|---|---|---|
| `X10` | a métrica do NOC é `/metrics` no formato Prometheus, coletada pelo `wyd-ops` | `wyd-1.0.0.md` §7.3 |
| `LAC-POR-06` | a telemetria de canal vem de fonte externa; o portal consulta o Prometheus e mostra "não medido" quando falha | `wyd-portal/lacunas.md` §2 |
| `AGENTS.md` §5.7 | segredo, token, senha e PIN nunca aparecem em log; payload e PII não entram | `AGENTS.md` §5.7 |

**Decisão derivada.** O transporte expõe métricas de **conexões, erros por tipo, bytes e latência** em
`/metrics`, e grava log **só de metadados** (evento, sessão, tamanhos, caminho), nunca payload, credencial
ou PII. Nome, rótulo e unidade de cada métrica são implementação, não decisão pendente.

**Por que não há escolha.** A opção C ("sem métrica/log") contradiz `X10`, que é contrato decidido, e
`LAC-POR-06`, que já escolheu a fonte externa; log de payload contradiz `AGENTS.md` §5.7. Resta o
conjunto A. Não é preferência de arquitetura: é a única combinação compatível com três decisões
vigentes.

**Efeito registrado.** `LNET-04` fecha em `wyd-net/lacunas.md`; o `wyd-net` passa a
`4 lacunas / 1 aberta / 0 propostas / 3 fechadas`; `Q-NET-4` sai da fila do dono; o contrato `X10`
ganha conteúdo de implementação. `LNET-01` **não** fecha: continua aberta como medição (§3.10).

**O que teria custado a resposta oposta.** Escolher C exigiria reabrir `X10` e `LAC-POR-06`; aceitar log
de payload exigiria revogar `AGENTS.md` §5.7. Nenhuma das duas é decisão desta rodada, e as duas
deixariam o NOC sem a métrica que o portal já publica.

### 2.2 `Q-BOT-4` = A — `LBOT-04` fechada

**A pergunta original.** O que uma execução falha entrega: log, captura, trace ou nada?

**Regras que decidem, medidas no repositório.**

| Regra | O que já fixa | Fonte |
|---|---|---|
| gate do `wyd-bots`, item 4 | "Relatório por execução com falha reproduzível (log + captura), sem `sleep` como sincronização" | `wyd-bots/wyd-bots-1.0.0.md` §6 |
| gate do `wyd-bots`, item 5 | "Nenhum segredo de jogador em configuração de bot" | `wyd-bots/wyd-bots-1.0.0.md` §6 |
| `AGENTS.md` §5.7 | segredo/PII nunca em log | `AGENTS.md` §5.7 |
| `P-22` | PII mora só em `portal`; o jogo referencia `person_id`, nunca copia e-mail | `wyd-db/padrao-banco.md` `P-22` |

**Decisão derivada.** Uma execução falha entrega **log estruturado + frame/captura anonimizada no ponto
da falha**, com o cenário, o `ClientTick` e o passo reproduzível; o log não carrega payload, credencial,
nome de personagem nem dado de jogador. Trace da conexão é opcional e, se existir, segue a mesma
higiene.

**Por que não há escolha.** O item 4 do gate já exige log **e** captura: a opção C ("nada além do exit
code") reprova o gate, e B ("só log textual") reprova a parte "captura". O item 5 e `AGENTS.md` §5.7
fixam a higiene. A é o único conjunto que cumpre o gate que o próprio sistema já declarou.

**Efeito registrado.** `LBOT-04` fecha em `wyd-bots/lacunas.md`; o `wyd-bots` passa a
`4 lacunas / 3 abertas / 0 propostas / 1 fechada`; `Q-BOT-4` sai da fila do dono. `LBOT-01`, `LBOT-02` e
`LBOT-03` continuam abertas (§3.3, §3.7 e §3.9).

**O que teria custado a resposta oposta.** C deixaria a falha sem diagnóstico e reprovaria o item 4; B
não provaria o estado do fio. Em ambos os casos o gate do sistema teria de ser rebaixado — o que
contraria `AGENTS.md` §8 (nenhum `SKIP` silencioso) e `wyd-bots-1.0.0.md` §6.

## 3. Itens de decisão do dono

§3.1–§3.3 registram, para rastreabilidade, as respostas já dadas a `Q-SIM-3`, `Q-SIM-2` e `Q-BOT-1`;
§3.4–§3.11 são as 8 respostas ainda pendentes, na ordem recomendada por §6. Cada item traz a recomendação única;
as alternativas ficam registradas na página dona, não como menu.

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
não tem mais pergunta aberta; a resposta seguinte da rodada é `Q-BOT-1`.

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
em `crates/wyd-bots` e a captura A1 (`LPRT-03`/`LPRT-06`).

**Esforço e momento.** **Respondida pelo dono em 2026-11-23**, decisão documental, sem código. A próxima
da fila é `Q-BOT-2`.

### 3.4 `Q-BOT-2` — como nasce a conta e o personagem sintéticos? (`LBOT-02`, aberta)

**Pergunta decidível.** A conta e o personagem de cada cenário nascem de fixture/seed no banco efêmero,
com reset por execução, ou de conta criada pela API do portal?

**Recomendação única: A** — fixture/seed sintético no banco efêmero do cenário, com reset por execução.

**Motivo medido.** O contrato B3 já diz "fixture, não conta real" e "nenhum dado de jogador"
(`wyd-bots/wyd-bots-1.0.0.md` §3), o que exclui a conta real; `M19` foi decidida em 2026-11-19 (senha de
dev fixa, documentada no seed) e `seeds/dev.sql` ainda não existe (`wyd-db/lacunas.md` §2.3;
`esqueleto` §7). `AGENTS.md` §5.7 e `P-22` mantêm PII fora do jogo. B acopla o bot ao portal, que ainda
não existe, e pode deixar estado persistido se o reset falhar.

**O que desbloqueia.** Fecha `LBOT-02`; libera o seed de bot e o replay reprodutível; o caminho de
`seeds/dev.sql` (já decidido em `M19`) deixa de ser promessa e vira fixture do cenário. **1 lacuna + 1
seed + o replay do `bots`.**

**Se a resposta for a oposta.** B exercita o fluxo real, mas depende do portal e deixa estado
persistido; a fixture continuaria necessária para o CI. O retrabalho é manter dois caminhos de criação
(portal e seed) e limpar banco depois de cada falha.

**Esforço e momento.** **Responder logo depois de `Q-BOT-1`**: a localização e o reset da fixture
dependem do ambiente escolhido. Sem essa ordem, a resposta fica sem chão.

### 3.5 `Q-LCH-1` — qual é o formato do manifesto? (`LLCH-01`, aberta)

**Pergunta decidível.** O manifesto do patch é JSON versionado com `sha256` por arquivo, um formato
binário compacto, ou o manifesto do legado?

**Recomendação única: A** — JSON versionado com lista de arquivos, `sha256` por arquivo, versão do patch
e compatibilidade.

**Motivo medido.** `X11` está em lacuna e é o contrato `ops` → `launcher` → `client`
(`wyd-1.0.0.md` §7.3), e §7.5 lista "patch e manifesto" como bloqueio. O portal já decidiu que publica
versão, notas e **URL do manifesto assinado** em tabela própria (`LAC-POR-03`, `wyd-portal/lacunas.md`
§3), o que exige um manifesto auditável e assinável. B é difícil de revisar e evoluir sem ferramenta; C
não tem prova de que cobre hash, versão e compatibilidade (`wyd-launcher/lacunas.md` `LLCH-01`).

**O que desbloqueia.** Fecha `LLCH-01`; **libera duas perguntas dependentes** — `Q-LCH-3` (assinatura
sobre o manifesto) e `Q-LCH-4` (pacote assinado) —, destrava `X11`, a implementação do launcher e a
entrada `LCLI-02` (lista do patch). **1 lacuna + 2 perguntas dependentes + 1 contrato.**

**Se a resposta for a oposta.** B perde auditabilidade e exige ferramenta para qualquer revisão; C
começa de um formato conhecido, mas sem prova de hash/versão/compatibilidade. O retrabalho é reescrever
manifesto, canonização e verificador quando o primeiro patch real precisar de integridade por arquivo.

**Esforço e momento.** **Responder agora.** É a raiz da cadeia do launcher; sem ela, `Q-LCH-3` e
`Q-LCH-4` não têm objeto.

### 3.6 `Q-LCH-3` — o manifesto é assinado? (`LLCH-03`, aberta)

**Pergunta decidível.** O manifesto é assinado (Ed25519) e verificado antes de aplicar, tem só
`sha256`, ou não tem assinatura?

**Recomendação única: A** — assinatura destacada do manifesto, verificada antes de aplicar, com chave
pública no launcher.

**Motivo medido.** O risco declarado é execução de código arbitrário por patch adulterado
(`wyd-launcher/lacunas.md` `LLCH-03`). `X11` é o contrato do patch (`wyd-1.0.0.md` §7.3), e
`AGENTS.md` §5.7 fixa a higiene de segredo/chave. B detecta corrupção, mas não autentica a origem; C
aceita patch adulterado.

**O que desbloqueia.** Fecha `LLCH-03`; libera `Q-LCH-4` (a assinatura do manifesto se aplica ao
pacote) e fecha a integridade autenticada de `X11`. **1 lacuna + 1 pergunta dependente + 1 contrato.**

**Se a resposta for a oposta.** B deixa a origem sem autenticação; C aceita troca de arquivo. O
retrabalho é criar chave, cerimônia de assinatura, canonização e verificador depois — e reinstalar o
launcher já distribuído.

**Esforço e momento.** **Responder depois de `Q-LCH-1`**: a assinatura precisa de um formato definido
para assinar.

### 3.7 `Q-LCH-2` — de onde vem a lista de canais? (`LLCH-02`, aberta)

**Pergunta decidível.** A lista de canais e o estado de cada um vêm do portal/publicação com cache
local, de arquivo estático na release, ou do próprio servidor no login?

**Recomendação única: A** — portal/publicação como fonte, com cache local do launcher e estado por
canal.

**Motivo medido.** O portal já tem `/admin/canais` no planejamento (rota 37 de
`wyd-portal/docs/arquitetura/rotas-e-permissoes.md`) e publica metadados do launcher em tabela própria
(`LAC-POR-03`, `wyd-portal/lacunas.md` §3). A topologia é do `wyd-server`, e o launcher só consome
(`wyd-launcher/docs/index.md`). B fica preso à release; C mostra a lista só depois de conectar.

**O que desbloqueia.** Fecha `LLCH-02`; libera o contrato de leitura portal → launcher, o cache offline
e a tela de canais do launcher. **1 lacuna + 1 contrato de leitura + 1 tela.**

**Se a resposta for a oposta.** B exige uma release para cada mudança de canal; C inverte a ordem e
esconde a lista antes do login. O retrabalho é acrescentar cache e invalidação depois — o mesmo custo,
sem o benefício agora.

**Esforço e momento.** **Responder agora ou junto de `Q-LCH-1`.** É independente das outras perguntas do
launcher.

### 3.8 `Q-LCH-4` — como o aplicativo é empacotado e distribuído? (`LLCH-04`, aberta)

**Pergunta decidível.** Instalador assinado com atualização do launcher, zip/portable manual, ou nada
nesta versão?

**Recomendação única: A** — instalador assinado + atualização do launcher, publicados pelo `wyd-ops`.

**Motivo medido.** `wyd-launcher/wyd-launcher-1.0.0.md` §2 define o launcher como aplicação Tauri que
coloca o cliente no estado certo; `AGENTS.md` §9 limita a operação ao gratuito e proíbe produção em free
tier; `X11` é o contrato `ops` → `launcher` → `client` (`wyd-1.0.0.md` §7.3). A assinatura de `Q-LCH-3`
se aplica ao pacote. B não atualiza nem confere versão; C não entrega release.

**O que desbloqueia.** Fecha `LLCH-04`; libera o artefato de release da onda E e o fluxo de atualização.
**1 lacuna + 1 artefato de release.**

**Se a resposta for a oposta.** B não atualiza e não verifica versão; C não publica nada. O retrabalho é
construir pipeline, assinatura e rollback quando a primeira release for necessária — com o agravante de
já haver launcher instalado sem caminho de atualização.

**Esforço e momento.** **Responder depois de `Q-LCH-3`**, porque a assinatura do pacote depende da
decisão de assinatura do manifesto.

### 3.9 `Q-BOT-3` — qual limiar de carga reprova? (`LBOT-03`, aberta)

**Pergunta decidível.** Qual é o limiar de conexões, duração e latência que reprova a carga?

**Recomendação única: A** — medir primeiro em ambiente controlado; **nenhum limiar numérico entra no
gate antes da medição**. A primeira medição vira o limiar, com data.

**Motivo medido.** O plano do sistema diz que "limiar é medido, não estimado" (`wyd-bots/docs/index.md`)
e o gate da onda E exige "cenário de carga medido" (`wyd-1.0.0.md` §7.7). Não há conexões, latência nem
taxa de sucesso medidas (`wyd-bots/docs/proposta-decisoes-bots.md` §2). Fixar um número agora seria
afirmação sem `V`, contra `AGENTS.md` §5.4. B mede infraestrutura, não o cenário; C inventa número.

**O que desbloqueia.** Fixa a política do gate; **`LBOT-03` continua aberta como medição** — o número
entra depois, no ambiente de `Q-BOT-1`. Não fecha lacuna hoje; fecha a pergunta "como o número entra".
**0 fechadas + 1 política + 1 medição dependente.**

**Se a resposta for a oposta.** B dá um teto imediato, mas de infraestrutura, e pode bloquear jogador
legítimo; C vira promessa sem fonte. O retrabalho é substituir o número na primeira medição e explicar
por que o gate anterior não valia.

**Esforço e momento.** **Responder agora para fixar a política**, mas a lacuna só fecha com medição —
depois de `Q-BOT-1`. Não bloqueia nada além da própria medição.

### 3.10 `Q-NET-1` — qual limite, timeout e rate-limit? (`LNET-01`, aberta)

**Pergunta decidível.** Quais valores de limite por conexão, timeout e rate-limit entram no gate?

**Recomendação única: A** — medir primeiro em ambiente controlado; limite, timeout e rate-limit só
entram no gate depois da medição, com data.

**Motivo medido.** Não há conexões, latência nem abuso medidos (`wyd-net/docs/proposta-decisoes-net.md`
§2), e `wyd-1.0.0.md` §7.6 registra 0 testes e 0 medições. `LNET-01` está listada como bloqueio de
ligação em `wyd-1.0.0.md` §7.5. Fixar número agora seria afirmação sem `V` (`AGENTS.md` §5.4). B trata
provisório como regra; C fixa número sem fonte.

**O que desbloqueia.** Fixa a política; **`LNET-01` continua aberta como medição** e depende do ambiente
controlado de `Q-BOT-1`. **0 fechadas + 1 política + 1 medição dependente.**

**Se a resposta for a oposta.** B pode bloquear jogador legítimo com um teto de infraestrutura; C vira
promessa sem fonte. O retrabalho é refazer o gate quando a primeira medição chegar.

**Esforço e momento.** **Responder agora para fixar a política**; a lacuna fecha só com medição, depois
de `Q-BOT-1`.

### 3.11 `LAC-POR-09` — dono do fechamento de leilão (`LAC-POR-09`, proposta)

**Pergunta decidível.** Aprovar que o **BFF do portal** fecha o leilão vencido e que o **`wyd-ops`**
agenda e alerta (A)?

**Recomendação única: A** — rota interna autenticada do BFF, com `SKIP LOCKED`, uma transação por
anúncio e chamada idempotente ao `wyd-admin`; o `wyd-ops` agenda e alerta.

**Motivo medido.** `LAC-POR-01`/`P-17` fixam que só o BFF escreve `portal` e o portal não escreve
`game`; `M25` fixou o dreno com `FOR UPDATE SKIP LOCKED` em transação curta; D4 recusou fechar no
acesso; `RF-POR-025` e `RNF-POR-009` fixam o resultado e a idempotência
(`wyd-portal/docs/proposta-decisoes-portal.md` §1–§4; `wyd-portal/lacunas.md` §2). B exigiria worker com
escrita em `portal` e `GRANT` novo; C faria o admin escrever `portal`, contra `M28`; D já foi recusada.

**O que desbloqueia.** Fecha a única proposta do portal; libera a F9 (fechamento de leilão e devolução
de custódia) e uma rota agendada + alerta no `wyd-ops`. **1 lacuna + 1 fase do portal.**

**Se a resposta for a oposta.** B reabre a fronteira, cria segundo escritor e exige `GRANT` aprovado no
`wyd-db`; C/D contrariam `LAC-POR-01`/D4. O retrabalho é desfazer o segundo escritor e reconciliar
estado de mercado.

**Esforço e momento.** **Pode ser adiada sem bloquear a rodada**: a F9 é da onda D/F. Precisa do "sim"
do dono de operação (`wyd-ops`), que é aprovação de execução, não escolha entre alternativas.

## 4. Itens fora da rodada de decisão (não são escolha do dono)

Estes cinco compõem os itens que não são pergunta técnica entre os 13 não fechados de §1. Nenhum vira
pergunta de desenho; o que falta é medição, execução ou direito.

| ID | Por que não é pergunta técnica | O que falta | O que desbloqueia |
|---|---|---|---|
| `LPRT-03` | medição de fio: origem da `KeyWord` no handshake | captura A1 datada + segunda leitura | `header.md`, `enquadramento.md` e `keyword-e-checksum.md`; passo 1 do esqueleto |
| `LPRT-06` | medição de fio: encoding e terminador de texto | captura B/C com conta sintética (`seeds/dev.sql`) | nome de personagem e chat; passo 3 do esqueleto |
| `LCLI-02` | depende da lista do patch e de `Q-IP-1`/`Q-IP-2` (direitos) | inventário do patch e liberação de direitos | entrada do manifesto (`LLCH-01`) e patch |
| `LCLI-03` | é o **nome** do QA que assina a segunda leitura; D6 já decidiu o papel | designação nominal | segunda leitura da captura |
| `LCLI-05` | jurídico (`Q-IP-1`–`Q-IP-6`), sem recomendação técnica | titular dos direitos e revisão jurídica | patch, release e publicação de conteúdo do cliente |

`LPRT-03` e `LPRT-06` são as duas medições que a captura A1 (e as capturas seguintes) fecha; até lá
continuam `abertas` e sem valor suposto (`wyd-1.0.0.md` §6.4).

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
payload por `Type`, o handshake de `net`/`server` e o replay do `bots`. **2 lacunas de medição + 4
frentes.**

**Se a resposta for "não agora".** O fio continua `H`, a onda C não começa, `LPRT-03`/`LPRT-06`
continuam abertas e `X12` continua com 0 capturas. O custo é de calendário, não de retrabalho: o desenho
já está pronto e a implementação não muda por esperar. O risco de esperar é a onda C inteira parada
atrás do mesmo bloqueio.

**Esforço e momento.** **Depois da rodada de §3**, como `wyd-1.0.0.md` §6.5 já fixou: não é a primeira
resposta porque custa um ciclo de implementação (peer, `ECAP`, CLI, captura e segunda leitura), não uma
palavra de desenho. Não começar sem a autorização explícita.

## 6. Ordem por desbloqueio ÷ esforço

Critério: quantas lacunas fecham e quantas frentes/perguntas param de esperar por unidade de esforço de
**resposta** (não de implementação). Empatados, primeiro o que é pré-requisito do outro.

| Ordem | Resposta | Fecha | Destrava | Depende de | Esforço de resposta |
|---:|---|---|---|---|---|
| — | `Q-NET-4` = A | `LNET-04` | `X10` implementável | nada | **já decidida por regra** |
| — | `Q-BOT-4` = A | `LBOT-04` | gate do `bots` cumprível | nada | **já decidida por regra** |
| concluída | `Q-SIM-3` = A | `LSIM-03` | local/formato dos vetores decidido; implementação de fixtures continua pendente | — | resposta explícita do dono |
| concluída | `Q-SIM-2` = A | `LSIM-02` | versão do fixture, reprodutibilidade | — | resposta explícita do dono |
| concluída | `Q-BOT-1` = A | `LBOT-01` | `Q-BOT-2`, `Q-BOT-3`, replay; `X13` segue lacuna por `LBOT-02` | — | resposta explícita do dono |
| 1 | `Q-BOT-2` = A | `LBOT-02` | seed do bot, replay, `seeds/dev.sql` | `Q-BOT-1` | uma palavra |
| 2 | `Q-LCH-1` = A | `LLCH-01` | `Q-LCH-3`, `Q-LCH-4`, `X11` | nada | uma palavra |
| 3 | `Q-LCH-3` = A | `LLCH-03` | `Q-LCH-4`, integridade de `X11` | `Q-LCH-1` | uma palavra |
| 4 | `Q-LCH-2` = A | `LLCH-02` | serverlist e contrato portal → launcher | nada | uma palavra |
| 5 | `Q-LCH-4` = A | `LLCH-04` | artefato de release (onda E) | `Q-LCH-3` | uma palavra |
| 6 | `Q-BOT-3` = A | política; `LBOT-03` segue medição | limiar medido | `Q-BOT-1` | uma palavra |
| 7 | `Q-NET-1` = A | política; `LNET-01` segue medição | limites medidos | ambiente de `Q-BOT-1` | uma palavra |
| 8 | `LAC-POR-09` = A | `LAC-POR-09` | F9 do portal | `wyd-ops` | uma aprovação |
| 9 | A1: autorizar | — | `LPRT-03`, `LPRT-06`, `X12`, onda C | implementação | uma autorização + ciclo |

**Sequência de respostas pendentes:** `Q-BOT-2` → `Q-LCH-1` → `Q-LCH-3` → `Q-LCH-2` → `Q-LCH-4` →
`Q-BOT-3` → `Q-NET-1` → `LAC-POR-09` → autorização A1. `Q-BOT-1` = A, `Q-SIM-2` = A e `Q-SIM-3` = A já
foram respondidas e não aparecem como pendência.

**Maior desbloqueio total:** a autorização de A1 — 2 lacunas de medição, `X12`, a onda C e o handshake
de `net`/`server`. `Q-SIM-2` e `Q-SIM-3` já resolveram documentalmente a versão e o local dos vetores,
mas as decisões por si não implementam fórmulas nem provam o gate B.

**Próxima resposta única recomendada: `Q-BOT-2` = A.** Ela fecha `LBOT-02`, define onde a fixture mora e
como reseta, converte `seeds/dev.sql` em fixture do cenário e é pré-requisito do replay completo e de
`Q-BOT-3`. A captura A1 continua sendo a decisão que libera a onda C; exige autorização explícita e um
ciclo de implementação, portanto permanece separada da resposta de desenho.

**Projeção se as 8 respostas ainda pendentes seguirem a recomendação (`P`):** 6 lacunas fecham
(`LLCH-01`–`04`, `LBOT-02`, `LAC-POR-09`) e 2 continuam abertas como medição
(`LNET-01`, `LBOT-03`). O total projetado será `118 = 6 abertas + 1 proposta + 108 fechadas + 3 externas`;
os 7 não fechados restantes serão `LNET-01`, `LBOT-03`, `LPRT-03`, `LPRT-06`, `LCLI-02`, `LCLI-03` e
`LCLI-05`.

## 7. Contagens e registro

| Medida | Antes do briefing | Após Q-NET-4/Q-BOT-4 | Após Q-SIM-3 | Após Q-SIM-2 | Após Q-BOT-1 |
|---|---:|---:|---:|---:|---:|
| Lacunas totais | 118 | 118 | 118 | 118 | 118 |
| Abertas | 15 | 13 | 13 | 12 | 11 |
| Propostas | 3 | 3 | 2 | 2 | 2 |
| Fechadas | 97 | 99 | 100 | 101 | 102 |
| Externas (`A11`, `A20`, `M21`) | 3 | 3 | 3 | 3 | 3 |

**Somas:** estado anterior `118 = 15 + 3 + 97 + 3`; após Q-NET-4/Q-BOT-4 `118 = 13 + 3 + 99 + 3`;
após Q-SIM-3 = A, `118 = 13 + 2 + 100 + 3`; após Q-SIM-2 = A, `118 = 12 + 2 + 101 + 3`; após Q-BOT-1 = A,
`118 = 11 + 2 + 102 + 3`. Os 13 não fechados são `11 abertas + 2 propostas`; `8 têm pergunta ainda
pendente` e `5 não são pergunta técnica`; as 8 respostas restantes incluem 6 fechamentos e 2 políticas de
medição.

**Onde cada fechamento foi registrado.**

| Lacuna | Registro | Evidência |
|---|---|---|
| `LNET-04` | `wyd-net/lacunas.md` §2; `wyd-net/docs/proposta-decisoes-net.md` §4 e §6; `wyd-net/docs/index.md` | `X10`; `LAC-POR-06`; `AGENTS.md` §5.7 |
| `LBOT-04` | `wyd-bots/lacunas.md` §2; `wyd-bots/docs/proposta-decisoes-bots.md` §4 e §6; `wyd-bots/docs/index.md` | `wyd-bots-1.0.0.md` §6, itens 4–5; `AGENTS.md` §5.7; `P-22` |
| `LSIM-03` | `wyd-sim/lacunas.md` §2; `wyd-sim/docs/proposta-decisoes-sim.md` §4 e §6; `wyd-sim/docs/index.md` | resposta explícita do dono `Q-SIM-3` = A; formato/local fechados, fixtures não implementadas |
| `LSIM-02` | `wyd-sim/lacunas.md` §2; `wyd-sim/docs/proposta-decisoes-sim.md` §4 e §6; `wyd-sim/docs/index.md`; `wyd-sim/wyd-sim-1.0.0.md` §3 | resposta explícita do dono `Q-SIM-2` = A; `M3` decidido, versão documentada e fórmula não escrita |
| `LBOT-01` | `wyd-bots/lacunas.md` §2; `wyd-bots/docs/proposta-decisoes-bots.md` §4 e §6; `wyd-bots/docs/index.md`; `wyd-bots/wyd-bots-1.0.0.md` §3 e §6 | resposta explícita do dono `Q-BOT-1` = A; B2 decidido, job de CI não implementado |

Nenhuma lacuna sumiu: as cinco fechadas nesta rodada têm data e documento dono; `LNET-04`/`LBOT-04`
foram derivadas de regras vigentes e `LSIM-02`/`LSIM-03`/`LBOT-01` foram fechadas pela resposta explícita
do dono. Nenhuma fechada foi confundida com implementação das fixtures nem do job de CI. A mudança de
Q-SIM-3 é o delta `0 abertas / −1 proposta / +1 fechada`; as de Q-SIM-2 e Q-BOT-1 são o delta `−1 aberta /
0 propostas / +1 fechada` da tabela.

## 8. Registro de respostas da rodada

| Item | Recomendação registrada | Resposta / estado |
|---|---|---|
| `Q-SIM-3` | fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, entrada, saída esperada e identificador da versão de fórmula | A, respondida pelo dono em 2026-11-23 |
| `Q-SIM-2` | A — versão de fórmula própria, registrada com o `content_version` | A, respondida pelo dono em 2026-11-23 |
| `Q-BOT-1` | A — CI efêmero com servidor descartável | A, respondida pelo dono em 2026-11-23 |
| `Q-BOT-2` | A — fixture/seed no banco efêmero, com reset | pendente |
| `Q-LCH-1` | A — JSON versionado com `sha256` por arquivo | pendente |
| `Q-LCH-3` | A — assinatura Ed25519 verificada antes de aplicar | pendente |
| `Q-LCH-2` | A — portal/publicação com cache local | pendente |
| `Q-LCH-4` | A — instalador assinado com atualização | pendente |
| `Q-BOT-3` | A — medir primeiro; nenhum número antes da medição | pendente |
| `Q-NET-1` | A — medir primeiro; nenhum limite antes da medição | pendente |
| `LAC-POR-09` | A — BFF fecha; `wyd-ops` agenda e alerta | pendente |
| A1 | autorizar peer A + `ECAP` + CLI + captura | autorização pendente |

## 9. O que este briefing não decide

- Não implementa `ECAP`, CLI, peer, job, launcher, fórmula ou teste; não cria issue, ADR, commit ou
  release (`AGENTS.md` §5.1, §5.9).
- Não escolhe produto, escopo, balanceamento nem direito autoral; não inventa coeficiente, número de
  carga, opcode, byte ou conteúdo de cliente.
- Não substitui o `lacunas.md` de cada sistema como dono do estado, nem `wyd-1.0.0.md` §6.3 como dono da
  fila; aponta, não redefine.
- Não reabre nada decidido: `Q-NET-2`, `Q-SIM-1`, `Q-SIM-2`, `Q-SIM-3`, `Q-SIM-4`, `Q-BOT-1`,
  `Q-SRV-5` (C com condição reprovada),
  `Q-PRT-1`, `LNET-03`, `LAC-POR-01`, `M25` e `D4` continuam como estão.
- Não promete data: a ordem é dependência declarada nas fontes citadas, não cronograma.

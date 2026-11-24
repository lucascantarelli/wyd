# Decisões do `wyd-sim` — fórmulas, versão, vetores e aritmética

> Pergunta: "quais fórmulas existem, como a versão acompanha o conteúdo, onde ficam os vetores de
> prova e ponto flutuante é permitido?"
> Público: produto, arquitetura, `wyd-sim`, `wyd-core`, `wyd-db` e QA
> Dono: equipe do `wyd-sim`
> Base: `lacunas.md` (`LSIM-01`–`LSIM-04`); `docs/index.md`; `wyd-sim-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §7.1, §7.3 (X4) e §7.6; `wyd-db/padrao-banco.md` `P-07`; `AGENTS.md` §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Nenhuma pergunta em aberto.** As quatro lacunas fecharam pela
resposta do dono em 2026-11-23: `Q-SIM-1` = A (`LSIM-01`), `Q-SIM-2` = A (`LSIM-02`), `Q-SIM-3` = A
(`LSIM-03`) e `Q-SIM-4` = A (`LSIM-04`), com registro em §4 e §6. Nada foi implementado: não há fórmula,
fixture nem execução; nenhum coeficiente é medido.

## 1. Fronteira

**Fecha:** as quatro perguntas do sistema, respondidas pelo dono em 2026-11-23: a lista de fórmulas
(`LSIM-01`, `Q-SIM-1` = A), a versão da fórmula (`LSIM-02`, `Q-SIM-2` = A), a localização/formato dos
vetores (`LSIM-03`, `Q-SIM-3` = A) e a política aritmética (`LSIM-04`, `Q-SIM-4` = A), registradas em §4
e §6. As fixtures continuam sem implementação; a implementação segue as quatro decisões, não as escolhe.

**Não decide:** quando a fórmula é aplicada (`wyd-core`), valor de balanceamento (produto), carga do
dado (`wyd-db`), formatação (`wyd-portal`/`wyd-protocol`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **0 abertas, 0 propostas, 4 fechadas** | `lacunas.md` §1 |
| fórmulas escritas | **0** | `wyd-sim-1.0.0.md` §4 |
| vetores de prova | **0** | `wyd-1.0.0.md` §7.6 |
| código | **0** (só `.gitkeep`) | `wyd-sim-1.0.0.md` §4 |
| desempenho de cálculo | **não medido** | `docs/index.md` |
| contrato X4 | fórmula determinística: **decidido** (`LSIM-01`, `LSIM-02`, `LSIM-04`, 2026-11-23) | `wyd-1.0.0.md` §7.3 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| dinheiro inteiro | `P-07`: dinheiro é inteiro no banco | `wyd-db/padrao-banco.md` `P-07` | a aritmética de economia não usa `f64` |
| cálculo puro | `wyd-sim` é cálculo determinístico; `wyd-core` aplica | `wyd-1.0.0.md` §7.1 | a fronteira é a pergunta, não o princípio |
| cobertura e anti-skip | 85% por sistema, sem `SKIP` silencioso | `AGENTS.md` §8 | vetor de prova não pode ser pulado |

## 4. Decisões

### Q-SIM-1 — Quais fórmulas existem na 1.0.0?

**Medido.** Não havia catálogo funcional nem fórmula escrita; `LSIM-01` fechou em 2026-11-23 pela
resposta abaixo, e a cobertura da 1.0.0 continua "não medido" (`wyd-1.0.0.md` §1.5). Nada foi
executado.

**O que está em jogo.** "Todas as fórmulas do legado" não tem lista; esperar o catálogo bloqueia;
uma fatia vertical prova o contrato com o core e cresce.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fatia vertical: dano físico, chance de acerto, drop e uma fórmula de economia, cada uma com vetor | prova o contrato X4 e a aritmética; cobertura parcial declarada |
| B | Lista completa do legado | cobre mais; sem catálogo, o tamanho e a prova são indefinidos |
| C | Esperar o catálogo | nenhum retrabalho; o sim fica sem fórmula |

**Recomendação: A.** É o menor conjunto que prova determinismo e a fronteira com o core; o catálogo
depois acrescenta. **Custo:** cobertura parcial. **Fontes:** `lacunas.md` `LSIM-01`;
`wyd-1.0.0.md` §1.5 e X4; `wyd-sim-1.0.0.md` §6. **Recusa B** por lista inexistente; **C** por
bloquear.

**Resposta do dono em 2026-11-23: A.** A `1.0.0` começa pela fatia vertical, cada fórmula com vetor de
entrada e saída: **dano físico, chance de acerto, drop, uma fórmula de economia e o deslocamento puro**
que o passo 6 do esqueleto exige. O deslocamento entra na lista porque é o cálculo do único passo de
jogo da fatia mínima (`wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md` §2, passo 6); sem ele a
fatia fecharia o contrato e não andaria. Cobertura parcial é declarada, e o catálogo funcional
acrescenta fórmulas depois, sem reabrir esta lista.

**Escopo do que fechou:** a decisão fixa **quais** fórmulas existem na fatia. O coeficiente de cada uma
vem do conteúdo, e o exemplo numérico nasce junto do vetor (`LSIM-03`) e da escrita da fórmula; nenhum
número de balanceamento é declarado aqui.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LSIM-01`; o `wyd-sim` passa de 3 para 2 lacunas abertas | `lacunas.md` §1 |
| fecha `X4`: com a lista e a aritmética decididas, o contrato `sim` → `core` deixa de ser proposta | `wyd-1.0.0.md` §7.3 |
| tira o bloqueio da fórmula da lista de §7.5 | `wyd-1.0.0.md` §7.5 |
| define o cálculo do passo 6 (deslocamento puro) | `wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md` §2 |

**Não fecha:** os `RF-SIM` e a escrita das fórmulas com coeficiente de fonte. `LSIM-02` foi decidida
depois, em 2026-11-23 (`Q-SIM-2` = A), e `LSIM-03` também (`Q-SIM-3` = A); as fixtures ainda precisam
ser implementadas.

### Q-SIM-2 — Como a versão da fórmula acompanha o `content_version`?

**Medido.** `game.character` já carrega `content_version`; não há versão de fórmula nem regra de
mudança. `LSIM-02` estava aberta quando esta pergunta foi escrita e fechou em 2026-11-23 (§6).

**O que está em jogo.** Sem versão, mudar coeficiente quebra a reprodutibilidade de um cálculo antigo;
amarrar tudo ao `content_version` mistura dado e código; uma versão própria separa os dois.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | Versão de fórmula própria, registrada com o `content_version` usado | reproduz cálculo antigo; exige campo/registro e disciplina de mudança |
| B | Amarrar a fórmula ao `content_version` | uma chave só; mudar código sem mudar conteúdo fica invisível |
| C | Sem versão | simples; cálculo antigo não é reproduzível |

**Recomendação: A.** Separa o que muda por código do que muda por dado, sem perder a combinação.
**Custo:** um identificador de versão. **Fontes:** `lacunas.md` `LSIM-02`;
`wyd-1.0.0.md` X4; `wyd-db/migrations/0006-game-character.md` §6.1 (`content_version`). **Recusa B** por
misturar; **C** por quebrar reprodutibilidade.

**Resposta do dono em 2026-11-23: A.** Cada fórmula carrega **versão própria** (`formula_version`,
inteiro a partir de 1), independente do conteúdo. Toda saída declara a dupla (`formula_version`,
`content_version` usado no cálculo); a versão nasce na spec da fórmula (`docs/specs/formulas/`) e é
escrita no cabeçalho da fixture decidida em `LSIM-03` (`tests/fixtures/sim/<formula>.json`). `LSIM-02`
fecha com esta escolha.

| Mudou | O que muda |
|---|---|
| código que altera o resultado da fórmula | incrementa `formula_version`; o `content_version` do cálculo pode continuar o mesmo |
| conteúdo (coeficiente, curva, tabela) | `formula_version` continua; o `content_version` vigente identifica o dado usado |

**O que é o `content_version` registrado.** O **número** de `content.content_version.version`
(`INTEGER`), não o `id` da linha única (`content.content_version` tem `CHECK (id = 1)`, `0001` §4).

**Por que A, e não as alternativas.**

| Alternativa | Por que foi descartada |
|---|---|
| B — amarrar a fórmula ao `content_version` | mistura código e dado: correção de fórmula sem mudança de conteúdo fica invisível; e a coluna que existe hoje não carrega o número, então B exigiria antes uma migration |
| C — sem versão | não atribui uma saída antiga à fórmula que a gerou; mudar coeficiente torna o vetor antigo irreprodutível |

**Observação de schema, fora desta decisão.** `game.character.content_version SMALLINT NOT NULL` tem FK
para `content.content_version (id)` e `id` é preso a `1` por `ck_content_version_linha_unica`
(`wyd-db/migrations/0001-bootstrap.md` §4; `0006-game-character.md` §4). A coluna aponta sempre para a
linha única e não distingue versões. Se o personagem deve persistir o número da versão de conteúdo, isso
é decisão do `wyd-db` e não foi tomada aqui; `Q-SIM-2` fixa apenas que o cálculo e o fixture registram o
número.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LSIM-02` | `lacunas.md` §1 |
| `M3` deixa de ser lacuna: o cálculo declara `formula_version` + `content_version` | `wyd-sim-1.0.0.md` §3 |
| preenche o identificador que `LSIM-03` deixou aberto no cabeçalho do fixture | §4, `Q-SIM-3` |
| `X4` fica sem lacuna de versão | `wyd-1.0.0.md` §7.3 |

**Não fecha:** as fórmulas, os `RF-SIM`, as fixtures de `LSIM-03` (arquivos não escritos) e a
persistência histórica do par versão/conteúdo — se houver consumidor, é decisão do `wyd-db`.

### Q-SIM-3 — Onde ficam os vetores de entrada e saída? (`LSIM-03` fechada; A escolhida)

**Medido.** `LSIM-03` fechou em 2026-11-23 pela resposta explícita do dono, `Q-SIM-3` = A; ainda não
há vetor nem fixture implementados (`wyd-1.0.0.md` §7.6 registra "0 vetores"). A decisão fixa o local e
o formato, não cria os arquivos de prova.

**O que está em jogo.** Vetor no código é fácil de perder; fixture versionada é reproduzível; vetor
externo não é auditável no repositório.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | `tests/fixtures/sim/<formula>.json` com entrada, saída esperada e identificador da versão de fórmula, versionado | prova reproduzível e revisável; exige disciplina de atualização |
| B | Vetores inline no teste | simples; dificulta reuso e revisão por fórmula |
| C | Vetores fora do repositório | não polui; não é auditável nem versionado com o código |

**Recomendação A, aceita pelo dono.** É a forma reproduzível que o alvo de teste aceita e permite rodar
a mesma prova em Rust. **Custo:** fixtures por fórmula. **Fontes:**
`lacunas.md` `LSIM-03`; `wyd-1.0.0.md` §7.6; `AGENTS.md` §8. **Recusa B** por baixo reuso; **C** por
não ser auditável.

**Resposta explícita do dono em 2026-11-23: A.** Os vetores ficam em fixtures JSON versionadas em
`tests/fixtures/sim/<formula>.json`, um arquivo por fórmula, com entrada, saída esperada e o identificador
da versão de fórmula. `LSIM-03` fecha com esta escolha; o valor que preenche o identificador foi
definido depois por `LSIM-02` (`Q-SIM-2` = A: `formula_version` + `content_version` usado). A decisão
não cria fixtures nem código.

**Justificativa de prioridade registrada antes da resposta.** `Q-SIM-3` foi priorizada porque era a
única decisão pendente do `sim` necessária ao gate da onda B — "determinismo provado por vetor"
(`wyd-1.0.0.md` §7.7) — depois de fecharem `LSIM-01` e `LSIM-04`. Isso não dizia que A1 não desbloqueia
uma onda: a autorização e implementação da captura A1 destravam a onda C. As demais pendências têm
outras dependências: `LAC-POR-09` é da onda D/F9; `Q-LCH-*` (`X11`) e `Q-BOT-*` (`X13`) são da onda E e
dependem da C; `Q-NET-1` é uma medição, cuja política pode ser indicada sem inventar o número;
`Q-NET-4` já fechou por regra vigente (`X10`/`LAC-POR-06`/`AGENTS.md` §5, item 7). Com a resposta do dono
registrada acima, `Q-SIM-3` deixou de estar pendente; `Q-SIM-2`, a seguinte, foi respondida na mesma data
(§6) e o `sim` não tem mais pergunta aberta.

**Por que A, e não as alternativas.**

| Alternativa | Por que foi descartada |
|---|---|
| B — vetor inline no teste | o mesmo vetor precisa servir ao teste do `sim`, à revisão de QA e ao contrato com o `core`; inline duplica e some quando o teste é refatorado |
| C — vetor fora do repositório | não é versionado nem auditável, e `AGENTS.md` §5, item 4 exige prova verificável por arquivo; a prova ficaria fora do que o gate lê |
| D — só teste de propriedade, sem vetor | não atende ao gate da onda B, que pede determinismo provado por vetor, não propriedade genérica |

**Efeito da resposta registrada em `LSIM-02` e `LSIM-03`.**

| Se a resposta for | `LSIM-03` | `LSIM-02` |
|---|---|---|
| A (escolhida pelo dono) | **fechada**: local/formato de fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e identificador da versão de fórmula; implementação pendente | **fechada depois** em 2026-11-23 por `Q-SIM-2` = A: `formula_version` próprio no cabeçalho, junto do `content_version` usado |
| B (não escolhida) | não se aplica | continua aberta, sem campo de versão versionado por fórmula |
| C (não escolhida) | não se aplica | continua aberta e sem registro de qual versão gerou a saída esperada |

**Fora de escopo.** O conjunto exato de campos de entrada e saída de cada fórmula (nasce na spec de cada
uma, `docs/specs/formulas/`), a escolha do runner de teste e qualquer coeficiente ou número de
balanceamento. A versão que preenche o identificador foi fixada por `LSIM-02` (`Q-SIM-2` = A); se o
personagem deve persistir o número da versão de conteúdo é decisão do `wyd-db`. O insumo que falta para
escrever o primeiro vetor é este: **os campos tipados de entrada e saída por fórmula ainda não estão
documentados**; a proposta fixa o local e o formato do arquivo, não os campos.

### Q-SIM-4 — Ponto flutuante é permitido e com que arredondamento?

**Medido.** Não havia política aritmética; `LSIM-04` fechou em 2026-11-23 pela resposta abaixo; `P-07`
já fixa dinheiro inteiro no banco. A linha de `LSIM-04` citava `P-06`, mas a regra de dinheiro inteiro é
`P-07`; o registro segue a fonte. Nenhum cálculo foi executado.

**O que está em jogo.** `f64` diverge entre plataformas e quebra determinismo; inteiro/fixed-point é
reproduzível, mas exige escala; misturar os dois cria borda.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Inteiro/fixed-point para tudo que afeta estado e economia; `f64` só para exibição, se necessário | determinismo e dinheiro inteiro; exige escala e arredondamento declarado |
| B | `f64` em toda fórmula | simples; divergência entre plataformas e quebra do determinismo |
| C | Tudo inteiro, inclusive dano e curva | determinístico; qualquer fração vira escala inteira e não há representação fracionária |

**Recomendação: A.** `P-07` já decide dinheiro inteiro; a política estende o mesmo critério ao estado.
**Custo:** uma regra de arredondamento (a escala só entra se uma fórmula precisar). **Fontes:**
`lacunas.md` `LSIM-04`;
`wyd-db/padrao-banco.md` `P-07`; `wyd-sim-1.0.0.md` §2. **Recusa B** por determinismo; **C** por não representar fração.

**Resposta do dono em 2026-11-23: A.** A aritmética do `wyd-sim` é inteira em tudo que afeta estado e
economia, com a regra numérica abaixo; `f64` existe só na formatação de exibição.

| Regra | Valor declarado |
|---|---|
| unidade | **inteiro de 64 bits na unidade do domínio**, sem escala global |
| percentual | **bps (1/10000)**, o mesmo tipo que o banco já usa (`rate_bps`, `tax_rate_bps`, `fee_bps`) |
| arredondamento | **meio-para-longe-do-zero**, aplicado uma única vez no fim de cada fórmula |
| divisão | inteira pelo denominador da unidade (10000 para bps), com o mesmo arredondamento |
| overflow | verificado, nunca silencioso |
| dinheiro | unidades menores inteiras, como `P-07` já fixa no banco |
| `f64` | proibido em estado e economia; permitido só na formatação de exibição |

**Por que a escala 1/1000 foi dispensada (`V`, 2026-11-23).** Ela não tem consumidor no domínio: vida,
AC e dano são `INTEGER`/`SMALLINT` em `game.character_scores`
(`wyd-db/migrations/0006-game-character.md`), o dano de arena é `BIGINT` (`0028`), dinheiro é inteiro
na menor unidade e percentual é bps inteiro (`P-07`; `0011`, `0021` e `0034`). Nenhuma migration
escrita tem coluna `NUMERIC`, `REAL`, `DOUBLE` ou `FLOAT` — a busca em `wyd-db/migrations` só encontra
essas palavras em prosa. Chance já é comparação inteira: sorteio em `[0, 10000)` contra o `rate_bps`
do conteúdo, sem divisão. Dano e economia são um produto pelo bps dividido uma vez por 10000. Uma
escala global de 1/1000 obrigaria a converter bps → milésimos e a arredondar duas vezes, criando
divergência sem ganho.

O “fixed-point” da opção A fica reservado à fórmula específica que comprovadamente precisar de
sub-unidade: nesse caso a escala é declarada na spec daquela fórmula (`docs/specs/formulas/`), não como
regra global. Escolher B quebraria o determinismo; C dispensava qualquer regra de arredondamento — a
regra única registrada acima é o que mantém o resultado reproduzível. A decisão fecha `LSIM-04` e não escreve fórmula, coeficiente nem fixture: o local/formato do vetor está decidido em `LSIM-03`, mas a fixture em si continua pendente de implementação.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LSIM-04` | `lacunas.md` §1 |
| remove a metade aritmética do bloqueio de `X4`, que ficou dependendo da lista (`LSIM-01`) | `wyd-1.0.0.md` §7.3 |
| cumpre o item 2 do gate do sistema | `wyd-sim-1.0.0.md` §6 |
| fixa a aritmética do passo 6 do esqueleto (deslocamento puro) | `wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md` §2, passo 6 |
| fixa o critério numérico para o estado do `core`, o inteiro do fio e a exibição do portal | `wyd-db/padrao-banco.md` `P-07`; `AGENTS.md` §8 |

**Não fecha:** a captura (`LPRT-03`/`LPRT-06`). `LSIM-02` fechou na mesma data (`Q-SIM-2` = A) e `LSIM-03` fechou
quanto à localização/formato; escrever as fixtures continua pendente de implementação.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LSIM-01` | `Q-SIM-1` | fechada em 2026-11-23 (A): fatia vertical com vetor por fórmula — dano físico, chance de acerto, drop, uma fórmula de economia e o deslocamento puro do passo 6; coeficiente e exemplo nascem com o vetor (`LSIM-03`). |
| `LSIM-02` | `Q-SIM-2` | fechada em 2026-11-23 (A): `formula_version` próprio por fórmula, declarado na spec e registrado no cabeçalho do fixture junto do número de `content_version` usado; mudança de código incrementa a versão, mudança de conteúdo mantém e é registrada pelo `content_version`. |
| `LSIM-03` | `Q-SIM-3` | fechada em 2026-11-23 (A, resposta explícita do dono): fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e identificador da versão de fórmula; implementação pendente. |
| `LSIM-04` | `Q-SIM-4` | fechada em 2026-11-23 (A): aritmética inteira em estado e economia, percentual em bps, sem escala global (1/1000 dispensada por falta de consumidor no schema), arredondamento meio-para-longe-do-zero uma vez por fórmula e overflow verificado; `f64` só em exibição. |

## 6. Registro da decisão

**As quatro perguntas respondidas em 2026-11-23.** `LSIM-01`–`LSIM-04` fecharam na opção A. As respostas
`Q-SIM-2` e `Q-SIM-3` fecharam decisões documentais — versão da fórmula e local/formato dos vetores —, não
implementaram fórmulas, vetores ou fixtures. O briefing atualizado (`../../briefing-de-decisoes-1.0.0.md`
§6) registra `Q-SIM-2` e `Q-SIM-3` concluídas e as remove da fila pendente.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-SIM-1` | `LSIM-01` | A — fatia vertical com vetor por fórmula: dano físico, chance de acerto, drop, uma fórmula de economia e o deslocamento puro do passo 6 (2026-11-23) |
| `Q-SIM-2` | `LSIM-02` | A — versão de fórmula própria (`formula_version`), registrada com o `content_version` usado no cálculo; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-SIM-3` | `LSIM-03` | A — fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e identificador da versão de fórmula; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-SIM-4` | `LSIM-04` | A — aritmética inteira em estado e economia, percentual em bps, sem escala global (1/1000 dispensada em 2026-11-23), arredondamento meio-para-longe-do-zero uma vez por fórmula; `f64` só em exibição |

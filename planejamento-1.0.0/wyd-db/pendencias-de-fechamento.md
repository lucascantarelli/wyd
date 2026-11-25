# Pendências de fechamento do `wyd-db`

> Pergunta: "o que ainda falta para o planejamento do `wyd-db` ser considerado fechado, quem responde por cada item e o que cada item desbloqueia?"
> Público: dono, arquitetura, operação, jurídico, curadoria de conteúdo e equipe do `wyd-db`
> Dono: `wyd-db` mantém o registro; cada linha nomeia o dono da decisão
> Base: `lacunas.md`; `proposta-paginas.md` §4–§8; `migrations-checklist.md` §2–§3; `proposta-decisoes-m33.md`; `proposta-retencao-pendente.md`; `proposta-decisoes-capacidade.md`; `plano-migrations.md` §4 e §6; `ddl-confirmado.md` §5.9
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-23). Este é um **registro de fechamento**, não uma nova lacuna e não um
plano de execução. Ele reúne o que as páginas donas já declaram, com o identificador de cada item, e
separa três coisas que estavam misturadas na fila: decisão do dono, ato de aceite e trabalho de redação
que ainda depende de medição ou de fonte externa.

O registro fica na raiz de `planejamento-1.0.0/wyd-db/`, ao lado de `lacunas.md` e
`proposta-paginas.md`, e **não entra no censo de 144 páginas autorais de `docs/`**. Ele não cria
lacuna, não renumera ID e não altera as contagens de `lacunas.md`.

## 1. Onde a frente está (medido em 2026-11-24)

| Medida | Hoje | Fonte |
|---|---|---|
| lacunas do `wyd-db` | **38**: 0 abertas, 0 propostas, 35 fechadas, 3 externas (`A11`, `A20`, `M21`) | `lacunas.md` §1 |
| páginas autorais de `docs/` | **144**: 143 com corpo, 0 escrevíveis, 0 de recorte, **1 bloqueada** (`planos/metricas.md`) | `proposta-paginas.md` §2–§6; §6.6.19 |
| migrations por arquivo | **34**: 28 com página, **6 seeds bloqueados** | `migrations-checklist.md` §2 e §4 |
| páginas de seção de `migrations/` | `index.md` e `serie-e-regras.md` escritos; `serie-e-regras.md` `aceito` em 2026-11-25 | `migrations/index.md`; `migrations/serie-e-regras.md`; `wyd-db-1.0.0.md` §11 |
| aceite de página de migration | **28 de 28 escritas** `aceito` (2026-11-24: 27 na leva `migrations/`, `0002` desde 2026-11-20); as 6 de seed seguem por escrever | `proposta-decisoes-m33.md` §2 e §4; §6.6.31 |
| aceite pendente no sistema | **141 atos**: 141 em `docs/` + **0** em `src/` + **0** em `migrations/` + **0** em `tests/` (12 de `tests/`, 37 de `src/` e 27 de `migrations/` aceitas em 2026-11-24) | §6.6.22.1, §6.6.27 e §6.6.31 (remedido em 2026-11-24) |
| decisões de retenção | **7 propostas, 7 resolvidas**, **0 abertas** (`Q-RET-5` por evidência; `Q-RET-1`, `Q-RET-2` e `Q-RET-4` pelo dono em 2026-11-23; `Q-RET-6` e `Q-RET-7` pelo dono em 2026-11-24; `Q-RET-3` pelo dono em 2026-11-24, opção A) | `proposta-retencao-pendente.md` §4 e §9 |
| decisões de capacidade | **0 abertas**; `Q-CAP-1`, `Q-CAP-2` e `Q-CAP-3` decididas em 2026-11-23; a matriz de `Q-CAP-2` está aplicada na página | `proposta-decisoes-capacidade.md` §4 e §9 |
| páginas de migration bloqueadas por dado | **6**: `0016`–`0018`, `0020`, `0023`, `0027` | `migrations-checklist.md` §3 |
| issues do roadmap | **40** com DoD, **0** criadas no GitHub; board com 40 itens | `docs/planos/riscos.md` `R-5`; §6.6.20 |
| páginas de `tests/` | **12 de 12** escritas e **12** `aceito` (2026-11-24); matriz `CAP-nn` com **39** capacidades e **13** cobertas por teste executado | §6.6.21, §6.6.23 e §6.6.30; `plano-de-testes.md` §5 |
| arquivos `.sql` da série | **28**, aplicados do zero como `wyd_migrator` em `postgres:17`; 13 cenários `sistema_pg` verdes | §6.6.30; `ls migrations/0*.sql` |

**O que não mudou:** as 38 lacunas continuam 0/0/35/3; `M33` continua fechada como decisão e
`M32` continua fechada; `A11`, `A20` e `M21` continuam `externa`. Este registro não reabre nenhuma
delas.

**Fontes que não podem determinar prazo:** `migrations/serie-e-regras.md` foi escrita e aceita em
2026-11-25, mas não fixa prazo de retenção; `planos/metricas.md` continua pendente de medição. Os seis
seeds estão nas migrations, mas não carregam prazo de retenção.

## 2. O que significa "fechar" nesta frente

Há dois limiares diferentes, e a fila não deve confundi-los:

| Limiar | Critério verificável | O que ainda falta |
|---|---|---|
| **Decisão da frente** | nenhuma pergunta de retenção/capacidade sem resposta; nenhuma categoria `pendente` sem dono; nenhuma pendência do `wyd-db` dependendo de suposição | **0 `Q-RET` abertas**; 3 lacunas externas permanecem externas; `Q-DP-2/validação` continua pendente como validação separada |
| **Documentação 1.0.0** | as 144 páginas de `docs/` escritas e `aceito`/`retirado`; 34 páginas de migration + `serie-e-regras`; `planos/metricas.md` medido; todos os links e gates verdes | 7 páginas bloqueadas; 6 seeds; `metricas.md`; aceite página a página |

O primeiro limiar é o que esta rodada ataca. O segundo depende do primeiro e de execução da série;
não pode ser declarado fechado só porque as decisões foram tomadas.

## 3. Decisões do dono (0 abertas)

As opções e a recomendação de cada pergunta estão na página dona citada. **`Q-RET-3`, `Q-RET-6` e
`Q-RET-7` foram respondidas em A em 2026-11-24; não resta pergunta de retenção sem resposta.** A
resposta de `Q-RET-3` foi **A — ciclo de vida da pessoa, sem expurgo por tempo**; ela foi tomada sem
parecer jurídico e **não** transforma a dispensa em validação jurídica. `Q-DP-2/validação` continua
pendente como registro separado. O custo é estimativa de planejamento, em sessões de trabalho e horas
de escrita/migration; não há medição de tempo no repositório.

**Leitura de alavancagem:** as sete decisões de retenção estão fechadas no papel; `Q-RET-3` deixou de
ser bloqueio de retenção e passou a ser **1 tabela + 1 tela** a aplicar, sem índice nem job. `Q-RET-6`
já decidiu 11 tabelas e deixou 1 índice e 1 job em aplicação; `Q-RET-7` fechou a política de backup e
deixou a rotina/restore em aplicação. `Q-RET-1`, `Q-RET-2` e `Q-RET-4` já decidiram 3 categorias, mas a
migration e o expurgo ainda não existem. `Q-CAP-1` já decidiu a página `proposto`; `Q-CAP-2` decidiu e
aplicou a matriz; `Q-CAP-3` decidiu sinal + dono + condição, sem limiar. Não resta decisão de capacidade
nem de retenção. A 5ª onda não fecha antes de `M20`.

### 3.1 Resolvidas

| ID | Estado | Prova ou decisão |
|---|---|---|
| `Q-RET-3` | **resolvida pelo dono em 2026-11-24** — A: ciclo de vida da pessoa; guardar enquanto a linha de `person` existir, sem expurgo por tempo | `docs/proposta-retencao-pendente.md` §Q-RET-3 e §9 registram a resposta e a restrição de projeto experimental sem gasto/apoio jurídico; a página `arquitetura/retencao-e-capacidade.md` e `proposta-dados-pessoais-e-retencao.md` §5.2 passam a linha para decidida. Nenhum job foi criado; `Q-DP-2/validação` continua pendente e **não** foi tratada como parecer |
| `Q-RET-5` | **resolvida em 2026-11-23** — `ops.config_revision` e `ops.config_change` são append-only e **sem purge** | `ddl-confirmado.md` §5.9: o dono aprovou em 2026-11-20 as quatro escolhas, incluindo "histórico append-only e **sem purge**; voltar atrás é revisão nova, nunca `UPDATE` nem `DELETE`"; a regra foi aplicada em `proposta-dados-pessoais-e-retencao.md` §5.2 e em `proposta-retencao-pendente.md` §4 |
| `Q-RET-1` | **resolvida pelo dono em 2026-11-23** — A: 30 dias após `expires_at`, usado ou não, com expurgo diário | `docs/proposta-retencao-pendente.md` §9 registra a resposta; `ddl-confirmado.md` §4 fixa `expires_at`, `used_at` e o índice parcial; `proposta-dados-pessoais-e-retencao.md` §5.2 e `lacunas.md` `M32` foram atualizados. O índice completo e o `DELETE` ainda não existem. |
| `Q-RET-2` | **resolvida pelo dono em 2026-11-23** — A: 30 dias após `occurred_at`, com expurgo diário | `docs/proposta-retencao-pendente.md` §9 registra a resposta; `ddl-confirmado.md` §5.4 fixa `occurred_at` e `ix_login_attempt_occurred`; `docs/dicionario-dados/dominio-auditoria.md` §Retenção agora declara o prazo; `proposta-dados-pessoais-e-retencao.md` §5.2 e `lacunas.md` `M32` foram atualizados. O `DELETE`/job ainda não existe. |
| `Q-RET-4` | **resolvida pelo dono em 2026-11-23** — A: 30 dias após `expires_at`, usado ou não, com expurgo diário | `docs/proposta-retencao-pendente.md` §9 registra a resposta; `ddl-confirmado.md` §5.5 fixa `expires_at`, `used_at` e a ausência de índice; `proposta-dados-pessoais-e-retencao.md` §5.2 e `lacunas.md` `M32` foram atualizados. O índice e o `DELETE`/job ainda não existem; o TTL continua em `B1`. |
| `Q-RET-6` | **resolvida pelo dono em 2026-11-24** — A: financeiro/custódia sem expurgo por tempo; `market_notification` lida com 90 dias e não lida nunca | `docs/proposta-retencao-pendente.md` §9 registra a resposta; a página `arquitetura/retencao-e-capacidade.md` atualiza as 11 linhas do mercado. O índice por `read_at`, o `DELETE`/job e a preservação legal continuam pendentes; nenhum índice ou job foi criado. |
| `Q-RET-7` | **resolvida pelo dono em 2026-11-24** — A: 30 dias de cópia diária, rotação, criptografia fora do host e restore trimestral | `docs/proposta-retencao-pendente.md` §9 registra a resposta; a página `arquitetura/retencao-e-capacidade.md` §9 registra a política. Fornecedor, contrato, RPO/RTO, criptografia e ensaio de restore continuam pendentes; nenhum backup foi executado. |
| `Q-CAP-1` | **resolvida pelo dono em 2026-11-23** — A: escrever a página agora como `proposto` | `docs/proposta-decisoes-capacidade.md` §9 registra a resposta; a página `arquitetura/retencao-e-capacidade.md` foi entregue como `proposto`, com volume `pendente de medição (M20)`. A 5ª onda continua condicionada a `M20`; as outras duas decisões de capacidade foram respondidas em seguida. |
| `Q-CAP-2` | **resolvida e aplicada pelo dono em 2026-11-23** — A: matriz por tabela crescente | `docs/proposta-decisoes-capacidade.md` §9 registra a resposta; a página `arquitetura/retencao-e-capacidade.md` aplica 25 linhas nominais e declara os 84 grupos restantes. |
| `Q-CAP-3` | **resolvida e aplicada pelo dono em 2026-11-23** — A: sinal + dono + condição de medição, sem limiar | `docs/proposta-decisoes-capacidade.md` §9 registra a resposta; a página §8 registra o sinal de revisão por `M20`, o dono `wyd-ops` e a ausência de limiar numérico. |

Essas resoluções **não** fecham `M32` de novo nem criam lacuna: `Q-RET-3` fixa a retenção por ciclo de
vida e retira a categoria `person_consent` da fila de decisão; a `Q-RET-5` retira uma contradição entre
§5.9 e a linha `config_*`; as decisões de `person_token`, `login_attempt` e `pin_reset` aplicam prazos de
30 dias; `Q-RET-6` fixa a política do mercado, sem criar índice/job; `Q-RET-7` fixa a política de backup,
sem criar rotina; `Q-CAP-1` entregou a página como `proposto`, com volume declarado; `Q-CAP-2` e
`Q-CAP-3` foram aplicadas. Nenhuma foi declarada `aceita`. **Não resta categoria de retenção aberta**; a
validação jurídica separada continua em §3.2/§3.3.

### 3.2 `Q-RET-3` — decisão A e o que ela não resolve

A tabela abaixo é a verificação feita contra o repositório. A decisão **A** está tomada; não há mais
pergunta de retenção em aberto. O que continua fora dela é a validação jurídica de finalidades
(`Q-DP-2/validação`) e a aplicação no portal.

| ID | Fonte consultada | O que a fonte decide | O que a decisão não resolve |
|---|---|---|---|
| `Q-RET-3` | `ddl-confirmado.md` §5.7; `lacunas.md` `M7`; `lacunas.md` `A12`; `padrao-banco.md` `P-21`; `proposta-retencao-pendente.md` §Q-RET-3 | `M7` manda revogar sem apagar o histórico; `A12` manda fechar a pessoa sem apagar a linha; a FK `CASCADE` só age em remoção física, que a `1.0.0` não faz; `P-21` exige retenção declarada, e **A** a declara por ciclo de vida | **A decisão não cria** o parecer de `Q-DP-2`, não valida a base legal, não implementa a tela de consentimento e não substitui a validação para produção com dado real. Também não cria job porque não há expurgo por tempo |

**O que continua condicionado:** `person_consent` e a tela de consentimento do `wyd-portal` dependem de
`Q-DP-2/validação` para uso de dado real; a retenção em si não é mais o bloqueio. `Q-RET-7` já tem
política (30 dias), mas a rotina de backup/restore, o fornecedor, o RPO/RTO e o jurídico continuam
pendentes; `Q-RET-6` já tem política, mas a produção do mercado (`LAC-ADM-07`, `LAC-POR-06`,
`wyd-admin`/`wyd-portal`) e o expurgo de `market_notification` ainda esperam índice, job e preservação
legal. `P-21` está satisfeito para `person_consent` pela decisão A; a coleta real continua sujeita à
validação de finalidades e ao canal do titular.

`Q-RET-1`, `Q-RET-2` e `Q-RET-4` já têm decisão, mas a **aplicação** continua pendente: o índice completo
por `expires_at` de `person_token`, o índice por `expires_at` de `pin_reset` e os `DELETE`/jobs diários
de `person_token`, `login_attempt` e `pin_reset` não existem; até lá, as categorias estão decididas no
papel, não executadas. `Q-RET-4` não desbloqueia a prontidão do reset de PIN sem o TTL de `B1`, que
continua na onda 6. `Q-RET-6` está na mesma situação: a política existe, mas o índice por `read_at`, o
`DELETE` diário de `market_notification` e a preservação legal do mercado não existem.

### 3.3 Cadeia de dependências e contagem de bloqueios

`Q-RET-3` **foi resolvida em 2026-11-24 (A)** e não depende mais do parecer. O `Q-DP-2` continua
respondido na opção A (inventário mínimo), mas `Q-DP-2/validação` é uma validação jurídica separada, ainda
pendente: a hipótese legal de cada finalidade segue `a validar`, não há parecer, advogado ou data de
fechamento no repositório, e o formato do registro foi definido em 2026-11-24 (A: parecer escrito por
finalidade) sem produzir o parecer. A coleta real também depende do canal de contato do titular, ainda
`pendente`. A decisão do dono não transforma a ausência do parecer em validação jurídica.

| Gate ou decisão | Depende de | Fecha ao ser respondida | Destrava | Bloqueios removidos | Custo |
|---|---|---|---|---|---|
| `Q-DP-2/artefato` | — | **decidida em 2026-11-24 (A)** — parecer jurídico escrito, por finalidade | define o registro que pode encerrar a validação | **0 tabelas**; `Q-RET-3` já foi resolvida pelo dono sem ele; a parte jurídica de `Q-RET-7` continua condicionada | não medido |
| `Q-DP-2/validação` | revisão jurídica externa (`Q-DP-1` §5.1); parecer ainda inexistente | — | **não é mais pré-requisito da decisão `Q-RET-3`**; continua pendente para a validação de finalidades e uso de dado real | **1 tabela** (`person_consent`) e **1 tela** continuam sujeitas a esta validação; `Q-RET-6` independe dela | não medido; validação externa |
| `Q-RET-3` | decisão do dono; `Q-DP-2` como validação separada | **resolvida em 2026-11-24 (A)** — ciclo de vida da pessoa, sem expurgo por tempo | `person_consent` e tela do portal no que dependem de retenção; **0 tabelas de produção criadas** | **1 tabela** e **1 tela** a aplicar; nenhum índice ou job | 0,5 sessão |
| `Q-RET-6` | preservação legal + `wyd-ops` | **resolvida em 2026-11-24 (A); aplicação pendente** | política das 11 tabelas de mercado | 11 tabelas decididas; 1 índice (`market_notification`) e 1 job ainda não criados | 1 sessão + 2–4 h + 1 sessão de `wyd-ops` |
| `Q-RET-7` | política/fornecedor de backup + jurídico | **resolvida em 2026-11-24 (A); aplicação pendente** | política de cópia, rotação e restore | 1 categoria de backup decidida; nenhuma tabela ou índice; a rotina continua pendente | 1 sessão + 1–2 sessões de operação |
| `Q-CAP-1` | nenhuma externa; `M20` limita a aceitação | **resolvida em 2026-11-23 (A)** | página `retencao-e-capacidade` (entregue como `proposto`); `Q-CAP-2` e `Q-CAP-3` decididas | 1 página entregue; 2 decisões dependentes resolvidas; a 5ª onda **não** fecha antes de `M20`; 0 tabelas | 4–6 h de escrita; tempo real não medido |
| `Q-CAP-2` | `Q-CAP-1`/página | **resolvida e aplicada em 2026-11-23 (A)** | matriz por tabela; 25 linhas nominais e 84 grupos declarados | 1 seção da página; 0 tabelas | 0,5–1 sessão |
| `Q-CAP-3` | `Q-CAP-2`/página | **resolvida e aplicada em 2026-11-23 (A)** | sinal + dono + condição de medição; sem limiar | 1 seção da página; 0 tabelas | 0,5 sessão |

**Contagem para escolher a próxima pergunta:** `Q-CAP-1`, `Q-CAP-2` e `Q-CAP-3` foram respondidas e
aplicadas. **Não resta decisão de capacidade nem de retenção.** `Q-RET-6` foi respondida em A e definiu
11 tabelas, mas não criou índice/job e não substitui a preservação legal; `Q-RET-7` foi respondida em A
e definiu backup, mas não criou rotina/restore; `Q-RET-3` foi respondida em A e não cria job. A próxima
frente é de **aplicação** ou de lacuna externa, não de nova decisão de retenção.

#### 3.3.1 Comparação das frentes abertas e da retenção

A tabela abaixo distingue o que uma resposta **fecha** do que continua pendente por terceiros. Ela não
é uma decisão: `Q-DP-2` segue respondida em A, `Q-DP-2/artefato` segue decidido em A, e `Q-RET-6` foi
respondida em A (2026-11-24); `Q-RET-3` também foi respondida em A na mesma data. As linhas que ainda
dizem “aberta” ou “sem resposta” são **histórico anterior** à resposta e ficam preservadas; o estado
vigente é `Q-RET-3 = A` e **0 decisões de retenção abertas**.

**Resposta do responsável (2026-11-24): A — parecer jurídico escrito por finalidade.** O registro
aceito é um parecer escrito, com uma conclusão por finalidade, emissor identificado, data e versão do
inventário de §5.4 de `proposta-dados-pessoais-e-retencao.md`; a conclusão pode ser `adequada`,
`adequada com condição` ou `não adequada`. O parecer **não existe** e não foi produzido nesta rodada.
O status é **decidida, com validação jurídica ainda pendente**: `Q-DP-2/validação` continua aberta,
mas `Q-RET-3` foi resolvida em A pelo dono e não está mais bloqueada por ela; `Q-RET-7` já tem política,
mas a rotina e o jurídico continuam condicionados. Custo, prazo, revisor e conclusão seguem **não
medidos**; nenhum nome, data ou parecer foi inventado.

| Frente | O que uma resposta fecha | O que continua bloqueado | Dependência que permanece | Esforço registrado | Não medido |
|---|---|---|---|---|---|
| **`Q-DP-2/artefato`** (a) | formato do registro de validação: parecer escrito por finalidade | o parecer em si, a validação de `Q-DP-2` e a parte jurídica de `Q-RET-7`; 0 tabelas | revisão jurídica externa; nenhum parecer, revisor ou data; `Q-RET-3` já foi resolvida sem ele | **não medido** | custo, prazo, revisor e conclusão |
| **`Q-DP-2/parecer`** (a2) | emissor e prazo do parecer, se o dono os fixar | `Q-DP-2/validação`, parte de `Q-RET-7`; **0 tabelas**; o parecer ainda precisa existir e concluir | contratação/emissor externo; não há custo, VPS nem data registrada; a pergunta foi **prejudicada** pela restrição de não gastar/contratar | **não medido** | emissor, custo, prazo e conclusão jurídica |
| **`Q-RET-6`** (b) | **respondida em 2026-11-24 (A)**: política das **11 tabelas** de mercado, com 1 índice e 1 job associados | **fecha a política**; não cria o índice, o `DELETE`/job, não mede volume e não substitui a preservação legal; `Q-RET-7` não é destravada; `Q-RET-3` já foi resolvida separadamente | preservação legal do mercado + execução por `wyd-ops` | 1 sessão + 2–4 h + 1 sessão de `wyd-ops`; volume e retenção legal não medidos | volume, custo de índice/job e exigência legal |
| **`Q-RET-6/aplicação`** (b2) | **execução** da política já decidida: 1 índice por `read_at`, 1 `DELETE`/job e preservação legal | **0 decisões novas**; aplica 11 tabelas; não cria decisão de retenção | preservação legal do mercado e execução por `wyd-ops`; não há autorização de código nesta rodada | 2–4 h de índice/migration + 1 sessão de `wyd-ops` | volume, custo de índice/job e exigência legal |
| **`Q-RET-7`** (c) | **respondida em 2026-11-24 (A)**: 30 dias de cópia diária, rotação, criptografia fora do host e restore trimestral | **fecha a política de 1 categoria**; nenhuma tabela ou índice; a rotina, o fornecedor, o contrato e o ensaio de restore continuam pendentes | política/fornecedor de backup de `wyd-ops` + jurídico | 1 sessão + 1–2 sessões de operação; RPO/RTO, tamanho e frequência não medidos | RPO/RTO, volume, fornecedor, região e custo |
| **Lacunas externas `A11`, `A20`, `M21`** (d) | **0 decisões novas do `wyd-db`**; são 3 das 38 lacunas, já `externa` | `A11` depende da captura do cliente 7662; `A20` e `M21` dependem da execução/planejamento do `wyd-portal`; não há tabela ou regra local a fechar | cliente, portal e captura; o dono do `wyd-db` não as encerra sozinho | não medido nesta fonte para `A11`; `A20`/`M21` pertencem ao portal | custo e artefato de cada frente externa |
| **`Q-OPS-2`** (e) | **respondida em 2026-11-24 (A)**: VPS paga com processo persistente e Compose; **1 decisão do `wyd-ops` fora das 38 lacunas**; **0 tabelas ou regras do `wyd-db`** | **não cria** VPS, contrato, backup, RPO/RTO, índice ou regra de schema; `backup-e-restore` continua bloqueada por fornecedor/RPO/RTO/ensaio | contrato/provisionamento + backup + operação; `wyd-ops` | 1 decisão de ops + contrato e operação; valor não medido | provedor, custo, capacidade e evidência operacional |

**Resposta do responsável (2026-11-24): `Q-RET-6` = A.** A opção separa o financeiro/custódia (sem
expurgo por tempo) da notificação lida (90 dias após `read_at`; não lida, nunca) e recusa o expurgo de
365 dias de `listing`/`bid`/`escrow`. A política foi decidida, mas **não** foi aplicada: o índice por
`read_at`, o `DELETE`/job, a preservação legal e o volume (`M20`) continuam pendentes.

**Resposta do responsável (2026-11-24): `Q-RET-7` = A.** A política de 30 dias de cópia diária, com
rotação, criptografia fora do host e restore trimestral, foi decidida; **não foi aplicada**. Fornecedor,
contrato, RPO/RTO, criptografia e ensaio de restore continuam pendentes. A aplicação de `Q-RET-6` segue
como a execução que mais remove bloqueios (11 tabelas, 1 índice e 1 job), mas não é uma decisão nova;
`Q-RET-3` foi resolvida em A e não depende mais de parecer.

**Relação `Q-OPS-2` → `Q-RET-7`:** `Q-RET-7` e `Q-RET-7/aplicação` citam fornecedor/contrato, RPO/RTO e
ensaio de restore, mas **não citam `Q-OPS-2` literalmente** como pré-requisito. A fonte de ops diz que a
opção A (VPS) exige contrato, backup e operação; logo, `Q-OPS-2` é insumo para o caminho de fornecedor e
operação, não uma condição já declarada que feche `Q-RET-7` ou a página `backup-e-restore`. As páginas
`backup-e-restore`, `recuperacao-de-desastre`, `manutencao-rotineira` e `plano-de-manutencao` continuam
bloqueadas por rotina/RPO/RTO/fornecedor, não apenas pelo host.

**Resposta do responsável (2026-11-24): `Q-OPS-2` = A.** A produção será uma **VPS paga com processo
persistente e deploy Compose**; a decisão escolhe o ambiente, mas **não aplica** contrato, VPS, backup,
RPO/RTO nem página de operação. A resposta fecha **1 decisão do `wyd-ops` fora das 38 lacunas** e remove
o bloqueio de escolha de host; não fecha `Q-RET-7`, `backup-e-restore` nem as três lacunas externas.

**Fila depois da resposta.** As aplicações de `Q-RET-6` e `Q-RET-7` continuam sendo execução já decidida
e dependem de `wyd-ops`; `Q-RET-3` foi resolvida em A e não cria job; `A11`/`A20`/`M21` continuam externas.
A comparação seguinte mede o que restou **depois** de `Q-OPS-2 = A`; o bloco `3.3.1.2` registra que a
pergunta anterior sobre emissor/prazo ficou prejudicada pela restrição de não contratar e que a decisão
A foi tomada sem parecer.

#### 3.3.1.1 Frentes restantes — medição de 2026-11-24, já com `Q-OPS-2` decidida

`Q-OPS-2` **saiu da lista de frentes abertas**: foi respondida em A (VPS paga com processo persistente e
Compose) e registrada em `planejamento-1.0.0/wyd-ops/docs/proposta-decisoes-ops.md` §4/§6 e em
`planejamento-1.0.0/wyd-ops/docs/index.md` §“Decisão de hospedagem”. A fonte dona diz que a decisão
**não cria** VPS, contrato, rotina de backup, RPO/RTO nem página de operação. Por isso, o que ela removeu
desta medição foi apenas a escolha de host; não removeu fornecedor, contrato, RPO/RTO nem as páginas de
operação.

| Frente ainda aberta | Evidência no repositório | Destrava hoje (descontado `Q-OPS-2 = A`) | Continua bloqueado por terceiro/condição | Esforço já registrado |
|---|---|---|---|---|
| **`Q-RET-3`** | `docs/proposta-retencao-pendente.md` §Q-RET-3: “Prova de consentimento com versão… Não há índice por tempo”; `pendencias-de-fechamento.md` §3.1 e §3.2 | **respondida em 2026-11-24 (A)**: **0 decisões abertas**; **1 tabela** (`portal.person_consent`) e **1 tela** do `wyd-portal` a aplicar; **0 páginas de `docs/`** dependem exclusivamente dela (a linha já existe em `retencao-e-capacidade.md` como decidida) | A retenção está decidida; a produção com dado real continua sujeita a `Q-DP-2/validação` e ao canal do titular; nenhum parecer foi produzido | 0,5 sessão; nenhum índice ou job |
| **`Q-RET-6/aplicação`** | `pendencias-de-fechamento.md` §4.3: “criar o índice por `read_at` e o `DELETE` diário de `market_notification`; preservar as demais tabelas conforme A”; `docs/proposta-retencao-pendente.md` §Q-RET-6 | **0 decisões novas**; aplica **11 tabelas**; **1 índice** e **1 job**; **0 páginas de `docs/`** são destravadas sozinhas (o dicionário já existe; `planos/plano-de-manutencao.md` depende do conjunto de jobs, não só deste) | Preservação legal do mercado e execução por `wyd-ops`; `Q-OPS-2 = A` não criou o job nem a preservação | 2–4 h de índice/migration + 1 sessão de `wyd-ops`; volume não medido |
| **`Q-RET-7/aplicação`** | `pendencias-de-fechamento.md` §4.3: “fornecedor/contrato de backup, RPO/RTO, criptografia e ensaio de restore; jurídico da cópia”; §5: `backup-e-restore.md` e `recuperacao-de-desastre.md` bloqueadas | **0 decisões novas**; **0 tabelas/índices**; **2 páginas de operação** (`backup-e-restore`, `recuperacao-de-desastre`) e **1 rotina** de cópia/restore; a política de 30 dias já está decidida | Fornecedor/contrato, RPO/RTO, criptografia, ensaio de restore, `wyd-ops` e jurídico; `Q-OPS-2 = A` escolheu o host, mas não contratou nem configurou nada | 1 sessão + 1–2 sessões de operação; RPO/RTO, volume e custo não medidos |
| **`A11`, `A20`, `M21`** | `lacunas.md` §2.2: `A11` “capturar no cliente” com estado `externa`; `A20` “telas e rotas do portal” com estado `externa`; `M21` “lista por tela é do planejamento do `wyd-portal`” com estado `externa` | **0 decisões, regras ou páginas do `wyd-db`**; são **3 lacunas externas**; destravam apenas o próprio dono externo: **1 captura** (`A11`), **1 implementação de MFA** (`A20`) e **1 lista de telas** (`M21`) | Cliente 7662 para `A11`; `wyd-portal` para `A20`/`M21`; nenhuma delas é fechada pelo dono do `wyd-db` | `A11` não estimado até endpoint/sessão sintética; `A20`/`M21` pertencem ao planejamento do portal; 35/3/0/0 preservados |

**Parecer de `Q-DP-2` — histórico e estado vigente.** Esta medição foi feita antes da resposta de
`Q-RET-3`; por isso ela tratava o parecer como bloqueio. O estado vigente é: **`Q-RET-3 = A`** (ciclo de
vida), decidida em 2026-11-24, e `Q-DP-2/validação` continua pendente como validação separada. Em
cascata, `person_consent` (**1 tabela**) e a tela do portal (**1 tela**) continuam sujeitas à validação
para dado real; **nenhuma das páginas bloqueadas** tem nela seu bloqueio exclusivo. As páginas
`backup-e-restore` e `recuperacao-de-desastre` herdam a parte jurídica de `Q-RET-7`, mas continuam
bloqueadas também por fornecedor, RPO/RTO e rotina; não são “fechadas pelo parecer”.

**Custo/estado do parecer.** `pendencias-de-fechamento.md` §3.3.1 registra o formato A (parecer escrito,
por finalidade) e diz explicitamente que **não há emissor, prazo, custo ou conclusão**; a linha
`Q-DP-2/parecer` está `não medido`. A `proposta-dados-pessoais-e-retencao.md` §5.1 mantém a revisão
jurídica como “externa, não contratada” e `pendente`. A pergunta anterior sobre **quem emite e em que
prazo** ficou **prejudicada**: o responsável declarou que o projeto é experimental, sem gasto e sem
apoio jurídico; as opções A/B/C que presumiam contratar um emissor são inválidas e não foram
reaproveitadas.

**Resposta do responsável (2026-11-24):** projeto experimental, de testes/aprendizado, **sem gasto** e
**sem apoio jurídico**; o responsável pediu a explicação do que seria o parecer e depois escolheu
**`Q-RET-3 = A`**. A restrição não é parecer, não é validação jurídica e não muda `Q-DP-2/validação`.
Ela apenas remove a exigência de contratar alguém para a decisão de retenção; a produção com dado real
continua não validada juridicamente. Teste com dado sintético é permitido e não exige esse parecer.

#### 3.3.1.2 `Q-RET-3 = A` e os caminhos sem terceiro (2026-11-24)

**A pergunta anterior ficou prejudicada.** As opções A/B/C de `Q-DP-2/parecer` presumiam contratar um
emissor jurídico externo. O responsável declarou que o projeto é experimental, **sem gasto** e **sem
apoio jurídico**; essa restrição invalida as três opções como caminho disponível. Ela **não** é parecer,
não é validação jurídica e não reabre `Q-DP-2 = A` nem `Q-DP-2/artefato = A`. A decisão de retenção foi
tomada separadamente: **`Q-RET-3 = A`**, ciclo de vida da pessoa.

| Caminho disponível sem terceiro | Evidência no repositório | O que destrava hoje | O que continua bloqueado |
|---|---|---|---|
| **Teste/planejamento com dado sintético** | `docs/specs/restricoes-e-premissas.md` §2: “fixture, golden e captura só com dado sintético declarado”; `proposta-dados-pessoais-e-retencao.md` §Q-DP-7: “só sintético declarado”; `lacunas.md` `M32`: “planejamento e DDL com dado fictício continuam livres” | testes, fixtures, goldens, protótipo da tela e documentação de `person_consent` com dado sintético; **não cria job nem tabela de produção** | dado pessoal real, produção, validação jurídica e `Q-DP-2/validação`; nenhum dado real pode entrar |
| **Documentar a retenção por ciclo de vida** | `docs/proposta-retencao-pendente.md` §Q-RET-3 e §9; `proposta-dados-pessoais-e-retencao.md` §5.2 | a linha `person_consent` passa a `decidido`; `P-21` não bloqueia mais a tabela por falta de prazo | a tela do portal e o uso real; não cria índice nem `DELETE`, porque não há expurgo por tempo |
| **Aplicar `Q-RET-6`/`Q-RET-7` na parte não jurídica** | `pendencias-de-fechamento.md` §4.3; `Q-RET-6` ainda exige `wyd-ops` + preservação legal; `Q-RET-7` exige fornecedor/contrato/RPO/RTO/restore e jurídico | nada imediato sem `wyd-ops`: são execução, não decisão; a política já está registrada | índice/job de `market_notification`, rotina de backup, contrato e restore; `Q-OPS-2 = A` só escolheu o host |
| **Manter o bloqueio e não avançar** | `docs/specs/restricoes-e-premissas.md` §2: coleta real bloqueada; `Q-DP-2/validação` pendente | nenhuma tabela, página ou rotina; preserva o estado sem inventar validação | todo o caminho de produção com dado real continua parado |

**Honestidade sobre o que existe:** o único caminho **já registrado** que permite avançar sem terceiro é o
**teste/planejamento com dado sintético** (`Q-DP-7 = A` e `M32`). **Não existe** no repositório um
substituto para o parecer que autorize dado real, nem uma opção de “validar sem jurídico”. Se o dono
quiser operar sem dado real, o caminho está escrito; se quiser produção real, o parecer ou uma decisão
externa continuam necessários. Uma “análise interna” só pode ser registrada como análise não jurídica;
ela não fecha `Q-DP-2/validação` nem muda o estado de `person_consent` para validado.

**Resposta do responsável (2026-11-24): A — avançar `person_consent` e a tela somente com dado sintético,
mantendo a produção real bloqueada.** A autorização segue `Q-DP-7 = A` e `M32`: fixture, golden, captura
e protótipo com dado sintético declarado; **não** autoriza dado pessoal real, não fecha
`Q-DP-2/validação`, não cria índice/job e não muda `Q-RET-3 = A`. A contagem de retenção continua **7
resolvidas, 0 abertas**; a validação jurídica permanece pendente como registro separado.

## 4. Atos do dono, de terceiros e de jurídico

### 4.1 Aceite de página (`M33`, `Q-M33-6`)

| ID | Pendência | Dono | Custo estimado | Dependência externa | Consequência de não fazer | Desbloqueia |
|---|---|---|---|---|---|---|
| `M33/Q-M33-6` | aceitar as páginas de migration **uma a uma**, na ordem da série; 28 escritas, 1 aceita | dono do `wyd-db`, com revisão por par | 34 atos × 0,5–1 h = **17–34 h**, sem contar correções | nenhuma; é ato do dono, não decisão técnica | `0001` não pode começar; a onda A fica sem porta de entrada aceita | início da série, `0001` em diante, e o critério `aceito` por página |

O aceite **não** foi simulado: a regra está decidida, mas nenhuma página nova foi aprovada nesta rodada.

### 4.2 Seeds bloqueados por dado de origem (6)

Todos dependem do cliente 7662 medido, não de uma decisão de schema; o valor herdado não é fonte
verificada (`plano-migrations.md` §4). Custo é estimativa de curadoria + escrita.

| ID | Pendência | Dono | Custo estimado | Dependência externa | Consequência de não fazer | Desbloqueia |
|---|---|---|---|---|---|---|
| `0016-seed-taxonomy` | reconferir contagens de taxonomia na fonte | curadoria de conteúdo + dono | 1 sessão + 2 h | cliente 7662 medido | página não escrita; seed de `0013` não roda | conteúdo de taxonomia |
| `0017-seed-item-catalog` | recontar o catálogo de itens (contagens de 2026-11-08/15) | curadoria de conteúdo + dono | 1–2 sessões | cliente 7662 medido | página e seed de `0014` ficam bloqueados | catálogo de itens |
| `0018-seed-skill-catalog` | curadoria de classe e linhagem das skills | curadoria de conteúdo + dono | 1–2 sessões | cliente 7662 medido | página e seed de `0015` ficam bloqueados | catálogo de skills |
| `0020-seed-world` | conferir templates, NPCs, lojas, níveis e regiões | curadoria de conteúdo + dono | 1–2 sessões | cliente 7662 medido | página e seed de `0019` ficam bloqueados | mundo inicial |
| `0023-seed-rules` | conferir refino, gemas, SANC e composição | curadoria de conteúdo + dono | 1–2 sessões | cliente 7662 medido | página e seed de `0021`/`0022` ficam bloqueados | regras iniciais |
| `0027-seed-quests-events` | conferir cadeia de quests e calendário de eventos | curadoria de conteúdo + dono | 1–2 sessões | cliente 7662 medido | página e seed de `0026` ficam bloqueados | quests e eventos |

### 4.3 Páginas e medidas que dependem de execução

| ID | Pendência | Dono | Custo estimado | Dependência externa | Consequência de não fazer | Desbloqueia |
|---|---|---|---|---|---|---|
| `M20/metricas` | escrever `planos/metricas.md` com número medido | arquitetura + operação | 1 sessão, **depois** da série | a série precisa existir e ser medida | o critério de saída da documentação 1.0.0 não fecha; o item 10 de §10 segue com a metade suspensa | métrica, contagem real e aceite de página |
| `SERIE-REGRAS` | ~~escrever `migrations/serie-e-regras.md`~~ — **feito em 2026-11-25**, `aceito` | `wyd-db` | concluído | nenhuma decisão nova; a fonte é `plano-migrations.md` §2/§6 e `src/migrate/spec.md` | a página existe, mas a política de `down` continua aberta em `Q-SERIE-1` | 5ª onda e operação da série |
| `Q-RET-1/aplicação` | criar o índice completo por `expires_at` e o `DELETE` diário de `portal.person_token` | `wyd-db` + `wyd-ops` | 2–4 h de índice/migration + 1 sessão de job | outra frente: `wyd-ops`; sem jurídico | a decisão existe, mas a tabela não está pronta para produção nem tem expurgo | produção de `person_token`, expurgo diário e a linha de índice |
| `Q-RET-2/aplicação` | criar o `DELETE` diário de `ops.login_attempt` usando `ix_login_attempt_occurred` | `wyd-ops` | 1 sessão de job | outra frente: `wyd-ops`; sem jurídico e sem índice novo | a decisão existe, mas a tabela continua sem expurgo e não vai a produção | produção de `login_attempt` e expurgo diário |
| `Q-RET-4/aplicação` | criar o índice por `expires_at` e o `DELETE` diário de `game.pin_reset` | `wyd-db` + `wyd-ops` | 2–4 h de índice/migration + 1 sessão de job | outra frente: `wyd-ops`; sem jurídico; TTL ainda é `B1` | a decisão existe, mas `pin_reset` não vai a produção e pedidos nunca usados crescem | produção e expurgo de `pin_reset` |
| `Q-RET-6/aplicação` | criar o índice por `read_at` e o `DELETE` diário de `market_notification`; preservar as demais tabelas conforme A | `wyd-db` + `wyd-ops` | 2–4 h de índice/migration + 1 sessão de job | preservação legal do mercado e execução por `wyd-ops` | a política existe, mas a notificação lida continua sem índice e sem expurgo | aplicação da política de 11 tabelas; índice e job de notificação |
| `Q-RET-7/aplicação` | política de 30 dias, rotação, criptografia fora do host e restore trimestral; destino, RPO e RTO definidos por `Q-BLK-03` (2026-11-24) | `wyd-ops` + jurídico | 1 sessão + 1–2 sessões de operação | contrato/região do destino, credencial administrativa e ensaio de restore; jurídico da cópia | a política existe e a página foi escrita em 2026-11-24, mas não há rotina, contrato nem restauração provada | página `backup-e-restore` (escrita, `proposto`), `recuperacao-de-desastre` (escrita, `proposto`) e a rotina de cópia |

**Decisões de 2026-11-24 aplicadas nesta tabela:** `Q-BLK-02` fixou o executor dos quatro expurgos
(`cron` do host no Compose) e `Q-BLK-03` fixou destino, RPO de 24 h e RTO de 4 h do backup. Nenhuma das
duas cria índice, job, contrato ou rotina: a execução continua nas linhas acima, e o texto que consome as
duas está em `docs/planos/plano-de-manutencao.md` e `docs/guia-administrador/backup-e-restore.md`.

**Fora desta tabela:** `Q-CAP-1/página` foi entregue em 2026-11-23 como `docs/arquitetura/retencao-e-capacidade.md`
`proposto`, com volume `pendente de medição (M20)`. A aceitação continua pendente; o tempo real de
escrita não foi medido.

### 4.4 Lacunas externas (3)

Estas não são decisões do `wyd-db` e **não entram** nas decisões de retenção acima. Elas continuam
`externas` em `lacunas.md`; nenhuma foi reaberta.

| ID | Pendência | Dono | Custo estimado | Dependência externa | Consequência de não decidir | Desbloqueia |
|---|---|---|---|---|---|---|
| `A11` | charset e tamanho do login | frente `wyd-client` | não estimado até haver endpoint e sessão sintética | captura do cliente 7662; `LCLI-04`, `D3`–`D6` | `A11` fica externa e a captura não é feita | fechamento de `A11`, sem DDL novo |
| `A20` | MFA | frente `wyd-portal` | planejamento do portal, não do `wyd-db` | telas e rotas do portal; DDL de `0031` já aprovado | `A20` fica externa; MFA não é implementado no portal | implementação de MFA no portal |
| `M21` | lista de telas do portal | frente `wyd-portal` | planejamento do portal | telas e escopo do portal; decisão de 2026-11-20 já fixou o escopo | `M21` fica externa; a lista nominal não existe | planejamento do portal |

## 5. A página de `docs/` bloqueada

Esta é a fila de `proposta-paginas.md` §6, reproduzida com o dono e o bloqueio de cada página. Ela
**não** é uma lista de patch e não autoriza escrever por dedução: cada linha só sai quando a condição
declarada existir. A soma agora é **7**: `arquitetura/retencao-e-capacidade.md` saiu em 2026-11-23,
como `proposto` com volume pendente (`Q-CAP-1`); as três páginas de entrada na cena e as quatro de
economia e custódia saíram em 2026-11-24; no lote C do mesmo dia saíram sete — quatro escritas e três
retiradas do teto por `Q-BLK-01` —, e o teto autoral passou de **147 para 144**; depois do lote,
`arquitetura/fluxo-de-dados.md` foi escrita em 2026-11-24; e o **lote D** escreveu quatro no mesmo dia —
`sustentacao`, `manutencao-rotineira`, `fixtures-e-factories` e `dados-de-teste`, com `Q-BLK-04` e
`Q-BLK-05` aprovadas em A. O **lote E** escreveu duas em 2026-11-24 e o **lote F** escreveu as quatro
que dependiam de decisão — `rastreabilidade`, `stakeholders`, `implantacao` e `modelo-conceitual` —, e a
fila fica em **1**.

| ID (caminho) | Bloqueio | Dono / marco | Desbloqueia |
|---|---|---|---|
| `planos/metricas.md` | número medido; `M20` só depois da série | arquitetura (`M20`, `P2`) | Onda 6; fechamento da documentação |

**Saíram desta fila depois da revisão de 2026-11-24:** `guia-desenvolvedor/pool-e-transacoes.md` e
`specs/requisitos-nao-funcionais.md` no lote E (§6.6.18); `specs/rastreabilidade.md`,
`specs/stakeholders.md`, `arquitetura/implantacao.md` e `arquitetura/modelo-conceitual.md` no lote F
(§6.6.19), este último por decisão de §6.6.8 a §6.6.11 em A. Resta **1** página bloqueada, e ela não é
pergunta: depende de `M20` executado.

**Fora desta tabela, mas ainda pendente:**

- `migrations/serie-e-regras.md` — 1 página de seção, 5ª onda; ver §4.3.
- seis páginas de seed `0016`–`0018`, `0020`, `0023`, `0027` — 6 dos 34 arquivos; ver §4.2.
- `planos/metricas.md` aparece nas duas listas por ser, ao mesmo tempo, página bloqueada e entrega de
  decisão; não é contado duas vezes na fila de 19. `arquitetura/retencao-e-capacidade.md` já saiu da
  fila: foi entregue como `proposto`, com volume pendente.

## 6. Ordem recomendada (maior desbloqueio por sessão)

**A fila vigente é a do §6.6.22 (lote I, 2026-11-24):** (1) aceite das **12** páginas de `tests/` — **12**
atos; (2) aceite da leva `migrations/` — **27**; (3) as **40** issues em 3 lotes por onda; (4) leva `src/` —
**37**; (5) leva `docs/` — **141**. Em paralelo: `Q-BLK-19` (dívida das 11 capacidades) e o dicionário
`banco-catalogo.md`, este quando o PR de ferramentas for autorizado. O pacote `Q-BLK` está respondido até
`Q-BLK-16`; naquele momento as quatro do lote I (`Q-BLK-17` a `Q-BLK-20`) seguiam abertas e **nenhuma
delas era pré-requisito da leva 1** — a `17` foi respondida pela própria execução da leva (§6.6.22.5).

As perguntas que destravam as páginas bloqueadas continuam em §6.6: 11 decisões que liberaram 17 páginas e
2 bloqueios que são trabalho técnico. Esta seção 6 continua sendo o histórico de prioridade do projeto;
§6.6 é o pacote de decisões e §6.6.22 é a ordem vigente.

Esta é a fila **interna** do `wyd-db`. No nível do projeto, `wyd-1.0.0.md` §6.2 registrou a captura A1
como próxima ação até 2026-11-23; em **2026-11-24**, `Q-FOCO-1 = D` priorizou a remoção dos bloqueios
das páginas de `docs/` — 26 na decisão, **19** depois dos dois lotes escritos. A1 continua autorizada, mas
não é o foco imediato; a fila de retenção também não remove
o bloqueio de fio.

**Decisão do dono em 2026-11-23, completada em 2026-11-24:** a rodada **do `wyd-db`** foi a de **retenção**.
`Q-RET-1`, `Q-RET-2` e `Q-RET-4` foram respondidas (30 dias); `Q-RET-6` e `Q-RET-7` foram respondidas em A
(2026-11-24); `Q-RET-3` foi respondida em A (2026-11-24). **Não resta decisão de retenção aberta.** A ordem
abaixo é a fila de retenção, **subordinada ao foco D** (`Q-FOCO-1 = D`, 2026-11-24); não é o foco imediato.

**Fila de retenção (histórico, não o foco vigente):** não restam decisões de retenção nem de capacidade;
`Q-DP-2/validação` continua pendente, mas não é pré-requisito de `Q-RET-3`. `Q-RET-6` e `Q-RET-7` têm
política decidida e aplicação pendente; a aplicação de `Q-RET-1/2/4/6/7` é trabalho técnico, não decisão
nova; `Q-RET-3` não cria job e a matriz de `Q-CAP-2` e o sinal de `Q-CAP-3` estão aplicados na página.

1. **Quando a retenção for priorizada, aplicar `Q-RET-1`, `Q-RET-2`, `Q-RET-4`, `Q-RET-6` e `Q-RET-7` primeiro.** As cinco já têm decisão e
esperam índice/`DELETE`, job ou rotina; `Q-RET-4` também espera a migration por `expires_at`. `Q-RET-3`
não cria job: a retenção é o ciclo de vida da pessoa.
2. **Aplicar `Q-RET-3` na documentação/testes com dado sintético.** A decisão existe; a tela e a tabela
continuam sujeitas a `Q-DP-2/validação` para dado real. Não há índice nem `DELETE` a criar.
3. **Aplicar a matriz de `Q-CAP-2` e o sinal de `Q-CAP-3`.** A página já existe; a matriz e o sinal
   são documentação de arquitetura/operação, sem schema nem produção. Ambos foram entregues nesta rodada.
4. **Aceitar `M33/Q-M33-6` em paralelo, página a página.** São 34 atos do dono (17–34 h estimadas) e
   não dependem das decisões de retenção; liberam o início da série.
5. **Deixar os seis seeds para a rodada de curadoria com o cliente 7662.** Eles dependem de dado
   verificado, não de nova regra; nenhuma sessão de arquitetura os fecha por dedução.

**Justificativa:** não resta categoria de retenção pendente; `Q-RET-1`, `Q-RET-2`, `Q-RET-4`, `Q-RET-6` e
`Q-RET-7` ainda esperam aplicação, e `Q-RET-3` é 1 tabela/1 tela sem job. Capacidade resolve uma página;
seeds dependem de terceiro; as 7 páginas bloqueadas dependem de execução ou de decisão já listada. A
validação jurídica de `Q-DP-2/validação` permanece separada e não deve ser declarada feita.

## 6.5 Foco do projeto — comparação das frentes (2026-11-24)

**Pergunta:** "qual frente aberta resolve mais bloqueio por sessão e deve ser o foco do projeto, não apenas do `wyd-db`?"

**Estado:** recomendação técnica registrada em 2026-11-24 e **superada pela resposta de prioridade
`Q-FOCO-1 = D`**: o foco passou a ser remover os bloqueios das 26 páginas (23 depois do lote de
entrada na cena e **19** depois do lote de economia e custódia). A comparação abaixo fica como
histórico da decisão; a resposta não reabre `Q-RET-3`, `Q-RET-6`, `Q-RET-7`, `Q-DP-2` nem a autorização
de A1. Naquela rodada, nenhuma página bloqueada foi escrita, nenhum código foi criado e nenhuma
contagem de `lacunas.md` mudou; a execução posterior está em §6.5.4.

**Leitura curta.** O projeto tem **118 lacunas** somadas nos 12 registros, das quais **7 não estão
fechadas**: 6 abertas e 1 proposta; o `wyd-db` tem **38 = 0 abertas + 0 propostas + 35 fechadas + 3
externas** (`A11`, `A20`, `M21`). Não há pergunta de desenho aberta no `wyd-db` e a retenção está em
**7 resolvidas / 0 abertas**. O que resta no banco é documentação bloqueada, seeds sem fonte, aplicação
de retenção e aceite da série — não uma decisão de schema. A comparação abaixo mede o que cada frente
fecha, o que apenas prepara e o que continua dependendo de terceiro.

| Frente | O que fecha ou destrava (quantidade) | Páginas de `docs/` | Dependência removida | Terceiro que continua | Esforço registrado | Veredito |
|---|---|---|---|---|---|---|
| **A1 — peer, `ECAP`, CLI e captura** | fecha **1 lacuna** (`LPRT-03`, após a segunda leitura) e **prepara 1** (`LPRT-06`); libera handshake, offsets por `Type`, `X12` e a onda C; **11 arquivos autorais**, **0 migrations** | **0 das 19**; não é página de `wyd-db` | bloqueio de fio e de handshake | segunda leitura interna (Lucas Cantarelli), já nomeada; implementação ainda não existe | 11 arquivos, autorizado em 2026-11-23 | **Recomendada** |
| **`Q-DP-2/validação` + canal do titular** | **0 lacunas**; **1 tabela** (`person_consent`) e **1 tela** do portal; não libera dado real | **0 das 19** | nenhuma página; apenas a restrição de produção com PII real | parecer jurídico externo e canal do titular; a dispensa de parecer **não** é validação | não medido; contratação/apoio indisponível | **Não recomendada como foco** |
| **`Q-RET-6/aplicação`** | **0 decisões novas**; aplica **11 tabelas** de mercado, **1 índice** por `read_at` e **1 job**; **0 páginas** ficam escrevíveis só com isto | **0 das 19** | política de mercado deixa de estar “pendente”; a retenção passa a ter execução | preservação legal do mercado e `wyd-ops`; `Q-OPS-2 = A` só escolheu o host | 2–4 h de índice/migration + 1 sessão de `wyd-ops` | **Execução, não foco** |
| **`Q-RET-7/aplicação`** | **0 decisões novas**; **0 tabelas**; **2 páginas** (`backup-e-restore`, `recuperacao-de-desastre`) saem do bloqueio de política, mas **0 ficam escrevíveis** sem provedor/RPO/RTO; **1 rotina** | **2 das 23, apenas parciais** | política de backup deixa de estar “não decidida” | fornecedor/contrato, RPO/RTO, criptografia, ensaio de restore, jurídico da cópia e `wyd-ops` | 1 sessão + 1–2 sessões de operação; RPO/RTO não medido | **Não fecha sozinha** |
| **Fechar as páginas de `docs/` bloqueadas** | **19 páginas** restantes na decisão (23 antes do lote A; **11 hoje**) são o alvo, mas **0 formam uma frente executável isolada**: 2 dependem de `Q-RET-7`, 2 de limiar/medição, 1 de matriz de atores, 2 de suíte, 1 de pool, 4 de documentação e 7 de série/produto | **19** (alvo) | nenhuma dependência comum; cada grupo tem bloqueio próprio | série aplicada, `M20`, produto/arquitetura, operação e `wyd-docs`; não há um único terceiro | escrita de 19 páginas, depois dos bloqueios; não medido | **Não é um ato único** |
| **6 seeds de `wyd-db`** | **6 páginas de migration + 6 seeds**; **0 decisões**; nenhum valor pode ser inventado | **6** (alvo; 0 hoje) | nada | cliente 7662 medido; 6–11 sessões + 2 h somadas de §4.2, sem fonte de dado | curadoria + escrita; não medido | **Incapaz sem terceiro** |
| **`A11`, `A20`, `M21`** | **3 lacunas externas**: 1 captura do cliente, 1 implementação de MFA e 1 lista de telas; **0 páginas do `wyd-db`** | **0** | nenhuma dependência local | cliente 7662 e frente `wyd-portal`; o dono do banco não as fecha | não medido para `A11`; `A20`/`M21` são do portal | **Incapaz sem terceiro** |
| **Aceite da série (`M33`) + `serie-e-regras`** | **1/28 páginas aceitas**; **34 atos** de aceite; **28 páginas de migration** já escritas; inicia `0001` e a onda A; **0 lacunas** | **0 das 19** | aceite e porta de entrada da série | nenhum terceiro; é ato do dono com revisão por par | 34 atos × 0,5–1 h = **17–34 h**, mais `serie-e-regras` | **Segunda opção** |
| **Expandir propostas de outros sistemas** | **0–1 lacuna** alcançável (`LCLI-02`; `LCLI-03` é execução); **0 páginas do `wyd-db`** | **0** | nada além do próprio sistema | direitos do cliente (`LCLI-05`) e ficha assinada; não há pergunta de desenho isolada | baixo retorno; não medido | **Retorno marginal** |

**Reauditoria de 2026-11-25.** `A20` e `M21` já têm a condição de saída cumprida do lado do planejamento: as telas e rotas do segundo fator (`/admin/seguranca`) e o mapa de 50 rotas do portal estão em `planejamento-1.0.0/wyd-portal/docs/arquitetura/rotas-e-permissoes.md`, com as fatias `WYDPOR-F0`–`F11`. O que falta nas duas é execução, não decisão nem documento. As três seguem `externa`: em 2026-11-25 o titular aceitou a recomendação de **não reclassificar** `A20`/`M21` até a execução, e `A11` continua externa de fato, porque depende da captura no cliente 7662.

**O que a tabela não conta como página destravada.** `Q-RET-6` não torna nenhuma das páginas bloqueadas
escrevível sozinha: a página `planos/plano-de-manutencao.md` depende do conjunto de rotinas, não só
do job de `market_notification`. `Q-RET-7` não fecha `backup-e-restore` nem `recuperacao-de-desastre`
com a política de 30 dias: as duas continuam bloqueadas por fornecedor, RPO/RTO, criptografia e ensaio
de restore. `Q-DP-2/validação` não é dono de nenhuma das 19: a linha de `person_consent` já existe e a
produção real continua bloqueada. O aceite `M33` não escreve página nova: ele libera a série que já
está documentada, mas ainda não foi aceita arquivo por arquivo.

**Fora desta comparação:** o **catálogo funcional** é o maior bloqueio da declaração de `1.0.0` — a
cobertura continua “não medida” e nenhuma versão pode ser declarada completa sem ele (`wyd-1.0.0.md`
§1.5 e §4.2) —, mas não há total, entrada aprovada nem dono executando o levantamento; ele não é uma
frente decidível hoje. O `wyd-db` também não resolve as páginas bloqueadas, os seis seeds ou as três externas
com uma sessão de decisão: nenhuma delas tem pergunta de desenho aberta.

**Recomendação técnica (histórico) — A1, não “fechar o `wyd-db` por volume”.** A captura A1 é a única
candidata que:

1. **fecha uma lacuna real** (`LPRT-03`) e **prepara a seguinte** (`LPRT-06`), em vez de apenas mover
   bloqueio de documentação;
2. **remove o bloqueio de fio** que impede a onda C e o caminho `protocol → net → core → server`;
3. **não depende de parecer, contrato, fornecedor ou cliente medido**; a segunda leitura é interna e a
   autorização já existe desde 2026-11-23;
4. **cabe em 11 arquivos e 0 migrations**, enquanto o aceite da série custa 17–34 h e as páginas
   bloqueadas não podem ser escritas em bloco.

O custo é honesto: A1 **não** fecha as páginas bloqueadas, os 6 seeds, as 3 externas nem as aplicações de
`Q-RET-6/7`; ela apenas impede que o projeto continue parado no fio. A segunda opção é o **aceite da
série** (`M33`), que não fecha lacuna, mas transforma 28 páginas escritas em série iniciável. As demais
frentes **não devem ser recomendadas como foco** enquanto dependerem de terceiro: `Q-DP-2/validação`,
`Q-RET-7/aplicação`, os 6 seeds e `A11`/`A20`/`M21`. `Q-RET-6/aplicação` é execução interna, mas seu
retorno imediato é 0 página e 1 job, não a liberação de uma frente.

**Sequência técnica registrada antes da resposta (histórico):** (1) implementar peer/`ECAP`/CLI e
capturar, com a segunda leitura; (2) fechar `LPRT-03` e registrar `LPRT-06` como próximo; (3) em
paralelo ao aceite da série, usar `M33` para iniciar `0001`, sem tratar isso como fechamento das
páginas bloqueadas; (4) só depois aplicar `Q-RET-6/7` e retomar as páginas que dependem de operação. Nenhum passo
dessa sequência cria migration, job, contrato ou código nesta rodada.

### 6.5.1 Decisão de foco — `Q-FOCO-1`

**Pergunta submetida em 2026-11-24:** "A1 continua sendo o foco até o próximo marco, ou o aceite da
série (`M33`) passa à frente?" A pergunta é de **prioridade**, não de desenho; a resposta não reabre A1,
não altera as 38 lacunas e não conta como nova `Q-RET`. O registro da resposta entra abaixo com a data
e a opção efetivamente escolhida.

| Opção | O que destrava | Custo | O que não resolve |
|---|---|---|---|
| **A (recomendada)** — A1 primeiro | 1 lacuna fechada, 1 preparada, fio e onda C | 11 arquivos; implementação e segunda leitura | 26 páginas, 6 seeds, 3 externas e aplicações de retenção |
| **B** — aceite da série primeiro | 34 atos; 28 páginas aceitas; início de `0001`; onda A | 17–34 h; `serie-e-regras` | fio, onda C, 26 páginas, seeds e externas |
| **C** — `Q-RET-6/7` primeiro | 11 tabelas, 1 índice, 1 job e 2 páginas parciais | operação + jurídico | fio, 26 páginas, 6 seeds e externas |
| **D** — 26 páginas primeiro | 0 páginas plenamente escrevíveis sem os bloqueios; 26 é apenas o alvo | série, `M20`, produto/operação | fio, lacunas abertas e seeds |

**Resposta do responsável (2026-11-24): D — priorizar as 26 páginas.** A escolha **não** torna as 26
páginas escrevíveis de uma vez: o registro continua dizendo que **0** têm bloqueio exclusivo resolvido
por `Q-DP-2/validação` e que 2 dependem de `Q-RET-7`, 7 de texto de fluxo/estado naquele momento (4
restam depois do lote A), 2 de limiar/medição, 1 de matriz de atores, 2 de suíte, 1 de pool, 4 de
documentação e 7 de série/produto. O foco D
significa atacar **a fila de bloqueios das páginas**, começando pelo grupo que não depende de cliente,
fornecedor ou parecer; não significa escrever página bloqueada por dedução. A1 permanece autorizada e
não é cancelada; ela deixa de ser o foco imediato. As 38 lacunas, as 3 externas e as 7 decisões de
retenção permanecem exatamente como estavam: **0 abertas, 0 propostas**. Não há validação jurídica nem
autorização de dado real nesta resposta.

**Leitura operacional de D.** O maior grupo tratável sem terceiro é o das **7 páginas de fluxos/estados**
(`criação-e-seleção`, `carregamento-e-save`, `concessão-de-grant`, `compra-e-crédito`, `personagem`,
`grant`, `item-e-refino`): o DDL de `0006`, `0009`, `0011`, `0012` e `0021` está fechado e não há cliente
ou fornecedor na dependência; falta o texto do fluxo, que é decisão de produto + arquitetura. As três
primeiras foram escritas em 2026-11-24; restam as quatro de economia. O grupo
de **4 páginas de `guia-documentação`** é menor, mas 3 repetem `convencoes.md` e a quarta depende de um
`mkdocs.yml` que o projeto decidiu não ter; escrevê-las como duplicata viola “um fato, um dono”. As 2
páginas de backup/DR continuam presas a provedor/RPO/RTO. Por isso a próxima pergunta é de **ordem
dentro das 23**, não uma nova tentativa de reabrir A1.

### 6.5.2 Próxima decisão dentro de D — `Q-PAG-1`

**Pergunta submetida em 2026-11-24:** "qual lote de páginas bloqueadas recebe a primeira sessão de
D?" A decisão é de prioridade de escrita; não cria requisito, não fecha lacuna, não altera schema e não
autoriza código.

| Opção | Páginas | O que destrava | Custo/risco | Veredito |
|---|---:|---|---|---|
| **A (recomendada)** — entrada na cena | 3 (`specs/fluxos/criacao-e-selecao-de-personagem.md`, `specs/fluxos/carregamento-e-save.md`, `specs/estados/personagem.md`) | primeira fatia jogável, coerente com `LCORE-01` (entrar na cena antes de inventário/combate); usa DDL fechado de `0006` e `0009` | 3 páginas; exige texto de produto + arquitetura; não fecha as outras 23 | **Recomendada** |
| **B** — economia e custódia | 4 (`fluxos/concessão-de-grant`, `fluxos/compra-e-crédito`, `estados/grant`, `estados/item-e-refino`) | fluxo de grant, compra e estado de item/refino; usa `0011`, `0012`, `0021` | 4 páginas; não começa pela entrada na cena; pode andar depois de A | **Alternativa** |
| **C** — guia de documentação | 4 (`markdown-e-mkdocs`, `diagramas`, `links-referencias-e-ids`, `imagens-e-evidencias`) | 3 são duplicata de `convencoes.md`; a de mkdocs depende de config que não existe; o ganho é de escopo, não de conteúdo | baixo; escrever duplicata viola `AGENTS.md` §5, item 5; retirar exige decisão de teto | **Não recomendada** |
| **D** — backup e recuperação | 2 (`backup-e-restore`, `recuperacao-de-desastre`) | política já decidida, mas as páginas continuam presas a fornecedor/RPO/RTO/ensaio | não fecha sem terceiro; não é executável nesta rodada | **Incapaz agora** |

### 6.5.3 Resposta — `Q-PAG-1 = A` (2026-11-24)

**Resposta do responsável: A — entrada na cena.** O primeiro lote do foco D é composto por **3 páginas**:
`specs/fluxos/criacao-e-selecao-de-personagem.md`, `specs/fluxos/carregamento-e-save.md` e
`specs/estados/personagem.md`. A escolha é de **ordem de escrita**, não de produto: nenhuma das três foi
escrita naquela rodada, nenhuma foi declarada `aceita` e as **26 páginas continuavam bloqueadas** até a
sessão de escrita. Depois do lote, restariam **23** páginas na fila, sem contar os seeds e as páginas de
migration.

**Por que A.** O lote usa o DDL já fechado de `0006` e `0009`, não depende de cliente 7662, fornecedor,
RPO/RTO, parecer ou `wyd-ops`, e coincide com a primeira fatia jogável de `LCORE-01` (entrar na cena com
posição/estado inicial, antes de inventário, combate ou chat). O lote **não** fecha lacuna, não altera as
**38** do `wyd-db` (0 abertas, 0 propostas, 35 fechadas, 3 externas) nem as **7 decisões de retenção**
(7 resolvidas, 0 abertas). Nenhum código, migration, índice, job ou contrato foi criado naquela rodada.

**O que continuava bloqueado naquele momento.** As outras **23** páginas de `docs/` mantinham os
bloqueios de §5: 7 de fluxo/estado (incluindo as 4 de economia que não são deste lote), 2 de
limiar/medição, 1 de matriz de atores, 2 de suíte, 1 de pool, 4 de documentação e 7 de série/produto; os
**6 seeds** seguem presos ao cliente 7662 e as **3 externas** (`A11`, `A20`, `M21`) continuam externas.
A1 permanece autorizada e não cancelada, apenas fora do foco imediato.

### 6.5.4 Execução do lote A (2026-11-24)

As três páginas existem no worktree como `proposto` e não foram declaradas `aceitas`:
`docs/specs/fluxos/criacao-e-selecao-de-personagem.md`, `docs/specs/fluxos/carregamento-e-save.md` e
`docs/specs/estados/personagem.md`. A medição de fechamento do lote é:

| Medida | Resultado | Fonte |
|---|---:|---|
| páginas autorais com corpo em `docs/` | **124** (120 com `**Estado:**` + 4 portas sem estado) | `find`/`grep` no worktree, 2026-11-24 |
| páginas bloqueadas | **23** | `proposta-paginas.md` §6; tabela de §5 |
| páginas de fluxo/estado ainda bloqueadas | **4**: grant, compra/crédito, estado de grant e item/refino | §5 |
| lacunas do `wyd-db` | **38 = 0 abertas / 0 propostas / 35 fechadas / 3 externas** | `lacunas.md` §1 |
| gate de documentação | **289 páginas, 287 fontes verificadas** | `docker compose --profile gate run --rm -T gate`, exit 0 |
| `git diff --check` | **exit 0** (sem erro; o Git avisa apenas sobre CRLF) | `git diff --check`, 2026-11-24 |
| `RF-DB` novos | **0** | nenhum requisito de personagem foi numerado |

As três páginas declaram as dependências que não fecham: seed de `content_version`/cliente 7662,
operação de criação fora de C4, versão otimista sem coluna, ausência de posição atual, carga de
inventário sem contrato e restauração não decidida. A1 continua autorizada e fora do foco; nenhuma
decisão de retenção, capacidade ou `Q-DP-2/validação` foi reaberta.

**Lote seguinte:** o segundo lote foi executado na mesma data e está medido em §6.5.5; não restam páginas
de fluxo ou estado na fila. A ordem do terceiro lote não foi decidida por este registro.

### 6.5.5 Execução do lote B — economia e custódia (2026-11-24)

As quatro páginas existem no worktree como `proposto` e não foram declaradas `aceitas`:
`docs/specs/fluxos/concessao-de-grant.md`, `docs/specs/fluxos/compra-e-credito.md`,
`docs/specs/estados/grant.md` e `docs/specs/estados/item-e-refino.md`. A medição de fechamento do lote é:

| Medida | Resultado | Fonte |
|---|---:|---|
| páginas autorais com corpo em `docs/` | **128** (124 com `**Estado:**` + 4 portas sem estado) | `find`/`grep` no worktree, 2026-11-24 |
| páginas bloqueadas | **19** (23 antes do lote) | `proposta-paginas.md` §6; tabela de §5 |
| páginas de fluxo/estado ainda bloqueadas | **0** | §5 |
| páginas de `specs/` com corpo | **17 de 20**; faltam `stakeholders`, `requisitos-nao-funcionais` e `rastreabilidade` | `proposta-paginas.md` §4.2 |
| lacunas do `wyd-db` | **38 = 0 abertas / 0 propostas / 35 fechadas / 3 externas** | `lacunas.md` §1 |
| gate de documentação | **293 páginas, 291 fontes verificadas** | `docker compose --profile gate run --rm -T gate`, exit 0 |
| `git diff --check` | **exit 0** | `git diff --check`, 2026-11-24 |
| `RF-DB` novos | **0** | nenhum requisito de economia foi numerado |

As quatro páginas declaram o que o DDL não fecha: cash sem coluna de saldo (extrato derivado), sem
vínculo entre `donate_debit` e `ops.delivery_queue`, sem issue que cubra cash (`WYDDB-020` exclui loja e
cash), `donate_*` sem FK para `game.account` (`P-17`), grant de item só para personagem (não para o baú),
`claimed` sem prazo nem retorno decidido, piedade presa ao slot (troca de item não a zera), `item_index`
sem FK para `0014`, seed `0023` bloqueado e ausência de `load_inventory` em C4. Nenhuma tabela, coluna,
índice, job, endpoint ou regra foi criada; as 38 lacunas e as 7 decisões de retenção seguem intactas, e
A1 continua autorizada e fora do foco.

## 6.6 Pacote de decisões — o que destrava cada página bloqueada (2026-11-24)

> [!NOTE] Estado: **5 aprovadas, 6 aguardando decisão** (2026-11-24)
> `Q-BLK-01` a `Q-BLK-05` foram aprovadas pelo responsável em 2026-11-24 (todas em A) e estão registradas
> em §6.6.1–§6.6.5; as outras seis continuam **recomendadas, aguardando decisão**. O texto de cada uma é
> recomendação técnica com a evidência que a sustenta — aprovar uma pergunta **não** escreve página: a
> escrita é do lote de §6.6.13, executado para as três primeiras em §6.6.14 e para `04`/`05` em §6.6.17.

**Estado vigente, medido em 2026-11-24:** das **19** páginas para as quais este pacote foi escrito,
**14 saíram** — **11 escritas** (`imagens-e-evidencias`, `plano-de-manutencao`, `backup-e-restore`,
`recuperacao-de-desastre`, `arquitetura/fluxo-de-dados`, `sustentacao`, `manutencao-rotineira`,
`fixtures-e-factories`, `dados-de-teste`, `guia-desenvolvedor/pool-e-transacoes` e
`specs/requisitos-nao-funcionais`) e 3 retiradas do teto (`Q-BLK-01`). Restam
**1**: depende de execução (`metricas.md`, §6.6.12); as **4** que dependiam de decisão
(`Q-BLK-08`–`Q-BLK-11`) foram aprovadas em A e escritas em 2026-11-24 (§6.6.19).
Uma estimativa foi corrigida pela execução: `Q-BLK-02` previa 2 páginas e entregou **1** —
`manutencao-rotineira` também precisa do inventário de papéis, então conta em `Q-BLK-04`.

**Pergunta:** "quais decisões eu preciso tomar para destravar as **7** páginas bloqueadas, em que ordem, e
o que cada uma libera?" Esta seção responde em um lugar só. `proposta-paginas.md` §6 continua dona do motivo
de cada bloqueio; a lista nominal das 7, com dono e marco, é §5 **deste** registro.

**Os rótulos `Q-BLK-nn` são nomes de pergunta deste pacote, não IDs de `lacunas.md`.** Não são lacuna
nova: as **38** seguem **0 abertas / 0 propostas / 35 fechadas / 3 externas**, nenhuma é reaberta e nenhum
`RF-DB` novo nasce. Também não há ADR: nenhuma decisão estrutural foi tomada.

**Caminho recomendado para seguir daqui:** responder `Q-BLK-06` e `Q-BLK-07` juntas — são as duas que
criam **número** que outra página já procura (`configuracao.md` declara o pool ausente e `WYDDB-021`
espera o timeout; o aceite espera o limiar de RNF), nenhuma depende de terceiro e cada uma libera uma
página; depois `Q-BLK-08` (que depende de `07`), e as três de leitura (`10`, `09`, `11`) em paralelo.

**Critério de ordem (alavancagem, não ID):** primeiro o que tira **mais páginas** da fila; no empate,
o que fixa **regra que outra página ou outro sistema já cita**; depois o que é **pré-requisito** de outro
item; por último o que só afeta a própria página. "Sai da fila" = escrita com fonte fechada **ou**
retirada do teto por decisão explícita; só a segunda muda o total `147`.

**Censo e formato, ditos uma vez:** o censo medido depois do lote F é **144 = 143 + 1** (§6.6.19) e a
única página que resta não é pergunta: depende de `M20`. Cada seção fecha com "**Aprovar:**" e as letras;
as **onze** primeiras foram respondidas em 2026-11-24 (§6.6.14 a §6.6.19) e as **cinco** do lote G também,
em **A**, na mesma data — decisão em §6.6.20 e execução em §6.6.21.
Pré-requisito e custo de cada item ficam no quadro acima, não se repetem nas seções.

### Quadro-resumo

| Ordem | Decisão (pergunta fechada) | Páginas que caem | Por que nesta posição | Pré-requisito | Custo estimado |
|---|---|---:|---|---|---|
| — | `Q-BLK-01` **aprovada** | **4** | decidida em 2026-11-24 | nenhum | 1 sessão + 2–3 h |
| — | `Q-BLK-02` **aprovada** | **1** (2 com `Q-BLK-04`) | decidida em 2026-11-24 | retenção já decidida (`Q-RET-1/2/4/6`) | 1 sessão + 3–4 h |
| — | `Q-BLK-03` **aprovada** | **2** | decidida em 2026-11-24 | `Q-RET-7` e `Q-OPS-2` (decididas) | 1 sessão + 4–6 h |
| — | `Q-BLK-04` **aprovada** | **2** | aprovada em A em 2026-11-24; escrita no lote D (§6.6.17) | `Q-BLK-02` (decidida) | feito |
| — | `Q-BLK-05` **aprovada** | **2** | aprovada em A em 2026-11-24; escrita no lote D (§6.6.17) | `M19` (fechada) | feito |
| — | `Q-BLK-06` **aprovada** | **1** | aprovada em A em 2026-11-24; escrita no lote E (§6.6.18) | nenhum | feito |
| — | `Q-BLK-07` **aprovada** | **1** | aprovada em A em 2026-11-24; escrita no lote E (§6.6.18) | nenhum | feito |
| — | `Q-BLK-08` **aprovada** | **1** | aprovada em A em 2026-11-24; escrita no lote F (§6.6.19) | `Q-BLK-07` (aprovada em 2026-11-24) | feito |
| — | `Q-BLK-10` **aprovada** | **1** | aprovada em A em 2026-11-24; escrita no lote F (§6.6.19) | `Q-OPS-2` (decidida) | feito |
| — | `Q-BLK-09` **aprovada** | **1** | aprovada em A em 2026-11-24; escrita no lote F (§6.6.19) | nenhum | feito |
| — | `Q-BLK-11` **aprovada** | **1** | aprovada em A em 2026-11-24; escrita no lote F (§6.6.19) | nenhum | feito |
| — | `Q-BLK-12` **aprovada** | **0** | aprovada em A em 2026-11-24; os testes vieram primeiro e as 12 páginas saíram no lote H (§6.6.21) | nenhum | feito (1 sessão) |
| — | `Q-BLK-13` **aprovada** | **0** | aprovada em A; as **12** páginas de `tests/` e a matriz `CAP-nn` estão escritas (§6.6.21) | `Q-BLK-12` = A | feito (1 sessão) |
| — | `Q-BLK-14` **aprovada** | **0** | aprovada em A: 3 lotes por onda, o primeiro depois do aceite da 1ª leva; **0** issues criadas | aceite da 1ª leva | 1 sessão por lote |
| — | `Q-BLK-15` **aprovada** | **0** | aprovada em A: **19** blocos `RF-DB-1xx` por domínio, na frente 4 | `Q-BLK-14` (ordem da frente 1) | 19 blocos |
| **7** | `Q-BLK-16` — aceite do `wyd-db` ou outro sistema | **1** (`metricas`) | aprovada em A: aceite em 3 levas, a frente seguinte; é o único caminho para a série, `M20` e 144/144 | `Q-BLK-12`–`15` | **205** atos, 100–205 h |
| **8** | `Q-BLK-17` — por onde começa a leva de aceite: `tests/` ou `migrations/` | **0** | **respondida** em A pela execução em 2026-11-24: a leva 1 começou por `tests/` e as 12 páginas foram aceitas (§6.6.23); custo realizado: **12** atos | nenhum | feito |
| **9** | `Q-BLK-18` — página de teste aceita libera código, ou o código espera a issue | **0** | **respondida** em A pelo dono em 2026-11-24: o aceite libera o código; as 37 de `src/` foram aceitas e `dsn.rs` nasceu (§6.6.27); custo realizado: **37** atos | `Q-BLK-17` (resolvida) | feito |
| **10** | `Q-BLK-19` — como sai a dívida das 11 capacidades sem requisito | **0** | **respondida** em A pelo dono em 2026-11-24: 6 `RNF-DB-008`–`013`, um por gate; as 11 capacidades repontadas e a dívida zerada (§6.6.28); custo realizado: **6** blocos | `Q-BLK-17` (resolvida) | feito |
| **11** | `Q-BLK-20` — o dicionário `banco-catalogo.md` na `1.0.0` ou pós | **0** | **fechada em 2026-11-24 (A)**, pelo dono: o dicionário fica pós-`1.0.0`, `CAP-08` é declarada não executável e o gate `catálogo` vira conferência manual (§6.6.31); custo realizado: 1 declaração, sem PR de ferramentas | `Q-BLK-17` (resolvida) | feito |
| **12** | `Q-BLK-21` — quem revisa as 12 páginas para o item 9 de §10 | **0** | **aprovada** em A em 2026-11-24: o dono revisou e assinou; as 12 estão `aceito` (§6.6.23) | `Q-BLK-17` (resolvida) | feito |
| **13** | `Q-BLK-22` — as 12 páginas contam no critério de 144 | **0** | **fechada em 2026-11-24 (A)**, pelo dono: as 12 de `tests/` não contam e o critério segue **144** (§6.6.31) | nenhum | feito (decide o critério) |
| — | `Q-BLK-23` **aprovada** | **0** | **respondida** em A pelo dono em 2026-11-24: os **16** cenários (`CAP-24`–`CAP-39`) escritos antes da leva de `migrations/` (§6.6.29); custo realizado: **16** cenários, 1 sessão | `Q-BLK-17` (resolvida) | feito |

Soma: **17 das 19 originais** saem por decisão. Das duas restantes, uma era `fluxo-de-dados.md` — escrita
em 2026-11-24 — e a outra é `metricas.md`, que está em §6.6.12 e **não** é pergunta. As duas que restavam (`Q-BLK-20` e `22`) foram decididas em **A** em 2026-11-24 (§6.6.31); **0** restam.

**Respostas registradas em 2026-11-24:** `Q-BLK-01` = **retirar as três do teto** (147 → 144) e escrever
`imagens-e-evidencias`; `Q-BLK-02` = **`cron` do host no Compose**; `Q-BLK-03` = **storage do mesmo
provedor da VPS, RPO 24 h e RTO 4 h**; `Q-BLK-04` = **A** (quatro funções, nome real só para o dono do
produto); `Q-BLK-05` = **A** (factory por agregado, limpeza por chave e 5 contas); `Q-BLK-06` = **A** e
`Q-BLK-07` = **A**. Em 2026-11-24 o responsável aprovou também `Q-BLK-08`, `Q-BLK-09`, `Q-BLK-10` e
`Q-BLK-11` em **A**, e as quatro páginas foram escritas no **lote F** (§6.6.19). A execução do lote C está
em §6.6.14, a do lote D em §6.6.17, a do lote E em §6.6.18, e o censo medido agora é **144 = 143 + 1**.
Em 2026-11-24 o responsável aprovou as **cinco** do lote G — `Q-BLK-12` a `Q-BLK-16` — em **A**, e o lote H
(§6.6.21) escreveu as **12** páginas de `tests/`, com a matriz `CAP-nn` de **23** capacidades. **16 de 16**
decisões do pacote original estão respondidas; o **lote I** (§6.6.22) abriu **4** novas — `Q-BLK-17` a
`Q-BLK-20` — e o **lote J** (§6.6.23) abriu **2** — `Q-BLK-21` e `Q-BLK-22` —, todas do dono; a `17` foi
respondida pela execução e a `21` pelo dono, ambas em 2026-11-24, e nenhuma delas é respondida aqui. As duas
últimas — `Q-BLK-20` e `Q-BLK-22` — foram respondidas em **A** em 2026-11-24 (§6.6.31); o pacote fica com
**0** abertas. O ranking publicado das frentes do projeto é de
`../wyd-ops/docs/prioridade-do-projeto.md`. A fila de §5 continua em **1** página, que não é pergunta.

**Frentes que saem do impasse, por decisão:** operação — `Q-BLK-02`, `Q-BLK-03`, `Q-BLK-04` e `Q-BLK-10`
(**6 páginas**); documentação — `Q-BLK-01` (**4**); qualidade e teste — `Q-BLK-05`, `Q-BLK-07` e `Q-BLK-08`
(**4**); arquitetura e esquema — `Q-BLK-11` (**1**); requisito e produto — `Q-BLK-09` (**1**); conexão e
pool — `Q-BLK-06` (**1**). `metricas` e `fluxo-de-dados` não entram em nenhuma destas frentes por decisão:
saem por execução (§6.6.12).

### 6.6.1 `Q-BLK-01` — guia de documentação: retirar do teto ou inverter o dono (4 páginas)

**Pergunta fechada:** "das quatro páginas de `guia-documentacao/`, três repetem seções que já existem em
`docs/convencoes.md` — retirar as três do teto (**147 → 144**) e escrever só `imagens-e-evidencias`, ou
mover as seções de `convencoes.md` para as três páginas e mantê-las nas 147?"

**Recomendação: retirar as três do teto e escrever só `imagens-e-evidencias.md`.**

Evidência medida no worktree:

- `docs/convencoes.md` **§Links e citações** (linha 114) já fixa link relativo, citação `arquivo:linha`, a
  proibição de link para fora de `docs/` e o `code` para página planejada;
- **§Diagramas** (linha 266) já tem a tabela pergunta → tipo de diagrama, um tipo por diagrama, rótulos em
  português e o que reprova;
- **§Recursos do site** (linha 295) já tem a tabela de recursos que a 2.0 renderiza, a lista de proibidos e
  a cor por tipo de aviso;
- as seções obrigatórias que a tabela de `wyd-db-1.0.0.md` §6.14 exigia para `markdown-e-mkdocs`,
  `diagramas` e `links-referencias-e-ids` **eram** essas regras: escrevê-las criaria dois donos para o
  mesmo fato (`AGENTS.md` §5, item 5). A tabela as retirou em 2026-11-24, e a nota ao fim de §6.14 diz que o
  conteúdo delas já é de `convencoes.md` §Recursos do site, §Diagramas e §Links e citações;
- `markdown-e-mkdocs` teria de enunciar **quais fences a 2.0 renderiza**, e a única fonte disso é o config
  do site, que o projeto decidiu **não ter**: `convencoes.md` §Diagramas (linha 269) condiciona o fence
  `mermaid` ao “config do site”, item da frente `wyd-docs` (`EXT`); `AGENTS.md` §9 diz “sem `mkdocs.yml` …
  Não crie `mkdocs.yml`”; `lacunas.md` `D-B` (linha 435) manda não recriar config de site no diretório de
  planejamento. A montagem é `wyd-docs/preparar.py`, citado **sem caminho** em `convencoes.md` §Recursos do
  site;
- `imagens-e-evidencias` é a única das quatro **sem** seção equivalente: peso, formato, texto alternativo e
  “o que nunca aparece na captura” não estão em `convencoes.md` (zero ocorrências de “imagem”, “alternativ”
  e “peso” no arquivo).

**Se não decidir:** as quatro continuam na fila e o teto continua 147; três delas não podem ser escritas
sem violar “um fato, um dono” — ficam bloqueadas por decisão, não por fonte, e a fila não anda por esse lado.

**Impacto:** **4 das 19** (3 retiradas + 1 escrita). Frentes: documentação do `wyd-db`, regra de escrita para
os sistemas que publicam no site (`wyd-docs`) e o dono do teto (`plano-de-documentacao`). Aprovar muda
**147 → 144** e **19 → 15** e exige atualizar **todos** os donos de número na mesma entrega:
`proposta-paginas.md` §2 e §6, este registro §1/§2/§5/§6, `docs/planos/plano-de-documentacao.md` §3,
`wyd-db-1.0.0.md` (§ medido e a fila), `../wyd-1.0.0.md` §1.2 e §6.2 e `docs/index.md`.

**Pré-requisito:** nenhum. **Custo:** 1 sessão + 2–3 h (1 página + as atualizações de contador).

**Alternativa (não recomendada):** mover §Links, §Diagramas e §Recursos de `convencoes.md` para as três
páginas, deixando ponteiro. Mantém o teto em 147, mas mexe no contrato de leitura citado por quase toda
página e custa 2–3× mais.

**Resposta do responsável (2026-11-24): aprovada.** Retirar as três do teto e escrever só
`imagens-e-evidencias.md`. Aplicado: as três saíram de `proposta-paginas.md` §2/§6 e de §5 deste registro;
`docs/guia-documentacao/imagens-e-evidencias.md` foi escrito como `proposto`; o teto foi para **144** em
`plano-de-documentacao.md` §3, `wyd-db-1.0.0.md` §4 e `../wyd-1.0.0.md` §1.2. O `convencoes.md` **não** foi
alterado: a regra continua lá, e o índice da pasta aponta para ela.

### 6.6.2 `Q-BLK-02` — quem executa os quatro expurgos (2 páginas)

**Pergunta fechada:** "os expurgos de `portal.person_token`, `ops.login_attempt`, `game.pin_reset` e
`market_notification` rodam por `cron` do host (Compose, `psql` com DSN por variável) ou por `pg_cron`
agendado dentro do banco?"

**Recomendação: `cron` do host no Compose.**

Evidência:

- as quatro políticas estão decididas e só esperam execução — `pendencias-de-fechamento.md` §4.3 registra
  a linha de execução das quatro (`Q-RET-1/aplicação`, `Q-RET-2/aplicação`, `Q-RET-4/aplicação` e
  `Q-RET-6/aplicação`), mas **nenhuma dessas linhas traz o prazo**: os 30 dias de `Q-RET-1`, `Q-RET-2` e
  `Q-RET-4` e os 90 dias só para notificação lida de `Q-RET-6` estão em §3.1 e em
  `docs/proposta-retencao-pendente.md` §9;
- a quarta tabela é **`portal.market_notification`** (`ddl-confirmado.md`, linha 4645); §4.3 a escreve sem
  schema, enquanto as outras três aparecem qualificadas;
- `Q-RET-3 = A` não gera job: a retenção é o ciclo de vida da pessoa (`§3.1`);
- `Q-OPS-2 = A` já escolheu **VPS com processo persistente e Compose**
  (`../wyd-ops/docs/proposta-decisoes-ops.md` §4 e §6) — o host passa a existir e é o candidato natural ao
  agendamento, enquanto `pg_cron` seria extensão nova no banco, com permissão elevada e dependência de
  imagem. A fonte de ops **não** fala de processo agendado: essa parte é leitura desta seção, não medição;
- `AGENTS.md` §5, item 7 e §9 exigem segredo só em `.env`/secret: o `cron` lê o DSN por variável do Compose, sem
  credencial dentro do banco.

**Se não decidir:** `plano-de-manutencao` não pode responder “quem faz” nem “registro”, e
`manutencao-rotineira` repete a falta — as duas continuam bloqueadas por decisão, com a política já pronta.

**Impacto:** **1 das 19 sozinha** — `plano-de-manutencao`; **2** com `Q-BLK-04`, porque
`manutencao-rotineira` também precisa do inventário de papéis. Frentes: operação do `wyd-db` e
agenda/alerta do `wyd-ops`, que hoje não tem dono do calendário das cinco rotinas (quatro expurgos e o
backup).

**Pré-requisito:** as decisões de retenção, todas já tomadas. **Custo:** 1 sessão + 3–4 h (calendário,
comandos e registro). Não cria índice nem job: a execução continua em §4.3.

**Resposta do responsável (2026-11-24): aprovada.** `cron` do host no Compose, sem `pg_cron`. Aplicado em
`docs/planos/plano-de-manutencao.md` (rotina, credencial declarada como pendente e registro);
`docs/guia-administrador/manutencao-rotineira.md` ficou para o lote D, porque dependia do inventário de
papéis (`Q-BLK-04`): foi escrito em 2026-11-24, depois da aprovação, junto das outras três páginas
(§6.6.17). Nenhum índice, `DELETE` ou job foi criado — as quatro aplicações de §4.3
continuam pendentes de execução por `wyd-ops`.

### 6.6.3 `Q-BLK-03` — destino do backup e RPO/RTO (2 páginas)

**Pergunta fechada:** "o backup diário de 30 dias vai para storage de objeto do **mesmo provedor da VPS**,
fora do host, com **RPO de 24 h** e **RTO de 4 h**?"

**Recomendação: sim.**

Evidência:

- `Q-RET-7 = A` já fixou 30 dias de cópia diária, rotação, criptografia fora do host e restore **uma vez por
  trimestre** (`docs/proposta-retencao-pendente.md` §9; resumo em `§3.3.1`); na data da pergunta faltavam
  **onde** e **quanto**, e o §9 registra a resposta de `Q-BLK-03` desde 2026-11-24;
- `Q-OPS-2 = A` já escolheu **VPS paga com processo persistente e Compose**, sem nomear provedor
  (`../wyd-ops/docs/proposta-decisoes-ops.md` §4/§6; `§3.3.1` lista provedor como não medido): contratar
  host e storage no **mesmo** provedor evita fornecedor novo, conta nova e contrato novo — o que a
  restrição de “experimental, sem gasto” de `§3.3.1.2` torna preferível;
- **RPO 24 h** é o que a política já entrega: cópia diária não sustenta promessa menor;
- **RTO 4 h** é o alvo que o ensaio trimestral de `Q-RET-7` vai medir; a decisão fixou a **cadência**
  (trimestral), não uma janela de 4 h — o número dá alvo ao ensaio e não substitui o ensaio.

**Ressalva honesta:** se o plano contratado da VPS não tiver storage de objeto, a alternativa sem gasto é
um provedor S3-compatible com nível gratuito; nesse caso a decisão passa a incluir fornecedor novo e o custo
sobe (cadastro + credencial + 1 sessão).

**Se não decidir:** `backup-e-restore` não tem “onde” nem “como medir sucesso” e `recuperacao-de-desastre`
fica sem “prazo alvo”; as duas seguem presas ao mesmo item, com a política de retenção já decidida e sem uso.

**Impacto:** **2 das 19**. Frentes: operação (`wyd-ops`: rotina, RPO/RTO, ensaio) e guia do administrador do
`wyd-db`. **Pré-requisito:** `Q-RET-7` e `Q-OPS-2`, ambas decididas. **Custo:** 1 sessão + 4–6 h (2 páginas e
o procedimento). Não cria rotina nem contrato.

**Resposta do responsável (2026-11-24): aprovada.** Storage de objeto do **mesmo provedor da VPS**, fora do
host, com **RPO 24 h** e **RTO 4 h**. Aplicado em `docs/guia-administrador/backup-e-restore.md` §1–§3 e em
`docs/guia-administrador/recuperacao-de-desastre.md` §1; a linha de `Q-RET-7/aplicação` em §4.3 passou a
registrar destino e prazos. Contrato e região continuam de `wyd-ops`; a credencial administrativa
**não** é de `Q-BLK-04`, que a deixou explicitamente de fora (§6.6.4, item (e)): ela segue sem dono
nomeado em `configuracao.md` e em `guia-administrador/index.md` §3. A decisão não contrata nem configura
nada.

### 6.6.4 `Q-BLK-04` — inventário de papéis e contatos da sustentação (2 páginas)

**Resposta do responsável (2026-11-24): A.** Páginas escritas no mesmo dia:
[`sustentacao.md`](docs/guia-administrador/sustentacao.md) e
[`manutencao-rotineira.md`](docs/guia-administrador/manutencao-rotineira.md); medição em §6.6.17. O
restante desta seção é o registro da recomendação, mantido como histórico.

**(a) O que está em jogo:** sem nome de função, `sustentacao` e `manutencao-rotineira` ficam sem “quem
faz” — e duas páginas escritas e a porta do guia usam “operador” sem que ele exista em lugar nenhum.

**(b) Opções reais**

| Opção | O que a página passa a dizer | Consequência | Veredito |
|---|---|---|---|
| **A** | quatro funções — dono do produto, operador do banco, revisor de página, suporte ao jogador — com nome real **só** para o dono do produto; as demais por função e canal | escreve as duas páginas com o que a fonte sustenta | **Recomendada** |
| B | inventário nominal completo, com nomes, horários e escala de plantão | exige dado que nenhum arquivo tem: horário e escala viram `H` disfarçado de `V` | **Não recomendada** |
| C | uma função só, “dono”, sem separar operação de revisão | junta quem opera de quem revisa — o defeito que `sustentacao` existe para evitar | **Não recomendada** |

**(c) Recomendação e razão:** A. O único nome registrado no planejamento é o do dono do produto (`M32`),
e “operador” já existe como papel **distinto** do QA, com nome ainda não publicado (`LCLI-03`, no
`wyd-client`); nomear quatro funções é o máximo que a fonte paga.

**(d) Custo da alternativa não recomendada:** B inventa escala — reprova por `H` sem ID e a página volta
bloqueada na revisão; C faz a rotina atribuir vacuum e expurgo a “quem estiver por perto”, e
`recuperacao-de-desastre` §4 perde o dono que hoje nomeia “operador do banco”.

**(e) Desbloqueia:** `guia-administrador/sustentacao.md` e `guia-administrador/manutencao-rotineira.md`
(`guia-administrador` **6 → 8**, a pasta fecha); fixa a regra “quem faz” que `plano-de-manutencao` §2
(“ato do operador”) e `recuperacao-de-desastre` §4 usam sem definição. A credencial **nominal** continua de
fora: é de `configuracao.md` e de `guia-administrador/index.md` §3.

**(f) Fonte fechada:** `wyd-db-1.0.0.md` §6.12 (seções obrigatórias das duas páginas); `lacunas.md` `M32`
(dono do produto nomeado); `../wyd-client/lacunas.md` `LCLI-03` (operador ≠ QA, nome na ficha);
`docs/guia-administrador/index.md` §3 (credencial administrativa sem nome); `docs/planos/plano-de-manutencao.md`
§2; `docs/guia-administrador/recuperacao-de-desastre.md` §4.

**Evidência fraca declarada:** não há organograma, escala nem segundo nome em nenhum arquivo do
planejamento; a opção A é a única que não preenche essa ausência com suposição. E o `LCLI-03` é o operador
da **captura do cliente**, não o operador do banco: ele serve de precedente para “papel sem nome”, não de
prova de que exista um operador de banco.

**Se não decidir:** as duas páginas seguem bloqueadas por decisão; a rotina tem calendário (lote C) e não
tem responsável. **Resposta:** A (2026-11-24).

### 6.6.5 `Q-BLK-05` — dado de teste, factory e catálogo de contas (2 páginas)

**Resposta do responsável (2026-11-24): A.** Páginas escritas no mesmo dia:
[`fixtures-e-factories.md`](docs/guia-teste/fixtures-e-factories.md) e
[`dados-de-teste.md`](docs/guia-teste/dados-de-teste.md); medição em §6.6.17. O restante desta seção é o
registro da recomendação, mantido como histórico.

**(a) O que está em jogo:** sem factory e sem catálogo, cada teste improvisa conta e dado — e o `wyd-bots`
já declarou que “não há fixture, seed de bot nem conta de teste”.

**(b) Opções reais**

| Opção | O que a página passa a dizer | Consequência | Veredito |
|---|---|---|---|
| **A** | factory por agregado, banco (ou transação) por teste com limpeza por chave, e **5 contas** — jogador, staff, administrador, operador, bot — com a senha de dev de `M19` | fecha as duas páginas e dá ao bot a fixture que ele não tem | **Recomendada** |
| B | um seed/dump único compartilhado por toda a suíte | começa mais rápido, mas cria estado global entre testes | **Não recomendada** |
| C | contas reais em ambiente comum | usa conta e dado de cliente | **Não recomendada** |

**(c) Recomendação e razão:** A. `AGENTS.md` §8 já normatiza factory e proíbe id mágico, `sleep` como
sincronização e estado global — a regra existe, falta o concreto; `M19` já fixou a senha de dev; e os
cinco papéis não são invenção: `person.role` tem `member`, `moderator` e `admin`
(`dicionario-dados/gerado/person.md`), “operador” é o do ambiente (`guia-administrador/index.md` §3) e o
bot precisa de conta para entrar — o contrato B3 de `wyd-bots` recusa “conta real” e pede fixture.

**(d) Custo da alternativa não recomendada:** B reproduz o estado global que §8 proíbe, e o resultado de
um teste passa a depender da ordem dos outros; C faz o que os 6 seeds bloqueados do cliente 7662 já não
podem fazer (dado de cliente no banco, §4.2) e deixa estado que o reset precisa apagar.

**(e) Desbloqueia:** `guia-teste/fixtures-e-factories.md` e `guia-teste/dados-de-teste.md` (`guia-teste`
**5 → 7**, a pasta fecha); fixa a regra de dado que a suíte e o `wyd-bots` consomem, inclusive a senha
única de desenvolvimento.

**(f) Fonte fechada:** `AGENTS.md` §8 (factory; proibição de id mágico, `sleep` e estado global);
`lacunas.md` `M19` (senha de dev do seed); `wyd-db-1.0.0.md` §6.13 (o que `dados-de-teste` pede);
`docs/dicionario-dados/gerado/person.md` (valores de `role`); `../wyd-bots/docs/proposta-decisoes-bots.md`,
bloco “Medido” (“não há fixture, seed de bot nem conta de teste”).

**Evidência fraca declarada:** não existe suíte, então “limpeza por chave” é desenho, não prova; e os
cinco papéis são proposta de cobertura, não um inventário medido de contas.

**Se não decidir:** as duas páginas continuam “desenho da suíte ausente” e a suíte nasce sem catálogo.
**Resposta:** A (2026-11-24).

### 6.6.6 Ordem 1 — `Q-BLK-06` — pool e timeouts (1 página)

**Correção de premissa, antes das opções.** A formulação anterior era **estimativa**: propunha **5 s** de
aquisição e **30 min** de idle, e nenhum dos dois é o default do driver. Os defaults reais do `sqlx` são
**aquisição 30 s** e **idle 10 min**; o `max_connections = 10` que eu já vinha citando é default do `sqlx`,
não invenção. Também não existe “teto de conexões do MySQL no Compose”: o projeto é **PostgreSQL 17**, e o
Compose do repositório **não tem serviço de banco** (só `docs` e o perfil `gate`) — o teto aplicável é o
default do servidor PostgreSQL.

**(a) O que está em jogo:** quem abre pool não sabe quantas conexões pode abrir, quanto espera e quando
desiste — e `configuracao.md` declara, na seção de lacunas, que isso não tem desenho.

**Os números e de onde vêm** (nada abaixo é estimativa):

| Número | Valor | Origem | Tipo |
|---|---:|---|---|
| `max_connections` do servidor | **100** | default do PostgreSQL 17; o projeto **não** fixa em nenhum arquivo (`grep` por `max_connections` só acha os pools do runner/provision) | `V` (default do servidor) |
| conexões utilizáveis por papel comum | **97** | 100 − `superuser_reserved_connections` (3), default do PostgreSQL; o próprio `sqlx` documenta 97 | `V` |
| pool de execução do runner | **1** | `src/migrate/adapter/pg.md` (“a série é serial”) | `V` |
| pool de lock do runner | **1** | mesmo arquivo | `V` |
| pool do `provision` | **1** | `src/provision/adapter/pg.md` | `V` |
| `lock_timeout` do runner | **10 s** (faixa 1 s–10 min) | `configuracao.md` (tabela de parâmetros); `modelos-e-invariantes.md`; `src/migrate/config.md` | `V` |
| pool padrão do `sqlx` | **10**, mínimo 0 | `PoolOptions::new()` do `sqlx` | `V` (default do driver) |
| `acquire_timeout` padrão | **30 s** | `sqlx` | `V` |
| `idle_timeout` padrão | **10 min** | `sqlx` | `V` |
| `max_lifetime` padrão | **30 min** | `sqlx` | `V` |
| processos com pool persistente | **4** — jogo, portal/BFF, `wyd-admin`, leitura | `contexto-c4.md` (uma credencial por serviço) + `AGENTS.md` §6 (“um processo por canal”) | `V` |
| `statement_timeout` | **não declarado** | nada no repositório o fixa; default do PostgreSQL é **0** (sem limite) | `V` (ausência) |

**Orçamento de conexões (aritmética, não meta):** 4 pools de runtime × 10 = **40**; runner 2 + provision 1 =
**3**; pool de teste 10 = **10**. Total **53** de **97** utilizáveis — **44 livres**. Se o servidor for
configurado com `max_connections` menor que 53, a soma reprova; hoje ninguém a configurou.

**(b) Opções reais**

| Opção | O que a página e `configuracao.md` passam a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | declarar o que **já vale**: `pool = 10` por processo, aquisição **30 s**, idle **10 min**, vida **30 min**; `statement_timeout` fica em **0** (default), com pendência explícita; `WYDDB-021` lê o **acquire de 30 s** | escrever 4 linhas em `configuracao.md` e 1 seção em `pool-e-transacoes`; nenhum número novo nasce | **Recomendada** |
| B | não declarar nada e herdar o `sqlx` | nenhum trabalho agora; a linha de parâmetro fica vazia e `WYDDB-021` continua sem fonte | **Não recomendada** |
| C | dimensionar por concorrência medida antes de fixar | espera medição que não existe: `M20` mede contagem de arquivo e tabela, não concorrência | **Não recomendada** |

**(c) Recomendação e razão:** A. Os quatro valores são **defaults reais** (`sqlx`) e o teto é **default real**
(PostgreSQL 17) — declarar não inventa nada, só dá dono ao que já acontece; `WYDDB-021` exige “o timeout já
configurado, **sem inventar número**” e hoje não há número, então o DoD não tem o que ler; e o orçamento
(53/97) fecha com folga, o que a página pode mostrar em vez de prometer.

**(d) Custo de cada opção:** **A** custa uma decisão de 15 min e quatro linhas de parâmetro; em troca,
`configuracao.md` deixa de ter lacuna e o shutdown ganha limite. **B** é grátis e mantém o defeito: a página
`pool-e-transacoes` fica sem “configuração e defaults” e `WYDDB-021` segue bloqueada por falta de fonte,
não de código. **C** trava a página até a série existir e, quando ela existir, ainda não entrega concorrência
— não resolve o que promete.

**(e) O que andou quando a resposta veio (2026-11-24):** `guia-desenvolvedor/pool-e-transacoes.md`
(1 página, escrita; a fila caiu de **7** para **6**); a seção de parâmetros em `configuracao.md` (dono dos
parâmetros); o DoD de `WYDDB-021` (shutdown com timeout real); e a frente de operação, porque a rotina e o
diagnóstico passam a citar limite em vez de “não há desenho”. Não fecha `WYDDB-026` (threshold de pool segue
fora de escopo dela) nem `WYDDB-005` (driver).

**(f) Fonte fechada:** `docs/guia-administrador/configuracao.md` (lacuna declarada do pool; `lock_timeout`
10 s); `src/migrate/adapter/pg.md` e `src/provision/adapter/pg.md` (pools = 1);
`docs/roadmap/WYDDB-021.md` (DoD do shutdown); `WYDDB-026.md` e `WYDDB-005.md` (pool fora de escopo);
`docs/guia-desenvolvedor/portas-e-contratos.md` §2 (“o mesmo pool”, sem tamanho); `AGENTS.md` §6 (um
processo por canal); defaults do `sqlx` (`PoolOptions::new`: 10 / 30 s / 10 min / 30 min) e do PostgreSQL 17
(`max_connections` 100, `superuser_reserved_connections` 3 → 97).

**O que continua sem fonte:** `statement_timeout` — nenhum arquivo o fixa, e o default do servidor é 0. A
opção A o deixa em 0 **declarado**, com pendência aberta; fixá-lo agora seria inventar.

**Se ficar aberta:** `pool-e-transacoes` continua bloqueada e `WYDDB-021` continua sem número para ler.
**Aprovar:** A / B / C.

**Resposta do responsável (2026-11-24): A.** Defaults reais declarados, `statement_timeout` em 0 **com
pendência aberta**, orçamento de 53 de 97 conexões.

**Aplicado:** `docs/guia-desenvolvedor/pool-e-transacoes.md` escrita em 2026-11-24 (oito seções, da
configuração ao teste que cobre) e `guia-administrador/configuracao.md` ganhou a seção **§3 “Pool e timeouts
da conexão”**. Medição do lote em §6.6.18.

### 6.6.7 Ordem 2 — `Q-BLK-07` — limiares de RNF (1 página)

**A pergunta estava mal formulada.** Ela pedia para aprovar **p95 ≤ 50 ms**, **boot ≤ 10 s** e **100
conexões** — três números sem fonte no repositório; e “100 conexões” **contradiz o teto real**: o PostgreSQL
tem 100 de `max_connections` e só **97** utilizáveis. Fixar aqueles valores seria estimativa com aparência de
decisão. A pergunta reformulada é: **“a página de RNF nasce com limiar só onde há fonte e ‘a medir’ onde não
há?”**

**(a) O que está em jogo:** a página exige “métrica, limiar, como medir, condição de reprovação”, e
`retencao-e-capacidade.md` registra que **nenhum** limiar de latência ou capacidade foi aprovado.

**Os números que existem, e a origem de cada um:**

| Número | Valor | Origem | Tipo |
|---|---:|---|---|
| cobertura mínima | **85%** por sistema | `AGENTS.md` §8 | `V` |
| espera máxima de DDL | **10 s** (`lock_timeout`, faixa 1 s–10 min) | `configuracao.md`; `src/migrate/config.md` | `V` |
| `SKIP`/`#[ignore]` novo | **0** (reprova) | `gates-e-anti-skip.md` | `V` |
| conexões simultâneas | teto **97** utilizáveis; uso **não medido** | default do PostgreSQL 17 | `V` (teto) / ausência (uso) |
| latência p95 | **não medida**; nenhum limiar aprovado | `retencao-e-capacidade.md` (“Existe limiar de latência ou capacidade? **não**”) | ausência |
| tempo de boot | **não medido**; `WYDDB-040` fala de boot sem prazo | `roadmap/WYDDB-040.md` | ausência |
| tempo da suíte | regra “teto próprio”, **sem número** | `testes-de-sistema.md` §Tempo limite | ausência |

**(b) Opções reais**

| Opção | O que a página passa a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | **três `RNF-DB` com número rastreável** — cobertura 85%, espera de DDL ≤ 10 s, `SKIP` = 0 — e **dois com métrica e “sem limiar até `M20`”** (latência de leitura de personagem, tempo de boot), cada um com “como medir” e “condição de reprovação” (medir sem limiar aprovado **reprova**) | a página fica honesta e menor; o aceite ganha três critérios verificáveis hoje | **Recomendada** |
| B | fixar p95 50 ms, boot 10 s e 100 conexões como `P` | três números sem fonte, um deles acima do teto de 97; a página passa a afirmar o que ninguém mediu | **Não recomendada** |
| C | esperar a série e escrever só o medido | mantém a página bloqueada — e a espera não produz latência: `M20` mede contagem, não tempo de resposta | **Não recomendada** |

**(c) Recomendação e razão:** A. É a única que escreve a página **sem inventar**: usa os três limiares que já
existem no projeto e transforma a ausência de medição em pendência declarada, não em número. O critério de
reprovação “medir sem limiar aprovado reprova” impede que a página vire desculpa: quem medir precisa trazer
o número para aprovação.

**(d) Custo de cada opção:** **A** custa aceitar uma página de RNF com dois itens “a medir” — menos
confortável de ler, mas verdadeira; em troca, a Onda 3 sai do bloqueio e o aceite ganha critério. **B** custa
a credibilidade do pacote: o projeto já registrou que nada foi medido, e um `P` com número inventado seria
reaberto na primeira revisão. **C** custa tempo indeterminado e não entrega o que promete.

**(e) O que andou quando a resposta veio (2026-11-24):** `specs/requisitos-nao-funcionais.md` (1 página,
escrita; a fila caiu de **6** para **5**); nasceram os primeiros `RNF-DB-nnn` — `RNF-DB-001` a
`RNF-DB-005`, registrados em `specs/index.md`; a matriz `Q-BLK-08` deixa de esperar RNF; e o aceite por página (`M33`) ganha critério
verificável de qualidade. O teto de conexões também alimenta a resposta de `Q-BLK-06`.

**(f) Fonte fechada:** `docs/arquitetura/retencao-e-capacidade.md` (limiar: **não**); `lacunas.md` `M20`
(número medido só depois da série); `wyd-db-1.0.0.md` §6.5 (bloco do `RNF-DB`); `AGENTS.md` §8 (85% e
`SKIP`); `docs/guia-teste/gates-e-anti-skip.md` (regra de `SKIP`); `docs/guia-administrador/configuracao.md`
(`lock_timeout` 10 s); `docs/guia-teste/testes-de-sistema.md` §Tempo limite (regra sem número).

**O que esta opção não faz:** não mede nada e não promete latência. Quem quiser p95, boot e concorrência com
número precisa de `M20` executado — e o lugar desse número é `planos/metricas.md`, não esta página.

**Se ficar aberta:** RNF continua no bloqueio literal “nenhum limiar medido”, a matriz nasce sem linha de
RNF e o aceite segue sem critério de qualidade. **Aprovar:** A / B / C.

**Resposta do responsável (2026-11-24): A.** Três `RNF-DB` com limiar de fonte real e dois com métrica
declarada e **sem limiar até `M20`**.

**Aplicado:** `docs/specs/requisitos-nao-funcionais.md` escrita em 2026-11-24: `RNF-DB-001` (cobertura
≥ 85 %), `002` (espera de DDL ≤ 10 s), `003` (`SKIP` = 0), `004` (p95, a medir) e `005` (boot, a medir).
Medição do lote em §6.6.18.

### 6.6.8 Ordem 3 — `Q-BLK-08` — coluna de teste da matriz de rastreabilidade (1 página)

**Resposta do responsável (2026-11-24): A.** Aplicada em `docs/specs/rastreabilidade.md` (§6.6.19).

**Correção de premissa, antes das opções.** O rascunho desta seção tratava os insumos como se já
estivessem ligados: “16 `RF-DB`, 7 `RN-DB`, 3 `UC-DB` e 40 `WYDDB-nnn`”. Os quatro conjuntos existem —
medidos em 2026-11-24 —, mas **as arestas entre eles não**: nenhuma das 40 páginas de issue cita `RF-DB`,
`RN-DB` ou `UC-DB` (0 de 40), e os blocos `RF` apontam “issue a alocar na onda 8”. A matriz, portanto, não
nasce copiando vínculo pronto: nasce com cinco colunas, das quais **três têm fonte parcial** e **duas são
declaração** (`teste` e `issue`). A pergunta fechada é: **“a matriz nasce com o que existe e declara o que
falta?”**

**(a) O que está em jogo:** o molde do `wyd-db` pede cinco colunas — `RF/RN/RNF/UC → tabela → porta →
teste → issue` — e um dos critérios de conclusão da `1.0.0` depende desta página: “`RF`/`RN`/`RNF` cobrem
todas as tabelas e a matriz de rastreabilidade não tem lacuna sem dono” (`wyd-db-1.0.0.md` §11). É o único
critério que liga requisito a DDL; sem a matriz ele não é verificável, e hoje ninguém sabe quantas das 109
tabelas estão sob algum requisito.

**Os números e de onde vêm** (nada abaixo é estimativa):

| Insumo | Valor | Origem | Tipo |
|---|---:|---|---|
| blocos `RF-DB` | **16** (`001`–`016`) | `docs/specs/requisitos-funcionais.md` | `V` |
| blocos `RN-DB` | **7** (`001`–`007`) | `docs/specs/regras-de-negocio.md` | `V` |
| blocos `UC-DB` | **3** (`001`–`003`; `004` aparece só como “próximo livre”) | `docs/specs/casos-de-uso.md` | `V` |
| blocos `RNF-DB` | **5** (`001`–`005`), criados em 2026-11-24 por `Q-BLK-07` = A | `docs/specs/requisitos-nao-funcionais.md` | `V` |
| **linhas da matriz** | **31** = 16 + 7 + 5 + 3 | soma | `V` |
| tabelas e views do catálogo | **109** tabelas e **2** views | `docs/roadmap/WYDDB-040.md`, DoD do catálogo | `V` |
| tabelas nomeadas por um `RF` | **7** de 109 | `grep` dos nomes `game.*` em `requisitos-funcionais.md`, únicos | `V` |
| páginas de domínio com “Requisitos atendidos” | **1** de **19** | `docs/dicionario-dados/dominio-*.md` | `V` |
| portas do crate | **7** (5 de `migrate` + 2 de `provision`) + **C4** | `portas-e-contratos.md` §1–§2 e §4 | `V` |
| páginas de issue | **40** (`WYDDB-001`–`WYDDB-040`) | `docs/roadmap/WYDDB-*.md` | `V` |
| issues que citam `RF-DB`/`RN-DB`/`UC-DB` | **0** de 40 | `grep -l` em `docs/roadmap/WYDDB-*.md` | `V` |
| testes executados | **0** naquele lote (o crate ainda não tinha `Cargo.toml`; desde 2026-11-24 roda 6 de `dsn`, §6.6.27) | `cargo test` em `crates/wyd-db` | `V` |

**As arestas que já existem** (medidas, não presumidas):

| Aresta | Cobertura | Onde está escrita |
|---|---|---|
| `RF` → tabela e porta | **16 de 16** blocos têm o campo “Tabela e porta” | `requisitos-funcionais.md` |
| `UC` → requisitos cobertos | **3 de 3** blocos têm “Requisitos cobertos” | `casos-de-uso.md` |
| `RN` → onde é garantida | **7 de 7** blocos têm o campo; só **5** linhas citam `game.` | `regras-de-negocio.md` |
| `RNF` → métrica | **5 de 5** blocos têm métrica; **0** tem teste executado | `requisitos-nao-funcionais.md` |
| `tabela` → requisito | **15 de 109** tabelas nomeadas em `RF` ou `RN` (recontado em 2026-11-24 no lote F); **1 de 19** páginas de domínio com a seção reversa | `requisitos-funcionais.md`; `dicionario-dados/` |
| requisito → `issue` | **0 de 31** linhas | nenhum arquivo — aresta inexistente |

**(b) Opções reais**

| Opção | O que a página passa a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | as **5** colunas do molde para as **31** linhas: `tabela` e `porta` preenchidas onde a fonte declara (16 `RF`, 3 `UC`, 7 `RN`), `RNF` com métrica em vez de tabela; `teste` como **alvo planejado `P`** com o tipo (`contrato` para schema/catálogo, `sistema` para C4/sessão); `issue` como “a alocar na onda *N*” enquanto não houver vínculo; seção “sem cobertura” com as **31** linhas sem teste executado e as **102** de 109 tabelas sem `RF`/`RN` nomeado | 1 sessão + 3 h de leitura (31 blocos + 40 DoD) | **Recomendada** |
| B | esperar a suíte das ondas 4–6 e só então montar a matriz | nenhum trabalho agora; a página fica parada até existir código | **Não recomendada** |
| C | matriz só `RF → tabela` (2 colunas) | mais barata: 16 linhas, 2 colunas | **Não recomendada** |

**(c) Recomendação e razão:** A. Três das cinco colunas **já têm fonte parcial** e as outras duas são
declaração honesta, não invenção: `P` com alvo nomeado para `teste` e “a alocar na onda *N*” para `issue` —
nenhum teste é inventado e nenhum vínculo issue↔requisito é fabricado. A matriz também é a **única** forma de
medir o critério de §11: hoje o número real é 7 de 109 tabelas sob requisito nomeado, e 0 de 31 linhas com
issue — dois vazios que o dono não consegue ver sem esta página. E o molde pede `RNF` na matriz, o que só
passou a ser possível em 2026-11-24 (`Q-BLK-07` = A): sem `Q-BLK-08`, a série `RNF-DB` existe e não tem
onde ser rastreada.

**(d) Custo de cada opção:** **A** custa 31 linhas × 5 colunas escritas à mão; o molde já exige “regra de
atualização”, então o risco de envelhecer é tratado dentro da própria página. **B** custa zero agora e trava
a página em código que só nasce depois do planejamento (`AGENTS.md` §5, item 1) — e não resolve a coluna
`issue`, que **não depende de teste nenhum**: ela continuaria vazia mesmo com a suíte verde. **C** custa
menos e entrega menos: cai de 5 para 2 colunas, não serve ao molde nem ao critério de §11, e a rastreabilidade
volta a ser afirmação sem tabela.

**(e) O que andaria quando a resposta viesse:** `specs/rastreabilidade.md` escrita (1 página; a fila cairia
de **5** para **4**); o critério de §11 passaria a ser verificável — inclusive para reprovar; o aceite por
página (`M33`) ganharia o checklist que ele cita; e o par com `Q-BLK-07` fecharia, porque a linha de `RNF`
passaria a ter destino na matriz.

**(f) Fonte fechada:** `docs/specs/index.md` §Convenção de ID (16/7/3, e a série `RNF-DB-001`–`005`);
`docs/specs/requisitos-funcionais.md` (campo “Tabela e porta” em 16 de 16); `docs/specs/regras-de-negocio.md`
(“Onde é garantida” em 7 de 7); `docs/specs/casos-de-uso.md` (“Requisitos cobertos” em 3 de 3);
`docs/guia-desenvolvedor/portas-e-contratos.md` §1–§2 e §4 (7 portas + C4); `docs/roadmap/WYDDB-*.md`
(40 páginas, 0 citando requisito); `docs/roadmap/WYDDB-040.md` (109 tabelas e 2 views);
`wyd-db-1.0.0.md` §5.3 (todo requisito tem “destino na matriz”), §6.5 (molde das 5 colunas) e §11 (critério
de conclusão); `lacunas.md` `M33` (aceite).

**O que continua sem fonte:** a aresta requisito → `issue` (0 de 40) e o teste executado (0 no repositório).
As duas não são lacuna a fechar aqui: viram `P` declarado na página. Medir o vazio de tabelas (102 de 109) e
de issue (as 31 linhas) é o **produto** da matriz — se o dono quiser tratá-los como lacuna, quem registra é
`lacunas.md`, dono único (`AGENTS.md` §3).

**Evidência fraca declarada:** a coluna `teste` será `P` em **100%** das 31 linhas e a coluna `issue` estará
vazia em **100%** delas; a matriz prova cobertura **planejada**, e a página precisa dizer isso na primeira
seção, não na nota de rodapé. Defeito vizinho, encontrado na medição e **não corrigido aqui**: o aviso de
`requisitos-funcionais.md` afirma que “as 40 páginas `WYDDB-nnn` com DoD ainda não existem”, o que deixou de
ser verdade em 2026-11-22 — quem escrever a matriz corrige o aviso na mesma entrega.

**Se não decidir:** `specs/rastreabilidade.md` continua bloqueada, o critério de §11 segue sem verificação e
a série `RNF-DB` nasce sem destino de rastreio. **Aprovar:** A / B / C.

### 6.6.9 Ordem 4 — `Q-BLK-10` — topologia de implantação (1 página)

**Resposta do responsável (2026-11-24): A.** Aplicada em `docs/arquitetura/implantacao.md` (§6.6.19).

**O que a página precisa entregar, e o que já existe.** O molde a fixa como “C4 nível 4; processo, imagem,
portas, volume, supervisor; ligação com `docs/rust/05-stack-docker.md` do repositório (sem duplicar)”
(`wyd-db-1.0.0.md` §6.3). Do C4, o projeto tem **2 dos 4 níveis**: `contexto-c4.md` cobre nível 1 e nível 2 e
não tem nível 3 nem 4. E o arquivo que o molde manda citar **não existe**: `docs/rust/05-stack-docker.md`
`find` retorna 0 resultado, porque a `docs/` da raiz do repositório não existe (`../wyd-1.0.0.md` §2, `D8`).

**(a) O que está em jogo:** `plano-de-implantacao` já fixa papéis, as **6** DSNs, a ordem de **5** etapas e o
preflight — mas não diz **onde** isso roda. Container, volume e supervisor ficam sem forma, e sem eles o C4
não fecha o nível 4.

**Os fatos e de onde vêm** (nada abaixo é estimativa):

| Fato | Valor | Origem | Tipo |
|---|---|---|---|
| ambiente de produção | **VPS paga, processo persistente, deploy Compose** (`Q-OPS-2` = A, 2026-11-24) | `../wyd-ops/docs/proposta-decisoes-ops.md` §4 e §6 | `V` |
| o que `Q-OPS-2` **não** criou | VPS, contrato, rotina de backup, RPO/RTO e página de operação | mesma resposta do dono, §4 | `V` |
| níveis de C4 escritos | **2 de 4** (nível 1 e nível 2; sem nível 3 e 4) | `docs/arquitetura/contexto-c4.md` §1–§3 | `V` |
| serviços no Compose hoje | **1** serviço (`docs`) + **1** sob perfil (`gate`); **0** de banco | `docker-compose.yml`, medido em 2026-11-24 | `V` |
| DSNs por papel | **6** (`WYD_PROVISION_DSN`, `_MIGRATOR`, `_GAME`, `_PORTAL`, `_READ`, `_TEST`) | `src/provision/spec.md` §3 | `V` |
| papéis de cluster | **6** + `wyd_owner`, que é `NOLOGIN` | `docs/planos/plano-de-implantacao.md` §Verificação; `seguranca.md` | `V` |
| etapas de implantação | **5** (provisionar → série → conteúdo → serviços → verificação) | `plano-de-implantacao.md` §Ordem | `V` |
| regra de porta do banco | fora do loopback **só** com justificativa registrada | `plano-de-implantacao.md` §Pré-requisitos | `V` |
| processos de runtime com pool | **4** (jogo, portal/BFF, `wyd-admin`, leitura) | `contexto-c4.md` §2; `pool-e-transacoes.md` §2 | `V` |
| orçamento de conexões | **53 de 97** (44 livres) | `pool-e-transacoes.md` §2 | `V` |
| arquivo que o molde manda citar | `docs/rust/05-stack-docker.md` — **inexistente** | `find` no repositório: 0 resultado | ausência |

**(b) Opções reais**

| Opção | O que a página passa a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | **1 host**: PostgreSQL 17 em **1 container** com **1 volume nomeado**; **5432 não publicada** (rede interna do Compose); `provision` e runner como one-shot com o binário na máquina que executa; supervisor = Compose com `restart: unless-stopped`, o único que o Compose já usa; as **6** DSNs não mudam | escrever 1 página a partir de 4 arquivos que já existem; nenhum número novo | **Recomendada** |
| B | PostgreSQL gerenciado (DBaaS), sem container | contradiz `Q-OPS-2` = A e `Q-BLK-03` (backup no storage do mesmo provedor da VPS) | **Não recomendada** |
| C | vários hosts e **1 container por canal** já na `1.0.0` | multiplica supervisão sem medição de carga; a soma de conexões (53/97) cabe em 1 host | **Não recomendada** |

**(c) Recomendação e razão:** A. `Q-OPS-2` = A (2026-11-24) já escolheu o ambiente e **não** criou a página;
o trabalho desta decisão é a **forma**, e a forma sai do que já existe, sem inventar: a regra de porta do plano
de implantação (fora do loopback só com justificativa), o `restart: unless-stopped` que o Compose já usa no
serviço `docs`, os **4** processos e as **53 de 97** conexões que cabem em um host, e as **6** DSNs que não
mudam. O que falta — `05-stack-docker.md` — a página declara como pendência de link, em vez de citar arquivo
inexistente.

**(d) Custo de cada opção:** **A** custa 1 página e 4 leituras; o que ela não pode prometer é capacidade, e
não promete. **B** contraria duas decisões já tomadas — `Q-OPS-2` e `Q-BLK-03` — e cria dois donos para o
mesmo host lógico; o backup passaria a ter destino diferente do decidido. **C** cria um container por canal
para um `wyd-server` que declara **1 processo por canal** como fronteira, não como frota: dobra a supervisão
sem medir carga, e `AGENTS.md` §9 veda depender de plano gratuito para produção de jogo.

**(e) O que andaria quando a resposta viesse:** `arquitetura/implantacao.md` escrita (1 página; `arquitetura/`
passa de **15** para **16** páginas com corpo, e a **17** quando `modelo-conceitual` também fechar); o C4 ganha
o nível 4; e `wyd-ops` ganha a forma do serviço de banco, que hoje não existe no Compose.

**(f) Fonte fechada:** `../wyd-ops/docs/proposta-decisoes-ops.md` §4 e §6 (resposta de `Q-OPS-2`);
`docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md` (provisionamento fora da série);
`wyd-db-1.0.0.md` §6.3 (processo, imagem, portas, volume, supervisor);
`docs/planos/plano-de-implantacao.md` (pré-requisitos, ordem, verificação); `docker-compose.yml` (1 serviço,
`restart: unless-stopped`); `docs/guia-desenvolvedor/pool-e-transacoes.md` §2 (4 pools, 53/97);
`../wyd-1.0.0.md` §2 `D8` (a `docs/` da raiz não existe); `AGENTS.md` §9.

**O que continua sem fonte:** carga, conexões simultâneas e memória — nenhuma foi medida; “1 processo por
canal” vem da fronteira do `wyd-server`, não de capacidade. E a ligação exigida pelo molde aponta para arquivo
inexistente: a página registra a ausência, não a preenche.

**Evidência fraca declarada:** o supervisor não é decisão desta página, é leitura do `restart: unless-stopped`
que o Compose já usa; se o dono quiser systemd em vez de Compose para o banco, isso é pergunta nova, não
detalhe de redação.

**Se não decidir:** `implantacao` continua sem nível 4, `plano-de-implantacao.md` cita um “serviço” que não
tem forma e o C4 fica pela metade. **Aprovar:** A / B / C.

### 6.6.10 Ordem 5 — `Q-BLK-09` — atores da `1.0.0` (1 página)

**Resposta do responsável (2026-11-24): A.** Aplicada em `docs/specs/stakeholders.md` (§6.6.19).

**(a) O que está em jogo:** `stakeholders` precisa da matriz ator → requisito, e o conjunto de atores não
está escrito em lugar nenhum — o que existe são **6** credenciais no C4 e **3** valores de `person.role` no
DDL.

**Os fatos e de onde vêm** (nada abaixo é estimativa):

| Fato | Valor | Origem | Tipo |
|---|---|---|---|
| credenciais no C4 | **6**: jogo, portal/BFF, staff, operação/BI, runner de migrations, provisionamento | `docs/arquitetura/contexto-c4.md` §2 | `V` |
| credenciais de **produto** | **4** (as quatro primeiras) | idem | `V` |
| credenciais de **infraestrutura** | **2**: runner e provisionamento | idem | `V` |
| valores de `person.role` | **3**: `member`, `moderator`, `admin`; é papel **do portal, não de jogo** | `docs/dicionario-dados/gerado/person.md` (`ck_person_role`); `migrations/0003-portal-person.md` | `V` |
| papéis de cluster | **6** + `wyd_owner` `NOLOGIN` — não são atores de produto | `docs/arquitetura/seguranca.md`; `plano-de-implantacao.md` §Verificação | `V` |
| requisitos com ID | **31** (16 `RF` + 7 `RN` + 3 `UC` + 5 `RNF`) | `docs/specs/index.md` | `V` |
| atores nomeados hoje | **0** — `specs/stakeholders.md` não existe | `proposta-paginas.md` §6 | ausência |

**(b) Opções reais**

| Opção | O que a página passa a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | **5** atores, união medida de 4 credenciais de produto com 3 valores de `role`: **jogador** (dono da conta de jogo; age por `WYD_DB_URL_GAME`, sem credencial própria), **titular do portal** (`member` + `WYD_DB_URL_PORTAL`), **staff de suporte** (`moderator` + `WYD_DB_URL_ADMIN`), **administrador** (`admin` + `WYD_DB_URL_ADMIN`), **operador** (`WYD_DB_URL_READ`, sem `role`) | 5 linhas + 2 colunas (requisito que atende, quem aprova); nenhum ator inventado | **Recomendada** |
| B | **3** atores: só os valores de `person.role` | perde o jogador — dono dos `RF` de conta, personagem e sessão — e o operador, que tem credencial e não tem `role` | **Não recomendada** |
| C | **6** atores: todos os do C4, incluindo runner e provisionamento | põe na matriz “runner atende `RF-DB-001`”: quem não serve requisição e não tem requisito a atender | **Não recomendada** |

**(c) Recomendação e razão:** A. Os cinco **não são lista inventada**: são a união de dois conjuntos medidos —
4 credenciais de produto (`WYD_DB_URL_GAME`, `_PORTAL`, `_ADMIN`, `_READ`) e 3 valores de `person.role` —, e a
união dá 5 porque **staff e administrador compartilham a credencial `wyd_admin` e se distinguem pelo `role`**.
O DDL já separa os dois (`ck_person_role`), então juntá-los seria apagar distinção que existe no banco; e o
runner e o provisionamento ficam de fora porque não são atores de produto: um aplica DDL, o outro cria papel,
e nenhum dos dois atende requisito de jogador, titular, staff ou operador.

**(d) Custo de cada opção:** **A** custa 5 linhas com duas colunas e deixa uma pendência explícita — o jogador
não tem credencial própria, porque quem conecta é o servidor de jogo em nome dele. **B** custa menos e erra
duas vezes: apaga a distinção `moderator`/`admin` e deixa `WYD_DB_URL_READ` sem responsável na matriz. **C**
custa o mesmo e cria quatro linhas sem conteúdo: `RF`, `RN`, `UC` e `RNF` do banco não são atendidos por
runner nem por provisionamento, e a matriz passaria a afirmar o contrário.

**(e) O que andaria quando a resposta viesse:** `specs/stakeholders.md` escrita (1 página; a fila cairia de
**5** para **4**); a matriz ganharia a coluna de ator que `Q-BLK-08` cita; e a Onda 3 fecharia a ponta “quem
aprova mudança de requisito”, que hoje não tem dono por escrito.

**(f) Fonte fechada:** `docs/arquitetura/contexto-c4.md` §2 (as 6 credenciais e o que cada uma pode);
`docs/dicionario-dados/gerado/person.md` (`role` = `member`/`moderator`/`admin`, papel do portal);
`docs/arquitetura/seguranca.md` (6 papéis de cluster); `docs/specs/index.md` (31 requisitos com ID);
`docs/guia-administrador/index.md` §3 (operador do ambiente); `lacunas.md` §2.2 (`A20` e `M21` externas).

**O que continua sem fonte:** o **nome** do operador — `Q-BLK-04` = A decidiu que nome real fica com o dono do
produto — e o segundo fator do staff (`A20`) e as telas do portal (`M21`), que seguem externos. A página pode
nomear o ator sem publicar a pessoa.

**Evidência fraca declarada:** o titular do portal entra como ator **mesmo com `MFA` (`A20`) e telas (`M21`)
ainda externas** — o requisito existe, a tela não; e “jogador” não tem credencial própria, o que a página
precisa dizer na primeira linha da matriz, não em nota.

**Se não decidir:** `stakeholders` segue bloqueada por “matriz ator → requisito dos demais domínios”, e a
Onda 3 não tem quem aprova mudança por escrito. **Aprovar:** A / B / C.

### 6.6.11 Ordem 6 — `Q-BLK-11` — agrupamento do modelo conceitual (1 página)

**Resposta do responsável (2026-11-24): A.** Aplicada em `docs/arquitetura/modelo-conceitual.md` (§6.6.19).

**(a) O que está em jogo:** o ER conceitual cobre **109 tabelas** e precisa de um recorte que o leitor já
conheça; hoje `modelo-conceitual` não tem nenhum.

**Os números e de onde vêm** (nada abaixo é estimativa):

| Fato | Valor | Origem | Tipo |
|---|---|---|---|
| páginas de domínio do dicionário | **19** | `docs/dicionario-dados/index.md` (tabela `Domínio / Cobre / Tabelas`) | `V` |
| tabelas por domínio | de **1 a 22**; soma **109** | idem, coluna `Tabelas` | `V` |
| três maiores domínios | economia **22**, mundo **13**, acesso **9** = **44** de 109 | idem | `V` |
| domínios com **3 ou menos** tabelas | **5** de 19 (canal, moderação e ponte com 1; auditoria e inventário com 3) | idem | `V` |
| agregados que o molde nomeia | **5**: conta, personagem, clã, conteúdo, operação | `wyd-db-1.0.0.md` §6.3 (descrição literal de `modelo-conceitual.md`) | `V` |
| dono da fronteira de escrita | `arquitetura/fronteira-hot-cold.md` | idem | `V` |
| ER escrito hoje | **0** — `arquitetura/modelo-conceitual.md` não existe | `proposta-paginas.md` §6 | ausência |
| páginas em `arquitetura/` | **15** | `proposta-paginas.md` §2 | `V` |

**(b) Opções reais**

| Opção | O que a página passa a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | agrupa pelos **19 domínios** do dicionário e usa os **5 agregados** (conta, personagem, clã, conteúdo, operação) como recorte transversal, citando `fronteira-hot-cold.md` como dono da fronteira de escrita | 19 blocos + 1 recorte transversal; o mapa já existe | **Recomendada** |
| B | agrupa pelas **5 fronteiras de escrita** e deixa o domínio como detalhe | cria segundo dono para a mesma regra: a fronteira já tem dono | **Não recomendada** |
| C | ER único e plano das **109** tabelas, sem agrupamento | 109 nós num diagrama; nenhum bloco responde “quem cuida deste assunto” | **Não recomendada** |

**(c) Recomendação e razão:** A. O molde do próprio `wyd-db` já descreve a página assim — “ER conceitual por
domínio (19, as páginas de `dicionario-dados/`), sem coluna; cardinalidades; agregados (conta, personagem, clã,
conteúdo, operação)” (`wyd-db-1.0.0.md` §6.3) —, então A não é escolha nova: é escrever o que o plano já pede,
com os 19 blocos que existem e somam os 109 que o catálogo conta. Os 5 agregados entram como recorte
**transversal**, e não como segundo dono: a fronteira de escrita continua em `fronteira-hot-cold.md`, que a
página cita (`AGENTS.md` §5, item 5). O argumento numérico fecha a escolha: 5 domínios têm **3 ou menos**
tabelas e um tem **22** — num diagrama plano de 109 nós, o assunto de 22 tabelas e o de 1 tabela disputam o
mesmo espaço, e o leitor perde justamente a noção de tamanho que o agrupamento preserva.

**(d) Custo de cada opção:** **A** custa 19 blocos com cardinalidade e um recorte transversal; a página fica
longa, e é a única forma de manter o mapa que o leitor já usa. **B** custa menos e duplica regra: as duas
páginas passam a poder divergir sobre a mesma fronteira, e a de arquitetura perde o dono. **C** custa um
diagrama ilegível na prática — 109 nós sem hierarquia —, não conversa com o dicionário e não responde “quais
entidades cuidam deste assunto”, que é a pergunta da página.

**(e) O que andaria quando a resposta viesse:** `arquitetura/modelo-conceitual.md` escrita (1 página;
`arquitetura/` passa de **15** para **16** páginas com corpo e resta só `implantacao` na pasta); o conjunto que
a Onda 2 cita ficaria completo e `modelo-logico` deixaria de ter página pendente a jusante.

**(f) Fonte fechada:** `docs/dicionario-dados/index.md` (19 domínios, **109** tabelas, coluna `Tabelas` com
**1** na menor e **22** na maior); `docs/arquitetura/fronteira-hot-cold.md` (dono da fronteira de escrita);
`wyd-db-1.0.0.md` §6.3 (descrição literal: 19 domínios, sem coluna, cardinalidades, 5 agregados);
`proposta-paginas.md` §2 (15 páginas em `arquitetura/`); `AGENTS.md` §5, item 5 (um fato, um dono).

**O que continua sem fonte:** nenhum arquivo mede legibilidade. O que existe é **uso** — 19 páginas de domínio
já escritas e a soma de 109 tabelas que fecha com o catálogo — e a descrição literal do molde; a escolha do
recorte é de leitura, e a página precisa dizer isso em vez de apresentá-la como medida.

**Evidência fraca declarada:** com **5** domínios de 3 ou menos tabelas e um de **22**, qualquer agrupamento
vai produzir blocos de tamanho desigual; o desequilíbrio é consequência do DDL, não do recorte, e a página não
pode prometer simetria.

**Se não decidir:** `modelo-conceitual` continua bloqueada por “agrupamento conceitual”. **Aprovar:**
A / B / C.

### 6.6.12 Bloqueios que **não** são decisão — trabalho técnico (1 página)

Esta não vira pergunta: nenhuma resposta do dono cria o dado que falta.

| Página | Por que não é pergunta | O que falta | Dono |
|---|---|---|---|
| `planos/metricas.md` | o número **medido** só existe depois da série aplicada (`M20`); decidir antes seria inventar medição | série executada + aceite `M33` + 1 sessão de medição | arquitetura + operação |

`arquitetura/fluxo-de-dados.md` estava nesta tabela como redação a partir do DDL fechado e **foi escrita em
2026-11-24**: a tabela origem → caminho → destino cabia sem decisão e sem medição, e a página declara
como `H` o que o desenho não fecha (atomicidade de auditoria, leitura cruzada sem `GRANT`).

**Fora das 19, mas presos ao mesmo tipo de trabalho** (não entram na conta de páginas bloqueadas): os **6
seeds** (cliente 7662, §4.2), as **cinco aplicações de retenção** (índice/`DELETE`/job, §4.3),
`migrations/serie-e-regras.md` (escrita, §4.3), a **suíte** (código das ondas 4–6) e `Q-DP-2/validação`
(parecer externo). `A11`, `A20` e `M21` seguem `externa` (§4.4).

### 6.6.13 Próximo lote recomendado — 4 páginas

**Lote C — operação e documentação (4 páginas), depois de `Q-BLK-01`, `Q-BLK-02` e `Q-BLK-03`.**

| Ordem | Página | Por que nesta ordem |
|---:|---|---|
| 1 | `guia-documentacao/imagens-e-evidencias.md` | fecha o grupo das 4 com **1** página; é a única sem seção equivalente em `convencoes.md`; depende só de `Q-BLK-01` |
| 2 | `planos/plano-de-manutencao.md` | é o dono da rotina: sem ele os quatro expurgos não têm calendário nem registro, e as duas páginas seguintes o citam |
| 3 | `guia-administrador/backup-e-restore.md` | prova a política de `Q-RET-7` com o destino e o RPO/RTO de `Q-BLK-03`; o ensaio trimestral entra no calendário do item 2 |
| 4 | `guia-administrador/recuperacao-de-desastre.md` | mesmo insumo do item 3 e cita o procedimento de restore em vez de repeti-lo — mesma regra de “um fato, um dono” |

**Resultado (executado em 2026-11-24):** **4 das 19** caíram; com `Q-BLK-01` aprovada, o teto foi a **144**
e a fila a **15**; o lote C fechou as quatro páginas e a fila ficou em **12**; `fluxo-de-dados.md`, escrito
na mesma data, deixou a fila em **11** — medição em §6.6.14 —, e o lote D (`Q-BLK-04` e `Q-BLK-05`) a
deixou em **7** — medição em §6.6.17. As cinco decisões envolvidas não dependiam de cliente, fornecedor
novo, parecer ou código.

**Depois dos lotes C e D (ordem de §6.6.6–§6.6.11):** o lote seguinte foi `Q-BLK-06` + `Q-BLK-07` — o
número de pool e o primeiro `RNF-DB` —, **executado em §6.6.18**; agora `Q-BLK-08`, que depende de `07`;
em seguida `Q-BLK-10`, `Q-BLK-09` e `Q-BLK-11`, uma página cada, que podem andar em paralelo. `metricas.md`
só entra com a série aplicada (`M20`).

### 6.6.14 Execução do lote C e de `fluxo-de-dados` (2026-11-24)

As **cinco** páginas existem no worktree como `proposto` e **não** foram declaradas `aceitas`. A medição de
fechamento **naquele ponto** — superada pelo lote D em §6.6.17 — é:

| Medida | Resultado | Fonte |
|---|---:|---|
| páginas autorais com corpo em `docs/` | **133** (129 com `**Estado:**` + 4 portas sem estado) — no lote C | `find`/`grep` no worktree, 2026-11-24 |
| teto autoral | **144** (147 − 3 retiradas por `Q-BLK-01`) | `proposta-paginas.md` §2/§6 |
| páginas bloqueadas | **11** (19 − 8: 7 do lote C e `fluxo-de-dados`) | §5 |
| pasta `arquitetura/` | **15** páginas; restam `implantacao` e `modelo-conceitual` | `proposta-paginas.md` §4.1 |
| pasta `guia-documentacao/` | **5** páginas (teto da pasta: 8 → 5) | `proposta-paginas.md` §4.6 |
| pasta `planos/` | **9** páginas; resta `metricas.md` | `proposta-paginas.md` §4.4 |
| pasta `guia-administrador/` | **6** páginas; restavam `manutencao-rotineira` e `sustentacao` (escritas no lote D) | `proposta-paginas.md` §4.6 |
| lacunas do `wyd-db` | **38 = 0 abertas / 0 propostas / 35 fechadas / 3 externas** | `lacunas.md` §1 |
| gate de documentação | **298 páginas, 296 fontes verificadas** | `docker compose --profile gate run --rm -T gate`, exit 0 |
| `git diff --check` | **exit 0** | 2026-11-24 |
| `RF-DB` novos | **0** | nenhum requisito foi numerado |

**O que as cinco páginas declaram como pendente, sem inventar valor:** a credencial administrativa de
backup, vacuum e expurgo não tem nome fixado na árvore (`guia-administrador/index.md` §3); o índice por
`read_at` de `market_notification` e o `DELETE`/job das quatro políticas continuam em §4.3, agora com
executor decidido; `game.pin_reset` não tem índice por `expires_at`, então o expurgo varre a tabela até
`M20`; e a regra de migration aplicada em produção continua sem dono publicado, porque
`migrations/serie-e-regras.md` não existe. Nenhuma tabela, coluna, índice, job, endpoint ou regra foi
criada nesta execução.

### 6.6.15 O que este pacote não faz

- **Não responde em nome do dono.** Nenhuma pergunta tem “resposta”; o que existe é recomendação.
- **Não cria lacuna nem ID de lacuna.** `Q-BLK-nn` é rótulo deste pacote; `lacunas.md` continua com **38**
  (0/0/35/3) e o registro não ganha linha lá.
- **Não muda contador antes da decisão.** O estado vigente, depois das sete respostas e dos lotes C, D e E,
  é **144 = 139 + 5** (§6.6.18); `147` e `19` são o estado anterior a 2026-11-24.
- **Não escreve página bloqueada**, não aceita página, não cria índice, job, migration ou código.
- **Não reabre lacuna fechada** e não toca em `A11`, `A20` e `M21`, que continuam `externa`.
- **Não decide A1.** A captura continua autorizada e fora do foco, como em `Q-FOCO-1 = D`.

### 6.6.16 Revisão de 2026-11-24 — fonte conferida e o que é fraco
**Sem arquivo no repositório:** `recuperacao-de-disastre.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Duas passadas, ambas lendo o arquivo e não a memória do pacote: §6.6.4–§6.6.11 na primeira tabela e
§6.6.1–§6.6.3 na segunda. O que confere e o que mudou:

| Citação | O que o arquivo diz | Resultado |
|---|---|---|
| `roadmap/WYDDB-021.md` | “usa o timeout já configurado, sem inventar número” | confere |
| `roadmap/WYDDB-026.md` | “Não definir threshold de pool, disco ou latência; não criar dashboard.” | confere |
| `roadmap/WYDDB-005.md` | “Não escolher driver, pool ou política de conexão” | confere |
| `guia-desenvolvedor/portas-e-contratos.md` §2 | “Um único adapter implementa as duas portas com o mesmo pool.” | confere |
| `guia-administrador/configuracao.md` | “Pool, timeout de query e tamanho de conexão: não há desenho documentado” | confere |
| `arquitetura/retencao-e-capacidade.md` | “Existe limiar de latência ou capacidade? **não**” | confere |
| `specs/index.md` | 16 `RF-DB`, 7 `RN-DB`, 3 `UC-DB` | confere |
| `dicionario-dados/gerado/person.md` | `role` ∈ `member`/`moderator`/`admin` | confere |
| `../wyd-bots/docs/proposta-decisoes-bots.md` (`Q-BOT-2`) | “o plano exige ‘fixture, não conta real’” | confere |
| `arquitetura/contexto-c4.md` | **seis** credenciais (jogo, portal, staff, operação, migrator, provision) | **corrigido**: eu escrevia “quatro pontas” |
| `planos/plano-de-manutencao.md` §2, `guia-administrador/recuperacao-de-desastre.md` §4, `guia-administrador/index.md` §3 | “operador” sem nome | **corrigido**: eu escrevia “três páginas”, e `backup-e-restore` não usa a palavra |
| `planos/plano-de-implantacao.md` | citava **92** tabelas | **corrigido** para **109** (dono: `plano-migrations.md` §3) |
| `src/migrate/adapter/pg.md` | dois pools, ambos `max_connections = 1` (série serial e lock) | confere |
| `src/provision/adapter/pg.md` | `PgPoolOptions::new().max_connections(1)` | confere |
| `guia-administrador/configuracao.md` | `lock_timeout` **10 s**, faixa 1 s–10 min | confere |
| `grep` por `max_connections` no sistema | só os pools do runner/provision; nenhum runtime fixado | confere (ausência) |
| `docker-compose*.yml` na raiz | serviços `docs` e perfil `gate`; **nenhum serviço de banco**, e nenhum MySQL | confere (corrige a premissa do pedido) |
| defaults do `sqlx` (`PoolOptions::new`) | 10 conexões, mínimo 0, aquisição 30 s, idle 10 min, vida 30 min | confere (fonte externa, versão atual) |
| default do PostgreSQL 17 | `max_connections` 100, `superuser_reserved_connections` 3 → **97** | confere (fonte externa) |

**Segunda passada — §6.6.1–§6.6.3.** As três seções ficaram fora da conferência acima; a leitura abaixo é
arquivo por arquivo.

| Citação | O que o arquivo diz | Resultado |
|---|---|---|
| `convencoes.md` §Links e citações, linha 114 | tabela de situações (link relativo, `arquivo:linha`, caminho em `code`, âncora) e “link relativo para fora de `docs/` reprova” | confere |
| `convencoes.md` §Diagramas, linha 266 | tabela pergunta → tipo, “Um tipo por diagrama”, “Rótulos em português” e fence que não renderiza é defeito bloqueante | confere (a palavra “legenda” é da abertura; o corpo diz “rótulos”) |
| `convencoes.md` §Recursos do site, linha 295 | tabela de recursos, cor por tipo de aviso e lista de proibidos | confere |
| `convencoes.md` §Recursos do site, linha 297 | cita `preparar.py` **sem caminho** | **corrigido**: eu escrevia que o caminho `wyd-docs/preparar.py` era “citado” ali |
| `wyd-db-1.0.0.md` §6.14 | a tabela **não tem mais** linha para `markdown-e-mkdocs`, `diagramas` e `links-referencias-e-ids`; a nota ao fim de §6.14 registra a retirada | **corrigido**: eu afirmava no presente que as seções obrigatórias “são” essas regras |
| `AGENTS.md` §9 | “sem `mkdocs.yml`, plugin ou modo estrito … Não crie `mkdocs.yml`” | confere |
| `lacunas.md` `D-B`, linha 435 | “Não recriar config de site no diretório de planejamento”; `mkdocs.yml` removido em 2026-11-17 | confere — é a fonte que faltava à frase “config que o projeto decidiu não ter” |
| `convencoes.md` por “imagem”, “alternativ” e “peso” | **zero** ocorrências | confere (ausência): a página de imagem é a única das quatro sem equivalente |
| `pendencias-de-fechamento.md` §4.3, linhas `Q-RET-*`/`aplicação` | quatro linhas de execução; **nenhuma traz o prazo** | **corrigido**: os 30/90 dias são de §3.1 e de `proposta-retencao-pendente.md` §9 |
| `ddl-confirmado.md`, linha 4645 | `CREATE TABLE portal.market_notification` | **corrigido**: o dossiê a nomeava sem schema |
| `proposta-decisoes-ops.md` §4 e §6 | `Q-OPS-2` = A, “VPS paga com processo persistente e deploy Compose”, **sem provedor nomeado** e sem processo agendado | **corrigido**: “provedor da VPS” e “host já é dono de agendamento” eram leitura minha, agora declarada |
| `wyd-ops/docs/` por `cron`, `agendad` e `calendário` | **zero** ocorrências | confere (ausência): o calendário das cinco rotinas não tem dono |
| `proposta-retencao-pendente.md` §9 | as sete respostas; `Q-RET-7` = “restaurar uma vez por trimestre” | **corrigido**: `Q-RET-7` fixou a cadência, não uma janela de 4 h |
| `proposta-retencao-pendente.md` §9, linha 333 | citava `recuperacao-de-disastre.md` | **corrigido no arquivo**: o nome é `recuperacao-de-desastre.md` |
| `pendencias-de-fechamento.md` §3.3.1 e §3.3.1.2 | resumo de `Q-RET-7` e a restrição “experimental, sem gasto” | confere |
| `plano-de-documentacao.md` §3, `wyd-db-1.0.0.md` §4, `../wyd-1.0.0.md` §1.2 | **144** páginas | confere |
| `docs/guia-administrador/manutencao-rotineira.md` | existe, e o §2 aplica `Q-BLK-02` | **corrigido**: o parágrafo “Aplicado” de §6.6.2 dizia que a página não foi escrita |
| `backup-e-restore.md` §1–§3 e `recuperacao-de-desastre.md` §1 | destino “mesmo provedor da VPS”, RPO 24 h e RTO 4 h | confere |
| §6.6.4, item (e) | “A credencial **nominal** continua de fora: é de `configuracao.md` e de `guia-administrador/index.md` §3” | **corrigido**: §6.6.3 dizia que a credencial continuava “de `Q-BLK-04`” |

**`AGENTS.md` §5 — forma canônica fixada em 2026-11-24.** Primeiro a conferência, depois a convergência.
O §5 é **lista numerada**, não subseções: o item 5 é “Um fato, um dono” e o item 7 é “Segredo nunca
aparece”, e é isso que as três seções citam — o destino estava certo, a forma não. A regra de
`convencoes.md` §Coluna de dono (linha 189) prescreve “arquivo + `§` + item”, então a forma canônica é
`AGENTS.md` §5, item 5 e, com mais de um, `AGENTS.md` §5, itens 5 e 7. Até esta data o projeto escrevia o
item como decimal do parágrafo — o `§5` seguido de ponto e do número —, forma que a regra não prevê e que
foi aplicada inclusive neste registro.

A varredura de 2026-11-24 achou **108** referências nessa forma, em **44** arquivos: **92** com o arquivo
entre crases, **2** sem crase, **4** escritas “invariante N”, **4** numa lista com o sinal duplicado e os
itens em decimal, **5** itens que continuavam uma lista citada e **1** dentro do próprio `AGENTS.md` §4.
**Todas** convergiram — a varredura de fechamento tem **0**, e a forma canônica aparece em **100**
citações, porque as listas de itens foram fundidas numa citação só. “Invariante N” virou “item N” porque
é a palavra que a regra manda; se o dono preferir “invariante”, muda-se a regra uma vez, não a forma em 44
arquivos. Fora da conta ficam só árvores que não são documentação viva — `deprecated/` e
`wyd-docs/.build/` —, contadas no bloco do §2 abaixo.

**Reconciliação de 2026-11-24 (segunda passada, árvore maior).** O parágrafo acima é **log do dia** e fica
intacto: 108 na forma decimal, 44 arquivos e 100 citações canônicas foram o estado daquela varredura. O que
mudou foi a árvore — os lotes C, D e E e os dossiês de §6.6 escreveram páginas que citam a lista —, então a
fotografia de hoje é: **109** citações ao §5 de `AGENTS.md`, em **45** arquivos, **101** na forma canônica
(**96** com `item` e **5** com `itens`) e **8** citando a seção inteira — seis linhas `Base:` que citam a
lista toda (`§5`, `§5/§6` ou `§5 e §7`), uma que usa a lista como argumento
(`wyd-protocol-1.0.0.md` §117) e este registro. As oito ficam como estão: nenhuma afirma invariante
específico, e itemizar um `Base: §5/§6` criaria ruído. A varredura de fechamento continua **0** na forma
decimal, que é o que a convergência prometeu. A contagem anterior errava por 5 em cada lado porque somava
só `item` e deixava `itens` de fora — 96 + 13 virou **101 + 8**, e a soma segue **109**. O bloco do §2 logo
abaixo mede escopo diferente — soma as três citações daquele §2 e dá **111** em **47** arquivos — e também
é log do dia; hoje as três estão em **0** na árvore viva, porque foram resolvidas, não reformatadas.

**Origem dos “59” — não localizada.** A busca está registrada, e nenhum recorte dá 59: árvore viva em
`.md` (**0** na forma decimal), arquivos não-`.md` rastreados por `git grep -I` (**0**), `deprecated/`
(**0**), artefatos de `wyd-docs/.build/` (**0**); as quatro pontas de branch — `develop` **77** em 31
arquivos, `origin/develop` **8** em 6, `origin/docs/fechar-board-github` **8** em 6 e `origin/main`
**0** — e os **58** commits de `git rev-list --all`, cujo máximo é **77** e nenhum dá 59. Recortes
alternativos também não fecham: 33 citações no `wyd-db` hoje, 45 arquivos, 109 citações e as **49**
menções ao bloco 5.5 mais **26** ao bloco 5.7 de `ddl-confirmado.md` dentro do `wyd-db` (**75**). Esses
dois são subseção real daquele arquivo — citação de subseção, não da lista numerada —, e por isso não se
convertem. O número **não existe** na árvore nem no histórico, e **nenhum** dos recortes medidos soma 59 —
os blocos de `ddl-confirmado.md` também não explicam, porque somam **75**. A origem fica sem explicação
localizada.

**O resíduo de §2 — fechado em 2026-11-24.** A varredura com o §2 incluído achou **mais 3** referências
do mesmo tipo, todas no `wyd-db`: `rascunho-accounts.md` §R13 e duas páginas publicadas
(`arquitetura/seguranca.md` e `guia-administrador/configuracao.md`) que atribuíam `SESSION_SECRET` ao
`AGENTS.md`. Total do escopo: **111** referências em **47** arquivos, **0** restantes. O §2 tem **4**
itens, todos de ordem de leitura (panorama, plano do sistema, pilotos do `wyd-db`, `deprecated` como
referência): **nenhum** trata de hash, senha, PIN ou `SESSION_SECRET`, então não havia item para onde
apontar — as três citações foram resolvidas, não reformatadas. §R13: a alegação “senha e PIN só como hash
resistente (argon2id)” não vem do `AGENTS.md`; é medida em `repo/accounts.rs` (coluna “Fonte medida”, que
passa a citar só ele) e fixada por `M9` e pelos `CHECK` de prefixo de `ddl-confirmado.md` §5.1–§5.2, que
a própria linha já citava em “Onde o desenho atende hoje” — requisito, número e decisão intactos. As duas
páginas perderam a atribuição falsa e mantiveram a fronteira (segredo do portal, fora do contrato do
`wyd-db`); o precedente do `SESSION_SECRET` está em `ddl-confirmado.md` §5.20.

Varredura de fechamento, sobre o conteúdo **versionado** — é o que o git chama de repositório: **422**
arquivos `.md` entre versionados e não ignorados:
`git ls-files --cached --others --exclude-standard '*.md' | xargs grep -lE 'AGENTS\.md.{0,2}§[0-9]+\.[0-9]'`
→ **0 arquivos**. Fora dela ficam só árvores **ignoradas pelo git**: `deprecated/` (`.gitignore:4`, zero
arquivos versionados, **15** referências a um `AGENTS.md` de árvore que não existe mais — reescrevê-las
afirmaria item inexistente) e `wyd-docs/.build/` (a montagem `.build/docs` foi regenerada por
`preparar.py` nesta data e tem **0**; `.build/validacao/` guarda **14** snapshots de execuções
anteriores). Nada fica aberto para a próxima passada.

**Reconciliação de fechamento (2026-11-24, árvore de hoje).** Esta entrada é **log datado** — os números
acima ficam como o dia os mediu —, e esta linha é a fotografia de fechamento. A varredura de hoje, sobre os
**443** arquivos `.md` versionados ou não ignorados (`git ls-files --cached --others --exclude-standard '*.md'
| wc -l`), acha **111** ocorrências do §5 do `AGENTS.md` em **45** arquivos e **0** na forma decimal
(`git ls-files --cached --others --exclude-standard -z | xargs -0 grep -nI -E 'AGENTS\.md.{0,2}§5'`).
Das 111, **duas** são este bloco fixando a própria forma — a abertura e o exemplo `§5, item 5` — e não
contam como citação; a terceira menção da mesma linha (`§5, itens 5 e 7`) entra na conta, porque é a lista
de itens que ela exemplifica. Sobram **109**, em **45** arquivos — **96** com `item` singular, **5** com `itens` (lista de
itens) e **8** com a seção inteira (`§5/§6`, `§5 e §7`, `§5, §8 e §9`, `Base: §5`). 96 + 5 + 8 = **109**: a
soma é a mesma da segunda passada; o que muda é o desdobramento — as 5 de lista estavam somadas às
canônicas (**101**) e agora aparecem à parte, de modo que a leitura “**96** canônicas + **13** não
singulares” (5 listas + 8 seções inteiras) descreve a mesma árvore. As 8 de seção inteira continuam
intencionais e não são itemizadas; as 5 de lista já nomeiam os itens, e as `§5.N` de `ddl-confirmado.md`
seguem fora da conta porque citam subseção daquele arquivo, não a lista numerada.

O que a varredura anterior não cobria, medido hoje: `deprecated/` tem **1.175** arquivos `.md` fora de
`target/` e **1** citação ao §5 do `AGENTS.md` —
`deprecated/.agents/prompts/análises/wyd-db/rust/12-proposta-de-evolucao.md:241`, seção inteira —, com
**74** arquivos mencionando `AGENTS.md` sem citar o §5 (a linha anterior dizia “15 referências”: era outra
medida, de outra árvore); `wyd-docs/.build/` tem **68** ocorrências em **38** arquivos — **56** em
`.build/docs/` (28 arquivos, todos em `sistemas/`) e **12** em dez snapshots de `.build/validacao/` —,
todas espelho de página, nenhuma fonte (a linha anterior dizia **0**: a montagem foi regenerada desde então); arquivos
**não-`.md`** versionados: **0** (`git grep` com `:(exclude)*.md`); e o histórico tem **171** commits, com
as quatro pontas em `develop` **87** ocorrências em 37 arquivos, `origin/develop` **17** em 13,
`origin/docs/fechar-board-github` **17** em 13 e `origin/main` **0** — os números de apoio da linha
“Origem dos 59” (58 commits, máximo 77, 15 em `deprecated`, 0 em `.build`) eram da árvore daquele momento e
ficam corrigidos por esta.

**Origem dos “59” — segue não localizada, agora com a varredura ampliada.** Nenhum recorte novo dá 59:
`deprecated/` dá **1**, `.build/` dá **68** (cópias), não-`.md` dá **0** e as quatro pontas dão 87, 17, 17 e
0. No histórico, a busca por conteúdo (`git log --all -S'59 citações'` e
`-G '59[^0-9]{0,30}(cita|refer)'`) não achou commit com a expressão, e o próprio número entra neste
registro na árvore de trabalho, sem commit que o introduza (`git log -S'59' --
planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` → vazio). O único “59” de contagem do histórico é
`docs/analysis/banco-de-dados.md:4` — “59 migrations aplicadas, 15.059 linhas de referência”, em `f39aed3`
—, documento arquivado que **não é citação** ao §5 e **não é reescrito**. Não refiz o censo commit a
commit; o que está provado é que a expressão não existe no histórico nem em nenhuma das árvores medidas.

**Proposta, não medição** — nenhuma destas linhas tem fonte para o valor, e todas estão marcadas como `P`
na página que as usar. **`Q-BLK-06` e `Q-BLK-07` não estão nesta tabela desde 2026-11-24**: os números
propostos para elas passaram a ser defaults reais do `sqlx` e do PostgreSQL, ou foram retirados (§6.6.6 e
§6.6.7) — o que continua sem fonte nelas está declarado dentro das próprias seções.

| Item | O que é proposta |
|---|---|
| `Q-BLK-05` | as 5 contas são cobertura proposta; não há suíte nem catálogo medido |
| `Q-BLK-04` | não há organograma, escala nem segundo nome — o inventário será por função |
| `Q-BLK-08` | 100% da coluna de teste será `P`: não há teste executado |
| `Q-BLK-09` | o titular consta como ator com MFA (`A20`) e telas (`M21`) ainda externas |
| `Q-BLK-10` | “1 processo por canal” vem da fronteira do `wyd-server`; “5432 só interna” é inferência |
| `Q-BLK-11` | o recorte por domínio é escolha de leitura; nenhum arquivo mede legibilidade |

**Consequência que eu havia esticado e recuei:** `Q-BLK-05` dizia que conta real “escreveria em
`game.market_escrow` e `ops.delivery_queue`”. `market_escrow` só é movida por `wyd_admin`
(`gerado/market_escrow.md`) e a fila é enfileirada pelo portal — uma conta de teste não passa por nenhuma
das duas por si. A frase agora fala só de dado de cliente (§4.2) e de estado que o reset precisa apagar.

**O que segue sem base e não virou afirmação:** conexões simultâneas, memória, latência e volume não foram
medidos; nenhuma das oito perguntas depende disso para ser respondida.

### 6.6.17 Execução do lote D (2026-11-24)

`Q-BLK-04` e `Q-BLK-05` foram aprovadas em A e as quatro páginas existem no worktree como `proposto`:
[`sustentacao.md`](docs/guia-administrador/sustentacao.md),
[`manutencao-rotineira.md`](docs/guia-administrador/manutencao-rotineira.md),
[`fixtures-e-factories.md`](docs/guia-teste/fixtures-e-factories.md) e
[`dados-de-teste.md`](docs/guia-teste/dados-de-teste.md). A medição de fechamento é:

| Medida | Resultado | Fonte |
|---|---:|---|
| páginas autorais com corpo em `docs/` | **137** (133 com `**Estado:**` + 4 portas) | `find`/`grep` no worktree, 2026-11-24 |
| páginas bloqueadas | **7** (11 − 4) | §5 |
| `guia-administrador/` | **8** páginas — pasta no teto (era 6) | `proposta-paginas.md` §4.6 |
| `guia-teste/` | **7** páginas — pasta no teto (era 5) | `proposta-paginas.md` §4 |
| arquivos `.md` / com `> Pergunta:` | **328** / **320** | `find` e `grep`, 2026-11-24 |
| gate de documentação | **302 páginas, 300 fontes verificadas** | `docker compose --profile gate run --rm -T gate`, exit 0 |
| `git diff --check` | **exit 0** | 2026-11-24 |
| lacunas e `RF-DB` novos | **38 = 0/0/35/3**; **0** | `lacunas.md` §1 |

**O que as quatro páginas declaram como pendente, sem inventar valor:** a credencial administrativa continua
sem nome (`configuracao.md` e `guia-administrador/index.md` §3); não há política de rotação de log no sistema
(o dono é `wyd-ops`); nenhum parâmetro de autovacuum foi fixado; o número de pool é `Q-BLK-06`; os 6 seeds
seguem presos ao cliente 7662; a suíte não existe, então as factories e o catálogo de contas são forma
planejada, não código; e os nomes das contas ficam no `seeds/dev.sql` quando ele existir.

### 6.6.18 Execução do lote E (2026-11-24)

As **duas** páginas de `Q-BLK-06` e `Q-BLK-07` estão no worktree como `proposto` — nenhuma foi declarada
`aceita` —, ambas com resposta **A**.

| Página | Seção | O que ela fixa | Fila |
|---|---|---|---:|
| `docs/guia-desenvolvedor/pool-e-transacoes.md` | §6.6.6 | 10 conexões por pool (mínimo 0), acquire 30 s, idle 10 min, vida 30 min, `lock_timeout` 10 s, `statement_timeout` **0 declarado** e o orçamento de **53 de 97** conexões, 44 livres | 7 → 6 |
| `docs/specs/requisitos-nao-funcionais.md` | §6.6.7 | `RNF-DB-001` cobertura ≥ 85 %, `RNF-DB-002` espera de DDL ≤ 10 s, `RNF-DB-003` `SKIP` = 0, `RNF-DB-004` p95 e `RNF-DB-005` boot **sem limiar até `M20`** | 6 → 5 |

**O que a opção A recusou:** inventar. `statement_timeout` continua **0** e está escrito como *estado*, não
como alvo; p95 e tempo de boot ganharam métrica, “como medir” e a regra “medir sem limiar aprovado
reprova”. Nenhum dos cinco `RNF-DB` tem número escolhido por estimativa: os três com limiar já valem no
projeto, e os dois restantes esperam `M20`.

**Fora das duas páginas:**

- `docs/guia-administrador/configuracao.md` ganhou a seção **§3 “Pool e timeouts da conexão”** (seis
  parâmetros com dono do valor e a conta do teto); as antigas §3–§5 subiram para §4–§6, e o item que
  declarava “Pool, timeout de query e tamanho de conexão: não há desenho documentado” virou ponteiro para a
  página nova.
- Índices: `docs/guia-desenvolvedor/index.md` (trilha de 9 passos e mapa dos guias) e `docs/specs/index.md`
  (tabela das 8 irmãs, série `RNF-DB` e estado por área).
- Contadores: `proposta-paginas.md` §1/§2/§4/§6, §1 e §5 deste arquivo, quadro-resumo de §6.6,
  `wyd-db-1.0.0.md` §4, `../wyd-1.0.0.md` §1.2 e `D6`, `docs/planos/plano-de-documentacao.md` e
  `docs/index.md` — todos passaram de **137/7** para **139/5**.

**Medido depois das edições, em 2026-11-24:**

| Medida | Resultado | Comando |
|---|---:|---|
| páginas com corpo em `docs/` | **139** | `find docs -name '*.md' -not -path 'docs/dicionario-dados/gerado/*' \| grep -v 'docs/proposta-' \| wc -l` |
| com `**Estado:**` | **135** + 4 portas = 139 | `grep -rl '^\*\*Estado:\*\*' docs \| grep -v gerado \| grep -c -v 'proposta-'` |
| `guia-desenvolvedor/` e `specs/` | **11** e **18** | `ls docs/guia-desenvolvedor/*.md`; `find docs/specs -name '*.md'` |
| páginas bloqueadas | **5**: `arquitetura/implantacao.md`, `arquitetura/modelo-conceitual.md`, `specs/rastreabilidade.md`, `specs/stakeholders.md` e `planos/metricas.md` | `proposta-paginas.md` §6; 139 + 5 = 144 |
| gate de documentação | **304 páginas, 302 fontes verificadas**, exit 0 | `docker compose --profile gate run --rm -T gate` |
| `git diff --check` | **exit 0** | 2026-11-24 |

O gate subiu de **302/300** para **304/302** páginas e fontes: cada página nova entra com o próprio
cabeçalho de fonte. Nada mais na árvore mudou de contagem.

**O que o lote E não fez:** não mediu latência, boot ou concorrência; não fixou `statement_timeout`; não
respondeu `Q-BLK-08` a `Q-BLK-11`; não aceitou página; não tocou em `A11`, `A20` ou `M21`.

### 6.6.19 Execução do lote F — as quatro páginas de §6.7 (2026-11-24)

O responsável aprovou `Q-BLK-08`, `Q-BLK-09`, `Q-BLK-10` e `Q-BLK-11` em **A**, na ordem de §6.7.4, e as
quatro páginas foram escritas na mesma data. Nenhuma pergunta ficou aberta: o quadro de §6.6 passa a ter
**11 de 11** respondidas, e a fila de §5 fica em **1** página — `planos/metricas.md`, que não é pergunta.

| Ordem | Pergunta | Resposta | Página escrita | Medição que a página usa |
|---:|---|---|---|---|
| 1 | `Q-BLK-08` | **A** | `docs/specs/rastreabilidade.md` | **31** requisitos; **15** de **109** tabelas nomeadas |
| 2 | `Q-BLK-09` | **A** | `docs/specs/stakeholders.md` | **5** atores; **4** credenciais; **3** valores de `role` |
| 3 | `Q-BLK-10` | **A** | `docs/arquitetura/implantacao.md` | **1** host, **1** container, **1** volume; **53** de **97** conexões |
| 4 | `Q-BLK-11` | **A** | `docs/arquitetura/modelo-conceitual.md` | **19** domínios; **109** tabelas; **5** agregados (32+17+4+51+5) |

**O que cada página fixa:**

- **rastreabilidade** — matriz de **5** colunas com as **31** linhas e `teste` como alvo `P`; a coluna
  `issue` fica "a alocar na onda 8", e o vazio aparece contado: as **31** linhas sem teste e **94** das 109
  tabelas sem `RF`/`RN` que as nomeie, por domínio.
- **stakeholders** — as **5** linhas (jogador, titular do portal, staff, administrador, operador), com
  credencial, `person.role`, interesse e requisito que o atende; a regra de aprovação de §3 aponta a
  página dona do requisito como quem decide e nomeia quem é citado na mesma entrega.
- **implantação** — C4 nível 4 com **1** host, PostgreSQL 17 em **1** container, `pgdata` como **1**
  volume, **5432 não publicada**, runner e `provision` one-shot e `restart: unless-stopped` como
  supervisor; a ordem de instalação de `plano-de-implantacao.md` é referida, não repetida.
- **modelo conceitual** — **19** blocos de domínio (de **1** a **22** tabelas) nos **5** agregados, mais
  as **11** cardinalidades que o DDL e `modelo-logico.md` §1/§3 já fixavam; a fronteira quente/fria fica
  em `fronteira-hot-cold.md`.

**Medido depois das quatro páginas, em 2026-11-24:**

| Medida | Antes | Depois |
|---|---:|---:|
| páginas autorais com corpo em `docs/` | 139 | **143** |
| com `**Estado:**` | 135 + 4 portas | **139 + 4 portas** |
| páginas bloqueadas | 5 | **1** (`planos/metricas.md`) |
| `specs/` | 18 | **20** |
| `arquitetura/` | 15 | **17** |
| gate de documentação | 304 páginas, 302 fontes | **308 páginas, 306 fontes** |

**Duas correções de número, feitas no mesmo lote:** (1) o quadro de §6.6.8 dizia "**7 de 109** tabelas
nomeadas em `RF`"; a recontagem de 2026-11-24, por `grep` das menções `schema.tabela` em
`requisitos-funcionais.md` e `regras-de-negocio.md`, dá **15** — e a página usa 15, com o comando ao lado da
contagem; (2) `modelo-logico.md` §4 dizia que `modelo-conceitual.md` "continua bloqueado" e que repetir o
ER lá criaria segunda fonte — a página agora é o **dono** da cardinalidade, e o §4 aponta para ela.

**O que o lote F não fez:** não escreveu teste (as 31 linhas seguem `P`), não criou issue, não fixou tag de
imagem nem recurso do host, não mediu carga e não mexeu no censo de citações de §6.6.16.

### 6.6.20 Lote G — as frentes depois da fila vazia (2026-11-24)

> **Números deste lote são de antes do lote H e do lote I.** As medições abaixo (inclusive os **205** atos
de aceite, sem as 12 páginas de `tests/`) registram o estado de 2026-11-24 **antes** da escrita da suíte;
a medição vigente é a de §6.6.22.1 e o valor corrente do aceite é **141**, depois do aceite das 27 páginas
de `migrations/` (§6.6.31). A contagem de células da matriz
foi atualizada para a medição vigente — **77** = **37** `Teste` + **40** `Issue` (`docs/specs/rastreabilidade.md`
§1, depois de `Q-BLK-19` = A) —, porque era o único número desta tabela que o dono pediu para recontar; o
resto fica como registro da decisão, não como contador.

Com `Q-BLK-01` a `Q-BLK-11` respondidas e a fila de páginas bloqueadas em **1**, a matriz do lote F
mostrou onde está o gargalo de verdade. Este lote **não fecha página nenhuma**: ele escolhe onde a próxima
sessão entra, medindo cada frente pelos quatro critérios da missão — itens bloqueantes, lacunas, regras e
padrões fixados, planejamentos liberados — **por unidade de esforço**. A consolidação das seis frentes,
com a curadoria do 7662 incluída, está em `foco-e-sequencia.md`, na raiz do sistema.

#### 6.6.20.1 O que ficou à vista (medido em 2026-11-24, antes do lote H)

| Fato | Número | Fonte |
|---|---|---|
| páginas autorais de `docs/` | **143**: 137 `proposto`, **2** `aceito`, 4 portas | §6.6.19; `grep` de `**Estado:**` |
| páginas de `src/` | **37 de 59** escritas, **0** `aceito` | `find src -name '*.md'` |
| páginas de migration | **28 de 34** escritas, **1** `aceito` (`0002`) | `migrations-checklist.md` §4 |
| páginas de `tests/` | **0 de 12** (naquele momento; hoje **12 de 12**, §6.6.23) | `wyd-db-1.0.0.md` §6.17 |
| páginas de `prompts/` e `skills/` | **0 de 18** (9 + 9) | §6.18 e §6.19 |
| issues do roadmap | **40** com DoD, **0** criadas; board com 40 itens | `riscos.md` `R-5`; `../wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3 |
| linhas da matriz | **31** naquele momento (hoje **34**, §6.6.22.1); **0** com teste, **0** com issue; **15 de 109** tabelas nomeadas | `docs/specs/rastreabilidade.md` §2–§3 |
| critérios de saída do teste | **6**; o item 6 exige a matriz `CAP-nn`, que **não existia** então | `docs/planos/plano-de-testes.md` §5 |
| lacunas abertas no `wyd-db` | **0** (35 fechadas, 3 externas) | `lacunas.md` §1 |
| aceite pendente | **205 atos** naquele momento: 141 em `docs/` + 37 em `src/` + 27 em `migrations/` (**168** depois de §6.6.27; hoje **141**, §6.6.31) | contagem acima; §10 tem 11 itens por página |

#### 6.6.20.2 Ranking das frentes, pelo critério da missão

| # | Frente | Itens bloqueantes que resolve | Lacunas | Regras e padrões | Libera depois dela | **Não** libera | Custo |
|---:|---|---:|---:|---:|---|---|---|
| **1** | **Issues + suíte de testes** | **65** células pendentes na medição de então (31 `teste` + 34 `issue`) + **1** critério sem artefato (`CAP-nn`) | 0 | **3**: fixture/factory, dado de teste, matriz `CAP-nn` | onda 8 executável; `tests/` (12 páginas); **29** DoDs passam a ser verificáveis | código: §5.1 continua pedindo página `aceita` | **52** artefatos (40 issues + 12 páginas), 2–3 sessões |
| **2** | **Aceite (`M33` ampliado)** | **205** atos de aceite na medição de então (11 itens cada) | 0 | 0 (as regras já foram decididas em `Q-M33-1`–`6`) | o portão do código: 22 páginas de `src/` a escrever, série aplicada, `planos/metricas.md`, 144/144 | nada de outro sistema | **205** atos, **100–205 h** pela taxa de §4.1 |
| **3** | **Métricas (`M33` + `M20`)** | **1** página bloqueada (`planos/metricas.md`) | 0 | 0 | o número medido e o fechamento da release de documentação | nada antes da série | depende da frente 2 e da série aplicada |
| **4** | **Requisito das 94 tabelas** | **1** critério de saída violado ("`RF`/`RN`/`RNF` cobrem todas as tabelas") | 0 | **1**: política de cobertura | matriz completa; critério de §11.1 cumprido | nada de execução | 19 blocos (A) a 94 (B) |
| **5** | **Expandir outros sistemas** | **0** no `wyd-db` | **5** abertas, todas de captura (protocol 2, net 1, bots 1, client 1) | 0 | 12 sistemas, começando por `wyd-protocol` (5 de 131 páginas) | nada do `wyd-db` | alto; preso à captura A1 |

#### 6.6.20.3 A frente vencedora

**Vence a frente 1 — issues + suíte de testes — com o número 52:** são **52** artefatos (40 issues + 12
páginas de `tests/`), e eles não fecham a matriz célula a célula. As **12** páginas fecham o **único** critério
de saída do plano de testes sem artefato (o critério 6, matriz `CAP-nn`) e dão cenário às linhas que ela nomeia
— **15** das 40 hoje, `docs/specs/rastreabilidade.md` §3.4 —; as **40** issues são a fila do roadmap, e a coluna
`Issue` registra a onda de cada linha, não um número de issue; as células `Teste` seguem `P` até haver teste
executado. O censo das células pendentes tem dono: `rastreabilidade.md` §1. Custo: **2–3 sessões**.

**Por que não o aceite**, que liberava mais: porque libera por **205 atos** naquele momento (**168** depois de §6.6.27; hoje **141**, §6.6.31; 100–168 h), e o aceite não fica
menos necessário por ser adiado — ele continua sendo o portão de §5.1. A frente 1 é a única que produz
resultado **sem** depender do aceite, porque página de teste é planejamento, não código.

**Por que não as 94 tabelas:** o critério existe e está violado, mas fechá-lo custa de 19 a 94 blocos de
requisito e não move nenhuma execução — é a pergunta 4 do lote (§6.6.20.4), não a próxima sessão.

**Por que não os outros sistemas:** nenhuma das 5 lacunas abertas do projeto está no `wyd-db`, e todas
esperam captura (A1); abrir `wyd-protocol` antes disso repete o que já está bloqueado.

#### 6.6.20.4 As cinco perguntas do lote

**Q-BLK-12 — a frente vencedora começa pelos testes ou pelas issues?**

- **Medido.** As duas metades custam quase o mesmo (12 páginas × 40 issues) e **29 das 40** issues citam
teste no próprio `## DoD`; hoje nenhuma das duas colunas da matriz tem valor real.
- **A)** as **12 páginas de `tests/`** primeiro (com `index`, `specs/capacidades.md` e os 5 alvos), e as
  issues em 3 lotes por onda depois. **Recomendada:** fecha o único critério de saída sem artefato e é o
  que transforma `P` em alvo conferível nas 31 linhas de então (hoje **34**); sem a matriz `CAP-nn`, o DoD
  de teste das 29 issues continuava sem onde apontar — a matriz veio no lote H.
- **B)** as **40 issues** primeiro, de uma vez; testes depois.
- **C)** as duas em paralelo, em duas sessões.
- **Desbloqueia em A:** os critérios 1–3 e 6 de `plano-de-testes.md` §5 passam a ter artefato e as 29 issues
  com DoD de teste ficam verificáveis.
- **Resposta do dono (2026-11-24): A** — os testes vieram primeiro; as 12 páginas foram escritas no lote H
  (§6.6.21) e as issues ficam para os 3 lotes por onda de `Q-BLK-14`.

**Q-BLK-13 — o `tests/` entra inteiro (12 páginas) ou só os alvos (7)?**

- **Medido.** §6.17 define **12** páginas: `index`, `fixtures-e-factories`, `dados-de-teste`,
  `resources/README`, **5** `alvos/*` e **3** `specs/*`. O item 6 de §5 aponta para `specs/capacidades.md`.
- **A)** as **12**, como estão no molde. **Recomendada:** o `index` é o que dá comando de execução e
  `capacidades.md` é a matriz que o critério 6 exige; escrever só os alvos deixa o aceite sem prova.
- **B)** **7**: `index` + os 5 alvos + `specs/capacidades.md`, adiando fixtures e dado.
- **C)** adiar o `tests/` até a série existir.
- **Desbloqueia em A:** a árvore de teste inteira documentada e a matriz `CAP-nn` com dono.
- **Resposta do dono (2026-11-24): A** — as **12** páginas, escritas no lote H (§6.6.21), com a matriz de
  **23** capacidades em `tests/specs/capacidades.md`.

**Q-BLK-14 — as 40 issues são criadas agora, em lotes, ou só na onda 8?**

- **Medido.** 40 páginas `WYDDB-nnn` com DoD (nenhuma sem), **0** issues no GitHub e um board que já tem 40
  itens com campos preenchidos. Criar issue é escrita no GitHub e exige pedido explícito (`AGENTS.md` §5,
  item 9).
- **A)** criar em **3 lotes por onda**, o primeiro com as ondas 1–2 depois do aceite da primeira leva.
  **Recomendada:** issue cedo demais vira board morto; issue na hora da onda entra com DoD verificável e
  liga o PR.
- **B)** criar as **40** de uma vez, agora.
- **C)** não criar nenhuma; o board de 40 itens basta até a onda 8.
- **Desbloqueia em A:** a coluna `issue` das 31 linhas da matriz e o vínculo PR ↔ issue na primeira onda
  executada.
- **Resposta do dono (2026-11-24): A** — **3 lotes por onda**, o primeiro depois do aceite da 1ª leva;
  **0** issues criadas até aqui, porque a escrita no GitHub não foi pedida.

**Q-BLK-15 — como fechar o critério "`RF`/`RN`/`RNF` cobrem todas as tabelas"?**

- **Medido.** **15** das **109** tabelas têm requisito nomeado; o critério é item explícito da release de
  documentação em `wyd-db-1.0.0.md` §11.1.
- **A)** **19 blocos `RF-DB-1xx`**, um por página de domínio, cobrindo as 109 tabelas (1 bloco → N
  tabelas). **Recomendada:** fecha o critério com 19 blocos em vez de 94, e o domínio já é o dono do
  assunto no dicionário.
- **B)** **94 blocos** individuais, um por tabela descoberta.
- **C)** manter 15 e **alterar o critério** de saída, declarando DDL + dicionário como cobertura suficiente.
- **Desbloqueia em A:** a última coluna da matriz sem lacuna e o critério de §11.1 cumprido sem tocar em
  requisito existente.
- **Resposta do dono (2026-11-24): A** — **19** blocos `RF-DB-1xx` por domínio, um por página de domínio,
  na frente 4; nenhum bloco escrito ainda.

**Q-BLK-16 — depois da frente 1, o aceite do `wyd-db` ou outro sistema?**

- **Medido.** O aceite são **205** atos naquele momento (**168** depois de §6.6.27; hoje **141**, §6.6.31) e é o único caminho para a série, `M20` e 144/144; fora do `wyd-db`,
  as 5 lacunas abertas esperam captura A1.
- **A)** **aceite do `wyd-db` em 3 levas** (`migrations/` → `src/` → `docs/`). **Recomendada:** é o portão
  de §5.1 e a única frente que destrava código e a 144ª página.
- **B)** abrir `wyd-protocol` (5 de 131 páginas; 2 lacunas de captura).
- **C)** abrir `wyd-ops` (implantação, CI e pré-requisitos), que não depende da captura.
- **Desbloqueia em A:** `src/` (22 páginas a escrever), a série aplicada e `planos/metricas.md`.
- **Resposta do dono (2026-11-24): A** — aceite do `wyd-db` em **3 levas** (`migrations/` → `src/` →
  `docs/`), como frente seguinte; nenhuma leva iniciada.

#### 6.6.20.5 O que este lote não faz

- **Não escreve página de implementação:** nada de `tests/`, `prompts/` ou `skills/` sai daqui; aprovado o
  lote, a escrita é do seguinte — e foi o que aconteceu: as 12 páginas de `tests/` saíram no lote H
  (§6.6.21), com as cinco respostas em A registradas acima.
- **Não cria issue, label nem milestone:** `Q-BLK-14` decide **se** e **quando**; a escrita no GitHub exige
  pedido explícito.
- **Não reabre `Q-M33-1`–`6`** nem o critério de pronto de §10: o aceite da frente 2 é execução deles.
- **Não fecha lacuna:** o `wyd-db` não tem lacuna aberta; as 5 do projeto são de captura e ficam onde estão.
- **Não muda o censo:** 143 escritas e 1 bloqueada continuam valendo; este lote não move página.

### 6.6.21 Execução do lote H — as 12 páginas de `tests/` e a matriz `CAP-nn` (2026-11-24)

> **Retrato de 2026-11-24, ao fim do lote H — antes do aceite das 12.** Números e estados desta seção
> descrevem aquele dia; o valor vigente está em §6.6.23. Onde o texto disser “0 de 12” ou “página não
> aceita”, leia-se o estado daquele momento, não o de hoje.
**Sem arquivo no repositório:** `accounts.json`, `serie.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

O responsável aprovou `Q-BLK-12`, `Q-BLK-13`, `Q-BLK-14`, `Q-BLK-15` e `Q-BLK-16` em **A**, e as **12**
páginas de `tests/` foram escritas na mesma data, na ordem de §6.17. As issues, os blocos `RF-DB-1xx` e o
aceite **não** foram executados naquele lote: são as frentes 1 (segunda metade), 4 e 2, nesta ordem, e cada
uma esperava o seu gatilho — as issues, o aceite da 1ª leva, que veio em 2026-11-24 (§6.6.23).

| Ordem | Página | O que fixa | Medição que usa |
|---:|---|---|---|
| 1 | `tests/index.md` | árvore de `crates/wyd-db/tests/`, tabela alvo → arquivo → tipo → banco, comando por alvo e por cenário | **5** alvos; **2** sem banco |
| 2 | `tests/fixtures-e-factories.md` | inventário de `tests/common/` (**4** arquivos) e **6** factories, uma por agregado | limpeza por chave; **0** auxiliares implementados |
| 3 | `tests/dados-de-teste.md` | **5** contas de teste por papel e a pessoa com **5** contas de `CAP-14` | `seeds/dev.sql` **não** é dado de teste |
| 4 | `tests/resources/README.md` | formato de `accounts.json` e **8** fixtures de série (`valida/` + **7** defeitos) | cada fixture viola **1** invariante |
| 5 | `tests/alvos/contrato-migrations.md` | `CAP-01`–`CAP-07`, **7** cenários, cada um contra a série real e contra a fixture que falha | **0** tempo medido |
| 6 | `tests/alvos/contrato-catalogo.md` | `CAP-08`, paridade catálogo × série: **109** tabelas e **2** views | bloqueado: `banco-catalogo.md` não existe |
| 7 | `tests/alvos/sistema-pg.md` | `CAP-09`–`CAP-19`, **11** cenários contra PG17 real | PostgreSQL **17**; DSN `WYD_DB_URL_TEST` |
| 8 | `tests/alvos/sistema-sessoes.md` | `CAP-20`–`CAP-21`: `permission denied` observado, leitura só por view, auditoria append-only | papéis do provisionamento + `0032-admin-grants.md` |
| 9 | `tests/alvos/sistema-stack-caps.md` | `CAP-22`–`CAP-23` e a conferência que impede `CAP` órfã | **23** capacidades somadas nas seções da matriz |
| 10 | `tests/specs/contrato.md` | **8** regras da camada sem banco | parser único (`serie.rs`) |
| 11 | `tests/specs/sistema.md` | **18** regras: preparo, DSN, isolamento, dado | sem `SKIP`, sem `sleep`, sem id mágico |
| 12 | `tests/specs/capacidades.md` | a matriz `CAP-nn → alvo → cenário → requisito` | **23** capacidades, **0** cobertas |

**A matriz, em números:** **23** capacidades em **5** alvos — `contrato_migrations` **7** (`CAP-01`–`07`),
`contrato_catalogo` **1** (`CAP-08`), `sistema_pg` **11** (`CAP-09`–`19`), `sistema_sessoes` **2**
(`CAP-20`–`21`), `sistema_stack_caps` **2** (`CAP-22`–`23`). **12** têm requisito nomeado
(`CAP-10`–`19`, `CAP-21`, `CAP-23`); **11** são governadas por gate da série (`CAP-01`–`09`, `CAP-20`,
`CAP-22`). **0** estão cobertas, porque nenhum alvo existe em `Cargo.toml` — e a página diz isso com todas
as letras, em vez de trocar "planejado" por "coberto".

**O que a matriz não inventa:** `RF-DB-015` e `RF-DB-016` não geram `CAP` — são regra de `wyd-server`
(`docs/specs/rastreabilidade.md` §2.1). Onde a propriedade é da série e não há requisito numerado, a linha
aponta o gate e a dívida fica contada: são as **11** acima.

**Medido depois das doze páginas, em 2026-11-24:**

| Medida | Antes | Depois |
|---|---:|---:|
| páginas de `tests/` | 0 de 12 | **12 de 12** |
| matriz `CAP-nn` | não existia | **23** capacidades, **0** cobertas |
| páginas de `prompts/` e `skills/` | 0 de 18 | 0 de 18 (não pedidas) |
| páginas autorais de `docs/` | 143 | **143** (o `tests/` não é publicado) |
| arquivos `.md` do sistema, com cabeçalho | 336 / 328 | **348 / 340** |
| gate de documentação | 308 páginas, 306 fontes | **308 páginas, 306 fontes** |

**Uma consequência declarada:** `CAP-08` não pode rodar. O documento de catálogo
(`docs/dicionario-dados/banco-catalogo.md`) depende do PR de ferramentas e **não existe**, e o leitor em uso
não enxerga `CREATE TABLE esquema.tabela`. A paridade catálogo × série continua conferência manual, e o
alvo está escrito como desenho — não como verde.

**O que o lote H não fez (naquele dia):** não criou `Cargo.toml` nem código de teste, não executou cenário
nenhum, não criou issue, não escreveu bloco de requisito, não marcou página como `aceita` — o aceite veio no
lote seguinte (`Q-BLK-21` = A, §6.6.23) —, não tocou em seed, e não mexeu nos dossiês §6.6.8–§6.6.11 nem no
censo de §6.6.16.

### 6.6.22 Lote I — o próximo caminho, medido depois do lote H (2026-11-24)

O lote H escreveu as **12** páginas de `tests/` e a matriz `CAP-nn`. Este lote **não reusa contagem
nenhuma**: remediu as pastas, a matriz, a rastreabilidade, o roadmap e o dicionário neste turno (comando
na coluna de fonte) e escolhe o próximo passo pelo critério da missão — **maior número de bloqueios
liberados por unidade de trabalho, com o caminho crítico na frente**. Nenhuma resposta de `Q-BLK` é dada
aqui: as quatro perguntas de §6.6.22.5 são do dono.

#### 6.6.22.1 O que está de fato aberto (remedido em 2026-11-24)

| Medida | Valor medido agora | Como foi medido |
|---|---:|---|
| páginas autorais de `docs/` | **143**: 139 com `**Estado:**` (**137** `proposto`, **2** `aceito`) + 4 portas | `find docs -name '*.md' -not -path 'docs/dicionario-dados/gerado/*' -not -name 'proposta-*'`; `grep -l '^\*\*Estado:\*\*'` |
| páginas de `tests/` | **12 de 12** com `Estado`; **12** `aceito` (2026-11-24) | `find tests -name '*.md'` + `grep -l '^\*\*Estado:\*\* aceito'` |
| páginas de `src/` | **37** escritas; **37** `aceito` (2026-11-24) | `find src -name '*.md'` + `grep -l '^\*\*Estado:\*\* aceito'` |
| arquivos de `migrations/` | **29**: 28 páginas + `index`; **28** `aceito` (2026-11-24, §6.6.31) | `find migrations -name '*.md'` |
| **atos de aceite pendentes** | **142**: **142** em `docs/` + **0** em `src/` + **0** em `migrations/` + **0** em `tests/` | soma das quatro linhas acima; §10 tem 11 itens por página; migrations aceitas em §6.6.31 |
| matriz `CAP-nn` | **39** capacidades; **13** cobertas por teste executado; **39** com requisito; **0** só com gate | `tests/specs/capacidades.md` §7; cobertura medida em §6.6.30 |
| rastreabilidade | **40** linhas (17 `RF` + 7 `RN` + 13 `RNF` + 3 `UC`); **13** com teste executado, **24** em `P` e **3** `—`; **0** com issue | `docs/specs/rastreabilidade.md` §2; §6.6.30 |
| tabelas com requisito nomeado | **16 de 109** → **93** sem | `grep -oh 'schema.tabela'` em `requisitos-funcionais.md` + `regras-de-negocio.md` |
| roadmap | **40** páginas com `## DoD`; **29** citam teste | `ls docs/roadmap/WYDDB-*.md`; laço sobre o bloco `## DoD` |
| issues no GitHub | **não remedido** — escrita exige pedido explícito (`AGENTS.md` §5, item 9) | — |
| seeds bloqueadas | **6** de 34 migrations (28 `concluída`) | `migrations-checklist.md` §2 |
| lacunas do sistema | **0** abertas (38: 0 abertas, 0 propostas, 35 fechadas, 3 `externa`) | `lacunas.md` §1 |
| `docs/dicionario-dados/banco-catalogo.md` | **ausente**; o dicionário tem **109** páginas geradas + `index` | `ls`; `ls docs/dicionario-dados/gerado/*.md` |
| `prompts/` + `skills/` | **0 de 18** | `find prompts skills -name '*.md'` |
| `docs/planos/metricas.md` | **ausente** (1 página bloqueada por `M20`) | `ls` |

**Dois números não remedidos, declarados em vez de estimados:** as **0 issues criadas** naquela medição
(sem pedido para ler ou escrever o GitHub) e o **board de 40 itens**
(`../wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3, não reconferido). O resto desta tabela saiu de
comando naquele turno.

**Atualização de 2026-11-25:** a leitura somente de `lucascantarelli/wyd` encontrou **40 issues abertas**
(`WYDDB-001`–`040`) e milestones existentes; o “0” acima descreve a medição histórica, não o estado atual.
Nada foi criado ou alterado no GitHub por esta rodada.

#### 6.6.22.2 Tabela de bloqueio

“Atos” = páginas a aceitar, 11 itens de §10 cada. “Itens” = bloqueios contáveis que a frente libera.
“Destrava execução?” pergunta se, sem ela, algum artefato pode nascer (invariante §5.1).

| # | Frente | O que fica esperando | Itens | Custo | Itens/ato | Destrava execução? |
|---:|---|---|---:|---:|---:|---|
| **1** | **Aceite das 12 de `tests/`** | código dos **5** alvos, **23** capacidades, **31** células `teste`, **29** DoDs de issue | **88** | **12** atos | **7,3** | **sim** — teste é código |
| 2 | Aceite da leva `migrations/` | **28** `.sql` de DDL (34 − 6 seeds), série aplicada, `metricas.md` | 29 | 27 atos | 1,1 | **sim** — migration é código |
| 3 | Aceite da leva `src/` | implementação do crate (**59** páginas planejadas, **22** a escrever) | 1 artefato | 37 atos | 0,03 | sim, depois das frentes 1–2 |
| 4 | Aceite da leva `docs/` | fechamento editorial (144/144), **0** execução | 0 | 141 atos | 0,0 | não |
| 5 | Abrir as **40** issues | **34** células `issue`, **29** DoDs verificáveis, vínculo PR ↔ issue | 60 | 3 sessões | 20,0 | **não** — é registro |
| 6 | Executar as **23** capacidades | as 23 células da matriz | 23 | — | — | **bloqueada** pela frente 1 |
| 7 | Dicionário `banco-catalogo.md` | `CAP-08` e o gate `catálogo` | 1 | 1 página + PR de ferramentas (externo) | 1,0 | não |
| 8 | Fechar as **11** capacidades sem requisito | as 11 capacidades governadas por gate | 11 | 6 a 11 blocos `RNF` | 1,0–1,8 | não |
| 9 | **19** blocos `RF-DB-1xx` | critério de §11.1; **94** tabelas sem requisito | 94 | 19 blocos | 4,9 | não |
| 10 | Curadoria do 7662 | **6** seeds, 6 páginas, 6 `.sql`, `content_version = 7` | 19 | terceiro | — | não |
| 11 | `prompts/` + `skills/` | **18** páginas do teto | 18 | 2 sessões | 9,0 | não |
| 12 | `planos/metricas.md` | 1 página e 144/144 | 1 | depende da série | — | não |

**Como a recomendação sai da tabela:** primeiro a coluna “destrava execução?” — só as frentes **1**, **2** e
**3** estão no caminho crítico, e a **3** depende das duas anteriores. Entre **1** e **2**, a razão decide:
**7,3** contra **1,1** por ato, e **12** atos contra **27**. A frente 5 tem a maior razão bruta (**20,0**), mas não
destrava artefato nenhum: é registro, depende de autorização de escrita no GitHub e o gatilho dela já foi
fixado por `Q-BLK-14 = A` (depois do aceite da 1ª leva).

#### 6.6.22.3 A frente recomendada

**Aceitar as 12 páginas de `tests/` como primeira leva** — **12** atos que liberam **88** itens medidos,
e o único passo que transforma a matriz `CAP-nn` de planejamento em código autorizado. É também a leva
mais barata em número absoluto: **12** atos contra **27** (`migrations/`), **37** (`src/`) e **141**
(`docs/`).

**Ordem depois dela:**

1. **aceite das 12 de `tests/`** — 12 atos;
2. **aceite da leva `migrations/`** — 27 atos; libera os **28** `.sql` de DDL e a série aplicada;
3. **as 40 issues em 3 lotes por onda** — é o gatilho de `Q-BLK-14 = A`, cumprido depois do passo 1;
4. **aceite da leva `src/`** — 37 atos;
5. **aceite da leva `docs/`** — 141 atos, o fechamento editorial.

**Em paralelo, sem competir com a fila:** `Q-BLK-19` (a dívida das 11 capacidades, 6 a 11 blocos de
requisito) e o dicionário `banco-catalogo.md` quando o PR de ferramentas for autorizado. A curadoria do
7662 continua esperando terceiro e entra na frente da fila no dia em que a medição chegar.

#### 6.6.22.4 Por que não as outras cinco

- **Executar as 23 capacidades** — não é escolha: é consequência. Teste é código e o invariante §5.1
exige a página `aceita` antes. Sem a frente 1, a matriz continua **0 de 23** por construção.
- **Abrir as 40 issues** — maior razão bruta, mas registro puro: **0** artefato nasce de uma issue e a
escrita no GitHub não foi pedida. `Q-BLK-14 = A` já fixou o gatilho (depois do aceite da 1ª leva), então
esta frente é a **3ª** da fila, não a primeira.
- **Criar o dicionário `banco-catalogo.md`** — libera **1** capacidade (`CAP-08`) e depende do PR de
ferramentas, que é terceiro: **1** item por página mais bloqueio externo é a pior razão da tabela entre as
de decisão local.
- **Fechar as 11 capacidades sem requisito** — é dívida real (**11** itens) e barata, mas não destrava
execução nenhuma; por isso vai em paralelo, como pergunta (`Q-BLK-19`), não como frente.
- **Aceitar `migrations/` primeiro** (a ordem de `Q-BLK-16 = A` antes do lote H) — libera mais `.sql` em
número absoluto (**28**), mas custa **2,25×** os atos e adia as **31** células `teste` e os **29** DoDs
que só o código de teste fecha. As duas levas continuam na fila; muda a ordem entre elas.

#### 6.6.22.5 As quatro perguntas do lote

**Q-BLK-17 — a leva de aceite começa pelas 12 páginas de `tests/` ou pelas 27 de `migrations/`?**

**Fechada em 2026-11-24 (`A`), pela execução:** a leva 1 começou por `tests/` e as 12 páginas foram
revisadas e assinadas no mesmo dia (§6.6.23). O dossiê abaixo fica como registro da decisão; não há mais o
que responder.

- **Medido.** `tests/`: **12** atos → **88** itens (**7,3**/ato). `migrations/`: **27** atos → **29** itens
  (**1,1**/ato). `Q-BLK-16 = A` fixou as três levas (`migrations/` → `src/` → `docs/`) **antes** de o
  `tests/` existir; as 12 páginas não estavam na conta.
- **A)** `tests/` primeiro (**12 atos**), depois `migrations/` (**27**), `src/` (**37**) e `docs/` (**141**).
  **Recomendada:** maior razão da tabela entre as frentes que destravam execução e leva mais barata em
  atos; libera os 5 alvos, as 23 capacidades, as 31 células `teste` e os 29 DoDs.
- **B)** `migrations/` primeiro, como estava; `tests/` depois. Mantém a ordem de `Q-BLK-16` intacta, mas
  adia 88 itens para depois de 27 atos e deixa a matriz em **0 de 23** por mais uma leva.
- **Desbloqueia em A:** o código dos 5 alvos e, com ele, as 23 capacidades e as 31 células `teste`.

**Q-BLK-18 — página de teste `aceita` libera o código do alvo, ou o código espera a issue no GitHub?**

- **Medido.** O invariante §5.1 exige página `aceita` para qualquer código nascer; `Q-BLK-14 = A` fixou
  as issues **depois** do aceite da 1ª leva; o elo 5 da cadeia de F2 depende disso.
- **A)** o aceite libera o código do alvo e a issue registra o vínculo quando o lote da onda abrir.
  **Recomendada:** os 12 atos liberam os 5 alvos sem esperar 3 sessões de escrita no GitHub, que não foi
  pedida; o vínculo PR ↔ issue entra no passo 3 da fila.
- **B)** o código só nasce depois de a issue existir no GitHub: adia a frente por uma autorização que
  ainda não foi dada e não acrescenta prova nenhuma ao alvo.
- **Desbloqueia em A:** `Cargo.toml` e os 5 alvos de teste, com a matriz já escrita como contrato.
- **Fechada em 2026-11-24 (`A`), pelo dono:** a página `aceita` libera o código do alvo e a issue registra
  o vínculo depois. Na mesma data, as **37** páginas de `src/` foram revisadas e aceitas (§6.6.27) e o
  primeiro `Cargo.toml`/`src/dsn.rs` nasceu sob página aceita.
- **Estado em 2026-11-24, depois da assinatura:** a leva 1 está feita (12 `aceito`), então a resposta não
  espera mais nada a montante. Com **A**, o próximo ato é de execução, sem decisão nova: aceitar as **37**
  páginas de `src/` (todas `proposto`, mesmo padrão de revisão de `Q-BLK-21`) e só então o primeiro `.rs`,
  porque §5.1 do `AGENTS.md` exige página `aceita` para o que descreve o código. Com **B**, os **40** itens
  do GitHub entram na frente e o primeiro `.rs` espera a escrita no GitHub, que não foi pedida.

**Q-BLK-19 — como sai a dívida das 11 capacidades sem requisito nomeado?**

- **Medido.** **11** capacidades apontam gate em vez de `RF`/`RN`/`RNF` (`CAP-01`–`09`, `CAP-20`,
  `CAP-22`); elas usam **6** dos **8** gates de `plano-migrations.md` §7 (estrutura, constraints, tipos,
  catálogo, fronteira, ambiente); `RNF-DB-001` a `005` já estão em uso.
- **A)** **6** `RNF-DB-008` a `RNF-DB-013`, um por gate que governa capacidade. **Recomendada:** 6 IDs
  cobrem as 11 capacidades e a redação já existe no gate; `conteúdo` e `vertical` ficam fora porque não
  governam capacidade da matriz, e sim seed e corte vertical. A série começa em **008** porque
  `Q-BLK-24` = A ocupou `006` e `007` em 2026-11-24 (§6.6.26).
- **B)** **11** `RNF-DB-008` a `RNF-DB-018`, um por capacidade: mais rastreável, com 11 blocos para
  escrever e 11 linhas novas na matriz de rastreabilidade.
- **Desbloqueia em A:** as 11 capacidades deixam de citar gate solto e a dívida cai de 11 para 0, com 6
  linhas novas de requisito.
- **Fechada em 2026-11-24 (`A`), pelo dono:** cumprida na mesma data — os 6 blocos existem em
  `requisitos-nao-funcionais.md` §8–§13, as 11 linhas da matriz citam `RNF-DB-008`–`013` e a matriz de
  rastreabilidade passou a 40 linhas (§6.6.28).

**Q-BLK-20 — o dicionário `banco-catalogo.md` entra na `1.0.0` ou fica declarado pós-`1.0.0`?**

- **Medido.** O arquivo **não existe**; o gate `catálogo` e `CAP-08` dependem dele; o PR de ferramentas
  é pré-requisito declarado (`docs/guia-desenvolvedor/como-adicionar-migration.md` §"catálogo").
- **A)** fica **pós-`1.0.0`**, com `CAP-08` declarada **não executável** e o gate `catálogo` como
  conferência manual até lá. **Recomendada:** 1 capacidade não justifica bloquear a leva nem o PR
  externo; a dívida fica escrita, não maquiada.
- **B)** entra na `1.0.0` e bloqueia a leva de `migrations/` até o PR de ferramentas ser autorizado e
  executado: mais rigor no gate `catálogo`, com prazo de terceiro na frente crítica.
- **Desbloqueia em A:** a leva de `migrations/` anda sem esperar ferramenta; a lacuna de `CAP-08`
  continua visível e datada.

#### 6.6.22.6 O que este lote não faz

- **Não aceita página:** nenhuma página virou `aceito` neste turno; os atos de aceite do sistema seguiram todos pendentes.
- **Não fecha lacuna:** as 38 do sistema continuam 0/0/35/3.
- **Não cria issue, commit nem push:** a escrita no GitHub segue sem pedido.
- **Não enuncia requisito:** `Q-BLK-19` decide se e como os 6 `RNF` nascem.
- **Não responde as perguntas:** `Q-BLK-17` a `Q-BLK-20` ficaram abertas para o dono naquele lote; a `17`
  fechou pela execução (§6.6.22.5) e as outras três seguem abertas.

### 6.6.23 Lote J — as duas perguntas que o ranking do projeto revelou (2026-11-24)

O ranking publicado das frentes do projeto está em `../wyd-ops/docs/prioridade-do-projeto.md` e é o dono
da recomendação. Ao montá-lo, duas decisões que ninguém tinha enunciado apareceram e **travam a leva 1**
recomendada; elas ficam registradas aqui, no formato do pacote, e **nenhuma está respondida**.

**Q-BLK-21 — quem revisa as 12 páginas de `tests/` para o item 9 de §10?**

- **Medido.** `wyd-db-1.0.0.md` §10 exige, no item 9, revisão por **quem não escreveu**; as 12 páginas
  foram escritas pelo agente nesta rodada e **nenhuma** tem revisor registrado. A leva 1 depende disso: sem
  revisor, aceite não existe.
- **A)** o **dono revisa e assina** — ele não escreveu as páginas, logo o item 9 é satisfeito, com o nome e
  a data registrados no lote. **Recomendada:** a leva 1 não espera terceiro e a revisão fica com quem tem
  autoridade para aceitar.
- **B)** um **segundo desenvolvedor** revisa antes — mais independente, mas a leva 1 para até haver segunda
  pessoa disponível, e hoje não há ninguém escalado.
- **Desbloqueia em A:** as 12 páginas podem ser aceitas na próxima sessão, sem contratar revisor.

**Fechada em 2026-11-24 (`A`), pelo dono.** A folha `revisao-de-q-blk-21.md`, na raiz deste sistema, é o
registro da revisão: **7** páginas assinadas como estavam, **3** assinadas após correção de redação e **2**
tratadas antes da assinatura — `dados-de-teste.md` teve a prosa de §2 alinhada a `ck_account_slot_range` e
`alvos/contrato-catalogo.md` recebeu a nota de que **`Q-BLK-20`** decide o destino de `CAP-08`, sem antecipar
a decisão. Efeito: as **12** páginas estão `aceito (2026-11-24)`, `tests/` tem **0** `proposto`, e os atos de
aceite pendentes caíram em **12** — as 12 páginas de `tests/` saíram da fila —, com o valor vigente em **205**
naquele momento (**168** depois do aceite das 37 de `src/`; hoje **141**, §6.6.31).
Nenhum `.rs` nasceu naquele ato e nenhuma capacidade foi coberta —
`Q-BLK-18` (o aceite libera o código?) e `Q-BLK-20` seguiam abertas naquele lote; hoje as duas estão
respondidas (§6.6.27 e §6.6.31).

**Q-BLK-22 — as 12 páginas de `tests/` contam no critério de release de 144?**

- **Medido.** O `wyd-db` tem **143** páginas autorais em `docs/` + `planos/metricas.md` bloqueada = teto
  **144**; as 12 de `tests/` ficam **fora** de `docs/` e não entram em `../../wyd-docs/preparar.py`. O critério
  de release (`wyd-db-1.0.0.md` §2) fala das 144 de `docs/` escritas e `aceito`/`retirado`.
- **A)** **não contam**: o critério segue **144** e as 12 são artefato de engenharia, aceitas ou não fora
  dele. **Recomendada:** aceitar a suíte não muda o tamanho da release de documentação nem o teto.
- **B)** **contam**: o teto e o critério passam a **156** (144 + 12) e o censo de `docs/` deixa de ser o
  único número do fechamento documental.
- **Desbloqueia em A:** a leva 1 fecha sem reabrir a decisão de teto de `Q-BLK-01`.

**O que este lote não faz:** não responde as duas, não revisa página, não altera o teto e não mexe no
ranking de `../wyd-ops/docs/prioridade-do-projeto.md`, que é o dono da recomendação.

### 6.6.24 Lote K — o censo 15/31 e a pergunta que ele abre (2026-11-24)

> **Superado em 2026-11-24:** `Q-BLK-23` = A foi decidida e executada — os **16** cenários estão escritos
> (`CAP-24`–`CAP-39`, §6.6.29), e a lista de linhas sem cenário desta seção fica como registro da medição
> daquele dia.

A revisão da coluna `Página` de `docs/specs/rastreabilidade.md`, fechada em 2026-11-24, refez a conta linha a
linha contra as páginas: **15** das **31** linhas têm página que **nomeia** o requisito em uma `CAP-nn`;
**16** não têm — **13** porque nenhum cenário escrito as prova, **2** porque a regra é de `wyd-server`
(`RF-DB-015`, `RF-DB-016`) e **1** porque é gate de CI (`RNF-DB-001`). A lista está em
`rastreabilidade.md` §3.4.

**O que isso corrige no lote I.** O número “**88** itens (5 alvos, 23 capacidades, 31 células `teste`, 29
DoDs)” de §6.6.22.3 media a leva como se ela alcançasse as 31 linhas. Não alcança: o aceite das 12 páginas
cobre **15** linhas da matriz, e as **13** de requisito sem cenário seguem sem prova. O lote I fica como log
do que foi escrito; esta linha é a fotografia de hoje.

> **Recontado em 2026-11-24, depois de `Q-BLK-24` = A (§6.6.26 (h)).** O censo do lote K envelheceu em uma
> grandeza: a matriz tem **34** linhas, **15** com página e **19** sem — **16** sem cenário (as **13**
> listadas abaixo mais `RNF-DB-002`, `004` e `005`, que perderam o ponteiro falso para `sistema-pg`), **2** de
> `wyd-server` e **1** de gate de CI. A pergunta `Q-BLK-23` passa a valer para **16** linhas, não 13; o
> resto do lote K fica como registro da medição daquele dia.

**Q-BLK-23 — as 16 linhas sem cenário entram na `1.0.0`?**

- **Medido.** **16** linhas têm critério de aceitação na página dona e **nenhuma** `CAP-nn` que as
  nomeie: **13** requisitos — `RF-DB-001`, `002`, `004`, `007`, `008`, `010`, `012`, `013`,
  `RN-DB-006`, `007` e `UC-DB-001`–`003` — e **3** `RNF` (`RNF-DB-002`, `004`, `005`, que medem espera de
  DDL, p95 e tempo de boot). A matriz tem **23** capacidades e **0** cobertas.
- **A)** escrever os **16** cenários nos alvos escritos, com **uma `CAP-nn` por linha** (numeração a
  alocar), **antes** da leva de `migrations/`. **Recomendada:** `AGENTS.md` §5, item 1 exige página aceita
  antes do código, e migration sem cenário é regra sem prova; os cenários entram em `tests/`, fora do teto
  de **144** (`Q-BLK-22` = A). Os três `RNF` medem números que ainda não têm limiar (`M20`), e o cenário
  deles mede e registra, sem fixar teto.
- **B)** aceitar as 12 e adiar as 16. **Consequência:** a matriz fica em **15 de 34** durante a leva de
  `migrations/`, e **13** regras (`CHECK`, FK, índice único) entram no schema sem cenário que as recuse.
- **C)** declarar as 16 fora da `1.0.0`. **Consequência:** exige retirar ou rebaixar **16** linhas das
  páginas donas — decisão maior que esta, e dona de outra pergunta; os três `RNF` ainda teriam de
  explicitar que ficam sem prova até `M20`.
- **Desbloqueia em A:** as 34 linhas passam a ter cenário e a leva de `migrations/` nasce com prova de
  recusa para cada regra; em B, a frente anda e a dívida fica datada.

**O que este lote não faz:** não escreve cenário, não atribui `CAP-nn`, não muda a matriz de capacidades,
não reabre `Q-BLK-12`–`Q-BLK-22` e não mexe no teto de 144.

### 6.6.25 Lote L — pré-aceite das 12 páginas de `tests/` (2026-11-24)

> **Superado no mesmo dia:** as 12 foram corrigidas e aceitas em 2026-11-24 (`Q-BLK-21` = A). O estado
> vigente está em §6.6.23 e em `revisao-de-q-blk-21.md` §5; a tabela abaixo fica como registro do pré-aceite.

Conferência das **12** páginas de `tests/` contra os **11** critérios de §10 de `wyd-db-1.0.0.md`, feita em
2026-11-24, uma a uma, sem editar página e **sem marcar nenhuma como `aceito`** naquele momento (as 12
estavam `**Estado:** proposto`). Veredito do pré-aceite: **0 de 12** prontas para assinar então; **6**
critérios fechavam nas 12, **5** não. As correções de §6.6.25.4 e a assinatura de §6.6.23 mudaram o resultado.

#### 6.6.25.1 Os 11 critérios, como estavam no pré-aceite

> A coluna **Resultado** é o retrato daquele dia, antes das correções de §6.6.25.4 e do aceite de §6.6.23;
> não é o estado de hoje.

| # | Critério (§10) | Resultado | Evidência de 2026-11-24 |
|---:|---|---|---|
| 1 | cabeçalho de §7.1 e pergunta de §2 | **falta** | as 5 linhas existem em **12 de 12**; mas `Público:` não é “um dos públicos de §2” em nenhuma: usam rótulos derivados ("Desenvolvedor do `crates/wyd-db`", "revisor de PR", "revisor de migration", "revisor de segurança", "quem revisa aceite"). A pergunta de §2 (QA/testador) é respondida nas 12 |
| 2 | seções obrigatórias do tipo | **ok** | §6.17 fixa o obrigatório por arquivo: índice (árvore, tabela alvo→arquivo→tipo→banco, rodar tudo, rodar um alvo); `alvos/` (capacidades, cenários, pré-requisito, falha, tempo); `specs/` (regras sem banco, regras com banco + isolamento, matriz); `fixtures`, `dados`, `resources` — presentes e preenchidas em 12 de 12 |
| 3 | todo número com fonte **e data** | **parcial** | fonte citada em quase todos; onde o número é do dono, o rastro leva à página dona. Falha em **3** páginas: **23**, **0**, **12**, **11**, **5** (`tests/specs/capacidades.md` §7) e **12** páginas, **23**, **0** (`tests/index.md` §2.1 e §4) nascem aqui **sem data**; “os **seis** papéis do provisionamento” (`tests/alvos/sistema-sessoes.md` §3) está **sem fonte** (`convencoes.md`: “número sem fonte e data é hipótese”) |
| 4 | nível `V`/`H`/`P` por afirmação | **falta** | **0** ocorrências de `V`, `H` ou `P` nas 12 páginas; **88** páginas de `docs/` usam a marca. A linha de estado (`proposto`) basta para página de um nível só (`convencoes.md` §Marcações), mas estas misturam fato medido (0 alvos, 23 `CAP`) e plano sem legenda |
| 5 | todo link resolve | **ok** | gate de 2026-11-24: 309 páginas, links, âncoras e navegação verdes |
| 6 | diagramas renderizam | **ok** | **0** blocos Mermaid em `tests/`; §7.3 não exige diagrama nesta pasta, então não há o que renderizar |
| 7 | nenhum segredo | **ok** | só nomes de variável (`WYD_DB_URL_*`), nenhum DSN, hash ou senha real; a senha de `accounts.json` é declarada fictícia |
| 8 | nenhum `TODO`/“em construção”/seção vazia | **ok** | 0 `TODO`, 0 `FIXME`, 0 “em construção”; nenhuma seção sem conteúdo |
| 9 | revisão de quem não escreveu | **falta** no pré-aceite | nenhuma das 12 registrava revisor; foi o último critério fechado, pela revisão e assinatura do dono em 2026-11-24 (`Q-BLK-21` = A; §6.6.23) |
| 10 | linkada no `index.md` da pasta; métricas suspensa | **ok** | as 11 estão em `tests/index.md` §2.1; o índice é a porta da pasta; `planos/metricas.md` suspensa por `M33` |
| 11 | `Base:` preenchido e nada pendurado em material externo | **falta** | `Base:` preenchido em 12 de 12 e nenhuma citação a legado ou à web; mas há **4** defeitos de caminho (abaixo) |

**Os 4 defeitos do critério 11:**

1. `tests/specs/capacidades.md` — `Base:` e §1 e §8 apontam `../plano-migrations.md`, que a partir de
   `tests/specs/` resolve para `tests/plano-migrations.md` (inexistente); o caminho é `../../plano-migrations.md`
   (linhas 4, 21 e 121).
2. `tests/dados-de-teste.md` — `Base:` cita `resources/accounts.json`, arquivo **planejado e inexistente**
   (a própria página diz “nenhuma conta criada”); `Base:` tem de apontar material consultado ou `—`.
3. `tests/fixtures-e-factories.md` §1 — `../../src/migrate/spec.md`; a partir de `tests/` o caminho é
   `../src/migrate/spec.md` (linha 31).
4. `tests/index.md` §6, `tests/specs/sistema.md` §7 e `tests/alvos/sistema-stack-caps.md` §8 — citam
   `docs/planos/plano-ci-real.md`, que **não existe**; o único arquivo que o menciona como futuro é
   `docs/planos/plano-de-testes.md` §81. É referência a dono não escrito, não material externo.

#### 6.6.25.2 Página por página, no pré-aceite
**Sem arquivo no repositório:** `../plano-migrations.md`, `../../src/migrate/spec.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Página | Critérios que faltavam | O que faltava, em concreto |
|---|---|---|
| `tests/index.md` | 1, 3, 4, 9, 11 | `Público:` fora de §2; data para **12**/**23**/**0** (§2.1 e §4); legenda de níveis; revisor; `plano-ci-real.md` inexistente (§6) |
| `tests/specs/capacidades.md` | 1, 3, 4, 9, 11 | `Público:` fora de §2; data para **23**/**0**/**12**/**11**/**5** (§7); legenda; revisor; `../plano-migrations.md` → `../../` (4, 21, 121) |
| `tests/specs/contrato.md` | 1, 4, 9 | `Público:` fora de §2; legenda; revisor. Sem defeito de caminho |
| `tests/specs/sistema.md` | 1, 4, 9, 11 | `Público:` fora de §2; legenda; revisor; `plano-ci-real.md` (§7) |
| `tests/alvos/contrato-migrations.md` | 1, 4, 9 | `Público:` fora de §2; legenda; revisor |
| `tests/alvos/contrato-catalogo.md` | 1, 4, 9 | `Público:` fora de §2; legenda; revisor |
| `tests/alvos/sistema-pg.md` | 1, 4, 9 | `Público:` fora de §2; legenda; revisor |
| `tests/alvos/sistema-sessoes.md` | 1, 3, 4, 9 | `Público:` fora de §2; “os **seis** papéis do provisionamento” sem arquivo ou § de fonte; legenda; revisor |
| `tests/alvos/sistema-stack-caps.md` | 1, 4, 9, 11 | `Público:` fora de §2; legenda; revisor; `plano-ci-real.md` (§8) |
| `tests/fixtures-e-factories.md` | 1, 4, 9, 11 | `Público:` fora de §2; legenda; revisor; `../../src/migrate/spec.md` → `../src/` (31) |
| `tests/dados-de-teste.md` | 1, 4, 9, 11 | `Público:` fora de §2; legenda; revisor; `Base:` cita `resources/accounts.json` inexistente |
| `tests/resources/README.md` | 1, 4, 9 | `Público:` fora de §2; legenda; revisor. Sem defeito de caminho |

#### 6.6.25.3 O que faltava para a leva poder assinar

1. **`Público:` — 12 linhas.** Normalizar para um público de §2 (QA / testador, com “Desenvolvedor novo”
   onde a página é do crate). É conformidade com §7.1, não decisão nova.
2. **Legenda de níveis — 12 páginas.** Uma linha de legenda `P`/`V` com a fonte local, ou marca por bloco,
   como fazem as 88 páginas de `docs/`.
3. **Data nos números que nascem aqui — 2 páginas:** `tests/specs/capacidades.md` §7 e `tests/index.md`
   §2.1 e §4, medidos em 2026-11-24.
4. **Caminhos — 4 defeitos:** os dois `Base:` e os dois trechos citados em §6.6.25.1; a referência a
   `plano-ci-real.md` fica como pendência até a página existir.
5. **Revisão de quem não escreveu — `Q-BLK-21`.** É o único critério que nenhuma edição de página resolve:
   com **A**, o dono revisa e assina; foi a última coisa antes de `**Estado:** aceito`, e está feita
   (§6.6.23).

**O que este lote não faz:** não edita as 12 páginas, não muda `**Estado:**`, não assina revisão, não cria
`CAP-nn` e não decide `Q-BLK-21`. Depois dos itens 1–4, o critério 9 era o único aberto — e ele era do dono;
foi fechado pela assinatura de §6.6.23.

#### 6.6.25.4 Correções aplicadas (2026-11-24)
**Sem arquivo no repositório:** `../../plano-migrations.md`, `../plano-migrations.md`, `../../src/migrate/spec.md`, `../src/migrate/spec.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Os itens 1–4 de §6.6.25.3 foram aplicados nas **12** páginas, que então seguiam `**Estado:** proposto`:

| Correção | Onde | Resultado |
|---|---|---|
| `Público:` de §2 | 12 páginas | `QA / testador` nas 12 — é o público de `tests/` em §2 do plano diretor |
| legenda `P`/`H`/`V`/sem marca | 12 páginas, logo após a linha de estado | forma de `padroes-postgres.md`: o que cada nível cobre **na página** e a fonte local |
| data nos números | `tests/specs/capacidades.md` §7 e §7.1; `tests/index.md` §2.1 e §4 | “medido em 2026-11-24” antes do censo e junto ao número |
| caminho `Base:` | `tests/specs/capacidades.md` linhas 4, 30 e 132 | `../plano-migrations.md` → `../../plano-migrations.md` |
| caminho `Base:` | `tests/dados-de-teste.md` | `resources/accounts.json` (inexistente) → `resources/README.md` |
| caminho `src/` | `tests/fixtures-e-factories.md` §1 | `../../src/migrate/spec.md` → `../src/migrate/spec.md` |
| `plano-ci-real.md` | `tests/index.md` §6, `tests/specs/sistema.md` §7, `tests/alvos/sistema-stack-caps.md` §8 | declarada **planejada e não escrita**, com a menção de `docs/planos/plano-de-testes.md` §81 |

**O que ficou para o passo seguinte:** o critério **3** em `tests/alvos/sistema-sessoes.md` §3 (“os **seis**
papéis do provimento”) — número sem arquivo ou § de fonte, fora da lista mecânica — e o critério **9** nas
12 (`Q-BLK-21`). O primeiro foi fechado em §6.6.25.5; o segundo foi resolvido pela revisão e assinatura do
dono no mesmo dia, e as 12 passaram a `aceito (2026-11-24)` (§6.6.23).

#### 6.6.25.5 Fonte fechada e estado dos 11 critérios (2026-11-24)

**Os “seis papéis”.** A contagem existe e é verificável: a tabela de
`docs/arquitetura/seguranca.md` §“Papéis do banco” lista **seis** — `wyd_owner`, `wyd_migrator`, `wyd_game`,
`wyd_portal`, `wyd_admin`, `wyd_read` — e `docs/arquitetura/implantacao.md` §1, passo 2, confirma que
`provision apply` deixa “os 6 papéis”. `ADR-DB-0001` registra que o total passou de cinco a seis com
`M27`/`0032`. A frase de `tests/alvos/sistema-sessoes.md` §3 passou a citar as duas fontes e a nomear os
seis; a contagem foi conferida contra a tabela, não copiada.

**Outras afirmações numéricas sem fonte.** Nenhuma outra das 12 páginas afirmava fato externo sem fonte.
Duas ficaram com o mesmo defeito de forma e foram corrigidas: `tests/alvos/contrato-migrations.md` §1
passou a citar os **34** de `plano-migrations.md` §1; `tests/index.md` §intro trocou “as três regras”,
contagem sem referente, por “as regras que valem para toda a suíte, em `specs/`”.

**Achado que não é falta de fonte — mapeamento `CAP` ↔ `RNF`.** `CAP-17`–`CAP-19` apontam
`RNF-DB-002`, `RNF-DB-005` e `RNF-DB-004`, mas os três RNF medem espera de DDL (`10 s`, **dois**
processos), tempo de boot e latência p95: nenhum enuncia o *deadlock* com quatro sessões, a versão única
lida no boot ou a espera finita do pool. Os números dos cenários (`quatro` sessões, `duas`
`content_version`) não vêm de requisito nenhum, então foram marcados **H** com a pergunta registrada em
`tests/specs/capacidades.md` §4 e §9 e em `tests/alvos/sistema-pg.md` §6 — o dono é quem enuncia os
requisitos, não este lote. A pergunta ganhou dossiê em **§6.6.26** (`Q-BLK-24`), com as três opções, e foi
aprovada em **A** e executada no mesmo dia (§6.6.26 (h)). A leitura da árvore acrescentou dois fatos ao achado: a entrada de `CAP-18` não é
representável (`0001` fecha `content_version` em uma linha, prova 9 de §9) e nenhum dos **28** blocos de
`docs/specs/rastreabilidade.md` §2 enuncia as três propriedades — então não há requisito a que reapontar
dentro de `docs/specs/`.

| # | Critério | Estado | Onde |
|---:|---|---|---|
| 1 | cabeçalho §7.1 + pergunta §2 | **ok** | 12/12 com as 5 linhas e `Público: QA / testador`, um dos públicos de §2 |
| 2 | seções obrigatórias | **ok** | molde de §6.17 conferido em 12/12 |
| 3 | número com fonte e data | **ok** | seis papéis (fonte acima); 34 (`plano-migrations` §1); censo datado em `capacidades` §7/§7.1 e `index` §2.1/§4; “três regras” removido |
| 4 | nível `V`/`H`/`P` | **ok (com H registrado)** | legenda nas 12; `H` de `CAP-17`–`CAP-19` localizado em §9/§6 |
| 5 | links resolvem | **ok** | gate de 2026-11-24 |
| 6 | diagramas | **ok** | 0 Mermaid na pasta |
| 7 | segredo | **ok** | só nomes de variável; senha fictícia declarada |
| 8 | placeholder | **ok** | 0 `TODO`/“em construção”/seção vazia |
| 9 | revisão de quem não escreveu | **ok** | `Q-BLK-21` = A: o dono revisou e assinou as 12 em 2026-11-24; folha e registro em `revisao-de-q-blk-21.md` §5 |
| 10 | índice da pasta | **ok** | 11 links em `tests/index.md` §2.1 |
| 11 | `Base:` e material externo | **ok** | `Base:` em 12/12; 4 caminhos corrigidos em §6.6.25.4; `plano-ci-real.md` declarada planejada |

**Estado das 12 páginas:** `**Estado:** aceito (2026-11-24)` em todas, **0** `proposto`. Os 11 critérios
fecham — o 9 foi o último, com a assinatura do dono em §6.6.23.

### 6.6.26 Mapeamento `CAP-17`–`CAP-19` ↔ `RNF-DB` — `Q-BLK-24` (1 matriz, 1 alvo)

**Pergunta:** "as três capacidades de sistema que nenhum requisito enuncia ganham requisito próprio, ou
são reapontadas para um dos cinco `RNF-DB` que existem?"

**(a) O que está em jogo.** `tests/specs/capacidades.md` é o único lugar que liga `CAP-nn` a requisito, e
`docs/specs/rastreabilidade.md` §1.1 usa essa ligação para dizer que a linha **tem página**. Ponteiro falso
não reprova gate nenhum: ele faz três requisitos parecerem provados por um cenário que prova outra coisa —
e o roteiro de §1.1 (`requisito → alvo → página`) passa a apontar para um teste que mede outro número.

**O que cada lado diz**, lido em 2026-11-24, **antes** da execução de (h):

| `CAP` | O que o cenário faz | Apontava então | O que o `RNF` apontado mede |
|---|---|---|---|
| `CAP-17` | quatro sessões escrevem em ordem trocada; falha se houver deadlock | `RNF-DB-002` | espera pelo advisory lock **da série**, `10 s`, com **dois** processos (`requisitos-nao-funcionais.md` §2) |
| `CAP-18` | duas `content_version` correntes e o boot recusando a ambiguidade | `RNF-DB-005` | tempo entre o início do processo e o pool aceitar conexão, **a medir** (§5) |
| `CAP-19` | esgotar o pool, medir a espera até `acquire_timeout` e o reuso | `RNF-DB-004` | p95 da leitura de personagem, **a medir** (§4) |

**(b) O que a leitura da árvore mostrou** — três fatos que a divergência escondia:

1. **`CAP-18` não é executável como está escrito.** `content_version` nasce em `0001` com
   `pk_content_version` (`id`) e `ck_content_version_linha_unica CHECK (id = 1)`, e a prova 9 de
   `migrations/0001-bootstrap.md` §9 já recusa `id = 2` com `23514`. Duas linhas correntes **não são
   representáveis**: o banco impede a ambiguidade na escrita, então não há boot que a recuse. O cenário
   pede uma entrada que o schema não produz.
2. **Os três `RNF` não têm capacidade própria.** `RNF-DB-002` promete "o teste de concorrência com
   **dois** processos" (§2), `RNF-DB-004` promete p95 (§4) e `RNF-DB-005` promete tempo de boot (§5);
   nenhuma das 23 `CAP` mede essas três coisas. Se os `RNF` ficam como estão, os três ficam **sem teste** —
   hoje eles apontam `sistema-pg`, que não mede nenhum dos três.
3. **Nenhum requisito enuncia as três propriedades.** Os **28** blocos de `docs/specs/rastreabilidade.md`
   §2 (16 `RF` + 7 `RN` + 5 `RNF`) não dizem "sem deadlock", "espera finita do pool" nem "versão única".
   Elas estão escritas **fora** de `docs/specs/`: `guia-desenvolvedor/pool-e-transacoes.md` §3 ("a espera
   é finita e é de 30 s"), `specs/visao-e-escolpo.md` §1 ("o boot lê um catálogo misturado"),
   `arquitetura/modelo-logico.md` §3 ("um boot lê um corte; mistura de versões é recusada"),
   `specs/fluxos/carga-de-conteudo.md` §2–§3 e `arquitetura/implantacao.md` §1, passo 4. "Reapontar ao requisito
   que já a enuncia" não tem alvo **dentro** de `docs/specs/` — os alvos são guia, arquitetura e fluxo,
   que não são donos de requisito.

**O limiar de `RNF-DB-006` não tem página de fonte.** Varredura de 2026-11-24 na árvore viva
(`deadlock`, `40P01`, `SQLSTATE`): nenhuma página de `docs/` trata deadlock como falha. `padrao-banco.md`
**§13 é "Fontes"** — bibliografia de 16 links do PostgreSQL, sem entrada de deadlock (o item 15 cobre
advisory lock) —, então não serve de § de regra; o §10 daquela página fala do teste de violação de
`CHECK`/índice único, não de escrita concorrente. As três páginas mais próximas governam outra coisa:
`arquitetura/padroes-postgres.md` §6 fixa **três** garantias de concorrência (um arquivo/uma transação;
dois processos e a série; recurso finito por índice único) e declara em nota que não decide o resto;
`guia-teste/testes-de-sistema.md` §"Tempo limite" exige limite declarado e diz que ele é **da operação,
não da suíte**; e a única frase que trata `deadlock detected` como falha está em
`tests/alvos/sistema-pg.md` §2/§4 — que é o próprio cenário, não fonte. **Declarado:** o limiar de
`RNF-DB-006` se apoia só no SQLSTATE `40P01` (`deadlock_detected`) e na decisão do dono; se A for
aprovada, ele entra no bloco assim declarado, sem citação de empréstimo.

**Os resumos a jusante já copiaram as capacidades, não os `RNF`.** `rastreabilidade.md` §2.3 resumia
`RNF-DB-002` como "concorrência sem deadlock em 4 processos", `004` como "pool: espera e reuso" e `005`
como "boot lê um único `content_version`"; `stakeholders.md` §2 repetia ("concorrência sem deadlock; pool
medido"). Isso não é segunda fonte: é a mesma propriedade das `CAP` escrita na coluna de requisito, e o
projeto dizia **duas coisas** sobre o mesmo ID. **Corrigido em (h):** os três resumos voltaram ao que os
blocos medem, e as duas páginas apontam `RNF-DB-006`, `RNF-DB-007` e `RF-DB-017` para as propriedades.

**(c) Opções reais**

| Opção | O que a árvore passa a dizer | Custo | Veredito |
|---|---|---|---|
| **A** | **requisito próprio por propriedade:** `RNF-DB-006` (concorrência de escrita sem deadlock; reprova qualquer `40P01`, limiar **sem página de fonte** — parágrafo acima), `RNF-DB-007` (espera por conexão finita em **30 s**, `acquire_timeout`, e conexão reutilizada) e `RF-DB-017` (o banco admite **uma** versão corrente e o personagem a referencia por FK). `CAP-18` tem a entrada reescrita: `id = 2` recusado (`23514`) em vez de "duas correntes" | 3 blocos de requisito (1 limiar declarado sem fonte), 1 entrada de cenário, censo recontado | **Recomendada** |
| B | **reescrever os três `RNF`** para as propriedades das `CAP` (é a única leitura coerente de "reapontar": manter os `RNF` e apontar as `CAP` para eles é a afirmação falsa de hoje) | reabre `Q-BLK-07` = A, apaga o único limiar de tempo do projeto (`10 s` de DDL) e as duas métricas "a medir"; o lock da série fica sem requisito e pede `RNF` novo de qualquer forma | **Não recomendada** |
| C | **híbrido:** `RNF-DB-006` e `007` para `CAP-17`/`CAP-19` e **retirar `CAP-18`** (`retirado`, motivo: propriedade estrutural já provada pela prova 9 de `0001` e pela FK) | 2 blocos, 1 retirada, matriz cai de 23 para 22 capacidades | **Não recomendada** |

**(d) Recomendação e razão: A.** As três capacidades são de **duas naturezas**: `CAP-17` e `CAP-19` são
propriedade de qualidade (erro e espera), `CAP-18` é regra de recorte de conteúdo — forçá-las todas em
`RNF`, ou nenhuma, erra em um dos dois lados. A é a única opção em que **toda** linha da matriz tem
requisito verdadeiro e os cinco `RNF` de `Q-BLK-07` continuam intocados. Dois dos três blocos novos
nascem de fonte: `RNF-DB-007` cita `configuracao.md` §3 e `pool-e-transacoes.md` §3 (`30 s` é default do
`sqlx`, `V`) e `RF-DB-017` cita quatro páginas que já escrevem a regra. O terceiro, `RNF-DB-006`, tem
limiar **0** que não é número de desempenho — é ausência do erro `40P01`, como o `SKIP` = 0 de
`RNF-DB-003` —, mas, ao contrário daquele, **não tem página que o enuncie**: o limiar se apoia só no
SQLSTATE e entra no bloco com essa declaração, não com citação de empréstimo. C economiza um bloco e, em
troca, tira do teste a única capacidade de recorte de conteúdo.

**O que A não faz:** não escolhe número de cenário. As **quatro** sessões de `CAP-17` continuam parâmetro
do teste, e o tempo limite dele continua sendo teto de harness
(`docs/guia-teste/testes-de-sistema.md` §Tempo limite: "teto próprio, sem número"), não limiar de `RNF`.

**(e) Consequência de A, com número** — recontada em 2026-11-24, depois da execução de (h).
`docs/specs/rastreabilidade.md` passa de **31** para **34** linhas (16 → 17 `RF`, 5 → 7 `RNF`). As linhas
com página continuam **15**: as três novas ganham cenário (`CAP-17`, `CAP-18`, `CAP-19`) e os três `RNF`
que apontavam para requisito alheio perdem o ponteiro (`002`, `004` e `005` passam a `—`), então os
ponteiros para `sistema-pg` continuam **13**. A previsão escrita antes da execução era 15 → **18** e
13 → **16**; ela supunha que `RNF-DB-002`, `004` e `005` mantivessem `sistema-pg`, o que contradiz o
fato 2 de (b) — número previsto, não medido. As linhas sem página vão de **16** para **19**: **16** sem
cenário (as **13** de antes mais `RNF-DB-002`, `004` e `005`), **2** de outro sistema e **1** de gate de CI.
O censo de tabela muda por `RF-DB-017`: **15** de **109** → **16** de **109**, e **94** → **93** sem
requisito. `capacidades.md` §7 continua com **12** capacidades com requisito nomeado — as mesmas —, mas
três delas deixam de apontar para o requisito errado. `Q-BLK-19` reservava `RNF-DB-006`–`011`; com A
executada primeiro, a série dela vira `RNF-DB-008`–`013` (regra do próximo livre,
`docs/specs/index.md` "Convenção de ID").

**(f) Fontes fechadas:** `tests/specs/capacidades.md` §4 e §9; `tests/alvos/sistema-pg.md` §2 e §6;
`docs/specs/requisitos-nao-funcionais.md` §2–§5 e §9; `docs/specs/rastreabilidade.md` §2.3;
`docs/specs/stakeholders.md` §2; `migrations/0001-bootstrap.md` §4, §6 e §9 (prova 9);
`docs/guia-administrador/configuracao.md` §3; `docs/guia-desenvolvedor/pool-e-transacoes.md` §1 e §3;
`docs/specs/visao-e-escolpo.md` §1; `docs/arquitetura/modelo-logico.md` §3;
`docs/specs/fluxos/carga-de-conteudo.md` §2–§3; `docs/arquitetura/implantacao.md` §1; `padrao-banco.md`
§10 e §13 (bibliografia, **não** regra); `docs/arquitetura/padroes-postgres.md` §6;
`docs/guia-teste/testes-de-sistema.md` §"Tempo limite".

**(g) O que teria ficado quebrado se ficasse aberta — não se aplica.** A decisão foi **A** e (h) a
executou. O registro serve ao caso inverso: manter a dúvida declarada como `H` não reprovava gate nem o
aceite das 12 páginas, porque o critério 4 a admite; o que seguia quebrado era o uso de
`rastreabilidade.md` §1.1 como prova de rota para as três linhas.

**(h) Executado em 2026-11-24 — opção A aprovada pelo dono.** O que mudou nesta entrega:

| Arquivo | Mudança |
|---|---|
| `docs/specs/requisitos-nao-funcionais.md` | §6 `RNF-DB-006` (limiar **0** de `40P01`, **sem página de fonte**, declarado) e §7 `RNF-DB-007` (`30 s` de `acquire_timeout` e reuso); a tabela de dimensões e a seção final passaram a §8 e §9 |
| `docs/specs/requisitos-funcionais.md` | `RF-DB-017` (linha única de `content_version`, `23514` em `ck_content_version_linha_unica`, `23503` em `fk_character_content_version`); próximo `RF` livre = **018** |
| `tests/specs/capacidades.md` §4 e §9 | `CAP-17` → `RNF-DB-006`, `CAP-18` → `RF-DB-017`, `CAP-19` → `RNF-DB-007`; o parágrafo **H** virou decisão; `Q-BLK-19` renumerada para `008`–`013` |
| `tests/alvos/sistema-pg.md` §1, §2 e §6 | `CAP-18` = recusa da segunda linha; `boot_uma_versao` recusa `id = 2`; as duas dúvidas de §6 fecharam com o rastro |
| `docs/specs/rastreabilidade.md` §1–§3.4 | **34** linhas; resumos de `RNF-DB-002`, `004` e `005` corrigidos para o que os blocos medem; três linhas novas; **16** de **109** tabelas com requisito |
| `docs/specs/index.md`, `stakeholders.md` §2, `arquitetura/modelo-conceitual.md` §4 | último número de cada série, ponteiros de ator e a relação de versão de conteúdo |
| `../wyd-ops/docs/prioridade-do-projeto.md` §2, §5, §9, §10 e §11 | censo **15 de 34**; `Q-BLK-24` fechada — **`Q-BLK`** abertas **8 → 7** —, `Q-BLK-21` fechada — **7 → 6** —, `Q-BLK-17` respondida pela execução — **6 → 5** —, `Q-BLK-18` respondida em A — **5 → 4** — com as 37 de `src/` aceitas (§6.6.27), e a pergunta de foco `Q-FOCO-3` nasce em §11 |

**O que a execução não fez (naquele lote):** nenhuma página virou `aceito` (seguia **0** em `tests/`; hoje
12 de `tests/` e 37 de `src/`, §6.6.27), nenhum teste rodou, nenhum `.rs`, issue, commit ou push nasceu —
o primeiro `.rs` é de 2026-11-24, §6.6.27 —, `Q-BLK-19` seguia **aberta** naquele lote (respondida em §6.6.28; só a numeração foi deslocada) e
o teto de **144** páginas de `docs/` não foi tocado — os três blocos novos cabem nas páginas donas, sem
página nova.

**Evidência da entrega:** `docker compose --profile gate run --rm -T gate`, em 2026-11-24, terminou com
`verificação: 309 páginas, links, âncoras, navegação e sintaxe ok (307 fontes verificadas)` e exit **0** —
o total de páginas não mudou, porque nenhuma página nasceu.

**Decidida em A pelo dono em 2026-11-24; execução em (h).**

## 6.7 Prioridade consolidada — o que atacar agora (2026-11-24)

> **Histórico, não contador.** As quatro perguntas desta seção foram aprovadas em **A** e as quatro páginas
> existem (§6.6.19). Os números abaixo são os de antes daquela execução (139 páginas, 7 de 109 tabelas,
> 34 atos) e **não** valem hoje: a medição vigente é a de **§6.6.22.1** — 143 páginas em `docs/`, **15** de
> **109** tabelas com requisito e **142** atos de aceite; o censo de rastreabilidade vigente é **15 de 34**
> linhas com cenário (§6.6.24, recontado em §6.6.26 (h)). Onde este texto divergir, vale §6.6.22, §6.6.24 ou a página dona.

**Pergunta:** "das frentes abertas do projeto, qual atacar agora, e o que cada uma espera de quem?" Esta
seção **consolida e liga**; ela não substitui o censo de §1, os atos de §4, a fila de §5 nem os dossiês
de §6.6 — cada número abaixo nomeia o dono onde foi contado, e a pergunta de cada decisão continua em
§6.6.8 a §6.6.11. Ela **não decidiu nada**: as quatro perguntas de §6.7.3 foram respondidas em **A** pelo
dono (§6.6.19), e a ordem vigente depois delas é a do lote I (§6.6.22).

### 6.7.1 O foco, em uma frase

**Responder `Q-BLK-08`, `09`, `10` e `11` em A e escrever as quatro páginas que elas liberam** — porque
é a única frente que leva as páginas bloqueadas de **5 para 1** com **4 decisões** que não esperam
terceiro, e porque é o começo do único caminho que produz código neste projeto: página `aceita` (`M33`) →
série aplicada → `M20` → `planos/metricas.md`.

| O que a frente move | Quanto | Fonte |
|---|---:|---|
| páginas bloqueadas por decisão | **4** de **5** | §5; quadro-resumo de §6.6 |
| decisões do pacote `Q-BLK` ainda abertas | **4** (7 já aprovadas: `01`–`07`) | §6.6; respostas em §6.6.6, §6.6.7 e lotes em §6.6.14, §6.6.17 e §6.6.18 |
| página que continua bloqueada depois das quatro | **1** (`planos/metricas.md`, por `M20`) | §6.6.12 |
| atos de aceite que a etapa seguinte destrava | **34** páginas, 17–34 h | §4.1 |
| seeds que **não** são destravados por ela | **6** | §4.2 |
| linhas de execução que dependem da série | **7** | §4.3 |
| lacunas abertas que ela fecha | **0** — não há lacuna aberta no `wyd-db` | §1; `lacunas.md` §1: 0 abertas, 0 propostas, 35 fechadas, 3 externas |

**Por que vale mais que escrever outra coisa:** o `wyd-db` tem **139** páginas autorais escritas para um
teto de **144** (`proposta-paginas.md` §2), mas só **2** estão `aceito` — os dois ADRs. Escrever mais
páginas não move o limiar de "documentação 1.0.0" (§2), que exige as 144 escritas **e**
`aceito`/`retirado`; o que move é fechar as quatro que faltam por decisão e começar o aceite. E as quatro
são baratas: cada dossiê custa **0,5 a 1 sessão** (quadro-resumo de §6.6).

### 6.7.2 Ranking das frentes abertas, por poder de desbloqueio

| # | Frente | Itens abertos (contados) | O que destrava | Espera de quem | Natureza |
|---:|---|---|---|---|---|
| **1** | **Finalizar o `wyd-db`** | **21**: as **4** decisões de §6.6 + as **17** linhas de §4 (1 aceite com 34 páginas, 6 seeds, 7 execuções, 3 externas) | 4 páginas agora; depois `M33` → série → `M20` → a 144ª página | do dono (4 decisões; 34 atos de aceite), da curadoria com o cliente 7662 (6 seeds) e de `wyd-ops` (retenção) | **pré-requisito**: é o portão do código (item 1 do §5 de `AGENTS.md`) |
| **2** | **Definir regras e padrões** | **0** no `wyd-db` (`lacunas.md` §1: 0 abertas, 0 propostas) | nada novo: as regras que as outras frentes citam já estão decididas — `M33`/`Q-M33-6`, a forma de citação (`convencoes.md` §Coluna de dono) e os dois ADRs | ninguém | **cumprida** — o que resta é **aplicar** `M33`, já contado na frente 1 |
| **3** | **Expandir propostas dos outros sistemas** | **12** sistemas fora do `wyd-db` (§1.1): **1** sem plano diretor (`wyd-ops`), **6** "abertos" sem requisito numerado (`wyd-net`, `wyd-core`, `wyd-server`, `wyd-sim`, `wyd-launcher`, `wyd-bots`) e `wyd-protocol` com **5 de 131** páginas do teto | nada no `wyd-db`: contrato C4, DDL e papéis já fechados | de cada dono de sistema; do `wyd-db`, só leitura | **paralela** — não trava nem é travada |
| **4** | **Fechar lacunas abertas** | **5** no projeto: `wyd-protocol` **2** (`LPRT-03`, `LPRT-06`), `wyd-net` **1**, `wyd-bots` **1** e `wyd-client` **1** (`LCLI-02`) — §1.1 e `AGENTS.md` §6 | cinco lacunas de **medição/captura**, não de decisão | da captura A1 (frente `wyd-client`) e da primeira execução | **paralela**, com prazo de terceiro |

**O que isso quer dizer na prática:** só a frente 1 é pré-requisito de alguma coisa. A frente 2 já foi
cumprida e não precisa de sessão. As frentes 3 e 4 andam em paralelo sem destravar nada do `wyd-db` — e,
importante, **o `wyd-db` também não as trava**: o DDL de `ddl-confirmado.md`, o contrato C4 — decidido em
2026-11-23 — e os papéis de `seguranca.md` são a dependência real de `wyd-server`, `wyd-portal` e
`wyd-admin`, e ela está satisfeita.

### 6.7.3 As quatro perguntas — abertas, com recomendação e consequência

**1. `Q-BLK-08` — a matriz de rastreabilidade nasce com a coluna de teste como alvo planejado?**

- **Recomendação do dossiê: A** (§6.6.8).
- **Se A:** a matriz nasce com as **5** colunas do molde e **31** linhas (16 `RF` + 7 `RN` + 5 `RNF` + 3 `UC`);
  `teste` fica como alvo `P` (contrato × sistema) e `issue` como "a alocar na onda *N*"; a página expõe dois
  vazios que hoje ninguém vê — **7 de 109** tabelas sob requisito nomeado e **0 de 40** issues com vínculo.
- **Custo de B/C:** B espera a suíte das ondas 4–6 e **não** resolve a coluna `issue`, que não depende de
  teste nenhum; C cai de 5 para 2 colunas e deixa de atender ao molde e ao critério de §11 do plano.
- **Depende de:** **só do dono** — é escolha de escopo, nada no repositório a deriva.

**2. `Q-BLK-09` — a matriz de atores tem cinco linhas?**

- **Recomendação do dossiê: A** (§6.6.10).
- **Se A:** **5** atores = **4** credenciais de produto (`GAME`, `PORTAL`, `ADMIN`, `READ`) ∪ **3** valores de
  `person.role` (`member`, `moderator`, `admin`), porque staff e administrador compartilham a credencial e se
  distinguem pelo `role`; o jogador entra sem credencial própria (quem conecta é o servidor de jogo).
- **Custo de B/C:** B (só os 3 `role`) apaga a distinção `moderator`/`admin` do DDL e deixa `WYD_DB_URL_READ`
  sem responsável; C (os 6 do C4) põe runner e provisionamento na matriz, e a matriz passaria a afirmar que
  eles atendem requisito de jogador.
- **Depende de:** **só do dono** — a derivação numérica existe, a decisão de recorte é dele.

**3. `Q-BLK-10` — a topologia de implantação é um host com um container?**

- **Recomendação do dossiê: A** (§6.6.9).
- **Se A:** **1** host (VPS paga, `Q-OPS-2` = A), PostgreSQL 17 em **1** container com **1** volume nomeado,
  **5432** fora da rede interna não publicada, runner e `provision` como one-shot, supervisor = o
  `restart: unless-stopped` que o Compose já usa, as **6** DSNs inalteradas; e o C4 ganha o nível 4, que hoje
  não existe (o `contexto-c4.md` para no nível 2).
- **Custo de B/C:** **B já está fora por decisão tomada** — DBaaS contraria `Q-OPS-2` = A e `Q-BLK-03`
  (backup no storage do mesmo provedor da VPS, 2026-11-24); C (vários hosts, um container por canal) não tem
  medição de carga: o orçamento de **53 de 97** conexões cabe em um host.
- **Depende de:** **parcialmente derivável** — B está excluído pelo que já foi decidido; resta A contra C, e C
  não tem número que o sustente.

**4. `Q-BLK-11` — o modelo conceitual agrupa pelos 19 domínios do dicionário?**

- **Recomendação do dossiê: A** (§6.6.11).
- **Se A:** **19** blocos de domínio (de **1** a **22** tabelas, somando **109**) mais os **5** agregados que o
  molde nomeia (conta, personagem, clã, conteúdo, operação), citando `fronteira-hot-cold.md` como dono da
  fronteira de escrita.
- **Custo de B/C:** B (5 fronteiras de escrita) cria segundo dono para a mesma regra; C (ER plano de 109
  nós) não responde "quais entidades cuidam deste assunto" e perde a noção de tamanho — cinco domínios têm
  3 ou menos tabelas e um tem 22.
- **Depende de:** **derivável do molde** — `wyd-db-1.0.0.md` §6.3 já descreve a página exatamente assim; a
  aprovação formal continua sendo do dono.

### 6.7.4 O caminho curto depois de cada resposta

| Resposta | Próximo passo, já definido | Arquivo | Efeito no censo |
|---|---|---|---|
| `Q-BLK-08` = A | escrever a matriz com as 5 colunas e as 31 linhas | `docs/specs/rastreabilidade.md` | escritas 139 → 140; bloqueadas 5 → 4 |
| `Q-BLK-09` = A | escrever as 5 linhas de ator e quem aprova mudança | `docs/specs/stakeholders.md` | 140 → 141; 4 → 3 |
| `Q-BLK-10` = A | escrever a topologia com container, volume, porta e supervisor | `docs/arquitetura/implantacao.md` | 141 → 142; 3 → 2 |
| `Q-BLK-11` = A | escrever os 19 blocos e o recorte dos 5 agregados | `docs/arquitetura/modelo-conceitual.md` | 142 → 143; 2 → **1** |
| as quatro em A | abrir `M33` (34 atos) → série aplicada → `M20` | `planos/metricas.md` fecho | **144/144** escritas; 1 bloqueada cai |

### 6.7.5 O que esta seção não faz

- **Não decide.** As quatro perguntas de §6.7.3 foram respondidas em **A** em 2026-11-24 (§6.6.19); as do
  lote G (§6.6.20) são do dono e seguem abertas. Nada aqui marca página como `aceita`.
- **Não reabre o censo de citações** nem toca nos números de §6.6.16 — eles ficam como estão.
- **Não substitui os dossiês:** §6.6.8 a §6.6.11 seguem donos da evidência de cada pergunta; esta seção só
  as ordena e liga ao próximo passo.
- **Não conta página escrita como aceita** nem soma os 34 atos de `M33` como feitos.
- **Não cria lacuna:** o vazio de **102** tabelas e o de **31** linhas de issue são produto da matriz, e quem
  os registra, se o dono quiser, é `lacunas.md`, dono único.

## 7. O que este registro não faz

- Não responde decisão do dono em nome dele; registra as respostas reais.
- Não aceita página, não escreve página bloqueada e não cria índice ou job.
- Não altera as 38 lacunas, as 3 externas, a distribuição do `wyd-client` nem `LCLI-02`/`LCLI-05`.
- Não usa o inventário da árvore do cliente como lista de patch.
- Não trata as perguntas de §6.6 como respondidas, nem conta página retirada antes da decisão de teto.
- Não trata o pedido ao titular como enviado.

### 6.6.27 Aceite das 37 páginas de `src/` — `Q-BLK-18` = A (2026-11-24)

**Decisão do dono:** `Q-BLK-18` = **A** — página de teste `aceita` libera o código do alvo e a issue registra
o vínculo depois (§6.6.22.5). Com a resposta, a instrução foi seguir para o aceite das **37** páginas de
`src/` e só depois abrir o primeiro `Cargo.toml`/`.rs` do `wyd-db`.

**Método da revisão** (as 37, antes da assinatura): cabeçalho de 5 linhas; linha de estado; legenda
`P`/`H`/`V`; `Base:` preenchido; links relativos e caminhos inline resolvendo; nenhum `TODO`/placeholder;
nenhum segredo; números com fonte; e coerência com as decisões fechadas (`Q-BLK-24`, série de 34 arquivos).
Checagem mecânica por `grep`/laço nos 37 arquivos e **leitura integral das 37** (11 primeiro, 26 na
passada de fechamento).

| Achado | Onde | Correção aplicada |
|---|---|---|
| série descrita como `0001`…`0029` | `migrate/spec.md` §2 | `0001`…`0034`, **34** arquivos (`plano-migrations.md` §3) |
| série `0001`…`0029` em página publicada | `docs/planos/plano-de-implantacao.md` §1 | `0001`…`0034` |
| "31 → 32 na série" sem data | `provision/spec.md` §2 | "naquele momento; hoje a série fechou em **34**" |
| referência a `docs/rust/05-stack-docker.md`, que não existe | `provision/spec.md` §9 | `docs/planos/plano-de-implantacao.md` §1 (dono de quem conecta com cada papel) |
| referência a `pool-config.md`/`src/pool/config.rs`, que não existem | `dsn.md` §4 | `docs/guia-desenvolvedor/pool-e-transacoes.md` (dono dos valores do pool) |
| dono do `stem` contradizia `LoadedFile` (sem campo de nome) e `runner.md` §3 | `migrate/port/source.md` §2 | o `stem` vem de `path.file_stem()` no **runner**, não no `adapter/fs` |
| passo 2 chamava `MigrationSet::build(files)` com `Vec<LoadedFile>`, mas a assinatura é `Vec<(String, String)>` | `migrate/app/runner.md` §3 | `build(pairs(files))`, com a conversão nomeada na tabela |
| dono do `ChecksumError` apontado para `port::ledger` | `migrate/migration/set.md` §2 | `migration::checksum` (o ledger não define hash) |
| `Report::Planned(Plan)` e os bounds `Catalog`/`Script` sem `use`/reexport no bloco | `provision/mod.md` §2 | `pub use domain::plan::{Drift, Plan, Step};` + `use port::{Catalog, Script}` |

**Sem defeito:** as outras 28 não tinham contradição interna, número com fonte errada, link quebrado,
`TODO` ou segredo; todas têm `Base:`, legenda e as 5 linhas de cabeçalho.

**Efeito no censo:** as **37** estão `aceito (2026-11-24)` e `src/` tem **0** `proposto`. Os atos de aceite
pendentes caíram de **205** para **168** (141 `docs/` + **0** `src/` + 27 `migrations/` + **0** `tests/`) e as
páginas aceitas do sistema subiram de **15** para **52** (2 ADRs + `0002` + 12 `tests/` + 37 `src/`).

**Primeiro código, na mesma data (o aceite é o que o libera):** `crates/wyd-db/Cargo.toml`
(`name = "wyd-db"`, `version = "0.0.0"`, `edition = "2021"`, `publish = false`, sem dependências),
`crates/wyd-db/src/lib.rs` (`pub mod dsn;`) e `crates/wyd-db/src/dsn.rs` com os 6 testes de §5 da página
(`cargo test`: **6 passed, 0 failed**). `Cargo.lock` foi gerado sem dependência nenhuma.

**O que este aceite não faz:** não promete implementação além de `dsn`. Nenhum outro `.rs` nasceu, nenhum
teste rodou contra PG, nenhum `[[test]]` foi declarado, nenhuma issue foi criada e nada foi commitado.

### 6.6.28 `Q-BLK-19` = A — os seis `RNF` de gate e a dívida zerada (2026-11-24)

**Decisão do dono:** `Q-BLK-19` = **A** — criar `RNF-DB-008` a `RNF-DB-013`, **um por gate** que governa
capacidade, e repontar as 11 capacidades da matriz para eles.

**O que foi escrito:** seis blocos em `docs/specs/requisitos-nao-funcionais.md` §8–§13, na ordem do gate
(`estrutura`, `constraints`, `tipos`, `catálogo`, `fronteira`, `ambiente`). Cada bloco usa os cinco campos da
página (métrica, limiar, como medir, reprova, fonte) e o limiar é o **do gate** — `0` violação ou `100 %` —,
não número novo. O dono do número continua `plano-de-qualidade-e-gates.md` §1 (e, para as contagens, o
plano que as mede).

| `RNF` | Gate | Governa |
|---|---|---|
| `RNF-DB-008` | `estrutura` | `CAP-01`–`CAP-05`, `CAP-09` |
| `RNF-DB-009` | `constraints` | `CAP-06` |
| `RNF-DB-010` | `tipos` | `CAP-07` |
| `RNF-DB-011` | `catálogo` | `CAP-08` (sem ambiente; declarada não executável com `Q-BLK-20` = A, §6.6.31) |
| `RNF-DB-012` | `fronteira` | `CAP-20` |
| `RNF-DB-013` | `ambiente` | `CAP-22` |

**O que mudou nos censos, remedido nesta passada:** rastreabilidade **34 → 40** linhas (17 `RF` + 7 `RN` +
**13** `RNF` + 3 `UC`); células pendentes **65 → 77** = **37** `Teste` + **40** `Issue` (as **3** de `Teste`
que são `—` por regra continuam fora da conta); matriz `CAP-nn` **12 com requisito + 11 por gate → 23 com
requisito + 0 por gate** e a dívida visível **11 → 0**; sem página na rastreabilidade **19 → 25** (os 6
novos são gate de série, não cenário). `Q-BLK` abertas **4 → 3** (`20`, `22`, `23`) e F6 = **9**
(3 `Q-BLK` + 2 de foco + 4 de seed).

**O que a decisão não faz:** não escreve cenário novo (as **16** linhas sem cenário seguiam iguais naquele
momento), não destrava `CAP-08` (o catálogo segue ausente; `CAP-08` declarada não executável em §6.6.31), não torna nenhum gate
executável — `tipos`, `catálogo` e `ambiente` seguem conferência manual declarada — e não muda limiar de
`RNF-DB-001` a `007`.

> **Superado em 2026-11-24, no mesmo dia:** `Q-BLK-23` = A escreveu os **16** cenários (§6.6.29) e a
> rastreabilidade passou a **31 de 40** linhas com página.

### 6.6.29 `Q-BLK-23` = A — os 16 cenários antes da leva de `migrations/` (2026-11-24)

**Decisão do dono:** `Q-BLK-23` = **A** — escrever os **16** cenários que faltavam, uma `CAP-nn` por linha,
**antes** da leva de `migrations/`.

> **Superado em parte em 2026-11-24 (§6.6.30):** a leva de `migrations/` foi entregue e **13** dos 16 cenários
> rodaram verdes contra a base migrada; a cobertura passou de **0** para **13 de 39** e as células pendentes de
> **77** para **64**. O resto deste dossiê — o que foi escrito, `Q-BLK` abertas e o que a decisão não faz —
> permanece como registro da passada que o escreveu.

**O que foi escrito:** **16** capacidades novas em `tests/specs/capacidades.md` §4 — `CAP-24` a `CAP-39`, uma
por linha que tinha critério de aceitação e nenhum cenário — e **16** cenários em `tests/alvos/sistema-pg.md`
§1–§2, no alvo que já executa as capacidades de banco. Cada capacidade cita o requisito da própria linha, e
nenhum limiar novo foi inventado.

| `CAP` | Cenário | Requisito | O que o cenário prova |
|---|---|---|---|
| `CAP-24` | `login_tamanho_e_fold` | `RF-DB-001` | `23514` em `ck_account_login_len`; `login_fold` dobra caixa e borda |
| `CAP-25` | `pin_falha_persistida` | `RF-DB-002` | contador e instante na linha, lidos em conexão nova |
| `CAP-26` | `cofre_da_conta` | `RF-DB-004` | `23503` em `fk_vault_item_account`; apagar personagem não apaga o cofre |
| `CAP-27` | `bloqueio_com_motivo` | `RF-DB-007` | três `23514`, um por `CHECK` de moderação |
| `CAP-28` | `donate_idempotente` | `RF-DB-008` | `23505` em `uq_donate_credit_idem`; valor positivo |
| `CAP-29` | `token_como_hash` | `RF-DB-010` | `23514` em `ck_person_token_hash_forma`/`ck_pin_reset_token_forma` |
| `CAP-30` | `papel_de_jogo_fechado` | `RF-DB-012` | `23514` em `ck_account_role` |
| `CAP-31` | `hash_argon2id` | `RF-DB-013` | `23514` em `ck_person_senha_hasheada`/`ck_account_senha_hasheada` |
| `CAP-32` | `bloqueio_nao_muda_status` | `RN-DB-006` | `status` continua `active`; `suspended` recusado |
| `CAP-33` | `limiar_de_pin_e_politica` | `RN-DB-007` | contador negativo recusado; seis falhas passam |
| `CAP-34` | `autenticar_portal` | `UC-DB-001` | duas sessões revogáveis; tentativa em `ops.login_attempt` |
| `CAP-35` | `criar_conta_na_vaga` | `UC-DB-002` | conta e carteira; sexta recusada em `ux_account_slot` |
| `CAP-36` | `abrir_sessao_de_jogo` | `UC-DB-003` | `character_id` nulo aceito; canal inexistente recusado |
| `CAP-37` | `espera_de_ddl` | `RNF-DB-002` | espera de até **10 s** e erro nomeado |
| `CAP-38` | `p95_leitura_personagem` | `RNF-DB-004` | p95 registrado, sem limiar |
| `CAP-39` | `tempo_de_boot` | `RNF-DB-005` | tempo registrado, sem prazo |

**O que mudou nos censos, remedido nesta passada:** capacidades **23 → 39**, todas com requisito nomeado e
alvo escrito; cobertas por teste executado continuam **0 de 39**; linhas da rastreabilidade com página que
nomeia o requisito **15 → 31** das 40; sem página **25 → 9** (2 de `wyd-server`, 1 de gate de CI, 6 de gate de
série) e sem cenário **16 → 0**; células pendentes continuam **77** (37 `Teste` + 40 `Issue`), porque cenário
escrito não é teste executado; `Q-BLK` abertas **3 → 2** (`20`, `22`) e F6 = **8**.

**O que a decisão não faz:** não escreve teste (a coluna `Teste` segue `P` e a matriz segue **0 de 39**
cobertas), não cria alvo novo — os 16 cenários entram no `sistema_pg` escrito —, não fixa limiar de latência
nem de boot (`CAP-38` e `CAP-39` medem e registram), não destrava `CAP-08` nem respondia `Q-BLK-20` (respondida depois em §6.6.31) e não altera o teto
de 144 (`Q-BLK-22` = A).

**O que fica dito e não provado:** `CAP-25` e `CAP-33` provam o **estado** de PIN, não o limiar de cinco
falhas em dez minutos, que é do código (`RN-DB-007`); declarar o limiar provado no banco seria mentira.

### 6.6.30 Leva de `migrations/` — 28 arquivos aplicados e 13 cenários verdes (2026-11-24)

**O que foi entregue:** os **28** arquivos de `migrations/` da série (as 28 páginas de DDL, `0001`–`0034`, sem
os 6 bloqueados por seed), extraídos do bloco `up` de cada página, e o alvo `crates/wyd-db/tests/sistema_pg.rs`
com os **13** cenários que rodam contra a base migrada.

**Ambiente (pré-requisito, não parte da série):** container `postgres:17` vazio, banco `wyd`; os seis papéis de
cluster criados à mão (`wyd_owner`, `wyd_migrator`, `wyd_game`, `wyd_portal`, `wyd_admin`, `wyd_read`, mais
`GRANT wyd_owner TO wyd_migrator` e `GRANT CREATE, CONNECT ON DATABASE wyd TO wyd_migrator`). O `provision`
(`src/provision/spec.md`) ainda não existe: criar o ambiente foi ato do operador, e é essa a lacuna de
`RNF-DB-013` que o runner fecha.

**Aplicação do zero:** os 28 arquivos, em ordem, como `wyd_migrator`, com `ON_ERROR_STOP=1` — **28 ok, 0 erro**.
Estado final medido: **4** schemas (`content`, `game`, `portal`, `ops`), **109** tabelas, **2** views, **245**
índices, **655** constraints, **3** funções e **46** triggers.

**Reaplicar não duplica:** segunda passada, 27 dos 28 arquivos abortam com DDL já existente (`0001`: `schema
"content" already exists`; `0002`: `relation "audit_log" already exists`; …; `0034`: `relation "market_wallet"
already exists`) e `0032` (grants) reaplica sem efeito. Censo antes e depois idêntico, linha por linha
(`diff` vazio): **0** objeto novo, **0** duplicata. O DDL não é `IF NOT EXISTS` por desenho — quem impede a
segunda aplicação é o ledger `schema_migrations`, que o runner ainda não tem (`RNF-DB-008`).

**Objetos que os cenários nomeiam — todos na série:**

| Objeto | Arquivo:linha | Objeto | Arquivo:linha |
|---|---|---|---|
| `ck_account_login_len` | `0005:25` | `uq_donate_credit_idem` | `0011:168` |
| `ck_account_login_forma` | `0005:26` | `ck_donate_credit_valor` | `0011:169` |
| `ck_account_role` | `0005:27` | `ck_person_token_hash_forma` | `0004:58` |
| `ck_account_status` | `0005:28` | `ck_pin_reset_token_forma` | `0005:108` |
| `ck_account_senha_hasheada` | `0005:29` | `ck_login_attempt_result` | `0002:41` |
| `ck_account_pin_falhas` | `0005:31` | `portal.person_session` | `0004:5` |
| `login_fold` (coluna gerada) | `0005` | `game.account_wallet` | `0005:46` |
| `ux_account_slot` / `ux_account_login` | `0005:39` / `0005:38` | `fk_session_canal` | `0007:48` |
| `pin_failure_count` / `pin_locked_until` | `0005:11` / `0005:12` | `ux_session_viva_por_conta` | `0007:59` |
| `fk_vault_item_account` | `0009:46` | `ck_server_channel_jogavel` | `0007:19` |
| `ck_account_moderation_motivo/autor/instante` | `0005:84/90/87` | `ck_person_senha_hasheada` | `0003:29` |

Nenhum requisito de `CAP-24`–`CAP-36` ficou sem objeto. Os que ficaram sem código são os três `RNF` abaixo.

**Execução, um por um:** `cargo test -p wyd-db --test sistema_pg -- --test-threads=1` com
`WYD_DB_TEST_CONTAINER` — **13 passed; 0 failed; 0 ignored**. Na primeira passada `CAP-36` falhou porque a
fixture do canal violava `ck_server_channel_jogavel` (canal > 0 exige `max_players`, `maintenance` e `role`);
o defeito era do cenário, foi corrigido nele e passou na segunda. `CAP-24` também precisou do caso certo: o DDL
recusa `' Bob '` em `ck_account_login_forma` antes de qualquer fold, e o cenário passou a provar as duas coisas
— borda recusada e caixa dobrada com `'Bob'` → `'bob'`.

| Resultado | Quais |
|---|---|
| **verde** | `CAP-24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36` |
| **sem código que execute** | `CAP-37` (`RNF-DB-002`: `MigrateConfig`/runner), `CAP-38` (`RNF-DB-004`: consulta da porta C4), `CAP-39` (`RNF-DB-005`: boot do binário) |

**O que mudou nos censos, medido nesta passada:** cobertas por teste executado **0 → 13 de 39**; células
pendentes da rastreabilidade **77 → 64** (**24** `Teste` em `P` + **40** `Issue`; **13** `Teste` em `V`, **3**
`—`); linhas com página **31**, sem página **9**, sem cenário **0** (inalterados); alvos no `Cargo.toml`
**0 → 1 de 5**; `.sql` da série **0 → 28**; `Q-BLK` abertas **0** (as duas foram respondidas em §6.6.31; naquela passada
eram **2**); F6 = **4**; atos de aceite **141** (as 27 páginas de `migrations/` foram aceitas em
2026-11-24, §6.6.31).

**O que este dossiê não faz:** não aceita página de `migrations/` (é ato do dono; as 27 seriam aceitas depois,
§6.6.31), não
implementa o runner (`RNF-DB-008`, `002`), o pool (`RNF-DB-007`), a porta C4 (`RNF-DB-004`) nem o boot
(`RNF-DB-005`), não decide o cliente `sqlx`, não cria as 6 seeds bloqueadas e não escreve as 40 issues. O
harness chama `psql` no container: ele prova o **schema**, não o crate.

### 6.6.31 Lote M — D-01/D-03 aprovados em A: 27 páginas de `migrations/` aceitas (2026-11-24)

**O que o dono decidiu:** `D-01` = **A** (foco: finalizar o `wyd-db`) e `D-03` = **A** (aceitar as **27**
páginas de `migrations/` agora), com `Q-BLK-20` = **A** e `Q-BLK-22` = **A**. O registro canônico das duas
decisões é `../wyd-ops/docs/prioridade-do-projeto.md` §0.1; esta seção registra o ato e a medição.

**O que foi executado:** as **27** páginas escritas de `migrations/` passaram a
`**Estado:** aceito (2026-11-24)` — com `0002-ops-audit.md`, que já estava `aceito (2026-11-20)`, a pasta
fica com **28 de 28**. A assinatura é o ato do dono (`Q-FOCO-4`); a revisão de quem não escreveu é o item 9 de §10 de
`wyd-db-1.0.0.md`. Antes de assinar, as **16** páginas que ainda diziam “nada disto rodou” ou “prova
prevista (não executada)” foram corrigidas para o fato medido: o `.sql` existe, a série aplicou do zero em
PostgreSQL 17 em 2026-11-24 (§6.6.30) e as provas de `psql` de cada página não foram executadas uma a uma.

**Efeito nos censos, medido no gate:**

| Medida | Antes | Depois | Fonte |
|---|---:|---:|---|
| atos pendentes, antes → depois | **168** | **141** (141 `docs/` + 0 nas outras três pastas) | `verificar-censos.py` |
| páginas `aceito` no `wyd-db` | **52** | **79** (2 `docs/` + 28 `migrations/` + 37 `src/` + 12 `tests/`) | `verificar-censos.py` |
| `Q-BLK` abertas | **2** (`20`, `22`) | **0** | §6.6.22.5 e §6.6.23 |
| capacidade sem ambiente | **1** (`CAP-08`) | **1**, agora **declarada não executável na `1.0.0`** | `tests/specs/capacidades.md` §9 |

**O que `Q-BLK-20` = A e `Q-BLK-22` = A passam a valer:** o dicionário `banco-catalogo.md` fica
**pós-`1.0.0`**; `CAP-08` continua **0 coberta** e é declarada **não executável na `1.0.0`**, com o gate
`catálogo` como conferência manual até o PR de ferramentas; e as **12** páginas de `tests/` **não contam**
no critério de release de **144**, que segue inalterado.

**O que este lote não faz:** não escreve `migrations/serie-e-regras.md`, não implementa o runner
(`RNF-DB-008`, `002`), o pool (`RNF-DB-007`), a porta C4 (`RNF-DB-004`) nem o boot (`RNF-DB-005`), não
cria as **6** seeds bloqueadas, não abre as **40** issues e não aceita as **141** páginas de `docs/`.
Nenhum `.sql` novo nasceu: os **28** da série continuam os mesmos.

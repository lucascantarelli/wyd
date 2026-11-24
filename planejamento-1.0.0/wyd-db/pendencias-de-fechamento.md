# Pendências de fechamento do `wyd-db`

> Pergunta: "o que ainda falta para o planejamento do `wyd-db` ser considerado fechado, quem responde por cada item e o que cada item desbloqueia?"
> Público: dono, arquitetura, operação, jurídico, curadoria de conteúdo e equipe do `wyd-db`
> Dono: `wyd-db` mantém o registro; cada linha nomeia o dono da decisão
> Base: `lacunas.md`; `proposta-paginas.md` §4–§8; `migrations-checklist.md` §2–§3; `proposta-decisoes-m33.md`; `proposta-retencao-pendente.md`; `proposta-decisoes-capacidade.md`; `plano-migrations.md` §4 e §6; `ddl-confirmado.md` §5.9
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-23). Este é um **registro de fechamento**, não uma nova lacuna e não um
plano de execução. Ele reúne o que as páginas donas já declaram, com o identificador de cada item, e
separa três coisas que estavam misturadas na fila: decisão do dono, ato de aceite e trabalho de redação
que ainda depende de medição ou de fonte externa.

O registro fica na raiz de `planejamento-1.0.0/wyd-db/`, ao lado de `lacunas.md` e
`proposta-paginas.md`, e **não entra no censo de 147 páginas autorais de `docs/`**. Ele não cria
lacuna, não renumera ID e não altera as contagens de `lacunas.md`.

## 1. Onde a frente está (medido em 2026-11-23)

| Medida | Hoje | Fonte |
|---|---|---|
| lacunas do `wyd-db` | **38**: 0 abertas, 0 propostas, 35 fechadas, 3 externas (`A11`, `A20`, `M21`) | `lacunas.md` §1 |
| páginas autorais de `docs/` | **147**: 121 com corpo, 0 escrevíveis, 0 de recorte, **26 bloqueadas** | `proposta-paginas.md` §2–§6 |
| migrations por arquivo | **34**: 28 com página, **6 seeds bloqueados** | `migrations-checklist.md` §2 e §4 |
| páginas de seção de `migrations/` | `index.md` escrito; `serie-e-regras.md` **não escrito** | `migrations/index.md`; `wyd-db-1.0.0.md` §11 |
| aceite de página de migration | **1 de 28 escritas** (`0002-ops-audit.md`); 34 atos previstos | `proposta-decisoes-m33.md` §2 e §4 |
| decisões de retenção | **7 propostas, 7 resolvidas**, **0 abertas** (`Q-RET-5` por evidência; `Q-RET-1`, `Q-RET-2` e `Q-RET-4` pelo dono em 2026-11-23; `Q-RET-6` e `Q-RET-7` pelo dono em 2026-11-24; `Q-RET-3` pelo dono em 2026-11-24, opção A) | `proposta-retencao-pendente.md` §4 e §9 |
| decisões de capacidade | **0 abertas**; `Q-CAP-1`, `Q-CAP-2` e `Q-CAP-3` decididas em 2026-11-23; a matriz de `Q-CAP-2` está aplicada na página | `proposta-decisoes-capacidade.md` §4 e §9 |
| páginas de migration bloqueadas por dado | **6**: `0016`–`0018`, `0020`, `0023`, `0027` | `migrations-checklist.md` §3 |

**O que não mudou:** as 38 lacunas continuam 0/0/35/3; `M33` continua fechada como decisão e
`M32` continua fechada; `A11`, `A20` e `M21` continuam `externa`. Este registro não reabre nenhuma
delas.

**Fontes ausentes que não podem determinar prazo:** `migrations/serie-e-regras.md` e `planos/metricas.md`
não existem no worktree em 2026-11-23; são páginas pendentes, não fonte de decisão. `migrations/index.md`
existe e não fixa retenção. Os seis seeds estão nas migrations, mas não carregam prazo de retenção.

## 2. O que significa "fechar" nesta frente

Há dois limiares diferentes, e a fila não deve confundi-los:

| Limiar | Critério verificável | O que ainda falta |
|---|---|---|
| **Decisão da frente** | nenhuma pergunta de retenção/capacidade sem resposta; nenhuma categoria `pendente` sem dono; nenhuma pendência do `wyd-db` dependendo de suposição | **0 `Q-RET` abertas**; 3 lacunas externas permanecem externas; `Q-DP-2/validação` continua pendente como validação separada |
| **Documentação 1.0.0** | as 147 páginas de `docs/` escritas e `aceito`/`retirado`; 34 páginas de migration + `serie-e-regras`; `planos/metricas.md` medido; todos os links e gates verdes | 26 páginas bloqueadas; 6 seeds; `metricas.md`; `serie-e-regras.md`; aceite página a página |

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
para dado real; **nenhuma das 26 páginas bloqueadas** tem nela seu bloqueio exclusivo. As páginas
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
| `SERIE-REGRAS` | escrever `migrations/serie-e-regras.md` | `wyd-db` | 1 sessão | nenhuma decisão nova; a fonte é `plano-migrations.md` §2/§6 e `src/migrate/spec.md` | a 5ª onda continua aberta; a política da série não tem um único dono publicado | 5ª onda e operação da série |
| `Q-RET-1/aplicação` | criar o índice completo por `expires_at` e o `DELETE` diário de `portal.person_token` | `wyd-db` + `wyd-ops` | 2–4 h de índice/migration + 1 sessão de job | outra frente: `wyd-ops`; sem jurídico | a decisão existe, mas a tabela não está pronta para produção nem tem expurgo | produção de `person_token`, expurgo diário e a linha de índice |
| `Q-RET-2/aplicação` | criar o `DELETE` diário de `ops.login_attempt` usando `ix_login_attempt_occurred` | `wyd-ops` | 1 sessão de job | outra frente: `wyd-ops`; sem jurídico e sem índice novo | a decisão existe, mas a tabela continua sem expurgo e não vai a produção | produção de `login_attempt` e expurgo diário |
| `Q-RET-4/aplicação` | criar o índice por `expires_at` e o `DELETE` diário de `game.pin_reset` | `wyd-db` + `wyd-ops` | 2–4 h de índice/migration + 1 sessão de job | outra frente: `wyd-ops`; sem jurídico; TTL ainda é `B1` | a decisão existe, mas `pin_reset` não vai a produção e pedidos nunca usados crescem | produção e expurgo de `pin_reset` |
| `Q-RET-6/aplicação` | criar o índice por `read_at` e o `DELETE` diário de `market_notification`; preservar as demais tabelas conforme A | `wyd-db` + `wyd-ops` | 2–4 h de índice/migration + 1 sessão de job | preservação legal do mercado e execução por `wyd-ops` | a política existe, mas a notificação lida continua sem índice e sem expurgo | aplicação da política de 11 tabelas; índice e job de notificação |
| `Q-RET-7/aplicação` | política de 30 dias, rotação, criptografia fora do host e restore trimestral; escrever `backup-e-restore` | `wyd-ops` + jurídico | 1 sessão + 1–2 sessões de operação | fornecedor/contrato de backup, RPO/RTO, criptografia e ensaio de restore; jurídico da cópia | a política existe, mas não há rotina, fornecedor, RPO/RTO nem restauração provada | política de backup e a página `backup-e-restore` |

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

## 5. As 26 páginas de `docs/` bloqueadas

Esta é a fila de `proposta-paginas.md` §6, reproduzida com o dono e o bloqueio de cada página. Ela
**não** é uma lista de patch e não autoriza escrever por dedução: cada linha só sai quando a condição
declarada existir. A soma agora é **26**: `arquitetura/retencao-e-capacidade.md` saiu em 2026-11-23,
como `proposto` com volume pendente (`Q-CAP-1`).

| ID (caminho) | Bloqueio | Dono / marco | Desbloqueia |
|---|---|---|---|
| `arquitetura/implantacao.md` | desenho de processo, porta e volume | arquitetura, depois da série | Onda 2/6; implantação |
| `arquitetura/fluxo-de-dados.md` | mapa das tabelas restantes | arquitetura, depois da série | Onda 2; fluxo entre sistemas |
| `arquitetura/modelo-conceitual.md` | agrupamento conceitual das 109 tabelas | arquitetura | Onda 2; modelo conceitual |
| `specs/requisitos-nao-funcionais.md` | nenhum limiar medido | arquitetura, ondas 3/7 | Onda 3; RNF |
| `specs/rastreabilidade.md` | matriz sem porta, teste e issue | arquitetura, ondas 3/7/8 | Onda 3/8; rastreabilidade |
| `specs/fluxos/criacao-e-selecao-de-personagem.md` | texto do fluxo; DDL de `0006` fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; fluxo de personagem |
| `specs/fluxos/carregamento-e-save.md` | texto do fluxo; DDL fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; carga e save |
| `specs/fluxos/concessao-de-grant.md` | texto do fluxo; DDL fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; grants |
| `specs/fluxos/compra-e-credito.md` | texto do fluxo; DDL fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; compra e crédito |
| `specs/estados/personagem.md` | texto do estado; DDL fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; estados |
| `specs/estados/grant.md` | texto do estado; DDL fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; estados |
| `specs/estados/item-e-refino.md` | texto do estado; DDL fechado | produto + arquitetura (`P1`/`P3`) | Onda 3; estados |
| `specs/stakeholders.md` | matriz ator → requisito dos demais domínios | produto, onda 3 | Onda 3; atores |
| `planos/plano-de-manutencao.md` | rotina que executa a retenção; `M22` decidida, falta o job | operação (`P3`) | Onda 6; operação |
| `planos/metricas.md` | número medido; `M20` só depois da série | arquitetura (`M20`, `P2`) | Onda 6; fechamento da documentação |
| `guia-administrador/backup-e-restore.md` | política decidida (`Q-RET-7 = A`), mas rotina, fornecedor e RPO/RTO pendentes | operação | Onda 6; backup/restore |
| `guia-administrador/manutencao-rotineira.md` | rotina e inventário organizacional | operação | Onda 6; manutenção |
| `guia-administrador/recuperacao-de-desastre.md` | política de backup/DR e RPO/RTO | operação | Onda 6; recuperação |
| `guia-administrador/sustentacao.md` | inventário organizacional e rotina | operação | Onda 6; sustentação |
| `guia-teste/fixtures-e-factories.md` | desenho da suíte ausente | QA + arquitetura | Onda 7; fixtures |
| `guia-teste/dados-de-teste.md` | catálogo de contas de teste não existe; `M19` fechada | QA + arquitetura | Onda 7; dados de teste |
| `guia-desenvolvedor/pool-e-transacoes.md` | pool, timeout e saturação sem desenho/medição | arquitetura, onda 7 | Onda 7; pool |
| `guia-documentacao/markdown-e-mkdocs.md` | config de mkdocs ainda não existe | arquitetura + `wyd-docs` | Onda 9; publicação |
| `guia-documentacao/diagramas.md` | duplicaria `convencoes.md` §Diagramas | arquitetura + `wyd-docs` | Onda 9; diagramas |
| `guia-documentacao/links-referencias-e-ids.md` | duplicaria `convencoes.md` §Links e citações | arquitetura + `wyd-docs` | Onda 9; links |
| `guia-documentacao/imagens-e-evidencias.md` | duplicaria `convencoes.md`; build é do `wyd-docs` | arquitetura + `wyd-docs` | Onda 9; imagens e evidências |

**Fora desta tabela, mas ainda pendente:**

- `migrations/serie-e-regras.md` — 1 página de seção, 5ª onda; ver §4.3.
- seis páginas de seed `0016`–`0018`, `0020`, `0023`, `0027` — 6 dos 34 arquivos; ver §4.2.
- `planos/metricas.md` aparece nas duas listas por ser, ao mesmo tempo, página bloqueada e entrega de
  decisão; não é contado duas vezes na fila de 26. `arquitetura/retencao-e-capacidade.md` já saiu da
  fila: foi entregue como `proposto`, com volume pendente.

## 6. Ordem recomendada (maior desbloqueio por sessão)

Esta é a fila **interna** do `wyd-db`. No nível do projeto, `wyd-1.0.0.md` §6.2 mantém a captura A1 como
próxima ação técnica, porque ela remove o bloqueio de fio; a fila de retenção não o remove.

**Decisão do dono em 2026-11-23, completada em 2026-11-24:** a rodada **do `wyd-db`** é a de **retenção**.
`Q-RET-1`, `Q-RET-2` e `Q-RET-4` foram respondidas (30 dias); `Q-RET-6` e `Q-RET-7` foram respondidas em A
(2026-11-24); `Q-RET-3` foi respondida em A (2026-11-24). **Não resta decisão de retenção aberta.** A ordem
abaixo é a proposta de execução.

**Próxima frente:** não restam decisões de retenção nem de capacidade; `Q-DP-2/validação` continua
pendente, mas não é pré-requisito de `Q-RET-3`. `Q-RET-6` e `Q-RET-7` têm política decidida e aplicação
pendente; a aplicação de `Q-RET-1/2/4/6/7` é trabalho técnico, não decisão nova; `Q-RET-3` não cria job e
a matriz de `Q-CAP-2` e o sinal de `Q-CAP-3` estão aplicados na página.

1. **Aplicar `Q-RET-1`, `Q-RET-2`, `Q-RET-4`, `Q-RET-6` e `Q-RET-7` primeiro.** As cinco já têm decisão e
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
seeds dependem de terceiro; as 26 páginas bloqueadas dependem de execução ou de decisão já listada. A
validação jurídica de `Q-DP-2/validação` permanece separada e não deve ser declarada feita.

## 7. O que este registro não faz

- Não responde decisão do dono em nome dele; registra as respostas reais.
- Não aceita página, não escreve página bloqueada e não cria índice ou job.
- Não altera as 38 lacunas, as 3 externas, a distribuição do `wyd-client` nem `LCLI-02`/`LCLI-05`.
- Não usa o inventário da árvore do cliente como lista de patch.
- Não trata o pedido ao titular como enviado.

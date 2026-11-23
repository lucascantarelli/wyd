# Retenção — prazos pendentes da `M32`

> Pergunta: "por quanto tempo cada tabela que a `M32` deixou `pendente` guarda linha, e o que o
> repositório já permite saber para decidir isso?"
> Público: dono do produto, `wyd-db`, `wyd-ops` e revisão jurídica
> Dono: operação + produto (os prazos); `wyd-db` mantém o DDL e este registro
> Base: `proposta-dados-pessoais-e-retencao.md` §5.2 (`M32`); `lacunas.md` (`M22`, `M25`, `M30`,
> `B1`, `M7`, `A12`); `ddl-confirmado.md` §5.1, §5.4, §5.7, §5.9, §5.33 e §5.34;
> `padrao-banco.md` `P-21`; `docs/arquitetura/retencao-e-capacidade.md` (não escrita)
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Sete perguntas, uma por categoria que a `M32` deixou `pendente`.
Nenhuma está respondida e nada foi aplicado. **Nenhum número desta página é medição de volume**: o
planejamento não tem volume medido (§2), então todo prazo abaixo é uma decisão proposta, com a
justificativa e o custo declarados.

## 1. Fronteira desta rodada

**Fecha:** os prazos de `person_token`, `login_attempt`, `person_consent`, `pin_reset`, `config_*`,
`market_*` e backup — as sete linhas `pendente` de `proposta-dados-pessoais-e-retencao.md` §5.2.

**Não decide:** a rotina que executa o expurgo (dono `wyd-ops`); o TTL dos tokens (`B1`, onda 6); a
base legal de cada tratamento (`M32`, `Q-DP-2`, `a validar`); a política de backup em si — só a
retenção das cópias; e a criação de `planos/retencao-e-capacidade.md`, que continua bloqueada por
medição.

**Não reabre:** `M22` (sessão e auditoria), `M25` (fila), `M30` (idempotência), `M7` (consentimento
versionado), `A12` (fechamento anonimiza e-mail), `P-21` (retenção declarada) nem `M32` (as oito
`Q-DP`). Onde uma decisão anterior já responde a categoria, esta rodada **confirma**, não refaz.

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

### 2.1 Volume: não há medição

Nenhuma página do repositório mede volume de linha, e as fontes que o mediriam não existem:

| O que mediria | Estado | Fonte |
|---|---|---|
| volume por tabela e crescimento | `planos/retencao-e-capacidade.md` **não escrita**, "bloqueada por medição" | `docs/arquitetura/index.md`; `docs/arquitetura/fronteira-hot-cold.md` |
| limiar, latência e capacidade | `requisitos-nao-funcionais.md` bloqueada: "nenhum limiar foi medido" | `docs/specs/requisitos-funcionais.md` |
| carga, stress e volume | "não há volume nem limiar aprovado" | `docs/planos/plano-de-testes.md` |
| produção | nada está em produção e nenhum serviço está no ar | `docs/specs/restricoes-e-premissas.md` §2 |

O que existe é **teto estrutural de linha viva**, derivado de índice único — não é volume:

| Tabela | Teto de linha viva | De onde vem |
|---|---|---|
| `portal.person_token` | 3 por pessoa (um vivo por `purpose`) | `ux_person_token_vivo` |
| `portal.person_consent` | 3 por pessoa (um vivo por `kind`) | `ux_person_consent_vivo` |
| `game.pin_reset` | sem teto; uma linha por pedido | não há único "vivo por conta" |
| `ops.login_attempt` | sem teto; uma linha por tentativa | append-only |
| `ops.config_revision` / `config_change` | sem teto; uma revisão e N mudanças por edição de staff | append-only |
| `portal.market_wallet`, `market_reputation` | 1 por conta | PK `account_id` |
| `portal.market_*` e `game.market_escrow` | sem teto; uma linha por evento | append-only/estado |

**Histórico é ilimitado em todas**: só os índices únicos limitam a linha **viva**.

### 2.2 Índices que servem (ou não servem) ao expurgo

| Tabela | Índice existente | Serve ao expurgo? |
|---|---|---|
| `portal.person_token` | `ix_person_token_expira (expires_at) WHERE used_at IS NULL` | **parcial**: não alcança linha já usada |
| `ops.login_attempt` | `ix_login_attempt_occurred (occurred_at DESC)` | **sim** — criado "para retenção/reaper por tempo" |
| `portal.person_consent` | `ix_person_consent_pessoa (person_id, granted_at DESC)` | **não**: não há índice por tempo |
| `game.pin_reset` | `ix_pin_reset_account (account_id, created_at DESC)` | **não**: não há índice por `expires_at` |
| `ops.config_revision` | `ix_config_revision_aplicada (applied_at DESC)` | serviria, mas a tabela é sem purge por decisão (§3) |
| `ops.config_change` | `ix_config_change_revision`, `ix_config_change_alvo` | **não**: a tabela não tem timestamp próprio |
| `portal.market_notification` | `ix_market_notification_conta_nao_lida` (`read_at IS NULL`) | **não** para linha lida |
| `game.market_escrow` | `ix_market_escrow_estado (state, created_at)` | **sim** para linha fechada |
| `portal.market_listing` | `ix_market_listing_status_prazo (status, ends_at) WHERE status = 'ativo'` | **não** para linha fechada |
| `portal.market_transaction` | `(buyer_account_id, settled_at DESC)` e `(seller_account_id, settled_at DESC)` | **não** é expurgo |
| `portal.market_ledger` | `ix_market_ledger_conta_criado (account_id, created_at DESC)` | **não** é expurgo |
| `portal.market_dispute` | `(state, created_at)` parcial para aberta | **não** para linha decidida |

Consequência medida: **três categorias exigem índice novo** antes de qualquer expurgo por tempo —
`person_token` (índice completo por `expires_at`), `pin_reset` (idem) e `market_notification` (por
`read_at`). As demais podem usar o índice que já existe ou não têm expurgo proposto.

## 3. O que já está decidido, e o que isso já garante

- **`M22`** — `person_session` 30 dias após `expires_at`; `game.session` 365 dias após `released_at`;
  `audit_log` 730 dias, append-only. O padrão "data de expiração + 30 dias" já é decisão do dono.
- **`M25`** — `delivery_queue`: `applied`/`failed` 30 dias após `closed_at`; `pending` nunca; índice
  `ix_delivery_queue_fechadas` serve o expurgo.
- **`M30`** — `idempotency_record`: 7 dias após `created_at`; piso de 24 h (janela de replay), teto
  de custo.
- **`§5.9` do DDL** — `config_revision`/`config_change` são append-only e **sem purge**, decisão
  registrada e aprovada pelo dono; voltar atrás é revisão nova.
- **`M7`** — consentimento é versionado; revogar preenche `revoked_at` e **não apaga** o histórico.
- **`A12`** — fechar a pessoa anonimiza o e-mail, mas não apaga a linha; o histórico de `game` fica.
- **`P-21`** — toda tabela que cresce tem retenção declarada; a declaração pode ser por tempo ou por
  ciclo de vida, mas não pode faltar.

## 4. As sete decisões

### Q-RET-1 — Quanto tempo `portal.person_token` guarda linha?

**Medido.** `expires_at` é obrigatório e a linha guarda `used_at`; há um token vivo por pessoa e
propósito (`email_verify`, `password_reset`, `email_change`). O índice de expiração é **parcial**
(`WHERE used_at IS NULL`), então não serve para apagar linha já usada. O TTL do token é política de
`B1` (onda 6), não está no schema. Volume: no máximo 3 linhas vivas por pessoa; histórico por emissão,
sem medição.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Apagar todo token **30 dias após `expires_at`**, usado ou não, em expurgo diário | alinha com `M22` (sessão: 30 dias após expirar); mantém o hash por 30 dias para investigar reenvio/abuso; exige índice completo por `expires_at` |
| B | Apagar só o token usado, no ato do uso; o expirado fica para sempre | duas regras para a mesma tabela e histórico sem teto |
| C | Guardar 1 ano após `expires_at` | mais armazenamento sem ganho: token é de uso único e vida curta |

**Recomendação: A.** O prazo é **decisão, não medição**: 30 dias é o mesmo número que `M22` já usa
para sessão expirada, e cobre a janela de reenvio de `B1` (3 reenvios/h) com folga. **Custo:** um
índice completo novo (`expires_at`) e um `DELETE` diário; nenhum corpo de tabela muda. **Fontes:**
`M22` e `B1` em `lacunas.md`; `ddl-confirmado.md` §5.4 (`ix_person_token_expira` parcial).

### Q-RET-2 — Quanto tempo `ops.login_attempt` guarda linha?

**Medido.** Append-only, uma linha por tentativa; `occurred_at` é o carimbo; o índice
`ix_login_attempt_occurred` existe e foi criado "para retenção/reaper por tempo (`M22`)". A pergunta
que a tabela responde é "quantas falhas para este identificador **na última hora**". O identificador é
hash, não PII em claro. Volume: o maior da lista (uma linha por tentativa), sem medição.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Apagar **30 dias após `occurred_at`**, em expurgo diário | cobre 720× a janela documentada de 1 h; mesmo número de `M22`/`M25`; índice pronto |
| B | 7 dias | barato, mas perde investigação de credential stuffing que atravessa a semana |
| C | 365 dias | guarda um ano de tentativa; volume alto sem uso documentado além de 1 h |

**Recomendação: A.** É **decisão, não medição**; o único uso documentado da tabela é a janela de 1 h,
e 30 dias é o menor prazo já usado pelo projeto para trilha operacional. **Custo:** só o `DELETE`
diário; nenhum índice novo. **Fontes:** `ddl-confirmado.md` §5.4 (índice e comentário de reaper);
`M22` e `M25` em `lacunas.md`.

### Q-RET-3 — Quanto tempo `portal.person_consent` guarda linha?

**Medido.** Prova de consentimento com versão; `granted_at` e `revoked_at`; um vivo por `(person,
kind)`. Revogar preenche `revoked_at` e **não apaga** (`M7`). Não há índice por tempo. Volume: no
máximo 3 linhas vivas por pessoa; histórico por mudança de versão, sem medição.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Sem prazo por tempo: guardar **enquanto a linha de `person` existir**; o `CASCADE` da FK apaga junto se a pessoa for fisicamente removida (o que a `1.0.0` não faz) | retenção declarada por ciclo de vida, não indefinida por omissão; a prova sobrevive ao fechamento (`A12`), que anonimiza mas não apaga |
| B | Apagar linha revogada após 5 anos | precisa de validação jurídica que o repositório não tem; pode apagar a prova antes de uma disputa |
| C | Apagar a linha revogada no ato da revogação | destrói a prova de qual texto foi aceito e quando |

**Recomendação: A.** É a única que preserva a prova sem inventar prazo legal: o ciclo de vida já está
no DDL (`ON DELETE CASCADE`), e `A12` mantém a pessoa fechada. **Custo:** nenhum job; crescimento igual
ao número de mudanças de consentimento por pessoa, que é baixo por natureza. **Fontes:** `M7` e `A12`
em `lacunas.md`; `ddl-confirmado.md` §5.7; `P-21`.

### Q-RET-4 — Quanto tempo `game.pin_reset` guarda linha?

**Medido.** `created_at`, `expires_at`, `used_at`; FK `RESTRICT` para `game.account`, e conta não é
apagada; **não há índice por `expires_at`** nem único "vivo por conta" (diferença declarada em
`§5.5`). TTL é política de `B1`. Volume: uma linha por pedido de reset, sem medição.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Apagar **30 dias após `expires_at`**, usado ou não, em expurgo diário | mesmo padrão de `person_token` e `M22`; sem isso a tabela cresce para sempre, porque a conta nunca é apagada |
| B | Apagar só após o uso | linha expirada e nunca usada fica para sempre |
| C | Guardar 1 ano | cresce sem uso documentado |

**Recomendação: A.** **Decisão, não medição**, pelo mesmo motivo e número de `Q-RET-1`; a diferença é
que aqui não existe nem índice parcial. **Custo:** índice novo por `expires_at` mais o `DELETE`
diário. **Fontes:** `ddl-confirmado.md` §5.5; `B1` e `M22` em `lacunas.md`.

### Q-RET-5 — `ops.config_revision` e `config_change` têm prazo?

**Medido — e é aqui que a `M32` diverge do DDL.** `§5.9` registra decisão aprovada pelo dono:
histórico append-only e **sem purge**; "retenção com purge" está entre as alternativas recusadas.
`M32` §5.2 lista `config_*` como `pendente`. As duas coisas não podem valer ao mesmo tempo. Além
disso, `config_change` **não tem timestamp próprio**: a idade só vem de `config_revision.created_at`
por join.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Confirmar o que `§5.9` já decidiu: **sem expurgo**, retenção pelo tempo de vida do banco; a linha sai de `pendente` | não reabre decisão fechada; o histórico continua explicando qualquer configuração aplicada; volume é de edição de staff, sem medição |
| B | Expurgar mudança com mais de 5 anos | reabre `§5.9`, exige validação jurídica e quebra a promessa de histórico completo |
| C | Expurgar `config_change` e manter `config_revision` | revisão fica sem as mudanças que a explicam — histórico pela metade |

**Recomendação: A.** A rodada não cria regra: ela **resolve a contradição** entre `M32` §5.2 e
`§5.9`, mantendo a decisão que o dono já aprovou. **Custo:** nenhum job; a tabela cresce uma revisão e
N mudanças por edição de staff. **Fontes:** `ddl-confirmado.md` §5.9 (as quatro escolhas e o registro
da aprovação); `M32` §5.2; `P-21`.

### Q-RET-6 — Quanto tempo as tabelas de mercado guardam linha?

**Medido.** São 11 tabelas, com naturezas diferentes: `market_wallet` (1 por conta), `market_ledger`
(append-only, `UPDATE`/`DELETE` revogados), `market_transaction` (FK `RESTRICT` para `listing`),
`market_listing` (status fechado), `market_bid` (FK `RESTRICT`), `market_escrow` (`retido`/
`entregue`/`devolvido`, `closed_at`), `market_dispute` (FK `RESTRICT`), `market_review` (FK para
transação), `market_reputation` (1 por conta), `market_notification` (`read_at`), `market_fee_rule`.
As FKs `RESTRICT` impedem apagar `listing`/`transaction`/`escrow` sem apagar o que os referencia.
`market_notification` não tem índice por `read_at` que sirva ao expurgo de lida. A `M32` registra
"pode haver preservação legal".

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Separar por natureza: **financeiro e custódia sem expurgo por tempo** (`wallet`, `ledger`, `transaction`, `dispute`, `review`, `reputation`, `fee_rule`, `escrow`); **`market_notification` lida: 90 dias após `read_at`**; não lida: nunca | preserva integridade e prova de negócio; o único crescimento sem uso é a notificação lida; exige índice por `read_at` |
| B | Expurgar `listing`/`bid`/`escrow` fechados após 365 dias | as FKs `RESTRICT` impedem apagar o que tem transação/lance; exigiria arquivo e reescrita de FK — projeto novo, não expurgo |
| C | Sem expurgo para tudo, inclusive notificação | simples, mas notificação lida acumula para sempre sem uso |

**Recomendação: A.** Não há um prazo único possível: o financeiro é prova e tem FK que impede
apagar; a notificação é a única linha sem consequência. O prazo de 90 dias é **decisão, não medição**,
e vale só para a lida. **Custo:** índice novo por `read_at` e `DELETE` diário só em
`market_notification`; as demais não ganham job. **Fontes:** `ddl-confirmado.md` §5.34 (FKs, estados,
índices); `M32` §5.2 (preservação legal); `A12`.

### Q-RET-7 — Quanto tempo as cópias de backup ficam?

**Medido.** Não existe `guia-administrador/backup-e-restore.md` (listado como "não escrito"), não há
RPO/RTO, nenhum backup foi executado e nenhum serviço está no ar. `M32` §5.2 atribui a retenção de
backup a `wyd-ops`. Não há medição de tamanho nem frequência.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | **30 dias de cópia diária**, com rotação, criptografada e fora do host; restaurar uma vez por trimestre | cobre erro humano e corrupção recente; mesmo número de `M22`/`M25`; não serve como arquivo histórico |
| B | 7 dias de cópia diária | metade do custo, mas perde recuperação de erro descoberto tarde |
| C | 1 ano | mais seguro para auditoria, mas armazenamento e tempo de restauração sem medição |

**Recomendação: A.** **Decisão, não medição**: 30 dias é o menor prazo já usado pelo projeto e o
bastante para o cenário operacional; arquivo de longo prazo é outra decisão. **Custo:** armazenamento,
criptografia, rotação e um ensaio de restore trimestral; **backup não é fonte de expurgo por linha** —
apagar a linha viva não apaga a cópia, e a `M32` já decidiu que nada promete apagamento físico.
**Fontes:** `docs/guia-administrador/index.md` (página não escrita); `M32` §5.2 e §8; `M22`/`M25`.

## 5. O quadro que as sete decisões produzem

| Categoria | Prazo proposto | Âncora | Índice | Job |
|---|---|---|---|---|
| `person_token` | 30 dias | `expires_at` | **novo** (completo) | diário |
| `login_attempt` | 30 dias | `occurred_at` | pronto | diário |
| `person_consent` | ciclo da pessoa | `person` (FK `CASCADE`) | — | nenhum |
| `pin_reset` | 30 dias | `expires_at` | **novo** | diário |
| `config_*` | sem expurgo (confirmado) | — | — | nenhum |
| `market_*` (financeiro/custódia) | sem expurgo | — | — | nenhum |
| `market_notification` (lida) | 90 dias | `read_at` | **novo** | diário |
| backup | 30 dias (cópia diária) | — | — | rotação + restore trimestral |

## 6. O que esta rodada não decide

- **Não fixa o TTL dos tokens**: é `B1`, onda 6; esta rodada fixa quanto tempo a **linha** fica depois
  de expirar.
- **Não decide a base legal** nem a retenção exigida por lei: `M32` `Q-DP-2` está `a validar`.
- **Não decide a rotina** (frequência, janela, ferramenta): dono `wyd-ops`.
- **Não escreve `planos/retencao-e-capacidade.md`** nem mede volume: a página continua bloqueada.
- **Não decide arquivo histórico** de mercado nem de configuração.
- **Não aplica nada**: nenhum índice, nenhum `DELETE`, nenhuma linha de `M32` muda até a resposta.

## 7. O que muda quando as sete forem respondidas

| # | Entrega | Arquivo | Dono |
|---|---|---|---|
| 1 | prazos no quadro de §5.2 | `proposta-dados-pessoais-e-retencao.md` §5.2 | `wyd-db` |
| 2 | regra de retenção por categoria em §2 | `docs/specs/restricoes-e-premissas.md` | `wyd-db` |
| 3 | índices de expurgo (`person_token`, `pin_reset`, `market_notification`) | migration nova | `wyd-db` |
| 4 | rotina de expurgo e rotação de backup | `wyd-ops` / `planos/retencao-e-capacidade.md` | `wyd-ops` |
| 5 | `M32` §5.2 sem `pendente` | `proposta-dados-pessoais-e-retencao.md` | `wyd-db` |

## 8. O que continua bloqueado depois desta rodada

- **Produção de qualquer tabela ainda `pendente`**, até os prazos serem respondidos (`M32`).
- **A rotina de expurgo**, até `wyd-ops` desenhar o job e o registro.
- **A página de retenção e capacidade**, até haver volume medido (`planos/retencao-e-capacidade.md`).
- **A declaração de conformidade**, até a revisão jurídica da base legal (`M32`).

## 9. Registro da decisão

**Aguardando resposta do dono.** Nenhuma das sete foi respondida; `M32` §5.2 continua com as sete
linhas `pendente`, e nenhum índice ou job foi criado. Quando houver resposta, o registro entra aqui
com a data, a opção escolhida por pergunta e o arquivo de cada entrega.

| Pergunta | Resposta | Data |
|---|---|---|
| `Q-RET-1` | — | — |
| `Q-RET-2` | — | — |
| `Q-RET-3` | — | — |
| `Q-RET-4` | — | — |
| `Q-RET-5` | — | — |
| `Q-RET-6` | — | — |
| `Q-RET-7` | — | — |

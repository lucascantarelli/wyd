> Pergunta: "o que cresce no `wyd-db` e o que fazer com isso?"
> Público: Arquiteto de schema, operação e revisor de capacidade
> Dono: arquitetura + operação
> Base: `wyd-db-1.0.0.md` §6.3; `padrao-banco.md` `P-21`; `ddl-confirmado.md` §4, §5.1–§5.7, §5.9, §5.16, §5.33 e §5.34; `proposta-retencao-pendente.md` §2–§4; `proposta-dados-pessoais-e-retencao.md` §5.2; `proposta-decisoes-capacidade.md` §4; `lacunas.md` `M20`, `M22`, `M25`, `M30` e `M32`; `docs/planos/plano-de-documentacao.md` §2
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-23)

**Volume:** **pendente de medição (`M20`)**. Nenhum volume por tabela, crescimento por dia, bytes por
linha, custo de vacuum, latência ou limiar de alerta foi medido no repositório. A matriz abaixo é
estrutural: ela registra o que o DDL e as decisões já fixam e deixa a lacuna de volume visível em cada
linha. Este documento **não** é `aceito` e não fecha a Onda 5.

**`Q-CAP-2`:** decidida em **A** pelo responsável em 2026-11-23 — a forma é a **matriz por tabela
crescente**. Ela está **aplicada** nesta versão para as populações de identidade/sessão, operação e
mercado/custódia; os grupos fora desse escopo estão declarados em §3.4, sem omissão silenciosa. A
página continua `proposto` e nenhuma coluna de volume foi preenchida.

Três níveis de afirmação convivem aqui:

- **`V`** — lido no DDL ou na decisão citada, em 2026-11-23; vale para o fato daquela linha, não para
  uma estimativa de volume.
- **`H`** — lacuna aberta, com o ID ou a pergunta que a fecha; não é uma escolha diferente.
- **`P`** — planejado e ainda não executado; descreve intenção, não comportamento em produção.

## 1. O que está medido e o que não está

| Pergunta | Resposta | Fonte |
|---|---|---|
| Quanto cada tabela ocupa hoje? | **não medido** | `proposta-retencao-pendente.md` §2.1 |
| Quanto cresce por dia ou por evento? | **não medido** | idem |
| Quantos bytes por linha? | **não medido** | idem |
| Qual é o custo de vacuum ou o bloat? | **não medido** | idem; `guia-administrador/diagnostico.md` §4 |
| Existe limiar de latência ou capacidade? | **não**; nenhum limiar foi aprovado | `specs/requisitos-funcionais.md`, "não medido" |
| Existe número de produção para comparar? | **não**; não há produção nem volume medido | `specs/restricoes-e-premissas.md` §2 |
| Qual é o limiar de partição? | **`~10 M` linhas por tempo**, decisão de `P-21`, **não medição** | `padrao-banco.md` `P-21` |
| Quando a medição acontece? | **depois da série**; `planos/metricas.md` não pode ser escrito antes | `lacunas.md` `M20`; `planos/plano-de-documentacao.md` §2 |

**O que esta página não faz:** não estima volume por analogia com o legado e não transforma o teto
estrutural de uma linha viva em volume. O teto de `3` tokens por pessoa, por exemplo, é uma regra de
unicidade; o histórico de emissões continua sem medição.

## 2. Regra que já pode ser afirmada

`P-21` exige que toda tabela de estado vivo que cresce tenha retenção declarada; a declaração pode ser
por tempo ou por ciclo de vida, mas não pode faltar. Partição só entra acima de `~10 M` linhas, por
tempo (`RANGE (occurred_at)`), e auditoria e sessão são os candidatos futuros — nenhum dos dois foi
medido nem particionado.

Duas consequências já valem:

- categoria com retenção `pendente` **não vai a produção**, embora o planejamento e o DDL sigam livres;
- a página não escolhe prazo de retenção: os prazos decididos estão nas fontes citadas e os abertos
  continuam em [`proposta-retencao-pendente.md`](../proposta-retencao-pendente.md).

## 3. Matriz de crescimento e retenção

**Como ler:** `Volume` é sempre **não medido** nesta versão; a coluna não repete um número que não
existe. `Retenção` registra a decisão vigente ou a pergunta aberta. `Aplicação` diz se o índice e o
expurgo já existem; nenhum job de expurgo foi executado ou criado no repositório. `Faixa` usa apenas
as categorias de [`fronteira-hot-cold.md`](fronteira-hot-cold.md); `não classificada` significa que a
fonte não coloca a tabela em nenhuma das quatro faixas.

### 3.1 Identidade e sessão

**Medições nesta seção:** para **toda linha**, `Volume`, `Crescimento/dia`, `Bytes/linha`, `Custo de
vacuum`, `Latência` e `Limiar` são **não medidos (`M20`)**. A coluna `Cresce por` registra apenas a
unidade estrutural que a tabela acumula, não uma taxa: nenhum evento/dia foi medido.

| Tabela | Cresce por | Teto vivo | Retenção | Aplicação | Índice de expurgo | Faixa | Medições |
|---|---|---|---|---|---|---|---|
| `portal.person` | cadastro de pessoa; uma linha por e-mail | 1 por e-mail (`ux_person_email`) | sem expurgo; fechar anonimiza e mantém a linha (`A12`) | fechamento é `status`; sem `DELETE` de runtime | `ux_person_email`; sem índice por data | frio de identidade | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.person_profile` | perfil opcional; uma linha por pessoa | 1 por pessoa | **não declarada**; a linha acompanha a pessoa (`A12`; `CASCADE` só em remoção física) | sem expurgo | PK `person_id`; sem índice por data | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.person_authenticator` | cadastro e revogação de fator TOTP | 1 ativo por `(person, kind)`; histórico sem teto | **não declarada**; sem expurgo automático; prazo, se existir, é política de `wyd-ops` (`ddl-confirmado.md` §5.20) | sem expurgo automático | `ux_person_authenticator_ativo` e `ix_person_authenticator_person`; sem índice por `revoked_at` | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.person_session` | sessão do portal, com histórico por login | sem teto único; N sessões | 30 dias após `expires_at` (`M22`) | decisão; `DELETE`/job pendente | `ix_person_session_expira` (`expires_at`) | frio de identidade | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.person_token` | emissão de token de cerimônia | 3 por pessoa, um vivo por `purpose` | 30 dias após `expires_at`, usado ou não (`Q-RET-1`) | decisão; índice completo e `DELETE` pendentes | `ix_person_token_expira` é parcial (`WHERE used_at IS NULL`) | frio de identidade | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.person_consent` | versão de consentimento por pessoa | 3 por pessoa, um vivo por `kind` | **decidida (`Q-RET-3 = A`, 2026-11-24): ciclo de vida da pessoa** | sem expurgo por tempo; a prova sobrevive ao fechamento | `ix_person_consent_pessoa` não é por tempo | frio de identidade | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `game.session` | conexão de jogo, com histórico por sessão | 1 viva por conta e 1 por personagem; histórico sem teto | 365 dias após `released_at` (`M22`) | decisão; índice e `DELETE`/job pendentes | índices vivos (`ux_session_viva_*`, `ix_session_reaper`); não há índice de expurgo por `released_at` | quente enquanto viva; histórica depois de liberada | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `game.pin_reset` | pedido de reset de PIN | sem teto; uma linha por pedido | 30 dias após `expires_at`, usado ou não (`Q-RET-4`) | decisão; índice e `DELETE` pendentes; TTL é `B1` | `ix_pin_reset_account` não é por `expires_at` | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |

### 3.2 Operação

**Medições nesta seção:** para **toda linha**, `Volume`, `Crescimento/dia`, `Bytes/linha`, `Custo de
vacuum`, `Latência` e `Limiar` são **não medidos (`M20`)**; `Cresce por` é a unidade estrutural, não uma
taxa.

| Tabela | Cresce por | Teto vivo | Retenção | Aplicação | Índice de expurgo | Faixa | Medições |
|---|---|---|---|---|---|---|---|
| `ops.login_attempt` | tentativa de login | sem teto; uma linha por tentativa | 30 dias após `occurred_at` (`Q-RET-2`) | decisão; `DELETE`/job pendente | `ix_login_attempt_occurred` serve | frio operacional | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `ops.audit_log` | evento auditável de staff, portal ou jogo | sem teto; append-only | 730 dias, append-only (`M22`) | decisão; `DELETE`/job pendente | `ix_audit_log_occurred` serve | frio operacional | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `ops.config_revision` | edição de configuração por staff | sem teto; uma revisão por edição | **sem purge**, append-only (`ddl-confirmado.md` §5.9) | decisão; nenhum expurgo por desenho | `ix_config_revision_aplicada` não é usado para expurgo | frio operacional | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `ops.config_change` | N mudanças por revisão | sem teto; N por revisão | **sem purge**, append-only (`ddl-confirmado.md` §5.9) | decisão; nenhum expurgo por desenho | não tem timestamp próprio | frio operacional | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `ops.delivery_queue` | pedido de concessão drenado pelo jogo | sem teto; `pending` nunca é expurgada | `applied`/`failed` 30 dias após `closed_at`; `pending` nunca (`M25`) | decisão; `DELETE`/job pendente | `ix_delivery_queue_fechadas` serve | frio operacional; é a ponte de escrita | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `ops.idempotency_record` | ação de staff com chave idempotente | 1 por `(caller, method, route, key)`; histórico sem teto | 7 dias após `created_at` (`M30`) | decisão; `DELETE`/job pendente | `ix_idempotency_record_criado` serve | frio operacional | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |

### 3.3 Mercado e custódia

As onze tabelas de `M31` têm política decidida em `Q-RET-6 = A` (2026-11-24); a aplicação (índice, job
e preservação legal) continua pendente. O DDL tem índices de
consulta, mas nenhum deles foi criado para expurgo por tempo; `market_notification` não tem índice para
linha já lida. A tabela `game.market_escrow` é a única de `game` neste bloco. Para **toda linha**, as
medições de volume, taxa, bytes, vacuum, latência e limiar são **não medidas (`M20`)**; a coluna
`Cresce por` é estrutural, não uma taxa.

| Tabela | Cresce por | Teto vivo | Retenção | Aplicação | Índice de expurgo | Faixa | Medições |
|---|---|---|---|---|---|---|---|
| `portal.market_wallet` | uma carteira por conta | 1 por conta | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | PK `account_id`; sem índice por tempo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_ledger` | lançamento financeiro | sem teto; um por lançamento | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | `ix_market_ledger_conta_criado` é de consulta, não de expurgo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_fee_rule` | nova regra de taxa | sem teto; uma por vigência | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | `uq_market_fee_rule_vigencia`; sem índice por tempo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_listing` | anúncio criado | sem teto; um por anúncio | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | índices por status/prazo, não por expurgo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_bid` | lance em anúncio | sem teto; um por lance | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | `ix_market_bid_listing_criado` é de consulta | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_transaction` | transação concluída | 1 por anúncio | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | índices por comprador/vendedor, não por expurgo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_review` | avaliação de transação | 1 por transação | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | `uq_market_review_transaction`; sem índice por tempo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_reputation` | uma reputação por conta | 1 por conta | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | PK `account_id`; sem índice por tempo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_notification` | notificação de anúncio, transação ou disputa | sem teto; uma por evento | **decidida (A, 2026-11-24)** | lida: 90 dias após `read_at`; não lida: nunca; índice por `read_at` e job pendentes | `ix_market_notification_conta_nao_lida` só cobre não lida | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `portal.market_dispute` | disputa aberta | sem teto; uma por disputa | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | `ix_market_dispute_estado_aberta` só cobre aberta | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |
| `game.market_escrow` | custódia de item por operação | sem teto; uma por operação | **decidida (A, 2026-11-24)** | sem expurgo por tempo; preservação legal pendente | `ix_market_escrow_estado` cobre estado, não expurgo por tempo | não classificada na fonte | volume, taxa, bytes, vacuum, latência e limiar: **não medidos (`M20`)** |

### 3.4 Fora desta matriz, sem omissão silenciosa

O escopo da `Q-CAP-2` foi aplicado às populações de **identidade/sessão, operação e mercado/custódia**.
As populações abaixo não entram na matriz porque a fonte não lhes dá retenção por evento/tempo e o
`Q-RET` não as colocou em decisão. Elas não são uma estimativa de volume e continuam sob `M20`:

| População | Tabelas | Por que não entram | Volume e medições |
|---|---|---|---|
| conteúdo e catálogo | as **41** tabelas `content.*` | conteúdo versionado/permanente por `content_version` (`padrao-banco.md` §2); não há expurgo por evento/tempo | não medidos (`M20`); a forma de cada tabela é do dicionário |
| estado de jogador fora de sessão/mercado | as **33** tabelas `game.*` restantes | retenção por ciclo de vida da conta (`padrao-banco.md` §2), sem expurgo por evento/tempo; não há uma decisão `Q-RET` para elas | não medidos (`M20`); não são lixo de retenção |
| conteúdo e economia do portal | as **10** tabelas `portal.*` restantes | conteúdo, catálogo, configuração ou lançamento de economia; não há retenção por evento/tempo decidida | não medidos (`M20`); `daily_reward_claim` é evento, mas não tem prazo declarado |

**Contagem do escopo:** 25 tabelas com linha nominal em §3.1–§3.3; 84 nas três populações declaradas
acima; 109 no DDL. A matriz não transforma ausência de política em política: onde a fonte não declara
retenção, a linha diz `não declarada` ou `pendente`, não um prazo inventado.

## 4. Teto vivo não é volume

O DDL tem limites estruturais que não devem ser lidos como volume; os tetos de cada tabela estão na
matriz, e os exemplos abaixo são os que mais mudam a leitura:

| Tabela | Teto de linha viva | De onde vem |
|---|---|---|
| `portal.person_token` | 3 por pessoa, um vivo por `purpose` | `ux_person_token_vivo` |
| `portal.person_consent` | 3 por pessoa, um vivo por `kind` | `ux_person_consent_vivo` |
| `game.pin_reset` | sem teto; uma linha por pedido | não há único "vivo por conta" |
| `ops.login_attempt` | sem teto; uma linha por tentativa | append-only |

`game.session` tem no máximo uma sessão viva por conta e uma por personagem, mas o histórico de
sessões liberadas não é limitado por esses índices. Em todas as tabelas de histórico, o que limita é o
expurgo — quando ele existe.

## 5. Partição

Nenhuma tabela foi medida acima de `~10 M` linhas e nenhuma partição existe no DDL. A regra de `P-21`
é a única afirmação permitida hoje: partição só acima do limiar, por tempo. `ops.audit_log` e
`game.session` são os candidatos registrados; escolher um deles sem medição seria inventar um fato.

**Condição para rever:** depois da série, medir o volume por tabela e comparar com `~10 M`; só então
decidir se a partição por `occurred_at` ou `released_at` entra em uma migration futura. Nenhuma
migration de partição foi criada nesta rodada.

## 6. Custo de vacuum

Não há custo de vacuum, taxa de bloat, `fillfactor` ou ajuste por tabela medidos no repositório. A
rotina de manutenção também não foi escrita: `planos/plano-de-manutencao.md` continua bloqueado por
rotina e inventário organizacional, e `guia-administrador/manutencao-rotineira.md` continua bloqueado
por rotina. O que esta página pode dizer é só isso: **o custo é desconhecido até a medição**; não há
número para publicar nem ação prescrita.

## 7. Quente × histórica

A distinção útil é a de [`fronteira-hot-cold.md`](fronteira-hot-cold.md): quente é o que o jogo altera
a cada sessão e lê no caminho crítico; frio é conteúdo, auditoria e dado de operação. Nesta matriz:

| Faixa | Tabelas desta página | O que isso muda |
|---|---|---|
| quente enquanto viva | `game.session` | a linha viva serve presença e reaper; a liberada vira histórico |
| frio de identidade | `portal.person`, `person_session`, `person_token`, `person_consent` | o acesso é do portal; a retenção é de identidade |
| frio operacional | `ops.login_attempt`, `audit_log`, `config_*`, `delivery_queue`, `idempotency_record` | a operação é dona da rotina e do expurgo |
| não classificada na fonte | `portal.person_profile`, `portal.person_authenticator`, `game.pin_reset`, as onze tabelas de mercado | a fonte hot/cold não as nomeia; não se inventa faixa |

**O que a classificação não decide:** não define prazo, volume, partição nem alerta. Ela só diz onde o
dado vive e quem o escreve; a retenção continua nas fontes de §3.

## 8. Sinal e dono — decidido em `Q-CAP-3 = A`

**Decisão do responsável em 2026-11-23: A — sinal, dono e condição de medição, sem limiar numérico.**
Não há sistema de alerta no `wyd-db`, e `WYDDB-025` proíbe criar limiar, painel ou SLO sem medição. O
único número disponível é o `~10 M` de `P-21`, que é **limiar de partição**, não de alerta.

O sinal registrado é uma **revisão de capacidade**: quando a série existir e `M20` medir o volume, a
tabela que alcançar o limiar de partição de `P-21` entra em revisão de partição. Isso não é um alerta
automático nem um SLO; é a condição de medição que autoriza reavaliar a forma. O dono da operação é
`wyd-ops`, que mede e executa a rotina; a decisão de mudar a partição continua de arquitetura. Nenhum
limiar numérico de alerta foi aprovado, e esta página não inventa um.

## 9. O que esta página não cobre

- **A rotina de expurgo, o backup, o reaper e o restore:** o executor dos expurgos foi decidido em
  2026-11-24 (`Q-BLK-02` = `cron` do host no Compose) e o backup ganhou destino, RPO de 24 h e RTO de 4 h
  (`Q-BLK-03`); `Q-RET-7 = A` já fixava 30 dias de cópia diária, rotação, criptografia fora do host e
  restore trimestral. O que **não** existe: contrato e região do destino, credencial administrativa
  nominal, ensaio de restore e os próprios jobs — a rotina escrita está em
  [`planos/plano-de-manutencao.md`](../planos/plano-de-manutencao.md) e o procedimento, em
  [`guia-administrador/backup-e-restore.md`](../guia-administrador/backup-e-restore.md).
- **A validação jurídica de `Q-DP-2`:** o dono definiu em 2026-11-24 o artefato **parecer jurídico
escrito, por finalidade**; o parecer **não existe** e `Q-DP-2/validação` continua pendente. `Q-RET-3`
foi decidida em A pelo dono na mesma data, por ciclo de vida, **sem** transformar essa dispensa em
validação jurídica; esta página não declara base legal.
- **O prazo das categorias `pendente`:** está em
  [`proposta-retencao-pendente.md`](../proposta-retencao-pendente.md), não aqui. Não resta categoria de
  retenção `pendente`; `Q-RET-6` tem política decidida, mas a aplicação do índice/job e a preservação
  legal continuam pendentes.
- **O número medido e o percentual de prontidão:** são de `planos/metricas.md`, que não pode ser
  escrito antes da série (`M20`).
- **A matriz nominal das 109 tabelas:** a forma foi decidida em `Q-CAP-2 = A` e aplicada às 25 tabelas
  das populações de §3.1–§3.3; as 84 restantes estão declaradas em §3.4, não omitidas.
- **O limiar de alerta:** `Q-CAP-3 = A` decidiu sinal + dono + condição de medição, sem limiar numérico;
  esta página não o inventa.

## 10. Condição de retomada

Esta página passa de `proposto` para uma versão com números somente quando:

1. a série existir e `M20` permitir a medição;
2. o volume por tabela, o crescimento e o custo de vacuum vierem de comando/arquivo com data;
3. o sinal de `Q-CAP-3 = A` for exercitado sobre essa medição, sem ganhar limiar inventado;
4. a 5ª onda puder ser declarada fechada sem antecipar `M20`.

Até lá, a ausência de número é uma **lacuna declarada**, não um zero e não uma estimativa.

## 11. Como verificar esta página

- Os nomes de tabela, colunas e índices vêm de `ddl-confirmado.md` §4, §5.1–§5.7, §5.9, §5.16,
  §5.33 e §5.34 (lido em 2026-11-23).
- Os prazos e a ausência de expurgo vêm de `M22`, `M25`, `M30`, `Q-RET-1`, `Q-RET-2`, `Q-RET-4` e
  `Q-RET-5`; `Q-RET-3`, `Q-RET-6` e `Q-RET-7` foram decididas (A, 2026-11-24), com aplicação pendente
  para `Q-RET-6/7`; **não resta categoria de retenção aberta**.
- A regra de partição e a retenção declarada vêm de `padrao-banco.md` `P-21`.
- A ausência de volume, limiar e custo de vacuum vem de
  [`proposta-retencao-pendente.md`](../proposta-retencao-pendente.md) §2 e de
  [`proposta-decisoes-capacidade.md`](../proposta-decisoes-capacidade.md) §2.
- A página **não** foi aceita, não foi medida e não fecha a Onda 5.

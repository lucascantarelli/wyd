# Dados pessoais e retenção — rodada de decisões da `M32`

> Pergunta: "quais dados pessoais o WYD trata, para quais finalidades, quem responde por eles e como
> são retidos ou eliminados?"
> Público: dono do produto, arquitetura, `wyd-db`, `wyd-portal`, `wyd-admin`, `wyd-ops`, segurança, QA
> e revisão jurídica
> Dono: produto (finalidades e política); `wyd-db` mantém o modelo e o registro de lacunas
> Base: `padrao-banco.md` (`P-21`, `P-22`); `lacunas.md` (`A12`, `B1`, `M6`, `M7`, `M22`, `M25`, `M30`);
> `ddl-confirmado.md` §5.1, §5.4–§5.7 e §5.34; `docs/specs/restricoes-e-premissas.md` §2;
> `docs/arquitetura/seguranca.md`; `wyd-client/lacunas.md` (`LCLI-04`)
> Revisado em: 2026-11-23

**Estado:** decidido (2026-11-23). As oito perguntas foram respondidas pelo dono; §9 registra a
escolha e as alternativas recusadas. Esta
página **não** declara conformidade com LGPD, GDPR ou outra lei, **não** atribui papel de controlador
ou encarregado por inferência e **não** cria obrigação jurídica: ela organiza o que o projeto pode
decidir sozinho e nomeia o que depende de validação externa.

## 1. Fronteira desta rodada

**Decide:** a política mínima que o projeto pode fixar sem consultoria jurídica — inventário
derivado do DDL, papéis por função, o que fica bloqueado até validação, como se trata o que já está
em `ops` e o que se exige de terceiros.

**Não decide:** base legal de cada tratamento, texto de política de privacidade, nomes de pessoas
responsáveis, contratação de fornecedor, prazos de retenção que exijam medição de volume. Cada uma
dessas aparece na pergunta correspondente com a informação que falta.

**Não reabre:** `P-22` (PII só em `portal`), `M7` (consentimento versionado), `M22` (três prazos),
`A12` (fechamento anonimiza e-mail), `M6` (identificador hasheado em `login_attempt`), `B1` (e-mail
real na `1.0.0`, provedor na onda 6), `M25` e `M30` (expurgos da fila e da idempotência). Onde uma
opção tocaria uma delas, o texto diz isso na própria opção.

## 2. O que já está decidido, e o que isso já garante (`V`)

| Decisão | O que ela garante | O que ela **não** garante |
|---|---|---|
| `P-22` | PII de identidade só em `portal`; `game` referencia `person_id` e não copia e-mail | base legal, finalidade, prazo de nada |
| `M7` | `portal.person_consent` guarda `kind` (`terms`, `privacy`, `marketing`), `version` e instante; revogar preenche `revoked_at` | que exista texto de política versionado — **não existe no repositório** |
| `M22` | `person_session` 30 dias após `expires_at`; `game.session` 365 após `released_at`; `audit_log` 730, append-only | prazo de qualquer outra tabela (ver §4, `Q-DP-4`) |
| `A12` | fechar a pessoa reescreve o e-mail para `closed-<id>@removido.invalid` e o índice é total, sem reuso | apagamento físico: FK e auditoria impedem |
| `M6` | `ops.login_attempt.identifier_hash` é `sha256` hex; o claro nunca entra em `ops` | que o hash seja irreversível a quem já suspeita do endereço (`Q-DP-3`) |
| `B1` | envio real de e-mail na `1.0.0`, provedor escolhido na onda 6, 3 reenvios/h por pessoa | qual provedor, em que região, com que contrato (`Q-DP-8`) |

## 3. Inventário derivado do DDL

O que o schema já cria, lido de `ddl-confirmado.md`. **Isto é inventário técnico, não base legal:** a
classificação jurídica de cada campo é do `Q-DP-2`.

| Onde | Campo | O que é | Identifica |
|---|---|---|---|
| `portal.person` §5.1 | `email`, `email_fold` | endereço único, normalizado | **direto** |
| `portal.person` §5.1 | `password_hash` | `argon2id` | credencial |
| `portal.person` §5.1 | `status`, `suspended_reason`, `suspended_by`, `closed_at` | estado e anotação de staff | indireto |
| `portal.person_profile` §5.6 | `display_name` | nome de exibição, 2–32 | indireto; se aparece em ranking/armaria é decisão de tela (`M21`, externa) |
| `portal.person_profile` §5.6 | `birth_date` | data de nascimento, opcional | indireto |
| `portal.person_profile` §5.6 | `country` | país ISO alpha-2, opcional | indireto fraco |
| `portal.person_session` §5.5 | `ip`, `user_agent` | origem da sessão | indireto |
| `portal.person_token` §5.6 | `requested_ip`, `token_hash` | origem do pedido, hash do token | indireto |
| `portal.person_consent` §5.7 | `kind`, `version`, `granted_at`, `revoked_at` | prova de aceite | indireto |
| `ops.login_attempt` §5.4 | `identifier_hash`, `ip` | hash do e-mail/login e origem | **indireto reidentificável por dicionário** |
| `ops.audit_log` §5.3 | `actor_kind`, `actor_id`, `actor_ref` | quem agiu (`M18`) | indireto |
| `portal.market_*` §5.34 | `account_id`, anúncio, lance, transação | atividade econômica da conta | indireto |

**Campos sem consumidor provado no repositório:** `birth_date` e `country`. O DDL os cria como
opcionais; nenhuma página do projeto declara para que servem. Isso é assunto do `Q-DP-2`.

**O que não está no repositório e não pode ser inventado:** base legal por finalidade; texto e versão
de política de privacidade; nomes de responsáveis e canal de contato; fornecedor de e-mail,
hospedagem e pagamento; medição de volume para dimensionar prazos.

## 4. As oito decisões

### Q-DP-1 — Quem responde pelas decisões de tratamento?

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fixar os **papéis por função** — responsável de produto, responsável técnico (`wyd-ops`), revisão jurídica externa — e deixar nome de pessoa e canal de contato como campo `pendente` que bloqueia coleta real | a política tem dono verificável hoje; a coleta real continua bloqueada; nenhum nome ou endereço é inventado |
| B | O dono do projeto declara agora os nomes e um contato | resolve o campo nominal; o canal ainda depende de endereço real, e quem não foi consultado passa a constar como responsável |
| C | Não registrar nada até designação formal | a política inteira fica sem dono, contra `AGENTS.md` §5, item 5 |

**Recomendação: A.** O que o projeto pode decidir sozinho é a estrutura de responsabilidade, não o
nome de quem aceita o risco. `C` deixa a lacuna sem dono — exatamente o defeito que `M32` existe para
corrigir.

**Decidido: B (2026-11-23).** O dono assumiu os papéis de produto e técnico: §5.1 nomeia os três
papéis, e revisão jurídica e contato do titular seguem `pendente`, com o bloqueio explícito. A
recomendação era `A`, que não nomeia pessoa; o dono preferiu declarar o papel a si mesmo.

**Não decide:** quem são as pessoas, o endereço de contato e se haverá encarregado formal.

### Q-DP-2 — Quais finalidades existem, e o que se faz com o campo sem consumidor?

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Publicar a finalidade de cada grupo de campo derivada do uso já planejado (identidade e autenticação, sessão, prova de consentimento, antiabuso, auditoria de staff, economia do mercado), marcar **cada uma** com `hipótese legal: a validar` e deixar `birth_date`/`country` como **não coletados** até existir consumidor declarado | inventário mínimo verificável; minimização aplicada; nenhuma base legal declarada; cadastro real continua bloqueado |
| B | Declarar consentimento como base de todo tratamento | contraria `M7` (que guarda consentimento, não o elege base única) e ignora tratamento que não se apoia em consentimento |
| C | Não publicar finalidade nenhuma até o jurídico responder | nada é coletado nem planejado com método; o inventário continua vazio |

**Recomendação: A.** Minimização é decisão do projeto; base legal não é. `birth_date` e `country` sem
consumidor são o caso concreto: coletar "porque a coluna existe" é o oposto de finalidade declarada.

**Aplicado (A, 2026-11-23):** a tabela de finalidades está em §5.4, e `birth_date`/`country` ficaram
como **não coletados**.

**Não decide:** qual hipótese legal se aplica a cada finalidade; se `birth_date` será necessária para
verificação de idade; o texto da política.

### Q-DP-3 — O `identifier_hash` conta como reidentificável?

Fato medido: `identifier_hash` é `sha256(email_fold)` ou `sha256(login_fold)`, **sem segredo**
(`ddl-confirmado.md` §5.4). Quem suspeita de um endereço calcula o hash e confirma a existência; o
que `M6` impede é listar endereços, não confirmar um.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Manter `M6` como está; classificar o hash como **identificador indireto reidentificável**; registrar o risco e a mitigação já existente (sem `SELECT` para runtime de jogo, sem export, acesso só de operação) | nenhum DDL muda; o risco fica escrito; a mitigação é a que o `GRANT` já dá |
| B | Trocar a derivação para `HMAC-SHA256` com segredo por ambiente | fecha o ataque de confirmação; **reabre `M6`**, cria segredo novo em `src/provision/spec.md` §3/§7 e muda o contrato de quem escreve (`wyd_portal` e `wyd_game`) |
| C | Remover o identificador e guardar só `ip` e `result` | perde a pergunta que a tabela existe para responder (falhas por identificador); `ix_login_attempt_identifier` deixa de ter uso |

**Recomendação: A.** `B` é tecnicamente melhor e custa uma decisão fechada mais um segredo que o
projeto hoje não tem para hash; `C` destrói a função da tabela. `A` é a única que registra o risco sem
fingir que ele não existe, e a troca para `B` continua possível como decisão própria com evidência.

**Aplicado (A, 2026-11-23):** o risco e a mitigação estão em `docs/arquitetura/seguranca.md` e na
regra de §2 de `restricoes-e-premissas.md`; `M6` ficou como está.

**Não decide:** migrar para `HMAC`; criar segredo de hash; exportar `ops` para BI.

### Q-DP-4 — O que vale para as categorias sem prazo?

Antes desta rodada, só três tabelas tinham prazo (`M22`). `ix_login_attempt_occurred` e
`ix_person_token_expira` existiam "para o reaper" no comentário do DDL, mas **nenhum prazo tinha sido
decidido** para elas; `Q-RET-1`, `Q-RET-2` e `Q-RET-4` decidiram os três em 2026-11-23.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Aplicar `P-21` como regra de aceite: **tabela crescente sem prazo declarado não entra em produção**; as categorias sem prazo ficam listadas como `pendente`, com dono (`operação` + `produto`) e bloqueio de produção — não de DDL nem de planejamento; cada prazo novo nasce de proposta com medição de volume | o crescimento fica contido sem inventar número; a série pode ser escrita; produção fica condicionada |
| B | Fixar prazo para todas as categorias agora, por analogia | fecha a lista hoje, mas com número sem medição — o tipo de estimativa que `M20` proíbe |
| C | Declarar retenção indefinida nas categorias fora de `M22` | aceita crescimento sem política, contra `P-21` |

**Recomendação: A.** `P-21` já exige retenção declarada; o que falta é a consequência. Fixar prazos
sem volume é chute com aparência de decisão, e `M22` só fechou porque teve dono e data.

**Aplicado (A, 2026-11-23):** a lista com as categorias `pendente` está em §5.2, e
`restricoes-e-premissas.md` §2 tem a regra.

**Não decide:** os prazos em si; a rotina de expurgo (dono `wyd-ops`); retenção de backup — esta foi
decidida em `Q-RET-7 = A` (2026-11-24), com rotina/fornecedor/RPO/RTO pendentes.

### Q-DP-5 — Como o titular exerce acesso, correção e exclusão?

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Exclusão = **fechamento com anonimização** (`A12`), preservando o que `M22` exige (auditoria 730 dias) e o que a FK não deixa apagar (histórico de `game`); acesso e correção = tela do portal ou atendimento por staff com registro em `ops.audit_log`; nada promete apagamento físico | coerente com o DDL e com `M22`; o que fica preservado é explícito; a tela é de `M21`/`wyd-portal` |
| B | Prometer apagamento físico completo em até N dias | impossível hoje: `ops.audit_log` é append-only, `game` referencia a pessoa e backup não é versionado por linha |
| C | Só atendimento manual, sem registro nem prazo | some a prova de que o pedido foi atendido, e `audit_log` passa a ser opcional |

**Recomendação: A.** É o único caminho que o schema suporta hoje. `B` vira promessa que a primeira
solicitação real quebra.

**Aplicado (A, 2026-11-23):** §5.3 fixa o que fica preservado, e `restricoes-e-premissas.md` §2 tem a
regra de exclusão.

**Não decide:** o prazo de resposta ao titular (é política de produto, sem base no repositório); a
tela de export (é de `wyd-portal`); se haverá exclusão física no futuro.

### Q-DP-6 — O que pode entrar em log e telemetria?

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Log e telemetria usam `person_id`, `account_id` ou hash — **nunca e-mail, login ou PIN em claro**; retenção de log fica `pendente` com dono `wyd-ops`; telemetria com identificador bloqueia produção até ter prazo | operação funciona; o vazamento por log fica barrado; a retenção de log continua aberta, mas com dono |
| B | Logar identificador em claro para facilitar diagnóstico | PII fora de `portal`, contra `P-22`, e cópia em sistema que ninguém expurga |
| C | Proibir qualquer identificador em log | torna diagnóstico de abuso impossível; `login_attempt` já existe justamente para isso |

**Recomendação: A.** `P-22` decide onde a PII mora; log com e-mail em claro a colocaria em todo lugar.
Identificador indireto resolve o diagnóstico sem criar cópia em claro.

**Aplicado (A, 2026-11-23):** a regra está em `restricoes-e-premissas.md` §2, e `erros-e-logs.md` §5
recebeu o ponteiro.

**Não decide:** formato, destino e prazo do log (dono `wyd-ops`); ferramenta de observabilidade.

### Q-DP-7 — Que dado pode entrar em teste, fixture e captura?

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fixture, golden e captura só com dado **sintético declarado**; captura do cliente só em sessão sintética controlada (`LCLI-04`); captura de sessão real exige decisão nova, com higienização verificável | mantém `LCLI-04` e `LPRT-10` como estão; nenhum golden carrega sessão real |
| B | Permitir captura real higienizada à mão, caso a caso | a higienização não é verificável no repositório; um golden vira cópia de sessão sem registro |
| C | Adiar toda captura até a política fechar | trava `LPRT-03` e a onda A do `wyd-protocol` sem necessidade |

**Recomendação: A.** Já é a regra de `LCLI-04`; o que falta é dizer que ela vale para golden, fixture
e ticket — e que captura real precisa de decisão própria, não de boa vontade.

**Aplicado (A, 2026-11-23):** a regra está em `restricoes-e-premissas.md` §2, com `LCLI-04` citado.

**Não decide:** o formato da captura sintética (`wyd-client`, `D3`–`D6`); o conteúdo dos goldens.

### Q-DP-8 — Quais terceiros podem receber dado?

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Nenhum fornecedor que receba PII está autorizado nesta rodada; cada candidato (e-mail, hospedagem, pagamento) entra no registro **antes** do uso, com região, contrato, instrução de exclusão e dono, e o do e-mail sai da onda 6 (`B1`) | nada é contratado por engano; a escolha continua sendo decisão com dono e data |
| B | Aprovar categorias de fornecedor sem nomear | autoriza por classe o que ninguém avaliou; `B1` já fixou que o provedor se escolhe na onda 6 |
| C | Deixar a lista para depois da política completa | o primeiro serviço que precisar de e-mail ou hospedagem decide sozinho, sem registro |

**Recomendação: A.** O repositório não tem fornecedor escolhido nem contrato; aprovar por categoria
seria decidir sem dono. `A` mantém a decisão onde ela pertence: `B1` e o planejamento de `wyd-ops`.

**Aplicado (A, 2026-11-23):** a regra está em `restricoes-e-premissas.md` §2, com `B1` citado.

**Não decide:** qual provedor; região; contrato; se haverá pagamento na `1.0.0`.

## 5. Os quadros que as decisões produzem

Preenchidos com as respostas de 2026-11-23 (§9). Em `Q-DP-1` o dono optou por nomear; nas demais, a
opção recomendada.

### 5.1 Responsáveis (`Q-DP-1`, opção B)

| Papel | Quem | Estado |
|---|---|---|
| Responsável de produto (finalidades e risco) | dono do projeto | **nomeado em 2026-11-23** |
| Responsável técnico (controles, log, backup) | dono do projeto; `wyd-ops` executa | **nomeado em 2026-11-23** |
| Revisão jurídica | externa, não contratada | `pendente` — bloqueia declaração de conformidade; o dono declarou em 2026-11-24 que o projeto é experimental, sem gasto e sem apoio jurídico, o que **não** produz o parecer nem fecha `Q-DP-2` |
| Canal de contato do titular | a definir | `pendente` — sem endereço real no repositório |

### 5.2 Retenção: o que existe e o que falta (`Q-DP-4`)

| Categoria | Tabela | Prazo | Estado |
|---|---|---|---|
| sessão do portal | `portal.person_session` | 30 dias após `expires_at` | decidido (`M22`) |
| sessão de jogo | `game.session` | 365 dias após `released_at` | decidido (`M22`) |
| auditoria | `ops.audit_log` | 730 dias, append-only | decidido (`M22`) |
| fila de entrega | `ops.delivery_queue` | `applied`/`failed` 30 dias após `closed_at`; `pending` nunca | decidido (`M25`) |
| idempotência | `ops.idempotency_record` | 7 dias após `created_at` | decidido (`M30`) |
| token de cerimônia | `portal.person_token` | 30 dias após `expires_at`, usado ou não | **`decidido`** (opção A do dono, 2026-11-23; índice completo e expurgo ainda não criados) |
| tentativa de login | `ops.login_attempt` | 30 dias após `occurred_at` | **`decidido`** (opção A do dono, 2026-11-23; `DELETE` diário ainda não criado) |
| reset de PIN | `game.pin_reset` | 30 dias após `expires_at`, usado ou não | **`decidido`** (opção A do dono, 2026-11-23; índice e `DELETE` diário ainda não criados; TTL é `B1`) |
| histórico de consentimento | `portal.person_consent` | ciclo de vida da pessoa | **`decidido`** (`Q-RET-3 = A`, 2026-11-24: guardar enquanto a linha de `person` existir; sem expurgo por tempo; revogação não apaga) |
| configuração versionada | `ops.config_revision`, `ops.config_change` | — | **`decidido`** — append-only e sem expurgo (`ddl-confirmado.md` §5.9, aprovado em 2026-11-20; resolvido por evidência em 2026-11-23) |
| mercado e custódia | `portal.market_*`, `game.market_escrow` | — | **`decidido`** (`Q-RET-6 = A`, 2026-11-24: financeiro/custódia sem expurgo; notificação lida 90 dias; índice, job e preservação legal pendentes) |
| backup | — | — | **`decidido`** (`Q-RET-7 = A`, 2026-11-24: cópia diária de 30 dias, rotação, criptografia fora do host e restore trimestral; rotina/fornecedor/RPO/RTO pendentes) |

Regra que acompanha o quadro: **categoria sem retenção decidida não vai a produção** — planejamento e
DDL seguem livres. Com `Q-RET-3 = A`, não resta categoria de retenção `pendente`; a coleta real continua
sujeita a `Q-DP-2/validação` e ao canal do titular, que não foram resolvidos por esta decisão.

### 5.3 O que fica preservado ao fechar a conta (`Q-DP-5`)

| O que | Por quê | Fonte |
|---|---|---|
| `ops.audit_log` da pessoa | auditoria append-only, 730 dias | `M22` |
| histórico de `game` (personagem, itens, ledger) | integridade referencial; o `account` não some com a pessoa | `ddl-confirmado.md` §5.2 |
| linha de `portal.person` | fechada, com e-mail anonimizado | `A12` |
| sessões e tokens | cascata da FK ao fechar | `ddl-confirmado.md` §5.5–§5.6 |

### 5.4 Finalidades declaradas (`Q-DP-2`, opção A)

| Grupo de campo | Finalidade | Hipótese legal |
|---|---|---|
| `person.email`, `password_hash`, `status` | identidade, autenticação e estado da conta | a validar |
| `person_session.*` | manter a sessão do portal e permitir revogação | a validar |
| `person_token.*` | verificação de e-mail, redefinição de senha e troca de e-mail (`B1`) | a validar |
| `person_consent.*` | provar qual texto o titular aceitou (`M7`) | a validar |
| `person_profile.display_name` | nome de exibição no portal | a validar |
| `login_attempt.*` | antiabuso: falhas por identificador e por origem | a validar |
| `audit_log.*` | auditoria de ação de staff (`M18`, `M22`) | a validar |
| `market_*`, `market_escrow`, `account_ledger` | executar e auditar a economia do mercado (`M16`, `M31`) | a validar |

**Não coletados até existir consumidor:** `person_profile.birth_date` e `person_profile.country`. As
colunas continuam no DDL como opcionais (`ddl-confirmado.md` §5.6), e nenhuma tela as preenche
enquanto não houver finalidade declarada que as justifique — nenhum requisito do `wyd-portal` as cita
(`V`, busca em `wyd-portal/`, `wyd-admin/` e `wyd-db/docs/`, 2026-11-23).

## 6. As lacunas externas que esta rodada não decide

As três `externa` do `wyd-db` (`lacunas.md` §1) não entram nas oito perguntas. O motivo de cada uma:

| Lacuna | O que falta | Por que não é decidível aqui |
|---|---|---|
| `A11` — charset e tamanho do login | captura do cliente 7662 | depende de captura de tráfego, que depende de endpoint e de sessão sintética (`LCLI-04`, `D3`–`D6`); nenhum documento do repositório traz o dado |
| `A20` — MFA | telas e rotas do portal | escopo já decidido em 2026-11-18 (staff antes de público; `member` fora da 1.0.0) e DDL aprovado em `0031`; o que falta é a frente `wyd-portal` implementar, não uma decisão de `wyd-db` |
| `M21` — telas do portal | a lista por tela | escopo decidido em 2026-11-20 (portal completo, legado como `H`); a lista nominal pertence ao planejamento do `wyd-portal`, que ainda não a escreveu |

## 7. O que mudou com as respostas (2026-11-23)

| # | Entrega | Arquivo | Dono |
|---|---|---|---|
| 1 | registro da decisão, com opções recusadas e data | esta página, §9 | `wyd-db` |
| 2 | regras de PII, retenção, log e captura | `docs/specs/restricoes-e-premissas.md` §2 | `wyd-db` |
| 3 | nota do `identifier_hash` reidentificável | `docs/arquitetura/seguranca.md` | `wyd-db` |
| 4 | `M32` fechada, com decisão, alternativas recusadas e evidência | `lacunas.md` §2.3, §1 e §2.4 | `wyd-db` |
| 5 | contagem de lacunas refletida na cópia publicada | `wyd-docs/proximos-passos.md` | `wyd-docs` |

As cinco entregas foram feitas na mesma data; a `M32` fechou em `lacunas.md` §2.3.

## 8. O que continua bloqueado depois desta rodada

- Coleta, cadastro ou login de pessoa real — até validação jurídica da base legal (`Q-DP-2`) e
  definição do canal de contato do titular (`Q-DP-1`; os papéis já estão nomeados em §5.1).
- Publicar declaração de conformidade, aviso de privacidade ou prazo de atendimento ao titular.
- Ativar fornecedor que receba PII (`Q-DP-8`), log com identificador em claro (`Q-DP-6`) e captura de
  sessão real (`Q-DP-7`).
- Levar a produção qualquer tabela da lista `pendente` de §5.2.

## 9. Registro da decisão (`V`, 2026-11-23)

Rodada respondida pelo dono em 2026-11-23. A `M32` fechou na mesma data em `lacunas.md` §2.3, e as
regras de §5 foram aplicadas em `docs/specs/restricoes-e-premissas.md` §2 e em
`docs/arquitetura/seguranca.md`.

| # | Decisão | Alternativas recusadas | Efeito |
|---|---|---|---|
| `Q-DP-1` | **B — nomear agora**: o dono do projeto é responsável de produto e técnico; revisão jurídica externa e contato do titular ficam `pendente` | A (papéis por função, nomes depois), C (nada até designação formal) | §5.1 |
| `Q-DP-2` | **A — inventário mínimo** com finalidade por grupo de campo, hipótese legal `a validar`, e `birth_date`/`country` fora de coleta | B (consentimento como base única), C (nada até o jurídico) | §5.4; `restricoes-e-premissas.md` §2 |
| `Q-DP-3` | **A — manter `M6`**, classificar o hash como identificador indireto reidentificável e registrar o risco | B (HMAC com segredo, reabrindo `M6`), C (remover o identificador) | `seguranca.md`; `restricoes-e-premissas.md` §2 |
| `Q-DP-4` | **A — `P-21` como regra de aceite**: categoria sem prazo não vai a produção; as 3 categorias restantes **naquela data** ficam `pendente` com dono (as 6 iniciais menos `person_token`, `login_attempt` e `pin_reset`, decididas em 2026-11-23; `config_*` já estava fora da contagem); `Q-RET-6` fez a linha de mercado passar a `decidido` em 2026-11-24 | B (fixar prazos por analogia), C (retenção indefinida) | §5.2; `restricoes-e-premissas.md` §2 |
| `Q-DP-5` | **A — fechamento com anonimização** (`A12`), preservando auditoria e histórico de `game`; acesso e correção por tela ou staff com registro | B (apagamento físico), C (manual sem registro) | §5.3; `restricoes-e-premissas.md` §2 |
| `Q-DP-6` | **A — identificador indireto** em log; retenção de log `pendente` com `wyd-ops`; telemetria identificável bloqueia produção | B (identificador em claro), C (proibir identificador) | `restricoes-e-premissas.md` §2; `erros-e-logs.md` §5 |
| `Q-DP-7` | **A — só sintético declarado** em fixture, golden e captura; sessão real exige decisão nova | B (captura real higienizada), C (adiar toda captura) | `restricoes-e-premissas.md` §2 |
| `Q-DP-8` | **A — nenhum fornecedor autorizado**; cada candidato entra no registro antes do uso, com região, contrato e exclusão | B (aprovar categorias sem nome), C (deixar para depois) | `restricoes-e-premissas.md` §2 |

**Evidência da rodada:** as oito perguntas foram respondidas pelo dono na sessão de 2026-11-23, com
`Q-DP-1` complementada por pergunta de acompanhamento sobre os nomes (respondida "o dono assume produto
e técnico"). `Q-DP-2`–`Q-DP-8` foram respondidas na opção recomendada.

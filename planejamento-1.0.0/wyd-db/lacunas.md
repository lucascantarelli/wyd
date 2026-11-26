# Lacunas, divergências e achados estruturais — registro único

> **Estado: registro vivo.** Este é o **único** lugar onde uma lacuna do `wyd-db` 1.0.0 tem dono,
> decisão pendente, bloqueio e prazo. Quando dois documentos divergirem, vale este registro, com a
> evidência anexada na linha.
>
> **O que este arquivo cobre:** as **38 lacunas** registradas (5 bloqueantes `B`, 15 altas `A`, 18
> médias `M`), as **11 divergências** internas do plano diretor (`P1`–`P11`) e os **3 achados
> estruturais** desta rodada (`D-A`, `D-B`, `D-C`).
>
> **O que este arquivo não é:** não decide nada por conta própria. Cada linha diz **quem** decide e
> **o que** precisa ser decidido; a decisão entra no rascunho, no ADR ou no plano, e volta aqui como
> uma linha fechada com data.
>
> Medido em **2026-11-17**, recontado em **2026-11-18**, quando `A17`–`A21` entraram (§2.2), em
> **2026-11-20**, quando `A7`, `A21` e `M21` fecharam (§2.2/§2.3) e `P3` fechou com elas (§3), e de
> novo em 2026-11-20, quando `M25` entrou e fechou com o desenho do lote `0012` (§2.3), e em
> **2026-11-21**, quando `M26` entrou e fechou na mesma rodada: a produção das páginas por tabela
> do dicionário deixou de ser gerador presumido e passou a ser escrita à mão, em lotes. Na mesma
> data, `M27` entrou como **`proposta`** — o papel `wyd_admin` do serviço `wyd-admin` — e **fechou na
> mesma data** na opção A + S1 + C1 + D1 (`docs/arquitetura/seguranca.md`, `0032_admin_grants.sql`).
> Ainda em 2026-11-21, `M28` entrou **aberta** — a frase que atribui a "API interna" a credencial
> `wyd_portal` não se sustenta para escrita em `game` — e **fechou na mesma data** na **redação B**:
> a "API interna" é o serviço `wyd-admin`, as quatro tabelas de `portal` ficam com o BFF do portal e
> `wyd_portal` não serve para `game` (§2.3; `ddl-confirmado.md` §5.15; `migrations/0030_portal_content.md` §8).
> Em **2026-11-22**, `M29` entrou e fechou na mesma rodada: o `0032` **não** leva `ALTER DEFAULT
> PRIVILEGES` e o `GRANT` de tabela criada depois da troca fica no arquivo que a cria (§2.3;
> `docs/arquitetura/seguranca.md`). Ainda em **2026-11-22**, `M30` e `M31` entraram e fecharam na mesma
> rodada: o registro de idempotência do `wyd-admin` ganhou tabela em `ops`, com **7 dias** de retenção
> (fecha `LAC-ADM-02` e `LAC-ADM-06`), e o mercado/armaria ganhou lote próprio, com onze tabelas (fecha
> `LAC-ADM-07`). A série passa a **34 arquivos e 109 tabelas**.
> Nenhum teste ou Postgres foi executado para as **contagens** deste arquivo: elas vêm de `grep` nas
> migrations do legado e da lista nominal do §11 do rascunho. A única medição em Postgres é a de `M29`
> (§2.3), feita em PG 17.11 em 2026-11-22.
> Em **2026-11-23**, `M32` entrou como `proposta` e **fechou na mesma data**, com as oito perguntas
> `Q-DP` respondidas pelo dono: a política de dados pessoais e retenção está em
> `proposta-dados-pessoais-e-retencao.md` §4 e §9. Ela não reabre `P-22`, `M7` nem `M22`.

---

## 0. Como ler

O **quadro de fechamento** — o que ainda falta, com dono, custo estimado e o que cada item desbloqueia —
está em [`pendencias-de-fechamento.md`](pendencias-de-fechamento.md). Ele é um registro operacional,
não cria lacuna e não altera as contagens deste arquivo.

### 0.1 Estados

| Estado | Significado |
|---|---|
| **fechada** | decidida e **já refletida** num arquivo citado na coluna "evidência" |
| **bloqueada** | falta decisão, execução ou evidência; a linha nomeia o desbloqueio (dono e documento). Execução que depende de frente fora do `wyd-db` entra aqui, com o dono externo nomeado |

### 0.2 Prazos (marco, não data)
**Sem arquivo no repositório:** `0001_bootstrap.sql`, `0007_game_session.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Não há calendário: a ordem do projeto é por **dependência**, não por semana. Prazo é o marco antes
do qual a lacuna precisa estar fechada.

| Marco | Significado |
|---|---|
| **P0** | antes de **congelar** `0001_bootstrap.sql` / `0007_game_session.sql` — muda DDL ou contagem de tabelas |
| **P1** | antes da **ADR de fronteira pessoa × conta** (sem número ainda) e do `planos/metricas.md` — fecha desenho, não muda tabela |
| **P2** | antes da **reconciliação do plano diretor** — a passada única de `P1`–`P11` (§3) sobre `wyd-db-1.0.0.md`; não é passo da ordem de construção de `plano-migrations.md` §9 Refletida em `planejamento-1.0.0/wyd-db/docs/convencoes.md`. |
| **P3** | antes da **onda de documentação** que registra o assunto |
| **P4** | antes da **onda de código** dona do assunto — não bloqueia schema nem doc de identidade Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/index.md`. |
| **EXT** | dono fora do `wyd-db`; sem data até a frente abrir |

**Notação:** em prosa fora de §0.2 e §2.4, o marco sai qualificado — "marco `P2`" — porque `P1`–`P4`
também são IDs de divergência no §3; em célula cujo cabeçalho já diz `Prazo` ou `Marco`, o código
basta. Nenhuma das duas séries foi renumerada: o qualificador é o que as distingue.

### 0.3 Donos

| Dono | Decide |
|---|---|
| **produto** | regra de negócio, comunicação ao jogador, política de suporte |
| **arquitetura** | forma do modelo, nomes, fronteiras, o que vira tabela × coluna × política |
| **operação** | retenção, backup, reaper, limite de recurso |
| **frente `wyd-server`** | handshake, limite por IP, liberação de sessão no canal |
| **frente `wyd-portal`** | telas, cadastro, recuperação, sessão do BFF |

"arquitetura" hoje significa **eu proponho, você aprova**. Nenhuma linha deste registro foi fechada
por decisão minha sozinha.

---

## 1. Situação em uma linha

| Grupo | Quantas | Bloqueadas | Fechadas |
|---|---:|---:|---:|---:|
| Lacunas (`B`/`A`/`M`) | 38 | 3 (`A11`, `A20`, `M21` — execução externa) | 35 (`B1`, `B2`, `B3`, `B4`, `B5`, `A7`, `A8`, `A9`, `A10`, `A12`, `A13`, `A14`, `A15`, `A16`, `A17`, `A18`, `A19`, `A21`, `M16`, `M17`, `M18`, `M19`, `M20`, `M22`, `M23`, `M24`, `M25`, `M26`, `M27`, `M28`, `M29`, `M30`, `M31`, `M32`, `M33`) |
| Divergências do plano diretor (`P`) | 11 | 0 (—) | 11 (`P1`, `P2`, `P3`, `P4`, `P5`, `P6`, `P7`, `P8`, `P9`, `P10`, `P11`) |
| Achados estruturais (`D-A`, `D-B`, `D-C`) | 3 | 0 | 3 (`D-A`, `D-B`, `D-C`) |

As decisões abaixo entram aqui só pelo **índice**: o texto que as descreve mora no arquivo dono. A
revisão de 2026-11-17 comparou o bloco de identidade com OWASP (Authentication, Password Storage,
Session Management, Forgot Password, MFA), NIST SP 800-63B-4 e os schemas públicos de Supabase
Auth/GoTrue, Ory Kratos e Keycloak. `M1`–`M7` e `M9` foram aprovados, `M8` deferido; nenhuma decisão
cria tabela nova além das duas já previstas em `P-22`, e a série fica em **109 tabelas** — 97 até
2026-11-21, com as 4 do adendo `0030` e a do `0031`, mais a de `M30` e as 11 de `M31`, abertas em
2026-11-22 (`plano-migrations.md` §3).

| ID | Decisão | Por que | Refletida em |
|---|---|---|---|
| **M1** | `game.account.status` fica `active`/`closed`; `suspended` sai do CHECK | bloqueio disciplinar é **só** `game.account_moderation.blocked`; duas fontes para o mesmo estado era o antipadrão que `padrao-banco.md` §5 (item 5) e §11 proíbem | `ddl-confirmado.md` §5.2 |
| **M2** | `game.pin_reset` perde `delivered_to` e ganha `requested_ip` | e-mail dentro de `game` fura o `P-22` e o argumento de `pg_dump -n game` sem PII; o destinatário vem de `portal.person` por join | §5.2 |
| **M3** | `game.account` ganha `login_failure_count`, `login_locked_until`, `password_changed_at`, `updated_at` e trigger | o PIN tinha contador de falha e a senha de jogo (12 bytes no fio) não tinha nenhum; NIST/OWASP exigem limite em **todo** autenticador | §5.2 |
| **M4** | sessão do portal com expiração **absoluta** (`expires_at`) e **idle** (`last_seen_at`), `user_agent` ≤ 512, índice de expiração; `person.password_changed_at` e `revoke_all` na troca de senha | OWASP Session Management pede os dois timeouts; encerrar as sessões na troca de senha é requisito do fluxo de recuperação | §4, §5.1 |
| **M5** | `portal.person_password_reset` vira `portal.person_token`, com `purpose` (`email_verify`/`password_reset`/`email_change`), `attempt_count`, `requested_ip` e índice único parcial "um vivo por propósito" | OWASP Forgot Password: uso único, TTL, limite por conta e invalidação do anterior ao reemitir. O `purpose` com CHECK é o que impede um token genérico de servir a cerimônias diferentes | §4 |
| **M6** | `ops.login_attempt` guarda `identifier_hash` (sha256), `surface`, `ip INET` e `result` fechado | `ops` é lido por `wyd_read`: e-mail/login crus ali seriam PII fora de `portal`; o hash ainda responde "quantas falhas neste identificador por hora" | §5.4 Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/seguranca.md`. |
| **M7** | entram `portal.person_profile` e `portal.person_consent`, enxutas; **`P-22` vence o `D5`** | PII em `portal` e prova de consentimento **com versão** do documento aceito (LGPD/GDPR); sem tabela não há onde guardar nome de exibição sem sujar `person` | §5.6, §5.7 |
| **M9** | senha do portal **15–128**, sem regra de composição, sem expiração periódica, com blocklist; `argon2id m=19456, t=2, p=1` | NIST 800-63B-4 e OWASP pedem comprimento e blocklist, não "maiúscula+número+símbolo"; 10 caracteres era abaixo do mínimo vigente sem MFA | `B3` (metade da senha) |
| **M8** | MFA **não** entra na 1.0.0 (**deferido em 2026-11-17**); a forma fica reservada para `portal.person_authenticator`, com DDL no lote `0031` (`A20`) | sem consumidor medido (`D5`), mas staff `admin`/`moderator` precisa de segundo fator antes de o portal ir a público | `A20` (escopo e prazo, `externo`) |

**Demais decisões fechadas — com dono fora deste registro:** a coluna do meio nomeia a questão
fechada, não a resposta; assinatura, tipo e código moram só na página dona.

| Data | Questão fechada | Onde vive |
|---|---|---|
| 2026-11-17 | `P-01`/`P-02`: 4 schemas (`content`, `game`, `portal`, `ops`) e tabela no singular | `padrao-banco.md` §1 |
| 2026-11-17 | a série herdada **não** é substituída: a nova nasce numa branch, na pasta `migrations/`, e a antiga continua rodando (`SCHEMA_VERSION = 53`) | `plano-migrations.md` §6 |
| 2026-11-17 | PR de ferramentas **fora** da rodada; os gates de §7 do plano viram conferência manual | `plano-migrations.md` §1 |
| 2026-11-17 | `mkdocs.yml` do diretório removido; o build do site é da frente `wyd-docs` | `D-B` (§4) |
| 2026-11-17 | `M18-A`: `actor_kind` + `actor_id` + `actor_ref`, 3 CHECKs de coerência e sem FK por desenho | `ddl-confirmado.md` §5.3 + nota de §2.3 |
| 2026-11-17 | `A10`: quem libera a sessão (canal/reaper) e **sem** `released_reason` | `ddl-confirmado.md` §5.5 |
| 2026-11-18 | `A16`: token opaco, só o `sha256` no banco | `ddl-confirmado.md` §4 + página `0004` |
| 2026-11-18 | contagens da série: **29/92/29** planejados; medido só do catálogo, depois da série | `plano-migrations.md` §3 |
| 2026-11-18 | estado de tabela tem dono único e é **derivado**, não digitado | `D-C` + `migrations-checklist.md` §1.2 |
| 2026-11-18 | `P8`: os 29 slugs `NNNN-<slug>.md` aprovados, um por arquivo, derivados do nome do arquivo | `wyd-db-1.0.0.md` §6.15 |
| 2026-11-18 | que erro `MigrationTxn::record`/`forget` devolvem, e quem fica com `LedgerError` | `src/migrate/port/unit_of_work.md` §2 |
| 2026-11-18 | que variante e que código de saída tem a falha de `Source::load` | `src/migrate/error.md` §2–§3 |
| 2026-11-18 | como se chama e o que recebe o construtor do adapter do provisionamento | `src/provision/adapter/pg.md` §2 |
| 2026-11-18 | em que sessão, lock e transação o script do provisionamento roda | `src/provision/port/script.md` §3 |

**Fechadas sem tocar em DDL** — cada uma tem o dono da evidência em lugar próprio:

| Lacuna | Onde está a evidência |
|---|---|
| `B2`, `B4`, `B5` | tabela **"Evidência das fechadas"** em §2.1, colunas *onde está a decisão* e *efeito medido* (`B4`/`B5` sem tabela nova, `B2` com a lista nominal em `plano-migrations.md` §3) |
| `A9`, `A12`, `A15` | coluna **decisão necessária** da própria linha em §2.2 ("decidido: ..." / "sem rename"), com o bloqueio correspondente em *o que bloqueia* |
| `M20` | coluna **decisão necessária** da própria linha em §2.3 (fechada em 2026-11-18) | Refletida em `planejamento-1.0.0/wyd-db/plano-migrations.md` §3.
| `M24` | `src/provision/spec.md` §3 (variáveis e quem lê) e §7 (segredo) — a linha de §2.3 só registra e aponta |

Nenhuma das oito muda a contagem da série.

**O que a aprovação não compra:** nada disso foi executado — nenhum `.sql`, nenhum `crates/`, nenhum
Postgres; o efeito é só DDL de planejamento em `ddl-confirmado.md` e nas páginas de `migrations/`.

**O que continua aberto no plano diretor:** só `P3` (`planos/riscos.md`) — §3. `P1` e `P5` foram
fechadas em 2026-11-19.

---

## 2. Lacunas (38)

### 2.1 Bloqueantes
**Sem arquivo no repositório:** `src/security/policy.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| ID | O que falta | Dono | Decisão necessária | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|
| **B1** | Envio de e-mail não existe: os dois resets emitem link e nada entrega | produto + arquitetura | **decidida em 2026-11-19:** envio real na `1.0.0`; provedor escolhido na onda 6; limite de 3 reenvios/h por pessoa; TTL, templates e remetente definidos na mesma onda; `portal.person_token` (`password_reset`/`email_verify`) e `game.pin_reset` passam a entregar | `portal.person_token`/`game.pin_reset` em uso, prova S12, onda 6, `A7` | **P2** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/proposta-retencao-pendente.md`. |
| **B2** | Série nova sem número, ordem e tamanho | arquitetura | enumeração fechada antes da onda 1; `SCHEMA_VERSION` concreto | numeração contígua, ledger, página por migration | — | **fechada** Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/visao-geral.md`. |
| **B3** | Política de credencial: restam o PIN e o limiar — a senha foi decidida em `M9` | produto + arquitetura | **decidida em 2026-11-19:** PIN 4–6 dígitos, 5 falhas/10 min, desbloqueio por tempo; `game.account.pin_failure_count`/`pin_locked_until`; senha 15–128 e `argon2id` como `M9` | `src/security/policy.rs`, S7/S8, §política da ADR de fronteira pessoa × conta | **P1** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/seguranca.md`. |
| **B4** | Pessoa suspensa sem autor, motivo ou instante | arquitetura | 3 colunas na própria pessoa (`suspended_reason`, `suspended_by`, `suspended_at`) + CHECK de coerência; histórico no `audit_log` | auditoria de staff, S17, `dominio-acesso.md` | — | **fechada** |
| **B5** | Multibox por IP sem onde morar | arquitetura + `wyd-server` | `game.session.ip`; IP é auditoria, nunca identidade | `planejamento-1.0.0/wyd-db/rascunho-accounts.md` §6 `D12`, S18, M22 | — | **fechada** |

**Evidência das fechadas**

| ID | Onde está a decisão | Efeito medido |
|---|---|---|
| `B2` | `plano-migrations.md` §3 (lista nominal da série nova) | arquivos contíguos; contagem, `SCHEMA_VERSION` e a medição vivem na mesma seção (`M20`, fechada em 2026-11-18) |
| `B4` | `ddl-confirmado.md` §5.1 (colunas + CHECKs) + `rascunho-accounts.md` §6 `D11` | 0 tabela nova; 3 colunas em `portal.person`; contagem do plano inalterada (`plano-migrations.md` §3) |
| `B5` | `ddl-confirmado.md` §5.5 (coluna `ip`) + `rascunho-accounts.md` §6 `D12` | 0 tabela nova; 1 coluna em `game.session` |

Por que colunas e não tabela — a assimetria com a conta de jogo é deliberada: em `game.account`,
`blocked` **só existe** em `game.account_moderation` e `status` guarda ciclo de vida; na pessoa,
`suspended` **já é** estado de `status`. Anotar autor/motivo/instante na mesma linha usa uma fonte de
verdade em vez de duas, num `UPDATE` em vez de `UPDATE` + `INSERT`, e os CHECKs impedem o par
inválido (suspensa sem motivo, ativa com autor). O histórico de suspensões e reintegrações não se
perde: toda ação administrativa é auditada no `audit_log`, que já é o dono desse fato — uma segunda
tabela de decisões duplicaria o mecanismo de auditoria.

`released_reason` foi **reintroduzida em 2026-11-19 por `A14`**: apagar personagem derruba a sessão e
o motivo precisa sobreviver à conexão. A decisão de `A10` foi revisada na mesma data — a coluna
guarda o motivo operacional e `ops.audit_log` continua sendo o evento auditável; as duas fontes
respondem perguntas diferentes.

### 2.2 Altas

| ID | O que falta | Dono | Decisão necessária | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|
| **A7** | Cadastro de pessoa não existe (rota, regra, verificação) | produto | **decidida em 2026-11-20: autocadastro no portal, com a pessoa em `pending` até o clique no link `email_verify`; a conta de jogo só nasce depois da ativação** e o endereço não verificado não abre conta | rota de cadastro do §9, teste "cadastro cria pessoa + conta", onda 6 | **P4** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/dicionario-dados/dominio-acesso.md`. |
| **A8** | `failed_logins`/`locked_until` existem e ninguém os move | produto + arquitetura | **decidida em 2026-11-19:** mesmo limiar do PIN — 5 falhas/10 min, desbloqueio por tempo, sem intervenção de staff; os contadores existentes passam a ser movidos no login | login do portal (onda 4), prova de lockout | **P1** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/seguranca.md`. |
| **A9** | `updated_at` não se atualiza sozinho | arquitetura | **decidido:** trigger `set_updated_at()` criado uma vez em `0001`, uma função para a série | DDL de `0001`, teste que prova a coluna andando | — | **fechada** Refletida em `crates/wyd-db/migrations/0001_bootstrap.sql`. |
| **A10** | Sessão órfã de jogo: quem libera | `wyd-server` + operação | **fechada em 2026-11-18 e revisada em 2026-11-19 por `A14`:** canal libera no disconnect; reaper 60 s com `stale 180 s`; **`game.session.released_reason` passa a existir** (`disconnect`, `reaper`, `character_deleted`, `admin`) e o evento continua em `ops.audit_log` (`actor_kind = 'system'`) | `ddl-confirmado.md` §5.5 | **P0** | fechada |
| **A11** | Charset do login é UNVERIFIED | frente `wyd-server` (cliente 7662) | capturar no cliente; até lá manter permissivo (1..16, sem espaço nas bordas) e marcar `H` | fechar `GameLogin`, onda 5 | **EXT** | bloqueada — a frente `wyd-server` (cliente 7662) precisa capturar o charset do `GameLogin`; até lá o campo fica permissivo (1..16) e marcado `H` |
| **A12** | E-mail de conta fechada | produto + suporte | **decidido:** índice total + anonimização no fechamento (`closed-<id>@removido.invalid`); sem janela de reuso e sem exceção de suporte | índice `ux_person_email`, DDL de `0003_portal_person.sql`, texto de suporte do §9 | — | **fechada** Refletida em `planejamento-1.0.0/wyd-db/docs/dicionario-dados/gerado/person.md`. |
| **A13** | Fechar pessoa com contas ativas | produto + arquitetura | **decidida em 2026-11-19:** fechar tudo em uma transação — pessoa e contas no mesmo commit; recusar não é opção | porta `close_person`, S13, suporte | **P1** | fechada Refletida em `planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.8. |
| **A14** | Personagem apagado × sessão viva | arquitetura | **decidida em 2026-11-19:** soft-delete derruba a sessão **e registra o motivo** em `game.session.released_reason`; `A10` foi revisada na mesma data para não negar a coluna | semântica de `game.session.character_id`, `0007`, CHECK de motivo | **P1** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/dicionario-dados/dominio-personagem.md`. |
| **A15** | `player` × `member` | arquitetura | **sem rename**: `player` é papel de jogo, `member` é papel de portal | mapa de tradução banco/Rust/BFF/seed | — | **fechada** Refletida em `planejamento-1.0.0/wyd-db/docs/specs/requisitos-funcionais.md`. |
| **A16** | Transporte da sessão do portal: `jti` em claro no banco é bearer ou chave de busca? | arquitetura | **fechada em 2026-11-18: token opaco.** O cookie carrega o verificador e o banco guarda só `sha256` (64 hex) — sem JWT, sem `jti` em claro, sem segredo de assinatura. `M4` já exige um `UPDATE` por requisição (idle), então ler a linha pelo hash não é custo novo; o preço aceito é um `UPDATE` extra no login para gravar o hash | `portal.person_session.token_hash` (`ddl-confirmado.md` §4, página `0004` §6.1/§7/§9), `D7` | **P1** | **fechada** |
| **A17** | Conta de jogo pode ser transferida ou vendida entre pessoas | produto + arquitetura | **decidida em 2026-11-19: não** — conta não é transferível nem vendável; nenhum objeto novo e `0005` fecha sem coluna de titularidade | fechar `0005` (página `0005_game_account.md`, onda 5) e o dicionário `dominio-acesso.md` (onda 4) | **P3** | fechada Refletida em `planejamento-1.0.0/wyd-db/ddl-confirmado.md`. |
| **A18** | Cadastro de conta pode nascer dentro do jogo, além do portal | produto | **decidida em 2026-11-19: não** — conta nasce só no portal; `game.account.person_id NOT NULL` continua sendo a fronteira e não há rota de cadastro no cliente | ADR de fronteira pessoa × conta, onda 6; **não** muda `0005` | **P1** | fechada Refletida em `planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.8. |
| **A19** | Loja e cash no `1.0.0`: se existem e onde moram | produto | **decidida em 2026-11-19:** existem; loja e cash ficam no schema `portal` — `donate_*`, `daily_reward_*`, `kingdom_cape_price` e `donate_shop_item` (6 tabelas) em `portal`; `city`, `city_tax_ledger`, `item_price` e `personal_shop` (4) seguem em `game`; `0011` mantém as 10 tabelas | `0011` (10 tabelas; página `0011_game_economy.md`, onda 5) e `dominio-economia.md` | **P3** | fechada |
| **A20** | MFA fora da 1.0.0 (`M8`): quando e para quem o segundo fator existe | produto + arquitetura | **escopo e prazo decididos em 2026-11-18:** obrigatório para **staff** (`admin`/`moderator` do portal) **antes** de o portal ir a público; `member` fica fora da 1.0.0 e só volta em 1.1 com consumidor medido. Forma reservada: `portal.person_authenticator (person_id, kind, secret_encrypted, confirmed_at, last_used_step)`, TOTP como `kind` inicial; recuperação do segundo fator é fluxo de staff, nunca autoatendimento do titular; DDL no lote `0031` (`ddl-confirmado.md` §5.20, aprovado em 2026-11-20) | telas e rotas do portal (frente `wyd-portal`), onda 6 | **EXT** | bloqueada — a frente `wyd-portal` precisa entregar as telas e rotas do segundo fator (onda 6); a política e o DDL do lote `0031` já estão decididos **Reauditoria 2026-11-25:** a condição de saída registrada aqui deixou de valer — as rotas e telas do segundo fator existem em `planejamento-1.0.0/wyd-portal/docs/arquitetura/rotas-e-permissoes.md` §3 (`/admin/seguranca`, `RF-POR-013`) e o DDL está no lote `0031`; o que resta é a implementação nas fatias `WYDPOR-F0`/`F4`. Reclassificar mudaria a soma publicada e por isso era decisão do titular — **em 2026-11-25 o titular manteve `externa` até a execução**. |
| **A21** | Troca de e-mail da pessoa não tem fluxo nem decisão registrada | produto | **decidida em 2026-11-20: a troca entra na 1.0.0** — o titular pede autenticado, o token `email_change` vai para o endereço novo, o endereço antigo recebe aviso, e a conclusão revoga as sessões do portal como a troca de senha (`M4`); `ux_person_email` continua total | rota de troca de endereço e emissor do token `email_change` (frente `wyd-portal`, onda 6) | **P4** | fechada Refletida em `planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.8. |

`A17`–`A19` entraram neste registro em 2026-11-18: já eram citadas como decisão aberta em
`plano-migrations.md` §5 e em `migrations-checklist.md` §3, sem linha aqui. `A21` entrou no mesmo dia,
ao auditar `0004` §10: o `purpose = 'email_change'` já estava no CHECK e nenhuma linha o cobria.

`A17` e `A19` ficaram no marco `P3`, não no `P2`: cada uma travava a onda de documentação que registra
o assunto (`wyd-db-1.0.0.md` §11); ambas foram fechadas em 2026-11-19.

`A18` é `P1` porque a pergunta é de **fronteira**, não de tabela. **Evidência:** `game.account` não tem
coluna que registre onde a conta nasceu e `person_id` é `NOT NULL` (`ddl-confirmado.md` §5.2), então conta
criada dentro do jogo pressupõe a mesma pessoa do portal. `A17` fechou em 2026-11-19 sem objeto e
`A18` decidiu que a conta nasce só no portal; nenhuma coluna ou tabela nova entra em `0005`. A rota
continua assunto da ADR de fronteira. **Gatilho:** se uma coluna de origem aparecer, a linha volta aqui
e o marco é `P3`.

`A15` fecha por `D4` do rascunho. Não fecha por si: o mapa `GameRole` × `PortalRole` é newtype na
onda 3 (§8.1), e o teste de contrato da onda 1 passou a exigir os dois CHECKs — a decisão está
registrada, a implementação é onda 3.

`A9` fecha por `D13` do rascunho: `0001_bootstrap.sql` cria `ops.fn_set_updated_at()`, e cada arquivo
que cria tabela com `updated_at` cria o próprio trigger (`trg_person_set_updated_at` em `0003`); a
coluna deixa de depender de cada `UPDATE` escrito à mão. O teste que prova a coluna andando é da onda 1.

`A12` fecha por `D14`: `ux_person_email` é **total** (não filtra `status`) e a `UPDATE` que fecha a
pessoa grava `email = 'closed-' || id || '@removido.invalid'`, garantido pelo CHECK
`ck_person_fechamento_anonimo`. Não há "reuso após N dias": a linha fechada nunca guarda o endereço
real, o endereço original fica livre para um cadastro novo, e o histórico antigo continua preso ao
`id` velho no `audit_log`.

### 2.3 Médias

| ID | O que falta | Dono | Decisão necessária | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|
| **M16** | `game.account_wallet.coin` muda no jogo sem ledger | produto + arquitetura | **decidida em 2026-11-19: ledger** — nova `game.account_ledger` (`delta`, `balance_after`, `reason`, `created_at`), índice por conta/data e FK `RESTRICT`; a carteira passa a ser saldo derivado | contrato de `repo/wallet`, `dominio-economia.md` (onda 4) e `0005_game_account.md` (onda 5) | **P3** | fechada |
| **M17** | Chave de idempotência sem dono | arquitetura + `wyd-portal` | **decidida em 2026-11-19:** o BFF gera UUID; a chave é única em `0011`/`0012`; janela de replay de 24 h | `0011`/`0012` e as ações `accounts/donate*` da onda 4 | **P3** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/seguranca.md`. |
| **M18** | `audit_log.actor` sem formato | arquitetura | **decidido em 2026-11-17 (M18-A):** `actor_kind` (`person`/`account`/`system`) + `actor_id` + `actor_ref`, com 3 CHECKs de coerência e **sem FK, por desenho** (ver nota) | DDL de `0002_ops_audit.sql`, consulta de auditoria, índice `ix_audit_actor` | **P1** | **fechada** Refletida em `planejamento-1.0.0/wyd-db/plano-migrations.md`. |
| **M19** | Seed sem senha de dev documentada | arquitetura | **decidida em 2026-11-19:** senha fixa `dev-only`, documentada no próprio seed; nunca vale fora de desenvolvimento | gate da onda 2, teste manual | **P1** | fechada Refletida em `planejamento-1.0.0/wyd-db/docs/planos/plano-de-dados-e-seeds.md`. |
| **M20** | Contagens medidas, não estimadas | arquitetura | **fechada em 2026-11-18:** o **planejado** era 29 arquivos / 92 tabelas, dono único `plano-migrations.md` §3; o adendo `0030` (2026-11-20) levou a 30 / 96, o `0031` (mesma data) a 31 / 97 e `M27` (2026-11-21) acrescentou o `0032` sem tabela nova; o **medido** no artefato, conferido pelo gate de 2026-11-25, é **34 arquivos / 109 tabelas** (§3), e só esse número pode entrar em `planos/metricas.md` | `planos/metricas.md` (não pode ser escrito antes) | **P2** | **fechada** Refletida em `planejamento-1.0.0/wyd-db/plano-migrations.md` §3. |
| **M21** | Telas do portal não mapeadas | frente `wyd-portal` | **decidida em 2026-11-20: escopo completo** — portal público e área administrativa inteira (NOC, jogadores e contas, economia, conteúdo/CMS, launcher, logs e segurança), com configuração do servidor pelo portal, monitoramento, métricas e dashboards; as telas do legado são referência `H` e são reformuladas, e a lista por tela é do planejamento do `wyd-portal` | rotas do §9, onda 6, `planejamento-1.0.0/wyd-portal` (vazia) | **EXT** | bloqueada — a frente `wyd-portal` precisa escrever o planejamento e as rotas das telas (§9, onda 6); a pasta do sistema ainda está vazia **Reauditoria 2026-11-25:** a condição de saída registrada aqui deixou de valer — o planejamento e as rotas do portal existem (`planejamento-1.0.0/wyd-portal/docs/arquitetura/rotas-e-permissoes.md`, 50 rotas, e as fatias `WYDPOR-F0`–`F11` cobrem NOC, jogadores/contas, economia, CMS/launcher, logs e segurança); o que resta é a execução das telas. Reclassificar mudaria a soma publicada e por isso era decisão do titular — **em 2026-11-25 o titular manteve `externa` até a execução**. |
| **M22** | `portal.person_session`, `game.session` e `ops.audit_log` crescem sem política | operação | **decidida em 2026-11-19:** retenção por tabela — `person_session` 30 dias após `expires_at`; `game.session` 365 dias após `released_at`; `ops.audit_log` 730 dias, append-only | `docs/arquitetura/retencao-e-capacidade.md`, backup | **P3** | fechada |
| **M23** | `channel_capacity` e `channel_role` eram 1:1 com `server_channel`, sem ciclo de vida próprio | arquitetura | **fechada em 2026-11-18:** fundidas em `game.server_channel` (`max_players`, `maintenance`, `role`), com `ck_server_channel_jogavel` para `channel = 0` e sem `DEFAULT`; a separação só existia porque o legado proibia `ALTER TABLE` no `up` (`0040`/`0041`) | DDL de `0007`, contagem da série (93 → 92) | — | **fechada** Refletida em `planejamento-1.0.0/wyd-db/docs/dicionario-dados/dominio-canal.md`. |
| **M24** | Contrato de ambiente do `wyd-db`: quais variáveis o crate consome, com que nomes, e o que é segredo | arquitetura | **fechada em 2026-11-18:** o contrato inteiro vive em `src/provision/spec.md` §3 (variáveis e quem lê) e §7 (segredo) — esta linha só registra | implementação de `crates/wyd-db` (onda 1); gate de ambiente em `plano-migrations.md` §7 | **P4** | **fechada** |
| **M25** | Consumo e retenção da fila de entrega: como o dreno toma a linha em `ops.delivery_queue` e por quanto tempo a linha fechada fica guardada | arquitetura + frente `wyd-server` | **decidida em 2026-11-20:** o dreno reivindica com `SELECT … FOR UPDATE SKIP LOCKED` em transação curta e marca `claimed`/`claimed_at` antes de aplicar; `applied`/`failed` são expurgadas **30 dias após `closed_at`** e `pending` nunca é expurgada. Recusadas: dreno único sem lock (pode entregar duas vezes) e expurgo de tudo em 90 dias | DDL de `0012` (`ddl-confirmado.md` §5.16), página `0012_ops_delivery.md`, `docs/arquitetura/padroes-postgres.md` §6 e `docs/arquitetura/retencao-e-capacidade.md` | **P3** | fechada |
| **M26** | Dicionário de dados: como nascem as páginas por tabela (`dicionario-dados/gerado/<tabela>.md`), que o plano tratava como saída de um gerador `make dicionario-dados` | arquitetura + documentação | **decidida em 2026-11-21: escritas à mão, em lotes, ao fim do DDL.** Uma página curta por tabela, sem gerador e sem `Makefile` novo; cada uma aponta para a página de `migrations/`, que continua dona da forma, e a página por tabela resume e liga, sem copiar o DDL. Recusadas na mesma rodada: manter o gerador como projeto próprio (exigiria aprovar ferramenta e escrever código antes de existir página), adiar as páginas por tabela (deixaria a cobertura por tabela ausente sem data) e cancelar as páginas por tabela (revogaria `LPDOC-09`). **Nada é gerado:** **109 das 109** páginas foram escritas à mão, em lotes (`0001`–`0012`, `0013`, `0014`, `0015`, `0019`, `0021`, `0022`, `0024`, `0025`, `0026`, `0028`, `0030`, `0031`, `0033` e `0034`), até 2026-11-22 — o dicionário está completo e a fila está vazia | `dicionario-dados/` (onda 4), `LPDOC-09` do `wyd-docs`, gate do site; o formato da página é de `wyd-db-1.0.0.md` §6.8 | **P3** | fechada | Refletida em `planejamento-1.0.0/wyd-db/docs/dicionario-dados/como-ler.md`.
| **M27** | Papel `wyd_admin`: o serviço `wyd-admin` não tem papel próprio para escrever em `game` nem em `ops.audit_log` | `wyd-db` + `wyd-ops` | **decidida em 2026-11-21: A + S1 + C1 + D1** — `wyd_admin` com `LOGIN` e DSN própria (`WYD_DB_URL_ADMIN`), `USAGE` em `game` e em `ops`, `SELECT`/`INSERT`/`UPDATE` nas sete tabelas do C4 (`account_moderation`, `account_wallet`, `account_ledger`, `server_channel`, `session`, `city`, `item_price`), sem `DELETE`, e `INSERT` + `SELECT (id)` em `ops.audit_log`; `GRANT` em arquivo novo (`0032`) | `seguranca.md` (papel na tabela e decisão registrada); `src/provision/spec.md` §2/§4 (6º papel em `ROLES` e no golden); `migrations/0032_admin_grants.md` e `ddl-confirmado.md` §5.32; série 31 → 32 em `plano-migrations.md`, `migrations-checklist.md` e contagens; `LAC-ADM-01` do `wyd-admin`; `M28` **não** entrou | **P3** | fechada |
| **M28** | "API interna" sem dono: `ddl-confirmado.md` §5.15 e `migrations/0030_portal_content.md` §8 dizem que ela usa a credencial `wyd_portal`, mas esse papel não tem `USAGE` em `game`; o §9 da mesma página já separa conteúdo (`wyd_portal`) de ação em `game` (que é do `wyd-admin`) | arquitetura + `wyd-db` | **decidida em 2026-11-21: redação B** — a "API interna" é o serviço `wyd-admin`; as quatro tabelas de `portal` são escritas pelo BFF do portal com `wyd_portal`; o `wyd-admin` não as escreve e, para `game` e `ops.audit_log`, usa o papel de `M27` (`0032`), nunca `wyd_portal`. Recusada a redação A, que chamava o próprio BFF de "API interna" e divergia de `fronteira.md` §1–§2 do `wyd-admin`. Correção **só de prosa**, sem DDL nem `GRANT` | `ddl-confirmado.md` §5.15 e `migrations/0030_portal_content.md` §8 (redação B aplicada); teste de fronteira (`wyd_portal` sem `USAGE` em `game`) | **P3** | fechada |
| **M30** | Onde mora o registro de idempotência do contrato C4 do `wyd-admin` e por quanto tempo ele fica (lacunas `LAC-ADM-02` e `LAC-ADM-06` do `wyd-admin`) | `wyd-db` + `wyd-ops` | **decidida em 2026-11-22: tabela nova `ops.idempotency_record`, retenção de 7 dias.** A chave é a PK composta `(caller, method, route, key)`, o corpo é comparado por `body_sha256` e a linha nasce e fecha **na mesma transação da ação** — `in_progress` nunca é commitado, então não há chave presa nem estado a expirar; quem chega depois espera o `COMMIT` alheio e, estourando os 2 s da transação, recebe `409 IN_PROGRESS`. Retenção **7 dias após `created_at`**, por expurgo diário de operação: o piso é a janela de replay de 24 h de `M17` (apagar antes permitiria executar duas vezes) e o teto é o custo — ação de staff, volume baixo; depois do expurgo, `GET /actions/{key}` responde `404`. Recusadas: reaproveitar a chave natural de cada rota (só `0011`, `0012` e `0034` têm coluna de chave, e as outras tabelas do C4 não têm onde guardá-la) e `UNIQUE` parcial em `ops.audit_log` (a auditoria é append-only e passaria a guardar corpo de resposta, exigindo `UPDATE`/`DELETE` nela) | `ddl-confirmado.md` §5.33 (DDL, permissões e prova); `plano-migrations.md` §3 e `migrations-checklist.md` §2 (lote `0033`, série 32 → 33 e `Tab.` 97 → 98); `LAC-ADM-02` e `LAC-ADM-06` do `wyd-admin`; `contrato-c4.md` §2 e §5 | **P0** | fechada |
| **M31** | `game.market_escrow` e as dez `portal.market_*` do mercado/armaria não têm DDL (lacuna `LAC-ADM-07` do `wyd-admin`): a forma está em `armaria-e-mercado.md` §3 e não há lote que a crie | `wyd-db` + arquitetura | **decidida em 2026-11-22: lote `0034_portal_market.sql`, onze tabelas** — dez em `portal` (carteira, extrato, regra de taxa, anúncio, lance, transação, avaliação, reputação, notificação e disputa) e `game.market_escrow` para a custódia do item; DDL em `ddl-confirmado.md` §5.34. A série passa a **34 arquivos e 109 tabelas**, `SCHEMA_VERSION = 34`. O lote **emenda o `0032`**: sem `SELECT, INSERT, DELETE` em `game.character_item` e `SELECT` em `game.character`, a custódia aprovada não executava. Recusadas: adiar o lote (`ADM-10` do C4 ficaria sem forma e a armaria do portal sem schema) e duas migrations, uma por schema (separaria a custódia da transação que a cria, sem arquivo dono da fronteira) | `ddl-confirmado.md` §5.34 e §5.32 (emenda); `armaria-e-mercado.md` §7; `plano-migrations.md` §3 e `migrations-checklist.md` §2; `dominio-economia.md` (as 11 tabelas no domínio); `LAC-ADM-07` do `wyd-admin` | **P0** | fechada |
| **M29** | `0032_admin_grants.sql` sem `ALTER DEFAULT PRIVILEGES`: tabela criada depois da troca nasce sem privilégio para `wyd_admin` | arquitetura | **decidida em 2026-11-22: opção A** — sem `ALTER DEFAULT PRIVILEGES`; a obrigação que ela cria e a medição em PG 17.11 estão em `docs/arquitetura/seguranca.md`. Recusadas a B (default `SELECT`/`INSERT`/`UPDATE` por schema, que concede a mais e não distingue `account_wallet` de `account_moderation`) e a C (default amplo + `REVOKE` por tabela: mesmo trabalho, com janela de excesso). O default por schema de `P-18` continua valendo para os runtimes; a exceção é o papel de staff | `docs/arquitetura/seguranca.md` (decisão, alternativas e medição em PG 17.11); `migrations/0032_admin_grants.md` §5 e `ddl-confirmado.md` §5.32 (escopo do arquivo); nenhum `GRANT` ou DDL mudou | **P3** | fechada |
| **M32** | Dados pessoais e retenção: finalidades, hipótese legal, responsáveis, inventário de campos que identificam alguém (sessão, logs, auditoria, suporte, backup) e política das categorias fora de `M22` | produto + `wyd-db` | **decidida em 2026-11-23: B em `Q-DP-1`, A em `Q-DP-2`–`Q-DP-8`** — (1) responsáveis nomeados: o dono do projeto é responsável de produto e técnico, com revisão jurídica externa e contato do titular `pendente`; (2) inventário mínimo de finalidades publicado, hipótese legal `a validar` em todas, e `birth_date`/`country` fora de coleta por falta de consumidor; (3) `identifier_hash` classificado como identificador indireto reidentificável, com `M6` mantido; (4) `P-21` vira regra de aceite — tabela crescente sem prazo não vai a produção, e as 6 categorias sem prazo ficaram `pendente` com dono; em 2026-11-23, `Q-RET-1`, `Q-RET-2` e `Q-RET-4` foram decididas (30 dias cada), `Q-RET-5` confirmada por §5.9, `Q-RET-6` e `Q-RET-7` decididas em A em 2026-11-24 e `Q-RET-3` decidida em A em 2026-11-24, restando 0; (5) exclusão = fechamento com anonimização (`A12`), preservando auditoria e histórico de `game`; (6) log sem identificador em claro; (7) fixture, golden e captura só sintéticos; (8) nenhum fornecedor de PII autorizado. Recusadas: papéis sem nome, consentimento como base única, HMAC reabrindo `M6`, prazos por analogia, apagamento físico, identificador em claro, captura real higienizada e fornecedor aprovado por categoria | coleta real segue bloqueada por decisão própria — base legal a validar e contato `pendente` —, não por falta de política; planejamento e DDL com dado fictício continuam livres. Perguntas, opções e registro em `proposta-dados-pessoais-e-retencao.md` §4 e §9; regras em `docs/specs/restricoes-e-premissas.md` §2 | **P4** | fechada |
| **M33** | Estado de uma página de `migrations/` e aceite da onda A: as 27 páginas escritas não tinham a linha `**Estado:**` que `docs/convencoes.md` §Estados exige, e `wyd-db-1.0.0.md` §10 (item 10) exige `planos/metricas.md`, que não existe | `wyd-db` + arquitetura | **decidida em 2026-11-23: A em `Q-M33-1`–`Q-M33-6`** — (1) a página de `migrations/` carrega linha de estado própria e o checklist segue dono do estado do arquivo; (2) as 27 recebem `**Estado:** proposto (2026-11-23)`; (3) suspender só a metade "contabilizada em `planos/metricas.md`" do item 10 de §10, com retomada; (4) criar `migrations/index.md` agora e deixar `serie-e-regras.md` na 5ª onda; (5) o checklist não espelha o estado da página; (6) o aceite é por página, na ordem da série. Recusadas: exceção para `migrations/`, linha só no aceite, manter o item 10 inteiro, adiar o índice, coluna de estado no checklist e aceite em bloco. Aplicado em `docs/convencoes.md`, `migrations-checklist.md`, `migrations/*.md`, `migrations/index.md`, `wyd-db-1.0.0.md` §10 e a própria página da M33; nenhum DDL, migration `.sql` ou julgamento de página foi alterado. | `docs/proposta-decisoes-m33.md` §9; `docs/convencoes.md` §Estados; `migrations-checklist.md` §1; `migrations/index.md`; `wyd-db-1.0.0.md` §10 item 10 | **P3** | fechada |

`M16` e `M17` ficam no marco `P3`, não no `P4`: as duas bloqueiam **schema** — `M16` o bloco `0005`
(`ddl-confirmado.md` §5.2, uma das razões de o arquivo ser `parcial`) e `M17` os arquivos
`0011`/`0012` (`migrations-checklist.md` §3) —, o que `P4` exclui por definição; quem as fecha é a onda
de documentação que registra o assunto, como `A17`/`A19` (§2.2).

`M20` fecha em **109 tabelas** planejadas (`plano-migrations.md` §3): 92 originais mais as 4 do adendo
`0030`, a do `0031` (aprovadas em 2026-11-20), a de `M30` e as 11 de `M31` (2026-11-22). A derivação do rascunho —
`0001_contas` com 1 tabela, `0031`/`0034`/`0053` com 1 cada, 87 − 4 + 7 = 90 — fica registrada como
história do desenho **descartado**, não como número vigente: o padrão aprovado mudou o desenho. A
contagem continua **planejada**, não medida: o número real sai do catálogo depois da série.

**`M18` em detalhe (por que a coluna não é um texto `portal:42`).** Toda linha de auditoria responde
"quem fez", e o "quem" tem três origens diferentes:

| Quem age | Exemplo concreto | Precisa guardar |
|---|---|---|
| pessoa do portal | staff dá 10.000 coins para a conta 7 | `portal.person.id` |
| conta de jogo | GM logado no cliente 7662 usa comando | `game.account.id` |
| processo | dreno de entrega marca o pedido como entregue | nome do processo, sem id |

Uma coluna `TEXT` com prefixo (`portal:42`) resolve os três em 1 coluna, mas toda consulta vira
parse: "o que o staff 42 fez?" precisa `split`/`substring` e cast para `bigint`, e um typo
(`porta:42`, `portal: 42`) só é pego por regex de CHECK — que não é índice. Duas colunas de FK
(`actor_person_id`, `actor_account_id`) dão integridade real, mas obrigam uma escolha ruim: ou
`ON DELETE RESTRICT` (fechar conta passa a ser bloqueado por uma linha de auditoria, contradizendo o
tombstone de `A12`) ou `ON DELETE SET NULL` (a auditoria perde justamente a atribuição que a motivou).

A proposta M18-A é o meio: `actor_kind TEXT` + `actor_id BIGINT` + `actor_ref TEXT`, com
`CHECK ((actor_kind = 'system') = (actor_id IS NULL))` e `CHECK ((actor_kind <> 'system') = (actor_ref
IS NULL))`. Consulta por ator é `WHERE actor_kind = 'person' AND actor_id = 42` (índice
`ix_audit_actor (actor_kind, actor_id, occurred_at DESC)`), e a ausência de FK é **declarada**:
auditoria sobrevive ao ator e nunca bloqueia nem cascateia (mesma regra de `P-16`). Custo aceito:
não há rótulo congelado — o nome de quem agiu é lido por join no instante da consulta. Isso só é
honesto porque **pessoa e conta nunca são apagadas** (`A12` fecha com tombstone; conta fecha com
`closed_at` e a linha fica); se um dia alguém propuser `DELETE` nessas duas tabelas, este formato
quebra e a proposta tem de voltar à mesa.

**Decidido em 2026-11-17 (dono): fica a M18-A.** `ops.audit_log` nasce em `0002_ops_audit.sql` com
`actor_kind TEXT NOT NULL`, `actor_id BIGINT`, `actor_ref TEXT`, os três CHECKs de coerência e o
índice `ix_audit_actor (actor_kind, actor_id, occurred_at DESC)`. Sem FK — a linha de auditoria
sobrevive ao ator e nunca bloqueia fechamento nem cascateia. O que a decisão **não** compra, e fica
declaro no teste da onda dona: não existe rótulo congelado; o nome de quem agiu sai de join no
instante da consulta, e é por isso que `A12` (tombstone) e o fechamento com `closed_at` são
invariantes do desenho, não detalhe.

A decisão vale **só para o formato**. Quem grava `actor_kind`/`actor_id` em cada ação administrativa
é responsabilidade das ondas 3–4 (contrato de `repo/audit` e rotas do BFF), e toda ação nova que
não se encaixe em `person`/`account`/`system` volta aqui antes de virar `INSERT`.

### 2.4 Resumo por marco

| Marco | Lacunas bloqueadas | São elas |
|---|---|---|
| **P0** | 0 | — |
| **P1** | 0 | — |
| **P2** | 0 | — Refletida em `planejamento-1.0.0/wyd-db/docs/convencoes.md`. |
| **P3** | 0 | — |
| **P4** | 0 | — Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/index.md`. |
| **EXT** | 3 | `A11`, `A20`, `M21` |

`EXT` não é estado: é o marco das três entradas `bloqueadas` cuja execução pertence a uma frente fora
do `wyd-db` (o dono está na coluna **Dono** e a pendência, na coluna **O que falta**).

A tabela é o **saldo**, não o histórico: as **35** lacunas fechadas saíram dos marcos e continuam em
§2.1–§2.3 com a data e a evidência. Restam **3 bloqueadas por execução externa** e **0 abertas**
(`M33` fechou em 2026-11-23 com as seis `Q-M33`; `M32` fechou em 2026-11-23) — `A7` e `A21`
fecharam em 2026-11-20, `M21` teve a decisão de escopo registrada na mesma data e continua bloqueada por execução externa
porque a execução é do `wyd-portal`, e `M25` entrou e fechou no mesmo dia, com o desenho do lote
`0012`: o dreno usa `SKIP LOCKED` e a linha fechada é expurgada em 30 dias.

Ordem de execução que isso implica: `0001`/`0007` podem **congelar** — o DDL dos dois já reflete o
que os travava (`ddl-confirmado.md` §3 e §5.5), ressalvada a `H` de `game.character(id)` do `0006`,
declarada em `ddl-confirmado.md` §2 e §5.5 —, e a ADR de fronteira pessoa × conta pode ser escrita.
O que resta aberto não é decisão de produto nem desta série: `A11` é medição do cliente, `A20` é
execução da frente do portal sobre escopo já decidido e `M21` é a lista de telas do planejamento do
`wyd-portal`. O que ainda trava arquivo da série é **desenho de coluna** (§3 de
`migrations-checklist.md`), e nada mais: `M17` fechou em 2026-11-19 e `M25` em 2026-11-20, com o
lote `0012` aprovado.

**Enunciação (2026-11-18):** a **decisão** de cada item fechado vive na página dona — `M7` e `M23` em
`docs/guia-desenvolvedor/como-adicionar-tabela.md` §2, as demais onde a própria linha aponta — e é
lá que o critério se enuncia. Aqui fica o **registro** (id, estado, dono, prazo, o que bloqueia), que é
o que os índices e as ADRs citam; item aberto segue como bloqueio, citado pelo ID. Regra muda lá;
estado e dono mudam só aqui.

---

## 3. Divergências do plano diretor (11)
**Sem arquivo no repositório:** `migrate.md`, `model-account.md`, `provision.md`, `repo-accounts.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Todas em `planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md`. Fecham **numa passada só** (marco `P2`),
não em 11 patches — o motivo é `D-A` abaixo.

| ID | Onde | Divergência | Dono | O que precisa virar | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|---|
| **P1** | §7.1 | dizia "4 linhas" e listava 5 campos de cabeçalho | arquitetura | **decidido em 2026-11-19: 5 campos** — `Pergunta`, `Público`, `Dono`, `Base`, `Revisado em`; o plano e `docs/convencoes.md` já usam esse número | toda página nasce ambígua | **P2** | fechada |
| **P2** | §7.3/§10 | exige Mermaid que a config não habilitava | arquitetura | resolvido no desenho; a config do site é item do `wyd-docs` (`EXT`) — `custom_fences` obrigatório quando o site for montado | — | — | **fechada** Refletida em `planejamento-1.0.0/wyd-db/docs/convencoes.md`. |
| **P3** | `docs/convencoes.md` | manda defeito para `planos/riscos.md`, que não existe | arquitetura | **decidida em 2026-11-20: `docs/planos/riscos.md` criado na mesma data, dono arquitetura**, com defeito de página e risco no mesmo registro | defeito sem dono até a onda 6 | **P2** | fechada |
| **P4** | §4, §6.4, §11 onda 9 | "ADRs (14)" × §6.4 com 12 decisões | arquitetura | **decidido (2026-11-18):** catálogo de 12 ADRs removido da árvore e de §6.4; ADR nasce na ordem em que a decisão é tomada — hoje `ADR-DB-0001` (aceito) e `ADR-DB-0002` (runner, `aceito`); onda 9 conta só as existentes | árvore de §4, §6.4, onda 9 | **P2** | **fechada** (2026-11-18) Refletida em `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/index.md`. |
| **P5** | §12 `D-1` | escopo = schema aplicado hoje (`0053`) | produto | **decidido em 2026-11-19:** escopo = série reescrita do zero, identidade primeiro (`D-1` aprovada) | a decisão que o modelo novo derruba | **P2** | fechada Refletida em `planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md`. |
| **P6** | §1.2 `D-1`, §4, §6.8, §6.15, §11, §12 `D-1`/`D-2`/`D-4` | números da série herdada **e** da série do rascunho: "87 geradas", "53 páginas", "262 + 88", "50 arquivos", "90 tabelas", "`SCHEMA_VERSION = 50`" | arquitetura | **fechada em 2026-11-18:** fonte única `plano-migrations.md` §3 — **29 arquivos, 92 tabelas**, `SCHEMA_VERSION = 29` na troca (hoje **31 / 97** e `SCHEMA_VERSION = 31`, desde os adendos `0030`/`0031` aprovados em 2026-11-20). O plano diretor **cita**, não digita; o medido entra só depois da série. Os nomes de página fecharam em `P8` (2026-11-18) | catálogo, dicionário, `metricas.md` | **P2** | **fechada** |
| **P7** | §12 `D-3`, §6.16 | "20 páginas de módulo", `model-account.md`, `repo-accounts.md` | arquitetura | **fechada em 2026-11-18:** naquele estado §6.16 listava 26 páginas (as 24 da tabela do crate mais `migrate.md` e `provision.md`), `D-3` e a árvore de §4 diziam 26, e as sete que faltavam entraram na tabela (§6.16 é o dono da lista); `model-account`/`repo-accounts` não aparecem em página nenhuma. **Reaberta e refechada em 2026-11-18:** o dono pediu página por arquivo em `src/migrate/` e `src/provision/`; §6.16 passou a **58 páginas** (24 + 34) — medido no artefato: 24 linhas de tabela, 21 de §6.16.1 e 13 de §6.16.2 —, as duas páginas de módulo em camadas viraram índice e `D-3`/árvore foram revistas | onda 7 (`src/` com 58 páginas) | **P2** | **fechada** (2026-11-18) Refletida em `planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md` §6.16. |
| **P8** | §6.15 | slugs das 29 páginas sem nome aprovado — a série, a contagem e a ordem já estavam em `plano-migrations.md` §3; faltava o nome de cada página | arquitetura | **fechada em 2026-11-18: os 29 slugs aprovados, um por arquivo, derivados do nome do arquivo** (`0001_bootstrap.sql` → `0001_bootstrap.md`; regra: sem o prefixo numérico, `_` → `-`, `kebab-case` sem acento). Lista nominal: `0001_bootstrap`, `0002_ops_audit`, `0003_portal_person`, `0004_portal_access`, `0005_game_account`, `0006_game_character`, `0007_game_session`, `0008_game_skills`, `0009_game_inventory`, `0010_game_guild`, `0011_game_economy`, `0012_ops_delivery`, `0013_content_taxonomy`, `0014_content_item`, `0015_content_skill`, `0016-seed-taxonomy`, `0017-seed-item-catalog`, `0018-seed-skill-catalog`, `0019_content_world`, `0020-seed-world`, `0021_content_rules`, `0022_content_overrides`, `0023-seed-rules`, `0024_content_maps`, `0025_content_combat`, `0026_content_quests_events`, `0027-seed-quests-events`, `0028_game_quests_events`, `0029_views`; tabela arquivo × página em `wyd-db-1.0.0.md` §6.15 | escrita das 29 páginas e de `migrations/index.md`, que reproduz a coluna **arquivo** de `wyd-db-1.0.0.md` §6.15, não a lista de páginas (onda 5) | **P2** | **fechada** (2026-11-18) |
| **P9** | §4 | árvore lista `0001-....md … 0053-....md` | arquitetura | **fechada em 2026-11-18:** a árvore mostra `0001-….md … 0029-….md (29)`, mais `index.md` e `serie-e-regras.md` — 31 em `migrations/`. Lista **nominal de página** não é trabalho de `P9`: os slugs saíram em `P8` (2026-11-18) e §6.15 manda `migrations/index.md` reproduzir a lista de **arquivos** de `plano-migrations.md` §3 | árvore publicável | — | **fechada** (2026-11-18) |
| **P10** | §6.8 | 18 domínios, `dominio-acesso.md` sem as tabelas novas | arquitetura | **fechada em 2026-11-18:** `dominio-acesso.md` escrito com as **10** tabelas (3 de `0003`, 2 de `0004`, 4 de `0005` e `game.session`), estado por tabela (`planejado` × `provisório`) e as 6 seções de §6.8; ficam fora desta lacuna os `gerado/<tabela>.md` (esperam o gerador) e os IDs `RF-DB`/`RN-DB` (onda 3) | dicionário do domínio | **P1** | **fechada** |
| **P11** | §4 totais | "262 autorais + 88 geradas" | arquitetura | **fechada em 2026-11-18:** os totais derivados da série (92 geradas, 31 em `migrations/`) citam `plano-migrations.md` §3 com data, e o teto autoral/medido fica com `planos/metricas.md`; as strings "262"/"88" não existem mais no plano diretor | contagem de aceite | **P2** | **fechada** |

`B4`/`B5` **não** mexem em nenhum destes números: fecharam sem tabela nova, então o plano não ganhou
uma divergência por causa delas. Dos números falsos, `P6` fechou em 2026-11-18 (50/90/50 → 29/92/29,
fonte `plano-migrations.md` §3) e `P11` fechou no mesmo dia, ao apontar os totais de §4 para a mesma
fonte. `P10` era página, não número: foi escrita em 2026-11-18 e fechou no mesmo dia; `P9` fechou junto,
quando a árvore deixou de listar `0053`, e `P8` fechou no mesmo dia com os 29 slugs aprovados — nomes,
não contagem. O grupo de números e páginas não deixa pendência.

---

## 4. Achados estruturais (3)

Estes três não são lacunas de conteúdo: são **defeitos de estrutura do planejamento** que fazem as
outras 40 linhas parecerem resolvidas quando não estão.

### `D-A` — dois sistemas de ondas concorrentes

| Campo | Conteúdo |
|---|---|
| **O que é** | O plano diretor §11 tem **9 ondas de documentação**; a trilha de código tem outra numeração — `plano-migrations.md` §9, "Ordem de execução" (passos 1–7). Nenhuma das duas se refere à outra. |
| **Por que aconteceu** | O plano foi escrito para documentar um schema existente; o rascunho foi escrito para **produzir** o schema. Cada documento numerou a própria sequência com o vocabulário que tinha. |
| **O que quebra** | (1) Nenhuma onda do plano escreve o schema, mas a onda 4 exige **92 páginas geradas** dele; (2) "Onda 1" significa coisas diferentes em dois documentos irmãos; (3) a ordem de execução não existe em lugar nenhum. |
| **Dono** | arquitetura |
| **Decisão necessária** | **decidido em 2026-11-19:** **Onda** = documentação, **Etapa** = código; timeline única em `planos/plano-de-documentacao.md` (onda 8) com o ponto de junção (Etapa 1 → Onda 3). |
| **O que bloqueia** | Ondas 1–2 de documentação, leitura do §11, `metricas.md`, e a resposta a "por onde começo". |
| **Prazo** | **P2** |
| **Estado** | fechada |

### `D-B` — o gate de link não prova coerência

| Campo | Conteúdo |
|---|---|
| **O que é** | `planejamento-1.0.0/wyd-db/mkdocs.yml` tem `validation.links` com `not_found`, `anchors` e `absolute_links` em `ignore`. Resultado: `mkdocs build --strict` fica verde com link quebrado. |
| **Medição** | **35 citações** a **4 arquivos inexistentes** (`planos/metricas.md`, `planos/riscos.md`, `specs/restricoes-e-premissas.md`, `specs/rastreabilidade.md`), sendo **27 no plano diretor**. |
| **Por que aconteceu** | O `ignore` foi posto para o build passar enquanto a árvore está 95% vazia — decisão razoável na hora, e por isso o verde de hoje é falso. |
| **O que quebra** | A prova de "documentação coerente" no aceite (`wyd-db-1.0.0.md` §10) e a regra "uma informação, um dono": link para arquivo que não existe é a informação que não tem dono. |
| **Dono** | arquitetura |
| **Decisão necessária** | **Decidida em 2026-11-17:** remover a config do diretório de planejamento. O `--strict` deixa de ser gate do `wyd-db`; quando o site do `wyd-docs` for montado, `validation.links` entra em `warn` (nunca `ignore`) e os 4 arquivos são criados na onda dona. |
| **O que bloqueia** | nada do `wyd-db`; bloqueia o aceite de `--strict` do site do `wyd-docs` (`EXT`). |
| **Prazo** | **EXT** |
| **Estado** | **fechada** |

**Evidência:** `planejamento-1.0.0/wyd-db/mkdocs.yml` removido em 2026-11-17 por decisão do dono. Nada o consumia — `make docs-check` roda `tools/dev/check-docs-consistency.mjs` (links de README/AGENTS + contagem de crates) e `make docs-serve` lê `wyd-docs/mkdocs.yml` (apagado do disco, decisão de manter assim por enquanto). O defeito medido era real e desaparece com o arquivo, mas as citações a 4 arquivos inexistentes continuam pendência **de conteúdo** (`planos/metricas.md`, `planos/riscos.md`, `specs/restricoes-e-premissas.md`, `specs/rastreabilidade.md`), a fechar nas ondas donas — não é mais falha de gate. O build do site do `wyd-docs` é item daquela frente (`EXT`).

---

### `D-C` — o estado de tabela tinha três donos e era digitado à mão

| Campo | Conteúdo |
|---|---|
| **O que é** | `pronto`/`provisório`/`planejado` (definidos em `docs/convencoes.md`) apareciam digitados por tabela na página de domínio; o `ddl-confirmado.md` chamava de "tabelas **prontas**" as 6 com DDL fechado em arquivo aberto — palavra que `convencoes.md` reserva para "aplicado no ledger" —; e o checklist usava um quarto vocabulário (`pronta`/`parcial`/`bloqueada`/`concluída`) para o **arquivo**. Três listas de estado para o mesmo objeto, e a primeira já divergia no nome da palavra. |
| **Por que aconteceu** | Cada documento nasceu respondendo à própria pergunta (o DDL fechou? a página foi escrita? a tabela está pronta?), e "pronto" é a palavra natural nos três casos. |
| **O que quebra** | A regra "uma informação, um dono": as quatro tabelas de `0005` (arquivo aberto por `A17`/`M16`) eram `provisório` na página de domínio e "prontas" no `ddl-confirmado`, e a próxima página de domínio copiaria uma das duas. |
| **Dono** | arquitetura |
| **Decisão necessária** | **Decidida em 2026-11-18:** o estado é **derivado**, não digitado. Dono único da derivação: `migrations-checklist.md` §1.2 (estado do arquivo → estado das tabelas). `docs/convencoes.md` fica só com o significado das três palavras; `ddl-confirmado.md` fala apenas de DDL fechado × aberto e trocou "tabelas prontas" por "tabelas com DDL fechado"; página de domínio cita a linha do arquivo e não tem coluna de estado. |
| **O que bloqueia** | nada: `dominio-acesso.md` já foi ajustado, e as 17 páginas de domínio seguintes nascem sem a coluna. |
| **Prazo** | **P1** |
| **Estado** | **fechada** |

**Evidência:** `migrations-checklist.md` §1.2 (tabela de derivação), `docs/convencoes.md` (significado + ponteiro), `ddl-confirmado.md` §1 ("tabelas com DDL fechado", 6 e 14), `docs/dicionario-dados/dominio-acesso.md` (sem coluna de estado, cita §1.2).

---

## 5. O que não fazer enquanto isto estiver aberto

1. **Não congelar `0001_bootstrap.sql` com outro DDL.** `A9`/`A12` fecharam em `D13`/`D14` (trigger
   `set_updated_at` e tombstone de fechamento) e o DDL já reflete os dois (`ddl-confirmado.md` §3 e
   §5.1); arquivo sem eles é a versão anterior, não uma variante.
2. **Não escrever `planos/metricas.md` com número medido** — `M20` fechou em 2026-11-18 e `D-A` em
   2026-11-19. Hoje o
   número honesto é "planejado: 29/92/29" (`plano-migrations.md` §3); medido só existe depois da série.
3. **Não aplicar os 11 `P` de §3 em 11 commits.** `P1`–`P11` mais os achados `D` fecham numa passada,
   com a timeline única já escrita.
4. **Não recriar config de site no diretório de planejamento.** O `mkdocs.yml` daqui foi removido em 2026-11-17 (`D-B`); o build `--strict` é item do `wyd-docs` (`EXT`). Enquanto não existir, "build verde" não é critério de pronto de nada.
5. **Não abrir a frente `wyd-portal`** (`M21`) nem a captura do cliente 7662 (`A11`) por causa deste
   registro: são `EXT`, com dono próprio.

---

## 6. Regra de manutenção

- Fechar uma linha **exige evidência**: arquivo e seção onde a decisão está, ou número medido com o
  comando e a data. Linha sem evidência não fecha.
- Fechar uma linha **não apaga a linha**: muda o estado e preenche "evidência". O histórico de
  decisão do projeto é a coluna de estado, não um changelog paralelo.
- Lacuna nova entra com o próximo `B`/`A`/`M` livre e passa por este arquivo **antes** de virar
  trabalho — é o que impede a volta das 6 causas raiz.
- Este registro não substitui o ADR (decisão formal). Ele é o **índice** da decisão.

---

*Registro. Nenhum arquivo além deste foi criado ou alterado nesta etapa.*
---

## 7. Bloco de decisão das lacunas bloqueadas

Cada item abaixo responde à mesma pergunta: **quem decide, o que exatamente se decide, o que acontece em
cada opção e qual artefato fecha a linha**. As três recomendações abaixo **foram aceitas pelo dono em 2026-11-25 na opção A de cada bloco** (`AGENTS.md` §5, item 8), proposta da arquitetura. O estado da linha
em §2 não muda até o artefato de fechamento existir.

**A11 — charset do `GameLogin`.**
- **Dono da decisão:** frente `wyd-server` (cliente 7662), com arquitetura; o fato a medir entra em
  `planejamento-1.0.0/wyd-client/registro-do-cliente.md`.
- **Pergunta fechada:** o campo de identificação do `GameLogin` do build 7662 aceita apenas
  `[A-Za-z0-9_]` (1–16 bytes) ou também espaço e caractere acentuado?
- **Opções:** (A) **ASCII estrito** — cadastro e login recusam o resto com erro tipado e a borda de
  teste usa esse alfabeto; (B) **bytes 1–16 sem validação** — aceita hoje (`H` com `A11`), mas nome com byte fora do alfabeto entra no banco e quebra comparação sem erro visível.
- **Recomendação:** A, porque o campo já tem 16 bytes e whitelist é a única regra que o teste consegue
  provar; "permissivo" sem critério vira dívida.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — ASCII estrito `[A-Za-z0-9_]` em 1–16 bytes, recusando o resto com erro tipado; até a captura provar o contrário, o campo continua permissivo e marcado `H`.
- **Artefato que falta (o que fecha):** captura `G-LOGIN` de um nome com acento, segunda leitura e golden registrados pelo
  processo de `planejamento-1.0.0/wyd-protocol/docs/goldens/como-capturar.md`, com a linha `A11`
  citando o golden, o CHECK do DDL e o teste de borda.

**A20 — segundo fator do staff.**
- **Dono da decisão:** produto + arquitetura; o registro da decisão é o ADR de segurança do portal.
- **Pergunta fechada:** o portal pode ir a público com staff sem segundo fator, ou o MFA de staff é
  pré-requisito de publicação?
- **Opções:** (A) **MFA obrigatório antes de publicar** — `person_authenticator`, fluxo de inscrição e
  teste que reprova login de staff sem segundo fator; custo: a publicação espera o fluxo; (B)
  **publicar com staff sem MFA e exceção datada** no registro de risco; custo: a conta de staff vira
  alvo único e a exceção precisa de prazo e dono.
- **Recomendação:** A, porque a decisão de 2026-11-18 já fixou a obrigação e publicar antes inverte a
  ordem que ela estabeleceu.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — MFA obrigatório para staff **antes** de o portal ir a público; a linha continua `externa` (não houve reclassificação e o placar não muda).
- **Artefato que falta (o que fecha):** ADR de MFA de staff aceito, teste `staff sem segundo fator é recusado` no portal e a
  linha `A20` citando o ADR.

**M21 — telas do portal.**
- **Dono da decisão:** frente `wyd-portal` (produto); as rotas já escritas são de
  `planejamento-1.0.0/wyd-portal/docs/arquitetura/rotas-e-permissoes.md` e as fatias de
  `planejamento-1.0.0/wyd-portal/docs/roadmap/`.
- **Pergunta fechada:** qual é o critério de fechamento — todas as rotas do mapa com tela implementada
  e teste de ponta a ponta, ou um subconjunto publicado na 1.0.0?
- **Opções:** (A) **fechar por fatia, rota a rota** — cada rota de `rotas-e-permissoes.md` ganha tela,
  teste E2E e evidência de execução na fatia que a entrega; custo: as 12 fatias do portal precisam
  rodar; (B) **fechar só o portal público e mover a administração para 1.1**; custo: operação sem
  painel e RF administrativo sem tela, contra o escopo decidido em 2026-11-20.
- **Recomendação:** A, porque mapa e fatias já existem e o que falta é execução, não outra decisão de
  escopo.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — fechar por fatia, rota a rota (as 50 rotas de `rotas-e-permissoes.md`); a linha continua `externa` e o placar não muda.
- **Artefato que falta (o que fecha):** telas das fatias `WYDPOR-F0`–`F11` implementadas, cada rota com teste Playwright
  verde e relatório de execução citado na linha `M21`.

## 8. Alvo `runner_pg` — decidido, implementado e verde (2026-11-25)

O alvo `runner_pg` (apply/status/down pelo `PgAdapter` real contra PG17) está declarado no
`Cargo.toml`, escrito em `crates/wyd-db/tests/runner_pg.rs` e passou **5/5** em 2026-11-25 (log
`/tmp/wyd-runner-pg.log`), fechando `CAP-37`.

- **Decisões (`2026-11-25`):** `Q-ALVO-1` = **A1** (porta em `127.0.0.1`); `Q-ALVO-2` = **B1** (fixture
  contígua agora); `Q-ALVO-3` = **`wyd_migrator` com `LOGIN` só no ambiente de teste**.
- **Fixture:** `tests/resources/fixture/` com 4 arquivos contíguos e `tests/resources/fixture_falha/`
  com o caso que falha no meio do arquivo.
- **O que ele prova:** ledger real, `apply` idempotente, transação por arquivo, `search_path = ''`,
  drift de checksum, `down` até o vazio e `LockBusy` em 10 s com duas sessões (`CAP-37`).
- **Achado corrigido na implementação:** `PgAdapter::close` fechava o `lock_pool` com a conexão do
  lock ainda presa e travava com `max_connections(1)`; a conexão passa a voltar ao pool antes do
  `close`.
- **O que continua fora:** `CAP-38` (p95 da leitura de personagem, `RNF-DB-004`) é caminho de
  consulta; a série real só roda quando as seis versões ausentes fecharem.
- **Alvo documentado:** [`tests/alvos/runner-pg.md`](tests/alvos/runner-pg.md).

**`Q-SEP-1` = A′ (2026-11-25), aplicada:** os 28 arquivos foram renomeados para
`NNNN_slug_com_underscores.sql` e as páginas de `migrations/` junto; os nomes parseiam e o `Slug`
continua estrito. Detalhe em `migrations/serie-e-regras.md` §9.1; nada do parser foi alterado.

# Lacunas, divergências e achados estruturais — registro único

> **Estado: registro vivo.** Este é o **único** lugar onde uma lacuna do `wyd-db` 1.0.0 tem dono,
> decisão pendente, bloqueio e prazo. Quando dois documentos divergirem, vale este registro, com a
> evidência anexada na linha.
>
> **O que este arquivo cobre:** as **29 lacunas** registradas (5 bloqueantes `B`, 15 altas `A`, 9
> médias `M`), as **11 divergências** internas do plano diretor (`P1`–`P11`) e os **3 achados
> estruturais** desta rodada (`D-A`, `D-B`, `D-C`).
>
> **O que este arquivo não é:** não decide nada por conta própria. Cada linha diz **quem** decide e
> **o que** precisa ser decidido; a decisão entra no rascunho, no ADR ou no plano, e volta aqui como
> uma linha fechada com data.
>
> Medido em **2026-11-17** e recontado em **2026-11-18**, quando `A17`–`A21` entraram (§2.2).
> Nenhum teste ou Postgres foi executado: as contagens vêm de `grep` nas migrations do legado e da
> lista nominal do §11 do rascunho.

---

## 0. Como ler

### 0.1 Estados

| Estado | Significado |
|---|---|
| **aberta** | sem decisão; a linha dona não pode ser escrita até fechar |
| **proposta** | existe recomendação técnica registrada; falta o "sim" do dono |
| **fechada** | decidida e **já refletida** num arquivo citado na coluna "evidência" |
| **externa** | decidida, mas a execução depende de frente fora do `wyd-db` (cliente, servidor, portal) |

### 0.2 Prazos (marco, não data)

Não há calendário: a ordem do projeto é por **dependência**, não por semana. Prazo é o marco antes
do qual a lacuna precisa estar fechada.

| Marco | Significado |
|---|---|
| **P0** | antes de **congelar** `0001_bootstrap.sql` / `0007_game_session.sql` — muda DDL ou contagem de tabelas |
| **P1** | antes da **ADR de fronteira pessoa × conta** (sem número ainda) e do `planos/metricas.md` — fecha desenho, não muda tabela |
| **P2** | antes da **reconciliação do plano diretor** — a passada única de `P1`–`P11` (§3) sobre `wyd-db-1.0.0.md`; não é passo da ordem de construção de `plano-migrations.md` §9 |
| **P3** | antes da **onda de documentação** que registra o assunto |
| **P4** | antes da **onda de código** dona do assunto — não bloqueia schema nem doc de identidade |
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

| Grupo | Quantas | Abertas (`aberta`/`proposta`) | `externa` | Fechadas |
|---|---:|---:|---:|---:|
| Lacunas (`B`/`A`/`M`) | 29 | 14 (`B1`, `B3`, `A7`, `A8`, `A13`, `A14`, `A17`, `A18`, `A19`, `A21`, `M16`, `M17`, `M19`, `M22`) | 3 (`A11`, `A20`, `M21`) | 12 (`B2`, `B4`, `B5`, `A9`, `A10`, `A12`, `A15`, `A16`, `M18`, `M20`, `M23`, `M24`) |
| Divergências do plano diretor (`P`) | 11 | 3 (`P1`, `P3`, `P5`) | — | 8 (`P2`, `P4`, `P6`, `P7`, `P8`, `P9`, `P10`, `P11`) |
| Achados estruturais (`D-A`, `D-B`, `D-C`) | 3 | 1 (`D-A`) | — | 2 (`D-B`, `D-C`) |

As decisões abaixo entram aqui só pelo **índice**: o texto que as descreve mora no arquivo dono. A
revisão de 2026-11-17 comparou o bloco de identidade com OWASP (Authentication, Password Storage,
Session Management, Forgot Password, MFA), NIST SP 800-63B-4 e os schemas públicos de Supabase
Auth/GoTrue, Ory Kratos e Keycloak. `M1`–`M7` e `M9` foram aprovados, `M8` deferido; nenhuma decisão
cria tabela nova além das duas já previstas em `P-22`, e a série continua em
**91 tabelas** (`plano-migrations.md` §3).

| ID | Decisão | Por que | Refletida em |
|---|---|---|---|
| **M1** | `game.account.status` fica `active`/`closed`; `suspended` sai do CHECK | bloqueio disciplinar é **só** `game.account_moderation.blocked`; duas fontes para o mesmo estado era o antipadrão que `padrao-banco.md` §5 (item 5) e §11 proíbem | `ddl-confirmado.md` §5.2 |
| **M2** | `game.pin_reset` perde `delivered_to` e ganha `requested_ip` | e-mail dentro de `game` fura o `P-22` e o argumento de `pg_dump -n game` sem PII; o destinatário vem de `portal.person` por join | §5.2 |
| **M3** | `game.account` ganha `login_failure_count`, `login_locked_until`, `password_changed_at`, `updated_at` e trigger | o PIN tinha contador de falha e a senha de jogo (12 bytes no fio) não tinha nenhum; NIST/OWASP exigem limite em **todo** autenticador | §5.2 |
| **M4** | sessão do portal com expiração **absoluta** (`expires_at`) e **idle** (`last_seen_at`), `user_agent` ≤ 512, índice de expiração; `person.password_changed_at` e `revoke_all` na troca de senha | OWASP Session Management pede os dois timeouts; encerrar as sessões na troca de senha é requisito do fluxo de recuperação | §4, §5.1 |
| **M5** | `portal.person_password_reset` vira `portal.person_token`, com `purpose` (`email_verify`/`password_reset`/`email_change`), `attempt_count`, `requested_ip` e índice único parcial "um vivo por propósito" | OWASP Forgot Password: uso único, TTL, limite por conta e invalidação do anterior ao reemitir. O `purpose` com CHECK é o que impede um token genérico de servir a cerimônias diferentes | §4 |
| **M6** | `ops.login_attempt` guarda `identifier_hash` (sha256), `surface`, `ip INET` e `result` fechado | `ops` é lido por `wyd_read`: e-mail/login crus ali seriam PII fora de `portal`; o hash ainda responde "quantas falhas neste identificador por hora" | §5.4 |
| **M7** | entram `portal.person_profile` e `portal.person_consent`, enxutas; **`P-22` vence o `D5`** | PII em `portal` e prova de consentimento **com versão** do documento aceito (LGPD/GDPR); sem tabela não há onde guardar nome de exibição sem sujar `person` | §5.6, §5.7 |
| **M9** | senha do portal **15–128**, sem regra de composição, sem expiração periódica, com blocklist; `argon2id m=19456, t=2, p=1` | NIST 800-63B-4 e OWASP pedem comprimento e blocklist, não "maiúscula+número+símbolo"; 10 caracteres era abaixo do mínimo vigente sem MFA | `B3` (metade da senha) |
| **M8** | MFA **não** entra na 1.0.0 (**deferido em 2026-11-17**); a forma fica reservada para `portal.person_authenticator` | sem consumidor medido (`D5`), mas staff `admin`/`moderator` precisa de segundo fator antes de o portal ir a público | `A20` (escopo e prazo, `externo`) |

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
| 2026-11-18 | contagens da série: **29/91/29** planejados; medido só do catálogo, depois da série | `plano-migrations.md` §3 |
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
| `M20` | coluna **decisão necessária** da própria linha em §2.3 (fechada em 2026-11-18) |
| `M24` | `src/provision/spec.md` §3 (variáveis e quem lê) e §7 (segredo) — a linha de §2.3 só registra e aponta |

Nenhuma das oito muda a contagem da série.

**O que a aprovação não compra:** nada disso foi executado — nenhum `.sql`, nenhum `crates/`, nenhum
Postgres; o efeito é só DDL de planejamento em `ddl-confirmado.md` e nas páginas de `migrations/`.

**O que continua aberto no plano diretor:** `P1` (campos de cabeçalho), `P3` (`planos/riscos.md`) e
`P5` (escopo greenfield × ETL) — §3.

---

## 2. Lacunas (29)

### 2.1 Bloqueantes

| ID | O que falta | Dono | Decisão necessária | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|
| **B1** | Envio de e-mail não existe: os dois resets emitem link e nada entrega | produto + arquitetura | provedor; nome da variável de segredo; template de cada fluxo; TTL do link; limite de reenvio (proposta: 3/h por pessoa); remetente | `portal.person_token` (`password_reset`/`email_verify`)/`game.pin_reset` em uso, prova S12, onda 6, `A7` | **P2** | aberta |
| **B2** | Série nova sem número, ordem e tamanho | arquitetura | enumeração fechada antes da onda 1; `SCHEMA_VERSION` concreto | numeração contígua, ledger, página por migration | — | **fechada** |
| **B3** | Política de credencial: **restam só o PIN e o limiar** — a senha foi decidida em `M9` (2026-11-17) | produto + arquitetura | senha **15–128**, sem composição, sem expiração periódica, com blocklist, `argon2id m=19456, t=2, p=1` (**decidido**); **falta** PIN 4–6 dígitos e 5 falhas / 10 min com desbloqueio por tempo | `src/security/policy.rs`, S7/S8 exatos, §política da ADR de fronteira pessoa × conta (sem número) | **P1** | proposta |
| **B4** | Pessoa suspensa sem autor, motivo ou instante | arquitetura | 3 colunas na própria pessoa (`suspended_reason`, `suspended_by`, `suspended_at`) + CHECK de coerência; histórico no `audit_log` | auditoria de staff, S17, `dominio-acesso.md` | — | **fechada** |
| **B5** | Multibox por IP sem onde morar | arquitetura + `wyd-server` | `game.session.ip`; IP é auditoria, nunca identidade | `docs/wyd/multiconta.md` §"Controles recomendados", S18, M22 | — | **fechada** |

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

`released_reason` (que acompanhava o `ip`) foi **retirada** pelo mesmo critério: auditar multibox é
contar abertura por IP/hora, e isso não precisa do motivo do encerramento. Quem decide se o motivo
vira coluna é `A10`, que já é o dono de "quem libera a sessão e como provar".

### 2.2 Altas

| ID | O que falta | Dono | Decisão necessária | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|
| **A7** | Cadastro de pessoa não existe (rota, regra, verificação) | produto | autocadastro × criado por staff; `pending` exige clique no e-mail? | rota de cadastro do §9, teste "cadastro cria pessoa + conta", onda 6 | **P4** | aberta |
| **A8** | `failed_logins`/`locked_until` existem e ninguém os move | produto + arquitetura | mesmo limiar do PIN (5/10 min)? desbloqueio só por tempo × staff? | login do portal (onda 4), prova de lockout | **P1** | proposta |
| **A9** | `updated_at` não se atualiza sozinho | arquitetura | **decidido:** trigger `set_updated_at()` criado uma vez em `0001`, uma função para a série | DDL de `0001`, teste que prova a coluna andando | — | **fechada** |
| **A10** | Sessão órfã de jogo: quem libera | `wyd-server` + operação | **fechada em 2026-11-18:** canal libera no disconnect; reaper 60 s com `stale 180 s`; **sem `released_reason`** — o motivo é evento e vive em `ops.audit_log` (`actor_kind = 'system'`), não em coluna da sessão | `ddl-confirmado.md` §5.5, contrato `release` do `repo/sessions`, S9 | **P1** | **fechada** |
| **A11** | Charset do login é UNVERIFIED | frente `wyd-server` (cliente 7662) | capturar no cliente; até lá manter permissivo (1..16, sem espaço nas bordas) e marcar `H` | fechar `GameLogin`, onda 5 | **EXT** | externa |
| **A12** | E-mail de conta fechada | produto + suporte | **decidido:** índice total + anonimização no fechamento (`closed-<id>@removido.invalid`); sem janela de reuso e sem exceção de suporte | índice `ux_person_email`, DDL de `0003_portal_person.sql`, texto de suporte do §9 | — | **fechada** |
| **A13** | Fechar pessoa com contas ativas | produto + arquitetura | fechar tudo em transação (proposta) × recusar enquanto houver conta viva | porta `close_person`, S13, suporte | **P1** | proposta |
| **A14** | Personagem apagado × sessão viva | arquitetura | soft-delete derruba a sessão (proposta); se o motivo precisa ser registrado, depende de `A10` | semântica de `game.session.character_id`, `0007` | **P1** | proposta |
| **A15** | `player` × `member` | arquitetura | **sem rename**: `player` é papel de jogo, `member` é papel de portal | mapa de tradução banco/Rust/BFF/seed | — | **fechada** |
| **A16** | Transporte da sessão do portal: `jti` em claro no banco é bearer ou chave de busca? | arquitetura | **fechada em 2026-11-18: token opaco.** O cookie carrega o verificador e o banco guarda só `sha256` (64 hex) — sem JWT, sem `jti` em claro, sem segredo de assinatura. `M4` já exige um `UPDATE` por requisição (idle), então ler a linha pelo hash não é custo novo; o preço aceito é um `UPDATE` extra no login para gravar o hash | `portal.person_session.token_hash` (`ddl-confirmado.md` §4, página `0004` §6.1/§7/§9), `D7` | **P1** | **fechada** |
| **A17** | Conta de jogo pode ser transferida ou vendida entre pessoas — sem decisão registrada | produto + arquitetura | se a resposta for sim: titularidade em tabela no `0005` (objeto novo) ou instante da última troca como coluna em `game.account`, que reabre tabela fechada — só a forma tabela acrescenta objeto (`ddl-confirmado.md` §5.2 registra as duas formas) | fechar `0005` (página `0005-game-account.md`, onda 5) e o dicionário `dominio-acesso.md` (onda 4) | **P3** | aberta |
| **A18** | Cadastro de conta pode nascer dentro do jogo, além do portal — nenhum dono decidiu | produto | o cliente cria conta? se cria, rota e identidade no jogo; se não, só o portal (`A7` cobre o cadastro do portal) | ADR de fronteira pessoa × conta (a rota que a resposta exigir), onda 6 (rotas de cadastro); **não** muda `0005` — evidência e gatilho de revisão na nota abaixo | **P1** | aberta |
| **A19** | Loja e cash no `1.0.0`: se existem e onde moram | produto | entram em `portal`, ficam fora do schema, ou não existem | `0011` (10 tabelas sem coluna desenhada; `A19` é bloqueio declarado; página `0011-game-economy.md`, onda 5) e o dicionário `dominio-economia.md` (onda 4) | **P3** | aberta |
| **A20** | MFA fora da 1.0.0 (`M8`): quando e para quem o segundo fator existe | produto + arquitetura | **escopo e prazo decididos em 2026-11-18:** obrigatório para **staff** (`admin`/`moderator` do portal) **antes** de o portal ir a público; `member` fica fora da 1.0.0 e só volta em 1.1 com consumidor medido. Forma reservada: `portal.person_authenticator (person_id, kind, secret_encrypted, confirmed_at, last_used_step)`, TOTP como `kind` inicial; recuperação do segundo fator é fluxo de staff, nunca autoatendimento do titular | telas e rotas do portal (frente `wyd-portal`), onda 6 | **EXT** | **externa** |
| **A21** | Troca de e-mail da pessoa não tem fluxo nem decisão registrada | produto | a troca entra na 1.0.0? quem pode pedir; como o endereço novo é verificado; o que o endereço antigo recebe; se a troca encerra sessões como a senha (`M4`) | rota de troca de endereço e emissor do token `email_change` (frente `wyd-portal`, onda 6) | **P4** | aberta |

`A17`–`A19` entraram neste registro em 2026-11-18: já eram citadas como decisão aberta em
`plano-migrations.md` §5 e em `migrations-checklist.md` §3, sem linha aqui. `A21` entrou no mesmo dia,
ao auditar `0004` §10: o `purpose = 'email_change'` já estava no CHECK e nenhuma linha o cobria.

`A17` e `A19` ficam no marco `P3`, não no `P2`: cada uma trava a onda de documentação que registra o
assunto (`wyd-db-1.0.0.md` §11), e a reconciliação do plano diretor (marco `P2`) não é o que as fecha.

`A18` é `P1` porque a pergunta é de **fronteira**, não de tabela. **Evidência:** `game.account` não tem
coluna que registre onde a conta nasceu e `person_id` é `NOT NULL` (`ddl-confirmado.md` §5.2), então conta
criada dentro do jogo pressupõe a mesma pessoa do portal — e §5.2 nomeia só `A17` e `M16` como capazes de
mudar aquele bloco. Enquanto ninguém nomear a coluna ou tabela que a criação no jogo exigiria, a resposta
decide **rota** entre cliente e portal, assunto da ADR de fronteira. **Gatilho:** se a coluna aparecer, a
linha volta aqui, passa a mudar `0005` e o marco é `P3`, junto de `A17`/`A19`.

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
| **M16** | `game.account_wallet.coin` muda no jogo sem ledger | produto + arquitetura | ledger de `coin` × snapshot com auditoria amostral | contrato de `repo/wallet`, `dominio-economia.md` (onda 4) e a página `0005-game-account.md` (onda 5) | **P3** | aberta |
| **M17** | Chave de idempotência sem dono | arquitetura + `wyd-portal` | quem gera (BFF, proposta), formato, janela de replay (24 h) | `0011`/`0012` (idempotência de crédito e entrega) e as ações `accounts/donate*` da onda 4 | **P3** | aberta |
| **M18** | `audit_log.actor` sem formato | arquitetura | **decidido em 2026-11-17 (M18-A):** `actor_kind` (`person`/`account`/`system`) + `actor_id` + `actor_ref`, com 3 CHECKs de coerência e **sem FK, por desenho** (ver nota) | DDL de `0002_ops_audit.sql`, consulta de auditoria, índice `ix_audit_actor` | **P1** | **fechada** |
| **M19** | Seed sem senha de dev documentada | arquitetura | senha fixa `dev-only` documentada no próprio seed | gate da onda 2, teste manual | **P1** | proposta |
| **M20** | Contagens medidas, não estimadas | arquitetura | **fechada em 2026-11-18:** o **planejado** é **29 arquivos / 91 tabelas**, dono único `plano-migrations.md` §3; o **medido** sai do catálogo depois que a série existir e é o único número que pode entrar em `planos/metricas.md` | `planos/metricas.md` (não pode ser escrito antes) | **P2** | **fechada** |
| **M21** | Telas do portal não mapeadas | frente `wyd-portal` | cadastro, recuperação, lista de contas: quais telas existem | rotas do §9, onda 6, `planejamento-1.0.0/wyd-portal` (vazia) | **EXT** | externa |
| **M22** | `portal.person_session`, `game.session` e `ops.audit_log` crescem sem política | operação | retenção por tabela; a sessão liberada **é** a trilha de auditoria, não lixo | `planos/retencao-e-capacidade.md`, backup | **P3** | aberta |
| **M23** | `channel_capacity` e `channel_role` eram 1:1 com `server_channel`, sem ciclo de vida próprio | arquitetura | **fechada em 2026-11-18:** fundidas em `game.server_channel` (`max_players`, `maintenance`, `role`), com `ck_server_channel_jogavel` para `channel = 0` e sem `DEFAULT`; a separação só existia porque o legado proibia `ALTER TABLE` no `up` (`0040`/`0041`) | DDL de `0007`, contagem da série (93 → 91) | — | **fechada** |
| **M24** | Contrato de ambiente do `wyd-db`: quais variáveis o crate consome, com que nomes, e o que é segredo | arquitetura | **fechada em 2026-11-18:** o contrato inteiro vive em `src/provision/spec.md` §3 (variáveis e quem lê) e §7 (segredo) — esta linha só registra | implementação de `crates/wyd-db` (onda 1); gate de ambiente em `plano-migrations.md` §7 | **P4** | **fechada** |

`M16` e `M17` ficam no marco `P3`, não no `P4`: as duas bloqueiam **schema** — `M16` o bloco `0005`
(`ddl-confirmado.md` §5.2, uma das razões de o arquivo ser `parcial`) e `M17` os arquivos
`0011`/`0012` (`migrations-checklist.md` §3) —, o que `P4` exclui por definição; quem as fecha é a onda
de documentação que registra o assunto, como `A17`/`A19` (§2.2).

`M20` fecha em **91 tabelas** planejadas (`plano-migrations.md` §3). A derivação do rascunho —
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

| Marco | Lacunas abertas | São elas |
|---|---|---|
| **P0** | 0 | — |
| **P1** | 6 | `B3`, `A8`, `A13`, `A14`, `A18`, `M19` |
| **P2** | 1 | `B1` |
| **P3** | 5 | `A17`, `A19`, `M16`, `M17`, `M22` |
| **P4** | 2 | `A7`, `A21` |
| **EXT** | 3 | `A11`, `A20`, `M21` |

As 17 linhas acima são as lacunas **não fechadas**: **14** `aberta`/`proposta` mais **3** `externa`
(`A11`, `A20`, `M21`), a mesma partição da tabela de §1.

Ordem de execução que isso implica: **P0 está fechado** — o DDL de `0001` e de `0007` já reflete as
decisões que o travavam (`ddl-confirmado.md` §3 e §5.5), ressalvada a `H` de `game.character(id)` do
`0006` (declarada em `ddl-confirmado.md` §2 e §5.5), que segue decisão do dono e não é linha deste
registro —, então `0001`/`0007` podem **congelar**; depois **P1** (6 decisões, nenhuma mexe em
tabela), e só então a ADR de fronteira pessoa × conta pode ser escrita.
`P2` vem na reconciliação do plano; `P3`/`P4`/`EXT` não travam o corte vertical.

**Enunciação (2026-11-18):** a **decisão** de cada item fechado vive na página dona — `M7` e `M23` em
`docs/guia-desenvolvedor/como-adicionar-tabela.md` §2, as demais onde a própria linha aponta — e é
lá que o critério se enuncia. Aqui fica o **registro** (id, estado, dono, prazo, o que bloqueia), que é
o que os índices e as ADRs citam; item aberto segue como bloqueio, citado pelo ID. Regra muda lá;
estado e dono mudam só aqui.

---

## 3. Divergências do plano diretor (11)

Todas em `planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md`. Fecham **numa passada só** (marco `P2`),
não em 11 patches — o motivo é `D-A` abaixo.

| ID | Onde | Divergência | Dono | O que precisa virar | O que bloqueia | Prazo | Estado |
|---|---|---|---|---|---|---|---|
| **P1** | §7.1 | diz "4 linhas" e lista 5 campos de cabeçalho | arquitetura | 5 campos, como `docs/convencoes.md` já registra | toda página nasce ambígua | **P2** | aberta |
| **P2** | §7.3/§10 | exige Mermaid que a config não habilitava | arquitetura | resolvido no desenho; a config do site é item do `wyd-docs` (`EXT`) — `custom_fences` obrigatório quando o site for montado | — | — | **fechada** |
| **P3** | `docs/convencoes.md` | manda defeito para `planos/riscos.md`, que não existe | arquitetura | criar `planos/riscos.md` na onda que o plano já prevê | defeito sem dono até a onda 6 | **P2** | aberta |
| **P4** | §4, §6.4, §11 onda 9 | "ADRs (14)" × §6.4 com 12 decisões | arquitetura | **decidido (2026-11-18):** catálogo de 12 ADRs removido da árvore e de §6.4; ADR nasce na ordem em que a decisão é tomada — hoje `ADR-DB-0001` (aceito) e `ADR-DB-0002` (runner, `aceito`); onda 9 conta só as existentes | árvore de §4, §6.4, onda 9 | **P2** | **fechada** (2026-11-18) |
| **P5** | §12 `D-1` | escopo = schema aplicado hoje (`0053`) | produto | escopo = série reescrita do zero, identidade primeiro | a decisão que o modelo novo derruba | **P2** | aberta |
| **P6** | §1.2 `D-1`, §4, §6.8, §6.15, §11, §12 `D-1`/`D-2`/`D-4` | números da série herdada **e** da série do rascunho: "87 geradas", "53 páginas", "262 + 88", "50 arquivos", "90 tabelas", "`SCHEMA_VERSION = 50`" | arquitetura | **fechada em 2026-11-18:** fonte única `plano-migrations.md` §3 — **29 arquivos, 91 tabelas**, `SCHEMA_VERSION = 29` na troca. O plano diretor **cita**, não digita; o medido entra só depois da série. Os nomes de página fecharam em `P8` (2026-11-18) | catálogo, dicionário, `metricas.md` | **P2** | **fechada** |
| **P7** | §12 `D-3`, §6.16 | "20 páginas de módulo", `model-account.md`, `repo-accounts.md` | arquitetura | **fechada em 2026-11-18:** §6.16 lista as **26** páginas, `D-3` e a árvore de §4 dizem 26, e as sete que faltavam entraram na tabela (§6.16 é o dono da lista); `model-account`/`repo-accounts` não aparecem em página nenhuma. **Reaberta e refechada em 2026-11-18:** o dono pediu página por arquivo em `src/migrate/` e `src/provision/`; §6.16 passou a **58 páginas** (24 + 34), as duas páginas de módulo em camadas viraram índice e `D-3`/árvore foram revistas | onda 7 (`src/` com 58 páginas) | **P2** | **fechada** (2026-11-18) |
| **P8** | §6.15 | slugs das 29 páginas sem nome aprovado — a série, a contagem e a ordem já estavam em `plano-migrations.md` §3; faltava o nome de cada página | arquitetura | **fechada em 2026-11-18: os 29 slugs aprovados, um por arquivo, derivados do nome do arquivo** (`0001_bootstrap.sql` → `0001-bootstrap.md`; regra: sem o prefixo numérico, `_` → `-`, `kebab-case` sem acento). Lista nominal: `0001-bootstrap`, `0002-ops-audit`, `0003-portal-person`, `0004-portal-access`, `0005-game-account`, `0006-game-character`, `0007-game-session`, `0008-game-skills`, `0009-game-inventory`, `0010-game-guild`, `0011-game-economy`, `0012-ops-delivery`, `0013-content-taxonomy`, `0014-content-item`, `0015-content-skill`, `0016-seed-taxonomy`, `0017-seed-item-catalog`, `0018-seed-skill-catalog`, `0019-content-world`, `0020-seed-world`, `0021-content-rules`, `0022-content-overrides`, `0023-seed-rules`, `0024-content-maps`, `0025-content-combat`, `0026-content-quests-events`, `0027-seed-quests-events`, `0028-game-quests-events`, `0029-views`; tabela arquivo × página em `wyd-db-1.0.0.md` §6.15 | escrita das 29 páginas e de `migrations/index.md`, que reproduz a coluna **arquivo** de `wyd-db-1.0.0.md` §6.15, não a lista de páginas (onda 5) | **P2** | **fechada** (2026-11-18) |
| **P9** | §4 | árvore lista `0001-....md … 0053-....md` | arquitetura | **fechada em 2026-11-18:** a árvore mostra `0001-….md … 0029-….md (29)`, mais `index.md` e `serie-e-regras.md` — 31 em `migrations/`. Lista **nominal de página** não é trabalho de `P9`: os slugs saíram em `P8` (2026-11-18) e §6.15 manda `migrations/index.md` reproduzir a lista de **arquivos** de `plano-migrations.md` §3 | árvore publicável | — | **fechada** (2026-11-18) |
| **P10** | §6.8 | 18 domínios, `dominio-acesso.md` sem as tabelas novas | arquitetura | **fechada em 2026-11-18:** `dominio-acesso.md` escrito com as **10** tabelas (3 de `0003`, 2 de `0004`, 4 de `0005` e `game.session`), estado por tabela (`planejado` × `provisório`) e as 6 seções de §6.8; ficam fora desta lacuna os `gerado/<tabela>.md` (esperam o gerador) e os IDs `RF-DB`/`RN-DB` (onda 3) | dicionário do domínio | **P1** | **fechada** |
| **P11** | §4 totais | "262 autorais + 88 geradas" | arquitetura | **fechada em 2026-11-18:** os totais derivados da série (91 geradas, 31 em `migrations/`) citam `plano-migrations.md` §3 com data, e o teto autoral/medido fica com `planos/metricas.md`; as strings "262"/"88" não existem mais no plano diretor | contagem de aceite | **P2** | **fechada** |

`B4`/`B5` **não** mexem em nenhum destes números: fecharam sem tabela nova, então o plano não ganhou
uma divergência por causa delas. Dos números falsos, `P6` fechou em 2026-11-18 (50/90/50 → 29/91/29,
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
| **O que quebra** | (1) Nenhuma onda do plano escreve o schema, mas a onda 4 exige **91 páginas geradas** dele; (2) "Onda 1" significa coisas diferentes em dois documentos irmãos; (3) a ordem de execução não existe em lugar nenhum. |
| **Dono** | arquitetura |
| **Decisão necessária** | Renomear: **Onda** = documentação, **Etapa** = código. Uma timeline única em `planos/plano-de-documentacao.md` com as duas trilhas e o ponto de junção (Etapa 1 → Onda 3). |
| **O que bloqueia** | Ondas 1–2 de documentação, leitura do §11, `metricas.md`, e a resposta a "por onde começo". |
| **Prazo** | **P2** |
| **Estado** | aberta |

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
2. **Não escrever `planos/metricas.md` antes de `D-A`** (marco `P2`) — `M20` fechou em 2026-11-18. Hoje o
   número honesto é "planejado: 29/91/29" (`plano-migrations.md` §3); medido só existe depois da série.
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

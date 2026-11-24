> Pergunta: "o que o banco precisa fazer no bloco de identidade, e como provar cada requisito?"
> Público: Produto / gestão e arquitetura
> Dono: equipe do `wyd-db`
> Base: `rascunho-accounts.md` §2; `ddl-confirmado.md` §4, §5.1–§5.7, §5.16; `lacunas.md` §2; `migrations/0001-bootstrap.md` §4, §6 e §9; `migrations/0006-game-character.md`; `pendencias-de-fechamento.md` §6.6.26 (`Q-BLK-24` = A)
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-22)

Primeira leva de requisitos funcionais do `wyd-db`. O recorte é o **bloco de identidade** — os
`R1`–`R16` levantados em `rascunho-accounts.md` §2 — e nada além dele. Personagem, inventário,
economia e catálogo ainda não têm requisito levantado; entram nas ondas donas, com número novo.

Os IDs `RF-DB-001`–`RF-DB-016` nascem nesta página, na ordem de `R1`–`R16`, porque nenhum existia
antes (`docs/dicionario-dados/dominio-acesso.md` §Requisitos atendidos registrava a ausência). O
prefixo e o formato são de `wyd-db-1.0.0.md` §9.1. O número **não** é reutilizado depois de retirado
(§9.3). `RF-DB-017` nasce em 2026-11-24, pela decisão `Q-BLK-24` = A, e é o único bloco **fora** do
recorte de identidade: enuncia a versão corrente de conteúdo, que nenhum dos 16 anteriores dizia.

> [!NOTE]
> A coluna de **issue** está vazia de propósito: `D-7` foi aprovada em 2026-11-22, mas as 40 páginas
> `WYDDB-nnn` com DoD ainda não existem, e issue sem DoD não entra. Cada requisito aponta a onda que
> o entrega, e a issue nasce na onda 8, quando a página correspondente existir. Isto é recorte
> declarado, não pendência escondida.

## Como ler cada bloco

O critério de aceitação é sempre um comando contra a série aplicada em PG 17, com o dado de prova
declarado. O nome da constraint é o que o PostgreSQL devolve no erro: se o nome divergir, o defeito é
da página de `migrations/`, não do teste.

Nos exemplos, `person_id = 1` e `account_id = 1` são fixture de teste (uma pessoa e uma conta já
criadas). `$argon2id$...` representa um hash válido, nunca uma senha real.

## RF-DB-001 — login de conta de 1 a 16 caracteres

- **Enunciado:** a conta de jogo aceita login de 1 a 16 caracteres, sem espaço nas bordas, e o login
  dobrado (`login_fold`) é o que identifica a conta na busca.
- **Entrada:** `person_id`, `slot`, `login`, `password_hash`.
- **Saída:** linha em `game.account`, com `login_fold` gerada.
- **Critério de aceitação:** `INSERT INTO game.account (person_id, slot, login, password_hash)
  VALUES (1, 1, repeat('a', 17), '$argon2id$...')` falha com `23514` em `ck_account_login_len`;
  com 16 caracteres, insere.
- **Tabela e porta:** `game.account` (`migrations/0005-game-account.md`); porta `accounts` (planejada).
- **Issue:** a alocar na onda 8; `D-7` aprovada em 2026-11-22, página `WYDDB-nnn` ainda não escrita.

## RF-DB-002 — falha de PIN acumula e bloqueia

- **Enunciado:** PIN errado incrementa `pin_failure_count`; ao atingir o limiar, `pin_locked_until`
  recebe o instante de liberação e o estado sobrevive a reinício do processo.
- **Entrada:** `account_id` e PIN; a política é 4–6 dígitos, 5 falhas e 10 minutos (`B3`).
- **Saída:** contador e instante gravados na linha da conta.
- **Critério de aceitação:** após cinco falhas, `SELECT pin_failure_count, pin_locked_until FROM
  game.account WHERE id = 1` devolve `5` e instante não nulo; reiniciar o servidor e repetir a
  consulta mantém os valores. O limiar é de código: o banco só guarda o estado.
- **Tabela e porta:** `game.account` (`migrations/0005-game-account.md`); porta `accounts` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-003 — saldo é da conta, em inteiro de 32 bits

- **Enunciado:** a moeda pertence à conta, não ao personagem, e cabe em `i32` (`0`–`2147483647`).
- **Entrada:** `account_id` e `coin`.
- **Saída:** linha em `game.account_wallet`, uma por conta.
- **Critério de aceitação:** `UPDATE game.account_wallet SET coin = 2147483648 WHERE account_id = 1`
  falha com `23514` em `ck_account_wallet_coin`; `2147483647` passa.
- **Tabela e porta:** `game.account_wallet` (`migrations/0005-game-account.md`); porta `wallet`
  (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-004 — cofre pertence à conta

- **Enunciado:** o baú guarda item por conta, não por personagem; apagar personagem não apaga o baú.
- **Entrada:** `account_id`, `slot`, `item_index`, `qty`.
- **Saída:** linha em `game.vault_item`.
- **Critério de aceitação:** `\d game.vault_item` não tem coluna de personagem; `INSERT` com
  `account_id` inexistente falha com `23503` em `fk_vault_item_account`; `DELETE FROM game.character`
  da conta não remove a linha (a FK é da conta).
- **Tabela e porta:** `game.vault_item` (`migrations/0009-game-inventory.md`); porta `inventory`
  (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-005 — um resgate diário por conta e dia

- **Enunciado:** a recompensa diária é resgatada no máximo uma vez por conta e por dia.
- **Entrada:** `account_id`, `claim_date` e a recompensa.
- **Saída:** linha em `portal.daily_reward_claim`.
- **Critério de aceitação:** dois `INSERT` com o mesmo `(account_id, claim_date)` resultam no segundo
  `23505` em `uq_daily_reward_claim_dia`. A fronteira do dia é UTC; o banco guarda a data, a regra de
  fuso é de código.
- **Tabela e porta:** `portal.daily_reward_claim` (`migrations/0011-game-economy.md`); porta
  `rewards` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-006 — uma sessão viva por conta

- **Enunciado:** a mesma conta não abre duas sessões de jogo ao mesmo tempo; uma segunda conexão é
  recusada pelo banco.
- **Entrada:** `account_id`, `group_id`, `channel`, `client_version`.
- **Saída:** linha em `game.session` com `released_at` nulo.
- **Critério de aceitação:** dois `INSERT` com o mesmo `account_id` e `released_at IS NULL` resultam no
  segundo `23505` em `ux_session_viva_por_conta`; depois de `UPDATE ... SET released_at = now(),
  released_reason = 'disconnect'`, a nova sessão entra.
- **Tabela e porta:** `game.session` (`migrations/0007-game-session.md`); porta `session` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-007 — bloqueio exige motivo, autor e instante

- **Enunciado:** bloquear conta é decisão disciplinar registrada em `game.account_moderation`, com
  motivo não vazio, autor e instante; não altera `game.account.status`.
- **Entrada:** `account_id`, `reason`, `blocked_by`.
- **Saída:** linha de moderação com `blocked = true`.
- **Critério de aceitação:** `INSERT ... (account_id, blocked, reason, blocked_by, blocked_at) VALUES
  (1, true, '', 1, now())` falha com `23514` em `ck_account_moderation_motivo`; sem `blocked_by`,
  falha em `ck_account_moderation_autor`; sem `blocked_at`, em `ck_account_moderation_instante`.
- **Tabela e porta:** `game.account_moderation` (`migrations/0005-game-account.md`); porta `moderation`
  (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-008 — crédito e débito de donate idempotentes

- **Enunciado:** cada lançamento de donate carrega uma chave de idempotência; repetir a chave não
  duplica o lançamento.
- **Entrada:** `idempotency_key`, valor e motivo.
- **Saída:** linha em `portal.donate_credit` ou `portal.donate_debit`.
- **Critério de aceitação:** dois `INSERT` com a mesma `idempotency_key` resultam no segundo `23505`
  em `uq_donate_credit_idem` (ou `uq_donate_debit_idem`); o valor é positivo por `CHECK`.
- **Tabela e porta:** `portal.donate_credit`/`donate_debit` (`migrations/0011-game-economy.md`);
  porta `donate` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-009 — grant pendente impede apagar conta e personagem

- **Enunciado:** enquanto houver linha em `ops.delivery_queue`, a conta e o personagem referenciados
  não podem ser apagados.
- **Entrada:** `account_id`, `character_id` e o grant.
- **Saída:** linha na fila, com chave de idempotência.
- **Critério de aceitação:** com grant pendente, `DELETE FROM game.account WHERE id = 1` falha com
  `23503` em `fk_delivery_queue_account`; o mesmo para `game.character` em
  `fk_delivery_queue_character`.
- **Tabela e porta:** `ops.delivery_queue` (`migrations/0012-ops-delivery.md`); porta `delivery`
  (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-010 — token de reset só existe como hash

- **Enunciado:** o token de cerimônia e o de reset de PIN são opacos no fio e guardados apenas como
  `sha256` hexadecimal de 64 caracteres.
- **Entrada:** token em claro na borda; `token_hash` no banco.
- **Saída:** linha em `portal.person_token` ou `game.pin_reset`.
- **Critério de aceitação:** `INSERT` com `token_hash = 'XYZ'` falha com `23514` em
  `ck_person_token_hash_forma` (ou `ck_pin_reset_token_forma`); 64 hexadecimais passam.
- **Tabela e porta:** `portal.person_token` (`migrations/0004-portal-access.md`) e `game.pin_reset`
  (`migrations/0005-game-account.md`); porta `identity` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-011 — auditoria append-only, sem FK

- **Enunciado:** `ops.audit_log` recebe linha de jogo e portal, e nem `UPDATE` nem `DELETE` são
  concedidos; o ator é tripartido e não tem FK.
- **Entrada:** `actor_kind`, `actor_id`/`actor_ref`, `action`, `target`, `detail`.
- **Saída:** linha imutável em `ops.audit_log`.
- **Critério de aceitação:** `SET ROLE wyd_game; UPDATE ops.audit_log SET action = 'x'` falha com
  `42501`; `INSERT` de `wyd_game` passa; `\d ops.audit_log` não mostra FK.
- **Tabela e porta:** `ops.audit_log` (`migrations/0002-ops-audit.md`); porta `audit` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-012 — papel de jogo é fechado

- **Enunciado:** o papel de jogo é `player`, `moderator` ou `admin`; o papel de portal (`member`,
  `moderator`, `admin`) é outro tipo e não se traduz por igualdade de string.
- **Entrada:** `role` da conta.
- **Saída:** linha em `game.account`.
- **Critério de aceitação:** `UPDATE game.account SET role = 'gm' WHERE id = 1` falha com `23514` em
  `ck_account_role`; `player` passa.
- **Tabela e porta:** `game.account` (`migrations/0005-game-account.md`); porta `accounts` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-013 — senha e PIN só como hash argon2id

- **Enunciado:** senha de portal, senha de jogo e PIN só entram como hash argon2id; a senha do portal
  tem 15–128 caracteres e o PIN 4–6, validados na borda.
- **Entrada:** `password_hash`/`pin_hash`.
- **Saída:** linha da pessoa ou da conta.
- **Critério de aceitação:** `INSERT` com `password_hash = 'texto'` falha com `23514` em
  `ck_person_senha_hasheada` (portal) ou `ck_account_senha_hasheada` (jogo); o prefixo
  `$argon2id$` passa.
- **Tabela e porta:** `portal.person` (`migrations/0003-portal-person.md`) e `game.account`
  (`migrations/0005-game-account.md`); porta `identity` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-014 — cinco contas da mesma pessoa online

- **Enunciado:** a barreira de sessão é por conta e por personagem, nunca por pessoa; cinco contas da
  mesma pessoa podem estar vivas ao mesmo tempo, no mesmo canal ou em canais diferentes.
- **Entrada:** cinco `account_id` distintos, com a mesma `person_id`, em canais válidos.
- **Saída:** cinco linhas vivas em `game.session`.
- **Critério de aceitação:** cinco `INSERT` (uma por conta) passam e `SELECT count(*) FROM game.session
  WHERE released_at IS NULL` devolve `5`; a sexta conta não existe, porque a vaga é 1–5
  (`RN-DB-001`).
- **Tabela e porta:** `game.session` (`migrations/0007-game-session.md`); porta `session` (planejada).
- **Issue:** a alocar na onda 8.

## RF-DB-015 — contas da mesma pessoa formam grupo e trocam entre si

- **Enunciado:** o schema não impede grupo, troca ou loja entre contas da mesma pessoa; restringir
  isso é política de evento, não constraint.
- **Entrada:** duas contas da mesma pessoa.
- **Saída:** nenhuma linha criada pelo banco; a operação é de jogo.
- **Critério de aceitação:** não há `CHECK`/índice que relacione `account_id` a `person_id` em
  `game.session` além da FK da conta; a prova é a ausência de objeto no DDL
  (`ddl-confirmado.md` §5.5), não um comando.
- **Tabela e porta:** nenhuma tabela; regra de jogo (frente `wyd-server`).
- **Issue:** a alocar na onda 8.

## RF-DB-016 — limite por IP comporta cinco clientes por pessoa

- **Enunciado:** o limite de conexões por IP não pode ser menor que cinco por pessoa mais margem; a
  vaga é reservada no `accept`, antes do login.
- **Entrada:** conexões simultâneas do mesmo IP.
- **Saída:** todas aceitas até o limite configurado.
- **Critério de aceitação:** com `WYD_MAX_CONNECTIONS_PER_IP = 12`, cinco conexões de uma pessoa e
  cinco de outra, do mesmo IP, somam `10` e passam; a décima terceira conexão do mesmo IP é recusada.
  É requisito de **fronteira**, não de schema.
- **Tabela e porta:** nenhuma tabela; `cpsock/limits.rs` (frente `wyd-server`, `D10`).
- **Issue:** a alocar na onda 8, com dono `wyd-server`.

## RF-DB-017 — uma versão corrente de conteúdo, referenciada por FK

- **Enunciado:** o banco admite **uma única** versão corrente de conteúdo — `content.content_version` é
  linha única, `id = 1` — e o personagem referencia essa versão por FK; duas versões correntes **não são
  representáveis**.
- **Entrada:** `id`, `version`, `note`.
- **Saída:** a linha única de versão, criada pela carga de conteúdo.
- **Critério de aceitação:** `INSERT INTO content.content_version (id, version) VALUES (2, 1)` falha com
  `23514` em `ck_content_version_linha_unica`; `UPDATE content.content_version SET id = 2` falha no mesmo
  `CHECK`; `INSERT` em `game.character` com `content_version` inexistente falha com `23503` em
  `fk_character_content_version`.
- **Tabela e porta:** `content.content_version` (`migrations/0001-bootstrap.md` §4 e §9, prova 9) e a FK de
  `game.character` (`migrations/0006-game-character.md`); porta `catalog` (planejada).
- **Issue:** a alocar na onda 8.

## O que este recorte não cobre

| Fora | Por quê | Onde entra |
|---|---|---|
| requisitos de personagem, inventário, economia, conteúdo e entrega | não foram levantados como `R`; só o desenho existe | ondas donas, com `RF-DB` próprio |
| requisitos não funcionais (limiar, latência, capacidade) | nenhum limiar foi medido | [`requisitos-nao-funcionais.md`](requisitos-nao-funcionais.md), escrita com 5 limiares e 2 a medir |
| matriz requisito → tabela → porta → teste → issue | porta e teste existem como **alvo** — o teste tem matriz `CAP-nn` desde 2026-11-24 —, e a issue segue sem artefato | [`rastreabilidade.md`](rastreabilidade.md), escrita em 2026-11-24 |
| issue de entrega | `D-7` aprovada; falta a página `WYDDB-nnn` com DoD | onda 8 |

*Os `RF-DB-001`–`RF-DB-017` são a numeração de requisito funcional do `wyd-db`; o número 018 é o
próximo livre.*

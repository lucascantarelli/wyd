> Pergunta: "que regra vale sempre no `wyd-db`, e onde ela é garantida?"
> Público: Produto / gestão e arquitetura
> Dono: equipe do `wyd-db`
> Base: `padrao-banco.md` §5; `ddl-confirmado.md` §5.1, §5.2, §5.5; `rascunho-accounts.md` §2–§3 e §6; `lacunas.md` §2
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Regra de negócio é o que vale **sempre**, independentemente de tela, canal ou crate. Cada bloco diz
onde a regra é garantida — banco ou código — e o que acontece quando alguém tenta violá-la. As sete
desta página vêm do bloco de identidade e são a primeira numeração `RN-DB` do sistema.

> [!IMPORTANT]
> Banco e código não se substituem. Índice único e `CHECK` valem até para `psql` e para uma segunda
> instância do servidor; limiar de política (PIN, idle de sessão) só o código garante, e a página diz
> isso em vez de fingir que o schema resolve.

## RN-DB-001 — vaga de conta é 1..5 e a fechada é reutilizada

- **Enunciado:** cada pessoa tem no máximo cinco contas, uma por vaga (`slot` 1–5); conta `closed`
  libera a vaga, e o código escolhe o **menor** slot livre.
- **Exemplo numérico:** pessoa com slots 1–5 tenta a sexta conta → `23505` em `ux_account_slot`. Se a
  conta do slot 3 fecha, a próxima conta usa o slot 3, não o 6.
- **Onde é garantida:** banco para o limite e a exclusividade (`ck_account_slot_range` +
  `ux_account_slot WHERE status <> 'closed'`); código para "menor slot livre" (`D3`).
- **Efeito da violação:** `23505 unique_violation`; a sexta conta concorrente falha no banco, não na
  revisão de código.
- **Atende:** `RF-DB-001`, `RF-DB-014`.

## RN-DB-002 — uma sessão viva por conta e por personagem

- **Enunciado:** uma conta tem no máximo uma sessão com `released_at IS NULL`; um personagem também,
  quando já houver seleção.
- **Exemplo numérico:** dois `INSERT` para a conta 1 sem liberar o primeiro → segundo `23505` em
  `ux_session_viva_por_conta`. Dois `INSERT` com o mesmo `character_id` → segundo `23505` em
  `ux_session_viva_por_personagem`.
- **Onde é garantida:** banco, por dois índices únicos parciais em `game.session` (§5.5).
- **Efeito da violação:** `23505`; o canal recusa a segunda conexão da mesma conta, mesmo em outro
  processo.
- **Atende:** `RF-DB-006`, `RF-DB-014`.

## RN-DB-003 — cinco contas da mesma pessoa, sem barreira por pessoa ou IP

- **Enunciado:** o banco não tem, e não deve ter, "uma sessão por pessoa" nem "uma sessão por IP";
  cinco contas da mesma pessoa podem estar vivas em canais iguais ou diferentes.
- **Exemplo numérico:** pessoa com cinco contas em cinco canais soma `5` linhas vivas em
  `game.session`; o mesmo IP em todas não muda nada.
- **Onde é garantida:** banco **permite** (não há constraint por pessoa/IP); a cultura do jogo é a
  fonte da regra, e o limite por IP é do transporte (`RF-DB-016`).
- **Efeito da violação:** não há violação de schema; restringir troca entre contas do mesmo IP é
  política de evento competitivo, fora do banco.
- **Atende:** `RF-DB-014`, `RF-DB-015`.

## RN-DB-004 — fechar pessoa anonimiza o e-mail

- **Enunciado:** a linha de pessoa `closed` nunca guarda o endereço real; o e-mail vira
  `closed-<id>@removido.invalid` e o índice de e-mail é total, sem janela de reuso.
- **Exemplo numérico:** pessoa `id = 42` fechada com `email = 'ana@exemplo.com'` → `23514` em
  `ck_person_fechamento_anonimo`; com `closed-42@removido.invalid`, passa. Reabrir com o endereço
  antigo não é possível: o índice é total.
- **Onde é garantida:** banco (`CHECK` de fechamento anônimo + `ux_person_email` total).
- **Efeito da violação:** `23514 check_violation`; o suporte não tem exceção.
- **Atende:** `RF-DB-001` (identidade), `RN-DB-001` (vaga liberada só com `closed`).

## RN-DB-005 — moeda é inteira de 32 bits

- **Enunciado:** `coin` e `balance_after` são inteiros entre `0` e `2147483647`; saldo negativo não
  existe e o teto é o do tipo, não um número de negócio.
- **Exemplo numérico:** `coin = 2147483648` → `23514` em `ck_account_wallet_coin`; `balance_after =
  -1` no razão → `23514` em `ck_account_ledger_balance`.
- **Onde é garantida:** banco (`CHECK` de `game.account_wallet` e `game.account_ledger`).
- **Efeito da violação:** `23514`; a transação inteira aborta, sem saldo parcial.
- **Atende:** `RF-DB-003`.

## RN-DB-006 — bloqueio é moderação, não ciclo de vida

- **Enunciado:** bloquear conta grava `game.account_moderation.blocked` com motivo não vazio, autor e
  instante; `game.account.status` continua `active` ou `closed` e nunca vira `suspended`.
- **Exemplo numérico:** `blocked = true` com `reason = ''` → `23514` em
  `ck_account_moderation_motivo`; sem `blocked_by` → `ck_account_moderation_autor`; sem `blocked_at`
  → `ck_account_moderation_instante`.
- **Onde é garantida:** banco nos três `CHECK`s e no `CHECK` de `status` (`M1`); a decisão de quem
  bloqueia é do serviço `wyd-admin`.
- **Efeito da violação:** `23514`; não existe "conta suspensa" para o schema recusar.
- **Atende:** `RF-DB-007`.

## RN-DB-007 — limiar de PIN é política, o estado é do banco

- **Enunciado:** cinco falhas de PIN em dez minutos bloqueiam a conta por tempo; o banco guarda
  contador e instante, o limiar é do código.
- **Exemplo numérico:** cinco tentativas erradas → `pin_failure_count = 5` e `pin_locked_until =
  now() + interval '10 minutes'`; a sexta é recusada pelo código, sem chegar ao banco.
- **Onde é garantida:** código (`B3`); o banco garante apenas `pin_failure_count >= 0`
  (`ck_account_pin_falhas`) e o estado persistido.
- **Efeito da violação:** a conta fica bloqueada até o instante; reiniciar o processo não zera nada,
  porque o estado está na linha.
- **Atende:** `RF-DB-002`.

## O que estas regras não cobrem

- Regra de personagem, inventário, economia, conteúdo e entrega: cada domínio terá `RN-DB` próprio.
- Efeito de bloqueio ou fechamento sobre a **sessão de jogo em curso**: nenhum documento decide se o
  canal derruba a sessão; o schema não tem gatilho para isso. A página não afirma o que não está
  decidido.
- Limiar de idle da sessão do portal: o campo `last_seen_at` existe; o intervalo é política de código,
  não número desta página (`migrations/0004-portal-access.md` §10).

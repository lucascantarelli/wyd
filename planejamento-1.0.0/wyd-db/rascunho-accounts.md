# Requisitos do subsistema de identidade — histórico da 1.0.0

> **Estado: histórico de requisitos. Este arquivo não é o desenho do banco.**
> Ele guarda o que o sistema **precisa continuar fazendo** (medido no legado), o comportamento de
> multibox que a cultura do jogo exige e a lista de defeitos que a reescrita não reproduz. Nada aqui
> define coluna, constraint, nome de tabela nem ordem de arquivo.
>
> **Donos hoje:** `ddl-confirmado.md` (texto DDL), `padrao-banco.md` (forma), `plano-migrations.md`
> (ordem e escopo), `lacunas.md` (decisões abertas), `migrations-checklist.md` (andamento).
>
> Esta é a versão enxuta da especificação original (963 linhas). O DDL dela foi **descartado**, não
> corrigido: usava `public`, nome plural, `ip TEXT` e `SCHEMA_VERSION = 50`, tudo revogado por
> `P-01`, `P-02`, `P-10` e por `plano-migrations.md`.
>
> Revisado em 2026-11-17.

## 1. Como ler

O arquivo responde quatro perguntas: **o que o jogo já faz** (§2), **o que a cultura de multibox
exige** (§3), **o que o legado faz de errado e não volta** (§4) e **quais transições de estado são
legais** (§5). A §6 diz onde cada decisão da versão antiga vive hoje; a §8 mapeia as seções que
saíram.

Se você procura "como criar a tabela", está no arquivo errado: vá para `ddl-confirmado.md` e para a
página da migration em `migrations/`.

**Dentro do domínio:** identidade da pessoa, credencial do portal, sessão do portal, credencial da
conta de jogo (senha + PIN), vaga de conta por pessoa, papel de jogo, moderação da conta, carteira da
conta, tokens de recuperação, sessão viva de jogo.

**Fora (outros donos):** personagens, cofre, fila de entrega, recompensa diária, razão de donate,
auditoria, conteúdo, canais.

## 2. Inventário de requisitos (o que o legado prova que precisa existir)

Medido no código atual; cada linha vira requisito. Não é desenho.

| # | Requisito observado | Fonte medida | Onde o desenho atende hoje |
|---|---|---|---|
| R1 | Login de conta de jogo tem 1..16 caracteres no fio | `0030_sessao_unica.sql` | `ck_account_login_len` + tipo `GameLogin` (`ddl-confirmado.md` §5.2) |
| R2 | PIN errado acumula e bloqueia; reiniciar o processo não zera | `accounts.pin_failures`, `pin_locked_until`, `repo/accounts.rs::pin_guard` | `game.account.pin_failure_count`/`pin_locked_until` (§5.2) |
| R3 | `Coin` é saldo **da conta**, não do personagem, teto `i32::MAX` | `0034_account_wallet.sql` | `game.account_wallet.coin` (§5.2) |
| R4 | Cofre é da conta | `0003_inventario.sql::vault_items` | `game` (0009), outro arquivo |
| R5 | Um resgate diário por conta por dia UTC | `0028_estado_de_jogo.sql` | `game` (0011), outro arquivo |
| R6 | Uma sessão viva por conta; mesma conta em dois canais é recusada | `0030_sessao_unica.sql` | `ux_session_viva_por_conta` (`ddl-confirmado.md` §5.5) |
| R7 | Bloqueio exige motivo e autor | `0031_moderacao.sql` | `game.account_moderation` (§5.2) |
| R8 | Crédito/débito de donate idempotente por chave | `0042`, `0045` | `portal` (0011); formato é `M17`, decidida em 2026-11-19 |
| R9 | Grant pendente impede apagar conta/personagem (`RESTRICT`) | `0006_delivery_queue.sql` | `ops` (0012, desenho em `ddl-confirmado.md` §5.16) |
| R10 | Token de reset só existe como hash sha256 hex de 64 | `0053_pin_reset_token.sql` | `portal.person_token` (§4) e `game.pin_reset` (§5.2) |
| R11 | Auditoria append-only, sem FK | `0005_auditoria.sql` | `ops.audit_log` (§5.3), ator tripartido (`M18-A`) |
| R12 | Papel do jogador no jogo: `player`/`moderator`/`admin` | `accounts_role` | `game.account.role` (§5.2) |
| R13 | Senha e PIN só como hash resistente (argon2id) | `AGENTS.md` §2.5, `repo/accounts.rs` | CHECKs de prefixo (§5.1/§5.2) + política `M9` |
| R14 | As **5** contas da mesma pessoa podem estar online ao mesmo tempo, em qualquer canal | cultura do jogo (§3) + `docs/wyd/multiconta.md` | `game.session` (§5.5): sessão só por conta |
| R15 | Contas da mesma pessoa formam grupo e trocam entre si | `docs/wyd/multiconta.md` + comandos `/party*` | o schema não impede; restrição só por política |
| R16 | O limite de conexões por IP não pode ser menor que 5 por pessoa | `cpsock/limits.rs` (padrão 6), `tests/sistema/limite_ip.rs` | política de transporte, não schema (`D10`) |

## 3. Multibox, auto-attack e o limite por IP

Não é decisão a inventar: é cultura do jogo, confirmada por fonte. O WYD tem **macro embutido no
cliente** — ataque automático, uso de skill e caça — e ele é parte do jogo, não trapaça (Wikipedia,
*Gameplay*: "Integrated macro system"; o guia do 7662 descreve o *Ataque Automático* na tecla `T`).
O WYD Global liberava até três contas por computador no lançamento, e a comunidade opera várias
janelas (`docs/wyd/multiconta.md`).

| Regra | Consequência |
|---|---|
| Sessão viva é **por conta de jogo** | 5 contas ⇒ **5 sessões vivas simultâneas**, no mesmo canal ou em canais diferentes |
| Nunca por pessoa | o banco não tem, e não deve ter, barreira "1 sessão por pessoa" |
| Nunca por IP | 5 clientes do mesmo endereço é o caso normal, não abuso |
| Nunca por processo | canal é processo; a conta pode viver em qualquer canal do grupo |

O banco garante só isto: a mesma conta não entra duas vezes e o mesmo personagem não entra duas
vezes (`ddl-confirmado.md` §5.5). Grupo, troca e loja entre contas da mesma pessoa são permitidos;
restrição de troca entre contas do mesmo IP vale apenas em evento competitivo, por política — mesma
conclusão de `docs/wyd/multiconta.md` §"Controles recomendados".

**O que estourava, medido:** o transporte limita conexões por IP com `WYD_MAX_CONNECTIONS_PER_IP`
padrão 6 (`crates/wyd-server/src/cpsock/limits.rs`), comentado como "multiboxing custa 4–5". Com 5
contas por pessoa, uma pessoa sozinha ocupa 5 vagas e a segunda pessoa do mesmo NAT não entra. Pior:
a vaga é reservada no `accept`, **antes do login** (`IpSlot::reservar`), então conexão abandonada
consome vaga até o `Drop`. Piso do parâmetro: `5 × pessoas_por_IP + margem` — recomendação `D10`.

**Enunciação (2026-11-18):** o que o **banco** garante sobre sessão e o que fica como política de
transporte está em `docs/arquitetura/integridade-e-constraints.md` §6, com a linha de `R16` citada.
Aqui ficam as quatro regras de sessão (por conta, nunca por pessoa, por IP ou por processo), o limite
por IP medido e o que o legado prova — a fonte daquela classificação. O requisito numerado é outro:
`R16`, em §2. Requisito novo de sessão entra em §2; a classificação de onde ele vive, na página.

## 4. Defeitos do legado que a reescrita não reproduz

Não são requisito, são dívida.

| Defeito | Por que não volta |
|---|---|
| `accounts.email` com `UNIQUE` parcial | impede 1:N por construção; e-mail passa a ser exclusivo da pessoa |
| `login` sem teto no banco (aceitava 17–32 e travava na sessão) | CHECK de 16 + tipo `GameLogin` na borda |
| `character_session.account_login` (texto) como chave | sessão referencia `game.account.id`, imutável |
| Portal autenticando com credencial de jogo (`verifyCredentials`) | porta `portal/verify` separada |
| Sem revogação de sessão do portal | `portal.person_session` (`revoked_at`) + token opaco, só o `sha256` no banco |
| Limite de 5 contas só em código | vaga única no banco (`ux_account_slot`) |

## 5. Estados que o desenho exige

Transições legais, não nomes de coluna. O destino definitivo deste conteúdo é
`docs/specs/estados/` (ainda não escrito).

| Entidade | Transições | Efeito |
|---|---|---|
| pessoa (`status`) | `pending → active` (e-mail verificado); `active → suspended` (staff); `suspended → active` (revisão); `active`/`suspended → closed` (pedido do titular) | `closed` é terminal; `suspended` e `closed` recusam login no portal e **não** derrubam sessão de jogo em curso |
| conta de jogo (`status` + moderação) | `active → closed` (titular). Bloquear/liberar **não** é `status`: é `moderation.blocked` | `status` guarda ciclo de vida; bloqueio é decisão disciplinar com autor e instante |
| sessão do portal | nasce no login; morre por `expires_at`, por idle (`last_seen_at + intervalo`) ou por `revoked_at` | uma pessoa pode ter N sessões, cada uma revogável |
| sessão de jogo | nasce no handshake autenticado; morre em `released_at` (logout/canal) ou por TTL de `renewed_at` | no máximo 1 viva por conta e 1 por personagem; uma pessoa pode ter **5** |

Duas consequências que já foram decisão e continuam valendo: fechar pessoa **não** derruba sessão de
jogo (`D2`); e `closed` não tem decisão de staff — é pedido do titular e vive em `closed_at`.

## 6. Decisões D1–D14 — onde cada uma vive hoje

| # | Decisão original | Onde vive hoje |
|---|---|---|
| D1 | Limite de 5 contas por **vaga** (`slot` 1..5, índice único parcial) | `ddl-confirmado.md` §5.2 (`ux_account_slot`), `padrao-banco.md` `P-14` |
| D2 | Pessoa suspensa/fechada **não** derruba sessão de jogo em curso | **só aqui** (§5) — destino: ADR de fronteira pessoa × conta, sem número (marco `P1`) |
| D3 | Vaga de conta fechada é reutilizada pelo menor slot livre | **só aqui** — o índice permite; "menor slot" é política de código |
| D4 | Papel do portal `member`/`moderator`/`admin`; de jogo `player`/`moderator`/`admin` | `lacunas.md` `A15` (fechada) + CHECKs de `ddl-confirmado.md` §5.1/§5.2 |
| D5 | `portal_user_profile` **não** entra | **revogada** por `M7` (2026-11-17): `person_profile` entra — `ddl-confirmado.md` §5.6 |
| D6 | `ip` da sessão como `TEXT` | **revogada** por `P-10`: `INET` (`ddl-confirmado.md` §4, §5.2, §5.4) |
| D7 | Sessão do portal expira em 8 h, revogável na própria linha (`revoked_at`) | `ddl-confirmado.md` §4 + `M4`; transporte decidido em `A16` (2026-11-18): token opaco, `token_hash` no banco |
| D8 | Seed cria 1 pessoa com 2 contas | **só aqui** — entra na onda de seed (ainda não escrita) |
| D9 | As 5 contas online ao mesmo tempo, em qualquer canal, com grupo e troca | §3 e §5 deste arquivo + `ddl-confirmado.md` §5.5 |
| D10 | `WYD_MAX_CONNECTIONS_PER_IP` sobe para 12 | **só aqui** — `cpsock/limits.rs`, não implementado |
| D11 | Suspensão anotada na própria linha da pessoa, não em tabela de decisões | `lacunas.md` `B4` (fechada) + `ddl-confirmado.md` §5.1 |
| D12 | `game.session` guarda `ip`; IP é auditoria, nunca identidade | `lacunas.md` `B5` (fechada) + `ddl-confirmado.md` §5.5 |
| D13 | `updated_at` anda por trigger `set_updated_at()` | `lacunas.md` `A9` (fechada) + `0001` (`ddl-confirmado.md` §3), §5.1 e §5.2 |
| D14 | Fechar pessoa anonimiza o e-mail (tombstone) e o índice fica total | `lacunas.md` `A12` (fechada) + `ddl-confirmado.md` §5.1 |

`D2`, `D3`, `D8` e `D10` **não têm outro dono** hoje. Continuam valendo como decisão, mas precisam
de destino antes de a onda dona ser escrita — o lugar natural é a ADR de fronteira pessoa × conta,
sem número (marco `P1`).

## 7. O que continua UNVERIFIED

Não virou regra, e não deve virar sem captura no cliente 7662:

- conjunto de caracteres do login (o banco só impõe 1..16 e sem espaço nas bordas);
- se o handshake entrega `account_id` ou `login`;
- se o cliente envia caixa diferente do cadastro (por isso `login_fold` é decisão explícita, e não
  efeito colateral).

## 8. Para onde foram as seções removidas

| Antes | Agora |
|---|---|
| §2 requisitos | §2 (idêntico, com a coluna de destino atualizada) |
| §7.1 multibox e limite por IP | §3 |
| §13.2 defeitos do legado | §4 |
| §6 estados (pessoa, conta, sessão) | §5 (compactado; destino final `docs/specs/estados/`) |
| §12 `D1`–`D14` | §6 |
| §4, §7 (DDL de identidade e sessão) | `ddl-confirmado.md` §4, §5.1, §5.2, §5.5 |
| §3 invariantes | constraints de `ddl-confirmado.md` |
| §5 contratos com outros arquivos | `plano-migrations.md` §3 |
| §8 tipos e portas Rust | `src/` (a escrever) |
| §9 portal BFF | `src/` / frente `wyd-portal` (a escrever) |
| §10 seed, catálogo e testes | `tests/` + `migrations-checklist.md` |
| §11 árvore e ondas | `plano-migrations.md` §3/§9 |
| §0, §1, §13.1, §13.3, §13.4, §14 | removidos: história da decisão, sem valor de requisito |

*Histórico de requisitos. Nenhum SQL e nenhuma linha de `crates/` depende deste arquivo.*

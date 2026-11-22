> Pergunta: "quem pode o quê e o que é auditado?"
> Público: Arquiteto de schema e operação
> Dono: equipe do wyd-db
> Base: `padrao-banco.md` §2, §8, `ddl-confirmado.md`, `docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md`
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-18)

A segurança do `wyd-db` é feita de três camadas que não se substituem: **privilégio** (quem pode
tocar no quê), **hash** (o que não é reversível) e **auditoria** (o que fica registrado mesmo quando
o resto dá errado). Papel mal escolhido anula as outras duas.

## Papéis do banco

| Papel | Para que existe | Pode | Não pode |
|---|---|---|---|
| `wyd_owner` | dono dos schemas e dos objetos | nada em runtime: é `NOLOGIN` | conectar |
| `wyd_migrator` | aplicar a série | DDL no schema, escrita no ledger | servir requisição de jogo ou portal |
| `wyd_game` | servidor de jogo | escrever o estado vivo do jogo | ler dado de pessoa do portal, fazer DDL |
| `wyd_portal` | BFF do portal | ler identidade/sessão e escrever em `ops` | escrever estado vivo; conceder item direto (só via `delivery_queue`) |
| `wyd_admin` | serviço `wyd-admin` (staff) | escrever as sete tabelas do C4 em `game` e `INSERT` em `ops.audit_log` | escrever em `portal`/`content`, `DELETE`, DDL |
| `wyd_read` | operação e BI | `SELECT` nas views | escrever qualquer coisa |

Os papéis nascem **fora da série**, no provisionamento (`ADR-DB-0001`), porque criar papel exige
privilégio de cluster e a série roda como `wyd_migrator`. O runner recusa conexão que não seja
`wyd_migrator`; a única saída é `WYD_MIGRATE_ALLOW_SUPERUSER=1`, restrita a desenvolvimento e
sempre logada.

`wyd_admin` entrou em 2026-11-21, quando o dono fechou `M27` na opção A + S1 + C1 + D1: a forma do
papel, o privilégio por tabela e o arquivo do `GRANT` estão na seção "Decisão — papel `wyd_admin`
(`LAC-ADM-01`)", abaixo, que registra a escolha e as alternativas recusadas.

As DSNs são **segredo** — a senha viaja dentro da string, e nunca são logadas em claro. Nome, leitor de
cada DSN e a classificação segredo × dado de instalação estão em `src/provision/spec.md` §3; a regra de
credencial, em §7.

**Fora do contrato:** outros segredos do repositório — o `SESSION_SECRET` citado em `AGENTS.md` §2.5 é
um deles — **não** são lidos pelo `wyd-db` e não entram no contrato de ambiente
(`src/provision/spec.md` §3 e §7). Com cookie opaco e só o `sha256` no banco (`A16`), a 1.0.0 pode não
precisar de segredo de assinatura nenhum; se algum sobreviver, é do portal, e esta página não é o dono
do nome.

## Decisão — papel `wyd_admin` (`LAC-ADM-01`, 2026-11-21)

**Estado: decidida em 2026-11-21 — A + S1 + C1 + D1.** O papel entrou em `ROLES`
(`src/provision/spec.md` §2) e no golden `db/roles.sql`; o `GRANT` é o `0032_admin_grants.sql`
(`ddl-confirmado.md` §5.32, página `migrations/0032-admin-grants.md`). `M27` está fechada em
`lacunas.md` §2.3 e `LAC-ADM-01` fechou com ela; `M28` fechou na mesma data com a redação **B**.

**O que o serviço `wyd-admin` precisa.** As sete rotas de ação do C4 (`wyd-admin/docs/arquitetura/contrato-c4.md`
§4) escrevem em sete tabelas de `game` e em `ops.audit_log`; `POST /broadcasts` (§4.5) não grava tabela
de jogo. O `USAGE` em `game` abre o schema, não a escrita: o privilégio vem de `INSERT`/`UPDATE` por
tabela. Nenhuma rota pede `DELETE`.

| Tabela | Rota do C4 | Privilégio mínimo | Por que |
|---|---|---|---|
| `game.account_moderation` | §4.1 | `SELECT`, `INSERT`, `UPDATE` | upsert do bloqueio, conferindo `expected_blocked` |
| `game.account_wallet` | §4.2 | `SELECT`, `UPDATE` | trava, confere `expected_coin` e grava o saldo |
| `game.account_ledger` | §4.2 | `SELECT`, `INSERT` | linha de lançamento com `balance_after`, devolvida no `200` |
| `game.server_channel` | §4.3 | `SELECT`, `UPDATE` | estado do canal 1–10 |
| `game.session` | §4.4 | `SELECT`, `UPDATE` | `released_at`/`released_reason` só depois do ack |
| `game.city` | §4.6 | `SELECT`, `UPDATE` | alíquota com `expected_tax_rate_bps` |
| `game.item_price` | §4.7 | `SELECT`, `UPDATE` | preços com os dois `expected_*` |
| `ops.audit_log` | todas | `INSERT` + `SELECT (id)` | auditoria da ação e retorno do `audit_id` |

### Decisões registradas e alternativas (M27)

#### 1. Forma do papel

| Opção | Forma | Consequência |
|---|---|---|
| **A (decidida em 2026-11-21)** | `wyd_admin` com `LOGIN`, sem membership, senha própria por ambiente e DSN nova (`WYD_DB_URL_ADMIN`) | papel separado e atribuível; custo: mais um segredo a provisionar e rotacionar (`wyd-ops`) |
| B (não escolhida) | `wyd_admin` sem `LOGIN`, com o processo do admin fazendo `SET ROLE` a partir de uma credencial existente | economizaria uma DSN, mas o runtime passaria a carregar a credencial mais ampla e o `SET ROLE` viraria parte do contrato |
| C (recusada) | conceder a escrita em `game` a `wyd_game`, sem papel novo | ampliaria o runtime de jogo com escrita de staff e misturaria duas atribuições; contrariaria `ADR-DB-0001` e o princípio de credencial por runtime |

#### 2. Leitura nas tabelas do C4

| Opção | Forma | Consequência |
|---|---|---|
| **S1 (decidida em 2026-11-21)** | `SELECT` nas sete tabelas | cobre `expected_*`, `WHERE` e `RETURNING` sem depender de leitura em outra tabela; é o menor conjunto que faz o contrato funcionar |
| S2 (não escolhida) | `SELECT` por coluna, só nas conferidas e devolvidas | reduziria a leitura, mas cada mudança de resposta do C4 viraria mudança de `GRANT` |
| S3 (recusada) | nenhum `SELECT` | `RETURNING` e `expected_*` exigem leitura; sem ela, §4.1–§4.7 não fechariam |

#### 3. Auditoria

| Opção | Forma | Consequência |
|---|---|---|
| **C1 (decidida em 2026-11-21)** | `USAGE` em `ops` + `INSERT` em `ops.audit_log` + `SELECT (id)` | `USAGE` dá acesso ao schema, não às tabelas; o `SELECT (id)` permite devolver `audit_id`; nenhum DDL novo |
| C2 (não escolhida) | função `SECURITY DEFINER` em `ops`, com `EXECUTE` para `wyd_admin` e sem `USAGE` em `ops` | tiraria do papel o `INSERT` direto, mas criaria objeto novo e um caminho de escrita a mais; exigiria DDL e revisão própria |
| C3 (não escolhida) | `INSERT` sem `RETURNING` | dispensaria `SELECT` em `audit_log`, mas o `200` do C4 perderia `audit_id` — seria mudança de contrato, não de banco |

#### 4. Onde o `GRANT` mora

| Opção | Forma | Consequência |
|---|---|---|
| **D1 (decidida em 2026-11-21)** | arquivo novo `0032_admin_grants.sql` | não reabriu `0001`; a série passou de 31 para 32 arquivos, `Tab.` continua 97 e as citações de contagem e gates foram atualizadas na mesma entrega |
| D2 (não escolhida) | dentro de `0001_bootstrap.sql` | manteria a série em 31 arquivos (à época, com `0030`/`0031`), mas reabriria o DDL de `0001`, que o registro trata como congelável |
| D3 (recusada) | no `db/roles.sql` do provision | `src/provision/spec.md` §1 exclui privilégio de schema da série dali; `roles.sql` cria papel e mexe em `public` |

### Decisão registrada

**Decidido em 2026-11-21 pelo dono: A + S1 + C1 + D1.** Um papel `wyd_admin` com `LOGIN`, `USAGE` em `game` e em `ops`, `SELECT` e
`INSERT`/`UPDATE` nas sete tabelas do C4, sem `DELETE`, mais `INSERT` e `SELECT (id)` em
`ops.audit_log`; o `GRANT` entrou no arquivo novo `0032`. Isso mantém a escrita de staff separada da do jogo e do portal, não
reabre DDL aprovado e não cria tabela.

**O que a decisão mudou:** `ROLES` e o golden `db/roles.sql` (`src/provision/spec.md`), a DSN nova
(`WYD_DB_URL_ADMIN`) em `wyd-ops`, o arquivo `0032` (31 → 32 na série, `Tab.` 97), as citações de
contagem dos dashboards e o teste de `wyd-admin` que hoje depende deste papel. **O que ela não muda:** tabela, coluna ou índice —
as sete tabelas já existem.

**O que fechou `LAC-ADM-01`:** o "sim" do dono a A/S1/C1/D1 (2026-11-21), o papel em `ROLES`, o `GRANT`
no `0032`, `M27` fechada com data e a linha do `wyd-admin` apontando a decisão. `M28` fechou na mesma
data, em rodada própria, com a redação **B** aplicada em §5.15 e em `0030` §8.

**Divergência de prosa resolvida (`M28`, redação B):** `ddl-confirmado.md` §5.15 atribuía à "API
interna" a credencial `wyd_portal` para uma escrita que não alcança `game` (`0001`). A redação B, de
2026-11-21, corrigiu a prosa sem tocar DDL nem `GRANT` e não altera as decisões A + S1 + C1 + D1 de
`M27`.

### Redação aplicada para fechar `M28`

As duas alternativas substituíam **só prosa** — nenhum `GRANT`, nenhuma linha de DDL. A **B** foi a
escolhida e está aplicada; a **A** fica registrada como não escolhida. Os arquivos de DDL não mudaram.

**`M28` fechou em 2026-11-21 com a redação B.** A "API interna" nomeia o serviço `wyd-admin`; as
quatro tabelas de `portal` são escritas pelo BFF do portal com `wyd_portal`; o `wyd-admin` não as
escreve e, para `game` e `ops.audit_log`, usa o papel de `M27` (`0032`), nunca `wyd_portal` para `game`.
A redação A foi recusada por chamar o próprio BFF de "API interna" e divergir de `fronteira.md` §1–§2
do `wyd-admin`.

**Trechos de origem que a redação resolveu:**

- `ddl-confirmado.md` §5.15: a oração que atribuía à "API interna" a credencial `wyd_portal` para estas
  quatro tabelas foi substituída pela redação B.
- `0030-portal-content.md` §8, antes sob o rótulo **"Escrita pelo `wyd-admin`"**: o rótulo e o parágrafo
  passaram a **"Escrita pelo BFF do portal (não pelo `wyd-admin`)"**, com o papel de `M27` nomeado.

#### Redação A — não escolhida — "API interna" é o BFF administrativo do portal

**`ddl-confirmado.md` §5.15 diria:** "O papel `wyd_admin` existe para `game` e `ops.audit_log`
  (`M27`, `0032`). Estas quatro tabelas de `portal` são escritas pelo **BFF administrativo do portal**,
  com a credencial `wyd_portal`; essa credencial **não** alcança `game` (o `0001` não dá `USAGE` no
  schema a `wyd_portal`); as ações do C4 em `game` continuam sendo do `wyd-admin` (`LAC-POR-01`)."

**`0030-portal-content.md` §8, na redação A:** o rótulo passaria a **"Escrita pelo BFF do portal"** e o parágrafo seria:
  "O papel `wyd_admin` existe para `game` e `ops.audit_log` (`M27`, `0032`). As rotas
  `/admin/conteudo/*` e `/admin/launcher` usam a credencial `wyd_portal`; ela **não** cobre `game`, cujas
  ações do C4 são do `wyd-admin` (`LAC-POR-01`)."

**Efeito previsto se escolhida:** o texto identifica quem escreve as quatro tabelas sem sugerir que a mesma credencial serve ao
C4. **Fontes:** `0030` §9 (as rotas `/admin/conteudo/*` e `/admin/launcher` escrevem com `wyd_portal`, e
estas tabelas não são estado de jogo) e `fronteira.md` §7 do `wyd-admin` (o admin não escreve em
`portal`); o C4 §4.8 aplica o mesmo limite às três tabelas de economia, não a estas quatro.

#### Redação B — aplicada — "API interna" é o serviço `wyd-admin`

**`ddl-confirmado.md` §5.15 diz:** "O papel `wyd_admin` existe para `game` e
  `ops.audit_log` (`M27`, `0032`). Estas quatro tabelas de `portal` são escritas pelo **BFF do portal**
  com a credencial `wyd_portal`; o serviço `wyd-admin` **não** as escreve. `wyd_portal` não serve para
  `game` (o `0001` não dá `USAGE` no schema a `wyd_portal`)."

**`0030-portal-content.md` §8, na redação B:** o rótulo passou a **"Escrita pelo BFF do portal (não pelo `wyd-admin`)"**
  e o parágrafo ficou: "Estas quatro tabelas são escritas pelo BFF do portal com a credencial `wyd_portal`;
  o `wyd-admin` não as escreve e, para `game` e `ops.audit_log`, tem papel próprio (`M27`, `0032`) e não
  usa `wyd_portal` para `game`."

**Efeito:** explicita a fronteira que `fronteira.md` §7 já declara ("o admin **não** escreve em
`portal`") e nomeia o dono da escrita em `game`/`ops.audit_log`. **Custo aceito:** mexe também no
rótulo e repete a fronteira no texto; o C4 §4.8 é precedente (tabela de `portal` é do BFF), não a
fonte destas quatro.

| Critério | Redação A | Redação B |
|---|---|---|
| "API interna" nomeia | BFF administrativo do portal | serviço `wyd-admin` |
| Quem escreve as quatro tabelas | BFF, com `wyd_portal` | BFF, com `wyd_portal`; o texto diz que o admin não escreve |
| Tamanho da edição | uma frase em §5.15 e o parágrafo de §8 | uma frase em §5.15, o rótulo e o parágrafo de §8 |

Nas duas alternativas, o papel do `wyd-admin` em `game`/`ops.audit_log` já existe por `M27` (`0032`).
A escolha foi a **B**, decidida em 2026-11-21; `M28` está fechada em `lacunas.md` §2.3.

## O que é hash e com que algoritmo

| Dado | Forma | Regra |
|---|---|---|
| senha do portal (`portal.person`) | `argon2id`, com sal por linha | nunca reversível, nunca comparada em claro |
| PIN de conta de jogo | `argon2id`, com sal por linha | PIN curto não muda o algoritmo: o custo vem do parâmetro, não do tamanho |
| token de sessão do portal | hash do token | o valor em claro existe só no cookie do cliente |
| idempotência de crédito | chave UUID única gerada pelo BFF, janela de 24 h | impede crédito duplicado por reenvio (`M17`, `0011`/`0012`) |

Parâmetro de custo é decisão de instalação, documentada; baixar o custo para "ficar rápido" é
regressão de segurança, não ajuste de performance.

## Política de credencial (decidida em 2026-11-19)

- **PIN de conta de jogo:** 4–6 dígitos, 5 falhas consecutivas bloqueiam por 10 minutos, desbloqueio
  só por tempo; `game.account.pin_failure_count` e `pin_locked_until` são os contadores.
- **Login do portal:** o mesmo limiar — 5 falhas/10 min, desbloqueio por tempo, sem intervenção de
  staff; os contadores de `portal.person` passam a ser movidos no login.
- **Senha do portal:** 15–128 caracteres, sem regra de composição, sem expiração periódica, com
  blocklist e `argon2id` (`M9`).

## O que nunca é gravado

- Senha, PIN ou token em claro — nem em coluna "temporária", nem em log de erro.
- DSN, chave de assinatura de cookie ou token em log, em `Debug`/`Display` de struct, em artefato gerado ou no git.
- IP público do jogador: a coluna de IP existe para operação e limite de conexão, não para perfil.
- Destinatário de reset de PIN dentro de `game` (`M2`): o dado de contato fica no portal.
- IP privado em seed (`192.168.x.x`): dado de instalação não entra na série (`AGENTS` §7).

## O que é auditado e onde

| Evento | Onde fica | Quem escreve |
|---|---|---|
| ação administrativa (ban, grant, ajuste) | `ops.audit_log`, append-only | serviço que executou, com ator derivado da sessão |
| pedido de concessão vindo da web | `ops.delivery_queue`, com autor na linha | portal |
| mudança de configuração | `ops.config_revision` / `config_change` | quem editou, com valor anterior |
| sessão encerrada | `game.session.released_at` | reaper do jogo |

Auditoria não tem `CASCADE` e não é apagada por exclusão de conta: o registro sobrevive ao objeto
que ele descreve (`padrao-banco.md` §8; `M18-A`, com o DDL em `ddl-confirmado.md` §5.3).

## Ameaça considerada × controle

| Ameaça | Controle |
|---|---|
| superusuário em runtime "resolve" um bug | preflight de papel no runner + DSN por papel: runtime não tem a credencial que faz DDL |
| portal escreve estado vivo direto | fronteira testada (`permission denied`) e `delivery_queue` como ponte única (`padrao-banco.md` §2 e §8) |
| reenvio de grant dobra item ou crédito | chave de idempotência com unique (`M17`) |
| dump de `game` vaza e-mail | PII fora de `game`; teste que varre o dump (`M2`) |
| segredo em log de erro de conexão | DSN descrita por nome de variável, nunca impressa |
| conta fechada com personagem vivo | `RESTRICT` na FK; fechamento é transição explícita, não `DELETE` (`P-15`, `P-20`; `padrao-banco.md` §5, item 1) |

Parâmetros de configuração, defaults e onde cada um é lido ficam em
`guia-administrador/configuracao.md`; esta página só diz o que é segredo e por quê.

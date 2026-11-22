> Pergunta: "o que a área administrativa consome do banco, com que operação e sob qual permissão?"
> Público: Desenvolvedor novo e arquitetura
> Dono: arquitetura + `wyd-db`
> Base: `wyd-db/ddl-confirmado.md` §1/§2/§5.2/§5.5/§5.9–§5.14; `0001` (permissões); `0029` (views)
> Revisado em: 2026-11-22

**Estado:** aceito (2026-11-20); §3 corrigida em 2026-11-21 (`LAC-ADM-05`); §7.1 proposta em 2026-11-22

O que a área administrativa consome do `wyd-db`, tela por tela. Tabela citada sem arquivo é dívida:
as contagens confirmada/proposta estão em `plano-migrations.md` §3 e as que já têm DDL, em
`ddl-confirmado.md` §1.

## 1. Regra de leitura e escrita

| Regra | Efeito |
|---|---|
| `wyd_portal` não tem `USAGE` em `game` (`P-17`) | leitura de jogo só pela view `game.v_live_session`/`portal.v_person_identity` (`0029`), quando existir |
| Escrita administrativa em `game` | **bloqueada** por permissão; o BFF chama o `wyd-admin`, único executor (`LAC-POR-01`, decidida em 2026-11-20) |
| Coluna “Escreve” de §2–§6 | nomeia a **tabela afetada**, não quem executa o SQL: tabela de `game` é escrita pelo `wyd-admin` (C4); tabela de `portal`/`ops`, pelo BFF |
| Escrita em `portal` e `ops` | permitida ao `wyd_portal` pelo default privilege do `0001` |
| toda mutação | grava `ops.audit_log` na mesma transação |
| PII | e-mail e telefone nunca em log; tela mostra o mínimo e o acesso é auditado |

## 2. NOC, canais e monitoramento

| Tela | Lê | Escreve | Arquivo dono |
|---|---|---|---|
| `/admin` | `game.v_live_session`, `game.server_channel`, contagem de `game.account`/`character`, `account_wallet`; API de métricas do `wyd-ops` | — | `0005`, `0006`, `0007`, `0029`; `wyd-ops` (`LAC-POR-06`) |
| `/admin/canais` | `game.server_channel`, `game.session` (via view) | `server_channel` (capacidade, manutenção, papel) | `0007` |
| `/admin/logs` | `ops.audit_log`, `ops.login_attempt` | — | `0002` |
| `/admin/monitoramento` (38) | `game.v_live_session`; Prometheus, Loki e Grafana por HTTP | — | `0007`, `0029`; `monitoramento.md` |
| `/admin/monitoramento/servidor` (39) | `game.v_live_session`, `game.server_channel`; métricas `wyd_*` do Prometheus | — | `0007`, `0029`; `monitoramento.md` |
| `/admin/monitoramento/prometheus` (40) | Prometheus `/api/v1/query` e `/api/v1/alerts` | — | `wyd-ops` |
| `/admin/monitoramento/grafana` (41) | Grafana `/api/health` e `/api/search` | — | `wyd-ops` |
| `/admin/monitoramento/logs` (42) | Loki `query` e `query_range` | — | `wyd-ops` |
| `/admin/monitoramento/launcher` (43) | agregado em processo do BFF + log estruturado no Loki | — | `monitoramento.md` §3 |

**Limite declarado:** CPU, RAM, latência, logs, painéis e telemetria do launcher **não** vêm do banco.
O NOC e as seis telas de monitoramento (38–43) leem do `wyd-db` sessão viva, capacidade configurada e
ocupação; todo o resto vem das **APIs do Prometheus, do Loki e do Grafana** operadas pelo `wyd-ops`
(`LAC-POR-06`, decidida e reconfirmada em 2026-11-20). O `/metrics` do `wyd-server` é a fonte, a pilha
coleta e retém, e o BFF consulta por HTTP — sem tabela de série em `portal` ou `ops`. Falha de consulta
ou métrica ausente aparece como **“não medido”**, nunca `0`; a fonte e a degradação de cada número
estão em `monitoramento.md` §2/§8.

## 3. Jogadores, contas e moderação

| Tela | Lê | Escreve | Arquivo dono |
|---|---|---|---|
| `/admin/jogadores` | `game.character` (join por conta), `game.session` | — | `0006`, `0007` |
| `/admin/jogadores/[id]` | `character`, `character_item`, `vault_item`, `character_skill`, `character_affect`, `guild_member`, `ops.audit_log` | — (somente leitura) | `0006`, `0008`, `0009`, `0010` |
| `/admin/contas` | `game.account`, `portal.person` (via view) | — | `0005`, `0003`, `0029` |
| `/admin/contas/[id]` | `account`, `account_wallet`, `account_ledger`, `account_moderation`, `ops.login_attempt` | — (bloqueio/desbloqueio executado pelo `wyd-admin`, C4 §4.1) | `0005`, `0002` |
| `/admin/seguranca` | `portal.person`, `person_session`, `account_moderation`, `person_authenticator` | `person_session` (revogar), `person_authenticator` (reset de staff); bloqueio/desbloqueio de conta pelo `wyd-admin` (C4 §4.1) | `0003`, `0004`, `0005`, `0031` |

**Corrigido em 2026-11-21 (`LAC-ADM-05`, fechada no `wyd-admin`):** a escrita de
`game.account_moderation` é do `wyd-admin`, nunca do portal. `P-17` nega `USAGE` em `game` a
`wyd_portal`, e o C4 fixa `POST /accounts/{account_id}/moderation` como única via
(`fronteira.md` §3; `wyd-admin/docs/arquitetura/contrato-c4.md` §4.1). Ler a moderação segue a via
autorizada de leitura (`P-17`, adapter do `wyd-db` como em `LAC-POR-04`), não conexão SQL do BFF. A
correção é de documento: nenhuma coluna, constraint, índice ou `GRANT` mudou — a tabela continua a de
`0005` (`ddl-confirmado.md` §5.2).

**Segundo fator:** `admin`/`moderator` autentica com senha **e** TOTP (`LAC-POR-05`, `A20`); o
segredo do autenticador nunca é exibido nem logado, e a recuperação é ato de staff. `member` não tem
segundo fator na `1.0.0`.

**Nunca exposto na tela:** `password_hash`, `pin_hash`, `token_hash`, segredo TOTP e o valor de token em claro. O
admin vê metadados (existe? quando? expira?), não o segredo. Reset de PIN é pedido por token, não por
leitura de hash (`0005.pin_reset`).

## 4. Rankings e economia

| Tela | Lê | Escreve | Arquivo dono |
|---|---|---|---|
| `/admin/economia` | `account_wallet`, `account_ledger`, `character.coin`, `city_tax_ledger`, agregados de `character_item` | — | `0005`, `0006`, `0009`, `0011` |
| Home `/` e `/rankings` | `game.character`, `game.guild` (consulta via adapter autorizado do `wyd-db`, com `LIMIT`) | — | tabelas de origem; cache compartilhado do BFF, TTL 60 s (`LAC-POR-04`) |
| `/admin/economia/cidades` | `city`, `personal_shop`, `city_tax_ledger`, `item_price`, `kingdom_cape_price` | `city` (imposto), `item_price`, `kingdom_cape_price` | `0011` |
| `/admin/economia` (cash) | `donate_credit`, `donate_debit`, `donate_shop_item`, `daily_reward_item`, `daily_reward_claim` | `donate_shop_item`, `daily_reward_item` | `0011` |

Ajuste manual de saldo (`account_wallet.coin`) **não** é escrita direta de tela: é lançamento em
`account_ledger` com motivo, na mesma transação (`M16`).

## 5. Conteúdo, CMS e launcher

| Tela | Lê | Escreve | Arquivo dono |
|---|---|---|---|
| `/admin/conteudo` | contagens de `content.item_catalog`, `skill_catalog`, `npc_definition` | — | `0013`–`0019` |
| `/admin/conteudo/wydpedia` | `content.*` | **`H`**: revisão de catálogo ainda não tem tabela própria (depende do lote de conteúdo) | `LAC-POR-02` (fechada para notícia/guia) |
| `/admin/conteudo/noticias` | `portal.news`, `portal.banner`, `portal.guide` | mesma tabela, com estado de publicação e autor | `0030` |
| `/admin/launcher` | `portal.launcher_release` | versão, notas e URL do manifesto assinado | `0030` |

**Decidido em 2026-11-20 (`LAC-POR-02`, `LAC-POR-03`):** notícia, banner e guia são **três tabelas em
`portal`** (`news`, `banner`, `guide`), e o launcher ganha **uma** tabela em `portal`
(`launcher_release`) com versão, notas e URL do manifesto assinado — o binário e a assinatura ficam no
`wyd-ops`. Essas **4 tabelas não existem nas 92 originais** de `plano-migrations.md` §3: o lote
`0030_portal_content.sql` foi desenhado e aprovado em 2026-11-20 e está em
`planejamento-1.0.0/wyd-db/migrations/0030-portal-content.md` (texto DDL em `ddl-confirmado.md` §5.15),
com colunas, `PK`/`FK`, `DEFAULT`, índices e constraints. Com a aprovação, a série ficou em **30
arquivos / 96 tabelas** naquela data; com os lotes `0031` a `0034`, são **34 arquivos / 109 tabelas**, e as cinco tabelas dos dois adendos têm DDL fechado. É o número que este planejamento cita.

## 6. Configuração do servidor

| Tela | Lê | Escreve | Arquivo dono |
|---|---|---|---|
| `/admin/config` | `ops.config_revision`, `ops.config_change` | `ops.config_revision` + `ops.config_change` (mesma transação) | `0002` §5.9 |

Contrato já decidido: revisão com autor tripartido (`M18-A`), valores antes/depois em linhas
nomeadas, sem `JSONB`. A tela não inventa “chave mágica”: lista as chaves que o servidor consome.

## 7. Catálogo de tabelas citadas (52 + 2 views)

| Schema | Tabelas |
|---|---|
| `portal` | `person`, `person_profile`, `person_consent`, `person_session`, `person_token`, `person_authenticator`, `donate_shop_item`, `daily_reward_item`, `daily_reward_claim`, `kingdom_cape_price`, `donate_credit`, `donate_debit`, `news`, `banner`, `guide`, `launcher_release`, `market_wallet`, `market_ledger`, `market_fee_rule`, `market_listing`, `market_bid`, `market_transaction`, `market_review`, `market_reputation`, `market_notification`, `market_dispute` |
| `game` | `account`, `account_wallet`, `account_ledger`, `account_moderation`, `character`, `character_item`, `vault_item`, `character_skill`, `character_learned_skill`, `character_affect`, `guild`, `guild_member`, `session`, `server_channel`, `city`, `item_price`, `personal_shop`, `city_tax_ledger`, `market_escrow` |
| `ops` | `audit_log`, `login_attempt`, `config_revision`, `config_change` |
| `content` | `item_catalog`, `skill_catalog`, `npc_definition` (previstas) |
| views | `game.v_live_session`, `portal.v_person_identity` (`0029`, previstas) |

A lista por tela repete tabela entre telas; acima estão as **tabelas distintas** citadas: 26 de
`portal`, todas com DDL, 19 de `game`, 4 de `ops` e 3 de
`content` (2 com DDL e `npc_definition` prevista em `0019`), mais as 2 views de `0029` — **52 citadas,
das quais 51 têm DDL fechado**, + 2 views, medido neste arquivo em 2026-11-22 (`character_learned_skill`
entrou com a armaria e as 11 do mercado com o lote `0034`).

O monitoramento **não acrescenta tabela**: presença vem da view `game.v_live_session`, e métrica, log,
painel e telemetria do launcher vêm de API externa ou de log estruturado (`monitoramento.md` §3/§6).
O lote `0031` acrescentou `portal.person_authenticator`, o `0032` só concede `GRANT` e o `0034` trouxe o
mercado: a série está em **34 arquivos / 109 tabelas**, e o catálogo subiu de 39 para **52 citadas** (51
com DDL).

### 7.1 Views propostas pela armaria (F7–F11)

Quatro views que ainda **não têm DDL** e por isso ficam fora da contagem acima: a forma é de
`wyd-db/armaria-e-mercado.md` §2 e a reserva de lote é decisão de `wyd-db`. As onze **tabelas** da armaria
e do mercado entraram na tabela principal em 2026-11-22, com o lote `0034` (`M31`).

| Schema | Objetos |
|---|---|
| views | `game.v_character_armory`, `game.v_character_equipment`, `game.v_character_skill`, `content.v_item_sheet` |

## 8. O que esta página não decide

| Assunto | Onde fecha |
|---|---|
| Forma do contrato HTTP do `wyd-admin` | proposto em 2026-11-22 em `wyd-admin/docs/arquitetura/contrato-c4.md` (inclui a custódia de item, §4.9–§4.13) |
| Lote das 4 tabelas novas em `portal` | `wyd-db` (dono da série) |
| Séries históricas de métricas | `wyd-ops` (Prometheus/Grafana); o portal só consulta a API (`LAC-POR-06`) |
| DDL de `portal.person_authenticator` | fechado no lote `0031` (`ddl-confirmado.md` §5.20); mecanismo em `LAC-POR-05` |
| Contrato do adapter de leitura de `game` para ranking, sem `USAGE` SQL no portal | `wyd-db`/`wyd-admin`; decisão e TTL em `LAC-POR-04` |
| Fontes, variáveis e degradação do monitoramento | `monitoramento.md` §4/§8; intervalo, credencial e retenção definitiva no `wyd-ops` |

*Página de planejamento. Nenhuma consulta foi escrita ou executada.*

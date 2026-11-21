> Pergunta: "quais telas o portal tem, em que rota cada uma vive e quem pode abrir o quê?"
> Público: Desenvolvedor novo, produto e staff
> Dono: produto + arquitetura
> Base: `M21`; `0003`/`0004` (papéis e sessão); `wyd-portal-prompt.md` §5/§6; `consumo-do-banco.md`
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

Lista de telas da `1.0.0` com rota, dono do dado e nível de acesso. Rota que não está aqui não entra
sem passar por esta página; tela sem permissão escrita é tela sem contrato.

## 1. Níveis de acesso

O papel vem de `portal.person.role` (`0003`): `member`, `moderator`, `admin`. Papel de jogo
(`game.account.role`) **não** abre tela administrativa; serve ao servidor.

| Nível | Quem | Como o BFF decide |
|---|---|---|
| `público` | qualquer visitante | sem sessão |
| `titular` | pessoa autenticada | sessão válida **e** `person_id` = dono do recurso |
| `member` | pessoa autenticada | sessão válida |
| `staff` | `moderator` ou `admin` | `role IN ('moderator','admin')` |
| `admin` | `admin` apenas | `role = 'admin'`, após segundo fator (`LAC-POR-05`) |
| `staff` (MFA) | `moderator`/`admin` com TOTP confirmado | `role IN ('moderator','admin')` **e** autenticador confirmado (`RF-POR-013`) |

## 2. Portal público e conta (22 rotas)

| # | Rota | Tela | Acesso | Dado |
|---:|---|---|---|---|
| 1 | `/` | Home (destaques, status, rankings resumidos) | `público` | `content`, agregados |
| 2 | `/noticias` | Lista de notícias | `público` | `portal.news` (lote novo) |
| 3 | `/noticias/[slug]` | Notícia | `público` | `portal.news` |
| 4 | `/rankings` | Rankings (nível, PvP, guilda) | `público` | `game.character`, `game.guild` |
| 5 | `/wydpedia` | Entrada da enciclopédia e busca | `público` | `content` |
| 6 | `/wydpedia/itens` | Catálogo de itens com filtros | `público` | `content.item_catalog` |
| 7 | `/wydpedia/itens/[index]` | Item, tooltip e drops | `público` | `content`, `game` agregado |
| 8 | `/wydpedia/skills` | Skills e linhagens | `público` | `content.skill_catalog` |
| 9 | `/wydpedia/mobs` | Bestiário | `público` | `content.npc_*`, `mob_template_stat` |
| 10 | `/wydpedia/guias/[slug]` | Guia/manual editorial | `público` | `portal.guide` (lote novo) |
| 11 | `/atlas` | Atlas de mapas | `público` | `content.map_definition` |
| 12 | `/atlas/[mapa]` | Mapa com pins | `público` | `content`, `0019` |
| 13 | `/download` | Download e requisitos | `público` | `portal.launcher_release` (lote novo) |
| 14 | `/login` | Login | `público` | `portal.person` |
| 15 | `/cadastro` | Cadastro (nasce `pending`) | `público` | `portal.person` |
| 16 | `/recuperar-senha` | Pedido de recuperação | `público` | `person_token` |
| 17 | `/redefinir-senha/[token]` | Consumo do token | `público` | `person_token` |
| 18 | `/verificar-email/[token]` | Verificação de e-mail | `público` | `person_token` |
| 19 | `/conta` | Resumo da conta | `titular` | `portal.*` |
| 20 | `/conta/personagens` | Personagens e vaga | `titular` | `game.character` (view) |
| 21 | `/conta/seguranca` | Senha, sessões e PIN | `titular` | `person_session`, `0005` |
| 22 | `/conta/consentimentos` | Consentimentos e revogação | `titular` | `person_consent` |

## 3. Área administrativa (21 rotas)

| # | Rota | Tela | Acesso | Dado |
|---:|---|---|---|---|
| 23 | `/admin` | NOC: KPIs, online, pico, economia, CPU/RAM/latência | `staff` | `v_live_session`, `account`, `character`; API de métricas do `wyd-ops` |
| 24 | `/admin/jogadores` | Busca de personagens | `staff` | `character`, `session` |
| 25 | `/admin/jogadores/[id]` | Ficha: inventário, baú, skills, guilda, punições | `staff` | `0008`, `0009`, `0010`, `audit_log` |
| 26 | `/admin/contas` | Contas e vínculo com pessoa | `staff` | `account`, `person` |
| 27 | `/admin/contas/[id]` | Conta: carteira, ledger, moderação, login | `staff` | `account_wallet`, `account_ledger`, `account_moderation`, `login_attempt` |
| 28 | `/admin/economia` | Visão econômica e inflação | `staff` | `account_wallet`, `city_tax_ledger`, `0009` |
| 29 | `/admin/economia/cidades` | Cidades, imposto e lojas | `admin` | `city`, `personal_shop`, `city_tax_ledger`, `item_price` |
| 30 | `/admin/conteudo` | Painel de conteúdo | `staff` | `content.*` |
| 31 | `/admin/conteudo/noticias` | CMS de notícias e banners | `admin` | `portal.news`, `portal.banner` (lote novo) |
| 32 | `/admin/conteudo/wydpedia` | Revisão do catálogo | `admin` | `content.item_catalog`, `skill_catalog` |
| 33 | `/admin/launcher` | Versões, manifesto e notas | `admin` | `portal.launcher_release` (lote novo) |
| 34 | `/admin/logs` | Auditoria e tentativas de login | `staff` | `ops.audit_log`, `ops.login_attempt` |
| 35 | `/admin/seguranca` | Staff, papéis, bloqueios e recuperação de segundo fator | `admin` | `person`, `person_session`, `account_moderation`, `person_authenticator` |
| 36 | `/admin/config` | Configuração do servidor por revisão | `admin` | `ops.config_revision`, `ops.config_change` |
| 37 | `/admin/canais` | Canais, manutenção e capacidade | `admin` | `server_channel`, `session` |
| 38 | `/admin/monitoramento` | Resumo das fontes, motor por canal e alertas | `staff` | Prometheus, Loki, Grafana, `game.v_live_session` |
| 39 | `/admin/monitoramento/servidor` | Motor e host por canal, presença viva | `staff` | `wyd_*` no Prometheus, `game.v_live_session` |
| 40 | `/admin/monitoramento/prometheus` | Alvos, coletor e alertas avaliados | `staff` | Prometheus `/api/v1/query`, `/api/v1/alerts` |
| 41 | `/admin/monitoramento/grafana` | Catálogo de dashboards e painel embutido | `staff` | Grafana `/api/health`, `/api/search` |
| 42 | `/admin/monitoramento/logs` | Volume, erros e avisos por serviço; export NDJSON | `staff` | Loki `query`, `query_range` |
| 43 | `/admin/monitoramento/launcher` | Telemetria anônima por versão e operação | `staff` | `POST /api/launcher/telemetria` + log no Loki |

**Total: 43 rotas** — 22 públicas/de conta e 21 administrativas. Rota nova entra aqui antes de virar
código. As seis de monitoramento (38–43) preservam, uma por uma, as telas do portal legado: a paridade
e a fonte de cada número estão em `monitoramento.md` §1/§7.

## 4. Matriz de permissão por domínio

| Domínio | `público` | `member` | `staff` (`moderator`) | `admin` |
|---|---|---|---|---|
| Portal público | lê | lê | lê | lê |
| Conta e personagens | — | lê/escreve o próprio | — | — |
| NOC e canais | — | — | lê | lê/escreve |
| Monitoramento (38–43) | — | — | lê | lê |
| Jogadores e contas | — | — | lê | lê/escreve |
| Economia | — | — | lê | lê/escreve |
| Conteúdo/CMS | lê publicado | lê publicado | lê | lê/escreve |
| Launcher | lê versão | lê versão | lê | lê/escreve |
| Logs e auditoria | — | — | lê | lê |
| Configuração do servidor | — | — | — | lê/escreve |

**Regra de segundo fator:** `moderator`/`admin` só recebem sessão depois de senha + TOTP; `member`
continua só com senha na `1.0.0` (`LAC-POR-05`, `A20`). A tabela do autenticador é de `portal` e o
DDL está no lote `0031` do `wyd-db`.

**Regra de escrita administrativa:** toda mutação passa pelo BFF, grava o autor
(`actor_kind='person'`, `actor_id`) em `ops.audit_log` e, quando for configuração, abre revisão em
`ops.config_revision` com as mudanças em `ops.config_change` (`0002` §5.9). Sem trilha, a rota não
entra. Mutação de estado de jogo **não** é SQL do portal: ela chama o `wyd-admin` (`LAC-POR-01`,
decidida em 2026-11-20).

## 5. Rotas e o que elas **não** prometem

| Rota | Não faz | Por quê |
|---|---|---|
| `/admin/jogadores/[id]` | kick/ban por SQL | ação vai pelo `wyd-admin`; o portal não tem `USAGE` em `game` |
| `/admin/canais` | reiniciar processo | o portal altera configuração; reinício é ação do `wyd-admin` |
| `/admin/launcher` | publicar patch ou binário | o portal publica versão, notas e URL do manifesto; arquivo é `wyd-ops` |
| `/admin/conteudo/*` | publicar antes do lote | `news`/`banner`/`guide` estão no lote `0030`, aprovado em 2026-11-20 (série 31/97) |
| `/rankings` | ranking em tempo real garantido | cache/`ISR`; o número é “última medição”, não promessa de instante |
| `/admin` | guardar série ou substituir o Grafana | telemetria histórica, retenção e alerta são do `wyd-ops`; o portal consulta a API e mostra “não medido” quando ela falha (`LAC-POR-06`) |
| `/admin/monitoramento/grafana` | redesenhar os gráficos no portal | os dashboards são provisionados por arquivo no `wyd-ops`; o portal embute o painel |
| `/admin/monitoramento/logs` | guardar log ou aceitar LogQL do usuário | o log é do Loki (14 dias); filtros são allowlist e texto livre vira match literal |
| `/admin/monitoramento/launcher` | identificar jogador | o evento é anônimo e o contrato é fechado; nada de login, IP ou texto livre |

## 6. O que esta página não decide

| Assunto | Onde fecha |
|---|---|
| Forma do contrato HTTP do `wyd-admin` | planejamento do `wyd-admin` |
| DDL de `portal.person_authenticator` | fechado no lote `0031` (`ddl-confirmado.md` §5.20); mecanismo e dono em `LAC-POR-05` |
| Contrato de métricas, logs e painéis (URL, credencial, intervalo, retenção) | `monitoramento.md` §4/§6 fixa as fontes e os nomes de variável; o valor e a retenção definitiva são do `wyd-ops` |
| Rotas do launcher e do cliente | planejamento do `wyd-launcher` |

*Página de planejamento. Nenhuma rota foi implementada e nenhum componente foi criado.*

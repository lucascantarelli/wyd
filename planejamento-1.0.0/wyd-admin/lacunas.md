> Pergunta: "o que ainda não está decidido no `wyd-admin`, quem decide e o que fica bloqueado?"
> Público: Arquiteto, produto e operação
> Dono: equipe do wyd-admin
> Base: [contrato C4](docs/arquitetura/contrato-c4.md); `wyd-portal/docs/arquitetura/fronteira.md`; `AGENTS.md` §5, item 8
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); `LAC-ADM-07` abriu em 2026-11-22 com a custódia de item e fechou na
mesma data, junto de `LAC-ADM-02` e `LAC-ADM-06`. **Zero lacunas bloqueadas:** as duas
últimas (`LAC-ADM-03` e `LAC-ADM-04`) fecharam em **2026-11-22**, com o "sim" do dono às opções
recomendadas de `docs/arquitetura/autenticacao-e-controle.md`. Nenhuma foi fechada por suposição, e
nenhuma decisão de transporte virou schema.

Status possíveis: **fechada** (com data e evidência no documento dono) e **bloqueada** (com o
desbloqueio nomeado — dono e documento). Hoje não há linha bloqueada.

## Fechadas

| ID | Assunto | Decisão | Data |
|---|---|---|---|
| — | Executor único de escrita em `game` | `LAC-POR-01`: portal chama o `wyd-admin` por HTTP/JSON; nunca SQL direto | 2026-11-20 |
| — | Transporte do C4 | HTTP/JSON interno com timeout, chave de idempotência e erro tipado | 2026-11-20 |
| `LAC-ADM-05` | Escrita de `game.account_moderation` | É do `wyd-admin`, por `POST /accounts/{account_id}/moderation` (C4 §4.1); o portal só lê, pela via autorizada de `P-17`. O documento do portal foi corrigido e o C4 não mudou | 2026-11-21 Refletida em `planejamento-1.0.0/wyd-admin/docs/arquitetura/contrato-c4.md`. |
| `LAC-ADM-01` | Papel de banco do `wyd-admin` | `M27` fechada no `wyd-db` em A + S1 + C1 + D1: `wyd_admin` com `LOGIN` em `ROLES`/golden, `USAGE` em `game`/`ops` e os `GRANT`s no `crates/wyd-db/migrations/0032-admin-grants.sql` (`planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.32). O `0032` foi **emendado** em 2026-11-22 com os `GRANT` da custódia (`character_item`, `character`), que a decisão original não cobria | 2026-11-21 |
| `LAC-ADM-02` | Tabela do registro de idempotência | `M30` fechada no `wyd-db`: **tabela nova** `ops.idempotency_record` no lote `0033` (`ddl-confirmado.md` §5.33), com PK composta `(caller, method, route, key)`, `body_sha256` e a linha escrita **e fechada na mesma transação da ação** — `in_progress` nunca é commitado, então o `409 IN_PROGRESS` vem do bloqueio da PK e não há chave presa. A contagem da série vai de 97 para 109 tabelas com os dois lotes de 2026-11-22 | 2026-11-22 |
| `LAC-ADM-06` | Retenção do registro de idempotência | `M30`, mesma decisão: **7 dias após `created_at`**, qualquer estado, por expurgo diário de operação. O piso é a janela de replay de 24 h de `M17` — apagar antes permitiria executar duas vezes — e o teto é o custo de guardar corpo de resposta de ação de staff. Depois do expurgo, `GET /actions/{key}` responde `404`; `in_progress` não é um estado observável de outra sessão. O número é **decisão**, não medição | 2026-11-22 Refletida em `planejamento-1.0.0/wyd-admin/docs/arquitetura/contrato-c4.md`. |
| `LAC-ADM-03` | Contrato de controle com o `wyd-server` (kick, broadcast, manutenção) | **decidida em 2026-11-22 (opção A):** HTTP interno **síncrono** — o admin chama `POST /internal/v1/control` no processo do canal e recebe `200` com `{delivered_to}`; o mapa canal → URL é configuração do `wyd-ops` (`WYD_SERVER_CONTROL_URLS`) e o elo tem segredo próprio (`WYD_SERVER_CONTROL_TOKEN`). Sem ack não há escrita, então `released_at` só é gravado depois do `200`. **Sem tabela:** a série continua em 34 arquivos / 109 tabelas. Recusadas a B (fila `ops.server_command`, ack assíncrono e +1 tabela → 110/35) e a C (broker externo, dependência nova) | 2026-11-22 Refletida em `planejamento-1.0.0/wyd-admin/docs/arquitetura/adr/adr-adm-0001-canal-controle.md`. |
| `LAC-ADM-04` | Autenticação portal → admin | **decidida em 2026-11-22 (opção A):** token de serviço **por ambiente** em `Authorization: Bearer` sobre TLS, com `WYD_ADMIN_SERVICE_TOKEN` e `WYD_ADMIN_SERVICE_TOKEN_NEXT` aceitos durante a rotação e comparação em tempo constante. O admin não autentica pessoa: confia em `X-Staff-Person-Id` porque só o BFF detém o token, e o papel continua vindo de `RN-ADM-001`. **Sem tabela e sem `GRANT`.** Recusadas a B (mTLS, exige PKI interna para um único chamador), a C (HMAC por requisição, com cache de nonce) e a D (OIDC client credentials, provedor novo) | 2026-11-22 Refletida em `planejamento-1.0.0/wyd-admin/docs/arquitetura/autenticacao-e-controle.md`. |
| `LAC-ADM-07` | DDL de `game.market_escrow` e de `portal.market_*` | `M31` fechada no `wyd-db`: lote `crates/wyd-db/migrations/0034-portal-market.sql`, **onze tabelas** — dez em `portal` (carteira, extrato, taxa, anúncio, lance, transação, avaliação, reputação, notificação, disputa) e `game.market_escrow` para a custódia — no texto de `ddl-confirmado.md` §5.34 e na página `planejamento-1.0.0/wyd-db/migrations/0034-portal-market.md`; a série passa a **34 arquivos e 109 tabelas** e o `0032` é emendado com os dois `GRANT` da custódia | 2026-11-22 |

## Como uma lacuna fecha

1. O dono decide e a decisão entra no documento dono (aqui ou no `wyd-db`/`wyd-server`).
2. A linha muda de seção com **data** e o ID citado no requisito afetado.
3. Se a decisão mudar schema, contagem ou fronteira, ela passa pelo dono da série antes de virar DDL.
4. `bloqueada` só vira `fechada` com o "sim" registrado no documento dono; sem ele, a linha continua
   aqui — não existe fechamento por silêncio nem por escolha do agente.

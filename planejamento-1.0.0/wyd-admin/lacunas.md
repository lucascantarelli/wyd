> Pergunta: "o que ainda não está decidido no `wyd-admin`, quem decide e o que fica bloqueado?"
> Público: Arquiteto, produto e operação
> Dono: equipe do wyd-admin
> Base: [contrato C4](docs/arquitetura/contrato-c4.md); `wyd-portal/docs/arquitetura/fronteira.md`; `AGENTS.md` §5.8
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); `LAC-ADM-07` abriu em 2026-11-22 com a custódia de item e fechou na
mesma data, junto de `LAC-ADM-02` e `LAC-ADM-06`. **Nenhuma lacuna `aberta`**; duas estão `proposta`
(`LAC-ADM-03` e `LAC-ADM-04`), com opções, trade-off e recomendação escritas em
`docs/arquitetura/autenticacao-e-controle.md` em 2026-11-22 — **aguardando o "sim" do dono**. Nenhuma
foi fechada por suposição, e nenhuma decisão pendente virou schema.

## Propostas — aguardando aprovação do dono

Estado `proposta`: existe recomendação técnica registrada e falta o "sim" (`AGENTS.md` §5.8). O texto
das opções e do trade-off é de `docs/arquitetura/autenticacao-e-controle.md`, dono único; aqui fica só o
índice.

| ID | Lacuna | Por que importa | Dono | Proposta (2026-11-22) |
|---|---|---|---|---|
| `LAC-ADM-03` | Não existe contrato de controle com o `wyd-server` (kick, broadcast, manutenção) | `ADM-08` não pode ser implementada; “chamar o processo” hoje é intenção | arquitetura + `wyd-server` | **A recomendada:** HTTP interno síncrono `POST /internal/v1/control` no processo do canal, ack `200` com `delivered_to`, mapa canal → URL em configuração do `wyd-ops` e segredo próprio do elo; **sem** mudar schema. Recusadas: fila `ops.server_command` (ack assíncrono, +1 tabela → 110/35) e broker externo (nova dependência de infraestrutura). Fecha com o "sim" + endpoint nomeado no plano do `wyd-server` |
| `LAC-ADM-04` | Autenticação portal → admin não decidida: token de serviço, mTLS ou assinatura do ator | `RNF-ADM-002` e `RN-ADM-002` dependem disso; sem forma, `ADM-02` para | arquitetura + `wyd-ops` | **A recomendada:** token de serviço estático por ambiente em `Authorization: Bearer` sobre TLS, com **dois valores aceitos na rotação** e comparação em tempo constante; **sem** tabela nem `GRANT`. Recusadas: mTLS (PKI interna para um único chamador), HMAC por requisição (cache de nonce que a 1.0.0 não tem) e OIDC client credentials (provedor novo). Fecha com o "sim" + as variáveis nomeadas no `wyd-ops` |

## Fechadas

| ID | Assunto | Decisão | Data |
|---|---|---|---|
| — | Executor único de escrita em `game` | `LAC-POR-01`: portal chama o `wyd-admin` por HTTP/JSON; nunca SQL direto | 2026-11-20 |
| — | Transporte do C4 | HTTP/JSON interno com timeout, chave de idempotência e erro tipado | 2026-11-20 |
| `LAC-ADM-05` | Escrita de `game.account_moderation` | É do `wyd-admin`, por `POST /accounts/{account_id}/moderation` (C4 §4.1); o portal só lê, pela via autorizada de `P-17`. O documento do portal foi corrigido e o C4 não mudou | 2026-11-21 |
| `LAC-ADM-01` | Papel de banco do `wyd-admin` | `M27` fechada no `wyd-db` em A + S1 + C1 + D1: `wyd_admin` com `LOGIN` em `ROLES`/golden, `USAGE` em `game`/`ops` e os `GRANT`s no `0032_admin_grants.sql` (`ddl-confirmado.md` §5.32). O `0032` foi **emendado** em 2026-11-22 com os `GRANT` da custódia (`character_item`, `character`), que a decisão original não cobria | 2026-11-21 |
| `LAC-ADM-02` | Tabela do registro de idempotência | `M30` fechada no `wyd-db`: **tabela nova** `ops.idempotency_record` no lote `0033` (`ddl-confirmado.md` §5.33), com PK composta `(caller, method, route, key)`, `body_sha256` e a linha escrita **e fechada na mesma transação da ação** — `in_progress` nunca é commitado, então o `409 IN_PROGRESS` vem do bloqueio da PK e não há chave presa. A contagem da série vai de 97 para 109 tabelas com os dois lotes de 2026-11-22 | 2026-11-22 |
| `LAC-ADM-06` | Retenção do registro de idempotência | `M30`, mesma decisão: **7 dias após `created_at`**, qualquer estado, por expurgo diário de operação. O piso é a janela de replay de 24 h de `M17` — apagar antes permitiria executar duas vezes — e o teto é o custo de guardar corpo de resposta de ação de staff. Depois do expurgo, `GET /actions/{key}` responde `404`; `in_progress` não é um estado observável de outra sessão. O número é **decisão**, não medição | 2026-11-22 |
| `LAC-ADM-07` | DDL de `game.market_escrow` e de `portal.market_*` | `M31` fechada no `wyd-db`: lote `0034_portal_market.sql`, **onze tabelas** — dez em `portal` (carteira, extrato, taxa, anúncio, lance, transação, avaliação, reputação, notificação, disputa) e `game.market_escrow` para a custódia — no texto de `ddl-confirmado.md` §5.34 e na página `migrations/0034-portal-market.md`; a série passa a **34 arquivos e 109 tabelas** e o `0032` é emendado com os dois `GRANT` da custódia | 2026-11-22 |

## Como uma lacuna fecha

1. O dono decide e a decisão entra no documento dono (aqui ou no `wyd-db`/`wyd-server`).
2. A linha muda de seção com **data** e o ID citado no requisito afetado.
3. Se a decisão mudar schema, contagem ou fronteira, ela passa pelo dono da série antes de virar DDL.
4. `proposta` só vira `fechada` com o "sim" registrado no documento dono; sem ele, a linha continua
   aqui — não existe fechamento por silêncio nem por escolha do agente.

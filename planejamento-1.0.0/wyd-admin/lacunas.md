> Pergunta: "o que ainda não está decidido no `wyd-admin`, quem decide e o que fica bloqueado?"
> Público: Arquiteto, produto e operação
> Dono: equipe do wyd-admin
> Base: [contrato C4](docs/arquitetura/contrato-c4.md); `wyd-portal/docs/arquitetura/fronteira.md`; `AGENTS.md` §5.8
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21). Quatro lacunas abertas; nenhuma foi fechada por suposição.

## Abertas

| ID | Lacuna | Por que importa | Dono | Fecha com |
|---|---|---|---|---|
| `LAC-ADM-02` | Não há tabela para o registro de idempotência | sem armazenamento durável, retry duplica escrita; memória não sobrevive a restart | `wyd-db` | decisão entre tabela nova em `ops` (muda contagem de 97) ou reutilização de chave natural já única; DDL e contagem atualizada |
| `LAC-ADM-03` | Não existe contrato de controle com o `wyd-server` (kick, broadcast, manutenção) | `ADM-08` não pode ser implementada; “chamar o processo” hoje é intenção | arquitetura + `wyd-server` | porta/protocolo, ack, timeout e erro; página no `wyd-server` |
| `LAC-ADM-04` | Autenticação portal → admin não decidida: token de serviço, mTLS ou assinatura do ator | `RN-ADM-002` e `RN-ADM-005` dependem disso; sem forma, `ADM-02` para | arquitetura + `wyd-ops` | mecanismo, rotação de segredo e o que vai no cabeçalho |
| `LAC-ADM-06` | Retenção do registro de idempotência: a proposta usa 24 h, sem fonte | define tamanho, expurgo e o que `GET /actions/{key}` ainda responde | `wyd-db` + `wyd-ops` | prazo e rotina de expurgo, com data |

## Fechadas

| ID | Assunto | Decisão | Data |
|---|---|---|---|
| — | Executor único de escrita em `game` | `LAC-POR-01`: portal chama o `wyd-admin` por HTTP/JSON; nunca SQL direto | 2026-11-20 |
| — | Transporte do C4 | HTTP/JSON interno com timeout, chave de idempotência e erro tipado | 2026-11-20 |
| `LAC-ADM-05` | Escrita de `game.account_moderation` | É do `wyd-admin`, por `POST /accounts/{account_id}/moderation` (C4 §4.1); o portal só lê, pela via autorizada de `P-17`. O documento do portal foi corrigido e o C4 não mudou | 2026-11-21 |
| `LAC-ADM-01` | Papel de banco do `wyd-admin` | `M27` fechada no `wyd-db` em A + S1 + C1 + D1: `wyd_admin` com `LOGIN` em `ROLES`/golden, `USAGE` em `game`/`ops` e os `GRANT`s no `0032_admin_grants.sql` (`ddl-confirmado.md` §5.32), sem `DELETE` | 2026-11-21 |

## Como uma lacuna fecha

1. O dono decide e a decisão entra no documento dono (aqui ou no `wyd-db`/`wyd-server`).
2. A linha muda de seção com **data** e o ID citado no requisito afetado.
3. Se a decisão mudar schema, contagem ou fronteira, ela passa pelo dono da série antes de virar DDL.

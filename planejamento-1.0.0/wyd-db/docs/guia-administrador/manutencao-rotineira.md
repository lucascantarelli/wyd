> Pergunta: "o que faço por rotina, com qual credencial e onde fica o registro?"
> Público: Operação
> Dono: equipe do `wyd-db` com a operação
> Base: `pendencias-de-fechamento.md` §6.6.2 (`Q-BLK-02` = A) e §6.6.4 (`Q-BLK-04` = A, 2026-11-24); `docs/proposta-retencao-pendente.md` (`Q-RET-1/2/4/6`); [`configuracao.md`](configuracao.md); [`sustentacao.md`](sustentacao.md)
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Rotina é o que roda **sem decisão nova**. O que depende de medição (`M20`) ou de credencial nomeada
entra como pendência declarada (§8), não como tarefa com número inventado.

## 1. Quem executa, com qual credencial

**Decidido em 2026-11-24:** operador do banco (função, [`sustentacao.md`](sustentacao.md) §3) rodando
por `cron` do host, no Compose (`Q-BLK-02` = A). Sem `pg_cron`, sem extensão e sem agendador dentro do
banco.

| Item | Como é |
|---|---|
| credencial | administrativa do ambiente, **sem nome fixo** ([`index.md`](index.md) §3) |
| DSN | lida de variável de ambiente (`AGENTS.md` §5, item 7); nunca em linha de comando ou script versionado |
| dono do banco | `wyd_owner` é `NOLOGIN`: a rotina não roda como dono |
| execução | `DELETE` em lote com `LIMIT`, repetido até zero linhas |

## 2. Calendário

| Cadência | Tarefa | Forma do comando | Prova de sucesso |
|---|---|---|---|
| diário | expurgar `portal.market_notification` **lida**, com `read_at` além de 90 dias | `DELETE … WHERE read_at < now() - interval '90 days' LIMIT n` | contagem antes e depois |
| diário | expurgar `portal.person_token` além de 30 dias de `expires_at`, usado ou não | idem, por `expires_at` | contagem |
| diário | expurgar `ops.login_attempt` além de 30 dias | idem, pela coluna de data do DDL | contagem |
| diário | expurgar `game.pin_reset` além de 30 dias de `expires_at` | idem; sem índice por `expires_at`, varre a tabela | contagem e tempo |
| semanal | `vacuum (analyze)` nas tabelas que têm expurgo | `psql` com a credencial administrativa | `pg_stat_user_tables` |
| semanal | conferir contiguidade e checksum do ledger da série | `migrate status` (code `crates/`) | saída do comando |
| mensal | revisar crescimento das maiores tabelas | contagem e tamanho por tabela | **sem limiar** até `M20` |
| mensal | conferir espaço livre e o log do host | comando do host | **sem limiar** e **sem rotação definida** (§6) |

**Fora do calendário, porque falta aplicação:** `portal.person_session` (30 dias após `expires_at`,
`M22`) e `game.session` (365 dias após `released_at`, `M22`) têm prazo decidido, mas o índice e o
`DELETE` de expurgo ainda não existem — as cinco aplicações de retenção pendentes estão listadas em
`pendencias-de-fechamento.md` §4.3. Rodar o expurgo sem índice seria varredura de tabela grande: não é
rotina, é obra.

## 3. Vacuum e estatística

- **Não há parâmetro de autovacuum fixado nesta árvore:** nenhuma página do `wyd-db` cita `autovacuum`, e
  [`configuracao.md`](configuracao.md) só lista as variáveis que o processo lê. A página não inventa valor.
- `vacuum (analyze)` manual usa a credencial administrativa; `wyd_owner` é `NOLOGIN`.
- `analyze` depois de carga grande (seed de conteúdo ou migration que escreve muitas linhas).
- `ops.delivery_queue` não é expurgada por decisão: `pending` é entrega não perdida. O que cresce é o
  histórico entregue — contar, não apagar.

## 4. Revisão de crescimento

- O número medido pertence a `planos/metricas.md`, que está bloqueada até a série ser aplicada e medida
  (`M20`). Esta página não antecipa volume, taxa nem bytes.
- Até lá, a revisão mensal **conta** por tabela e registra (§7); não compara com limiar inexistente.
- Crescimento sem expurgo por decisão: `ops.audit_log` (730 dias, `M22`), o bloco financeiro do mercado
  (`Q-RET-6` = A: sem expurgo por tempo) e o histórico de `game.session`.

## 5. Se a rotina falhar

| Sintoma | O que significa | O que fazer |
|---|---|---|
| `42501` (permissão negada) | a credencial não é a administrativa, ou o papel não tem o privilégio | parar; não improvisar `GRANT` |
| lock longo no `DELETE` | o lote é grande ou há transação aberta | reduzir o lote; não matar sessão sem identificar o dono ([`diagnostico.md`](diagnostico.md)) |
| o expurgo não chega a zero | pode haver linha presa por FK ou condição errada | registrar o número e abrir pendência; não aumentar o lote “para resolver” |

## 6. Rotação de log — não há desenho

Nenhuma página do `wyd-db` fixa retenção, rotação ou destino do log do PostgreSQL ou do host: busca por
`logging_collector`, “rotação” e retenção de log não retorna desenho nesta árvore. O log é do
host/Compose e o dono é `wyd-ops`; esta página não inventa caminho de arquivo nem prazo. A tarefa mensal
é **conferir espaço livre**, não “rotacionar” — e nem o espaço tem limiar aprovado.

## 7. Registro do que foi feito

| Campo | Conteúdo |
|---|---|
| data e hora | quando a execução rodou |
| tarefa | qual linha de §2 |
| executor | função (o nome, quando houver) |
| comando | a forma exata usada, com o lote |
| contagem | antes e depois, por tabela |
| resultado | ok, falha ou parcial, com a mensagem |

Regra: execução sem registro **não conta como feita**. O registro vive na issue de operação do GitHub,
uma linha por execução — o projeto já organiza issue, milestone e dependência por lá
(`../../../wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3).

## 8. O que esta página não decide

- **A credencial nominal:** é de [`configuracao.md`](configuracao.md) e de [`index.md`](index.md) §3.
- **Limiar de disco, pool ou latência:** não medido (`M20`); o pool tem pergunta própria (`Q-BLK-06`).
- **Rotação de log:** dono é `wyd-ops`.
- **Prazo de retenção novo:** retenção é de [`retencao-e-capacidade.md`](../arquitetura/retencao-e-capacidade.md).

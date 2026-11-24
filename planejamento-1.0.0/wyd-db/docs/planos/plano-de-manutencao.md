> Pergunta: "o que roda todo dia, toda semana e todo mês no banco, com qual credencial, e como registro?"
> Público: Operação e DBA do `wyd-db`
> Dono: equipe do `wyd-db` com a operação
> Base: `docs/arquitetura/retencao-e-capacidade.md`; `proposta-retencao-pendente.md` §9; `pendencias-de-fechamento.md` §6.6 (`Q-BLK-02`, aprovada em 2026-11-24); `guia-administrador/backup-e-restore.md`; `guia-administrador/index.md` §3
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

A rotina existe para uma coisa só: **executar o que já foi decidido** e provar que rodou. Ela não decide
retenção, não escolhe prazo e não cria job novo — os quatro expurgos e a cópia diária já têm decisão do
dono; o que faltava era quem os executa, e isso foi respondido em 2026-11-24 (**`Q-BLK-02` = `cron` do
host no Compose**).

## 1. O que a rotina executa

| Rotina | Frequência | Ação | Fonte da decisão |
|---|---|---|---|
| expurgo de `portal.person_token` | diária | `delete` de `expires_at < now() - interval '30 days'`, usado ou não | `Q-RET-1 = A` |
| expurgo de `ops.login_attempt` | diária | `delete` de `occurred_at < now() - interval '30 days'` | `Q-RET-2 = A` |
| expurgo de `game.pin_reset` | diária | `delete` de `expires_at < now() - interval '30 days'` | `Q-RET-4 = A` |
| expurgo de `game.market_notification` | diária | `delete` de `read_at is not null and read_at < now() - interval '90 days'`; não lida **nunca** sai | `Q-RET-6 = A` |
| cópia de segurança | diária | `pg_dump` + `pg_dumpall --globals-only`, 30 dias de rotação | `Q-RET-7 = A`; `Q-BLK-03` |
| `vacuum (analyze)` das tabelas que sofrem expurgo | semanal | reduz dead tuple depois do `delete` em lote | `docs/arquitetura/retencao-e-capacidade.md` |
| revisão de crescimento | semanal | tamanho por tabela contra o previsto | `retencao-e-capacidade.md` §Capacidade |
| ensaio de restore | trimestral | restaurar a última cópia em ambiente separado | `Q-RET-7 = A`; `guia-administrador/backup-e-restore.md` §4 |
| revisão de retenção e volume | mensal | conferir se o volume medido muda alguma linha da política | `plano-de-manutencao.md` §4 |

**Fora da rotina, de propósito:** `portal.person_consent` não tem expurgo por tempo (`Q-RET-3 = A`,
retenção = ciclo de vida da pessoa); financeiro e custódia do mercado não têm expurgo por tempo
(`Q-RET-6 = A`); `ops.audit_log` é append-only com 730 dias (`M22`) e o expurgo dela não faz parte deste
plano — quando existir, entra por linha nova.

## 2. Quem executa e com qual credencial

**Decidido em 2026-11-24 (`Q-BLK-02`): `cron` do host, no Compose.** Uma entrada de `cron` do host roda o
`psql` no container do banco, com o DSN lido de variável de ambiente do `.env` — não há `pg_cron`, extensão
nova nem agendador dentro do banco.

| Decisão | Por quê |
|---|---|
| `cron` do host, não `pg_cron` | `Q-OPS-2 = A` já põe a operação no host com Compose; `pg_cron` exigiria extensão, permissão elevada e imagem própria |
| DSN por variável de ambiente | `AGENTS.md` §5, item 7 e §9: segredo só em `.env`/secret, nunca em linha de comando, script versionado ou log |
| execução em lote, repetida | `delete` em lote com `limit` repetido até zero linhas evita segurar lock longo numa tabela grande; o tamanho do lote é `P` até a primeira medição (`M20`) |

> [!WARNING] A credencial nominal ainda não está fixada
> `guia-administrador/index.md` §3 registra que backup, restore, vacuum e diagnóstico usam a **credencial
> administrativa do ambiente** e que ela não está definida no contrato da árvore. O nome da variável de DSN
> é de `configuracao.md`. Enquanto a operação não fixar isso, a rotina roda como ato do operador, não como
> contrato do `wyd-db` — e esta página não inventa o nome.

## 3. Comandos

Um arquivo por rotina, com o comando abaixo e o código de saída registrado. Os quatro expurgos são
`delete` com `returning` para contar o que saiu:

```bash
psql "$DSN_ADMIN" -v ON_ERROR_STOP=1 -c \
  "with apagado as (
     delete from portal.person_token
      where expires_at < now() - interval '30 days'
      returning 1)
   select count(*) from apagado;"
```

Repetir a mesma forma para `ops.login_attempt` (`occurred_at`), `game.pin_reset` (`expires_at`) e
`game.market_notification` (`read_at is not null`). `DSN_ADMIN` é rótulo do DSN administrativo do
ambiente; o nome da variável é de `configuracao.md`.

| Detalhe | Regra |
|---|---|
| `ON_ERROR_STOP` | obrigatório: expurgo pela metade não pode passar por sucesso |
| transação | um `delete` por execução; sem transação longa entre rotinas diferentes |
| índice | `ops.login_attempt` usa `ix_login_attempt_occurred`; `game.pin_reset` **não tem** índice por `expires_at` (`Q-RET-4`), então o expurgo varre a tabela — custo medido só em `M20`; `game.market_notification` espera o índice por `read_at` (`Q-RET-6/aplicação`, pendente) |
| falha | código de saída diferente de zero acende o sinal de §4 e **não** tenta de novo na mesma janela |

## 4. Sinais de degradação

| Sinal | Como ver | Ação |
|---|---|---|
| expurgo não rodou | linha ausente no log do job na janela do dia | rodar à mão com a mesma consulta e registrar a causa |
| expurgo falhou | código de saída ≠ 0 no log do `cron` | ler o erro do `psql`, corrigir e repetir; se for lock, checar `pg_stat_activity` |
| tabela cresceu acima do previsto | `pg_total_relation_size` por tabela, semanal | comparar com `retencao-e-capacidade.md` e abrir decisão se o crescimento for estrutural |
| muitos dead tuples | `n_dead_tup` em `pg_stat_user_tables` | `vacuum (analyze)` na tabela; se voltar, o lote do expurgo está grande |
| lock esperando | `wait_event_type = 'Lock'` em `pg_stat_activity` | identificar a sessão e encerrar a transação que segura |
| disco e pool | **sem limiar** — `M20` não mediu e `Q-BLK-06` ainda não foi respondida | declarar como lacuna, não inventar número |

## 5. Registro

- **Por execução:** data, rotina, linhas afetadas e código de saída, no log do job no host. Sem linha no
  log, a execução não aconteceu.
- **Por mês:** uma linha de resumo em [`registro-de-execucao.md`](registro-de-execucao.md) — o que rodou,
  o que falhou e o que mudou de volume. Essa página é dona do histórico de entrega; a rotina só escreve a
  linha do mês.
- **Nunca:** contagem de página, número de tabela ou métrica de planejamento. Esses são de
  `planos/metricas.md` (bloqueada por `M20`).

## 6. O que este plano não faz

- Não decide prazo de retenção: isso é de `docs/arquitetura/retencao-e-capacidade.md` e das `Q-RET`.
- Não define RPO/RTO nem destino da cópia: é de
  [`backup-e-restore.md`](../guia-administrador/backup-e-restore.md).
- Não cobre a recuperação de desastre: é de
  [`recuperacao-de-desastre.md`](../guia-administrador/recuperacao-de-desastre.md).
- Não fixa a credencial nominal nem o inventário de papéis: `guia-administrador/index.md` §3 e
  `Q-BLK-04` (pendente).
- Não cria job para `person_consent`: não há expurgo por tempo (`Q-RET-3 = A`).

> Pergunta: "como abro conexão e transação, e o que o pool já garante sem eu configurar nada?"
> Público: Desenvolvedor do crate e quem opera o processo
> Dono: equipe do `wyd-db`
> Base: `docs/guia-administrador/configuracao.md` §3; `src/migrate/adapter/pg.md`; `src/provision/adapter/pg.md`; `docs/guia-desenvolvedor/portas-e-contratos.md` §1–§2; `pendencias-de-fechamento.md` §6.6.6 (`Q-BLK-06` = A)
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

**Decisão:** `Q-BLK-06` = **A** (2026-11-24). A regra que vale é: **declarar o que já acontece**. Nenhum
número desta página nasceu de estimativa — os do pool são o default do driver, o teto é o default do
servidor, e o que não tem fonte aparece como pendência, não como promessa. Quem mudar um valor muda em
[`configuracao.md`](../guia-administrador/configuracao.md) §3, que é o dono dos parâmetros.

## 1. Configuração e defaults

| Parâmetro | Valor | Dono do valor | Tipo |
|---|---:|---|---|
| conexões por pool de runtime | **10** (mínimo 0) | default de `PoolOptions::new()` do `sqlx` | `V` |
| espera por conexão (`acquire_timeout`) | **30 s** | default do `sqlx` | `V` |
| conexão ociosa devolvida (`idle_timeout`) | **10 min** | default do `sqlx` | `V` |
| vida máxima da conexão (`max_lifetime`) | **30 min** | default do `sqlx` | `V` |
| `lock_timeout` do runner | **10 s** (faixa `1 s`–`10 min`) | `MigrateConfig`; fora da faixa o construtor recusa | `V` |
| `statement_timeout` | **0** (sem limite) | default do PostgreSQL; **não declarado** no projeto | `V` (ausência) |

**Nenhum destes é variável de ambiente.** Pool e timeouts de runtime são do driver; `lock_timeout` é campo
do `MigrateConfig`. Isso é o que `configuracao.md` §3 passa a declarar e o que `Q-BLK-06` fechou: antes, a
página de configuração dizia que “pool, timeout de query e tamanho de conexão” não tinham desenho.

Um pool por processo, criado no boot, com os quatro valores acima. Não há pool compartilhado entre papéis:
cada processo lê **só a DSN do seu papel**, e o pool nasce com ela.

## 2. Orçamento de conexões

O teto não é meta: é o default do PostgreSQL 17 (`max_connections` **100** menos
`superuser_reserved_connections` **3**), o que deixa **97** conexões para papéis comuns.

| Consumidor | Conexões | Origem |
|---|---:|---|
| servidor de jogo | 10 | pool de runtime (§1) |
| portal/BFF | 10 | pool de runtime (§1) |
| `wyd-admin` | 10 | pool de runtime (§1) |
| leitura operacional | 10 | pool de runtime (§1) |
| runner de migrations — série | 1 | `src/migrate/adapter/pg.md` (“a série é serial”) |
| runner de migrations — lock | 1 | mesmo arquivo |
| `provision` | 1 | `src/provision/adapter/pg.md` |
| suíte de sistema | 10 | pool de teste |
| **total** | **53** | soma, não estimativa |

**53 de 97 — 44 livres.** A conta fecha com folga de 44 conexões, e é aritmética: quem configurar
`max_connections` abaixo de 53 reprova a soma. Hoje ninguém configurou o servidor.

## 3. Timeout e saturação

**A espera é finita e é de 30 s.** Esgotado o pool, quem pede conexão espera até `acquire_timeout` e
recebe erro de aquisição — não fica pendurado. Esse é o número que o DoD de
[`WYDDB-021`](../roadmap/WYDDB-021.md) passa a ler no shutdown: o processo tem 30 s para devolver o que
tem, em vez de “o timeout já configurado, sem número”.

**O que não tem limite é a consulta.** `statement_timeout` em **0** é o default do servidor e o projeto
não o fixou: uma consulta patológica roda até o fim. A decisão A **declara** esse 0 como estado, não como
alvo — fixar um valor seria inventar número. Enquanto isso, a proteção contra consulta longa é a espera
do lock no runner (`10 s`), que não cobre runtime.

**Saturação não é erro de código:** com 4 pools de 10 e 44 conexões livres, o caso que estoura é
concorrência acima do orçamento (§2), e o sintoma é erro de aquisição depois de 30 s — não queda do
servidor. Medir concorrência real exige `M20` executado; esta página não promete número de concorrência.

## 4. Quando usar transação

O que existe desenhado é o **runner** e o **provisionamento**; runtime de jogo, portal e staff têm o
contrato da porta, não um desenho de transação nesta árvore.

| Situação | Regra | Fonte |
|---|---|---|
| aplicar um arquivo de migration | **uma transação por arquivo**, e a linha do ledger commita junto com o DDL | `portas-e-contratos.md` §1 (`MigrationExecutor`/`MigrationTxn`) |
| desistir de um arquivo | o `Drop` da transação **desfaz**; não existe `rollback` exposto | `portas-e-contratos.md` §1 |
| aplicar o SQL de papéis | **sem transação explícita**: o texto de `render()` vai numa sessão | `portas-e-contratos.md` §2 (`Script`) |
| ler sessão/personagem ou commitar estado | é da porta C4 do core, não desta página | `portas-e-contratos.md` §3 |

**Não segure transação aberta esperando outra coisa.** O contrato já impede o caso clássico: a transação
é um tipo que morre no fim do escopo, e o lock da série é de **sessão**, não de transação, porque a
sessão atravessa uma transação por arquivo.

## 5. Isolamento

**Nenhum arquivo do projeto fixa nível de isolamento.** Vale o default do servidor, **`READ COMMITTED`**
(`V`, default do PostgreSQL); subir para `REPEATABLE READ` ou `SERIALIZABLE` é decisão **por transação**,
com custo de conflito que ninguém mediu — e por isso não está fixado aqui.

O que serializa o que precisa ser serial é o **advisory lock de sessão** do runner
(`pg_advisory_lock`, `portas-e-contratos.md` §1): duas migrations não rodam ao mesmo tempo, e o segundo
processo espera dentro do `lock_timeout` de 10 s. Isolamento de linha, quando existir em runtime, é regra
de quem escreve a consulta — não desta página.

## 6. Como não vazar conexão

| Regra | Por quê | Fonte |
|---|---|---|
| o pool é do **adapter**, não do `app` nem do `domain` | o `app` chama porta, não `sqlx` | `portas-e-contratos.md` §1–§2 e §3 |
| um único adapter pode servir **duas portas com o mesmo pool** | `Catalog` e `Script` compartilham o pool do provisionamento | `portas-e-contratos.md` §2 |
| a conexão sai quando o tipo de transação morre | `Drop` desfaz o que não commitou | `portas-e-contratos.md` §1 |
| nada de pool global entre processos | um pool por processo, com a DSN do papel | §1 acima |

## 7. Teste que cobre

**Nenhum teste desta seção existe hoje** — o crate tem `Cargo.toml`, `src/dsn.rs` e `tests/sistema_pg.rs` (13 cenários de schema, nenhum de pool), então tudo nesta seção é `P`. O alvo já está
nomeado pelo guia de teste: concorrência, lock e isolamento são provados pelo **teste de contrato** com
`sistema_pg` e o teste de sessão ([`testes-de-contrato.md`](../guia-teste/testes-de-contrato.md)).

| O que o teste precisa provar | Como | Estado |
|---|---|---|
| dois runners não aplicam ao mesmo tempo | `AdvisoryLock` com dois processos e `lock_timeout` de 10 s | `P` |
| arquivo que falha não deixa linha no ledger | `MigrationTxn` com erro no meio e `Drop` | `P` |
| pool esgotado falha em 30 s, não pendura | `acquire_timeout` com pool de 1 e duas sessões | `P` |

O teste **não** prova latência nem concorrência de produção: esse número é de `M20`, e o lugar dele é
`planos/metricas.md`, não esta página.

## 8. O que esta página não decide

- **Valor por ambiente e rotação de credencial.** São de [`configuracao.md`](../guia-administrador/configuracao.md) §1 e §4, que é o dono dos parâmetros.
- **`statement_timeout`.** Continua **0 declarado**; fixá-lo exige decisão nova, com número que ninguém mediu.
- **Threshold de pool.** Fora de escopo por `WYDDB-026`, que proíbe definir threshold de pool, disco ou latência.
- **Driver.** Escolha de driver é `WYDDB-005`; esta página descreve o comportamento do `sqlx`, não o escolhe.
- **Concorrência e latência medidas.** Dependem de `M20` executado; o dono do número é `planos/metricas.md`.

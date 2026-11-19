> Pergunta: "como a série 1.0.0 é descoberta, validada e aplicada, e o que impede duas versões do binário de escreverem juntas?"
> Público: Desenvolvedor que for implementar `crates/wyd-db/src/migrate/`
> Dono: equipe do `wyd-db`
> Base: `planejamento-1.0.0/wyd-db/src/migrate/spec.md` §3–§11, `plano-migrations.md` §2 e §7
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-18)

# ADR-DB-0002 — Runner de migrations próprio, com ledger por versão

Quatro níveis de afirmação convivem aqui:

- **`P`** — o que a série 1.0.0 projeta: os 29 arquivos, os quatro schemas, as 92 tabelas e o runner
  novo.
- **`H`** — leitura da intenção do desenho herdado, dita com o que a fecharia.
- **`V`** — o que foi medido no runner herdado, com caminho e data: `crates/wyd-db/src/migrate.rs`,
  2026-11-17.
- **sem marca** — comportamento do PostgreSQL: lock de sessão vale pela sessão inteira, e
  `CREATE TABLE IF NOT EXISTS` não corrige a forma de uma tabela existente.

## Contexto

A série 1.0.0 não é a herdada com outro conteúdo: são 29 arquivos escritos do zero que trocam 87
tabelas em `public` (o legado não cria schema nenhum) por quatro schemas (`content`, `game`,
`portal`, `ops`) e 92 tabelas. O runner herdado já tem checksum e advisory lock de sessão; o que ele
não tem é o **contrato** que a série nova exige — e é isso, não um defeito, que motiva a decisão
(`V`, `crates/wyd-db/src/migrate.rs`, 2026-11-17):

| Hoje | O que a série 1.0.0 precisa |
|---|---|
| ledger `schema_migrations(version TEXT, checksum, applied_at)`, criado com `CREATE TABLE IF NOT EXISTS` | coluna `slug`, `version` inteira e **forma validada**: `IF NOT EXISTS` aceitaria o ledger antigo calado, e a coluna nova não existiria |
| checksum do bloco `up` apenas | checksum do arquivo inteiro com EOL normalizado: hoje alterar o `down` de um arquivo aplicado passa despercebido |
| baseline: ledger vazio **+** `public.accounts` presente ⇒ grava os arquivos no ledger e não executa nenhum | ledger vazio significa aplicar os 29; schema pré-existente sem ledger é erro, não baseline |
| `reset` como parâmetro do runner | recriar schema é operação, não passo de `apply` |
| advisory lock de sessão, chave `0x5759_4401`, `unlock` explícito | chave própria da série nova (`0x5759_4404`): binário antigo e novo não devem compartilhar lock |
| comandos `run(dsn)` e `run_dir(dsn, dir, reset)` | `status` sem escrever e `down --to` explícito, além de `apply` |

Nada disso é "defeito" do herdado: ele foi desenhado para **uma** série em `public`, já aplicada num
volume existente — a baseline marca tudo como aplicado em vez de executar (`V`); que esse fosse o
desenho pretendido é leitura da intenção (`H`). A 1.0.0 troca o contrato de schema (quatro schemas,
92 tabelas) e por isso redefine o registro — o herdado fica como baseline medido, não como desenho a
imitar (preâmbulo de `src/migrate/spec.md`). As linhas de `migrate.rs` que provam cada linha da
tabela estão em `Rastreabilidade`: elas valem enquanto o arquivo existir, e o arquivo é substituído
pelo módulo `migrate/` na implementação.

A restrição de arquitetura é a mesma do resto do crate: `clippy -D warnings` como gate, `async fn` em
trait sem `Box<dyn>`, e nada de I/O fora do adapter.

## Decisão

A série 1.0.0 tem **runner próprio**, em `crates/wyd-db/src/migrate/`, com seis pontos congelados. Cada
um nomeia onde a regra mora; esta ADR não a repete.

1. **Ledger por versão, em `public`.** `public.schema_migrations (version, slug, checksum, applied_at)`
   com `version` como chave, e a forma conferida por `prepare` em vez de `IF NOT EXISTS` cego — sem
   isso o ledger antigo passaria calado, com coluna faltando ou tipo trocado (`ddl-confirmado.md` §3).
   Regra em `src/migrate/spec.md` §3.
2. **Checksum do arquivo inteiro.** SHA-256 sobre cabeçalho, `up` e `down`, com EOL normalizado — não
   só o `up`, que deixaria o `down` alterado passar despercebido após a aplicação. Regra em
   `src/migrate/spec.md` §4.
3. **Uma transação por arquivo.** Arquivo e linha de ledger commitam juntos, com `search_path` vazio:
   instrução sem schema falha em vez de cair num schema do caminho. Falha desfaz só aquele arquivo, e
   o ledger diz onde parou. Regra em `src/migrate/spec.md` §6.
4. **Advisory lock de sessão.** Chave constante `0x5759_4404`, conexão dedicada e espera limitada.
   Lock de sessão, não de transação: a sessão precisa atravessar uma transação por arquivo. Regra em
   `src/migrate/spec.md` §5.
5. **Decisão pura, SQL só pela porta.** `plan(set, applied) -> Plan` compara disco e ledger sem I/O, e
   o SQL do arquivo atravessa apenas `MigrationTxn` — a orquestração não emite SQL de controle de
   transação. É o que permite provar o runner contra portas falsas. Regra em `src/migrate/spec.md` §8 e §6.
6. **Três comandos, nada automático na biblioteca.** `status` (±`--check`), `apply` (±`--dry-run`) e
   `down --to`, com a taxonomia de saída fixa de §11 — código novo é decisão registrada, não
   `catch-all`. A biblioteca não decide migrar: quem chama é o boot ou o binário. Regra em
   `src/migrate/spec.md` §8 e §11.

**Precisões de 2026-11-18 (contrato de tipos, não novo ponto congelado).** São três decisões
registradas em `lacunas.md` §1; esta ADR só aponta
([`../../convencoes.md`](../../convencoes.md) §*Nota de enunciação*):

- erro de `MigrationTxn::record`/`forget`, e por que não é `LedgerError` —
  `src/migrate/port/unit_of_work.md` §2.
- variante de `MigrateError` para falha de `Source::load` e o código de saída dela —
  `src/migrate/error.md` §2–§3.
- construtor do adapter do provisionamento, e por que o nome difere do `PgAdapter` deste runner —
  `src/provision/adapter/pg.md` §2 (`ADR-DB-0001`, item 5, também aponta).

O preflight de papel (`current_user = wyd_migrator`, comparação estrita) já é decisão da `ADR-DB-0001`
(Decisão item 6) e não se repete aqui.

## Consequências

- **Positivas:** drift de conteúdo vira erro nomeado antes de aplicar, não incidente de produção; dois
  processos serializam em vez de corromper; todo arquivo é obrigado a ter `down` (spec §2), então o
  rollback é executável e não teórico; o ledger passa a descrever exatamente o que está no banco; o erro de aplicação carrega a
  `Version`, e o boot falha alto em vez de subir com schema divergente.
- **Negativas:** todo arquivo aplicado fica imutável — corrigir um comentário num `.sql` já aplicado é
  erro de checksum, e o caminho é arquivo novo; `0001` precisa de um `down` que derruba os quatro
  schemas, então o teste de ciclo exige banco descartável; o ledger vira contrato público (mudar a
  forma é mudança de desenho, não de implementação); **o ledger herdado é incompatível por desenho**
  (sem `slug`, `version` texto), então não há adoção do histórico antigo — a série nova começa em
  banco sem ledger, ou precisa de uma decisão explícita que hoje não existe; a chave do lock muda, e binário
  antigo e novo não serializam entre si durante a troca.
- **Invariantes afetados:** (a) arquivo aplicado é imutável; (b) a série não abre transação; (c) SQL só
  por `MigrationTxn`; (d) boot nunca desfaz — `down` é comando explícito; (e) `search_path` vazio dentro
  da transação; (f) `verify` não existe como comando: era `status` com outro código de saída;
  (g) a biblioteca não emite `DROP` de schema nem de banco — o reset é o `down` dos próprios arquivos.
- **O que fica pendente:** `crates/wyd-db/src/migrate.rs` (o arquivo) e `crates/wyd-db/src/migrate/` (o
  módulo) não coexistem; a entrega que cria o módulo apaga o arquivo antigo, e `SCHEMA_VERSION` vai de
  `53` para `29` no mesmo commit da troca (`plano-migrations.md` §6). O `--reset` deixou de ser decisão
  em aberto em **2026-11-18**: a biblioteca não ganha comando nem `DROP` próprio — o reset é `down` até
  vazio (`Command::Down { to: DownTo::Empty }`, regra em `src/migrate/spec.md` §8.1), com o mesmo lock,
  uma transação por arquivo e a linha do ledger apagada arquivo por arquivo; ao fim os quatro schemas
  de domínio não existem e `public.schema_migrations` fica **vazia e com a forma validada**, pronta para
  o `apply` seguinte. O `DROP SCHEMA IF EXISTS public CASCADE` do herdado
  (`crates/wyd-db/src/migrate.rs:236-240`) não serve: derruba só `public`, deixaria os quatro schemas
  vivos sem ledger e o `CASCADE` anularia o gate que `migrations/0001-bootstrap.md` §8 usa ao derrubar
  os schemas **sem** `CASCADE`. A flag `--reset` passa a ser composição no chamador (`down` até vazio +
  `apply`); `wyd-admin` é legado (`run_dir(dsn, dir, reset)`, `crates/wyd-admin/src/main.rs:1429`), será
  replanejado inteiro e não é razão para desenhar o módulo em torno da assinatura dele.

## Alternativas consideradas

| Alternativa | Por que foi descartada |
|---|---|
| **Evoluir o `migrate.rs` herdado** | Ele já tem checksum e lock, mas o desenho é de série única em `public`: ledger sem `slug`, `version` texto, baseline implícito, `reset` dentro do runner e nenhum comando de leitura. Mudar tudo isso dentro do arquivo que também carrega o formato herdado misturaria as duas séries na mesma função, e a prova contra portas falsas exige as portas separadas. |
| **`sqlx::migrate!` / `refinery`** | Trazem tabela de controle, resolução de versão e lock próprios, com semântica de `down` e de checksum diferente da exigida aqui (EOL normalizado, arquivo inteiro). O projeto passaria a depender de um contrato que não controla, e o formato de marcadores da série (`-- migrate:up/down`, `plano-migrations.md` §2) não é o deles. |
| **Uma transação para a série inteira** | Um erro no arquivo 20 desfaz os 19 anteriores, e o ledger perde o ponto de parada — justamente a informação que o boot precisa. |
| **Lock de transação (`pg_advisory_xact_lock`)** | Não atravessa a série: uma transação por arquivo significa lock liberado entre arquivos, e outro processo entra no meio. |
| **Ledger em `ops`/`content`** | Os schemas são criados por `0001`, e o ledger precisa existir **antes** do primeiro arquivo. Em `public` ele já é exceção declarada (`ddl-confirmado.md` §3); em schema de domínio, seria dependência circular. |
| **Chave do ledger em `text` (nome do arquivo)** | Renomear o slug viraria "versão nova", e a ordem deixaria de ser numérica. `version` inteira é ordenável, comparável e imune a renomeação — que virou `DriftKind::Slug`, não linha nova. |
| **`Box<dyn MigrationTxn>`** | `async fn` em trait não é dyn-compatible; o tipo associado `MigrationExecutor::Txn` mantém o contrato sem alocação e sem `dyn` no caminho. |
| **Checksum só do `up`** | O `down` alterado também invalida o rollback, e seria aplicado sem aviso. O checksum cobre o arquivo inteiro, e só a quebra final é ignorada. |

## Como verificar

Cada prova é um teste do módulo (`src/migrate/spec.md` §13), não uma conferência manual:

| Prova | Esperado |
|---|---|
| aplicar a série em PG17 efêmero | 29 linhas no ledger, uma por arquivo |
| reaplicar em seguida | zero linha nova; `apply` é no-op |
| alterar um byte de arquivo aplicado | `status --check` sai `4` com o `DriftKind::Checksum` |
| apagar um arquivo do ledger | `status --check` sai `4` com `MissingFile` |
| ledger pré-existente com coluna a menos | `prepare` falha com `LedgerError::Shape` e não altera nada |
| diretório da série ausente | falha nomeando o caminho, sem tocar o banco; variante e código em `src/migrate/error.md` §3 |
| dois processos concorrentes | um aplica, o outro espera e termina sem aplicar |
| `down --to 1` | sobra só a versão 1, e o ledger também |
| `CREATE TABLE account` sem schema no `up` | falha (o `search_path` está vazio) |
| lock tomado por outro processo | erro `LockBusy` com `3`, não espera infinita |

## Rastreabilidade

- Consome: `src/migrate/spec.md` §3–§11 (desenho completo), `plano-migrations.md` §2 (formato da
  série) e §7 (gates), `ddl-confirmado.md` §3 (exceção do ledger em `public`) e `src/provision/spec.md`
  §4 (o privilégio de `public`, que é de ambiente e não da série).
- Evidência histórica (`V`, 2026-11-17, `crates/wyd-db/src/migrate.rs`, 396 linhas): ledger `:29`,
  `:248`; checksum do `up` `:153`, `:196`; baseline `:260`; `reset` `:209`; lock `:31`, `:230`,
  `:324`; comandos `:204`, `:209`. Crate com 24 arquivos Rust, 53 migrations e 87 `CREATE TABLE`;
  `SCHEMA_VERSION = 53` em `crates/wyd-db/src/lib.rs:29`. **Essas referências expiram:** o arquivo dá
  lugar ao módulo `migrate/`, e a série herdada só deixa de rodar na troca (`plano-migrations.md`
  §6) — até lá é ela que o produto executa, e é por isso que ela aparece aqui. Depois da troca, leia
  como registro do que era, não como caminho vivo.
- Evidência que falta para o aceite (`P`: nenhum teste rodado — não há `.sql` nem código da 1.0.0):
  (a) ciclo up/down/up em PG17 efêmero com os 29 arquivos; (b) dois processos concorrentes; (c) drift
  de um byte; (d) ledger com forma divergente recusado.
- Páginas desta árvore a atualizar na implementação: `docs/arquitetura/migracoes.md`,
  `docs/guia-desenvolvedor/como-adicionar-migration.md` e `docs/arquitetura/implantacao.md` (playbook
  de operação: provisionar papel, aplicar a série, conferir o ledger).
- Issues: `WYDDB-nnn` entram quando a série for aberta no `roadmap-rust/`.

## Revisão

Revisar quando: houver mais de uma série no mesmo banco (exige coluna de série no ledger e muda a
chave), entrar réplica de leitura com migrations (o lock é por cluster, não por nó), ou algum processo
deixar de chamar `migrate::run` no boot — o que reabre a discussão de quem aplica a série.

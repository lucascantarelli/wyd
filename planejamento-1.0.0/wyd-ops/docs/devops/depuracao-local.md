# Depuração local — teste real sob `lldb`

> Pergunta: "como depurar um teste de qualquer crate do workspace sem depender da UI do VS Code, e o que já foi executado?"
> Público: desenvolvimento e agentes
> Dono: `wyd-ops` (ferramental local)
> Base: `.vscode/tasks.json`; `.vscode/launch.json`; `AGENTS.md` §9; `.github/workflows/code.yml`
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26); a medição abaixo é `V` e foi executada nesta data. O depurador foi exercitado por linha de comando em **quatro crates** (oito configs), com breakpoint de linha atingido na thread do teste e teste concluído verde. A UI do VS Code **não** foi usada: a configuração foi exercitada com o mesmo programa, os mesmos argumentos, o mesmo `cwd` e o mesmo ambiente herdado do launch. A §7 repete a prova com `CARGO_TARGET_DIR` novo e vazio e registra a descoberta dos 10 crates pelo servidor LSP.

## 1. Configuração que localiza testes em todos os crates

Não há `Cargo.toml` na raiz: cada crate tem manifesto e `target/` próprios. Por isso o caminho do crate faz parte do comando, e nenhuma entrada vale para um crate só.

| Arquivo | Entrada | Como localiza o crate |
|---|---|---|
| `.vscode/tasks.json` | `rust: testes do crate (escolher)` | `input:crate` (lista fechada com os 10 crates) → `cargo test --manifest-path crates/${input:crate}/Cargo.toml --locked --lib --bins` |
| `.vscode/tasks.json` | `rust: check do crate (escolher)` | mesmo `input:crate` → `cargo check --manifest-path crates/${input:crate}/Cargo.toml --all-targets` |
| `.vscode/launch.json` | `Debug: teste da lib (escolher crate)` | `cargo.cwd = ${workspaceFolder}/crates/${input:crate}`, `cargo.args = ["test","--lib","--no-run"]`, `args = ["--test-threads=1","--nocapture"]` |
| `.vscode/launch.json` | `Debug: teste wyd-db sistema_pg` | `cargo.cwd = …/crates/wyd-db`, `--test sistema_pg`, `env.WYD_DB_TEST_CONTAINER=wyd-db-1` |
| `.vscode/launch.json` | `Debug: teste wyd-db runner_pg` | `--test runner_pg`, `env` com `WYD_DB_TEST_URL`, `WYD_DB_TEST_CONTAINER`, `WYD_DB_TEST_USER` e `WYD_DB_TEST_DB` |
| `.vscode/launch.json` | `Debug: teste wyd-db reversibilidade_pg` | `--test reversibilidade_pg`, `env` só com `WYD_DB_TEST_CONTAINER` (usuário/banco default do harness) |
| `.vscode/launch.json` | `Debug: wyd-ops-sign (bin)` | `cargo build --bin wyd-ops-sign` no crate, `program = target/debug/wyd-ops-sign.exe`, `args = []` → imprime o uso e sai 0 |
| `.vscode/launch.json` | `Debug: teste wyd-db (lib)` | `--lib`, `cwd` no crate, `args = ["--test-threads=1","--nocapture"]` |
| `.vscode/launch.json` | `Debug: teste wyd-ops-signer (lib)` | `--lib`, `cwd` no crate, mesmos args |
| `.vscode/launch.json` | `Debug: teste wyd-launcher (assinatura)` | `--test assinatura`, `cwd` no crate, `args = ["--nocapture"]` |

## 2. Depurador exercitado

| Item | Valor |
|---|---|
| Depurador | `lldb 22.1.8` (build `codelldb`) |
| Caminho | `~/.vscode/extensions/vadimcn.vscode-lldb-1.12.3/lldb/bin/lldb.exe` |
| Plataforma | Windows, toolchain `x86_64-pc-windows-msvc`, Rust 1.100.0 |
| Símbolos | PDB do próprio `target/debug`, ao lado do binário de teste |
| Env | herdado do shell nos crates 2.1/2.2/2.6; nos alvos PostgreSQL, exatamente o `env` do launch (2.3–2.5) |

O binário de teste é o que o Cargo produz com `cargo test --lib --no-run`; o `--batch` executa a mesma sequência que a config faria pela UI.

### 2.1 `wyd-server` — breakpoint atingido

```bash
cd crates/wyd-server
"$LLDB" --batch \
  -o "breakpoint set --file health/mod.rs --line 54" \
  -o "run" -o "bt 3" -o "frame variable" \
  -o "breakpoint disable 1" -o "continue" \
  -- target/debug/build/wyd-server/e46a12a70851b60e/out/wyd_server-e46a12a70851b60e.exe \
     --test-threads=1 --nocapture
```

Saída (recortada, verbatim):

```text
Breakpoint 1: where = wyd_server-e46a12a70851b60e.exe`wyd_server::health::avaliar + 184 at mod.rs:55, address = 0x0000000140007fb8
* thread #5, name = 'health::testes::health_responde_200', stop reason = breakpoint 1.1
  * frame #0: ... wyd_server::health::avaliar(pronto=false, requisicao=(data_ptr = "GET /health HTTP/1.1\xc8", length = 20)) at mod.rs:55
    frame #1: ... wyd_server::health::testes::health_responde_200 at mod.rs:121
(bool) pronto = false
(ref$<str$>) caminho = (data_ptr = "/health HTTP/1.1\xc8", length = 7)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
Process 13480 exited with status = 0 (0x00000000)
```

### 2.2 `wyd-admin` — mesmo fluxo, segundo crate

```bash
cd crates/wyd-admin
"$LLDB" --batch \
  -o "breakpoint set --file http/mod.rs --line 53" \
  -o "run" -o "bt 3" -o "frame variable" \
  -o "breakpoint disable 1" -o "continue" \
  -- target/debug/build/wyd-admin/ddb243398f61689b/out/wyd_admin-ddb243398f61689b.exe \
     --test-threads=1 --nocapture
```

```text
Breakpoint 1: where = wyd_admin-ddb243398f61689b.exe`wyd_admin::http::avaliar + 184 at mod.rs:54, address = 0x00000001400031d8
* thread #5, name = 'http::testes::health_responde_200', stop reason = breakpoint 1.1
  * frame #0: ... wyd_admin::http::avaliar(pronto=false, requisicao=(data_ptr = "GET /health HTTP/1.1\xc8", length = 20)) at mod.rs:54
    frame #1: ... wyd_admin::http::testes::health_responde_200 at mod.rs:119
(bool) pronto = false
(ref$<str$>) caminho = (data_ptr = "/health HTTP/1.1\xc8", length = 7)
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s
```

### 2.3 `wyd-db` / `sistema_pg` — banco migrado

Preparação equivalente ao job `pg / schema` de `ci-db.yml` (PostgreSQL 17 por digest, `tmpfs`, só em loopback). A task da IDE automatiza estes passos em `wyd-ops/scripts/ci/db-schema-local.sh`; o roteiro abaixo é o mesmo, à mão, para depurar:

```bash
make db-up
# papéis criados por db-prepare.sh, o mesmo script do passo "Criar papéis" do CI
docker exec -i -e PGHOST=127.0.0.1 -e PGUSER=postgres -e PGPASSWORD=test -e PGDATABASE=wyd \
  wyd-db-1 bash -s nologin < wyd-ops/scripts/ci/db-prepare.sh
# série: o mesmo bloco `-- migrate:up` de apply-series.sh, por stream — o host não tem psql
docker exec -i -e PGPASSWORD=test wyd-db-1 \
  psql -h 127.0.0.1 -U postgres -d wyd -v ON_ERROR_STOP=1 -X -q < "$TEMP/wyd-serie.sql"
```

Resultado medido: **68 tabelas** em `portal`, `game` e `ops`. O `runner_pg` também exige o papel de migração com LOGIN, então o mesmo script foi rodado no modo `migrator`.

```bash
cd crates/wyd-db
WYD_DB_TEST_CONTAINER=wyd-db-1 "$LLDB" --batch \
  -o "breakpoint set --file sistema_pg.rs --line 85" \
  -o "run" -o "bt 3" -o "frame variable" \
  -o "breakpoint disable 1" -o "continue" \
  -- target/debug/build/wyd-db/bf4062e8e1e15793/out/sistema_pg-bf4062e8e1e15793.exe \
     --test-threads=1 --nocapture
```

```text
Breakpoint 1: where = sistema_pg-bf4062e8e1e15793.exe`sistema_pg::cap24_login_tamanho_e_fold + 60 at sistema_pg.rs:85, address = 0x000000014000948c
* thread #5, name = 'cap24_login_tamanho_e_fold', stop reason = breakpoint 1.1
  * frame #0: ... sistema_pg::cap24_login_tamanho_e_fold at sistema_pg.rs:85
    frame #1: ... sistema_pg::cap24_login_tamanho_e_fold::{closure#0}(…) at sistema_pg.rs:83
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 43.03s
Process 13104 exited with status = 0 (0x00000000)
```

Neste ponto o `frame variable` devolveu `p = <variable not available>` (a build do CodeLLDB avisa que não há plugin para Rust): a prova aqui é o breakpoint na thread do teste e o backtrace, não a inspeção de variável.

### 2.4 `wyd-db` / `runner_pg` — `sqlx` e advisory lock

O alvo lê a DSN antes do pool; a config de launch passou a exportar `WYD_DB_TEST_URL` (sem ela o teste entra em `panic!`). Comando com o `env` completo do launch:

```bash
cd crates/wyd-db
WYD_DB_TEST_URL="postgres://wyd_migrator:test@127.0.0.1:5432/wyd" \
WYD_DB_TEST_CONTAINER=wyd-db-1 WYD_DB_TEST_USER=wyd_migrator WYD_DB_TEST_DB=wyd \
"$LLDB" --batch \
  -o "breakpoint set --file runner_pg.rs --line 103" \
  -o "run" -o "bt 4" -o "frame variable" \
  -o "breakpoint disable 1" -o "continue" \
  -- target/debug/build/wyd-db/7a40e4f6c045489b/out/runner_pg-7a40e4f6c045489b.exe \
     --test-threads=1 --nocapture
```

```text
Breakpoint 1: where = runner_pg-7a40e4f6c045489b.exe`runner_pg::apply_grava_ledger_real_e_e_idempotente::{closure#0} + 702 at runner_pg.rs:103, address = 0x000000014000773e
* thread #5, name = 'apply_grava_ledger_real_e_e_idempotente', stop reason = breakpoint 1.1
  * frame #0: ... runner_pg::apply_grava_ledger_real_e_e_idempotente::{closure#0}(…) at runner_pg.rs:103
    frame #1: ... Future>::poll(…) at future.rs:133
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.28s
Process 9764 exited with status = 0 (0x00000000)
```

### 2.5 `wyd-db` / `reversibilidade_pg` — ciclo `up → down → up`

Preparação — matriz `pg / reversibility`, lida em `.github/workflows/ci-db.yml`: `roles: nologin`, `series: false`, `user: postgres` e `url: postgres://postgres:test@127.0.0.1:5432/wyd` (linhas 43–48). O passo **Criar papéis** (linhas 85–88) roda `bash wyd-ops/scripts/ci/db-prepare.sh nologin` como `postgres`; o passo **Aplicar a série** (linhas 90–95) só executa com `if: matrix.series` e por isso é **pulado**. Em `db-prepare.sh`, `criar()` emite `CREATE ROLE … NOLOGIN` (linha 25) para os seis papéis (laço das linhas 29–31) e o `ALTER ROLE wyd_migrator LOGIN PASSWORD 'test'` está apenas no ramo `MODO = migrator` (linhas 36–40): nesta matriz o `wyd_migrator` fica **NOLOGIN** e o banco é entregue **vazio**. Os 28 `up`, 28 `down` e 28 `up` são aplicados pelo próprio alvo, na mesma execução.

```bash
make db-up
docker exec -i -e PGHOST=127.0.0.1 -e PGUSER=postgres -e PGPASSWORD=test -e PGDATABASE=wyd \
  wyd-db-1 bash -s nologin < wyd-ops/scripts/ci/db-prepare.sh
# nenhum dos quatro schemas existe (0), como a matriz entrega o banco
docker exec -e PGPASSWORD=test wyd-db-1 psql -h 127.0.0.1 -U postgres -d wyd \
  -tAc "select count(*) from pg_namespace where nspname in ('content','game','portal','ops')"  # 0
# o papel de migração desta matriz não tem LOGIN (saída `f`, falso)
docker exec -e PGPASSWORD=test wyd-db-1 psql -h 127.0.0.1 -U postgres -d wyd \
  -tAc "select rolcanlogin from pg_roles where rolname='wyd_migrator'"
```

```bash
cd crates/wyd-db
WYD_DB_TEST_CONTAINER=wyd-db-1 "$LLDB" --batch \
  -o "breakpoint set --file reversibilidade_pg.rs --line 210" \
  -o "run" -o "bt 3" -o "frame variable" \
  -o "breakpoint disable 1" -o "continue" \
  -- target/debug/build/wyd-db/61cc1dc581bfd93e/out/reversibilidade_pg-61cc1dc581bfd93e.exe \
     --test-threads=1 --nocapture
```

```text
Breakpoint 1: where = reversibilidade_pg-61cc1dc581bfd93e.exe`reversibilidade_pg::ciclo_up_down_up_em_pg17 + 574 at reversibilidade_pg.rs:210, address = 0x000000014000ff7e
* thread #5, name = 'ciclo_up_down_up_em_pg17', stop reason = breakpoint 1.1
  * frame #0: ... reversibilidade_pg::ciclo_up_down_up_em_pg17 at reversibilidade_pg.rs:210
    frame #1: ... ciclo_up_down_up_em_pg17::{closure#0}(…) at reversibilidade_pg.rs:197
(alloc::vec::Vec<reversibilidade_pg::Arquivo, alloc::alloc::Global>) arquivos = {
  buf = {
    inner = {
      cap = (__0 = 28)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 39.46s
Process 1880 exited with status = 0 (0x00000000)
```

O `env` do alvo no workflow (linhas 97–102) exporta `WYD_DB_TEST_URL`, e o runner instala `psql` (linhas 76–83). Neste host `command -v psql` não existe, então o harness usa o modo `WYD_DB_TEST_CONTAINER` (`crates/wyd-db/tests/comum/mod.rs`, precedência URL → container) e o `psql` roda dentro do `postgres:17`. A config de launch deste alvo não existia; foi criada com apenas `WYD_DB_TEST_CONTAINER`, sem DSN nem papel de migração — o alvo usa o default `postgres`/`wyd` e nunca conecta como `wyd_migrator`.

### 2.6 Demais configs de launch — bibliotecas e binário

Todas usam o `cargo.cwd` do crate (confirmado no schema da extensão: `cargo.cwd` é propriedade suportada), o programa que o Cargo produz e os args do launch. Em cada uma o `lldb` parou em pelo menos um breakpoint e o processo saiu com status 0.

| Config | Breakpoint | Parada | Saída |
|---|---|---|---|
| `Debug: teste da lib (escolher crate)` com `wyd-server` | `health/mod.rs:54` → `mod.rs:55` | thread `health::testes::health_responde_200` | `5 passed`; `exited with status = 0` |
| `Debug: wyd-ops-sign (bin)` | `src/main.rs:17` → `main.rs:19` (`main`) | thread `main`, frame `wyd_ops_sign::main` | imprime o `uso:`; `exited with status = 0` |
| `Debug: teste wyd-db (lib)` | `src/dsn.rs:75` | thread `dsn::tests::parse_ok_and_redacted` | `63 passed`; `exited with status = 0` |
| `Debug: teste wyd-ops-signer (lib)` | `src/lib.rs:100` | thread `tests::assinatura_tem_128_hex_minusculos` | `4 passed`; `exited with status = 0` |
| `Debug: teste wyd-launcher (assinatura)` | `tests/assinatura.rs:29` | thread `aceita_assinatura_do_fixture` | `7 passed`; `exited with status = 0` |

```bash
cd crates/<crate>
"$LLDB" --batch -o "breakpoint set --file <arquivo> --line <linha>" \
  -o run -o "bt 3" -o "breakpoint disable 1" -o continue \
  -- <programa> <args do launch>
```

No binário, a linha 24 (`if args.is_empty()…`) não recebeu parada neste build: o mapa registra `main+215`, mas o processo seguiu até o fim (`error: Command requires a process which is currently stopped` no `bt`). A linha 17 resolve para `main.rs:19` e para em `wyd_ops_sign::main`; é o breakpoint que funciona, e a config não codifica linha.

## 3. O que esta evidência prova — e o que não prova

| Prova | Não prova |
|---|---|
| o `lldb` do CodeLLDB anexa ao binário de teste real e o breakpoint de linha é atingido | que a UI do VS Code foi usada; ela não foi |
| o backtrace nomeia o teste (`health::testes::health_responde_200`) e a thread de execução | depuração do cliente WYD: a captura segue bloqueada em `wyd-client/registro-do-cliente.md` §7.1 |
| `frame variable` mostra `pronto = false` e `caminho = "/health HTTP/1.1…"` no ponto do breakpoint | inspeção de variável Rust nos alvos PostgreSQL (a build do `lldb` não tem plugin de Rust e devolveu `<variable not available>`) |
| o mesmo programa/args/cwd do launch roda até o fim com o teste verde | que todos os alvos de todos os crates foram depurados; cinco alvos foram exercitados |
| `sistema_pg`, `runner_pg` e `reversibilidade_pg` param em breakpoint e terminam verdes (13, 5 e 3 testes), com o `env` do launch e o banco preparado como no `ci-db.yml`; no último, o frame mostra o `Vec` de 28 arquivos e o ciclo `up → down → up` completo | inspeção de variável Rust nesses alvos: o `lldb` avisa que não tem plugin de Rust e, em `sistema_pg`, devolveu `<variable not available>` |
| as 8 configs de `launch.json` param em breakpoint e saem com status 0 (5 de biblioteca/binário em 2.6, 3 PostgreSQL em 2.3–2.5) | que a UI do VS Code executou os launches; o caminho sem UI foi usado |
| as 18 entradas de `tasks.json` executam com exit 0, inclusive a task genérica corrigida e a task PostgreSQL | que a UI do VS Code executou as tasks; o comando equivalente foi rodado no shell |
| a descoberta do rust-analyzer (`experimental/discoverTest`, a mesma capability do Test Explorer) devolve raiz para os 10 crates e 127 funções; o workspace fica quiescente em 61,2 s | que a UI renderizou a árvore; 30 funções dos 6 alvos de integração do launcher ficam fora com os 10 `linkedProjects` (§5.2) |

## 4. Checklist

- [x] `tasks.json` usa `input:crate` com os 10 crates; nenhuma task de teste aponta para crate fixo
- [x] `launch.json` tem config genérica por `input:crate`, além dos atalhos de alvo específico
- [x] breakpoint real atingido em `wyd-server` (`health/mod.rs:55`), com backtrace e variáveis
- [x] breakpoint real atingido em `wyd-admin` (`http/mod.rs:54`), com backtrace e variáveis
- [x] teste concluído verde sob o depurador nos dois crates
- [x] `sistema_pg` sob `lldb`, contra PostgreSQL 17 migrado (68 tabelas), 13 testes verdes
- [x] `runner_pg` sob `lldb`, com `WYD_DB_TEST_URL` e `wyd_migrator` LOGIN, 5 testes verdes
- [x] launch do `runner_pg` corrigido: exporta `WYD_DB_TEST_URL`, sem a qual o alvo falha antes do pool
- [x] `reversibilidade_pg` sob `lldb`, banco vazio com papéis `nologin`, ciclo `up → down → up` com 28 arquivos, 3 testes verdes
- [x] config de launch do `reversibilidade_pg` criada com só `WYD_DB_TEST_CONTAINER` (o alvo não lê DSN)
- [x] rodada fecha com `make fmt`, `make lint`, `make test`, `make gate-fast` e `make gate` (registro da rodada, 2026-11-26)
- [x] `make build` criado (não existia) e exit 0; bins de `wyd-ops-sign`, `wyd-server` e `wyd-admin` presentes
- [x] 19 tasks executadas com exit 0, incluindo a task de layout e a PostgreSQL com banco preparado
- [x] task genérica de testes corrigida: `--lib --bins`, porque `--locked` sozinho falhava no `wyd-db` (exit 101)
- [x] 8 configs de launch pararam em breakpoint e saíram com status 0
- [x] descoberta: 10/10 crates no rust-analyzer; 127 funções visíveis e 30 do launcher com causa medida (§5.2)
- [x] revalidação §7.2: 106 funções lib/bin listadas e executadas em 10/10 crates com `CARGO_TARGET_DIR` limpo
- [x] revalidação §7.3: `experimental/discoverTest` devolve 10/10 crates e 127 funções após `quiescent=true`
- [x] revalidação §7.4: binário `wyd-ops-sign` e harness de `wyd-server` param em breakpoint e saem com status 0 sob `lldb --batch`

## 5. Tasks e descoberta de testes

`make build` não existia no `Makefile` — `make build` respondia `No rule to make target 'build'`, embora o README de build citasse o alvo. Foi adicionado com o mesmo laço de `$(CRATES)`: no Windows compila 10 crates (o launcher entra), fora dele 9. `make build` → exit 0, e existem `crates/wyd-ops-signer/target/debug/wyd-ops-sign.exe`, `crates/wyd-server/target/debug/wyd-server.exe` e `crates/wyd-admin/target/debug/wyd-admin.exe`.

### 5.1 Tasks

| # | Task | Comando equivalente | Exit |
|---|---|---|---|
| 1 | `gate: fast (sem Docker)` | `make gate-fast` | 0 — `gate: PASS (fast, 5.0s)` |
| 2 | `layout: verificar árvore` | `make layout` | 0 — `layout PASS`, 917 arquivos, 0 desvios |
| 3 | `gate: completo (Docker)` | `make gate` | 0 — `gate: PASS (full, 29.6s)`; `links`: 495 páginas, 493 fontes |
| 4 | `gate: testes do gate (Python)` | `make test-gate` | 0 — `Ran 23 tests … OK (skipped=1)` |
| 5 | `rust: check do crate (escolher)` | 10 opções × `cargo check --all-targets` | 0 em todas |
| 6 | `rust: testes do crate (escolher)` | 10 opções × `cargo test --locked --lib --bins` | 0 em todas |
| 7 | `rust: check dos 3 crates` | `dependsOn` 8–10, em sequência | 0 |
| 8 | `rust: check wyd-db` | `cargo check --manifest-path crates/wyd-db/Cargo.toml --all-targets` | 0 |
| 9 | `rust: check wyd-ops-signer` | idem, no crate | 0 |
| 10 | `rust: check wyd-launcher` | idem, no crate | 0 |
| 11 | `rust: fmt (check)` | `make fmt` | 0 |
| 12 | `rust: clippy` | `make lint` | 0 |
| 13 | `rust: testes (todos)` | `make test` | 0 |
| 14 | `rust: testes wyd-db (lib)` | `cargo test --manifest-path crates/wyd-db/Cargo.toml --lib` | 0 — 63 testes |
| 15 | `rust: testes wyd-db PostgreSQL (sistema_pg)` | `bash wyd-ops/scripts/ci/db-schema-local.sh` (recria o banco vazio, aplica papéis `nologin` e a série, e roda `sistema_pg`) | 0 — 13 passed, 40.91 s; repetido após `make db-up`: 41.99 s |
| 16 | `rust: testes wyd-launcher` | `cargo test --manifest-path crates/wyd-launcher/Cargo.toml --locked` | 0 |
| 17 | `rust: testes wyd-ops-signer` | idem, no crate | 0 |
| 18 | `db: subir` | `make db-up` | 0 — container `wyd-db-1` healthy |
| 19 | `db: derrubar` | `make db-down` | 0 — container e rede removidos |

A task 6 estava quebrada para o `wyd-db`: `cargo test --locked` inclui os três alvos PostgreSQL e, sem banco, termina em exit 101 com `thread 'ciclo_up_down_up_em_pg17' panicked at tests/comum/mod.rs:84`. O comando passou a `--lib --bins`; os alvos PostgreSQL ficam na task 15 e nas configs de launch, e as integrações do launcher na task 16.

A task 15 também não dependia mais de preparo manual: `wyd-ops/scripts/ci/db-schema-local.sh` recria o container (`docker compose --profile db rm -sf db` e `up -d --wait db` — o `tmpfs` garante banco vazio), aplica `db-prepare.sh nologin` e roda `apply-series.sh` dentro do container (o host não tem `psql`), depois executa `sistema_pg` com `WYD_DB_TEST_CONTAINER=wyd-db-1`. O script exporta `MSYS_NO_PATHCONV=1`, porque o Git Bash convertia `/tmp/...` antes de chamar o `docker cp`.

### 5.2 Descoberta de testes

A descoberta foi exercitada no **próprio servidor do rust-analyzer** que alimenta o Test Explorer, não por `cargo test -- --list`. O cliente LSP abriu o workspace com `capabilities.experimental.testExplorer = true` e os `initializationOptions` iguais aos de `.vscode/settings.json` (10 `linkedProjects`, `cargo.allTargets`), esperou `experimental/serverStatus` com `health = ok` e `quiescent = true` e chamou `experimental/discoverTest` sem `testId` e depois para cada raiz. Servidor: `rust-analyzer 0.3.3130-standalone (37721d1413 2026-11-22)`, o binário empacotado na extensão; o shim do toolchain não tem o componente (`Unknown binary 'rust-analyzer.exe'`).

Resultado: **10/10 crates** têm raiz (`wyd_db`, `wyd_launcher`, `wyd_ops_signer`, `wyd_protocol`, `wyd_net`, `wyd_core`, `wyd_sim`, `wyd_server`, `wyd_admin`, `wyd_bots`), com 18 raízes brutas e 14 depois de deduplicar por `id` (o TestController usa um `Map` por `id`; lib/bin compartilham o nome). O workspace ficou quiescente em 61,2 s e a árvore foi resolvida por raiz:

| Crate / raiz | Testes descobertos | `cargo test -- --list` | Observação |
|---|---:|---:|---|
| `wyd_db` (lib) | 63 | 63 | módulos e testes da lib |
| `reversibilidade_pg` | 3 | 3 | alvo de integração, raiz própria |
| `runner_pg` | 5 | 5 | idem |
| `sistema_pg` | 13 | 13 | idem |
| `wyd_launcher` (lib) | 27 | 27 | |
| `assinatura`, `ciclo_assinatura`, `manifesto`, `patch_1_0_0`, `rfc8785`, `schema` | **0** | 30 | **não entram no grafo com os 10 `linkedProjects`** (causa abaixo) |
| `wyd_ops_signer` (lib) | 4 | 4 | |
| `wyd_ops_sign` (bin) | 3 | 3 | |
| `wyd_server` (lib) | 5 | 5 | |
| `wyd_admin` (lib) | 4 | 4 | |
| `wyd_protocol`, `wyd_net`, `wyd_core`, `wyd_sim`, `wyd_bots` | 0 | 0 | esqueleto, sem teste ainda |
| **Total visível** | **127** | **157** | 30 funções do launcher ficam fora |

Causa medida: o `wyd-ops-signer` é `dev-dependency` do launcher e também um dos 10 projetos linkados. Com os dois manifests, o rust-analyzer 0.3.3130 duplica os pacotes `wyd_launcher`/`wyd_ops_signer` no grafo e **não cria os crates dos 6 alvos de integração do launcher**; `rust-analyzer/viewCrateGraph` não tem nó `assinatura`, `manifesto`, etc., e `experimental/discoverTest` devolve 18 raízes sem eles. Reproduções:
- só `wyd-launcher` linkado: os 6 alvos aparecem (30 funções);
- `signer + launcher` (qualquer ordem): os 6 somem;
- declarar os 6 `[[test]]` no `Cargo.toml` do launcher: não muda o grafo (revertido);
- remover o signer dos `linkedProjects`: os alvos do launcher voltam, mas o bin `wyd_ops_sign` e seus 3 testes somem — troca pior;
- `cargo.allTargets: false`: sem efeito.

Não há correção de configuração que mantenha os 10 projetos e os dois lados: o conserto exigiria um workspace Cargo único, que o projeto não tem por decisão (cada crate é independente). A configuração fica com os 10 `linkedProjects`; a suíte completa continua na task 15 (`cargo test` do launcher) e o limite está em §6.

### 5.3 Artefatos

`make build` gera `crates/<crate>/target/debug/<bin>.exe`. Os launches de teste não fixam caminho: o `cargo` do CodeLLDB roda em `crates/<crate>/` e o Cargo resolve o harness em `target/debug/build/<crate>/<hash>/out/<target>-<hash>.exe` (exemplos em §2). Os `program`, `args`, `cwd` e `env` exercitados usaram esses artefatos, e nenhuma config aponta para binário de outro crate.

## 6. Limites e próximo passo

O depurador é o binário do CodeLLDB, não um `lldb` no `PATH`; o caminho está no item 2. Em outra máquina sem a extensão, a config aponta para o mesmo adaptador e o comando equivalente exige o `lldb` do pacote. A UI continua sendo o caminho normal (`F5` na config genérica); esta página registra o caminho sem UI para agentes e CI-like. Nos alvos PostgreSQL, `make db-up` sobe o banco **vazio**: `sistema_pg` e `runner_pg` precisam de papéis e série (§2.3); `reversibilidade_pg` precisa de papéis `nologin` e banco vazio (§2.5), porque ele mesmo aplica e reverte a série.

O Test Explorer nativo do rust-analyzer mostra os 10 crates, mas com os 10 `linkedProjects` não materializa os 6 alvos de integração do launcher (30 funções); a causa medida e as reproduções estão em §5.2. Esses testes continuam cobertos pela task 16 (`cargo test` do launcher) e não foram escondidos por uma troca de configuração que perderia os 3 testes do bin do signer.

## 7. Revalidação do workspace com target limpo (2026-11-26T17:25-03:00)

Esta rodada reexecutou as configurações versionadas em `.vscode/` sem reutilizar cache de build. O objetivo foi provar três coisas por execução: os quatro JSONC são válidos e apontam para caminhos existentes; os testes dos 10 crates são enumerados e executados a partir de um `CARGO_TARGET_DIR` novo e vazio; e o binário e o harness de teste iniciam sob o `lldb` do CodeLLDB em modo batch. A UI do VS Code não foi usada; os mesmos programas, argumentos, `cwd` e ambiente das configs foram executados no shell.

### 7.1 Arquivos e ferramentas

| Item | Medição |
|---|---|
| `.vscode/settings.json` | JSONC válido; 10 `linkedProjects`; `rust-analyzer.testExplorer=true`; `check.command=clippy` com `allTargets` |
| `.vscode/tasks.json` | JSONC válido; 19 tasks; `input:crate` com os 10 crates |
| `.vscode/launch.json` | JSONC válido; 8 configs; os 7 `cwd` fixos existem e a genérica usa `crates/${input:crate}` |
| `.vscode/extensions.json` | JSONC válido; 9 recomendações (rust-analyzer, CodeLLDB e apoio) |
| Toolchain | `cargo 1.100.0 (3acc79c0c 2026-11-05)`; `rustc 1.100.0 (0ef1bc386 2026-11-09)` |
| rust-analyzer | `0.3.3130-standalone (37721d1413 2026-11-22)`, binário da extensão `rust-lang.rust-analyzer-0.3.3130-win32-x64` |
| `lldb` | CodeLLDB `1.12.3`, `~/.vscode/extensions/vadimcn.vscode-lldb-1.12.3/lldb/bin/lldb.exe` |

A validação dos JSONC foi feita com um parser que remove comentários `//` sem tocar em strings (Python 3.15.2). Nenhum arquivo novo nasceu fora de `.vscode/`; o harness LSP temporário ficou em `.tmp/` (ignorado pelo layout) e foi removido depois da medição.

### 7.2 Enumeração e execução dos testes — target limpo

```bash
CLEAN=$(mktemp -d)
for c in wyd-db wyd-ops-signer wyd-launcher wyd-protocol wyd-net \
         wyd-core wyd-sim wyd-server wyd-admin wyd-bots; do
  CARGO_TARGET_DIR="$CLEAN" cargo test --manifest-path "crates/$c/Cargo.toml" \
    --locked --lib --bins -- --list
  CARGO_TARGET_DIR="$CLEAN" cargo test --manifest-path "crates/$c/Cargo.toml" \
    --locked --lib --bins
done
```

| Crate | Funções listadas | Execução |
|---|---:|---|
| `wyd-db` | 63 | 63 passed; exit 0 |
| `wyd-ops-signer` | 7 | 4 (lib) + 3 (bin) passed; exit 0 |
| `wyd-launcher` | 27 | 27 passed; exit 0 |
| `wyd-protocol` | 0 | 0 passed; exit 0 |
| `wyd-net` | 0 | 0 passed; exit 0 |
| `wyd-core` | 0 | 0 passed; exit 0 |
| `wyd-sim` | 0 | 0 passed; exit 0 |
| `wyd-server` | 5 | 5 passed; exit 0 |
| `wyd-admin` | 4 | 4 passed; exit 0 |
| `wyd-bots` | 0 | 0 passed; exit 0 |
| **Total** | **106** | **106 passed em 10/10 crates** |

A suíte completa do launcher, que a task genérica não cobre, foi executada no mesmo target limpo: `cargo test --manifest-path crates/wyd-launcher/Cargo.toml --locked` → **57 passed** (27 da lib + 30 dos 6 alvos de integração), exit 0. Os alvos PostgreSQL do `wyd-db` não entram em `--lib --bins`; eles foram exercitados pela task 15 (§7.5).

### 7.3 Descoberta pelo rust-analyzer

A descoberta foi exercitada no servidor LSP real da extensão, com `capabilities.experimental.testExplorer=true` e `experimental.serverStatusNotification=true`, e `initializationOptions` iguais aos de `.vscode/settings.json` (10 `linkedProjects`, `cargo.allTargets`, `files.exclude`). O harness esperou `experimental/serverStatus` com `health=ok` e `quiescent=true` e chamou `experimental/discoverTest` sem `testId`; os nós foram resolvidos por `testId` e contados por `id` único.

```text
initialize OK, server= 0.3.3130-standalone (37721d1413 2026-11-22)
serverStatus {'health': 'ok', 'quiescent': True, 'message': None}
raw_roots= 18 dedup_roots= 14
  wyd_db             tests=63   modules=24  kind=package
  reversibilidade_pg tests=3    modules=0   kind=test
  runner_pg          tests=5    modules=0   kind=test
  sistema_pg         tests=13   modules=0   kind=test
  wyd_launcher       tests=27   modules=8   kind=package
  wyd_ops_signer     tests=4    modules=1   kind=package
  wyd_ops_sign       tests=3    modules=1   kind=package
  wyd_server         tests=5    modules=2   kind=package
  wyd_admin          tests=4    modules=2   kind=package
  wyd_protocol, wyd_net, wyd_core, wyd_sim, wyd_bots: 0
TOTAL_TESTS= 127
```

Os 10 crates têm raiz; o total visível é **127** funções. A diferença para as 157 da suíte completa é conhecida e medida em §5.2: os 6 alvos de integração do launcher (30 funções) não entram no grafo enquanto `wyd-ops-signer` e `wyd-launcher` estiverem os dois em `linkedProjects`. A configuração não foi trocada para esconder isso; a suíte completa continua na task 16.

### 7.4 Depurador em modo batch

#### 7.4.1 Binário `wyd-ops-sign` (config `Debug: wyd-ops-sign (bin)`)

```bash
rm -rf crates/wyd-ops-signer/target
cd crates/wyd-ops-signer && cargo build --bin wyd-ops-sign
"$LLDB" --batch -o "breakpoint set --file main.rs --line 17" -o run \
  -o "bt 3" -o "breakpoint disable 1" -o continue \
  -- target/debug/wyd-ops-sign.exe
```

```text
Compiling wyd-ops-signer v0.0.0 (...)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 41.21s
Breakpoint 1: where = wyd-ops-sign.exe`wyd_ops_sign::main + 16 at main.rs:19
Process 692 stopped
* thread #1, name = 'main', stop reason = breakpoint 1.1
  * frame #0: ... wyd_ops_sign::main at main.rs:19
uso: wyd-ops-sign sign --manifest <manifest.json> --out <manifest.json.sig> \
[--key-file <arquivo>]
Process 692 exited with status = 0 (0x00000000)
```

O `lldb` saiu com 0. A config não codifica linha; o breakpoint foi pedido em `main.rs:17` e o mapa resolveu para `main.rs:19`, como já registrado em §2.6.

#### 7.4.2 Harness de teste `wyd-server` (config `Debug: teste da lib (escolher crate)`)

```bash
cd crates/wyd-server && rm -rf target
cargo test --lib --no-run --message-format=json   # executable = .../wyd_server-<hash>.exe
"$LLDB" --batch -o "breakpoint set --file health/mod.rs --line 54" -o run \
  -o "bt 3" -o "frame variable" -o "breakpoint disable 1" -o continue \
  -- "$EXE" --test-threads=1 --nocapture
```

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 1.37s
Breakpoint 1: where = wyd_server-...exe`wyd_server::health::avaliar + 184 at mod.rs:55
* thread #5, name = 'health::testes::health_responde_200', stop reason = breakpoint 1.1
  * frame #0: ... wyd_server::health::avaliar(pronto=false, requisicao=(...)) at mod.rs:55
    frame #1: ... wyd_server::health::testes::health_responde_200 at mod.rs:121
(bool) pronto = false
(ref$<str$>) caminho = (data_ptr = "/health HTTP/1.1\xc8", length = 7)
...
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
Process 15576 exited with status = 0 (0x00000000)
```

O `lldb` saiu com 0. O programa, os argumentos (`--test-threads=1 --nocapture`) e o `cwd` são os da config genérica; o caminho do harness veio do `cargo test --lib --no-run`, exatamente como o CodeLLDB resolve na UI.

### 7.5 Tasks executadas nesta rodada

| Task | Comando | Resultado |
|---|---|---|
| `rust: fmt (check)` | `make fmt` | exit 0 |
| `rust: clippy` | `make lint` | exit 0; 0 linha `warning:`/`error:` |
| `rust: testes (todos)` | `make test` | exit 0; 136 passed, 0 failed (28 linhas `test result`) |
| `gate: testes do gate (Python)` | `make test-gate` | exit 0; `Ran 28 tests … OK (skipped=1)` |
| `rust: check do crate (escolher)` | 10 × `cargo check --all-targets` | 10/10 exit 0 |
| `rust: testes do crate (escolher)` | 10 × `cargo test --locked --lib --bins` | 10/10 exit 0 (106 testes; §7.2) |
| `db: subir` | `make db-up` | exit 0; `wyd-db-1` healthy |
| `rust: testes wyd-db PostgreSQL (sistema_pg)` | `bash wyd-ops/scripts/ci/db-schema-local.sh` | exit 0; 13 passed em 33,75 s |
| `db: derrubar` | `make db-down` | exit 0; container e rede removidos |

As tasks de gate (§7.6) fecham a rodada. As configs de launch PostgreSQL e o launcher não foram reexercitados nesta rodada; a medição deles continua em §2.3–§2.5 e §2.6.

### 7.6 Gates e inventário

| Comando | Resultado |
|---|---|
| `make layout` | PASS |
| `make gate-fast` | PASS |
| `make gate` | PASS |

Nenhuma linha do `planejamento-1.0.0/wyd-ops/inventario-depreciacao.tsv` mudou por esta rodada: a configuração de IDE não altera deprecated, dependência nem captura. `CAP-01` e `CAP-02` continuam bloqueadas pelos insumos de `wyd-client/registro-do-cliente.md` §7.1 — conta de teste em peer que responda ao CPSock ou aprovação escrita para sessão com credencial sintética, mais o nome do operador. Não há lacuna de IDE em `wyd-ops/lacunas.md` para fechar.

### 7.7 Limites desta revalidação

- A UI do VS Code não foi usada. O que foi provado é que os mesmos programas, argumentos, `cwd` e ambiente das configs executam; a renderização do Test Explorer e o painel de debug não foram inspecionados.
- O `lldb` do CodeLLDB continua sem plugin de Rust: a inspeção de variáveis é limitada e o aviso aparece na saída. A prova é o breakpoint na thread correta e o teste/binário concluído.
- A descoberta do rust-analyzer cobre 127 funções, não as 157 da suíte completa; a causa e as reproduções estão em §5.2.
- O target limpo foi descartado ao fim da medição; os caminhos em `.vscode/` continuam apontando para os `target/` padrão de cada crate, que o Cargo recria.

### 7.8 Ruído de testes herdados (2026-11-26)

A aba Testing mostrava testes de `deprecated/wyd-portal` por duas fontes instaladas, nenhuma delas o rust-analyzer:

- `vitest.explorer` varria o workspace e encontrava `deprecated/wyd-portal/src/**/*.test.ts` e `deprecated/wyd-portal/tests/**/*.spec.ts`; o `wyd-portal` real ainda não tem teste nem config de Vitest.
- `swellaby.vscode-rust-test-adapter` aceita um único `rustTestExplorer.rootCargoManifestFilePath` (não há workspace Cargo na raiz) e, sem ele, varria os manifestos de `deprecated/`.

Correção versionada: `vitest.configSearchPatternExclude` ganhou `**/deprecated/**`, `deprecated` entrou em `search.exclude`, e `swellaby.vscode-rust-test-adapter`, `hbenl.vscode-test-explorer` e `1yib.rust-bundle` entraram em `unwantedRecommendations`. O controller que enumera os 10 crates é o `rust-analyzer` com a aba Testing nativa. Extensão já instalada não é removida por configuração: o usuário precisa desabilitá-la no workspace (ou desinstalá-la) para o ruído sumir de vez.

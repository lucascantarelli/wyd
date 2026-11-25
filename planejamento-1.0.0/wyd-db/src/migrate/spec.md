> Pergunta: "como as migrations da 1.0.0 são descobertas, validadas e aplicadas?"
> Público: Desenvolvedor do `wyd-db` que vai implementar o runner
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §2 e §7, `padrao-banco.md` §11, `src/provision/spec.md`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-24) — aguardando aceite do dono

# Runner de migrations da 1.0.0 — desenho

Este documento define o módulo `crates/wyd-db/src/migrate/` **do zero**. Nenhuma decisão aqui vem do
runner herdado, de `crates/wyd-db/src/migrate.rs` ou de qualquer comportamento observado antes da
série 1.0.0: o que existir de coincidência é coincidência de problema, não herança de desenho. A
referência de onde cada regra nasce é o campo `Base:` acima.

## 1. Responsabilidade e anti-objetivos

**Responsabilidade:** dada uma série de arquivos e um banco, garantir que **cada versão foi aplicada
exatamente uma vez, com o conteúdo registrado**, e que qualquer divergência entre disco e banco para o
processo com erro nomeado — nunca "aplicar assim mesmo".

**Não é:**

- provisionamento de papel ou schema (é `provision`, `ADR-DB-0001`, fora da série);
- `DROP` de schema ou de banco por conta própria: a biblioteca só executa o SQL dos arquivos da série,
  e o reset de desenvolvimento é o `down` até vazio (§8.1), não um `DROP SCHEMA … CASCADE`;
- seed de conteúdo (mora dentro dos arquivos da série);
- ferramenta de modelagem: não gera DDL, não compara coluna, não conserta drift;
- autoridade sobre a ordem dos arquivos (a lista é de `plano-migrations.md` §3).

## 2. Formato do arquivo da série

| Regra | Detalhe |
|---|---|
| nome | `NNNN_slug.sql` — 4 dígitos, `slug` em `snake_case`, extensão única |
| numeração | contígua a partir de `0001`, sem buraco e sem duplicata; a série é `0001`…`0034` (**34** arquivos, `plano-migrations.md` §3) |
| marcadores | exatamente **um** `-- migrate:up` e **um** `-- migrate:down`, nesta ordem |
| antes do `up` | só comentário de cabeçalho; instrução antes do marcador é erro de formato |
| `up` | obrigatório e não vazio |
| `down` | obrigatório e não vazio: um arquivo sem `down` não entra na série |
| transação | o arquivo **não** abre transação: `BEGIN`/`COMMIT` são do runner (§6) |
| comandos não transacionais | proibidos (`CREATE INDEX CONCURRENTLY`, `VACUUM`, `ALTER TYPE … ADD VALUE`): falham dentro da transação e a mensagem precisa dizer por quê |

O parser é estrito e puro: recebe `&str` e devolve `Result<RawMigration, FileError>`. Ele **não** abre
arquivo, não sabe o caminho e não conhece o banco — o que o torna testável com uma tabela de casos.

O texto entre marcadores não é `String`: é `Sql`, um tipo com invariante no construtor. Ele aparece
em três assinaturas (`RawMigration`, `MigrationTxn::run`, checksum do arquivo), e cada uma delas
precisa da mesma garantia.

```rust
/// Trecho já validado. Invariantes do construtor:
///  - não vazio (depois de aparar);
///  - sem marcador `-- migrate:` (o parser já os consumiu);
///  - sem controle de transação (`BEGIN`, `COMMIT`, `ROLLBACK`, `SAVEPOINT`) — ver §6.
pub struct Sql(String);

impl FromStr for Sql {                  // só o parser constrói
    type Err = FileError;
    fn from_str(raw: &str) -> Result<Self, Self::Err>;
}

impl AsRef<str> for Sql { /* ... */ }   // `run(sql.as_ref())`, sem `as_str` próprio
impl fmt::Display   for Sql { /* ... */ }
```

`SET` e `SET LOCAL` continuam permitidos: são instruções do arquivo, não controle de transação — a
série só não pode fechar o que o runner abriu.

```rust
#[non_exhaustive]
pub struct RawMigration {
    pub version: Version,     // não-zero, 1..=u32::MAX
    pub slug: Slug,           // snake_case validado
    pub up: Sql,              // texto entre os marcadores
    pub down: Sql,
}
```

**Quem lê o disco é uma porta.** O parser recebe `&str`; carregar arquivo é I/O, então mora em
`port/`, com adapter de filesystem. Sem isso, `migration/` acabaria importando `std::fs` e a pureza do
item de testes de unidade deixaria de existir.

```rust
/// `Send + Sync` é supertrait: quem consome a porta não repete o bound a cada chamada.
pub trait Source: Send + Sync {
    async fn load(&self) -> Result<Vec<LoadedFile>, SourceError>;
}

#[non_exhaustive]
pub struct LoadedFile {
    pub path: PathBuf,   // só para mensagem de erro
    pub content: String, // texto cru; o parser não conhece caminho
}
```

O adapter (`adapter/fs.rs`) lista o diretório, filtra `*.sql`, lê cada arquivo e devolve na ordem
lexicográfica — a ordenação numérica é do `set.rs`, que valida contiguidade e é puro.

## 3. Ledger

Criado **pelo runner**, não pela série, antes do primeiro arquivo — é o primeiro passo de `apply`
depois do lock e do preflight. O privilégio de `wyd_migrator` em `public` é **de ambiente**: vem do
provisionamento, não de um arquivo da série (`src/provision/spec.md` §4; `ADR-DB-0001`, item 8).
O nome `schema_migrations` e a localização em `public` são exceção declarada em `ddl-confirmado.md` §3.

`prepare` verifica existência antes: se a tabela não existe, cria; se existe, compara a **forma** e
falha em divergência. `IF NOT EXISTS` esconderia um ledger errado por baixo — coluna faltando, tipo
trocado e checagem removida passariam calados, e o erro apareceria depois como drift de conteúdo.

A forma esperada é dado no código, não `if` espalhado:

```rust
/// (nome, tipo canônico do PG, NOT NULL)
const LEDGER_COLUMNS: [(&str, &str, bool); 4] = [
    ("version",    "integer",                  true),
    ("slug",       "text",                     true),
    ("checksum",   "text",                     true),
    ("applied_at", "timestamp with time zone", true),
];
```

`prepare` lê `pg_catalog` uma vez e compara a forma: as quatro colunas (nome, tipo, nulidade), a chave
primária `version` e as duas `CHECK` `ck_schema_migrations_*` pelo nome. Qualquer diferença →
`LedgerError::Shape`, com o detalhe legível. `prepare` é idempotente e roda dentro do lock.

```sql
CREATE TABLE public.schema_migrations (
    version    INTEGER     PRIMARY KEY,
    slug       TEXT        NOT NULL,
    checksum   TEXT        NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT ck_schema_migrations_version  CHECK (version > 0),
    CONSTRAINT ck_schema_migrations_checksum CHECK (checksum ~ '^[0-9a-f]{64}$')
);
```

- `version` é a chave: duas linhas para a mesma versão são impossíveis por construção.
- `slug` guarda o slug aplicado. Renomear arquivo já aplicado é **erro**, não atualização de linha.
- `applied_at` é o instante do banco (`now()`), não do processo: dois hosts não precisam de relógio
  sincronizado para a ordem ser verdadeira.
- O ledger **não** guarda `down` nem duração: o primeiro não é executado em produção, o segundo é
  observabilidade (span), não estado.

Três portas cuidam do ledger e da execução, e **nenhuma delas conhece `sqlx`**: leitura, execução
transacional e criação inicial. A transação pertence ao adapter, não à orquestração — é o que permite cumprir a atomicidade
de §6 sem que o `app` abra `BEGIN` em SQL cru.

```rust
/// Leitura. Só consulta: não cria tabela e não escreve. Tabela ausente → `Ok(vec![])`.
pub trait Ledger: Send + Sync {
    async fn applied(&self) -> Result<Vec<AppliedMigration>, LedgerError>;
}

/// Unidade de trabalho: o adapter abre a transação, fixa `search_path` e a fecha.
/// Desfazer não tem método: `commit` consome a transação (`self`, não `Box<Self>`) e o `Drop` do
/// adapter faz ROLLBACK — receber o `Txn` por valor de `begin` já torna o caminho único.
/// `record` recebe o checksum calculado pelo `MigrationSet`: `RawMigration` não o carrega.
pub trait MigrationTxn: Send {
    async fn run(&mut self, sql: &Sql) -> Result<(), SqlError>;
    async fn record(
        &mut self,
        m: &RawMigration,
        checksum: Checksum,
    ) -> Result<(), SqlError>;
    async fn forget(&mut self, version: Version) -> Result<(), SqlError>;
    async fn commit(self) -> Result<(), SqlError>;
}

/// Bootstrap do ledger + fábrica de transação.
/// `Txn` é tipo associado, não `Box<dyn>`: `async fn` em trait não é dyn-compatible, e
/// `Box<dyn MigrationTxn>` não compilaria com este contrato.
pub trait MigrationExecutor: Send + Sync {
    type Txn: MigrationTxn;
    /// Cria o ledger se ausente; se existir com forma diferente, falha (nunca `IF NOT EXISTS` cego).
    async fn prepare(&self) -> Result<(), LedgerError>;
    async fn begin(&self) -> Result<Self::Txn, SqlError>;
}

/// O que `plan` compara. `applied_at` fica na tabela como auditoria e não entra aqui.
#[non_exhaustive]
pub struct AppliedMigration {
    pub version: Version,
    pub slug: Slug,
    pub checksum: Checksum,
}
```

O ledger usa a coluna `slug`, igual ao campo: um dado, um nome. `AppliedMigration` é dado do domínio
(`migration/applied.rs`), não da porta: `plan` o consome, e `migration/` não importa `port/` (§10).

**Tabela ausente é ledger vazio, não erro.** `applied()` devolve `Ok(vec![])` quando
`public.schema_migrations` ainda não existe: `status` e `apply --dry-run` não chamam `prepare` (§8) e
precisam listar a série inteira como pendente — falhar com `Shape` no banco virgem inverteria o
diagnóstico mais comum, que é justamente "nada aplicado ainda". `Shape` fica para a tabela que existe e
não entrega `AppliedMigration`: coluna ausente (`42703`) ou valor que o tipo recusa. Quem cria a tabela
é o executor, sob lock, em `prepare`.

## 4. Checksum

`sha256` do conteúdo do arquivo **normalizado**, nesta ordem: todo `CRLF` vira `LF`; depois **um** `\n`
final, se existir, é removido — `strip_suffix('\n')` uma vez, nunca em laço, porque dois `\n` no fim são
conteúdo. Sem isso, um clone no Windows com `autocrlf` produz checksum diferente para o mesmo commit e
o runner acusa drift que não existe. O repositório fixa a regra no `.gitattributes`:

```gitattributes
*.sql    text eol=lf
```

O checksum cobre o arquivo inteiro (cabeçalho, `up` e `down`), não só o `up`: alterar o `down` de uma
migração aplicada também invalida o rollback e precisa ser detectado. A única coisa descartada é a
quebra final, que o editor acrescenta sozinho — dois arquivos que só diferem nela têm o mesmo
checksum, de propósito.

```rust
pub struct Checksum([u8; 32]);          // invariante: sempre 32 bytes

impl Checksum {
    pub fn of(bytes: &[u8]) -> Self;    // único construtor: normaliza EOL e hasheia
}

impl FromStr for Checksum { type Err = ChecksumError; }  // 64 hex minúsculos
impl Display   for Checksum { /* 64 caracteres */ }
impl AsRef<[u8]> for Checksum { /* os 32 bytes */ }
```

Os três tipos de texto seguem a mesma convenção: `Version`, `Slug` e `Checksum` implementam `FromStr`
(parse com erro tipado), `Display` (o mesmo texto que o banco guarda) e `Ord`/`Eq` onde há ordem.
Nenhum método `to_string`/`parse` inventado por cima do `std`.

## 5. Lock

Advisory lock de **sessão** (`pg_advisory_lock`, não `xact`): a sessão precisa atravessar várias
transações, uma por arquivo. Chave constante e documentada, estável para sempre — mudar a chave é
mudar o contrato de concorrência entre versões do binário:

```rust
pub const MIGRATION_LOCK_KEY: i64 = 0x5759_4404; // "WYD" + 04 (migrations)

/// Porta do lock: sem variante de modo, sem `unlock` exposto. O `PgAdapter` guarda a conexão dedicada;
/// soltar o lock é devolver essa conexão.
pub trait AdvisoryLock: Send + Sync {
    async fn acquire(&self, timeout: Duration) -> Result<(), LockError>;
}
```

- Conexão dedicada só ao lock; a aplicação usa outra conexão do pool.
- Espera limitada: `lock_timeout` configurável (padrão 10 s). Estourou → `LockBusy`, com instrução
  ("outro processo está migrando; verifique `pg_locks`"), nunca espera infinita no boot.
- Quem pega o lock é o `run`, antes de `prepare`; a porta não expõe variante `xact`, então não existe
  como pedir o modo errado — a garantia é do contrato, não de uma checagem em runtime.

## 6. Transação por arquivo

```text
BEGIN
  SET LOCAL search_path = ''
  <up do arquivo>
  INSERT INTO public.schema_migrations (version, slug, checksum)
COMMIT
```

- Arquivo e registro de ledger commitam **juntos**: não existe "aplicado sem linha" nem "linha sem
  schema".
- Falha em qualquer instrução → `ROLLBACK`, erro com a versão e a posição; nada dos arquivos
  seguintes é tentado.
- `SET LOCAL search_path = ''` é deliberado: a série escreve nome qualificado (`padrao-banco.md` §1,
  P-01 e P-02, e §3), e um `CREATE TABLE account` sem schema deve falhar, não cair por acidente num
  schema do `search_path`.
- Uma transação por arquivo, **nunca** uma para a série: um erro no arquivo 20 não desfaz os 19
  anteriores, e o ledger diz exatamente onde parou.
- Quem emite `BEGIN`, `SET LOCAL` e `COMMIT` é o `MigrationTxn` (`adapter/pg.rs`). O runner chama
  `begin` → `run(up)` → `record` → `commit`; se não chamar `commit`, o `Drop` do adapter desfaz. A
  orquestração não escreve SQL de controle de transação — só o conteúdo do arquivo.

## 7. Preflight de papel e de servidor

Antes de qualquer `BEGIN`:

| Verificação | Falha |
|---|---|
| `SELECT current_user` igual a `EXPECTED_ROLE`, comparação estrita | `WrongRole` com o nome do papel encontrado |
| servidor é PostgreSQL **`SERVER_MAJOR`** (17) | `WrongServer` com a versão lida |
| `Source::load` devolve ao menos um arquivo: diretório existe, é legível e tem `NNNN_*.sql` | `Source(SourceError)` para diretório ausente/ilegível; `SetInvalid` para diretório vazio |

Comparação estrita porque `pg_has_role` responde `t` para superusuário. A única saída é o opt-out
explícito `WYD_MIGRATE_ALLOW_SUPERUSER=1` (`ADR-DB-0001`, Decisão item 6), lido **pelo binário**, nunca por este
módulo — e sempre logado como aviso.

## 8. Comandos

Três comandos (com as variações `--check`, `--dry-run` e `--force`), e nenhum deles lê ambiente:
recebem `Command` mais as portas e devolvem `Result<Report, MigrateError>`.

| Comando | O que faz | Lock | Escreve |
|---|---|---|---|
| `status` | lista cada versão com `applied`/`pending`/`drift` e o resumo | não | não |
| `status --check` | igual a `status`, mas sai `4` em drift, arquivo faltando ou série inválida | não | não |
| `apply` | aplica as pendentes em ordem | sim | sim |
| `apply --dry-run` | imprime o plano (`Plan`) e sai | não | não |
| `down --to <v>` | desfaz de `MAX` até `v` (deixa `v` aplicada), uma transação por arquivo, apagando a linha do ledger | sim | sim |
| `down --to 0` | desfaz a série inteira e deixa o ledger vazio — é o reset de desenvolvimento (§8.1) | sim | sim |

`verify` não existe como comando: era `status` com outro código de saída. Uma pergunta, um cálculo.

`status` e `apply --dry-run` **não** chamam `prepare`: não criam o ledger e leem a série inteira como
pendente quando a tabela ainda não existe (§3). Criar tabela em comando que promete "não escreve"
quebraria a coluna `Escreve` da tabela acima.

**A decisão de o que fazer é uma função pura**, separada do I/O — é ela que os testes unitários
cobrem:

```rust
#[non_exhaustive]
pub enum Command {
    Status { check: bool },
    Apply { dry_run: bool },
    Down { to: DownTo },   // `--force` é trava do binário, não campo da biblioteca
}

/// Alvo do `down`. `Empty` é o reset (§8.1) e existe porque `Version` é não-zero:
/// `--to 0` na CLI mapeia para cá, e nenhum `Version(0)` é construível.
#[non_exhaustive]
pub enum DownTo {
    Empty,
    Version(Version),
}

#[non_exhaustive]
pub enum Plan {
    UpToDate,
    Apply(Vec<Version>),   // pendentes, em ordem crescente
    Drift(Vec<Drift>),     // todos os achados, não o primeiro
}

#[non_exhaustive]
pub struct Drift {
    pub version: Version,
    pub kind: DriftKind,
}

#[non_exhaustive]
pub enum DriftKind {
    Checksum,     // conteúdo do disco != ledger
    Slug,         // arquivo renomeado depois de aplicado
    MissingFile,  // ledger tem versão sem arquivo
}

#[must_use]
pub fn plan(set: &MigrationSet, applied: &[AppliedMigration]) -> Plan;
```

`down` só existe como comando explícito: **boot nunca desfaz**, e `Command::Down` carrega apenas `to`.
Quem exige `--force` para destino não descartável é o **binário**, antes de chamar `run`: a trava não é
variável de ambiente nem campo da biblioteca — se fosse campo, todo chamador teria de repetir a regra.
Um `bool` só onde o nome do comando já diz o que ele faz (`--dry-run`, `--check`, `--force`).

### 8.1 Reset: `down` até vazio, nunca `DROP SCHEMA`

O runner herdado recebia `reset` como parâmetro e executava `DROP SCHEMA IF EXISTS public CASCADE;
CREATE SCHEMA public;` (`V`, `crates/wyd-db/src/migrate.rs:236-240`, 2026-11-17). Isso não atravessa a
troca de série, por dois motivos independentes:

- **Derruba o schema errado.** Na 1.0.0 o ledger mora em `public`, mas os objetos da série moram em
  `content`, `game`, `portal` e `ops`. Um `DROP` só de `public` apagaria o ledger e deixaria os quatro
  schemas vivos — exatamente o "schema pré-existente sem ledger" que a `ADR-DB-0002` trata como erro,
  não como baseline.
- **`CASCADE` anula o gate.** O `down` do `0001` derruba os quatro schemas **sem** `CASCADE`, de
  propósito: se outro arquivo esqueceu um objeto, o `down` estoura com "other objects depend on it" e
  aponta o culpado (`planejamento-1.0.0/wyd-db/migrations/0001-bootstrap.md` §8). Com `CASCADE`, o
  mesmo defeito passa verde e a sobra desaparece sem ninguém saber.

O reset passa a ser **um alvo do `down`**: `Command::Down { to: DownTo::Empty }`, exposto na CLI como
`down --to 0` (`Version` é não-zero; `0` não é versão, é ausência). Mesma sessão, mesmo advisory lock,
uma transação por arquivo, linha do ledger apagada arquivo por arquivo. Ao fim:

| Estado | Depois de `down --to 0` |
|---|---|
| os quatro schemas de domínio | não existem |
| `public.schema_migrations` | existe, **vazia**, com a forma validada por `prepare` e ainda acessível por `wyd_migrator` — nenhum arquivo da série toca em `public` |
| papéis do cluster | intactos (`ADR-DB-0001`, invariante (e): reset nunca toca papel) |
| `apply` seguinte | aplica as 34 do zero, porque não há linha aplicada |

Consequências que o desenho aceita:

- **A biblioteca não emite `DROP` que não esteja num arquivo da série.** Instrução destrutiva fora do
  `up`/`down` seria SQL de controle escondido na orquestração, o oposto do invariante (c).
- **Reset não substitui banco novo.** `down --to 0` devolve os quatro schemas e esvazia o ledger, mas
  não desfaz o que é de ambiente: o estado de `public` e os seis papéis continuam como o
  provisionamento os deixou (`src/provision/spec.md` §4; `ADR-DB-0001`, item 8). Em CI e teste, banco
  descartável (`WYD_DB_URL_TEST`, `src/provision/spec.md` §3) continua sendo o caminho; `down --to 0` é
  para o ambiente de desenvolvimento com banco vivo.
- **A flag `--reset` vive no chamador.** Ela é composição de dois comandos da biblioteca — `down` até
  vazio e depois `apply` —, e o binário já é quem aplica a trava de `--force` (§8). Um `reset: bool` na
  biblioteca voltaria a duplicar essa trava e a esconder o que foi desfeito. `wyd-admin --reset` é hoje
  um atalho legado (`run_dir(dsn, dir, reset)`, `crates/wyd-admin/src/main.rs:1429`); a frente será
  replanejada e a assinatura dela não é razão para desenhar este módulo (decisão de 2026-11-18,
  `ADR-DB-0002`, "O que fica pendente").

## 9. Variáveis de ambiente

O módulo **não lê ambiente**. Quem lê é o binário (`wyd-db`) ou o chamador no boot (`wyd-admin`), que
monta `MigrateConfig` e chama a biblioteca. A tabela diz só quem lê e o que o módulo **pressupõe**;
nome, default e o efeito de faltar são do contrato de `src/provision/spec.md` §3.

| Variável | Quem lê | Obrigatória | Efeito |
|---|---|---|---|
| `WYD_DB_URL_MIGRATOR` | binário | sim | DSN do papel `wyd_migrator` |
| `WYD_MIGRATIONS_DIR` | binário | não | diretório da série |
| `WYD_MIGRATE_ALLOW_SUPERUSER` | binário | não | opt-out do preflight, só desenvolvimento, logado (`ADR-DB-0001`, Decisão item 6) |

Nada de `DATABASE_URL` (`src/provision/spec.md` §3). Só o timeout é campo de `MigrateConfig`; papel
esperado e versão de servidor são constantes (§7). Criar variável nova exige atualizar
`src/provision/spec.md` §3 e a decisão `M24`, e não é decisão deste módulo.

Papel esperado e versão de servidor **não são configuração**: são constantes do módulo. Uma checagem
de segurança que se pode desligar por arquivo de ambiente não é checagem — é sugestão. Mudar qualquer
uma das duas é commit com `ADR-DB-0002` revista, não variável nova.

```rust
/// Constantes do desenho. Mudam por commit + revisão de ADR, nunca por ambiente.
/// `MIGRATOR` vem de `provision::spec`: um nome de papel, uma fonte.
pub const EXPECTED_ROLE: &str = crate::provision::MIGRATOR;
pub const SERVER_MAJOR: u8 = 17;

/// Campos privados: só `new`/`with_lock_timeout` constroem.
pub struct MigrateConfig {
    dsn: Dsn,                     // de `crate::dsn`; a redação do Debug é de lá (`src/dsn.md`)
    dir: PathBuf,
    lock_timeout: Duration,       // padrão 10 s
    allow_superuser: bool,         // padrão false; o binário liga pelo opt-out
}

impl MigrateConfig {
    pub fn new(dsn: Dsn, dir: PathBuf) -> Self;
    /// Fora de 1 s..=10 min é erro do chamador, não valor a corrigir em silêncio.
    pub fn with_lock_timeout(self, d: Duration) -> Result<Self, ConfigError>;
    pub fn with_allow_superuser(self, allow: bool) -> Self;
    pub fn dsn(&self) -> &Dsn;
    pub fn dir(&self) -> &Path;
    pub fn lock_timeout(&self) -> Duration;
    pub fn allow_superuser(&self) -> bool;
}
```

`Dsn` não é tipo deste módulo: `crate::dsn::Dsn` (`src/dsn.md`) é o tipo único do crate — parse,
invariante e `Debug` redigido moram lá, e aqui só se recebe o valor pronto. O preflight compara
`current_user`, não lê a DSN.

Os campos obrigatórios cabem em `new`; `with_lock_timeout` e `with_allow_superuser` são os únicos ajustes.
Não existe `from_env`: ler variável é do binário (§10.1), e `config.rs` não importa `std::env`. O que
sobra de configurável é tempo e o opt-out logado, não regra de papel/versão.

## 10. Arquitetura do módulo

Quatro pastas por **camada**, não por arquivo. O `root` fica com o que é de todos: a fachada, a
configuração e o erro.

```text
src/migrate/
├── mod.rs              fachada: run(Command) + reexport público (sem I/O)
├── config.rs           MigrateConfig (campos privados), constantes do desenho
├── error.rs            MigrateError + Result (thiserror); sem anyhow no domínio
│
├── migration/          modelo puro — não conhece banco, arquivo nem ambiente
│   ├── mod.rs          reexport do que a camada de cima precisa
│   ├── version.rs      Version(NonZeroU32), Slug (valida snake_case)
│   ├── checksum.rs     Checksum, normalização de EOL, sha256
│   ├── sql.rs          Sql(String): não vazio, sem marcador, sem controle de transação
│   ├── file.rs         parser estrito dos marcadores → RawMigration
│   ├── set.rs          MigrationSet: ordem, contiguidade, duplicata, checksum
│   ├── applied.rs      AppliedMigration: o que o ledger devolve
│   └── plan.rs         plan(): disco × ledger → Plan
│
├── port/               contratos; traits pequenas, sem sqlx e sem fs
│   ├── mod.rs
│   ├── source.rs       Source + LoadedFile
│   ├── ledger.rs       Ledger (só leitura)
│   ├── unit_of_work.rs MigrationTxn + MigrationExecutor + SqlError
│   └── lock.rs         AdvisoryLock
│
├── app/                orquestração — depende de migration + port, nunca de adapter
│   ├── mod.rs
│   └── runner.rs       preflight → lock → prepare → apply/down → Report
│
└── adapter/            infraestrutura — único lugar com sqlx e std::fs
    ├── mod.rs
    ├── pg.rs           PgAdapter: from_config, Ledger, MigrationExecutor, AdvisoryLock
    └── fs.rs           FsSource: lista o diretório e lê os `.sql`
```

**Regra de dependência (a única que precisa ser vigiada):**

| Camada | Pode importar | Não pode |
|---|---|---|
| `migration/` | `core`; `std` de tipos (`NonZeroU32`, `fmt`) | `std::fs`, `std::io`, `std::env`, `port`, `app`, `adapter`, `sqlx`, `tokio` |
| `port/` | `migration`; `std` de tipos e `usize` de `Future` | `std::fs`, `std::env`, `app`, `adapter`, `sqlx` |
| `app/` | `migration`, `port` | `adapter`, `sqlx`, `std::fs`, `std::env` |
| `adapter/` | `migration`, `port`, `sqlx`, `tokio`, `std::fs` | `app` (o adapter não chama o runner) |
| `mod.rs` | todas, para reexportar a fachada e declarar `run` | lógica própria, `sqlx`, `std::fs`, `std::env` |

Leitura de ambiente é do **binário**, não do `mod.rs`: a fachada recebe `MigrateConfig` já montado.
Adicionar arquivo novo em `migration/` não muda contrato nenhum; `sqlx` fora de `adapter/`, ou
`std::fs` fora de `adapter/fs.rs`, são violações que o desenho proíbe e a revisão pega (o gate
automático ainda não existe).

Entre módulos irmãos, as duas únicas dependências permitidas são **constante e tipo compartilhado**:
`config.rs` importa `crate::provision::MIGRATOR` (§9) e `crate::dsn::Dsn` (`src/dsn.md`), e nada mais
— importar `provision::domain::plan` traria o estado desejado de papéis para dentro do runner.

SRP e DIP não têm linha própria porque **são** a regra de camadas acima: cada arquivo tem uma
responsabilidade, e as dependências apontam para dentro. A tabela fica com o que a tabela de camadas
não expressa.

| Princípio | Como aparece |
|---|---|
| **OCP** | comando novo é variante de `Command` (`#[non_exhaustive]`), tratada no `run`; `Plan` ganha variante, não `if` |
| **LSP** | `Ledger`/`AdvisoryLock` têm contrato testável; `FakeLedger` usado nos testes de contrato obedece o mesmo contrato |
| **ISP** | cinco portas pequenas (`Source`, `Ledger`, `MigrationTxn`, `MigrationExecutor`, `AdvisoryLock`) em vez de um `Database` gordo; cada uma com um consumidor |
| **Clean code** | newtype com invariante no construtor; sem `unwrap`/`expect` no caminho; sem `bool` como parâmetro; erro carrega a `Version`; `#[must_use]` em `plan` e nos relatórios |

**`async fn` em trait.** As portas declaram `async fn` e o `run` é sempre aguardado com `block_on`
(o boot em `crates/wyd-server/src/boot/mod.rs`, o binário e os testes), nunca com `tokio::spawn` — o
futuro devolvido não precisa prometer `Send`. Isso deixa o lint `async_fn_in_trait` (warn por padrão,
e o gate roda `clippy -D warnings`) silenciado em `port/mod.rs` com
`#![allow(async_fn_in_trait)]` e o motivo no comentário do arquivo. Se algum chamador passar a
exigir `Send`, a troca é a assinatura (`-> impl Future<Output = …> + Send`), não o `allow`.

`runner` é genérico nas portas (`run<B, S>` com `B: Ledger + MigrationExecutor + AdvisoryLock` e
`S: Source`), e o binário instancia com os dois adapters. É o que permite provar o runner inteiro
contra portas falsas, sem PostgreSQL e sem tocar o disco.

### 10.1 Raiz de composição

Um único lugar monta as peças: o binário. A biblioteca nunca constrói adapter sozinha, e por isso
`mod.rs` continua sem I/O.

```rust
// crates/wyd-db/src/bin/wyd-db.rs (a escrever) — único ponto que lê ambiente
let cfg = MigrateConfig::new(dsn_from_env()?, dir_from_env()?);  // env só aqui, no bin
let pg  = adapter::pg::from_config(&cfg).await?;   // pool + Ledger + Executor + Lock
let src = adapter::fs::FsSource::new(cfg.dir.clone());
let report = migrate::run(Command::Apply { dry_run: false }, &pg, &src).await?;
```

```rust
// migrate/mod.rs — assinatura da fachada: tudo injetado, nada construído aqui
pub async fn run<B, S>(cmd: Command, backend: &B, source: &S) -> Result<Report, MigrateError>
where
    B: Ledger + MigrationExecutor + AdvisoryLock,
    S: Source;
```

Os bounds de `Send + Sync` **não** se repetem aqui: são supertraits das quatro portas que o `run` usa
(`Source`, `Ledger`, `MigrationExecutor`, `AdvisoryLock`). `MigrationTxn` declara só `Send` — ela é
consumida por `&mut` numa única tarefa, e exigir `Sync` proibiria implementação legítima.

- **Um** `from_config` para as três portas: pool, ledger e lock compartilham a configuração e evitam
  dois lugares criando conexão com parâmetros diferentes.
- O lock usa conexão dedicada, obtida dentro do `PgAdapter` — o assunto não vaza para o chamador.
- Teste de contrato instancia `run` com `FakeLedger`/`FakeExecutor`/`FakeSource`/`FakeLock`, sem
  PostgreSQL: é a prova de que a fachada não tem I/O escondido.

## 11. Erros e códigos de saída

```rust
#[non_exhaustive]
pub enum MigrateError {
    Config(ConfigError),          // variável ausente: mensagem nomeia a variável
    Connect(ConnectError),
    Source(SourceError),
    WrongRole { found: String, expected: &'static str },
    WrongServer { found: String, expected: u8 },
    SetInvalid(SetError),         // buraco, duplicata, marcador, nome fora do padrão
    LockBusy { waited: Duration },
    Drift(Vec<Drift>),            // todos os achados, como no `Plan`
    Ledger(LedgerError),
    Sql { version: Version, source: SqlError },   // `SqlError` é da porta, não do sqlx
}
```

| Código | Situação |
|---:|---|
| 0 | aplicado, já atualizado ou `status` sem problema |
| 2 | configuração, conexão, preflight (papel, servidor, diretório ausente ou ilegível) ou ledger com forma divergente |
| 3 | lock ocupado |
| 4 | drift, arquivo faltando, série vazia ou série inválida |
| 5 | erro de SQL durante a aplicação |

A taxonomia é fechada por desenho: código novo é decisão registrada, não `catch-all`. O boot trata
`Drift`/`Sql` como **não servir** — subir com schema divergente é pior que ficar fora.

`MigrateError::Drift` é **derivado** do `Plan` de §8: `run` converte o `Plan::Drift` que já calculou, sem
reler disco nem ledger. Duas consequências: `status` e a falha nunca discordam, e um erro de drift custa
zero consulta extra.

**Regra de `#[non_exhaustive]`:** todo tipo público que pode crescer o leva — enums (`Command`, `Plan`,
`DriftKind`, `MigrateError` e os erros de porta) e structs de dados (`Report`, `RawMigration`,
`LoadedFile`, `AppliedMigration`, `Drift`): campo ou variante nova não é breaking change. Ficam de fora
as que já não podem ser construídas por literal por terem campo privado (`Checksum`, `MigrateConfig`) —
o atributo ali seria decorativo.

## 12. Observabilidade

- Um span por arquivo (`migration.apply`), campos `version`, `slug`, `elapsed_ms`.
- Um evento de resumo no fim, com os mesmos campos de `Report`: `applied`, `skipped`, `pending`,
  `findings`, `elapsed_ms`.
- **Nunca** logar DSN, usuário com senha ou o texto do SQL: o SQL pode conter seed com dado sensível,
  e o erro de conexão costuma trazer a URL inteira.
- `Report` é dado, não string: quem formata é o binário.

```rust
#[non_exhaustive]
pub struct Report {
    pub applied: Vec<Version>,
    pub skipped: usize,
    pub pending: usize,      // o que `status` mostra e o que sobra depois de `apply`
    pub findings: Vec<Drift>,
}
```

## 13. Testes (Definition of Done do módulo)

| Nível | O que prova |
|---|---|
| unidade | `Version`/`Slug`/`Checksum`/`Sql` (tabela de válidos e inválidos, incluindo vazio e `COMMIT`); parser dos marcadores; `plan` para os três casos e cada `DriftKind`; normalização `CRLF`→`LF` |
| contrato | `run` com `FakeSource`/`FakeLedger`/`FakeExecutor`/`FakeLock`: ordem de aplicação, para no primeiro erro, `down` em ordem inversa, `down` até `Empty` deixando zero versão aplicada, `status --check` saindo `4` em drift, `commit` chamado uma vez por arquivo, `LockBusy` saindo `3` com o lock tomado |
| sistema | contra PG17 real via `WYD_DB_URL_TEST`: aplica a série, ledger com 34 linhas, reaplica e não muda nada, drift de um byte falha, ledger pré-existente com coluna a menos é recusado por `Shape`, `down --to 1` deixa só a versão 1, `down --to 0` deixa os quatro schemas ausentes e ledger vazio, e um `apply` seguinte reaplica as 34, dois processos concorrentes → um aplica e o outro espera e termina sem aplicar, `CREATE TABLE` sem schema falha com `search_path` vazio, `status` num banco sem ledger lista as 34 pendentes e não cria a tabela |
| proibido | `sleep` para sincronizar; teste que depende de ordem de execução; `SKIP` por variável ausente (falha nomeando) |

## 14. O que a `ADR-DB-0002` congela

Os seis pontos abaixo estão escritos em
`docs/arquitetura/adr/adr-db-0002-runner-de-migrations-proprio.md`, aceita em 2026-11-18. Cada um
nomeia onde está a regra, sem repeti-la: (1) ledger em `public` com as quatro colunas validadas por
forma (§3); (2) checksum do arquivo inteiro com EOL normalizado (§4); (3) uma transação por arquivo,
como em §6; (4) advisory lock de sessão, chave constante e espera limitada (§5); (5) `plan` puro e SQL só por
`MigrationTxn` (§8 e §6); (6) três comandos, nada automático no boot, códigos de saída fixos (§8 e
§11), com o reset servido pelo `down` até vazio (§8.1). Este documento é a fonte da implementação, e os dois passam a mudar juntos — mexer nesses pontos
é revisão de ADR, não de implementação.

## 15. Estado da implementação (2026-11-25)

**Ato 3 concluído no escopo de biblioteca e portas falsas.** O módulo `crates/wyd-db/src/migrate/`
existe com parser, `MigrationSet`, `plan`, portas, adapter PostgreSQL, `FsSource` e runner. O registro
de fechamento da nota transversal é `../../migrations/serie-e-regras.md` §8; esta seção guarda a mesma
evidência no documento de contrato que a nota cita como `Base:`.

| Comando | Resultado observado |
|---|---|
| `cargo test --manifest-path crates/wyd-db/Cargo.toml --lib` | **63 passed; 0 failed; 0 ignored** (exit 0) |
| `cargo check --manifest-path crates/wyd-db/Cargo.toml --all-targets` | concluído sem erro (exit 0) |
| `cargo clippy --manifest-path crates/wyd-db/Cargo.toml --all-targets -- -D warnings` | concluído sem aviso (exit 0) |
| `cargo fmt --manifest-path crates/wyd-db/Cargo.toml -- --check` | concluído sem diferença (exit 0) |

**Cobertura:** a prova é de **lib/fakes** (unidade e contrato), não de sistema com PostgreSQL. Não
foram executados `tests/sistema_pg.rs` (falta `WYD_DB_TEST_CONTAINER`/PG17), o gate de documentação em
container nem teste de binário/CLI. As 28 migrations materializadas têm **28** `up`, **0** `down` e
não foram alteradas; a decisão de rollback é `Q-SERIE-1` em `../../migrations/serie-e-regras.md` §9, com
recomendação **B** (escrever os 28 `down` reais).

A próxima frente canônica continua **A — D-01/`wyd-db`**, com a fatia **ato 4** (`CAP-37`–`CAP-39`,
3 células) e depois **ato 5** (40 issues + 141 atos).

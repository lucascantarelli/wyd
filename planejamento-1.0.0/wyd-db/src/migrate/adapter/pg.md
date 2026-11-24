> Pergunta: "como o adapter fala com o PG17: pool, lock dedicado, transação por arquivo e forma do ledger?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/adapter/pg.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §3, §5, §6, §7 e §10.1
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/adapter/pg.rs` — `PgAdapter`

## 1. Responsabilidade

Implementar `Ledger`, `MigrationExecutor` e `AdvisoryLock` sobre `sqlx`. É o único arquivo do runner com
SQL de infraestrutura (ledger e lock), e o único que conhece `sqlx::Error`.

## 2. Estrutura

```rust
pub struct PgAdapter {
    pool: sqlx::PgPool,                        // execução e leitura
    lock_pool: sqlx::PgPool,                   // max_connections = 1, sessão dedicada
    lock_held: tokio::sync::Mutex<Option<sqlx::postgres::PgConnection>>,
    allow_superuser: bool,                     // de MigrateConfig
}

impl PgAdapter {
    pub async fn from_config(cfg: &MigrateConfig) -> Result<Self, ConnectError>;
}
```

`from_config` monta **dois** pools a partir da mesma DSN: o de execução (padrão `max_connections = 1`,
porque a série é serial) e o de lock (`max_connections = 1`), e lê `cfg` para guardar o opt-out.

## 3. Ledger: DDL e forma

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

`prepare` (idempotente, sob lock):

1. `SELECT to_regclass('public.schema_migrations')` — `NULL` → executa o DDL acima e retorna.
2. Existe → uma consulta em `information_schema.columns` para as quatro colunas (`version`, `slug`,
   `checksum`, `applied_at`) com tipo canônico e nulidade, mais `pg_constraint` para a PK em `version` e
   as duas `CHECK` **pelo nome**.
3. Qualquer divergência → `LedgerError::Shape { detail }` com a diferença em texto legível. Nunca
   `IF NOT EXISTS`.

Constante local `LEDGER_COLUMNS: [(&str, &str, bool); 4]` é o dado comparado — não `if` espalhado.

Leitura (`impl Ledger for PgAdapter`), **sem** DDL e sem lock:

1. `SELECT to_regclass('public.schema_migrations')` — `NULL` → `Ok(vec![])`. É o caminho de `status` e
   `apply --dry-run` num banco virgem, que nunca chamam `prepare`.
2. Existe → `SELECT version, slug, checksum FROM public.schema_migrations`. Cada linha vira
   `AppliedMigration` com os três `FromStr`/`try_into`; valor recusado → `Shape`.
3. `42703` (`undefined_column`) → `Shape { detail }` com o nome da coluna; qualquer outro erro de
   consulta → `Query { detail }`, redigido. Ausência de tabela **nunca** é erro.

## 4. Transação

```rust
pub struct PgTxn { tx: sqlx::Transaction<'static, sqlx::Postgres> }
```

`begin` abre a transação e executa `SET LOCAL search_path = ''` na mesma transação. `run` usa
`sqlx::raw_sql(sql.as_ref())` (múltiplas instruções por arquivo). `record` faz `INSERT INTO
public.schema_migrations (version, slug, checksum) VALUES ($1,$2,$3)` com binds tipados. `forget` faz
`DELETE … WHERE version = $1`. `commit` consome e faz `COMMIT`; o `Drop` de `sqlx::Transaction` já faz
rollback, então não há `impl Drop` manual.

## 5. Lock

`acquire(timeout)`: pega a conexão do `lock_pool`, executa `SET lock_timeout = $1ms`, depois
`SELECT pg_advisory_lock($key)` com `MIGRATION_LOCK_KEY`; guarda a conexão em `lock_held` (mantém a
sessão). Erro `55P03` (`lock_not_available`) → `LockError::Busy { waited: timeout }`. `LockError::Query`
para o resto.

## 6. Preflight e política

`server_info` lê `current_user` e `current_setting('server_version_num')::int`.
`allows_superuser` devolve o campo — o runner decide, o adapter só informa.

## 7. Conversão de erro

| De | Para |
|---|---|
| `sqlx::Error` de conexão | `ConnectError::Refused/Pool { detail }`, **redigido** (sem DSN) |
| `sqlx::Error` de execução | `SqlError::{Server, Position}` extraindo `code()` e `position()` |
| tabela do ledger ausente | `Ok(vec![])`, não erro |
| coluna do ledger ausente (`42703`) | `LedgerError::Shape` |
| linha do ledger inválida | `LedgerError::Shape` |

## 8. Testes (sistema, PG17 efêmero)

| Teste | O que prova |
|---|---|
| `prepare_creates_then_is_noop` | duas chamadas, uma tabela |
| `applied_on_missing_table_is_empty` | banco sem ledger → `Ok(vec![])` e `to_regclass` continua `NULL` |
| `applied_maps_missing_column_to_shape` | tabela sem `checksum` → `Shape`, não `Query` |
| `prepare_rejects_extra_column` | coluna a mais → `Shape` |
| `begin_sets_empty_search_path` | `CREATE TABLE account` sem schema falha |
| `record_and_forget_roundtrip` | inserir e apagar deixa o ledger como estava |
| `lock_busy_maps_to_55P03` | segundo processo → `Busy`, não erro genérico |
| `connect_error_does_not_leak_dsn` | `format!("{err}")` sem senha |

## 9. Pronto quando

- nenhum `unwrap`/`expect` no caminho;
- `sqlx::Error` não aparece em tipo público;
- o DDL do ledger existe **uma** vez (aqui) e é o mesmo de `ddl-confirmado.md` §3.

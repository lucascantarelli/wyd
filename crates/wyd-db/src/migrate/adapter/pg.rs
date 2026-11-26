//! Adapter PostgreSQL do runner: ledger, transação, preflight e lock.

use crate::migrate::config::MigrateConfig;
use crate::migrate::error::ConnectError;
use crate::migrate::migration::applied::AppliedMigration;
use crate::migrate::migration::checksum::Checksum;
use crate::migrate::migration::file::RawMigration;
use crate::migrate::migration::sql::Sql;
use crate::migrate::migration::version::{Slug, Version};
use crate::migrate::port::ledger::{Ledger, LedgerError};
use crate::migrate::port::lock::{AdvisoryLock, LockError, MIGRATION_LOCK_KEY};
use crate::migrate::port::unit_of_work::{MigrationExecutor, MigrationTxn, ServerInfo, SqlError};
use sqlx::postgres::{PgDatabaseError, PgErrorPosition, PgPoolOptions};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::Mutex;

const LEDGER_DDL: &str = r#"
CREATE TABLE public.schema_migrations (
    version    INTEGER     PRIMARY KEY,
    slug       TEXT        NOT NULL,
    checksum   TEXT        NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT ck_schema_migrations_version  CHECK (version > 0),
    CONSTRAINT ck_schema_migrations_checksum CHECK (checksum ~ '^[0-9a-f]{64}$')
)
"#;

/// Pool de execução, pool dedicado ao lock e o lock mantido pela sessão.
pub struct PgAdapter {
    pool: PgPool,
    lock_pool: PgPool,
    lock_held: Mutex<Option<sqlx::pool::PoolConnection<Postgres>>>,
    allow_superuser: bool,
}

impl PgAdapter {
    /// Abre os dois pools a partir da configuração.
    pub async fn from_config(config: &MigrateConfig) -> Result<Self, ConnectError> {
        let pool = abrir_pool(config).await?;
        let lock_pool = abrir_pool(config).await?;
        Ok(Self {
            pool,
            lock_pool,
            lock_held: Mutex::new(None),
            allow_superuser: config.allow_superuser(),
        })
    }

    /// Fecha os pools; útil para o chamador liberar recursos.
    ///
    /// A conexão que mantém o advisory lock precisa voltar ao pool antes do `close`: com
    /// `max_connections(1)`, fechar o `lock_pool` com ela ainda presa esperaria para sempre.
    pub async fn close(self) {
        drop(self.lock_held.lock().await.take());
        self.pool.close().await;
        self.lock_pool.close().await;
    }
}

async fn abrir_pool(config: &MigrateConfig) -> Result<PgPool, ConnectError> {
    PgPoolOptions::new()
        .max_connections(1)
        .connect(config.dsn().as_str())
        .await
        .map_err(|_| ConnectError::Refused {
            detail: "não foi possível abrir o pool".to_owned(),
        })
}

impl Ledger for PgAdapter {
    async fn applied(&self) -> Result<Vec<AppliedMigration>, LedgerError> {
        if !ledger_exists(&self.pool).await? {
            return Ok(Vec::new());
        }
        validate_ledger_shape(&self.pool).await?;
        let rows = sqlx::query(
            "SELECT version, slug, checksum FROM public.schema_migrations ORDER BY version",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(query_error)?;

        let mut applied = Vec::with_capacity(rows.len());
        for row in rows {
            let version_raw: i32 = row.try_get("version").map_err(row_shape)?;
            let version_u32 =
                u32::try_from(version_raw).map_err(|_| shape("version fora de u32"))?;
            let version = Version::new(version_u32).map_err(|error| shape(&error.to_string()))?;
            let slug_raw: String = row.try_get("slug").map_err(row_shape)?;
            let slug = Slug::from_str(&slug_raw).map_err(|error| shape(&error.to_string()))?;
            let checksum_raw: String = row.try_get("checksum").map_err(row_shape)?;
            let checksum =
                Checksum::from_str(&checksum_raw).map_err(|error| shape(&error.to_string()))?;
            applied.push(AppliedMigration {
                version,
                slug,
                checksum,
            });
        }
        Ok(applied)
    }
}

impl MigrationExecutor for PgAdapter {
    type Txn = PgTxn;

    async fn prepare(&self) -> Result<(), LedgerError> {
        if !ledger_exists(&self.pool).await? {
            sqlx::raw_sql(LEDGER_DDL)
                .execute(&self.pool)
                .await
                .map_err(query_error)?;
            return Ok(());
        }
        validate_ledger_shape(&self.pool).await
    }

    async fn begin(&self) -> Result<Self::Txn, SqlError> {
        let mut tx = self.pool.begin().await.map_err(sql_error)?;
        sqlx::query("SET LOCAL search_path = ''")
            .execute(&mut *tx)
            .await
            .map_err(sql_error)?;
        Ok(PgTxn { tx })
    }

    async fn server_info(&self) -> Result<ServerInfo, SqlError> {
        let row =
            sqlx::query("SELECT current_user::text, current_setting('server_version_num')::int")
                .fetch_one(&self.pool)
                .await
                .map_err(sql_error)?;
        let current_user: String = row.try_get(0).map_err(row_sql_error)?;
        let server_version_num: i32 = row.try_get(1).map_err(row_sql_error)?;
        Ok(ServerInfo {
            current_user,
            server_version_num,
        })
    }

    fn allows_superuser(&self) -> bool {
        self.allow_superuser
    }
}

impl AdvisoryLock for PgAdapter {
    async fn acquire(&self, timeout: Duration) -> Result<(), LockError> {
        let mut conn = self.lock_pool.acquire().await.map_err(lock_error)?;
        let millis = timeout.as_millis().min(i64::MAX as u128) as i64;
        sqlx::query("SELECT set_config('lock_timeout', $1, false)")
            .bind(format!("{millis}ms"))
            .execute(&mut *conn)
            .await
            .map_err(lock_error)?;
        sqlx::query("SELECT pg_advisory_lock($1)")
            .bind(MIGRATION_LOCK_KEY)
            .execute(&mut *conn)
            .await
            .map_err(|error| {
                if error
                    .as_database_error()
                    .and_then(|db| db.code())
                    .is_some_and(|code| code == "55P03")
                {
                    LockError::Busy { waited: timeout }
                } else {
                    lock_error(error)
                }
            })?;
        *self.lock_held.lock().await = Some(conn);
        Ok(())
    }
}

/// Transação de um arquivo; sem `commit`, `Drop` faz rollback via `sqlx`.
pub struct PgTxn {
    tx: Transaction<'static, Postgres>,
}

impl MigrationTxn for PgTxn {
    async fn run(&mut self, sql: &Sql) -> Result<(), SqlError> {
        sqlx::raw_sql(sql.as_ref())
            .execute(&mut *self.tx)
            .await
            .map(|_| ())
            .map_err(sql_error)
    }

    async fn record(
        &mut self,
        migration: &RawMigration,
        checksum: Checksum,
    ) -> Result<(), SqlError> {
        sqlx::query(
            "INSERT INTO public.schema_migrations (version, slug, checksum) VALUES ($1, $2, $3)",
        )
        .bind(migration.version.get() as i32)
        .bind(migration.slug.as_str())
        .bind(checksum.to_string())
        .execute(&mut *self.tx)
        .await
        .map(|_| ())
        .map_err(sql_error)
    }

    async fn forget(&mut self, version: Version) -> Result<(), SqlError> {
        sqlx::query("DELETE FROM public.schema_migrations WHERE version = $1")
            .bind(version.get() as i32)
            .execute(&mut *self.tx)
            .await
            .map(|_| ())
            .map_err(sql_error)
    }

    async fn commit(self) -> Result<(), SqlError> {
        self.tx.commit().await.map_err(sql_error)
    }
}

async fn ledger_exists(pool: &PgPool) -> Result<bool, LedgerError> {
    let found: Option<String> =
        sqlx::query_scalar("SELECT to_regclass('public.schema_migrations')::text")
            .fetch_one(pool)
            .await
            .map_err(query_error)?;
    Ok(found.is_some())
}

async fn validate_ledger_shape(pool: &PgPool) -> Result<(), LedgerError> {
    let rows = sqlx::query(
        "SELECT column_name::text, data_type::text, is_nullable::text
           FROM information_schema.columns
          WHERE table_schema = 'public' AND table_name = 'schema_migrations'",
    )
    .fetch_all(pool)
    .await
    .map_err(query_error)?;
    if rows.len() != 4 {
        return Err(shape("schema_migrations precisa ter exatamente 4 colunas"));
    }

    let expected = [
        ("version", "integer", false),
        ("slug", "text", false),
        ("checksum", "text", false),
        ("applied_at", "timestamp with time zone", false),
    ];
    let mut found: HashMap<String, (String, bool)> = HashMap::new();
    for row in rows {
        let name: String = row.try_get("column_name").map_err(row_shape)?;
        let tipo: String = row.try_get("data_type").map_err(row_shape)?;
        let nullable: String = row.try_get("is_nullable").map_err(row_shape)?;
        found.insert(name, (tipo, nullable == "YES"));
    }
    for (name, tipo, nullable) in expected {
        match found.get(name) {
            Some((actual_type, actual_nullable))
                if actual_type == tipo && *actual_nullable == nullable => {}
            _ => {
                return Err(shape(&format!(
                    "coluna {name} divergente: esperado {tipo} not null={nullable}"
                )))
            }
        }
    }

    let constraints = sqlx::query(
        "SELECT c.conname::text, c.contype::text, pg_get_constraintdef(c.oid)::text
           FROM pg_constraint c
           JOIN pg_class t ON t.oid = c.conrelid
           JOIN pg_namespace n ON n.oid = t.relnamespace
          WHERE n.nspname = 'public' AND t.relname = 'schema_migrations'",
    )
    .fetch_all(pool)
    .await
    .map_err(query_error)?;
    let mut by_name: HashMap<String, (String, String)> = HashMap::new();
    for row in constraints {
        let name: String = row.try_get(0).map_err(row_shape)?;
        let tipo: String = row.try_get(1).map_err(row_shape)?;
        let definition: String = row.try_get(2).map_err(row_shape)?;
        by_name.insert(name, (tipo, definition));
    }
    let Some((pk_type, pk_def)) = by_name.get("schema_migrations_pkey") else {
        return Err(shape("falta a chave primária schema_migrations_pkey"));
    };
    if pk_type != "p" || !pk_def.contains("version") {
        return Err(shape("schema_migrations_pkey não é PRIMARY KEY (version)"));
    }
    for name in [
        "ck_schema_migrations_version",
        "ck_schema_migrations_checksum",
    ] {
        let Some((constraint_type, _)) = by_name.get(name) else {
            return Err(shape(&format!("falta a constraint {name}")));
        };
        if constraint_type != "c" {
            return Err(shape(&format!("{name} não é CHECK")));
        }
    }
    Ok(())
}

fn shape(detail: &str) -> LedgerError {
    LedgerError::Shape {
        detail: detail.to_owned(),
    }
}

fn query_error(error: sqlx::Error) -> LedgerError {
    LedgerError::Query {
        detail: format!("{error}"),
    }
}

fn row_shape(error: sqlx::Error) -> LedgerError {
    shape(&format!("linha do ledger inválida: {error}"))
}

fn sql_error(error: sqlx::Error) -> SqlError {
    if let Some(db) = error.as_database_error() {
        let sqlstate = db.code().map(|code| code.into_owned());
        let message = db.message().to_owned();
        let position =
            db.try_downcast_ref::<PgDatabaseError>()
                .and_then(|pg| match pg.position() {
                    Some(PgErrorPosition::Original(position)) => u32::try_from(position).ok(),
                    Some(PgErrorPosition::Internal { position, .. }) => {
                        u32::try_from(position).ok()
                    }
                    None => None,
                });
        if let Some(position) = position {
            SqlError::Position {
                sqlstate,
                message,
                position: Some(position),
            }
        } else {
            SqlError::Server { sqlstate, message }
        }
    } else {
        SqlError::Server {
            sqlstate: None,
            message: "falha de SQL".to_owned(),
        }
    }
}

fn row_sql_error(error: sqlx::Error) -> SqlError {
    SqlError::Server {
        sqlstate: None,
        message: error.to_string(),
    }
}

fn lock_error(error: sqlx::Error) -> LockError {
    LockError::Query {
        detail: error.to_string(),
    }
}

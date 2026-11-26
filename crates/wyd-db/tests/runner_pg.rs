//! Alvo `runner_pg` — a porta `PgAdapter` real contra PostgreSQL 17 (`CAP-37`).
//!
//! Contrato: `planejamento-1.0.0/wyd-db/src/migrate/spec.md` §13, `proposta-alvo-pgadapter.md`
//! (`Q-ALVO-1` = A1, `Q-ALVO-2` = B1, `Q-ALVO-3` = papel de teste, decididas em 2026-11-25) e
//! `tests/specs/capacidades.md` (`CAP-37`, `RNF-DB-002`).
//!
//! A série real ainda não é contígua (faltam `0016`, `0017`, `0018`, `0020`, `0023` e `0027`), então o
//! alvo aplica uma **fixture contígua** de quatro arquivos. O que ele prova não depende de qual série
//! roda: ledger real, transação por arquivo, `search_path = ''`, drift e o advisory lock em duas
//! sessões. A série materializada continua provada por `reversibilidade_pg`.
//!
//! O banco vem de `WYD_DB_TEST_URL`, com TCP em loopback e o papel `wyd_migrator` com `LOGIN` — o
//! preflight do runner recusa outro papel. Sem a variável o alvo **falha**, não se ignora.

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use wyd_db::migrate::adapter::fs::FsSource;
use wyd_db::migrate::adapter::pg::PgAdapter;
use wyd_db::migrate::config::{MigrateConfig, LOCK_TIMEOUT_DEFAULT};
use wyd_db::migrate::error::MigrateError;
use wyd_db::migrate::migration::plan::DriftKind;
use wyd_db::migrate::port::AdvisoryLock;
use wyd_db::migrate::{Command, DownTo, Report};

/// Os testes compartilham um banco só; a execução paralela embaralharia o `reset`.
static SERIAL: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn url() -> String {
    std::env::var("WYD_DB_TEST_URL").unwrap_or_else(|_| {
        panic!(
            "WYD_DB_TEST_URL não definida: o alvo `runner_pg` precisa de PostgreSQL 17 em \
             `127.0.0.1` e do papel `wyd_migrator` com LOGIN"
        )
    })
}

async fn pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&url())
        .await
        .expect("pool de teste")
}

fn fixture(nome: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("resources")
        .join(nome)
}

fn config(nome: &str) -> MigrateConfig {
    MigrateConfig::new(url().parse().expect("DSN de teste"), fixture(nome))
}

async fn apagar() {
    let pool = pool().await;
    sqlx::raw_sql(
        "DROP SCHEMA IF EXISTS fixture CASCADE; \
         DROP TABLE IF EXISTS public.schema_migrations;",
    )
    .execute(&pool)
    .await
    .expect("limpeza do banco de teste");
    pool.close().await;
}

async fn contar_ledger() -> i64 {
    let pool = pool().await;
    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM public.schema_migrations")
        .fetch_one(&pool)
        .await
        .expect("contagem do ledger");
    pool.close().await;
    total
}

async fn existe(objeto: &str) -> bool {
    let pool = pool().await;
    let achado: Option<String> = sqlx::query_scalar("SELECT to_regclass($1)::text")
        .bind(objeto)
        .fetch_one(&pool)
        .await
        .expect("to_regclass");
    pool.close().await;
    achado.is_some()
}

async fn aplicar(cfg: &MigrateConfig) -> Result<Report, MigrateError> {
    let adapter = PgAdapter::from_config(cfg).await.expect("adapter");
    let fonte = FsSource::new(cfg.dir().to_path_buf());
    let resultado = wyd_db::migrate::run(Command::Apply { dry_run: false }, &adapter, &fonte).await;
    adapter.close().await;
    resultado
}

#[tokio::test]
async fn apply_grava_ledger_real_e_e_idempotente() {
    let _serial = SERIAL.lock().await;
    apagar().await;
    let cfg = config("fixture");

    let primeiro = aplicar(&cfg).await.expect("primeiro apply");
    assert_eq!(primeiro.applied.len(), 4, "a fixture tem quatro arquivos");
    assert_eq!(contar_ledger().await, 4);

    let pool = pool().await;
    let checksum: String =
        sqlx::query_scalar("SELECT checksum FROM public.schema_migrations WHERE version = 1")
            .fetch_one(&pool)
            .await
            .expect("checksum da versão 1");
    let slug: String =
        sqlx::query_scalar("SELECT slug FROM public.schema_migrations WHERE version = 1")
            .fetch_one(&pool)
            .await
            .expect("slug da versão 1");
    pool.close().await;
    assert_eq!(checksum.len(), 64, "checksum em hexadecimal de 64 dígitos");
    assert_eq!(slug, "fixture_base");

    let segundo = aplicar(&cfg).await.expect("segundo apply");
    assert!(segundo.applied.is_empty(), "nada a reaplicar");
    assert_eq!(segundo.skipped, 4);
    assert_eq!(contar_ledger().await, 4, "reaplicar não duplica ledger");
}

#[tokio::test]
async fn arquivo_que_falha_nao_commita_parcial() {
    let _serial = SERIAL.lock().await;
    apagar().await;
    let cfg = config("fixture_falha");

    let erro = aplicar(&cfg)
        .await
        .expect_err("o segundo arquivo divide por zero");
    assert!(
        matches!(&erro, MigrateError::Sql { version, .. } if version.get() == 2),
        "esperava falha na versão 2; veio: {erro}"
    );
    assert_eq!(erro.exit_code(), 5);

    assert_eq!(contar_ledger().await, 1, "só a versão 1 commitou");
    assert!(
        !existe("fixture.meio").await,
        "o DDL da versão 2 precisa ter sido desfeito pelo rollback"
    );
    assert!(existe("fixture.base").await, "a versão 1 continua aplicada");
}

#[tokio::test]
async fn drift_de_checksum_reprova_status_e_apply() {
    let _serial = SERIAL.lock().await;
    apagar().await;
    let cfg = config("fixture");
    aplicar(&cfg).await.expect("apply inicial");

    let pool = pool().await;
    sqlx::query("UPDATE public.schema_migrations SET checksum = repeat('0', 64) WHERE version = 2")
        .execute(&pool)
        .await
        .expect("checksum divergente");
    pool.close().await;

    let adapter = PgAdapter::from_config(&cfg).await.expect("adapter");
    let fonte = FsSource::new(cfg.dir().to_path_buf());
    let erro = wyd_db::migrate::run(Command::Status { check: true }, &adapter, &fonte)
        .await
        .expect_err("drift precisa reprovar o status");
    match &erro {
        MigrateError::Drift(achados) => {
            assert_eq!(achados.len(), 1, "só a versão 2 divergiu");
            assert_eq!(achados[0].version.get(), 2);
            assert_eq!(achados[0].kind, DriftKind::Checksum);
        }
        outro => panic!("esperava Drift; veio: {outro}"),
    }
    assert_eq!(erro.exit_code(), 4);

    let erro = wyd_db::migrate::run(Command::Apply { dry_run: false }, &adapter, &fonte)
        .await
        .expect_err("apply com drift não pode escrever");
    assert!(matches!(erro, MigrateError::Drift(_)));
    adapter.close().await;
    assert_eq!(
        contar_ledger().await,
        4,
        "o apply recusado não alterou o ledger"
    );
}

#[tokio::test]
async fn down_ate_vazio_esquece_o_ledger() {
    let _serial = SERIAL.lock().await;
    apagar().await;
    let cfg = config("fixture");
    aplicar(&cfg).await.expect("apply inicial");

    let adapter = PgAdapter::from_config(&cfg).await.expect("adapter");
    let fonte = FsSource::new(cfg.dir().to_path_buf());
    let relatorio = wyd_db::migrate::run(Command::Down { to: DownTo::Empty }, &adapter, &fonte)
        .await
        .expect("down até o vazio");
    adapter.close().await;

    assert_eq!(
        relatorio.applied.len(),
        4,
        "as quatro versões foram desfeitas"
    );
    assert_eq!(
        contar_ledger().await,
        0,
        "o ledger esquece o que foi desfeito"
    );
    assert!(!existe("fixture.item").await, "a tabela sumiu");
    assert!(!existe("fixture.item_contagem").await, "a view sumiu");
}

#[tokio::test]
async fn cap37_segunda_execucao_espera_no_advisory_lock() {
    let _serial = SERIAL.lock().await;
    apagar().await;
    let cfg = config("fixture");

    let dono = PgAdapter::from_config(&cfg)
        .await
        .expect("adapter dono do lock");
    dono.acquire(Duration::from_secs(1))
        .await
        .expect("lock do primeiro processo");

    let espera = PgAdapter::from_config(&cfg)
        .await
        .expect("adapter concorrente");
    let fonte = FsSource::new(cfg.dir().to_path_buf());
    let inicio = Instant::now();
    let erro = wyd_db::migrate::run(Command::Apply { dry_run: false }, &espera, &fonte)
        .await
        .expect_err("a segunda execução precisa falhar enquanto o lock está tomado");
    let decorrido = inicio.elapsed();

    assert!(
        matches!(&erro, MigrateError::LockBusy { waited } if *waited == LOCK_TIMEOUT_DEFAULT),
        "esperava LockBusy em {:?}; veio: {erro}",
        LOCK_TIMEOUT_DEFAULT
    );
    assert_eq!(erro.exit_code(), 3);
    assert!(
        decorrido < Duration::from_secs(15),
        "a espera de {decorrido:?} não pode pendurar a execução"
    );
    assert!(
        !existe("public.schema_migrations").await,
        "a execução concorrente não pode nem preparar o ledger"
    );

    dono.close().await;
    let relatorio = wyd_db::migrate::run(Command::Apply { dry_run: false }, &espera, &fonte)
        .await
        .expect("liberado o lock, a execução concorrente aplica");
    espera.close().await;
    assert_eq!(relatorio.applied.len(), 4);
}

//! Alvo `contrato_api_migrate` — congela a fachada pública de `migrate`.
//!
//! Escopo (`WYDDB-002`): `Command`, `DownTo`, `Report` e `run`. O contrato é de
//! compilação, não de execução: remover ou renomear variante/campo público
//! quebra este alvo; `#[non_exhaustive]` permite acrescentar sem quebrar. Nada
//! aqui abre conexão nem executa `run`.

use std::time::Duration;
use wyd_db::migrate::error::MigrateError;
use wyd_db::migrate::migration::applied::AppliedMigration;
use wyd_db::migrate::migration::checksum::Checksum;
use wyd_db::migrate::migration::file::RawMigration;
use wyd_db::migrate::migration::plan::Drift;
use wyd_db::migrate::migration::sql::Sql;
use wyd_db::migrate::migration::version::Version;
use wyd_db::migrate::port::{
    AdvisoryLock, Ledger, LedgerError, LoadedFile, LockError, MigrationExecutor, MigrationTxn,
    ServerInfo, Source, SourceError, SqlError,
};
use wyd_db::migrate::{run, Command, DownTo, Report};

/// Congela as variantes de `Command`, as duas formas de `DownTo` e os tipos dos
/// campos que carregam: `check`/`dry_run` são `bool` e `to` não tem `force`.
fn congelar_command(command: Command) {
    match command {
        Command::Status { check } => {
            let _: bool = check;
        }
        Command::Apply { dry_run } => {
            let _: bool = dry_run;
        }
        Command::Down { to } => match to {
            DownTo::Empty => {}
            DownTo::Version(version) => {
                let _: Version = version;
            }
            _ => panic!("DownTo ganhou variante sem atualizar o contrato"),
        },
        _ => panic!("Command ganhou variante sem atualizar o contrato"),
    }
}

/// Compila `run` e o retorno `Result<Report, MigrateError>`. Não é pollada,
/// então nenhuma porta é chamada.
async fn chamar_run() -> Result<Report, MigrateError> {
    run(Command::Status { check: false }, &FakeBackend, &FakeSource).await
}

struct FakeBackend;

impl Ledger for FakeBackend {
    async fn applied(&self) -> Result<Vec<AppliedMigration>, LedgerError> {
        Ok(Vec::new())
    }
}

impl MigrationExecutor for FakeBackend {
    type Txn = FakeTxn;

    async fn prepare(&self) -> Result<(), LedgerError> {
        Ok(())
    }

    async fn begin(&self) -> Result<Self::Txn, SqlError> {
        Ok(FakeTxn)
    }

    async fn server_info(&self) -> Result<ServerInfo, SqlError> {
        Err(SqlError::Server {
            sqlstate: None,
            message: "fake de contrato".to_owned(),
        })
    }

    fn allows_superuser(&self) -> bool {
        false
    }
}

impl AdvisoryLock for FakeBackend {
    async fn acquire(&self, _timeout: Duration) -> Result<(), LockError> {
        Ok(())
    }
}

struct FakeTxn;

impl MigrationTxn for FakeTxn {
    async fn run(&mut self, _sql: &Sql) -> Result<(), SqlError> {
        Ok(())
    }

    async fn record(
        &mut self,
        _migration: &RawMigration,
        _checksum: Checksum,
    ) -> Result<(), SqlError> {
        Ok(())
    }

    async fn forget(&mut self, _version: Version) -> Result<(), SqlError> {
        Ok(())
    }

    async fn commit(self) -> Result<(), SqlError> {
        Ok(())
    }
}

struct FakeSource;

impl Source for FakeSource {
    async fn load(&self) -> Result<Vec<LoadedFile>, SourceError> {
        Ok(Vec::new())
    }
}

#[test]
fn contrato_publico_de_migrate_compila() {
    congelar_command(Command::Status { check: false });
    congelar_command(Command::Apply { dry_run: true });
    congelar_command(Command::Down { to: DownTo::Empty });
    congelar_command(Command::Down {
        to: DownTo::Version(Version::new(1).expect("versão 1")),
    });

    // `Report` é `#[non_exhaustive]` sem construtor público: os campos e seus
    // tipos são congelados por acesso, num item anônimo que não gera código.
    const _: fn(&Report) = |report| {
        let _: &Vec<Version> = &report.applied;
        let _: &usize = &report.skipped;
        let _: &usize = &report.pending;
        let _: &Vec<Drift> = &report.findings;
    };

    // Constrói o future de `chamar_run` sem pollá-lo; `run` não é executado.
    let _pendente = chamar_run();
}

//! Orquestração do runner: preflight → lock → prepare → plano → execução.

use crate::migrate::config::{EXPECTED_ROLE, LOCK_TIMEOUT_DEFAULT, SERVER_MAJOR};
use crate::migrate::error::{ConnectError, MigrateError};
use crate::migrate::migration::file::{FileError, NameReason};
use crate::migrate::migration::plan::{self, Plan};
use crate::migrate::migration::set::{MigrationSet, SetError};
use crate::migrate::port::{AdvisoryLock, Ledger, MigrationExecutor, MigrationTxn, Source};
use crate::migrate::{Command, DownTo, Report};

/// Executa um comando do runner.
pub async fn run<B, S>(command: Command, backend: &B, source: &S) -> Result<Report, MigrateError>
where
    B: Ledger + MigrationExecutor + AdvisoryLock,
    S: Source,
{
    let files = source.load().await.map_err(MigrateError::Source)?;
    let pairs = to_pairs(files)?;
    let set = MigrationSet::build(pairs).map_err(MigrateError::SetInvalid)?;

    let info = backend.server_info().await.map_err(|source| {
        MigrateError::Connect(ConnectError::Pool {
            detail: source.to_string(),
        })
    })?;
    preflight(backend, &info)?;

    match command {
        Command::Status { check } => status(backend, &set, check).await,
        Command::Apply { dry_run: true } => status(backend, &set, false).await,
        Command::Apply { dry_run: false } => apply(backend, &set).await,
        Command::Down { to } => down(backend, &set, to).await,
    }
}

fn to_pairs(
    files: Vec<crate::migrate::port::source::LoadedFile>,
) -> Result<Vec<(String, String)>, MigrateError> {
    files
        .into_iter()
        .map(|file| {
            let stem = file
                .path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(str::to_owned)
                .ok_or_else(|| {
                    let file_name = file.path.display().to_string();
                    MigrateError::SetInvalid(SetError::File {
                        file_name: file_name.clone(),
                        source: FileError::BadName {
                            stem: file_name,
                            reason: NameReason::NoSeparator,
                        },
                    })
                })?;
            Ok((stem, file.content))
        })
        .collect()
}

fn preflight<B>(backend: &B, info: &crate::migrate::port::ServerInfo) -> Result<(), MigrateError>
where
    B: MigrationExecutor,
{
    if info.current_user != EXPECTED_ROLE && !backend.allows_superuser() {
        return Err(MigrateError::WrongRole {
            found: info.current_user.clone(),
            expected: EXPECTED_ROLE,
        });
    }
    let major = info.server_version_num / 10_000;
    if major != i32::from(SERVER_MAJOR) {
        return Err(MigrateError::WrongServer {
            found: info.server_version_num.to_string(),
            expected: SERVER_MAJOR,
        });
    }
    Ok(())
}

async fn status<B>(backend: &B, set: &MigrationSet, check: bool) -> Result<Report, MigrateError>
where
    B: Ledger,
{
    let applied = backend.applied().await.map_err(MigrateError::Ledger)?;
    let plano = plan::plan(set, &applied);
    let findings = match &plano {
        Plan::Drift(findings) => findings.clone(),
        Plan::Apply(_) | Plan::UpToDate => Vec::new(),
    };
    let pending = pending_count(set, &applied, &plano);
    if check && !findings.is_empty() {
        return Err(MigrateError::Drift(findings));
    }
    Ok(Report {
        applied: Vec::new(),
        skipped: applied.len(),
        pending,
        findings,
    })
}

async fn apply<B>(backend: &B, set: &MigrationSet) -> Result<Report, MigrateError>
where
    B: Ledger + MigrationExecutor + AdvisoryLock,
{
    backend
        .acquire(LOCK_TIMEOUT_DEFAULT)
        .await
        .map_err(|source| match source {
            crate::migrate::port::LockError::Busy { waited } => MigrateError::LockBusy { waited },
            crate::migrate::port::LockError::Query { detail } => {
                MigrateError::Connect(ConnectError::Pool { detail })
            }
        })?;
    backend.prepare().await.map_err(MigrateError::Ledger)?;
    let applied = backend.applied().await.map_err(MigrateError::Ledger)?;

    let plano = plan::plan(set, &applied);
    match plano {
        Plan::Drift(findings) => Err(MigrateError::Drift(findings)),
        Plan::UpToDate => Ok(Report {
            applied: Vec::new(),
            skipped: applied.len(),
            pending: 0,
            findings: Vec::new(),
        }),
        Plan::Apply(versions) => {
            let mut applied_now = Vec::with_capacity(versions.len());
            for version in versions {
                let entry = set.get(version).ok_or_else(|| {
                    MigrateError::SetInvalid(SetError::NotContiguous {
                        expected: version.get(),
                        found: version.get(),
                    })
                })?;
                let mut txn = backend
                    .begin()
                    .await
                    .map_err(|source| MigrateError::Sql { version, source })?;
                txn.run(&entry.raw.up)
                    .await
                    .map_err(|source| MigrateError::Sql { version, source })?;
                txn.record(&entry.raw, entry.checksum)
                    .await
                    .map_err(|source| MigrateError::Sql { version, source })?;
                txn.commit()
                    .await
                    .map_err(|source| MigrateError::Sql { version, source })?;
                applied_now.push(version);
            }
            Ok(Report {
                applied: applied_now,
                skipped: applied.len(),
                pending: 0,
                findings: Vec::new(),
            })
        }
    }
}

async fn down<B>(backend: &B, set: &MigrationSet, to: DownTo) -> Result<Report, MigrateError>
where
    B: Ledger + MigrationExecutor + AdvisoryLock,
{
    backend
        .acquire(LOCK_TIMEOUT_DEFAULT)
        .await
        .map_err(|source| match source {
            crate::migrate::port::LockError::Busy { waited } => MigrateError::LockBusy { waited },
            crate::migrate::port::LockError::Query { detail } => {
                MigrateError::Connect(ConnectError::Pool { detail })
            }
        })?;
    backend.prepare().await.map_err(MigrateError::Ledger)?;
    let applied = backend.applied().await.map_err(MigrateError::Ledger)?;
    let plano = plan::plan(set, &applied);
    if let Plan::Drift(findings) = plano {
        return Err(MigrateError::Drift(findings));
    }

    let mut alvos: Vec<_> = applied
        .iter()
        .filter(|migration| match to {
            DownTo::Empty => true,
            DownTo::Version(version) => migration.version > version,
        })
        .map(|migration| migration.version)
        .collect();
    alvos.sort_unstable_by(|a, b| b.cmp(a));

    let mut desfeitas = Vec::with_capacity(alvos.len());
    for version in alvos {
        let entry = set.get(version).ok_or_else(|| {
            MigrateError::SetInvalid(SetError::NotContiguous {
                expected: version.get(),
                found: version.get(),
            })
        })?;
        let mut txn = backend
            .begin()
            .await
            .map_err(|source| MigrateError::Sql { version, source })?;
        txn.run(&entry.raw.down)
            .await
            .map_err(|source| MigrateError::Sql { version, source })?;
        txn.forget(version)
            .await
            .map_err(|source| MigrateError::Sql { version, source })?;
        txn.commit()
            .await
            .map_err(|source| MigrateError::Sql { version, source })?;
        desfeitas.push(version);
    }

    let restantes = applied.len().saturating_sub(desfeitas.len());
    Ok(Report {
        applied: desfeitas,
        skipped: restantes,
        pending: set.len().saturating_sub(restantes),
        findings: Vec::new(),
    })
}

fn pending_count(
    set: &MigrationSet,
    applied: &[crate::migrate::migration::applied::AppliedMigration],
    plano: &Plan,
) -> usize {
    match plano {
        Plan::UpToDate => 0,
        Plan::Apply(versions) => versions.len(),
        Plan::Drift(_) => set.len().saturating_sub(applied.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrate::migration::applied::AppliedMigration;
    use crate::migrate::migration::checksum::Checksum;
    use crate::migrate::migration::version::{Slug, Version};
    use crate::migrate::port::source::LoadedFile;
    use crate::migrate::port::{LockError, ServerInfo, SqlError};
    use std::path::PathBuf;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct StaticSource {
        files: Vec<LoadedFile>,
    }

    impl Source for StaticSource {
        async fn load(&self) -> Result<Vec<LoadedFile>, crate::migrate::port::SourceError> {
            Ok(self.files.clone())
        }
    }

    fn conteudo(version: u32) -> String {
        format!("-- migrate:up\nSELECT {version};\n-- migrate:down\nSELECT {version};\n")
    }

    fn arquivo(version: u32, slug: &str) -> LoadedFile {
        LoadedFile {
            path: PathBuf::from(format!("{version:04}_{slug}.sql")),
            content: conteudo(version),
        }
    }

    fn aplicada(version: u32, slug: &str) -> AppliedMigration {
        AppliedMigration {
            version: Version::new(version).expect("versão"),
            slug: Slug::new(slug).expect("slug"),
            checksum: Checksum::of(conteudo(version).as_bytes()),
        }
    }

    #[derive(Default)]
    struct Estado {
        applied: Vec<AppliedMigration>,
        prepared: usize,
        locks: usize,
        begins: usize,
        runs: usize,
        forgets: Vec<Version>,
        fail_on_run: Option<usize>,
        busy: bool,
    }

    struct FakeBackend {
        estado: Arc<Mutex<Estado>>,
        role: String,
        server_version_num: i32,
    }

    impl FakeBackend {
        fn new() -> Self {
            Self {
                estado: Arc::new(Mutex::new(Estado::default())),
                role: EXPECTED_ROLE.to_owned(),
                server_version_num: 170_000,
            }
        }

        async fn estado(&self) -> tokio::sync::MutexGuard<'_, Estado> {
            self.estado.lock().await
        }
    }

    impl Ledger for FakeBackend {
        async fn applied(
            &self,
        ) -> Result<Vec<AppliedMigration>, crate::migrate::port::LedgerError> {
            Ok(self.estado().await.applied.clone())
        }
    }

    impl MigrationExecutor for FakeBackend {
        type Txn = FakeTxn;

        async fn prepare(&self) -> Result<(), crate::migrate::port::LedgerError> {
            self.estado().await.prepared += 1;
            Ok(())
        }

        async fn begin(&self) -> Result<Self::Txn, SqlError> {
            self.estado().await.begins += 1;
            Ok(FakeTxn {
                estado: Arc::clone(&self.estado),
                pendente: None,
            })
        }

        async fn server_info(&self) -> Result<ServerInfo, SqlError> {
            Ok(ServerInfo {
                current_user: self.role.clone(),
                server_version_num: self.server_version_num,
            })
        }

        fn allows_superuser(&self) -> bool {
            false
        }
    }

    impl AdvisoryLock for FakeBackend {
        async fn acquire(&self, waited: std::time::Duration) -> Result<(), LockError> {
            if self.estado().await.busy {
                return Err(LockError::Busy { waited });
            }
            self.estado().await.locks += 1;
            Ok(())
        }
    }

    enum Pendente {
        Registrar(AppliedMigration),
        Esquecer(Version),
    }

    struct FakeTxn {
        estado: Arc<Mutex<Estado>>,
        pendente: Option<Pendente>,
    }

    impl MigrationTxn for FakeTxn {
        async fn run(
            &mut self,
            _sql: &crate::migrate::migration::sql::Sql,
        ) -> Result<(), SqlError> {
            let mut estado = self.estado.lock().await;
            estado.runs += 1;
            if estado.fail_on_run == Some(estado.runs) {
                return Err(SqlError::Server {
                    sqlstate: Some("XX000".to_owned()),
                    message: "falha injetada".to_owned(),
                });
            }
            Ok(())
        }

        async fn record(
            &mut self,
            migration: &crate::migrate::migration::file::RawMigration,
            checksum: Checksum,
        ) -> Result<(), SqlError> {
            self.pendente = Some(Pendente::Registrar(AppliedMigration {
                version: migration.version,
                slug: migration.slug.clone(),
                checksum,
            }));
            Ok(())
        }

        async fn forget(&mut self, version: Version) -> Result<(), SqlError> {
            self.pendente = Some(Pendente::Esquecer(version));
            Ok(())
        }

        async fn commit(self) -> Result<(), SqlError> {
            let mut estado = self.estado.lock().await;
            match self.pendente {
                Some(Pendente::Registrar(migration)) => estado.applied.push(migration),
                Some(Pendente::Esquecer(version)) => {
                    estado.applied.retain(|item| item.version != version);
                    estado.forgets.push(version);
                }
                None => {}
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn status_on_virgin_db_lists_all_pending_without_prepare() {
        let backend = FakeBackend::new();
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois")],
        };
        let report = run(Command::Status { check: false }, &backend, &source)
            .await
            .expect("status");
        assert_eq!(report.pending, 2);
        assert!(report.applied.is_empty());
        assert_eq!(backend.estado().await.prepared, 0);
    }

    #[tokio::test]
    async fn apply_is_noop_when_up_to_date() {
        let backend = FakeBackend::new();
        backend.estado().await.applied = vec![aplicada(1, "um"), aplicada(2, "dois")];
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois")],
        };
        let report = run(Command::Apply { dry_run: false }, &backend, &source)
            .await
            .expect("apply");
        assert!(report.applied.is_empty());
        assert_eq!(report.skipped, 2);
        assert_eq!(report.pending, 0);
        assert_eq!(backend.estado().await.begins, 0);
        assert_eq!(backend.estado().await.prepared, 1);
    }

    #[tokio::test]
    async fn dry_run_lists_plan_without_lock_or_prepare() {
        let backend = FakeBackend::new();
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois")],
        };
        let report = run(Command::Apply { dry_run: true }, &backend, &source)
            .await
            .expect("dry-run");
        assert_eq!(report.pending, 2);
        assert!(report.applied.is_empty());
        assert_eq!(backend.estado().await.locks, 0);
        assert_eq!(backend.estado().await.prepared, 0);
        assert!(backend.estado().await.applied.is_empty());
    }

    #[tokio::test]
    async fn status_check_fails_with_drift_but_does_not_prepare() {
        let backend = FakeBackend::new();
        let mut alterada = aplicada(1, "um");
        alterada.checksum = Checksum::of(b"outro");
        backend.estado().await.applied = vec![alterada];
        let source = StaticSource {
            files: vec![arquivo(1, "um")],
        };
        let erro = run(Command::Status { check: true }, &backend, &source)
            .await
            .expect_err("check");
        assert!(matches!(erro, MigrateError::Drift(_)));
        assert_eq!(erro.exit_code(), 4);
        assert_eq!(backend.estado().await.prepared, 0);
    }

    #[tokio::test]
    async fn apply_executes_only_pending_in_ascending_order() {
        let backend = FakeBackend::new();
        backend.estado().await.applied = vec![aplicada(1, "um")];
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois"), arquivo(3, "tres")],
        };
        let report = run(Command::Apply { dry_run: false }, &backend, &source)
            .await
            .expect("apply");
        assert_eq!(
            report.applied,
            vec![Version::new(2).expect("2"), Version::new(3).expect("3")]
        );
        assert_eq!(backend.estado().await.applied.len(), 3);
        assert_eq!(backend.estado().await.begins, 2);
    }

    #[tokio::test]
    async fn drift_blocks_apply_before_any_begin() {
        let backend = FakeBackend::new();
        let mut alterada = aplicada(1, "um");
        alterada.checksum = Checksum::of(b"outro");
        backend.estado().await.applied = vec![alterada];
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois")],
        };
        let erro = run(Command::Apply { dry_run: false }, &backend, &source)
            .await
            .expect_err("drift");
        assert!(matches!(erro, MigrateError::Drift(_)));
        assert_eq!(erro.exit_code(), 4);
        assert_eq!(backend.estado().await.begins, 0);
        assert_eq!(backend.estado().await.prepared, 1);
    }

    #[tokio::test]
    async fn down_reverse_order_and_empty() {
        let backend = FakeBackend::new();
        backend.estado().await.applied = vec![aplicada(1, "um"), aplicada(2, "dois")];
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois")],
        };
        let report = run(
            Command::Down {
                to: DownTo::Version(Version::new(1).expect("1")),
            },
            &backend,
            &source,
        )
        .await
        .expect("down até 1");
        assert_eq!(report.applied, vec![Version::new(2).expect("2")]);
        assert_eq!(backend.estado().await.applied.len(), 1);
        assert_eq!(
            backend.estado().await.forgets,
            vec![Version::new(2).expect("2")]
        );
    }

    #[tokio::test]
    async fn down_to_empty_removes_every_applied_row_in_reverse_order() {
        let backend = FakeBackend::new();
        backend.estado().await.applied = vec![aplicada(1, "um"), aplicada(2, "dois")];
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois")],
        };
        let report = run(Command::Down { to: DownTo::Empty }, &backend, &source)
            .await
            .expect("down vazio");
        assert_eq!(
            report.applied,
            vec![Version::new(2).expect("2"), Version::new(1).expect("1")]
        );
        assert_eq!(report.skipped, 0);
        assert_eq!(report.pending, 2);
        assert!(backend.estado().await.applied.is_empty());
    }

    #[tokio::test]
    async fn failure_stops_at_first_error_and_keeps_previous_commit() {
        let backend = FakeBackend::new();
        backend.estado().await.fail_on_run = Some(2);
        let source = StaticSource {
            files: vec![arquivo(1, "um"), arquivo(2, "dois"), arquivo(3, "tres")],
        };
        let erro = run(Command::Apply { dry_run: false }, &backend, &source)
            .await
            .expect_err("falha");
        assert!(matches!(erro, MigrateError::Sql { version, .. } if version.get() == 2));
        assert_eq!(backend.estado().await.applied.len(), 1);
        assert_eq!(backend.estado().await.begins, 2);
        assert_eq!(backend.estado().await.runs, 2);
    }

    #[tokio::test]
    async fn wrong_role_and_busy_lock_are_rejected_before_work() {
        let mut backend = FakeBackend::new();
        backend.role = "postgres".to_owned();
        let source = StaticSource {
            files: vec![arquivo(1, "um")],
        };
        let erro = run(Command::Status { check: false }, &backend, &source)
            .await
            .expect_err("papel");
        assert!(matches!(erro, MigrateError::WrongRole { .. }));
        assert_eq!(backend.estado().await.prepared, 0);

        let backend = FakeBackend::new();
        backend.estado().await.busy = true;
        let erro = run(Command::Apply { dry_run: false }, &backend, &source)
            .await
            .expect_err("lock");
        assert!(matches!(erro, MigrateError::LockBusy { .. }));
        assert_eq!(erro.exit_code(), 3);
        assert_eq!(backend.estado().await.begins, 0);
    }
}

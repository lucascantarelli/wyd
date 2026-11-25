//! Portas do runner.
//!
//! `async fn` em trait é warning por padrão e o gate roda `clippy -D warnings`;
//! nenhum chamador usa `tokio::spawn`, todos aguardam o futuro. Se um chamador
//! passar a exigir `Send`, a troca é na assinatura, não neste `allow`.
#![allow(async_fn_in_trait)]

pub mod ledger;
pub mod lock;
pub mod source;
pub mod unit_of_work;

pub use ledger::{Ledger, LedgerError};
pub use lock::{AdvisoryLock, LockError};
pub use source::{LoadedFile, Source, SourceError};
pub use unit_of_work::{MigrationExecutor, MigrationTxn, ServerInfo, SqlError};

//! Configuração e constantes do runner.

use crate::dsn::Dsn;
use crate::migrate::error::ConfigError;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Papel que aplica a série. O provisionamento deverá usar esta mesma constante.
pub const EXPECTED_ROLE: &str = "wyd_migrator";

/// Única versão de servidor aceita pelo preflight.
pub const SERVER_MAJOR: u8 = 17;

/// Piso do `lock_timeout`.
pub const LOCK_TIMEOUT_MIN: Duration = Duration::from_secs(1);
/// Teto do `lock_timeout`.
pub const LOCK_TIMEOUT_MAX: Duration = Duration::from_secs(600);
/// Espera padrão pelo lock.
pub const LOCK_TIMEOUT_DEFAULT: Duration = Duration::from_secs(10);

/// Configuração do runner. Campos privados: só os construtores entram.
#[derive(Clone)]
pub struct MigrateConfig {
    dsn: Dsn,
    dir: PathBuf,
    lock_timeout: Duration,
    allow_superuser: bool,
}

impl MigrateConfig {
    /// Monta a configuração com o timeout padrão.
    #[must_use]
    pub fn new(dsn: Dsn, dir: PathBuf) -> Self {
        Self {
            dsn,
            dir,
            lock_timeout: LOCK_TIMEOUT_DEFAULT,
            allow_superuser: false,
        }
    }

    /// Ajusta o timeout, recusando fora de `1 s ..= 600 s`.
    pub fn with_lock_timeout(self, timeout: Duration) -> Result<Self, ConfigError> {
        if !(LOCK_TIMEOUT_MIN..=LOCK_TIMEOUT_MAX).contains(&timeout) {
            return Err(ConfigError::TimeoutOutOfRange { given: timeout });
        }
        Ok(Self {
            lock_timeout: timeout,
            ..self
        })
    }

    /// Opt-out explícito do preflight de superusuário; o binário lê a variável.
    #[must_use]
    pub fn with_allow_superuser(self, allow: bool) -> Self {
        Self {
            allow_superuser: allow,
            ..self
        }
    }

    /// DSN do migrator.
    #[must_use]
    pub fn dsn(&self) -> &Dsn {
        &self.dsn
    }

    /// Diretório da série.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Espera pelo advisory lock.
    #[must_use]
    pub fn lock_timeout(&self) -> Duration {
        self.lock_timeout
    }

    /// Se o preflight aceita um papel superusuário.
    #[must_use]
    pub fn allow_superuser(&self) -> bool {
        self.allow_superuser
    }
}

impl fmt::Debug for MigrateConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MigrateConfig")
            .field("dsn", &self.dsn)
            .field("dir", &self.dir)
            .field("lock_timeout", &self.lock_timeout)
            .field("allow_superuser", &self.allow_superuser)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> MigrateConfig {
        MigrateConfig::new(
            "postgres://u:segredo@h:5432/db".parse().expect("dsn"),
            PathBuf::from("migrations"),
        )
    }

    #[test]
    fn new_uses_default_timeout() {
        let config = cfg();
        assert_eq!(config.lock_timeout(), Duration::from_secs(10));
        assert_eq!(config.dir(), Path::new("migrations"));
        assert!(!config.allow_superuser());
        assert!(cfg().with_allow_superuser(true).allow_superuser());
    }

    #[test]
    fn timeout_accepts_bounds_and_is_settable() {
        assert!(cfg().with_lock_timeout(Duration::from_secs(1)).is_ok());
        assert!(cfg().with_lock_timeout(Duration::from_secs(600)).is_ok());
        assert_eq!(
            cfg()
                .with_lock_timeout(Duration::from_secs(30))
                .expect("config")
                .lock_timeout(),
            Duration::from_secs(30)
        );
    }

    #[test]
    fn timeout_rejects_outside_range() {
        assert!(matches!(
            cfg().with_lock_timeout(Duration::from_millis(999)),
            Err(ConfigError::TimeoutOutOfRange { given }) if given == Duration::from_millis(999)
        ));
        assert!(matches!(
            cfg().with_lock_timeout(Duration::from_secs(601)),
            Err(ConfigError::TimeoutOutOfRange { given }) if given == Duration::from_secs(601)
        ));
    }

    #[test]
    fn debug_redacts_dsn() {
        let texto = format!("{:?}", cfg());
        assert!(texto.contains("Dsn(<redacted>)"));
        assert!(!texto.contains("segredo"));
        assert!(texto.contains("allow_superuser: false"));
    }
}

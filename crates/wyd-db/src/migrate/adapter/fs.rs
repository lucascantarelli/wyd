//! `FsSource` — o único ponto do runner que toca o disco.

use crate::migrate::port::source::{LoadedFile, Source, SourceError};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/// Lê a série de um diretório plano.
pub struct FsSource {
    dir: PathBuf,
}

impl FsSource {
    /// Cria a fonte para o diretório informado.
    #[must_use]
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }
}

impl Source for FsSource {
    async fn load(&self) -> Result<Vec<LoadedFile>, SourceError> {
        let dir = self.dir.clone();
        let path = dir.clone();
        tokio::task::spawn_blocking(move || load_blocking(&dir))
            .await
            .map_err(|error| SourceError::Read {
                path,
                detail: format!("falha na tarefa de leitura: {error}"),
            })?
    }
}

fn load_blocking(dir: &Path) -> Result<Vec<LoadedFile>, SourceError> {
    let metadata = fs::metadata(dir).map_err(|error| match error.kind() {
        std::io::ErrorKind::NotFound => SourceError::Missing {
            path: dir.to_path_buf(),
        },
        _ => SourceError::Read {
            path: dir.to_path_buf(),
            detail: error.to_string(),
        },
    })?;
    if !metadata.is_dir() {
        return Err(SourceError::NotADirectory {
            path: dir.to_path_buf(),
        });
    }

    let read_dir = fs::read_dir(dir).map_err(|error| SourceError::Read {
        path: dir.to_path_buf(),
        detail: error.to_string(),
    })?;
    let mut paths = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|error| SourceError::Read {
            path: dir.to_path_buf(),
            detail: error.to_string(),
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| SourceError::Read {
            path: path.clone(),
            detail: error.to_string(),
        })?;
        if file_type.is_file() && path.extension() == Some(OsStr::new("sql")) {
            paths.push(path);
        }
    }
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let content = fs::read_to_string(&path).map_err(|error| SourceError::Read {
            path: path.clone(),
            detail: if error.kind() == std::io::ErrorKind::InvalidData {
                "não é UTF-8".to_owned()
            } else {
                error.to_string()
            },
        })?;
        files.push(LoadedFile { path, content });
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn fonte(dir: &Path) -> FsSource {
        FsSource::new(dir.to_path_buf())
    }

    #[tokio::test]
    async fn loads_only_sql_and_does_not_recurse() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(dir.path().join("0001_a.sql"), "SELECT 1;").expect("sql");
        fs::write(dir.path().join("README.md"), "fora").expect("md");
        fs::write(dir.path().join("0002_b.SQL"), "SELECT 2;").expect("maiúscula");
        fs::create_dir(dir.path().join("sub")).expect("sub");
        fs::write(dir.path().join("sub/0003_c.sql"), "SELECT 3;").expect("subsql");

        let files = fonte(dir.path()).load().await.expect("load");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path.file_name(), Some(OsStr::new("0001_a.sql")));
        assert_eq!(files[0].content, "SELECT 1;");
    }

    #[tokio::test]
    async fn empty_dir_is_ok() {
        let dir = tempfile::tempdir().expect("tempdir");
        assert!(fonte(dir.path()).load().await.expect("load").is_empty());
    }

    #[tokio::test]
    async fn missing_dir_and_file_are_environment_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let missing = dir.path().join("nao-existe");
        assert!(matches!(
            fonte(&missing).load().await,
            Err(SourceError::Missing { .. })
        ));
        let file = dir.path().join("arquivo");
        fs::File::create(&file).expect("arquivo");
        assert!(matches!(
            fonte(&file).load().await,
            Err(SourceError::NotADirectory { .. })
        ));
    }

    #[tokio::test]
    async fn non_utf8_is_read_error() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("0001_x.sql");
        let mut file = fs::File::create(&path).expect("arquivo");
        file.write_all(&[0xff]).expect("bytes");
        assert!(matches!(
            fonte(dir.path()).load().await,
            Err(SourceError::Read { detail, .. }) if detail == "não é UTF-8"
        ));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn unreadable_dir_is_read_error() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o000)).expect("modo");
        assert!(matches!(
            fonte(dir.path()).load().await,
            Err(SourceError::Read { .. })
        ));
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).expect("restaura");
    }
}

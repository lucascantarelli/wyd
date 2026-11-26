//! `MigrationSet` — a série inteira, ordenada e validada.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/set.md`.

use crate::migrate::migration::checksum::Checksum;
use crate::migrate::migration::file::{self, FileError, RawMigration};
use crate::migrate::migration::version::Version;
use std::fmt;

/// Um arquivo da série já parseado, com o checksum do conteúdo cru.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Conteúdo já parseado do arquivo.
    pub raw: RawMigration,
    /// Checksum do conteúdo cru, calculado uma vez.
    pub checksum: Checksum,
    /// Nome do arquivo com a extensão `.sql`.
    pub file_name: String,
}

/// Série contígua e sem duplicatas, ordenada por versão.
#[derive(Debug, Clone)]
pub struct MigrationSet {
    entries: Vec<Entry>,
}

impl MigrationSet {
    /// Recebe `(stem, conteúdo)`; valida, ordena e calcula o checksum uma vez.
    pub fn build(files: Vec<(String, String)>) -> Result<Self, SetError> {
        if files.is_empty() {
            return Err(SetError::Empty);
        }

        let mut files = files;
        files.sort_by(|a, b| a.0.cmp(&b.0));

        let mut entries = Vec::with_capacity(files.len());
        for (stem, content) in files {
            let raw = file::parse(&stem, &content).map_err(|source| SetError::File {
                file_name: format!("{stem}.sql"),
                source,
            })?;
            let checksum = Checksum::of(content.as_bytes());
            entries.push(Entry {
                raw,
                checksum,
                file_name: format!("{stem}.sql"),
            });
        }

        let mut slugs: Vec<(String, u32)> = Vec::with_capacity(entries.len());
        for (index, entry) in entries.iter().enumerate() {
            if index == 0 {
                if entry.raw.version.get() != 1 {
                    return Err(SetError::NotContiguous {
                        expected: 1,
                        found: entry.raw.version.get(),
                    });
                }
            } else {
                let previous = &entries[index - 1];
                if entry.raw.version == previous.raw.version {
                    return Err(SetError::DuplicateVersion {
                        version: entry.raw.version.get(),
                        first: previous.file_name.clone(),
                        second: entry.file_name.clone(),
                    });
                }
                let expected = previous.raw.version.get() + 1;
                if entry.raw.version.get() != expected {
                    return Err(SetError::NotContiguous {
                        expected,
                        found: entry.raw.version.get(),
                    });
                }
            }

            if let Some((_, first_version)) = slugs
                .iter()
                .find(|(slug, _)| slug == entry.raw.slug.as_str())
            {
                return Err(SetError::DuplicateSlug {
                    slug: entry.raw.slug.to_string(),
                    versions: (*first_version, entry.raw.version.get()),
                });
            }
            slugs.push((entry.raw.slug.to_string(), entry.raw.version.get()));
        }

        Ok(Self { entries })
    }

    /// Quantidade de migrations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Se a série está vazia.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Itera em ordem crescente de versão.
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    /// Busca uma versão.
    #[must_use]
    pub fn get(&self, version: Version) -> Option<&Entry> {
        self.entries
            .binary_search_by_key(&version, |entry| entry.raw.version)
            .ok()
            .map(|index| &self.entries[index])
    }

    /// Itera as versões em ordem crescente.
    pub fn versions(&self) -> impl Iterator<Item = Version> + '_ {
        self.entries.iter().map(|entry| entry.raw.version)
    }

    /// A última versão, se houver.
    #[must_use]
    pub fn last(&self) -> Option<Version> {
        self.entries.last().map(|entry| entry.raw.version)
    }
}

/// Por que uma série não foi aceita.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetError {
    /// A lista de arquivos está vazia.
    Empty,
    /// A sequência de versões tem buraco ou não começa em 1.
    NotContiguous {
        /// Próxima versão esperada.
        expected: u32,
        /// Versão encontrada no lugar.
        found: u32,
    },
    /// A mesma versão aparece em dois arquivos.
    DuplicateVersion {
        /// Versão repetida.
        version: u32,
        /// Primeiro arquivo com a versão.
        first: String,
        /// Segundo arquivo com a versão.
        second: String,
    },
    /// O mesmo slug aparece em duas versões.
    DuplicateSlug {
        /// Slug repetido.
        slug: String,
        /// Par de versões que usam o slug.
        versions: (u32, u32),
    },
    /// Um arquivo da série não passou no parser.
    File {
        /// Nome do arquivo com a extensão `.sql`.
        file_name: String,
        /// Erro devolvido pelo parser.
        source: FileError,
    },
}

impl fmt::Display for SetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("série de migrations vazia"),
            Self::NotContiguous { expected, found } => {
                write!(
                    f,
                    "série não contígua: esperava {expected}, encontrou {found}"
                )
            }
            Self::DuplicateVersion {
                version,
                first,
                second,
            } => write!(f, "versão {version} duplicada em {first} e {second}"),
            Self::DuplicateSlug { slug, versions } => write!(
                f,
                "slug {slug} duplicado nas versões {} e {}",
                versions.0, versions.1
            ),
            Self::File { file_name, source } => write!(f, "arquivo {file_name}: {source}"),
        }
    }
}

impl std::error::Error for SetError {}

#[cfg(test)]
mod tests {
    use super::*;

    const SQL: &str = "-- migrate:up\nSELECT 1;\n-- migrate:down\nSELECT 2;\n";

    #[test]
    fn builds_single_file() {
        let set = MigrationSet::build(vec![("0001_bootstrap".into(), SQL.into())]).expect("set");
        assert_eq!(set.len(), 1);
        assert_eq!(set.last(), Some(Version::new(1).expect("versão")));
        assert_eq!(
            set.get(Version::new(1).expect("versão"))
                .expect("entrada")
                .raw
                .version
                .get(),
            1
        );
    }

    #[test]
    fn sorts_regardless_of_input_order() {
        let set = MigrationSet::build(vec![
            ("0002_second".into(), SQL.into()),
            ("0001_first".into(), SQL.into()),
        ])
        .expect("set");
        let versions: Vec<_> = set.versions().map(Version::get).collect();
        assert_eq!(versions, vec![1, 2]);
    }

    #[test]
    fn rejects_empty_and_gap() {
        assert!(matches!(MigrationSet::build(vec![]), Err(SetError::Empty)));
        assert!(matches!(
            MigrationSet::build(vec![
                ("0001_first".into(), SQL.into()),
                ("0003_third".into(), SQL.into()),
            ]),
            Err(SetError::NotContiguous {
                expected: 2,
                found: 3
            })
        ));
    }

    #[test]
    fn rejects_non_one_start() {
        assert!(matches!(
            MigrationSet::build(vec![("0002_second".into(), SQL.into())]),
            Err(SetError::NotContiguous {
                expected: 1,
                found: 2
            })
        ));
    }

    #[test]
    fn rejects_duplicate_version_and_slug() {
        assert!(matches!(
            MigrationSet::build(vec![
                ("0001_same".into(), SQL.into()),
                ("0001_other".into(), SQL.into()),
            ]),
            Err(SetError::DuplicateVersion { version: 1, .. })
        ));
        match MigrationSet::build(vec![
            ("0001_same".into(), SQL.into()),
            ("0002_same".into(), SQL.into()),
        ]) {
            Err(SetError::DuplicateSlug { slug, versions }) => {
                assert_eq!(slug, "same");
                assert_eq!(versions, (1, 2));
            }
            outro => panic!("esperava DuplicateSlug; veio {outro:?}"),
        }
    }

    #[test]
    fn checksum_is_of_raw_content() {
        let content = "-- migrate:up\r\nSELECT 1;\r\n-- migrate:down\r\nSELECT 2;\r\n";
        let set = MigrationSet::build(vec![("0001_windows".into(), content.into())]).expect("set");
        assert_eq!(
            set.iter().next().expect("entrada").checksum,
            Checksum::of(content.as_bytes())
        );
    }
}

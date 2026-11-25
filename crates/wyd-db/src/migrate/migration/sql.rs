//! Texto SQL validado: não vazio, sem marcador e sem controle de transação.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/migrate/migration/sql.md`.

use std::fmt;
use std::str::FromStr;

/// Trecho já validado. Campo privado: só `FromStr` constrói.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sql(String);

impl FromStr for Sql {
    type Err = SqlTextError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.trim().is_empty() {
            return Err(SqlTextError::Empty);
        }

        let bytes = raw.as_bytes();
        let mut index = 0;
        let mut at_statement_start = true;
        let mut saw_code = false;
        while index < bytes.len() {
            match bytes[index] {
                b'-' if bytes.get(index + 1) == Some(&b'-') => {
                    index += 2;
                    while index < bytes.len() && bytes[index] != b'\n' {
                        index += 1;
                    }
                }
                b'/' if bytes.get(index + 1) == Some(&b'*') => {
                    let mut depth = 1_usize;
                    index += 2;
                    while index < bytes.len() && depth > 0 {
                        if raw[index..].starts_with("-- migrate:") {
                            return Err(SqlTextError::MarkerInSql);
                        }
                        if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
                            depth += 1;
                            index += 2;
                        } else if bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/') {
                            depth -= 1;
                            index += 2;
                        } else {
                            index += 1;
                        }
                    }
                    if depth != 0 {
                        return Err(SqlTextError::UnterminatedBlockComment);
                    }
                }
                b'\'' => {
                    saw_code = true;
                    at_statement_start = false;
                    index += 1;
                    while index < bytes.len() {
                        if bytes[index] == b'\'' {
                            if bytes.get(index + 1) == Some(&b'\'') {
                                index += 2;
                            } else {
                                index += 1;
                                break;
                            }
                        } else {
                            index += 1;
                        }
                    }
                }
                b'$' => {
                    if let Some(tag_len) = dollar_tag_len(&bytes[index..]) {
                        saw_code = true;
                        at_statement_start = false;
                        let tag = &raw[index..index + tag_len];
                        index += tag_len;
                        while index < bytes.len() {
                            if bytes[index] == b'$' && raw[index..].starts_with(tag) {
                                index += tag.len();
                                break;
                            }
                            index += 1;
                        }
                    } else {
                        saw_code = true;
                        at_statement_start = false;
                        index += 1;
                    }
                }
                b';' => {
                    saw_code = true;
                    at_statement_start = true;
                    index += 1;
                }
                byte if byte.is_ascii_whitespace() => {
                    index += 1;
                }
                _ if at_statement_start => {
                    saw_code = true;
                    let start = index;
                    while index < bytes.len()
                        && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
                    {
                        index += 1;
                    }
                    if start == index {
                        at_statement_start = false;
                        index += 1;
                        continue;
                    }
                    let palavra = &raw[start..index];
                    let lower = palavra.to_ascii_lowercase();
                    let proibida = match lower.as_str() {
                        "begin" => Some("BEGIN"),
                        "commit" => Some("COMMIT"),
                        "rollback" => Some("ROLLBACK"),
                        "savepoint" => Some("SAVEPOINT"),
                        _ => None,
                    };
                    if let Some(keyword) = proibida {
                        return Err(SqlTextError::TransactionControl { keyword });
                    }
                    at_statement_start = false;
                }
                _ => {
                    saw_code = true;
                    at_statement_start = false;
                    index += 1;
                }
            }
        }

        if !saw_code {
            return Err(SqlTextError::Empty);
        }
        Ok(Self(raw.to_owned()))
    }
}

/// Se `bytes` começa por `$tag$`, devolve o comprimento do delimitador (incluindo os `$`).
fn dollar_tag_len(bytes: &[u8]) -> Option<usize> {
    if bytes.first() != Some(&b'$') {
        return None;
    }
    let mut index = 1;
    while index < bytes.len() && bytes[index] != b'$' {
        let byte = bytes[index];
        if index == 1 {
            if !(byte.is_ascii_alphabetic() || byte == b'_') {
                return None;
            }
        } else if !(byte.is_ascii_alphanumeric() || byte == b'_') {
            return None;
        }
        index += 1;
    }
    if index < bytes.len() && bytes[index] == b'$' {
        Some(index + 1)
    } else {
        None
    }
}

impl AsRef<str> for Sql {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Sql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Por que um texto não pode ser executado como SQL de uma migration.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlTextError {
    /// Vazio ou só comentário/espaço.
    Empty,
    /// Um marcador `-- migrate:` reapareceu fora de comentário/literal.
    MarkerInSql,
    /// Controle de transação no começo de uma instrução.
    TransactionControl { keyword: &'static str },
    /// Um comentário de bloco não fechou.
    UnterminatedBlockComment,
}

impl fmt::Display for SqlTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("trecho SQL vazio"),
            Self::MarkerInSql => f.write_str("marcador `-- migrate:` dentro do SQL"),
            Self::TransactionControl { keyword } => {
                write!(f, "controle de transação `{keyword}` dentro do arquivo")
            }
            Self::UnterminatedBlockComment => f.write_str("comentário de bloco não fechado"),
        }
    }
}

impl std::error::Error for SqlTextError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plain_ddl() {
        assert!("CREATE TABLE x (id INT);".parse::<Sql>().is_ok());
    }

    #[test]
    fn accepts_plpgsql_begin_end() {
        let sql =
            "CREATE FUNCTION f() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RETURN NEW; END; $$;";
        assert!(sql.parse::<Sql>().is_ok());
    }

    #[test]
    fn accepts_set_local_and_commit_in_string() {
        assert!("SET LOCAL search_path = '';".parse::<Sql>().is_ok());
        assert!("INSERT INTO x VALUES ('COMMIT');".parse::<Sql>().is_ok());
    }

    #[test]
    fn rejects_empty_and_only_comments() {
        assert_eq!("".parse::<Sql>(), Err(SqlTextError::Empty));
        assert_eq!("-- nada".parse::<Sql>(), Err(SqlTextError::Empty));
        assert_eq!("/* nada */".parse::<Sql>(), Err(SqlTextError::Empty));
    }

    #[test]
    fn rejects_transaction_control_at_statement_start() {
        assert_eq!(
            "COMMIT;".parse::<Sql>(),
            Err(SqlTextError::TransactionControl { keyword: "COMMIT" })
        );
        assert_eq!(
            "SELECT 1; begin;".parse::<Sql>(),
            Err(SqlTextError::TransactionControl { keyword: "BEGIN" })
        );
        assert_eq!(
            "SAVEPOINT s1;".parse::<Sql>(),
            Err(SqlTextError::TransactionControl {
                keyword: "SAVEPOINT"
            })
        );
    }

    #[test]
    fn does_not_match_prefix_or_inside_dollar_quote() {
        assert!("CREATE TABLE beginning (id INT);".parse::<Sql>().is_ok());
        assert!("DO $$ BEGIN COMMIT; END $$;".parse::<Sql>().is_ok());
    }

    #[test]
    fn rejects_marker_in_block_comment_but_allows_line_and_dollar_quote() {
        assert_eq!(
            "SELECT 1 /* -- migrate:up */;".parse::<Sql>(),
            Err(SqlTextError::MarkerInSql)
        );
        assert!("SELECT 1; -- migrate:up".parse::<Sql>().is_ok());
        assert!("$$ -- migrate:up $$".parse::<Sql>().is_ok());
    }
}

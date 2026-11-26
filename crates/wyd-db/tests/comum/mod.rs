//! Harness comum dos alvos de sistema do `wyd-db`: um `psql` no container.
//!
//! O container vem de `WYD_DB_TEST_CONTAINER`; `WYD_DB_TEST_USER` e `WYD_DB_TEST_DB` são
//! opcionais, com default `postgres` e `wyd`. Sem a variável o alvo **falha**, não se ignora:
//! um teste que se auto-ignora não prova nada na CI. O `psql` daqui é o **oráculo do schema**; o
//! `sqlx` é a porta do runner (`planejamento-1.0.0/wyd-db/docs/guia-teste/index.md` §5).

use std::io::Write;
use std::process::{Command, Stdio};

/// Executa `sql` no container e devolve `(sucesso, stdout+stderr)`.
pub fn psql(sql: &str) -> (bool, String) {
    let container = std::env::var("WYD_DB_TEST_CONTAINER").unwrap_or_else(|_| {
        panic!("WYD_DB_TEST_CONTAINER não definida: o alvo de sistema precisa do container PostgreSQL 17")
    });
    let user = std::env::var("WYD_DB_TEST_USER").unwrap_or_else(|_| "postgres".to_string());
    let database = std::env::var("WYD_DB_TEST_DB").unwrap_or_else(|_| "wyd".to_string());
    let mut child = Command::new("docker")
        .args([
            "exec",
            "-i",
            &container,
            "psql",
            "-U",
            &user,
            "-d",
            &database,
            "-X",
            "-q",
            "-t",
            "-A",
            "-v",
            "ON_ERROR_STOP=1",
            "-f",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|erro| panic!("falha ao executar `docker exec`: {erro}"));
    child
        .stdin
        .as_mut()
        .expect("stdin do psql")
        .write_all(format!("\\set VERBOSITY verbose\n{sql}\n").as_bytes())
        .expect("envio do SQL ao psql");
    let saida = child.wait_with_output().expect("leitura da saída do psql");
    let mut texto = String::from_utf8_lossy(&saida.stdout).to_string();
    texto.push_str(&String::from_utf8_lossy(&saida.stderr));
    (saida.status.success(), texto)
}

//! Harness comum dos alvos de sistema do `wyd-db`: um `psql` como oráculo.
//!
//! Dois modos, na ordem de precedência:
//!
//! 1. `WYD_DB_TEST_URL` (`postgres://user:senha@host:porta/banco`) — usado no CI
//!    com service container. O `psql` roda no runner, via TCP em loopback.
//! 2. `WYD_DB_TEST_CONTAINER` — usado no desenvolvimento local; o `psql` roda
//!    dentro do container com `docker exec`. `WYD_DB_TEST_USER` e
//!    `WYD_DB_TEST_DB` são opcionais, com default `postgres` e `wyd`.
//!
//! Sem nenhum dos dois o alvo **falha**, não se ignora: um teste que se
//! auto-ignora não prova nada na CI. O `psql` daqui é o **oráculo do schema**; o
//! `sqlx` é a porta do runner (`planejamento-1.0.0/wyd-db/docs/guia-teste/index.md` §5).

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

struct Dsn {
    user: String,
    password: Option<String>,
    host: String,
    port: String,
    database: String,
}

/// Lê o subconjunto de `postgres://` que o CI usa. Não é um parser de URL
/// completo: não há `%`-decoding nem query string; senha com `@` funciona porque
/// o corte é pelo último `@`.
fn dsn(url: &str) -> Dsn {
    let sem_esquema = url
        .strip_prefix("postgres://")
        .or_else(|| url.strip_prefix("postgresql://"))
        .unwrap_or_else(|| panic!("WYD_DB_TEST_URL sem esquema postgres://: {url}"));
    let (auth, resto) = sem_esquema
        .rsplit_once('@')
        .unwrap_or_else(|| panic!("WYD_DB_TEST_URL sem host: {url}"));
    let (user, password) = match auth.split_once(':') {
        Some((user, password)) => (user.to_string(), Some(password.to_string())),
        None => (auth.to_string(), None),
    };
    let (hostport, database) = resto
        .split_once('/')
        .unwrap_or_else(|| panic!("WYD_DB_TEST_URL sem banco: {url}"));
    let (host, port) = hostport.rsplit_once(':').unwrap_or((hostport, "5432"));
    Dsn {
        user,
        password,
        host: host.to_string(),
        port: port.to_string(),
        database: database.to_string(),
    }
}

fn psql_tcp(url: &str) -> Command {
    let d = dsn(url);
    let mut cmd = Command::new("psql");
    cmd.args([
        "-h",
        &d.host,
        "-p",
        &d.port,
        "-U",
        &d.user,
        "-d",
        &d.database,
        "-X",
        "-q",
        "-t",
        "-A",
        "-v",
        "ON_ERROR_STOP=1",
        "-f",
        "-",
    ]);
    if let Some(senha) = d.password {
        cmd.env("PGPASSWORD", senha);
    }
    cmd
}

fn psql_container() -> Command {
    let container = env::var("WYD_DB_TEST_CONTAINER").unwrap_or_else(|_| {
        panic!("defina WYD_DB_TEST_URL (service container) ou WYD_DB_TEST_CONTAINER (docker exec)")
    });
    let user = env::var("WYD_DB_TEST_USER").unwrap_or_else(|_| "postgres".to_string());
    let database = env::var("WYD_DB_TEST_DB").unwrap_or_else(|_| "wyd".to_string());
    let mut cmd = Command::new("docker");
    cmd.args([
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
    ]);
    cmd
}

/// Executa `sql` no banco de teste e devolve `(sucesso, stdout+stderr)`.
pub fn psql(sql: &str) -> (bool, String) {
    let mut child = match env::var("WYD_DB_TEST_URL") {
        Ok(url) => psql_tcp(&url),
        Err(_) => psql_container(),
    }
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap_or_else(|erro| panic!("falha ao executar `psql`: {erro}"));

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

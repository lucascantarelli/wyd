//! Alvo `sistema_pg` — as capacidades `CAP-24` a `CAP-39` contra a base migrada.
//!
//! Roda cada cenário num PostgreSQL 17 já migrado, por `psql` dentro do container
//! indicado em `WYD_DB_TEST_CONTAINER` (`WYD_DB_TEST_USER` e `WYD_DB_TEST_DB` são
//! opcionais, com default `postgres` e `wyd`). O alvo ainda não usa cliente nativo:
//! a decisão de `sqlx` segue aberta (`docs/guia-desenvolvedor/mapa-do-crate.md`), e
//! este harness prova o **schema**, não a porta.
//!
//! **Bloqueados, não pulados:** `CAP-37` (`RNF-DB-002`, exige o `MigrateConfig` do
//! runner), `CAP-38` (`RNF-DB-004`, exige a consulta da porta C4) e `CAP-39`
//! (`RNF-DB-005`, exige o boot do binário) dependem de código que não existe. Eles
//! não estão nesta suíte e ficam declarados em `tests/specs/capacidades.md` §9.

use std::io::Write;
use std::process::{Command, Stdio};

const HASH: &str = "$argon2id$v=19$m=8,t=1,p=1$c2FsdGQ$Zml4dHVyZQ";

fn psql(sql: &str) -> (bool, String) {
    let container = std::env::var("WYD_DB_TEST_CONTAINER").unwrap_or_else(|_| {
        panic!("WYD_DB_TEST_CONTAINER não definida: o alvo sistema_pg precisa do container PostgreSQL 17 migrado")
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

/// Exige sucesso e devolve a saída (usada para `SELECT` de uma coluna).
fn ok(sql: &str) -> String {
    let (sucesso, texto) = psql(sql);
    assert!(sucesso, "esperava sucesso; veio:\n{texto}");
    texto.trim().to_string()
}

/// Exige falha com o SQLSTATE e o nome do objeto (constraint, índice, FK).
fn erro(sql: &str, codigo: &str, objeto: &str) {
    let (sucesso, texto) = psql(sql);
    assert!(
        !sucesso,
        "esperava {codigo} em {objeto}, mas a instrução passou"
    );
    assert!(
        texto.contains(codigo),
        "esperava SQLSTATE {codigo}; veio:\n{texto}"
    );
    assert!(
        texto.contains(objeto),
        "esperava o objeto {objeto}; veio:\n{texto}"
    );
}

fn valor(sql: &str) -> String {
    ok(sql)
}

/// Cria (ou reaproveita) a pessoa de teste `tag@test.local`.
fn pessoa(tag: &str) {
    ok(&format!(
        r#"
DO $$ BEGIN
  INSERT INTO portal.person (email, password_hash)
    VALUES ('{tag}@test.local', '{HASH}');
EXCEPTION WHEN unique_violation THEN NULL; END $$;
"#
    ));
}

/// Cria (ou reaproveita) a conta `login` na vaga `slot` da pessoa `tag`.
fn conta(tag: &str, slot: i32, login: &str) {
    ok(&format!(
        r#"
DO $$ BEGIN
  INSERT INTO game.account (person_id, slot, login, password_hash)
    SELECT id, {slot}, '{login}', '{HASH}'
      FROM portal.person WHERE email_fold = '{tag}@test.local';
EXCEPTION WHEN unique_violation THEN NULL; END $$;
"#
    ));
}

fn conta_id(login: &str) -> String {
    format!("(SELECT id FROM game.account WHERE login_fold = '{login}')")
}

fn pessoa_id(tag: &str) -> String {
    format!("(SELECT id FROM portal.person WHERE email_fold = '{tag}@test.local')")
}

#[test]
fn cap24_login_tamanho_e_fold() {
    pessoa("cap24");
    conta("cap24", 1, "cap24");
    let p = pessoa_id("cap24");
    erro(
        &format!(
            "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({p}, 2, repeat('a', 17), '{HASH}')"
        ),
        "23514",
        "ck_account_login_len",
    );
    erro(
        &format!(
            "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({p}, 2, ' Bob ', '{HASH}')"
        ),
        "23514",
        "ck_account_login_forma",
    );
    ok("DELETE FROM game.account WHERE login_fold = 'bob'");
    ok(&format!(
        "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({p}, 2, 'Bob', '{HASH}')"
    ));
    assert_eq!(
        valor("SELECT login_fold FROM game.account WHERE login_fold = 'bob'"),
        "bob"
    );
}

#[test]
fn cap25_pin_falha_persistida() {
    pessoa("cap25");
    conta("cap25", 1, "cap25");
    ok("UPDATE game.account SET pin_failure_count = 5, pin_locked_until = now() + interval '10 minutes' WHERE login_fold = 'cap25'");
    assert_eq!(
        valor("SELECT pin_failure_count::text || '|' || (pin_locked_until IS NOT NULL)::text FROM game.account WHERE login_fold = 'cap25'"),
        "5|true"
    );
}

#[test]
fn cap26_cofre_da_conta() {
    pessoa("cap26");
    conta("cap26", 1, "cap26");
    erro(
        "INSERT INTO game.vault_item (account_id, slot, item_index) VALUES (999999999, 0, 1)",
        "23503",
        "fk_vault_item_account",
    );
    ok("INSERT INTO content.content_version (id, version) VALUES (1, 7) ON CONFLICT (id) DO NOTHING");
    let conta = conta_id("cap26");
    ok(&format!(
        "DELETE FROM game.vault_item WHERE account_id = {conta}"
    ));
    ok(&format!(
        "DELETE FROM game.character WHERE account_id = {conta}"
    ));
    ok("INSERT INTO game.character (account_id, slot, name, content_version) SELECT id, 0, 'Cap26Hero', 1 FROM game.account WHERE login_fold = 'cap26'");
    ok("INSERT INTO game.vault_item (account_id, slot, item_index) SELECT id, 0, 1 FROM game.account WHERE login_fold = 'cap26'");
    ok(&format!(
        "DELETE FROM game.character WHERE account_id = {conta}"
    ));
    assert_eq!(
        valor(&format!(
            "SELECT count(*) FROM game.vault_item WHERE account_id = {conta}"
        )),
        "1"
    );
}

#[test]
fn cap27_bloqueio_com_motivo() {
    pessoa("cap27");
    conta("cap27", 1, "cap27");
    let conta = conta_id("cap27");
    let p = pessoa_id("cap27");
    ok(&format!(
        "DELETE FROM game.account_moderation WHERE account_id = {conta}"
    ));
    erro(
        &format!(
            "INSERT INTO game.account_moderation (account_id, blocked, reason, blocked_by, blocked_at) VALUES ({conta}, true, '', {p}, now())"
        ),
        "23514",
        "ck_account_moderation_motivo",
    );
    erro(
        &format!(
            "INSERT INTO game.account_moderation (account_id, blocked, reason, blocked_by, blocked_at) VALUES ({conta}, true, 'disciplina', NULL, now())"
        ),
        "23514",
        "ck_account_moderation_autor",
    );
    erro(
        &format!(
            "INSERT INTO game.account_moderation (account_id, blocked, reason, blocked_by, blocked_at) VALUES ({conta}, true, 'disciplina', {p}, NULL)"
        ),
        "23514",
        "ck_account_moderation_instante",
    );
}

#[test]
fn cap28_donate_idempotente() {
    pessoa("cap28");
    conta("cap28", 1, "cap28");
    let chave = "'11111111-1111-1111-1111-111111111111'";
    ok(&format!(
        "DELETE FROM portal.donate_credit WHERE idempotency_key = {chave}"
    ));
    let conta = conta_id("cap28");
    ok(&format!(
        "INSERT INTO portal.donate_credit (idempotency_key, account_id, credits) VALUES ({chave}, {conta}, 100)"
    ));
    erro(
        &format!(
            "INSERT INTO portal.donate_credit (idempotency_key, account_id, credits) VALUES ({chave}, {conta}, 100)"
        ),
        "23505",
        "uq_donate_credit_idem",
    );
    erro(
        &format!(
            "INSERT INTO portal.donate_credit (idempotency_key, account_id, credits) VALUES ('22222222-2222-2222-2222-222222222222', {conta}, 0)"
        ),
        "23514",
        "ck_donate_credit_valor",
    );
}

#[test]
fn cap29_token_como_hash() {
    pessoa("cap29");
    conta("cap29", 1, "cap29");
    let hex = "a".repeat(64);
    let p = pessoa_id("cap29");
    let conta = conta_id("cap29");
    ok(&format!(
        "DELETE FROM portal.person_token WHERE token_hash = '{hex}'"
    ));
    ok(&format!(
        "INSERT INTO portal.person_token (person_id, purpose, token_hash, expires_at) VALUES ({p}, 'password_reset', '{hex}', now() + interval '1 hour')"
    ));
    erro(
        &format!(
            "INSERT INTO portal.person_token (person_id, purpose, token_hash, expires_at) VALUES ({p}, 'password_reset', 'XYZ', now() + interval '1 hour')"
        ),
        "23514",
        "ck_person_token_hash_forma",
    );
    ok(&format!(
        "DELETE FROM game.pin_reset WHERE token_hash = '{hex}'"
    ));
    ok(&format!(
        "INSERT INTO game.pin_reset (account_id, token_hash, expires_at) VALUES ({conta}, '{hex}', now() + interval '1 hour')"
    ));
    erro(
        &format!(
            "INSERT INTO game.pin_reset (account_id, token_hash, expires_at) VALUES ({conta}, 'XYZ', now() + interval '1 hour')"
        ),
        "23514",
        "ck_pin_reset_token_forma",
    );
}

#[test]
fn cap30_papel_de_jogo_fechado() {
    pessoa("cap30");
    conta("cap30", 1, "cap30");
    erro(
        "UPDATE game.account SET role = 'gm' WHERE login_fold = 'cap30'",
        "23514",
        "ck_account_role",
    );
    ok("UPDATE game.account SET role = 'player' WHERE login_fold = 'cap30'");
}

#[test]
fn cap31_hash_argon2id() {
    erro(
        "INSERT INTO portal.person (email, password_hash) VALUES ('cap31-bad@test.local', 'texto')",
        "23514",
        "ck_person_senha_hasheada",
    );
    pessoa("cap31");
    conta("cap31", 1, "cap31");
    let p = pessoa_id("cap31");
    erro(
        &format!(
            "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({p}, 2, 'cap31bad', 'texto')"
        ),
        "23514",
        "ck_account_senha_hasheada",
    );
    ok("DELETE FROM portal.person WHERE email_fold = 'cap31-ok@test.local'");
    ok(&format!(
        "INSERT INTO portal.person (email, password_hash) VALUES ('cap31-ok@test.local', '{HASH}')"
    ));
    assert_eq!(
        valor("SELECT password_hash LIKE '$argon2id$%' FROM portal.person WHERE email_fold = 'cap31-ok@test.local'"),
        "t"
    );
}

#[test]
fn cap32_bloqueio_nao_muda_status() {
    pessoa("cap32");
    conta("cap32", 1, "cap32");
    let conta = conta_id("cap32");
    let p = pessoa_id("cap32");
    ok(&format!(
        "DELETE FROM game.account_moderation WHERE account_id = {conta}"
    ));
    ok(&format!(
        "INSERT INTO game.account_moderation (account_id, blocked, reason, blocked_by, blocked_at) VALUES ({conta}, true, 'disciplina', {p}, now())"
    ));
    assert_eq!(
        valor("SELECT status FROM game.account WHERE login_fold = 'cap32'"),
        "active"
    );
    erro(
        "UPDATE game.account SET status = 'suspended' WHERE login_fold = 'cap32'",
        "23514",
        "ck_account_status",
    );
}

#[test]
fn cap33_limiar_de_pin_e_politica() {
    pessoa("cap33");
    conta("cap33", 1, "cap33");
    erro(
        "UPDATE game.account SET pin_failure_count = -1 WHERE login_fold = 'cap33'",
        "23514",
        "ck_account_pin_falhas",
    );
    ok("UPDATE game.account SET pin_failure_count = 6, pin_locked_until = now() + interval '10 minutes' WHERE login_fold = 'cap33'");
    assert_eq!(
        valor("SELECT pin_failure_count FROM game.account WHERE login_fold = 'cap33'"),
        "6"
    );
}

#[test]
fn cap34_autenticar_portal() {
    pessoa("cap34");
    let p = pessoa_id("cap34");
    let t1 = "b".repeat(64);
    let t2 = "c".repeat(64);
    ok(&format!(
        "DELETE FROM portal.person_session WHERE token_hash IN ('{t1}', '{t2}')"
    ));
    ok(&format!(
        "INSERT INTO portal.person_session (person_id, token_hash, expires_at) VALUES ({p}, '{t1}', now() + interval '8 hours')"
    ));
    ok(&format!(
        "INSERT INTO portal.person_session (person_id, token_hash, expires_at) VALUES ({p}, '{t2}', now() + interval '8 hours')"
    ));
    ok(&format!(
        "UPDATE portal.person_session SET revoked_at = now() WHERE token_hash = '{t1}'"
    ));
    assert_eq!(
        valor(&format!(
            "SELECT count(*) FILTER (WHERE revoked_at IS NULL)::text || '|' || count(*)::text FROM portal.person_session WHERE person_id = {p}"
        )),
        "1|2"
    );
    let idh = "d".repeat(64);
    ok(&format!(
        "DELETE FROM ops.login_attempt WHERE identifier_hash = '{idh}'"
    ));
    ok(&format!(
        "INSERT INTO ops.login_attempt (surface, identifier_hash, result) VALUES ('portal', '{idh}', 'bad_secret')"
    ));
    assert_eq!(
        valor(&format!(
            "SELECT count(*) FROM ops.login_attempt WHERE identifier_hash = '{idh}' AND result = 'bad_secret'"
        )),
        "1"
    );
}

#[test]
fn cap35_criar_conta_na_vaga() {
    pessoa("cap35");
    pessoa("cap35b");
    let p = pessoa_id("cap35");
    ok(&format!(
        "DELETE FROM game.account_wallet WHERE account_id IN (SELECT id FROM game.account WHERE person_id = {p})"
    ));
    ok(&format!("DELETE FROM game.account WHERE person_id = {p}"));
    ok(&format!(
        "DELETE FROM game.account WHERE person_id = {}",
        pessoa_id("cap35b")
    ));
    for (vaga, login) in [
        (1, "cap35a"),
        (2, "cap35b1"),
        (3, "cap35c"),
        (4, "cap35d"),
        (5, "cap35e"),
    ] {
        ok(&format!(
            "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({p}, {vaga}, '{login}', '{HASH}')"
        ));
    }
    ok("INSERT INTO game.account_wallet (account_id, coin) SELECT id, 0 FROM game.account WHERE login_fold = 'cap35a'");
    assert_eq!(
        valor(&format!(
            "SELECT count(*) FROM game.account WHERE person_id = {p}"
        )),
        "5"
    );
    assert_eq!(
        valor(&format!(
            "SELECT count(*) FROM game.account_wallet WHERE account_id = {}",
            conta_id("cap35a")
        )),
        "1"
    );
    erro(
        &format!(
            "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({p}, 1, 'cap35sexta', '{HASH}')"
        ),
        "23505",
        "ux_account_slot",
    );
    ok(&format!(
        "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({}, 1, 'cap35livre', '{HASH}')",
        pessoa_id("cap35b")
    ));
    erro(
        &format!(
            "INSERT INTO game.account (person_id, slot, login, password_hash) VALUES ({}, 2, 'cap35a', '{HASH}')",
            pessoa_id("cap35b")
        ),
        "23505",
        "ux_account_login",
    );
}

#[test]
fn cap36_abrir_sessao_de_jogo() {
    pessoa("cap36");
    pessoa("cap36b");
    conta("cap36", 1, "cap36");
    conta("cap36b", 1, "cap36b");
    ok("INSERT INTO game.server_channel (group_id, channel, host, max_players, maintenance, role) VALUES (1, 1, '127.0.0.1', 500, false, 'normal') ON CONFLICT DO NOTHING");
    let c = conta_id("cap36");
    ok(&format!("DELETE FROM game.session WHERE account_id = {c}"));
    ok("INSERT INTO game.session (account_id, group_id, channel, client_version) SELECT id, 1, 1, 7662 FROM game.account WHERE login_fold = 'cap36'");
    assert_eq!(
        valor(&format!(
            "SELECT character_id IS NULL FROM game.session WHERE account_id = {c} AND released_at IS NULL"
        )),
        "t"
    );
    erro(
        "INSERT INTO game.session (account_id, group_id, channel, client_version) SELECT id, 9, 9, 7662 FROM game.account WHERE login_fold = 'cap36b'",
        "23503",
        "fk_session_canal",
    );
}

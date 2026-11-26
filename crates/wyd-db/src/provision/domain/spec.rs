//! Estado desejado dos papéis e do schema `public`.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/src/provision/spec.md` §2 e §4.
//! Nada aqui lê ambiente, arquivo ou relógio.

use std::fmt::Write as _;

/// Um papel de cluster, com a membership **direta** que ele precisa ter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleSpec {
    /// Nome do papel, sempre prefixado por `wyd_`.
    pub name: &'static str,
    /// `true` emite `LOGIN`; `false`, `NOLOGIN`.
    pub login: bool,
    /// Membership direta, um nível, sem repetição.
    pub member_of: &'static [&'static str],
}

/// Os seis papéis da série 1.0.0, na ordem de emissão do SQL.
pub const ROLES: &[RoleSpec] = &[
    RoleSpec {
        name: "wyd_owner",
        login: false,
        member_of: &[],
    },
    RoleSpec {
        name: "wyd_migrator",
        login: true,
        member_of: &["wyd_owner"],
    },
    RoleSpec {
        name: "wyd_game",
        login: true,
        member_of: &[],
    },
    RoleSpec {
        name: "wyd_portal",
        login: true,
        member_of: &[],
    },
    RoleSpec {
        name: "wyd_admin",
        login: true,
        member_of: &[],
    },
    RoleSpec {
        name: "wyd_read",
        login: true,
        member_of: &[],
    },
];

/// Nome do papel que possui os schemas.
pub const OWNER: &str = "wyd_owner";

/// Nome do papel que aplica a série — o único aceito pelo preflight do runner.
pub const MIGRATOR: &str = "wyd_migrator";

const _: () = assert!(ROLES.len() == 6);

/// Membership direta esperada, como pares `(dono, membro)`.
pub fn memberships() -> impl Iterator<Item = (&'static str, &'static str)> {
    ROLES
        .iter()
        .flat_map(|role| role.member_of.iter().map(move |owner| (*owner, role.name)))
}

/// Renderiza o conteúdo completo de `db/roles.sql`. Determinístico: mesma
/// entrada, mesmo `String`.
#[must_use]
pub fn render() -> String {
    let mut out = String::new();
    out.push_str("-- db/roles.sql — GERADO por `wyd-db provision render`. Não editar à mão.\n");
    out.push_str("-- Papéis do cluster e o schema `public` deste banco: uma vez por banco, reexecutar é no-op. Não é migration.\n");
    out.push_str("DO $$\nBEGIN\n");
    for role in ROLES {
        let _ = writeln!(
            out,
            "  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = '{}') THEN",
            role.name
        );
        let attribute = if role.login { "LOGIN" } else { "NOLOGIN" };
        let _ = writeln!(out, "    CREATE ROLE {} {};", role.name, attribute);
        out.push_str("  END IF;\n");
    }
    out.push_str("END $$;\n\n");
    out.push_str("-- membership separada: `IN ROLE` só vale na criação do papel\n");
    for (owner, member) in memberships() {
        let _ = writeln!(out, "GRANT {owner} TO {member};");
    }
    out.push_str("\n-- `public` existe com o banco, antes da série, e é onde o runner cria o ledger: o estado dele é de\n");
    out.push_str("-- ambiente, não de migration. `PUBLIC` não escreve em `public`; `wyd_migrator` atravessa e cria ali.\n");
    out.push_str("REVOKE CREATE ON SCHEMA public FROM PUBLIC;\n");
    out.push_str("GRANT USAGE, CREATE ON SCHEMA public TO wyd_migrator;\n");
    out
}

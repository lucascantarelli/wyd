//! Alvo `contrato_api_provision` — congela a fachada pública de `provision`.
//!
//! Escopo (`WYDDB-003`): `Command`, `Report` e `run<B>`, mais as regras de
//! `render` (texto puro), `plan`/`check` (sem `Script`) e `apply` (um script).
//! O adapter PostgreSQL, o golden e o CLI ficam fora desta issue.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use wyd_db::provision::domain::observed::ObservedRole;
use wyd_db::provision::port::{Catalog, CatalogError, Script, ScriptError};
use wyd_db::provision::{render, run, Command, Report};

#[derive(Default)]
struct Fake {
    catalog_calls: AtomicUsize,
    script_calls: AtomicUsize,
    script: Mutex<Option<String>>,
}

impl Catalog for Fake {
    async fn observed(&self) -> Result<Vec<ObservedRole>, CatalogError> {
        self.catalog_calls.fetch_add(1, Ordering::SeqCst);
        Ok(Vec::new())
    }
}

impl Script for Fake {
    async fn apply(&self, sql: &str) -> Result<(), ScriptError> {
        self.script_calls.fetch_add(1, Ordering::SeqCst);
        *self.script.lock().expect("mutex") = Some(sql.to_owned());
        Ok(())
    }
}

impl Fake {
    fn catalog_calls(&self) -> usize {
        self.catalog_calls.load(Ordering::SeqCst)
    }

    fn script_calls(&self) -> usize {
        self.script_calls.load(Ordering::SeqCst)
    }

    fn sent_script(&self) -> Option<String> {
        self.script.lock().expect("mutex").clone()
    }
}

#[tokio::test]
async fn render_devolve_texto_sem_tocar_o_backend() {
    let fake = Fake::default();
    let report = run(Command::Render, &fake).await.expect("render");

    let Report::Rendered(sql) = report else {
        panic!("Render devolveu outra variante de Report");
    };
    assert!(sql.starts_with("-- db/roles.sql"));
    assert!(sql.contains("CREATE ROLE wyd_owner NOLOGIN;"));
    assert!(sql.contains("CREATE ROLE wyd_migrator LOGIN;"));
    assert!(sql.contains("GRANT wyd_owner TO wyd_migrator;"));
    assert!(sql.contains("REVOKE CREATE ON SCHEMA public FROM PUBLIC;"));
    assert!(!sql.contains("PASSWORD"));
    assert!(!sql.contains('\r'));
    assert_eq!(sql, render());
    assert_eq!(fake.catalog_calls(), 0);
    assert_eq!(fake.script_calls(), 0);
}

#[tokio::test]
async fn plan_e_check_leem_o_catalogo_e_nunca_o_script() {
    let fake = Fake::default();

    let report = run(Command::Plan, &fake).await.expect("plan");
    let Report::Planned(plano) = report else {
        panic!("Plan devolveu outra variante de Report");
    };
    assert_eq!(plano.steps.len(), 7);
    assert!(plano.drifts.is_empty());

    let report = run(Command::Check, &fake).await.expect("check");
    assert!(matches!(report, Report::Planned(_)));

    assert_eq!(fake.catalog_calls(), 2);
    assert_eq!(fake.script_calls(), 0);
}

#[tokio::test]
async fn apply_entrega_o_texto_de_render_uma_vez() {
    let fake = Fake::default();
    let report = run(Command::Apply, &fake).await.expect("apply");

    assert!(matches!(report, Report::Applied { steps: 7 }));
    assert_eq!(fake.catalog_calls(), 1);
    assert_eq!(fake.script_calls(), 1);
    assert_eq!(fake.sent_script().as_deref(), Some(render().as_str()));
}

//! Alvo `reversibilidade_pg` — o ciclo `up → down → up` da série, em PostgreSQL 17.
//!
//! Contrato: `planejamento-1.0.0/wyd-db/migrations/serie-e-regras.md` §1.3, §6 critério 8 e §9
//! (`Q-SERIE-1 = B`, decidida em 2026-11-25), com `ADR-DB-0002` mantida. Este alvo é a prova que
//! faltava: nenhuma migration da série depende de reversão manual não executada.
//!
//! A separação dos blocos é **textual**, não por `file::parse`. O alvo prova o **SQL** da série, não o
//! parser: os nomes já parseiam desde `Q-SEP-1` = A′ (`serie-e-regras.md` §9.1), e a porta do runner é
//! provada por `runner_pg`. Separação textual também mantém este alvo independente de `Slug`.
//!
//! O container vem de `WYD_DB_TEST_CONTAINER` e precisa estar **vazio**, com os papéis de
//! `db/roles.sql` criados. O ciclo é: aplica os 28 `up`; grava dado real em duas tabelas; aplica os
//! 28 `down` em ordem decrescente; confere que os quatro schemas sumiram; reaplica os 28 `up` e
//! compara o inventário de objetos com o do primeiro `up`, que tem de ser idêntico.

#[path = "../comum/mod.rs"]
mod comum;

use std::path::{Path, PathBuf};
use wyd_db::migrate::migration::file::{parse, FileError};

const ESQUEMAS: &str = "'content', 'game', 'portal', 'ops'";

const INVENTARIO: &str = r#"
SELECT coalesce(string_agg(linha, chr(10) ORDER BY linha), '')
FROM (
    SELECT format('table %s.%s', n.nspname, c.relname) AS linha
      FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname IN (SCHEMAS) AND c.relkind IN ('r', 'p')
    UNION ALL
    SELECT format('view %s.%s', n.nspname, c.relname)
      FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname IN (SCHEMAS) AND c.relkind = 'v'
    UNION ALL
    SELECT format('sequence %s.%s', n.nspname, c.relname)
      FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname IN (SCHEMAS) AND c.relkind = 'S'
    UNION ALL
    SELECT format('index %s.%s', n.nspname, c.relname)
      FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname IN (SCHEMAS) AND c.relkind IN ('i', 'I')
    UNION ALL
    SELECT format('function %s.%s', n.nspname, p.proname)
      FROM pg_proc p JOIN pg_namespace n ON n.oid = p.pronamespace
     WHERE n.nspname IN (SCHEMAS)
    UNION ALL
    SELECT format('trigger %s.%s.%s', n.nspname, c.relname, t.tgname)
      FROM pg_trigger t
      JOIN pg_class c ON c.oid = t.tgrelid
      JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname IN (SCHEMAS) AND NOT t.tgisinternal
    UNION ALL
    SELECT format('constraint %s.%s.%s', n.nspname, c.relname, con.conname)
      FROM pg_constraint con
      JOIN pg_class c ON c.oid = con.conrelid
      JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname IN (SCHEMAS)
) AS objetos
"#;

/// Um arquivo da série com os dois blocos já separados.
struct Arquivo {
    nome: String,
    versao: u32,
    up: String,
    down: String,
}

fn raiz() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

/// Lê a série materializada, em ordem de nome (= ordem de versão numérica de 4 dígitos).
fn serie() -> Vec<Arquivo> {
    let mut caminhos: Vec<PathBuf> = std::fs::read_dir(raiz())
        .expect("diretório de migrations")
        .map(|entrada| entrada.expect("entrada do diretório").path())
        .filter(|caminho| {
            caminho
                .extension()
                .is_some_and(|extensao| extensao == "sql")
        })
        .collect();
    caminhos.sort();
    caminhos.into_iter().map(ler).collect()
}

fn ler(caminho: PathBuf) -> Arquivo {
    let nome = caminho
        .file_name()
        .and_then(|nome| nome.to_str())
        .expect("nome do arquivo")
        .to_owned();
    let conteudo = std::fs::read_to_string(&caminho)
        .unwrap_or_else(|erro| panic!("leitura de {nome}: {erro}"))
        .replace("\r\n", "\n");
    assert_eq!(
        conteudo.matches("-- migrate:up").count(),
        1,
        "{nome} precisa de exatamente um `-- migrate:up`"
    );
    assert_eq!(
        conteudo.matches("-- migrate:down").count(),
        1,
        "{nome} precisa de exatamente um `-- migrate:down`"
    );
    let (_, apos_up) = conteudo
        .split_once("-- migrate:up")
        .unwrap_or_else(|| panic!("{nome} não tem `-- migrate:up`"));
    let (up, down) = apos_up
        .split_once("-- migrate:down")
        .unwrap_or_else(|| panic!("{nome} não tem `-- migrate:down`"));
    let versao = nome
        .get(..4)
        .and_then(|prefixo| prefixo.parse::<u32>().ok())
        .unwrap_or_else(|| panic!("{nome} não começa com 4 dígitos"));
    Arquivo {
        nome,
        versao,
        up: up.to_owned(),
        down: down.to_owned(),
    }
}

fn ok(sql: &str) -> String {
    let (sucesso, texto) = comum::psql(sql);
    assert!(sucesso, "esperava sucesso; veio:\n{texto}");
    texto.trim().to_string()
}

/// Aplica um dos blocos em ordem de série (`reverso` usa a ordem decrescente do `down`).
fn aplicar(arquivos: &[&Arquivo], usar_up: bool) {
    for arquivo in arquivos {
        let (parte, sql) = if usar_up {
            ("up", &arquivo.up)
        } else {
            ("down", &arquivo.down)
        };
        let (sucesso, texto) = comum::psql(sql);
        assert!(
            sucesso,
            "o `{parte}` de {} falhou; veio:\n{texto}\n--- SQL ---\n{sql}",
            arquivo.nome
        );
    }
}

#[test]
fn todo_arquivo_da_serie_tem_up_e_down_nao_vazios() {
    let arquivos = serie();
    assert!(!arquivos.is_empty(), "nenhum arquivo em `migrations/`");
    for arquivo in &arquivos {
        assert!(
            !arquivo.up.trim().is_empty(),
            "{} tem `up` vazio",
            arquivo.nome
        );
        assert!(
            !arquivo.down.trim().is_empty(),
            "{} tem `down` vazio",
            arquivo.nome
        );
    }
    let mut versoes: Vec<u32> = arquivos.iter().map(|arquivo| arquivo.versao).collect();
    versoes.sort_unstable();
    versoes.dedup();
    assert_eq!(
        versoes.len(),
        arquivos.len(),
        "duas migrations compartilham a mesma versão"
    );
}

/// Mede a forma do nome: `_` no slug funciona, hífen no slug não (`serie-e-regras.md` §9.1).
///
/// O separador `-` é aceito, mas o que vem depois dele é validado como `Slug` (`[a-z0-9_]`); por
/// isso `0001-bootstrap` passa e `0002-ops-audit` não. A decisão entre renomear a série ou aceitar
/// `-` no `Slug` é do dono — este teste fixa o comportamento atual para a mudança ser notada.
#[test]
fn parser_aceita_underscore_e_recusa_hifen_no_slug() {
    const CORPO: &str = "-- migrate:up\nSELECT 1;\n-- migrate:down\nSELECT 1;\n";
    let com_underscore = parse("0035_novo_arquivo", CORPO).expect("slug com `_` deveria parsear");
    assert_eq!(com_underscore.slug.as_str(), "novo_arquivo");
    assert!(
        parse("0001-bootstrap", CORPO).is_ok(),
        "slug de uma palavra deveria parsear"
    );
    assert!(
        matches!(
            parse("0035-novo-arquivo", CORPO),
            Err(FileError::BadSlug(_))
        ),
        "`Slug` recusa `-`; se isto passar a `Ok`, a decisão de §9.1 mudou"
    );
}

#[test]
fn ciclo_up_down_up_em_pg17() {
    let arquivos = serie();
    let ordem: Vec<&Arquivo> = arquivos.iter().collect();
    let reverso: Vec<&Arquivo> = arquivos.iter().rev().collect();

    let inicial = ok(&format!(
        "SELECT count(*) FROM pg_namespace WHERE nspname IN ({ESQUEMAS})"
    ));
    assert_eq!(
        inicial, "0",
        "o container precisa começar vazio para o ciclo provar o estado final"
    );

    aplicar(&ordem, true);
    let depois_do_primeiro_up = ok(&INVENTARIO.replace("SCHEMAS", ESQUEMAS));
    assert!(
        depois_do_primeiro_up.lines().count() > 100,
        "inventário pequeno demais para ser a série: {depois_do_primeiro_up}"
    );

    // Dado real antes da reversão: prova que o `down` não depende de tabela vazia.
    ok(r#"
INSERT INTO portal.person (email, password_hash)
    VALUES ('reversibilidade@test.local',
            '$argon2id$v=19$m=8,t=1,p=1$c2FsdGQ$Zml4dHVyZQ');
INSERT INTO content.game_system (slug, name) VALUES ('reversibilidade', 'Reversibilidade');
"#);

    aplicar(&reverso, false);

    let restantes = ok(&format!(
        "SELECT count(*) FROM pg_namespace WHERE nspname IN ({ESQUEMAS})"
    ));
    assert_eq!(
        restantes, "0",
        "o `down` de 0001 tem de deixar os quatro schemas ausentes (§6, critério 8)"
    );

    aplicar(&ordem, true);
    let depois_do_segundo_up = ok(&INVENTARIO.replace("SCHEMAS", ESQUEMAS));
    assert_eq!(
        depois_do_segundo_up, depois_do_primeiro_up,
        "o segundo `up` tem de reproduzir o schema do primeiro"
    );
}

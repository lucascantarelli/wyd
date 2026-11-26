//! O JSON Schema publicado descreve o manifesto que o launcher aceita.
//!
//! O teste lê o schema de `planejamento-1.0.0/wyd-launcher/docs/specs/manifest.schema.json`,
//! valida com ele os manifestos reais/fixture e prova que uma divergência reprova. O validador é
//! mínimo de propósito: cobre só as palavras-chave que o schema usa (`type`, `const`, `enum`,
//! `pattern`, `minLength`, `minItems`, `minimum`, `maximum`, `required`, `properties`,
//! `additionalProperties`, `items`, `allOf`, `not`) — se o schema usar outra, o teste não passa por
//! acidente.

use std::path::{Path, PathBuf};

use regex::Regex;
use serde_json::Value;

fn raiz() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn ler(relativo: &str) -> Value {
    let caminho = raiz().join(relativo);
    let texto = std::fs::read_to_string(&caminho)
        .unwrap_or_else(|erro| panic!("{}: {erro}", caminho.display()));
    serde_json::from_str(&texto).unwrap_or_else(|erro| panic!("{}: {erro}", caminho.display()))
}

fn tipo_ok(tipo: &str, valor: &Value) -> bool {
    match tipo {
        "object" => valor.is_object(),
        "array" => valor.is_array(),
        "string" => valor.is_string(),
        "boolean" => valor.is_boolean(),
        "null" => valor.is_null(),
        "integer" => valor.as_i64().is_some() || valor.as_u64().is_some(),
        "number" => valor.is_number(),
        _ => false,
    }
}

fn validar(schema: &Value, doc: &Value, caminho: &str, erros: &mut Vec<String>) {
    if let Some(tipo) = schema.get("type").and_then(Value::as_str) {
        if !tipo_ok(tipo, doc) {
            erros.push(format!("{caminho}: esperado {tipo}, veio {}", tipo_de(doc)));
            return;
        }
    }
    if let Some(constante) = schema.get("const") {
        if constante != doc {
            erros.push(format!("{caminho}: esperado const {constante}"));
        }
    }
    if let Some(enumero) = schema.get("enum").and_then(Value::as_array) {
        if !enumero.contains(doc) {
            erros.push(format!("{caminho}: fora do enum"));
        }
    }
    if let (Some(padrao), Some(texto)) =
        (schema.get("pattern").and_then(Value::as_str), doc.as_str())
    {
        if !Regex::new(padrao).expect("regex do schema").is_match(texto) {
            erros.push(format!("{caminho}: não casa com {padrao}"));
        }
    }
    if let (Some(minimo), Some(texto)) = (
        schema.get("minLength").and_then(Value::as_u64),
        doc.as_str(),
    ) {
        if (texto.chars().count() as u64) < minimo {
            erros.push(format!("{caminho}: menor que minLength {minimo}"));
        }
    }
    if let (Some(numero), Some(schema_minimo)) =
        (doc.as_f64(), schema.get("minimum").and_then(Value::as_f64))
    {
        if numero < schema_minimo {
            erros.push(format!("{caminho}: abaixo de minimum {schema_minimo}"));
        }
    }
    if let (Some(numero), Some(schema_maximo)) =
        (doc.as_f64(), schema.get("maximum").and_then(Value::as_f64))
    {
        if numero > schema_maximo {
            erros.push(format!("{caminho}: acima de maximum {schema_maximo}"));
        }
    }
    if let (Some(itens), Some(lista)) = (schema.get("items"), doc.as_array()) {
        for (indice, item) in lista.iter().enumerate() {
            validar(itens, item, &format!("{caminho}[{indice}]"), erros);
        }
    }
    if let (Some(minimo), Some(lista)) = (
        schema.get("minItems").and_then(Value::as_u64),
        doc.as_array(),
    ) {
        if (lista.len() as u64) < minimo {
            erros.push(format!("{caminho}: menos itens que minItems {minimo}"));
        }
    }
    if let Some(objeto) = doc.as_object() {
        if let Some(propriedades) = schema.get("properties").and_then(Value::as_object) {
            if let Some(obrigatorios) = schema.get("required").and_then(Value::as_array) {
                for nome in obrigatorios.iter().filter_map(Value::as_str) {
                    if !objeto.contains_key(nome) {
                        erros.push(format!("{caminho}: falta `{nome}`"));
                    }
                }
            }
            for (nome, valor) in objeto {
                match propriedades.get(nome) {
                    Some(schema_filho) => {
                        validar(schema_filho, valor, &format!("{caminho}.{nome}"), erros)
                    }
                    None => {
                        if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
                            erros.push(format!("{caminho}: campo extra `{nome}`"));
                        }
                    }
                }
            }
        }
    }
    if let Some(lista) = schema.get("allOf").and_then(Value::as_array) {
        for (indice, filho) in lista.iter().enumerate() {
            validar(filho, doc, &format!("{caminho}<allOf:{indice}>"), erros);
        }
    }
    if let Some(negado) = schema.get("not") {
        let mut internos = Vec::new();
        validar(negado, doc, caminho, &mut internos);
        if internos.is_empty() {
            erros.push(format!("{caminho}: casou com `not`"));
        }
    }
}

fn tipo_de(valor: &Value) -> &'static str {
    match valor {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn erros(schema: &Value, doc: &Value) -> Vec<String> {
    let mut saida = Vec::new();
    validar(schema, doc, "$", &mut saida);
    saida
}

const SCHEMA: &str = "planejamento-1.0.0/wyd-launcher/docs/specs/manifest.schema.json";
const FIXTURE: &str = "crates/wyd-launcher/tests/fixtures/assinatura/manifest.json";
const MEDIDO: &str = "planejamento-1.0.0/wyd-launcher/docs/specs/patch-1.0.0.json";

#[test]
fn schema_aceita_manifesto_da_fixture_de_assinatura() {
    assert_eq!(erros(&ler(SCHEMA), &ler(FIXTURE)), Vec::<String>::new());
}

#[test]
fn schema_aceita_manifesto_medido_do_patch() {
    assert_eq!(erros(&ler(SCHEMA), &ler(MEDIDO)), Vec::<String>::new());
}

#[test]
fn schema_tem_as_chaves_obrigatorias_do_codigo() {
    let schema = ler(SCHEMA);
    let obrigatorios: Vec<&str> = schema["required"]
        .as_array()
        .expect("required")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert_eq!(
        obrigatorios,
        vec![
            "schema",
            "channel",
            "version",
            "client_build",
            "min_version",
            "files"
        ]
    );
    let arquivo = &schema["properties"]["files"]["items"];
    let campos: Vec<&str> = arquivo["required"]
        .as_array()
        .expect("required do arquivo")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert_eq!(campos, vec!["path", "size", "sha256"]);
    assert_eq!(arquivo["additionalProperties"], Value::Bool(false));
}

#[test]
fn reprova_manifesto_com_campo_extra() {
    let mut doc = ler(FIXTURE);
    doc["extra"] = Value::from(1);
    assert!(!erros(&ler(SCHEMA), &doc).is_empty());
}

#[test]
fn reprova_manifesto_sem_campo_obrigatorio() {
    let mut doc = ler(FIXTURE);
    doc.as_object_mut().unwrap().remove("min_version");
    assert!(!erros(&ler(SCHEMA), &doc).is_empty());
}

#[test]
fn reprova_schema_ou_canal_fora_do_contrato() {
    let schema = ler(SCHEMA);
    let mut doc = ler(FIXTURE);
    doc["schema"] = Value::from(2);
    assert!(!erros(&schema, &doc).is_empty());
    let mut doc = ler(FIXTURE);
    doc["channel"] = Value::from("nightly");
    assert!(!erros(&schema, &doc).is_empty());
}

#[test]
fn reprova_hash_e_versao_fora_do_formato() {
    let schema = ler(SCHEMA);
    let mut doc = ler(FIXTURE);
    doc["client_build"] = Value::from("A".repeat(64));
    assert!(!erros(&schema, &doc).is_empty());
    let mut doc = ler(FIXTURE);
    doc["version"] = Value::from("1.0");
    assert!(!erros(&schema, &doc).is_empty());
}

#[test]
fn reprova_caminho_que_escapa_da_raiz() {
    let schema = ler(SCHEMA);
    for caminho in [
        "../fora.bin",
        "/absoluto.bin",
        "a\\b.bin",
        "C:/x.bin",
        "a//b.bin",
    ] {
        let mut doc = ler(FIXTURE);
        doc["files"][0]["path"] = Value::from(caminho);
        assert!(!erros(&schema, &doc).is_empty(), "aceitou {caminho}");
    }
}

#[test]
fn reprova_arquivo_sem_sha256_ou_com_url_insegura() {
    let schema = ler(SCHEMA);
    let mut doc = ler(FIXTURE);
    doc["files"][0].as_object_mut().unwrap().remove("sha256");
    assert!(!erros(&schema, &doc).is_empty());
    let mut doc = ler(FIXTURE);
    doc["files"][0]["url"] = Value::from("http://x");
    assert!(!erros(&schema, &doc).is_empty());
    let mut doc = ler(FIXTURE);
    doc["files"] = Value::Array(Vec::new());
    assert!(!erros(&schema, &doc).is_empty());
}

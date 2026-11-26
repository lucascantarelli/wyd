//! Vetor de teste da RFC 8785 §3.2.2.4 (exemplo normativo do JCS).
//!
//! O texto de entrada é o do RFC, com escapes JSON; a saída esperada é a forma canônica que o
//! próprio RFC publica. Este é o teste que impede a canonização de divergir do padrão.

use wyd_launcher::canon::{canonicalize_str, CanonError};

const ENTRADA: &str = concat!(
    r#"{"numbers":[333333333.33333329,1E30,4.50,2e-3,0.000000000000000000000000001],"#,
    r#""string":"\u20ac$\u000F\u000aA'\u0042\u0022\u005c\\\"\/","#,
    r#""literals":[null,true,false]}"#,
);

const ESPERADO: &str = concat!(
    r#"{"literals":[null,true,false],"#,
    r#""numbers":[333333333.3333333,1e+30,4.5,0.002,1e-27],"#,
    r#""string":"€$\u000f\nA'B\"\\\\\"/"}"#,
);

#[test]
fn vetor_do_rfc_8785() {
    assert_eq!(canonicalize_str(ENTRADA).expect("canonizar"), ESPERADO);
}

#[test]
fn ordem_dos_campos_nao_muda_os_bytes_canonicos() {
    // Mesmos valores, campos em outra ordem e `2E-3` em vez de `2e-3`.
    let reordenado = concat!(
        r#"{"string":"\u20ac$\u000F\u000aA'\u0042\u0022\u005c\\\"\/","#,
        r#""literals":[null,true,false],"#,
        r#""numbers":[333333333.33333329,1E30,4.5,2E-3,0.000000000000000000000000001]}"#,
    );
    assert_eq!(canonicalize_str(reordenado).unwrap(), ESPERADO);
}

#[test]
fn recusa_json_invalido() {
    assert!(matches!(
        canonicalize_str("{não é json}"),
        Err(CanonError::Json(_))
    ));
}

//! Canonização JSON conforme a RFC 8785 (JCS) — a forma sobre a qual a assinatura é feita.
//!
//! O `ADR-LCH-0001` fixa RFC 8785: UTF-8 sem BOM, sem espaço insignificante, chaves ordenadas por
//! unidade de código UTF-16, números na forma do `Number::toString` do ECMAScript e sem quebra de
//! linha final. Este módulo implementa só a transformação — quem assina e quem verifica entram com
//! `LLCH-03`.
//!
//! Limites declarados: o `serde_json` descarta chave repetida na leitura (a última vence), então
//! documento com chave duplicada não é recusado aqui; número que não cabe em `f64` exato é recusado
//! em vez de arredondado.

use serde_json::{Map, Value};

/// Falha de canonização — sempre por documento fora do domínio da RFC 8785.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CanonError {
    #[error("JSON inválido: {0}")]
    Json(String),
    #[error("número fora da precisão de f64: {0}")]
    Number(String),
    #[error("número não finito")]
    NotFinite,
}

/// Canoniza um texto JSON já UTF-8. Não valida o schema do manifesto.
pub fn canonicalize_str(texto: &str) -> Result<String, CanonError> {
    let valor: Value =
        serde_json::from_str(texto).map_err(|erro| CanonError::Json(erro.to_string()))?;
    canonicalize(&valor)
}

/// Canoniza um valor JSON. A saída é o texto que a assinatura cobre.
pub fn canonicalize(valor: &Value) -> Result<String, CanonError> {
    let mut saida = String::new();
    escrever(valor, &mut saida)?;
    Ok(saida)
}

fn escrever(valor: &Value, saida: &mut String) -> Result<(), CanonError> {
    match valor {
        Value::Null => saida.push_str("null"),
        Value::Bool(true) => saida.push_str("true"),
        Value::Bool(false) => saida.push_str("false"),
        Value::Number(numero) => saida.push_str(&formatar_numero(numero)?),
        Value::String(texto) => escrever_string(texto, saida),
        Value::Array(itens) => {
            saida.push('[');
            for (indice, item) in itens.iter().enumerate() {
                if indice > 0 {
                    saida.push(',');
                }
                escrever(item, saida)?;
            }
            saida.push(']');
        }
        Value::Object(mapa) => escrever_objeto(mapa, saida)?,
    }
    Ok(())
}

fn escrever_objeto(mapa: &Map<String, Value>, saida: &mut String) -> Result<(), CanonError> {
    let mut chaves: Vec<&String> = mapa.keys().collect();
    // RFC 8785 §3.2.3: ordem por unidade de código UTF-16, não por `char` nem por byte.
    chaves.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
    saida.push('{');
    for (indice, chave) in chaves.iter().enumerate() {
        if indice > 0 {
            saida.push(',');
        }
        escrever_string(chave, saida);
        saida.push(':');
        escrever(&mapa[*chave], saida)?;
    }
    saida.push('}');
    Ok(())
}

/// Escapa só o que a RFC 8785 §3.2.2.2 manda: `"`, `\` e controle abaixo de `0x20`.
fn escrever_string(texto: &str, saida: &mut String) {
    saida.push('"');
    for caractere in texto.chars() {
        match caractere {
            '"' => saida.push_str("\\\""),
            '\\' => saida.push_str("\\\\"),
            '\u{8}' => saida.push_str("\\b"),
            '\u{c}' => saida.push_str("\\f"),
            '\n' => saida.push_str("\\n"),
            '\r' => saida.push_str("\\r"),
            '\t' => saida.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write as _;
                let _ = write!(saida, "\\u{:04x}", c as u32);
            }
            c => saida.push(c),
        }
    }
    saida.push('"');
}

/// Inteiro exato vira dígitos; o resto vai para a forma do ECMAScript.
fn formatar_numero(numero: &serde_json::Number) -> Result<String, CanonError> {
    const LIMITE: u64 = 9_007_199_254_740_992; // 2^53
    if let Some(inteiro) = numero.as_u64() {
        if inteiro <= LIMITE {
            return Ok(inteiro.to_string());
        }
        return Err(CanonError::Number(inteiro.to_string()));
    }
    if let Some(inteiro) = numero.as_i64() {
        if inteiro.unsigned_abs() <= LIMITE {
            return Ok(inteiro.to_string());
        }
        return Err(CanonError::Number(inteiro.to_string()));
    }
    let real = numero.as_f64().ok_or(CanonError::NotFinite)?;
    formatar_f64(real)
}

/// `Number::toString` da ECMAScript: menor representação que volta ao mesmo `f64`, com expoente
/// só fora da faixa `1e-7 .. 1e21`.
fn formatar_f64(valor: f64) -> Result<String, CanonError> {
    if !valor.is_finite() {
        return Err(CanonError::NotFinite);
    }
    if valor == 0.0 {
        return Ok("0".to_string());
    }
    let negativo = valor < 0.0;
    // `{}` do Rust já é a menor representação que volta ao mesmo `f64`; só falta convertê-la
    // para a escolha de notação da ECMAScript.
    let textual = format!("{}", valor.abs());
    let (digitos, n) = if let Some((mantissa, expoente)) = textual.split_once('e') {
        let expoente: i32 = expoente
            .parse()
            .map_err(|_| CanonError::Number(textual.clone()))?;
        let digitos: String = mantissa.chars().filter(|c| *c != '.').collect();
        (digitos, expoente + 1)
    } else {
        let (inteira, fracao) = textual.split_once('.').unwrap_or((&textual, ""));
        let mut digitos = format!("{inteira}{fracao}");
        let mut n = inteira.len() as i32;
        while digitos.starts_with('0') {
            digitos.remove(0);
            n -= 1;
        }
        while digitos.ends_with('0') {
            digitos.pop();
        }
        (digitos, n)
    };
    let k = digitos.len() as i32;

    let mut saida = String::new();
    if negativo {
        saida.push('-');
    }
    if k <= n && n <= 21 {
        saida.push_str(&digitos);
        for _ in 0..(n - k) {
            saida.push('0');
        }
    } else if 0 < n && n <= 21 {
        saida.push_str(&digitos[..n as usize]);
        saida.push('.');
        saida.push_str(&digitos[n as usize..]);
    } else if -6 < n && n <= 0 {
        saida.push_str("0.");
        for _ in 0..(-n) {
            saida.push('0');
        }
        saida.push_str(&digitos);
    } else {
        saida.push_str(&digitos[..1]);
        if k > 1 {
            saida.push('.');
            saida.push_str(&digitos[1..]);
        }
        saida.push('e');
        let expoente_final = n - 1;
        if expoente_final >= 0 {
            saida.push('+');
        }
        saida.push_str(&expoente_final.to_string());
    }
    Ok(saida)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ordena_chaves_por_unidade_utf16() {
        let valor = json!({"b": 1, "a": 2, "A": 3});
        assert_eq!(canonicalize(&valor).unwrap(), r#"{"A":3,"a":2,"b":1}"#);
    }

    #[test]
    fn ordena_suplementar_antes_de_uffff() {
        // U+10000 é 0xD800 0xDC00 em UTF-16 e por isso vem antes de U+FFFF (0xFFFF).
        let valor = json!({"\u{ffff}": 1, "\u{10000}": 2});
        assert_eq!(
            canonicalize(&valor).unwrap(),
            "{\"\u{10000}\":2,\"\u{ffff}\":1}"
        );
    }

    #[test]
    fn escapa_controle_e_mantem_unicode() {
        let valor = json!({"s": "a\u{1f}b\t€"});
        assert_eq!(canonicalize(&valor).unwrap(), r#"{"s":"a\u001fb\t€"}"#);
    }

    #[test]
    fn formata_numeros_como_a_ecmascript() {
        let casos = [
            (json!(1.0), "1"),
            (json!(0.1), "0.1"),
            (json!(-0.0), "0"),
            (json!(1e21), "1e+21"),
            (json!(1e-7), "1e-7"),
            (json!(1e300), "1e+300"),
            (json!(1e-300), "1e-300"),
            (json!(1500.0), "1500"),
            (json!(0.000001), "0.000001"),
            (json!(9007199254740992_u64), "9007199254740992"),
        ];
        for (valor, esperado) in casos {
            assert_eq!(canonicalize(&valor).unwrap(), esperado, "{valor}");
        }
    }

    #[test]
    fn recusa_inteiro_acima_de_2_53() {
        let valor = json!(9007199254740993_u64);
        assert!(matches!(canonicalize(&valor), Err(CanonError::Number(_))));
    }

    #[test]
    fn documento_reindentado_tem_a_mesma_forma() {
        let compacto = r#"{"schema":1,"channel":"stable"}"#;
        let indentado = "{\n  \"channel\": \"stable\",\n  \"schema\": 1\n}\n";
        assert_eq!(
            canonicalize_str(compacto).unwrap(),
            canonicalize_str(indentado).unwrap()
        );
    }

    #[test]
    fn array_vazio_e_objeto_vazio() {
        assert_eq!(canonicalize(&json!([])).unwrap(), "[]");
        assert_eq!(canonicalize(&json!({})).unwrap(), "{}");
    }
}

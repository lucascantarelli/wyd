//! Anel de log do processo: as linhas estruturadas também ficam em memória.
//!
//! Serve para provar o que **não** pode aparecer no log (PIN, senha): o teste lê o anel e
//! falha se o segredo estiver lá. Também evita que o teste dependa de captura de `stdout`
//! do harness.

use std::collections::VecDeque;
use std::sync::Mutex;

/// Quantas linhas ficam retidas (suficiente para uma sessão de teste inteira).
const CAPACIDADE: usize = 512;

static ANEL: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// Registra a linha que também vai para o `stdout`.
pub fn record(linha: &str) {
    let mut anel = ANEL.lock().unwrap_or_else(|e| e.into_inner());
    if anel.len() == CAPACIDADE {
        anel.pop_front();
    }
    anel.push_back(linha.to_string());
}

/// Últimas `n` linhas (mais antigas primeiro).
#[must_use]
pub fn tail(n: usize) -> Vec<String> {
    let anel = ANEL.lock().unwrap_or_else(|e| e.into_inner());
    let inicio = anel.len().saturating_sub(n);
    anel.iter().skip(inicio).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anel_guarda_as_ultimas_linhas_na_ordem() {
        let marca = "teste-anel";
        for linha in 0..(CAPACIDADE + 88) {
            record(&format!("{marca} {linha}"));
        }
        // O anel é global: outras threads podem registrar linhas junto com estas,
        // então o que se afirma é o descarte do começo, não um total exato.
        let nossas: Vec<String> = tail(CAPACIDADE)
            .into_iter()
            .filter(|linha| linha.starts_with(marca))
            .collect();
        assert!(
            nossas.len() <= CAPACIDADE,
            "o anel nunca guarda mais que a capacidade"
        );
        assert!(
            nossas.len() >= CAPACIDADE - 8,
            "fora do anel só resta a janela recente: {}",
            nossas.len()
        );
        let primeira: u32 = nossas[0]
            .rsplit(' ')
            .next()
            .expect("linha com índice")
            .parse()
            .expect("índice numérico");
        assert_eq!(
            primeira,
            600 - u32::try_from(nossas.len()).expect("cabe em u32"),
            "as linhas antigas saem do começo, sem buracos"
        );
        assert_eq!(nossas.last().map(String::as_str), Some("teste-anel 599"));
    }

    #[test]
    fn tail_menor_que_o_anel_pega_as_mais_recentes() {
        let marca = "teste-tail";
        for linha in 0..3 {
            record(&format!("{marca} {linha}"));
        }
        // O anel é global: outras threads podem registrar linhas no meio, então
        // filtra as nossas antes de comparar a ordem.
        let ultimas: Vec<String> = tail(CAPACIDADE)
            .into_iter()
            .filter(|linha| linha.starts_with(marca))
            .collect();
        assert_eq!(
            ultimas,
            vec![
                format!("{marca} 0"),
                format!("{marca} 1"),
                format!("{marca} 2")
            ]
        );
        assert!(tail(0).is_empty(), "zero linhas é vazio");
    }
}

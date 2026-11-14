//! Variáveis de ambiente do processo: binds e defaults.
//!
//! Nenhum default silencioso de regra de jogo mora aqui — só endereço de escuta.

/// CPSock do canal (`:8281`).
pub(crate) const DEFAULT_CPSOCK: &str = "0.0.0.0:8281";
/// Status HTTP (`/serv00.htm` e `/health`).
pub(crate) const DEFAULT_STATUS: &str = "0.0.0.0:8080";
/// Sonda UDP de status, na mesma porta do CPSock.
pub(crate) const DEFAULT_UDP_STATUS: &str = "0.0.0.0:8281";
/// `/metrics` do processo.
pub(crate) const DEFAULT_METRICS: &str = "0.0.0.0:9898";
/// Alvo local do `--healthcheck`.
pub(crate) const DEFAULT_HEALTHCHECK: &str = "127.0.0.1:8080";

/// Lê um endereço de bind, com o default do processo.
pub(crate) fn bind(var: &str, default: &str) -> Result<String, String> {
    Ok(std::env::var(var).unwrap_or_else(|_| default.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_usa_o_default_sem_variavel() {
        let var = "WYD_TEST_BIND_SEM_VARIAVEL";
        std::env::remove_var(var);
        assert_eq!(bind(var, DEFAULT_CPSOCK), Ok(DEFAULT_CPSOCK.to_string()));
    }

    #[test]
    fn bind_prefere_a_variavel_do_ambiente() {
        let var = "WYD_TEST_BIND_COM_VARIAVEL";
        std::env::set_var(var, "127.0.0.1:9999");
        assert_eq!(bind(var, DEFAULT_CPSOCK), Ok("127.0.0.1:9999".to_string()));
        std::env::remove_var(var);
        assert_eq!(
            bind(var, DEFAULT_HEALTHCHECK),
            Ok(DEFAULT_HEALTHCHECK.to_string()),
            "remover a variável volta ao default"
        );
    }
}

//! Senha numérica (PIN): política, cadastro, troca e bloqueio persistido.
//!
//! O bloqueio vive em `accounts` (`pin_failures`/`pin_locked_until`): reiniciar o
//! canal não devolve tentativas ao atacante. PIN nunca vai para log nem para
//! resposta de erro.

use wyd_admin::auth::hash_secret;
use wyd_db::{Accounts, AuditTrail, NewAuditEntry, PgBackend};

use super::{
    log, mapear_db, AuthError, PinOutcome, LOCKOUT, MAX_PIN_DIGITS, MAX_PIN_FAILURES,
    MIN_PIN_DIGITS,
};

/// A conta já tem PIN numérico cadastrado?
pub(crate) fn registered(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
) -> Result<bool, AuthError> {
    let atual = runtime
        .block_on(backend.pin_hash(login))
        .map_err(mapear_db)?;
    Ok(atual.is_some())
}

/// Confere o PIN existente ou cadastra o primeiro.
pub(crate) fn secure(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
    token: &[u8],
) -> Result<PinOutcome, AuthError> {
    let candidatos = candidatos(token)?;
    check_lockout(backend, runtime, login)?;
    let atual = runtime
        .block_on(backend.pin_hash(login))
        .map_err(mapear_db)?;
    match atual {
        Some(hash) => {
            if candidatos
                .iter()
                .any(|pin| wyd_admin::verify_password(pin, &hash))
            {
                reset(backend, runtime, login)?;
                Ok(PinOutcome::Verified)
            } else {
                Err(registrar_falha(backend, runtime, login))
            }
        }
        None => {
            // Cadastro: o valor digitado é o PIN. Na duplicação do Alterar vale a forma
            // encurtada (último candidato), nunca o blob truncado em 6 posições.
            let pin = valor_digitado(token)?;
            let hash = hash_secret(&pin).map_err(|_| AuthError::PinFormat)?;
            runtime
                .block_on(backend.set_pin_hash(login, &hash))
                .map_err(|error| AuthError::Db(error.to_string()))?;
            reset(backend, runtime, login)?;
            auditar(
                backend,
                runtime,
                login,
                "accounts.pin_registered",
                "segunda senha cadastrada no cliente",
            );
            Ok(PinOutcome::Registered)
        }
    }
}

/// Grava um PIN novo. A borda **só** chama depois de `secure` aceitar o PIN atual.
pub(crate) fn change(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
    novo: &[u8],
) -> Result<PinOutcome, AuthError> {
    // O passo 2 chega no **mesmo layout** do `0x0FDE` que o cliente manda: o Alterar escreve o
    // valor duas vezes no campo de 6 bytes (`864286` para `8642`). O que vale é o valor
    // resolvido, não o blob — senão a troca gravava `864286` e o jogador não entrava com `8642`.
    let pin = valor_digitado(novo)?;
    let hash = hash_secret(&pin).map_err(|_| AuthError::PinFormat)?;
    runtime
        .block_on(backend.set_pin_hash(login, &hash))
        .map_err(|error| AuthError::Db(error.to_string()))?;
    reset(backend, runtime, login)?;
    auditar(
        backend,
        runtime,
        login,
        "accounts.pin_changed",
        "segunda senha trocada no cliente",
    );
    Ok(PinOutcome::Changed)
}

/// Valores a conferir para o campo `numeric_token[6]` do `0x0FDE`.
///
/// O botão **Alterar** do cliente 7662 escreve o PIN digitado **duas vezes** no campo e o
/// protocolo corta em 6 posições (medido no proxy em 2026-11-15: `8642` → `864286`). O
/// primeiro candidato é o valor literal; quando a cauda é exatamente o começo do campo —
/// ou seja, a duplicação é consistente — o prefixo encurtado também é aceito, porque é o
/// que o jogador realmente digitou. Um PIN que não confere continua falhando e contando
/// em `pin_failures`: o prefixo só vale quando casa com o hash gravado.
pub(crate) fn candidatos(token: &[u8]) -> Result<Vec<String>, AuthError> {
    let pin = normalizar(token)?;
    let mut lista = vec![pin.clone()];
    if pin.len() == MAX_PIN_DIGITS {
        let bytes = pin.as_bytes();
        for corte in (MIN_PIN_DIGITS..bytes.len()).rev() {
            let cauda = &bytes[corte..];
            if cauda == &bytes[..cauda.len()] {
                lista.push(String::from_utf8_lossy(&bytes[..corte]).to_string());
            }
        }
    }
    Ok(lista)
}

/// O que o jogador digitou, resolvido a partir do campo `numeric_token[6]`.
///
/// Usado no **cadastro** e na **troca**: os dois passos recebem o mesmo layout de 32 B, e o
/// último candidato de [`candidatos`] é o valor encurtado do Alterar (`864286` → `8642`).
pub(crate) fn valor_digitado(token: &[u8]) -> Result<String, AuthError> {
    candidatos(token)?.pop().ok_or(AuthError::PinFormat)
}

/// PIN do fio: até 6 bytes, terminação NUL, só dígitos, 4..=6 posições.
pub(crate) fn normalizar(token: &[u8]) -> Result<String, AuthError> {
    let bytes = wyd_protocol::nul_trimmed(token);
    if bytes.len() < MIN_PIN_DIGITS || bytes.len() > MAX_PIN_DIGITS {
        return Err(AuthError::PinFormat);
    }
    if !bytes.iter().all(u8::is_ascii_digit) {
        return Err(AuthError::PinFormat);
    }
    Ok(String::from_utf8_lossy(bytes).to_string())
}

fn check_lockout(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
) -> Result<(), AuthError> {
    let (_, segundos) = runtime
        .block_on(backend.pin_guard(login))
        .map_err(mapear_db)?;
    if segundos > 0 {
        return Err(AuthError::Locked {
            segundos: u64::try_from(segundos).unwrap_or(u64::MAX),
        });
    }
    Ok(())
}

fn registrar_falha(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
) -> AuthError {
    let limite = i64::from(MAX_PIN_FAILURES);
    let segundos = i64::try_from(LOCKOUT.as_secs()).unwrap_or(i64::MAX);
    match runtime.block_on(backend.pin_failure(login, limite, segundos)) {
        Ok((_, restante)) if restante > 0 => AuthError::Locked {
            segundos: u64::try_from(restante).unwrap_or(u64::MAX),
        },
        Ok(_) => AuthError::BadPin,
        Err(error) => AuthError::Db(error.to_string()),
    }
}

/// Trilha append-only do PIN mexido **no cliente** (cadastro e troca).
///
/// O ator é a própria conta (`id:role`) e o alvo `account:<id>`, o mesmo formato do reset pelo
/// portal; a linha do reset (`accounts.pin_reset*`) continua sendo escrita pelo gateway. Falha
/// de trilha não derruba um PIN já gravado — mas não some em silêncio: sai como
/// `event=audit result=lost`.
fn auditar(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
    acao: &str,
    detalhe: &str,
) {
    let conta = match runtime.block_on(backend.find_by_login(login)) {
        Ok(conta) => conta,
        Err(error) => {
            log(
                "warn",
                "audit",
                &format!("result=lost action={acao} account={login} reason={error}"),
            );
            return;
        }
    };
    let entrada = NewAuditEntry {
        actor_id: conta.id,
        actor_role: conta.role,
        action: acao.to_string(),
        target: format!("account:{}", conta.id),
        detail: detalhe.to_string(),
    };
    if let Err(error) = runtime.block_on(backend.record(&entrada)) {
        log(
            "warn",
            "audit",
            &format!(
                "result=lost action={acao} account={} reason={error}",
                conta.id
            ),
        );
    }
}

fn reset(
    backend: &PgBackend,
    runtime: &tokio::runtime::Runtime,
    login: &str,
) -> Result<(), AuthError> {
    runtime
        .block_on(backend.pin_success(login))
        .map_err(|error| AuthError::Db(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_exige_digitos_e_faixa() {
        assert_eq!(normalizar(b"1234\0\0"), Ok("1234".to_string()));
        assert_eq!(normalizar(b"123456"), Ok("123456".to_string()));
        assert_eq!(normalizar(b"123\0\0\0"), Err(AuthError::PinFormat));
        assert_eq!(normalizar(b"12a4\0\0"), Err(AuthError::PinFormat));
        assert_eq!(normalizar(b"\0\0\0\0\0\0"), Err(AuthError::PinFormat));
    }

    #[test]
    fn cadastro_e_troca_usam_o_valor_resolvido_do_campo() {
        // Corpo do Alterar capturado no proxy: o campo de 6 B leva o valor duas vezes.
        assert_eq!(valor_digitado(b"432143"), Ok("4321".into()));
        assert_eq!(valor_digitado(b"864286"), Ok("8642".into()));
        // PIN de 6 dígitos cabe inteiro no campo: nada de cortar.
        assert_eq!(valor_digitado(b"123456"), Ok("123456".into()));
        // Campo vazio continua fora da política (não vira PIN vazio).
        assert_eq!(valor_digitado(b"\0\0\0\0\0\0"), Err(AuthError::PinFormat));
    }

    #[test]
    fn alterar_do_cliente_duplica_o_campo_e_o_prefixo_vale() {
        // Medido no cliente 7662: o botão Alterar manda o valor duas vezes, cortado em 6.
        assert_eq!(
            candidatos(b"864286"),
            Ok(vec!["864286".into(), "8642".into()])
        );
        // PIN de 6 dígitos cabe inteiro: não há duplicação consistente e um só valor vale.
        assert_eq!(candidatos(b"123456"), Ok(vec!["123456".into()]));
        // Nada de aceitar qualquer 6 dígitos como prefixo: só a repetição exata conta.
        assert_eq!(candidatos(b"864295"), Ok(vec!["864295".into()]));
        assert!(candidatos(b"123\0\0\0").is_err());
    }
}

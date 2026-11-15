//! Autoatendimento do reset da segunda senha: link de uso único por e-mail.
//!
//! O staff já tem `account_admin/reset_numeric_pin`; aqui é o mesmo efeito (apagar o PIN
//! e zerar falhas) provado por outro fator: o jogador recebe um link no e-mail cadastrado.
//! O portal é quem gera o token (aleatório), manda o e-mail e entrega ao gateway apenas o
//! **hash** — o gateway nunca vê o token em claro, então nem log nem banco o guardam.
//!
//! Regras que não podem quebrar:
//!
//! - o `account_id` vem da sessão assinada do portal e tem de casar com o corpo;
//! - token expirado, já usado ou desconhecido responde a mesma recusa (sem revelar qual);
//! - toda emissão e todo consumo vão para a trilha com o autor da sessão;
//! - o efeito é o mesmo do reset por staff: `clear_pin_hash` (PIN e falhas).

use wyd_db::{Accounts, PinReset};

use crate::audit::{log_panel, AuditSink};
use crate::auth::Session;
use crate::error::AdminError;

/// Validade do link: curta de propósito — o jogador pede outro se expirar.
pub const TOKEN_TTL_SECONDS: i64 = 900;
/// Teto do e-mail (RFC 5321) e o mesmo da constraint do banco.
pub const EMAIL_MAX: usize = 254;
/// `sha256` em hex: 64 caracteres minúsculos.
const TOKEN_HASH_LEN: usize = 64;

/// Sessão obrigatória; quem injeta o `account_id` é o BFF.
///
/// Mesma fronteira de `panel/self` e `panel/history`: o portal autentica o jogador no
/// cookie assinado e chama daqui do loopback com o token de processo — o browser nunca
/// escolhe o `account_id`. O `session` presente é o que mantém a rota fechada para
/// chamada anônima.
fn require(session: Option<Session>, account_id: i64) -> Result<Session, AdminError> {
    let session = session.ok_or(AdminError::Unauthorized)?;
    if account_id <= 0 {
        return Err(AdminError::InvalidInput {
            reason: "conta inválida",
        });
    }
    Ok(session)
}

/// E-mail plausível: um `@`, algo antes e depois, ponto no domínio, sem espaço.
///
/// A validação de verdade é a entrega; aqui só se recusa o que nunca seria endereço válido.
#[must_use]
pub fn valid_email(email: &str) -> bool {
    if email.len() > EMAIL_MAX || email.contains(char::is_whitespace) {
        return false;
    }
    let mut partes = email.split('@');
    let (Some(usuario), Some(dominio), None) = (partes.next(), partes.next(), partes.next()) else {
        return false;
    };
    if usuario.is_empty() || dominio.is_empty() || !dominio.contains('.') {
        return false;
    }
    let dominio = dominio.trim_matches('.');
    !dominio.is_empty() && !dominio.starts_with('.') && !dominio.ends_with('.')
}

/// Hash do token no formato que o portal manda: 64 hex minúsculos.
#[must_use]
pub fn valid_token_hash(hash: &str) -> bool {
    hash.len() == TOKEN_HASH_LEN
        && hash
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Cadastra o e-mail da conta (`panel/email`).
///
/// Sem e-mail não há autoatendimento: é o canal de prova do reset.
pub async fn set_email(
    session: Option<Session>,
    account_id: i64,
    email: &str,
    repo: &dyn PinReset,
    sink: &impl AuditSink,
) -> Result<String, AdminError> {
    let session = require(session, account_id)?;
    let email = email.trim().to_lowercase();
    if !valid_email(&email) {
        return Err(AdminError::InvalidInput {
            reason: "e-mail inválido",
        });
    }
    repo.set_email(account_id, &email).await?;
    log_panel(
        sink,
        session,
        "panel.email",
        &format!("account:{account_id}"),
        "e-mail de recuperação atualizado",
    );
    Ok(email)
}

/// Emite o token e devolve o e-mail para onde o link deve ir.
///
/// O portal chama isto, envia a mensagem e só então responde ao jogador. E-mail ausente
/// é `422` com instrução — não há como mandar link para endereço nenhum.
pub async fn start_pin_reset(
    session: Option<Session>,
    account_id: i64,
    token_hash: &str,
    repo: &dyn PinReset,
    sink: &impl AuditSink,
) -> Result<String, AdminError> {
    let session = require(session, account_id)?;
    if !valid_token_hash(token_hash) {
        return Err(AdminError::InvalidInput {
            reason: "token fora do formato",
        });
    }
    let Some(email) = repo.email(account_id).await? else {
        return Err(AdminError::InvalidInput {
            reason: "cadastre um e-mail na conta antes de pedir o link",
        });
    };
    repo.issue_token(account_id, token_hash, &email, TOKEN_TTL_SECONDS)
        .await?;
    log_panel(
        sink,
        session,
        "accounts.pin_reset_request",
        &format!("account:{account_id}"),
        "link de reset da segunda senha enviado por e-mail",
    );
    Ok(email)
}

/// Consome o token e apaga o PIN da conta (mesmo efeito do reset por staff).
///
/// Recusa única para token inválido/expirado/usado: quem tem o link não descobre qual
/// dos três casos é, e o log do canal nunca recebe o token.
pub async fn confirm_pin_reset(
    session: Option<Session>,
    account_id: i64,
    token_hash: &str,
    accounts: &impl Accounts,
    repo: &dyn PinReset,
    sink: &impl AuditSink,
) -> Result<(), AdminError> {
    let session = require(session, account_id)?;
    if !valid_token_hash(token_hash) {
        return Err(AdminError::InvalidInput {
            reason: "link inválido ou expirado",
        });
    }
    if !repo.consume_token(account_id, token_hash).await? {
        return Err(AdminError::InvalidInput {
            reason: "link inválido ou expirado",
        });
    }
    let account = accounts.find_by_id(account_id).await?;
    accounts.clear_pin_hash(&account.login).await?;
    log_panel(
        sink,
        session,
        "accounts.pin_reset_self",
        &format!("account:{account_id}"),
        "segunda senha apagada pelo link de e-mail",
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::VecSink;
    use std::sync::Mutex;
    use wyd_db::{Account, DbError, Role};

    /// Porta fake: guarda e-mail e tokens, com a mesma regra de uso único do `pg`.
    #[derive(Default)]
    struct FakePinReset {
        email: Mutex<Option<String>>,
        tokens: Mutex<Vec<(String, bool)>>,
        emissao: Mutex<Vec<(String, i64)>>,
    }

    impl FakePinReset {
        fn email_guardado(&self) -> Option<String> {
            self.email.lock().expect("fake acessível").clone()
        }
    }

    impl PinReset for FakePinReset {
        fn email<'a>(
            &'a self,
            _account_id: i64,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Option<String>, DbError>> + Send + 'a>,
        > {
            Box::pin(async move { Ok(self.email_guardado()) })
        }

        fn set_email<'a>(
            &'a self,
            _account_id: i64,
            email: &'a str,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DbError>> + Send + 'a>>
        {
            let email = email.to_string();
            Box::pin(async move {
                *self.email.lock().expect("fake acessível") = Some(email);
                Ok(())
            })
        }

        fn issue_token<'a>(
            &'a self,
            _account_id: i64,
            token_hash: &'a str,
            _delivered_to: &'a str,
            ttl_seconds: i64,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DbError>> + Send + 'a>>
        {
            let hash = token_hash.to_string();
            Box::pin(async move {
                self.emissao
                    .lock()
                    .expect("fake acessível")
                    .push((hash.clone(), ttl_seconds));
                let mut tokens = self.tokens.lock().expect("fake acessível");
                for entry in tokens.iter_mut() {
                    entry.1 = true;
                }
                tokens.push((hash, false));
                Ok(())
            })
        }

        fn consume_token<'a>(
            &'a self,
            _account_id: i64,
            token_hash: &'a str,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool, DbError>> + Send + 'a>>
        {
            let hash = token_hash.to_string();
            Box::pin(async move {
                let mut tokens = self.tokens.lock().expect("fake acessível");
                for entry in tokens.iter_mut() {
                    if entry.0 == hash && !entry.1 {
                        entry.1 = true;
                        return Ok(true);
                    }
                }
                Ok(false)
            })
        }
    }

    struct Conta {
        limpos: Mutex<Vec<String>>,
    }

    impl Accounts for Conta {
        async fn find_by_login(&self, _login: &str) -> Result<Account, DbError> {
            Err(DbError::NotFoundKey { entity: "account" })
        }

        async fn find_by_id(&self, id: i64) -> Result<Account, DbError> {
            Ok(Account {
                id,
                login: "jogador".to_string(),
                role: Role::Player,
            })
        }

        async fn create_account(&self, login: &str, role: Role) -> Result<Account, DbError> {
            Ok(Account {
                id: 1,
                login: login.to_string(),
                role,
            })
        }

        async fn clear_pin_hash(&self, login: &str) -> Result<(), DbError> {
            self.limpos
                .lock()
                .expect("fake acessível")
                .push(login.to_string());
            Ok(())
        }
    }

    fn sessao(id: i64) -> Option<Session> {
        Some(Session {
            account_id: id,
            role: Role::Player,
        })
    }

    const HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn email_precisa_de_forma_minima() {
        assert!(valid_email("jogador@example.com"));
        assert!(!valid_email("jogador"));
        assert!(!valid_email("a@b"));
        assert!(!valid_email("a @b.com"));
        assert!(!valid_email("@b.com"));
        assert!(!valid_email(&format!("a@{}", "b".repeat(300))));
        assert!(valid_token_hash(HASH));
        assert!(!valid_token_hash("ABCDEF"));
        assert!(!valid_token_hash(&HASH.replace('a', "g")));
    }

    #[test]
    fn sem_email_nao_ha_link() {
        let repo = FakePinReset::default();
        let sink = VecSink::default();
        let erro = crate::block_on(start_pin_reset(sessao(7), 7, HASH, &repo, &sink));
        assert_eq!(
            erro,
            Err(AdminError::InvalidInput {
                reason: "cadastre um e-mail na conta antes de pedir o link"
            })
        );
        assert!(sink.entries().is_empty(), "falha antes de auditar");
    }

    #[test]
    fn token_emitido_uma_vez_e_consumido_uma_vez() {
        let repo = FakePinReset::default();
        let conta = Conta {
            limpos: Mutex::new(Vec::new()),
        };
        let sink = VecSink::default();
        crate::block_on(set_email(sessao(7), 7, "Jogador@Example.com", &repo, &sink))
            .expect("e-mail");
        assert_eq!(
            repo.email_guardado().as_deref(),
            Some("jogador@example.com")
        );

        let destino = crate::block_on(start_pin_reset(sessao(7), 7, HASH, &repo, &sink))
            .expect("token emitido");
        assert_eq!(destino, "jogador@example.com");
        assert_eq!(
            repo.emissao.lock().expect("fake acessível").as_slice(),
            &[(HASH.to_string(), TOKEN_TTL_SECONDS)],
            "TTL curto e fixo"
        );

        crate::block_on(confirm_pin_reset(sessao(7), 7, HASH, &conta, &repo, &sink))
            .expect("primeiro uso vale");
        assert_eq!(
            conta.limpos.lock().expect("fake acessível").as_slice(),
            &["jogador".to_string()]
        );

        let segundo = crate::block_on(confirm_pin_reset(sessao(7), 7, HASH, &conta, &repo, &sink));
        assert_eq!(
            segundo,
            Err(AdminError::InvalidInput {
                reason: "link inválido ou expirado"
            }),
            "uso único"
        );

        let acoes: Vec<String> = sink.entries().into_iter().map(|e| e.action).collect();
        assert_eq!(
            acoes,
            vec![
                "panel.email",
                "accounts.pin_reset_request",
                "accounts.pin_reset_self"
            ]
        );
    }

    #[test]
    fn sem_sessao_e_conta_invalida_nao_chegam_ao_banco() {
        let repo = FakePinReset::default();
        let conta = Conta {
            limpos: Mutex::new(Vec::new()),
        };
        let sink = VecSink::default();
        assert_eq!(
            crate::block_on(confirm_pin_reset(None, 7, HASH, &conta, &repo, &sink)),
            Err(AdminError::Unauthorized)
        );
        assert_eq!(
            crate::block_on(confirm_pin_reset(sessao(7), 0, HASH, &conta, &repo, &sink)),
            Err(AdminError::InvalidInput {
                reason: "conta inválida"
            })
        );
        assert!(conta.limpos.lock().expect("fake acessível").is_empty());
        assert_eq!(
            crate::block_on(set_email(None, 7, "a@b.com", &repo, &sink)),
            Err(AdminError::Unauthorized)
        );
    }
}

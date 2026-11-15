//! E-mail da conta e token de uso único do reset da segunda senha.
//!
//! O autoatendimento (jogador que esqueceu o PIN) prova a posse da conta por um link
//! enviado ao e-mail cadastrado. O banco guarda só o **hash** do token (`sha256` hex
//! calculado no BFF), com validade curta e consumo único: quem lê a tabela não consegue
//! reconstruir o link. Emitir um token novo invalida os anteriores ainda abertos da
//! mesma conta — pedir "de novo" nunca deixa dois links válidos circulando.
//!
//! A porta é `dyn`-compatível (`Pin<Box<dyn Future>>`, como `SessionTrail`) porque o
//! gateway a guarda como `Arc<dyn PinReset>`; sem isso o roteador não compila.

use std::future::Future;
use std::pin::Pin;

use sqlx::Row;

use crate::error::DbError;
use crate::pool::pg::{map_error, PgBackend};

/// E-mail e tokens de reset de PIN (porta do Cold Path).
pub trait PinReset: Send + Sync {
    /// E-mail cadastrado da conta (`None` = não cadastrado/ainda sem reset por link).
    fn email<'a>(
        &'a self,
        account_id: i64,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, DbError>> + Send + 'a>>;

    /// Grava o e-mail da conta. Conta inexistente é `NotFoundKey`.
    fn set_email<'a>(
        &'a self,
        account_id: i64,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), DbError>> + Send + 'a>>;

    /// Emite um token de uso único, invalidando os abertos da mesma conta.
    ///
    /// `token_hash` é o `sha256` hex do token que só o jogador recebeu por e-mail.
    fn issue_token<'a>(
        &'a self,
        account_id: i64,
        token_hash: &'a str,
        delivered_to: &'a str,
        ttl_seconds: i64,
    ) -> Pin<Box<dyn Future<Output = Result<(), DbError>> + Send + 'a>>;

    /// Consome o token: `false` = inválido, expirado ou já usado (nunca erro de transporte).
    fn consume_token<'a>(
        &'a self,
        account_id: i64,
        token_hash: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DbError>> + Send + 'a>>;
}

impl PinReset for PgBackend {
    fn email<'a>(
        &'a self,
        account_id: i64,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, DbError>> + Send + 'a>> {
        Box::pin(async move {
            let row = sqlx::query("SELECT email FROM accounts WHERE id = $1")
                .bind(account_id)
                .fetch_optional(self.pool())
                .await
                .map_err(map_error("accounts.email", "accounts"))?
                .ok_or(DbError::NotFoundKey { entity: "account" })?;
            row.try_get::<Option<String>, _>("email")
                .map_err(|_| DbError::Transport {
                    op: "accounts.email.decode",
                })
        })
    }

    fn set_email<'a>(
        &'a self,
        account_id: i64,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), DbError>> + Send + 'a>> {
        Box::pin(async move {
            let feito = sqlx::query("UPDATE accounts SET email = $2 WHERE id = $1")
                .bind(account_id)
                .bind(email)
                .execute(self.pool())
                .await
                .map_err(map_error("accounts.set_email", "accounts"))?;
            if feito.rows_affected() == 0 {
                return Err(DbError::NotFoundKey { entity: "account" });
            }
            Ok(())
        })
    }

    fn issue_token<'a>(
        &'a self,
        account_id: i64,
        token_hash: &'a str,
        delivered_to: &'a str,
        ttl_seconds: i64,
    ) -> Pin<Box<dyn Future<Output = Result<(), DbError>> + Send + 'a>> {
        Box::pin(async move {
            // Um link válido por conta: o pedido novo derruba o anterior.
            sqlx::query(
                "UPDATE pin_reset_token SET used_at = now() \
                 WHERE account_id = $1 AND used_at IS NULL",
            )
            .bind(account_id)
            .execute(self.pool())
            .await
            .map_err(map_error("pin_reset_token.invalidar", "pin_reset_token"))?;
            sqlx::query(
                "INSERT INTO pin_reset_token (account_id, token_hash, delivered_to, expires_at) \
                 VALUES ($1, $2, $3, now() + ($4 * interval '1 second'))",
            )
            .bind(account_id)
            .bind(token_hash)
            .bind(delivered_to)
            .bind(ttl_seconds)
            .execute(self.pool())
            .await
            .map_err(map_error("pin_reset_token.issue", "pin_reset_token"))?;
            Ok(())
        })
    }

    fn consume_token<'a>(
        &'a self,
        account_id: i64,
        token_hash: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DbError>> + Send + 'a>> {
        Box::pin(async move {
            let row = sqlx::query(
                "UPDATE pin_reset_token SET used_at = now() \
                 WHERE account_id = $1 AND token_hash = $2 AND used_at IS NULL \
                 AND expires_at > now() RETURNING id",
            )
            .bind(account_id)
            .bind(token_hash)
            .fetch_optional(self.pool())
            .await
            .map_err(map_error("pin_reset_token.consume", "pin_reset_token"))?;
            Ok(row.is_some())
        })
    }
}

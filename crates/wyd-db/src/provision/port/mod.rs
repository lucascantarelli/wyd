//! Contratos do provisionamento.
//!
//! `async_fn_in_trait` fica permitido aqui: o `run` é aguardado com `block_on`
//! pelo binário e pelos testes, nunca por `tokio::spawn`.
#![allow(async_fn_in_trait)]

pub mod catalog;
pub mod script;

pub use catalog::{Catalog, CatalogError};
pub use script::{Script, ScriptError};

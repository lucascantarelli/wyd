//! `wyd-launcher` — verificador do manifesto do patch.
//!
//! O manifesto é o do `ADR-LCH-0001`: JSON versionado com `schema`, `channel`, `version`,
//! `client_build`, `min_version` e `files[{path, size, sha256, url}]`. Este crate lê o
//! manifesto, recusa o que não é verificável e compara cada arquivo com o `sha256`
//! declarado antes de qualquer aplicação.
//!
//! O conteúdo do patch da `1.0.0` está fechado por `LCLI-02` (2026-11-25, opção A) e a
//! lista medida fica em `planejamento-1.0.0/wyd-client/registro-do-cliente.md` §5.1.
//!
//! A canonização RFC 8785 (`LLCH-01`) tem código em [`canon`] e a verificação da assinatura
//! Ed25519 destacada (`LLCH-03`) em [`signature`]. Assinar e publicar é do `wyd-ops`, com a chave
//! privada fora do repositório. Fora de escopo aqui: baixar o manifesto e aplicar os arquivos.
//! Verificar hash detecta corrupção; a assinatura autentica a origem.

pub mod canon;
pub mod manifest;
pub mod signature;
pub mod verify;

pub use canon::{canonicalize, canonicalize_str, CanonError};
pub use manifest::{FileEntry, Manifest, ManifestError};
pub use signature::{verificar, PublicKey, SignatureError, TrustStore};
pub use verify::{verify, Divergence, Report};

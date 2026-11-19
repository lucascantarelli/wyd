> Pergunta: "o que a camada pura do runner expõe para as camadas de cima?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2, §3, §4, §8 e §10
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/mod.rs` — reexport da camada pura

## 1. Responsabilidade

Declarar os oito módulos puros e reexportar os tipos que `port`, `app`, `adapter` e os testes precisam
nomear. É a **única** superfície da camada: nada em `migration/` importa `port`, `app` ou `adapter`.

## 2. Conteúdo exato

```rust
pub mod applied;
pub mod checksum;
pub mod file;
pub mod plan;
pub mod set;
pub mod sql;
pub mod version;

pub use applied::AppliedMigration;
pub use checksum::{Checksum, ChecksumError};
pub use file::{FileError, RawMigration, parse};
pub use plan::{plan, Drift, DriftKind, Plan};
pub use set::{MigrationSet, SetError};
pub use sql::{Sql, SqlTextError};
pub use version::{Slug, SlugError, Version, VersionError};
```

`mod.rs` **não** tem `#![allow(...)]` e não declara `#![deny(missing_docs)]`: a política é do crate
(`lib.rs`), uma vez.

## 3. Regras

| Regra | Por que |
|---|---|
| só reexport, sem tipo/função nova | o inventário de tipos é o de §2–§8 do spec; `mod.rs` não é lugar de conveniência |
| `pub use` de tipo **e** do erro correspondente | quem trata `VersionError` não deve importar `version` só por isso |
| `parse` é reexportado como função livre | o nome do módulo (`file::parse`) já diz o que faz; não vira `Parser` |
| nenhum `use crate::migrate::port::…` | violação da tabela de camadas (spec §10) |

## 4. Dependências (camada)

Permitido: `core`, `std` de tipos (`NonZeroU32`, `fmt`, `str`). Proibido: `std::fs`, `std::io`,
`std::env`, `port`, `app`, `adapter`, `sqlx`, `tokio` — a verificação `migration_layer_has_no_io` (§5)
confere por `grep` no diretório.

## 5. Testes

Não tem `#[cfg(test)]` próprio. A prova é indireta e suficiente:

| Teste | Onde |
|---|---|
| `migration_types_roundtrip` | integração do crate, importando só por `migrate::migration` |
| `migration_layer_has_no_io` | verificação de revisão: `grep -n "std::fs\|std::env\|sqlx" src/migrate/migration/` vazio |

## 6. Pronto quando

- todos os oito arquivos existem e o `pub use` compila sem `unused_imports`;
- nenhum caminho público da camada pura passa por `port` (conferido no `cargo doc`).

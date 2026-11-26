> Pergunta: "como o resto do crate chama o runner e o que este `mod.rs` expõe?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §1, §8, §10 e §10.1
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-24); implementado em 2026-11-26, com o contrato público congelado por `tests/contrato_api_migrate.rs` (`WYDDB-002`).

# `src/migrate/mod.rs` — fachada do runner

## 1. Responsabilidade

Ser a **única porta de entrada** do módulo: declarar os submódulos e expor `run` mais os tipos que o
binário, o boot e os testes de contrato precisam nomear. Não tem lógica própria, não lê ambiente, não
constrói adapter e não escreve SQL.

## 2. Declarações de módulo (ordem fixa)

```rust
pub mod adapter;      // único lugar com sqlx e std::fs
pub mod app;          // orquestração
pub mod config;       // MigrateConfig + constantes do desenho
pub mod error;        // MigrateError + Result
pub mod migration;    // modelo puro
pub mod port;         // contratos
```

Não existe `mod facade`: `run` é uma linha e mora aqui. Se um dia crescer, o corte é um arquivo novo
com ADR — não um módulo "opcional" deixado aberto.

`migration`, `port`, `app` e `adapter` são `pub` porque o binário nomeia `adapter::pg::from_config` e os
testes de contrato injetam portas falsas. `config` e `error` são `pub` porque aparecem em assinatura
pública. Nenhum `pub use` cria caminho alternativo para tipo que já tem dono: `Version` mora em
`migration::version`, não é reexportado no root.

## 3. API pública (assinatura exata)

```rust
use crate::migrate::error::MigrateError;
use crate::migrate::migration::plan::Drift;
use crate::migrate::port::{AdvisoryLock, Ledger, MigrationExecutor, Source};

/// Fachada: tudo injetado, nada construído aqui. `Send + Sync` são supertraits das portas.
pub async fn run<B, S>(command: Command, backend: &B, source: &S) -> Result<Report, MigrateError>
where
    B: Ledger + MigrationExecutor + AdvisoryLock,
    S: Source;

#[non_exhaustive]
pub enum Command {
    Status { check: bool },
    Apply { dry_run: bool },
    Down { to: DownTo },
}

#[non_exhaustive]
pub enum DownTo {
    Empty,            // `down --to 0`: desfaz a série inteira (§8.1)
    Version(Version), // deixa `to` aplicada
}

#[non_exhaustive]
pub struct Report {
    pub applied: Vec<Version>,
    pub skipped: usize,
    pub pending: usize,
    pub findings: Vec<Drift>,
}
```

O corpo é **uma linha**: `app::runner::run(command, backend, source).await`. A fachada existe para que o
binário não dependa de `app::runner`, que é detalhe de orquestração. A assinatura acima é a vigente em
`crates/wyd-db/src/migrate/mod.rs` e é congelada por `tests/contrato_api_migrate.rs`; a mesma assinatura
aparece em `app/mod.md` §2.

## 4. Reexports

| Caminho reexportado | De onde | Por que é público |
|---|---|---|
| `migrate::Command`, `DownTo`, `Report` | este arquivo | aparecem na assinatura de `run` |
| `migrate::config::MigrateConfig` | `config.rs` | o binário monta a configuração |
| `migrate::error::MigrateError` | `error.rs` | retorno de `run` |
| `migrate::migration::*` | `migration/` | `Plan`/`Drift` são lidos pelo boot e pelos testes |
| `migrate::port::*` | `port/` | testes de contrato implementam as portas falsas |
| `migrate::adapter::pg` | `adapter/pg.rs` | raiz de composição instancia o adapter |

## 5. Invariantes

1. **Sem I/O**: nenhum `std::fs`, `std::env`, `sqlx` ou `tokio` neste arquivo (spec §10, tabela de
   camadas). O `#[tokio::main]`/`block_on` é do binário.
2. **Sem estado global**: nenhum `static mut`, `OnceCell` ou cache de `MigrationSet`.
3. **Sem `unwrap`/`expect`**: o único `unwrap` permitido no módulo é em `#[cfg(test)]`.
4. **Sem reexport de tipo de terceiro**: `sqlx::Error` nunca aparece em assinatura pública.

## 6. Erros

Não define erro. Propaga `MigrateError` de `app::runner` sem traduzir nem embrulhar — embrulhar aqui
duplicaria contexto e quebraria o `match` do binário por variante.

## 7. Testes

| Teste | O que prova |
|---|---|
| `contrato_publico_de_migrate_compila` (`tests/contrato_api_migrate.rs`) | compila os três comandos, as duas formas de `DownTo`, os campos de `Report` e `run` com portas falsas, sem PostgreSQL |
| `reexports_are_reachable` (planejado) | os caminhos de §4 compilam a partir de `crate::migrate::…`; ainda não há teste próprio |

## 8. Pronto quando

- `cargo doc -p wyd-db` não acusa item público sem documentação neste arquivo;
- `cargo clippy -p wyd-db -- -D warnings` limpo, inclusive `missing_docs` se ligado no crate;
- o binário consegue `use wyd_db::migrate::{run, Command, Report, DownTo}` sem tocar em `app`.

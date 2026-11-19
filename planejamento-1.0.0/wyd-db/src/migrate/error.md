> Pergunta: "quais erros o runner devolve, o que cada um significa e com que código o processo sai?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/error.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §11
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/error.rs` — erro único do runner

## 1. Responsabilidade

Definir a taxonomia **fechada** de falha do runner (`MigrateError`), o alias `Result` do módulo e os
erros que nascem na configuração, na conexão e na leitura da série. Nenhuma variante carrega `String`
solta de SQL sem contexto: a versão viaja junto.

## 2. API pública

```rust
use std::time::Duration;
use crate::migrate::migration::plan::Drift;
use crate::migrate::migration::version::Version;
use crate::migrate::port::ledger::LedgerError;
use crate::migrate::port::source::SourceError;
use crate::migrate::port::unit_of_work::SqlError;
use crate::migrate::migration::set::SetError;

pub type Result<T> = std::result::Result<T, MigrateError>;

#[non_exhaustive]
pub enum MigrateError {
    Config(ConfigError),
    Connect(ConnectError),
    Source(SourceError),
    WrongRole { found: String, expected: &'static str },
    WrongServer { found: String, expected: u8 },
    SetInvalid(SetError),
    LockBusy { waited: Duration },
    Drift(Vec<Drift>),
    Ledger(LedgerError),
    Sql { version: Version, source: SqlError },
}

#[non_exhaustive]
pub enum ConfigError {
    TimeoutOutOfRange { given: Duration },
    MissingEnv { name: &'static str },   // usado pelo BINÁRIO ao montar a config
}

#[non_exhaustive]
pub enum ConnectError {
    /// Mensagem já redigida pelo adapter: nunca contém DSN nem senha.
    Refused { detail: String },
    Pool { detail: String },
}
```

`ConfigError::MissingEnv` existe aqui, e não no binário, porque a mensagem "variável ausente nomeia a
variável" (spec §9) é contrato do módulo: o binário só formata e sai com `2`.

## 3. Mapa de saída (fonte única)

| Código | Variantes | Quem lê |
|---:|---|---|
| `0` | `Ok` | binário |
| `2` | `Config`, `Connect`, `Source`, `WrongRole`, `WrongServer`, `Ledger` | binário |
| `3` | `LockBusy` | binário |
| `4` | `Drift`, `SetInvalid` | binário |
| `5` | `Sql` | binário |

```rust
impl MigrateError {
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self {
            MigrateError::Config(_) | MigrateError::Connect(_) | MigrateError::Source(_)
            | MigrateError::WrongRole { .. } | MigrateError::WrongServer { .. }
            | MigrateError::Ledger(_) => 2,
            MigrateError::LockBusy { .. } => 3,
            MigrateError::Drift(_) | MigrateError::SetInvalid(_) => 4,
            MigrateError::Sql { .. } => 5,
        }
    }
}
```

Código novo é decisão registrada: o `match` é exaustivo sobre `#[non_exhaustive]` do **próprio** crate
(compila por estar no mesmo crate); variante acrescentada quebra a compilação deste arquivo de
propósito.

## 4. Regras de `Display`

| Variante | Texto obrigatório |
|---|---|
| `Config` | nomeia a variável (`WYD_DB_URL_MIGRATOR`) ou a faixa aceita |
| `Connect` | "não conectou" + detalhe redigido; **nunca** a DSN |
| `Source` | caminho e motivo (`Missing`/`NotADirectory`/`Read`); nunca o conteúdo do arquivo |
| `WrongRole` | `esperado wyd_migrator, encontrado <found>` |
| `WrongServer` | `esperado PostgreSQL 17, encontrado <found>` |
| `SetInvalid` | delega ao `Display` de `SetError` (buraco/duplicata/nome) |
| `LockBusy` | esperou `<waited>`; sugere conferir `pg_locks` |
| `Drift` | lista **todas** as divergências, uma por linha, com `version` e `kind` |
| `Ledger` | forma divergente com o detalhe legível |
| `Sql` | `arquivo 0012: <sqlstate> — <mensagem>` |

`#[derive(Debug)]` em todos; `thiserror` para `Display`/`std::error::Error`. Sem `anyhow` no domínio
(spec §10).

## 5. Dependências (camada)

Permitido: `core`/`std`, `thiserror`, tipos de `migration/` e `port/`. Proibido: `sqlx`, `std::fs`,
`std::env`, `adapter` — o erro não conhece detalhe de driver, só `SqlError` da porta.

## 6. Testes

| Teste | O que prova |
|---|---|
| `exit_codes_are_fixed` | uma variante de cada grupo devolve `2`/`3`/`4`/`5`, incluindo `Ledger` e `Source` → `2` |
| `source_error_keeps_path` | `Missing { path }` formatado mostra o caminho e mapeia para `2` |
| `drift_display_lists_all` | duas divergências aparecem as duas, não só a primeira |
| `wrong_role_names_both` | texto contém o papel esperado **e** o encontrado |
| `sql_error_carries_version` | `Display` começa pela versão do arquivo |

## 7. Pronto quando

- `matches!` de `exit_code` cobre 100% das variantes e o teste acima falha se uma variante nova não for
  classificada;
- nenhum `Display` interpola `Debug` de DSN ou de pool.

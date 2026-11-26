> Pergunta: "quais erros o provisionamento devolve e com que código o processo sai?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/error.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §6
> Revisado em: 2026-11-18
**Arquivo implementado:** `src/provision/error.rs` (`WYDDB-003`).

**Estado:** aceito (2026-11-24); implementado em 2026-11-26 (`WYDDB-003`).

# `src/provision/error.rs` — erro do provisionamento
**Arquivo implementado:** `src/provision/error.rs` (`WYDDB-003`).

## 1. Responsabilidade

Definir `ProvisionError`, o alias `Result` e os dois erros que atravessam a fachada (configuração e
drift). Erros de porta (`CatalogError`, `ScriptError`) moram nas portas e são embrulhados aqui.

## 2. API pública

```rust
use crate::provision::domain::plan::Drift;
use crate::provision::port::catalog::CatalogError;
use crate::provision::port::script::ScriptError;

pub type Result<T> = std::result::Result<T, ProvisionError>;

#[non_exhaustive]
pub enum ProvisionError {
    Config(ProvisionConfigError),
    Catalog(CatalogError),
    Script(ScriptError),
    Drift(Vec<Drift>),
}

#[non_exhaustive]
pub enum ProvisionConfigError {
    MissingEnv { name: &'static str },      // "WYD_PROVISION_DSN"
    InvalidDsn { detail: String },          // redigido, sem senha
}
```

`ProvisionError::Drift` é derivado do `Plan` de `domain::plan::plan` — nunca recalculado no erro.

## 3. Mapa de saída

| Código | Variantes | Dono |
|---:|---|---|
| `0` | `Ok` | subcomando |
| `2` | `Config`, `Catalog`, `Script` | binário |
| `3` | — (divergência do `render --check` é comparação de arquivo, no binário) | subcomando |
| `4` | `Drift` | `ADR-DB-0001`, Decisão item 5 |

```rust
impl ProvisionError {
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self {
            ProvisionError::Drift(_) => 4,
            ProvisionError::Config(_) | ProvisionError::Catalog(_) | ProvisionError::Script(_) => 2,
        }
    }
}
```

Código `3` **não** existe aqui de propósito: `render --check` compara texto e arquivo, e essa é a única
pergunta do binário que não passa pelo módulo (spec §6).

## 4. Regras de `Display`

| Variante | Texto |
|---|---|
| `Config::MissingEnv` | nomeia a variável; nunca sugere default |
| `Config::InvalidDsn` | motivo do parse, sem repetir a DSN |
| `Catalog` / `Script` | delega ao `Display` da porta |
| `Drift` | uma linha por papel, com o atributo divergente |

## 5. Dependências (camada)

`core`/`std`, `thiserror`, tipos de `domain::plan` e das portas. Proibido: `sqlx`, `std::fs`,
`std::env`, `adapter`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `drift_exits_4` | `Drift` → `4` |
| `config_exits_2` | `MissingEnv` → `2`, texto contém o nome da variável |

`ProvisionConfigError` e a variante `Config` entram com `WYDDB-009`; nesta entrega o erro cobre
`Catalog`, `Script` e `Drift`, que são os caminhos do runner.
| `no_dsn_in_display` | `InvalidDsn` formatado não contém a senha da fixture |

## 7. Pronto quando

- `exit_code` é exaustivo e quebra a compilação se uma variante nova aparecer;
- nenhum `Display` interpola DSN crua.

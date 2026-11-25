> Pergunta: "como o binário chama o provisionamento e o que este `mod.rs` expõe?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §2, §5 e §6
> Revisado em: 2026-11-18
**Sem arquivo no repositório:** `src/provision/mod.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

**Estado:** aceito (2026-11-24) — `crates/wyd-db/src/provision/` não existe; especificação a implementar.

# `src/provision/mod.rs` — fachada do provisionamento
**Sem arquivo no repositório:** `src/provision/mod.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

## 1. Responsabilidade

Declarar os submódulos e expor `run`, `Command`, `Report` e o estado desejado (`ROLES`, `OWNER`,
`MIGRATOR`, `render`). Sem I/O, sem SQL, sem política: quem aplica é o adapter.

## 2. Conteúdo exato

```rust
pub mod adapter;
pub mod app;
pub mod domain;
pub mod error;
pub mod port;

pub use app::runner::run;
pub use domain::plan::{Drift, Plan, Step};
pub use domain::spec::{memberships, render, RoleSpec, MIGRATOR, OWNER, ROLES};
pub use error::{ProvisionError, Result};

use crate::provision::port::{Catalog, Script}; // só os bounds de `run`

#[non_exhaustive]
pub enum Command {
    Render,      // devolve o texto; o binário escreve/compara
    Plan,        // observado × desejado
    Apply,       // aplica os Steps
    Check,       // plan sem apply; o binário transforma em código 4
}

#[non_exhaustive]
pub enum Report {
    Rendered(String),
    Planned(Plan),
    Applied { steps: usize },
}

pub async fn run<B>(cmd: Command, backend: &B) -> Result<Report, ProvisionError>
where
    B: Catalog + Script;
```

`crate::provision::MIGRATOR` **precisa** existir como reexport: `src/migrate/config.rs` o consome como
`EXPECTED_ROLE` (spec §2). É a única razão de o root reexportar constante.

## 3. Regras

| Regra | Por que |
|---|---|
| `ROLES` e `render` vêm de `domain::spec` | o root não redefine estado desejado |
| `Command`/`Report` no root | são fachada, não orquestração |
| `run` só delega | `app::runner::run(cmd, backend).await` |
| sem `pub use sqlx` | driver não aparece na API pública |
| `Plan`/`Step`/`Drift` reexportados | o binário formata a saída de `plan`/`check` |

## 4. Dependências (camada)

Permitido: `core`/`std` e os submódulos. Proibido: `std::env`, `std::fs`, `sqlx` (spec §5, tabela de
camadas).

## 5. Testes

| Teste | O que prova |
|---|---|
| `root_reexports_migrator` | `crate::provision::MIGRATOR == crate::migrate::config::EXPECTED_ROLE` |
| `facade_runs_with_fakes` | `run(Command::Plan, &fake)` compila e devolve `Planned` |

## 6. Pronto quando

- o binário escreve `wyd_db::provision::{run, Command, Report}` sem tocar em `app`;
- nenhum tipo do driver em assinatura pública.

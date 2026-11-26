> Pergunta: "como o binário chama o provisionamento e o que este `mod.rs` expõe?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §2, §5 e §6
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-24); implementado em 2026-11-26 (`WYDDB-003`) — fachada, domínio, portas e runner. O adapter entra em `WYDDB-015`; o golden e o CLI ficam fora desta entrega.

# `src/provision/mod.rs` — fachada do provisionamento
**Arquivo implementado:** `crates/wyd-db/src/provision/mod.rs` (`WYDDB-003`).

## 1. Responsabilidade

Declarar os submódulos e expor `run`, `Command`, `Report` e o estado desejado (`ROLES`, `OWNER`,
`MIGRATOR`, `render`). Sem I/O, sem SQL, sem política: quem aplica é o adapter.

## 2. Conteúdo exato

```rust
pub mod app;
pub mod domain;
pub mod error;
pub mod port;

pub use app::runner::run;
pub use domain::plan::{Drift, Plan, Step};
pub use domain::spec::{memberships, render, RoleSpec, MIGRATOR, OWNER, ROLES};
pub use error::{ProvisionError, Result};

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

pub async fn run<B>(command: Command, backend: &B) -> Result<Report, ProvisionError>
where
    B: Catalog + Script;
```

`crate::provision::MIGRATOR` **precisa** existir como reexport: `src/migrate/config.rs` o consome como
`EXPECTED_ROLE` (spec §2). É a única razão de o root reexportar constante.

O `pub mod adapter;` do desenho de `spec.md` §5 entra em `WYDDB-015`, junto do `PgProvisionAdapter`;
esta entrega para em `app`, `domain`, `error` e `port`, que é o que o contrato de 2026-11-26 cobre.

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
| `render_devolve_texto_sem_tocar_o_backend` | `Command::Render` devolve `Rendered(render())`, sem catálogo nem script |
| `plan_e_check_leem_o_catalogo_e_nunca_o_script` | os dois leem `Catalog` e não chamam `Script` |
| `apply_entrega_o_texto_de_render_uma_vez` | `Apply` lê o catálogo, conta 7 passos e envia `render()` uma vez |

O alvo externo é `tests/contrato/contrato_api_provision.rs`; a execução está em §Evidência de execução da
`WYDDB-003`.

## 6. Pronto quando

- o binário escreve `wyd_db::provision::{run, Command, Report}` sem tocar em `app`;
- nenhum tipo do driver em assinatura pública;
- `adapter` não existe nesta entrega: entra em `WYDDB-015`.

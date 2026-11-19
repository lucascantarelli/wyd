> Pergunta: "em que ordem `render`, `plan`, `check` e `apply` rodam, e quando o processo para?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/app/runner.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3, §5 e §6; `ADR-DB-0001` item 3
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/app/runner.rs` — o fluxo

## 1. Responsabilidade

Traduzir `Command` em chamadas às duas portas, na ordem correta, e devolver `Report`. Único lugar onde a
sequência existe.

## 2. API

```rust
use crate::provision::domain::plan::{plan, Plan};
use crate::provision::domain::spec::{render, ROLES};
use crate::provision::error::{ProvisionError, Result};
use crate::provision::port::{Catalog, Script};
use crate::provision::{Command, Report};

pub async fn run<B>(cmd: Command, backend: &B) -> Result<Report>
where
    B: Catalog + Script;
```

## 3. Sequência

```text
Render            → Report::Rendered(render())            // sem porta, sem I/O
Plan | Check      → obs = backend.observed()?              → CatalogError → 2
                    p   = plan(ROLES, &obs)
                    Check && p.has_drift() → Err(Drift(p.drifts))   → 4
                    Report::Planned(p)
Apply             → obs = backend.observed()?              // relê: não confia em plano antigo
                    p   = plan(ROLES, &obs)
                    p.has_drift() → Err(Drift(p.drifts))   // drift não é corrigível por CREATE
                    p.steps.is_empty() → Report::Applied { steps: 0 }   // sem chamar a porta
                    backend.apply(&render())?               → ScriptError → 2
                    Report::Applied { steps: p.steps.len() }
```

Detalhes que não se improvisam:

| Ponto | Regra |
|---|---|
| `Apply` relê o cluster | entre dois `run` o estado mudou; reaproveitar `Plan` é aceitar estado velho |
| `Apply` não chama a porta com plano vazio | evita `CREATE ROLE` desnecessário e log enganoso |
| `Check` devolve `Report::Planned` em sucesso | o binário imprime o plano e sai `0` |
| `Render` não abre conexão | `--check` de arquivo é 100% offline (spec §6) |
| `steps` conta `Step`, não instrução SQL | 1 `CREATE ROLE` + 1 `GRANT` são 2 passos |
| erro de porta propaga sem tradução | `ProvisionError::Catalog`/`Script` |

## 4. Erros

Nunca converte `Drift` em `Step`. `ProvisionError` é construído só com as variantes de `error.rs`; o
código de saída vem de `exit_code()` no binário.

## 5. Dependências (camada)

`domain`, `port`, `error`. Proibido: `adapter`, `sqlx`, `std::env`, `std::fs`.

## 6. Testes (contrato, portas falsas)

| Teste | O que prova |
|---|---|
| `render_does_not_touch_backend` | `FakeCatalog` com contador: zero chamadas |
| `check_with_drift_is_error` | drift → `Err(Drift)`, `apply` não chamado |
| `check_without_drift_is_planned` | cluster completo → `Planned` com plano vazio |
| `apply_with_drift_does_not_execute` | `FakeScript` com contador: zero chamadas |
| `apply_empty_plan_skips_script` | cluster completo → `steps: 0`, contador zero |
| `apply_sends_render_output` | SQL recebido == `render()` |
| `apply_counts_steps` | cluster vazio → `steps == 6` |
| `catalog_error_propagates` | `Connect` → `ProvisionError::Catalog` |

## 7. Pronto quando

- a ordem de §3 aparece só aqui;
- `run` não menciona `PgProvisionAdapter`;
- `Apply` reaproveita o mesmo `plan` que reporta `steps` (sem recalcular).

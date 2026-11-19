> Pergunta: "quais são os dois contratos de infraestrutura do provisionamento?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/port/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3 e §5
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/port/mod.rs` — dois contratos

## 1. Responsabilidade

Declarar `Catalog` (leitura) e `Script` (execução) e seus erros, com o `allow` do lint de `async fn` em
trait no lugar correto.

## 2. Conteúdo exato

```rust
// Mesmo motivo de `src/migrate/port/mod.rs`: `async fn` em trait gera warning e o gate roda
// `clippy -D warnings`; nada aqui é `spawn`ado, tudo é aguardado no mesmo runtime.
#![allow(async_fn_in_trait)]

pub mod catalog;
pub mod script;

pub use catalog::{Catalog, CatalogError};
pub use script::{Script, ScriptError};
```

## 3. Regras

| Regra | Por que |
|---|---|
| **duas** portas | leitura e escrita separadas: `plan`/`check` não precisam de `Script` |
| `Send + Sync` como supertrait | `&B` atravessa `await` no runner |
| sem tipo de driver | `CatalogError`/`ScriptError` carregam `detail: String` |
| sem `Plan` aqui | a porta devolve `Vec<ObservedRole>`; decidir é do `domain` |

## 4. Dependências (camada)

`domain::observed` + `std`. Proibido: `sqlx`, `std::fs`, `std::env`, `app`, `adapter`.

## 5. Testes

| Teste | O que prova |
|---|---|
| `fakes_implement_ports` | `FakeCatalog`/`FakeScript` compilam contra as traits |
| `ports_are_object_safe` | `Box<dyn Catalog>` e `Box<dyn Script>` compilam |

## 6. Pronto quando

- `grep -rn "async_fn_in_trait" src/provision/` mostra só este arquivo;
- nenhuma porta tem `render`/`plan`/`ROLES` — estado desejado não atravessa porta.

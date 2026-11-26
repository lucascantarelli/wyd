> Pergunta: "o que a camada pura do provisionamento expõe?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/domain/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §2, §5
> Revisado em: 2026-11-18
**Arquivo implementado:** `src/provision/domain/mod.rs` (`WYDDB-003`).

**Estado:** aceito (2026-11-24); implementado em 2026-11-26 (`WYDDB-003`).

# `src/provision/domain/mod.rs` — modelo puro
**Arquivo implementado:** `src/provision/domain/mod.rs` (`WYDDB-003`).

## 1. Responsabilidade

Declarar os três arquivos puros e reexportar o que as camadas de cima usam. Nada de lógica.

## 2. Conteúdo exato

```rust
pub mod observed;
pub mod plan;
pub mod spec;

pub use observed::ObservedRole;
pub use plan::{plan, Drift, Plan, Step};
pub use spec::{memberships, render, RoleSpec, MIGRATOR, OWNER, ROLES};
```

## 3. Regras

| Regra | Por que |
|---|---|
| nenhum tipo definido aqui | cada arquivo tem um dono (estado desejado, observado, decisão) |
| `render` é função livre | não existe struct `Renderer`; entrada é `const` |
| sem `Result` na camada | `render` é infalível; `plan` devolve `Plan`, não erro |
| sem `use` de `port`/`app` | regra de dependência de spec §5 |

## 4. Dependências (camada)

`core`/`std` de tipos. Proibido: `std::env`, `std::fs`, `std::time`, `port`, `app`, `adapter`, `sqlx`,
`tokio` — é o que mantém o golden estável (spec §1).

## 5. Testes

| Teste | O que prova |
|---|---|
| `domain_has_no_io` | `grep -n "std::fs\|std::env\|sqlx" src/provision/domain/` vazio |
| `domain_types_are_reachable` | `use wyd_db::provision::domain::{ROLES, plan};` compila |

## 6. Pronto quando
**Arquivo implementado:** `spec.rs` (`WYDDB-003`).

- `render()` pode ser chamada em teste sem banco, disco ou relógio;
- o `pub use` não cria caminho para tipo que já tem dono em `spec.rs`.

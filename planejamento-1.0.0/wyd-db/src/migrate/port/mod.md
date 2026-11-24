> Pergunta: "quais são os contratos do runner e por que o lint de `async fn` em trait é silenciado aqui?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/port/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §3, §5, §7 e §10
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/port/mod.rs` — contratos do runner

## 1. Responsabilidade

Declarar as cinco portas, seus erros e os tipos de dado que atravessam a fronteira. É o ponto único onde
o `allow` do lint `async_fn_in_trait` vive, com o motivo escrito ao lado.

## 2. Conteúdo exato

```rust
// `async fn` em trait é warning por padrão (`async_fn_in_trait`) e o gate roda `clippy -D warnings`.
// O futuro devolvido não precisa prometer `Send`: nenhum chamador usa `tokio::spawn`, todos aguardam
// com `block_on` (binário, boot e testes). Se algum chamador passar a exigir `Send`, a troca é a
// assinatura (`-> impl Future<Output = …> + Send`), não este `allow`. Ver `src/migrate/spec.md` §10.
#![allow(async_fn_in_trait)]

pub mod ledger;
pub mod lock;
pub mod source;
pub mod unit_of_work;

pub use ledger::{Ledger, LedgerError};
pub use lock::{AdvisoryLock, LockError};
pub use source::{LoadedFile, Source, SourceError};
pub use unit_of_work::{MigrationExecutor, MigrationTxn, ServerInfo, SqlError};
```

## 3. Regras

| Regra | Por que |
|---|---|
| **cinco** portas, nenhuma a mais | um `Database` gordo violaria ISP (spec §10, tabela de princípios) |
| `Send + Sync` como supertrait | quem consome não repete o bound a cada chamada; `MigrationTxn` só `Send` |
| `async fn` em vez de `Box<dyn>` | `Txn` é tipo associado; `async fn` não é dyn-compatible |
| sem `sqlx` em assinatura | o erro de execução é `SqlError`, tipo da porta, não `sqlx::Error` |
| erro por porta | `LedgerError`, `LockError`, `SourceError`, `SqlError` — cada um com dono |

## 4. Regra de camada

`port/` importa `migration/` (tipos puros) e `std` de tipos. **Não** importa `app`, `adapter`, `sqlx`,
`std::fs`, `std::env` (spec §10). O adapter é quem implementa estas traits; o runner só as nomeia.

## 5. Testes

| Teste | O que prova |
|---|---|
| `fakes_implement_all_ports` | `FakeSource`/`FakeLedger`/`FakeExecutor`/`FakeLock` compilam contra as traits |
| `ports_are_object_safe_where_needed` | `Box<dyn Ledger>`, `Box<dyn AdvisoryLock>`, `Box<dyn Source>` compilam; `Box<dyn MigrationTxn>` **não** (esperado) |

## 6. Pronto quando

- `cargo clippy -p wyd-db -- -D warnings` passa com o `allow` **apenas** neste arquivo;
- `grep -rn "async_fn_in_trait" src/` mostra só este `mod.rs`;
- nenhuma porta tem método de conveniência (`exists`, `count`, `ping`).

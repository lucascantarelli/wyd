> Pergunta: "o que a orquestração do provisionamento expõe?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/app/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §5
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/app/mod.rs` — orquestração

## 1. Responsabilidade

Declarar o runner e reexportá-lo. Sem lógica própria.

## 2. Conteúdo exato

```rust
pub mod runner;

pub use runner::run;
```

## 3. Regras

| Regra | Por que |
|---|---|
| `runner` público | testes de contrato usam portas falsas |
| sem `Command`/`Report` | são da fachada (`provision/mod.rs`) |
| sem constante | `ROLES`/`OWNER` vivem em `domain::spec` |
| sem `adapter` | seta de dependência de spec §5 |

## 4. Dependências (camada)

Permitido: `domain`, `port`, `error`. Proibido: `adapter`, `sqlx`, `std::fs`, `std::env`.

## 5. Pronto quando

- `grep -n "sqlx" src/provision/app/` vazio;
- nenhum `#[cfg(test)]` aqui: a sequência é provada pelos testes de contrato de `runner`.

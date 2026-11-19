> Pergunta: "o que a camada de orquestração expõe e o que ela delega?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/app/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §8 e §10
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/app/mod.rs` — orquestração

## 1. Responsabilidade

Declarar o runner e reexportar a função que a fachada chama. Nada de lógica: o arquivo tem três linhas de
código e um reexport.

## 2. Conteúdo exato

```rust
pub mod runner;

pub use runner::run;
```

## 3. Regras

| Regra | Por que |
|---|---|
| `runner` é `pub` | os testes de contrato chamam `app::runner::run` com portas falsas |
| sem `Command`/`Report` aqui | esses tipos são da fachada (`migrate/mod.rs`), não da orquestração |
| sem constante de política | papel esperado e versão de servidor estão em `config.rs` |
| sem `use` de adapter | a camada proíbe (spec §10) |

## 4. Dependências (camada)

Permitido: `migration`, `port`, `error`. Proibido: `adapter`, `sqlx`, `std::fs`, `std::env`.

## 5. Pronto quando

- `grep -n "sqlx" src/migrate/app/` vazio;
- `cargo tree -p wyd-db -e features` não mostra `sqlx` puxado por `app`;
- nenhum `#[cfg(test)]` neste diretório: o fluxo é provado pelos testes de contrato de `runner` com
  portas falsas.

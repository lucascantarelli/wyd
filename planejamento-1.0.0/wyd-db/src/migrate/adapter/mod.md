> Pergunta: "quais adapters existem e por que `sqlx` e `std::fs` só aparecem aqui?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/adapter/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §10 e §10.1
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/adapter/mod.rs` — infraestrutura

## 1. Responsabilidade

Declarar os dois adapters e reexportá-los para a raiz de composição (o binário). É a fronteira onde
`sqlx` e `std::fs` entram no módulo.

## 2. Conteúdo exato

```rust
pub mod fs;
pub mod pg;

pub use fs::FsSource;
pub use pg::PgAdapter;
```

## 3. Regras

| Regra | Por que |
|---|---|
| adapter não chama `app` | o adapter implementa portas; quem orquestra é `app` (seta para dentro) |
| `from_config` só no adapter PG | pool, ledger e lock compartilham a mesma configuração (spec §10.1); `FsSource::new` recebe só o diretório |
| sem política de negócio | comparação de papel/versão é do runner; o adapter só informa |
| sem `pub` de tipo do driver | `PgPool`, `PgConnection` e `sqlx::Error` não aparecem na API pública |

## 4. Dependências (camada)

Permitido: `migration`, `port`, `sqlx`, `tokio`, `std::fs` (só `fs.rs`). Proibido: `app`.

## 5. Testes

| Teste | O que prova |
|---|---|
| `adapter_is_the_only_sqlx_user` | `grep -rl "sqlx" src/migrate/` devolve só `adapter/` |
| `fs_source_implements_source` | compila contra `dyn Source` |

## 6. Pronto quando

- `cargo tree` mostra `sqlx` alcançável apenas por `adapter`;
- nenhum `pub use sqlx::…` no módulo.

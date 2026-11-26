> Pergunta: "quem é o adapter do provisionamento e o que ele reexporta?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/adapter/mod.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3 e §5
> Revisado em: 2026-11-18
**Sem arquivo no repositório:** `src/provision/adapter/mod.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/provision/adapter/mod.rs` — infraestrutura
**Sem arquivo no repositório:** `src/provision/adapter/mod.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

## 1. Responsabilidade

Declarar o adapter PG e reexportá-lo para o binário. É onde `sqlx` entra no provisionamento.

## 2. Conteúdo exato

```rust
pub mod pg;

pub use pg::PgProvisionAdapter;
```

## 3. Regras

| Regra | Por que |
|---|---|
| um adapter para duas portas | `Catalog` e `Script` usam a mesma DSN e o mesmo pool |
| nome distinto do de `migrate` | o binário importa `PgAdapter` (runner) e `PgProvisionAdapter` no mesmo arquivo; nomes iguais obrigariam alias |
| sem tipo de driver público | `PgPool`/`sqlx::Error` não saem daqui |
| sem chamar `render` | o texto vem do runner |

## 4. Dependências (camada)

Permitido: `domain`, `port`, `sqlx`, `tokio`. Proibido: `app` (o adapter não orquestra).

## 5. Testes

| Teste | O que prova |
|---|---|
| `provision_adapter_is_object_safe` | `Box<dyn Catalog>`/`Box<dyn Script>` a partir do adapter |
| `only_adapter_uses_sqlx` | `grep -rl sqlx src/provision/` devolve só `adapter/` |

## 6. Pronto quando

- o binário monta o adapter com uma função só (`from_dsn`) e o passa por `&`;
- nenhum `pub use sqlx::…`.

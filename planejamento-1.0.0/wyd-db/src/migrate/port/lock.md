> Pergunta: "como dois processos serializam, com que chave, e por que não existe `unlock` na porta?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/port/lock.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §5 e §8
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/port/lock.rs` — exclusão mútua entre processos

## 1. Responsabilidade

Garantir que **um** processo aplique a série por vez, por sessão, com espera limitada. A porta expõe só
`acquire`: não há como pedir lock de transação nem liberar cedo, porque as duas coisas quebrariam a
serialização.

## 2. API pública

```rust
use std::time::Duration;

/// Chave estável para sempre: mudar é mudar o contrato de concorrência entre versões do binário.
pub const MIGRATION_LOCK_KEY: i64 = 0x5759_4404; // "WYD" + 04 (migrations)

pub trait AdvisoryLock: Send + Sync {
    /// Espera até `timeout`; estourou → `LockError::Busy`.
    async fn acquire(&self, timeout: Duration) -> Result<(), LockError>;
}

#[non_exhaustive]
pub enum LockError {
    Busy { waited: Duration },
    Query { detail: String },
}
```

## 3. Contrato

| Regra | Detalhe |
|---|---|
| lock de **sessão** (`pg_advisory_lock`) | a sessão atravessa uma transação por arquivo; lock `xact` soltaria entre arquivos |
| conexão dedicada | o adapter guarda a conexão do lock; a aplicação usa outra do pool |
| espera limitada | `SET lock_timeout = <timeout>` antes do `pg_advisory_lock`; estourou → `Busy` |
| sem `unlock` | liberar é devolver a conexão; expor `unlock` permitiria liberar no meio da série |
| sem variante `xact` | a garantia é do contrato, não de checagem em runtime |
| chave diferente da herdada | `0x5759_4401` era da série antiga; binário velho e novo não serializam entre si (aceito, ADR-0002) |

`Busy.waited` é o timeout configurado, não o tempo real medido: medir exigiria relógio e o valor não muda
a ação do operador ("outro processo está migrando; verifique `pg_locks`").

## 4. Regras de `Display`

| Variante | Texto |
|---|---|
| `Busy` | `outro processo está migrando (esperei <waited>); verifique pg_locks` |
| `Query` | detalhe redigido pelo adapter |

## 5. Dependências (camada)

`std::time::Duration`. Proibido: `sqlx`, `tokio`, `std::fs`, `std::env`, `app`, `adapter`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `fake_lock_can_be_held` | `FakeLock` tomado devolve `Busy` no segundo `acquire` |
| `key_is_fixed` | `MIGRATION_LOCK_KEY == 0x5759_4404` (regressão de contrato) |
| `second_process_waits_and_skips` (sistema) | dois processos: um aplica, o outro espera e termina sem aplicar |
| `busy_exits_3` (contrato) | `LockBusy` mapeia para código `3` |

## 7. Pronto quando

- a porta não tem `release`/`unlock`/`try_acquire`;
- a conexão do lock não é a mesma que executa a série (o adapter garante duas conexões);
- o teste de concorrência roda com dois processos reais, não duas threads no mesmo pool.

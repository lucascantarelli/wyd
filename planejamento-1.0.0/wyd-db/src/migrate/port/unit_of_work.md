> Pergunta: "como uma migration roda e é registrada na mesma transação, e como o preflight lê o servidor?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/port/unit_of_work.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §3, §6 e §7
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/port/unit_of_work.rs` — execução transacional

## 1. Responsabilidade

Ser a **única** porta que executa SQL e escreve no ledger, e a única que informa em que servidor/papel a
sessão está (preflight). A transação pertence ao adapter: a orquestração não emite `BEGIN`.

## 2. API pública

```rust
use crate::migrate::migration::file::RawMigration;
use crate::migrate::migration::sql::Sql;
use crate::migrate::migration::version::Version;
use crate::migrate::port::ledger::LedgerError;

/// Retrato mínimo da sessão, lido antes de qualquer `BEGIN` (spec §7).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerInfo {
    pub current_user: String,
    pub server_version_num: i32,   // 170000 para PG17
}

/// Unidade de trabalho. `commit` consome a transação; `Drop` faz ROLLBACK.
pub trait MigrationTxn: Send {
    async fn run(&mut self, sql: &Sql) -> Result<(), SqlError>;
    async fn record(&mut self, m: &RawMigration) -> Result<(), SqlError>;
    /// Apaga a linha do ledger — só o `down` usa (spec §8).
    async fn forget(&mut self, version: Version) -> Result<(), SqlError>;
    async fn commit(self) -> Result<(), SqlError>;
}

/// Bootstrap do ledger, fábrica de transação e retrato da sessão.
pub trait MigrationExecutor: Send + Sync {
    type Txn: MigrationTxn;
    /// Cria o ledger se ausente; se existir com forma diferente, falha (nunca `IF NOT EXISTS` cego).
    async fn prepare(&self) -> Result<(), LedgerError>;
    async fn begin(&self) -> Result<Self::Txn, SqlError>;
    /// `SELECT current_user, current_setting('server_version_num')`.
    async fn server_info(&self) -> Result<ServerInfo, SqlError>;
    /// Política do desenho vinda do binário: `WYD_MIGRATE_ALLOW_SUPERUSER=1` (spec §9).
    fn allows_superuser(&self) -> bool;
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlError {
    Server { sqlstate: Option<String>, message: String },
    Position { sqlstate: Option<String>, message: String, position: Option<u32> },
}
```

**Duas adições fecham lacunas do spec §7/§8 sem criar porta nova:** `server_info` (o preflight precisa
de `current_user` e da versão, e nenhuma outra porta responde) e `allows_superuser` (o binário lê a
variável, passa por `MigrateConfig`, o adapter guarda e a porta informa — a orquestração não lê ambiente).

`record` e `forget` devolvem `SqlError`, não `LedgerError`: são `INSERT`/`DELETE` dentro da transação, e
o que o operador precisa é `sqlstate` e `position`. `LedgerError` é da leitura (`Shape`) e do bootstrap
(`prepare`).

## 3. Contrato

| Regra | Detalhe |
|---|---|
| `run` executa **um** `Sql` já validado | o adapter não revalida; `Sql` é prova de formato |
| `record` grava `(version, slug, checksum)` | `applied_at` é `DEFAULT now()` do banco, não do processo |
| `forget` apaga **uma** linha | `down` e remoção do ledger commitam juntos |
| `commit(self)` consome | não existe `commit(&mut self)`: impossível commitar duas vezes |
| `Drop` sem `commit` = ROLLBACK | garantia do adapter, não do chamador |
| `prepare` é idempotente e roda sob lock | cria com a forma de §3; forma diferente → `LedgerError::Shape`; só `apply`/`down` chamam — `status`/`dry-run` leem ledger ausente como vazio |
| `server_info` não abre transação | é `SELECT` de sessão; o lock já está tomado neste ponto |

`SqlError` não carrega `sqlx::Error`: `sqlstate` e `position` vêm como dados, para a mensagem apontar
linha do arquivo (`position` é offset em bytes do texto enviado).

## 4. Regras de `Display`

| Variante | Texto |
|---|---|
| `Server` | `<sqlstate>: <message>` |
| `Position` | `<sqlstate>: <message> (posição <n>)`, omitindo a posição quando `None` |

## 5. Dependências (camada)

`migration::{file, sql, version}` e `port::ledger` (tipo de erro de `prepare`), `std`. Proibido: `sqlx`,
`tokio`, `std::fs`, `std::env`, `app`, `adapter`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `commit_is_consuming` | compila: `txn.commit()` e depois usar `txn` falha |
| `fake_txn_records_order` | `run`→`record`→`commit` na ordem, uma vez por arquivo |
| `drop_without_commit_rolls_back` (adapter/sistema) | tabela criada sem commit não existe |
| `prepare_rejects_foreign_shape` (sistema) | `LedgerError::Shape` com coluna a menos |
| `server_info_reports_role` (sistema) | `current_user == wyd_migrator`, `server_version_num / 10000 == 17` |
| `allows_superuser_default_false` | adapter criado por `from_config` sem opt-in responde `false` |

## 7. Pronto quando

- nenhum método desta porta aceita `&str` de SQL (só `&Sql`);
- o adapter abre `SET LOCAL search_path = ''` dentro de `begin`, não no chamador;
- `server_info` e `allows_superuser` são as únicas leituras fora de `Ledger`, e o teste de contrato as
  falsifica.

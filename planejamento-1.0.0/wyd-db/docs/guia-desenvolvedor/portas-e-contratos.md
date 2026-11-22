> Pergunta: "que porta eu uso para ler a série, gravar ledger, pegar lock ou aplicar um script?"
> Público: Desenvolvedor do runner e do provisionamento
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §10; `src/provision/spec.md` §5; `wyd-db-1.0.0.md` §6.16.1–§6.16.2
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Existem **sete portas desenhadas**, cinco no runner e duas no provisionamento. As portas de
repositório (`repo/`) ainda não têm contrato, então não entram aqui. A regra é simples: a porta é a
menor unidade que o `app` precisa; o adapter é quem fala `sqlx` ou `std::fs`.

## 1. Portas do runner

| Porta | Métodos | Garantia | Implementação | Quem chama |
|---|---|---|---|---|
| `Source` | `load() -> Vec<LoadedFile>` | entrega a série como texto; não cria ledger | `FsSource` | `app::runner` |
| `Ledger` | `applied() -> Vec<AppliedMigration>` | só leitura do que já foi aplicado | `PgAdapter` | `app::runner` |
| `MigrationExecutor` | `prepare()`, `begin() -> Txn` | prepara o ledger e abre uma transação por arquivo | `PgAdapter` | `app::runner` |
| `MigrationTxn` | `run(Sql)`, `record(RawMigration)`, `forget(Version)`, `commit()` | arquivo e linha do ledger commitam juntos; `Drop` desfaz | `PgAdapter` | `app::runner` |
| `AdvisoryLock` | `acquire(Duration)` | serializa dois processos; sem `unlock` exposto | `PgAdapter` | `app::runner` |

`MigrationTxn` é um tipo associado de `MigrationExecutor`, não `Box<dyn>`: `async fn` em trait não é
dyn-compatible. `Source` é separada de `Ledger` para permitir `FakeSource`/`FakeLedger` no teste de
contrato. `AdvisoryLock` usa lock de sessão (`pg_advisory_lock`), não de transação, porque a sessão
atravessa uma transação por arquivo.

## 2. Portas do provisionamento

| Porta | Método | Garantia | Implementação | Quem chama |
|---|---|---|---|---|
| `Catalog` | `observed() -> Vec<ObservedRole>` | lê `pg_roles` e `pg_auth_members`; nunca escreve | `PgProvisionAdapter` | `app::runner` |
| `Script` | `apply(sql: &str)` | envia o texto de `render()` numa sessão, sem transação explícita | `PgProvisionAdapter` | `app::runner` |

`plan` não usa `Script` e não abre conexão além do `Catalog`; `apply` chama `render()` e entrega o
texto. Um único adapter implementa as duas portas com o mesmo pool.

## 3. Tabela de decisão

| Preciso de… | Uso | Não uso |
|---|---|---|
| ler os arquivos da série | `Source` | `std::fs` no `app` |
| saber o que já foi aplicado | `Ledger` | consulta SQL no runner |
| aplicar um arquivo e registrar no ledger | `MigrationExecutor` + `MigrationTxn` | transação montada no `app` |
| impedir duas migrações concorrentes | `AdvisoryLock` | `SELECT ... FOR UPDATE` inventado |
| ler os papéis do cluster | `provision::Catalog` | `pg_roles` direto no `app` |
| aplicar o SQL de papéis | `provision::Script` | `sqlx` no `domain` |
| consultar pessoa, item ou sessão | porta de `repo/` | **não existe contrato ainda** |

## 4. Contratos que ainda não existem

- **Portas de `repo/`:** as operações de pessoa, conta, personagem, item, sessão e economia só têm
  página planejada; não há trait, assinatura nem erro.
- **Porta de transferência/save:** `transfer/save.rs` é página planejada; não há contrato de lote.
- **Porta de configuração:** `repo/config.rs` não tem contrato de leitura/escrita.
- **Erros de porta:** `SourceError`, `LedgerError`, `SqlError`, `LockError`, `CatalogError` e
  `ScriptError` são tipos do módulo; a página de erro de cada módulo é a dona da variante e do código.

## 5. O que este guia não cobre

- **SQL, pool e timeout:** são de `adapter/` e de `MigrateConfig`; não há número novo aqui.
- **Contrato de repositório:** enquanto não existir, não há “porta recomendada” para CRUD de jogo.
- **Teste de cada porta:** o alvo de contrato usa falsos; a página de teste é a dona do cenário.

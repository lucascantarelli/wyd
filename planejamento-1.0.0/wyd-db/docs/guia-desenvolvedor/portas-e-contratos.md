> Pergunta: "que porta eu uso para ler a série, gravar ledger, pegar lock ou aplicar um script?"
> Público: Desenvolvedor do runner e do provisionamento
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §10; `src/provision/spec.md` §5; `wyd-db-1.0.0.md` §6.16.1–§6.16.2;
> `wyd-core-1.0.0.md` §3.2
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22)

Existem **sete portas desenhadas** no runner/provisionamento e **uma porta de repositório com
contrato**, a C4 do core. As demais portas de `repo/` (pessoa, item, economia) ainda não têm contrato.
A regra é simples: a porta é a menor unidade que o `app` precisa; o adapter é quem fala `sqlx` ou
`std::fs`.

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
| consultar sessão/personagem ou commitar estado | C4 do core (`CoreRepository`) | SQL direto no core |
| consultar pessoa, item ou economia | porta de `repo/` ainda sem contrato | C4 não cobre |

## 4. Contratos que ainda não existem

- **Portas de `repo/`:** a exceção é C4, que tem contrato em §5 e em `wyd-core-1.0.0.md` §3.2;
  pessoa, conta, item e economia continuam sem trait, assinatura ou erro.
- **Porta de transferência/save:** `transfer/save.rs` é página planejada; não há contrato de lote.
- **Porta de configuração:** `repo/config.rs` não tem contrato de leitura/escrita.
- **Erros de porta:** `SourceError`, `LedgerError`, `SqlError`, `LockError`, `CatalogError` e
  `ScriptError` são tipos do módulo; a página de erro de cada módulo é a dona da variante e do código.

## 5. Porta C4 do core (2026-11-23)

**Quem é dono.** O `wyd-core` define a forma de `CoreRepository`, `UnitOfWork`, `StoreError` e dos
snapshots. O `wyd-db` implementa o adapter em `repo/`, é dono do SQL, da transação e do mapeamento de
constraint. O `wyd-server` compõe e injeta o adapter; o core não importa `wyd-db` nem `sqlx`.

| Operação | Entrada | Saída | Obrigação do adapter |
|---|---|---|---|
| `load_session` | identificador da sessão | `Option<SessionSnapshot>` ou erro tipado | ler uma sessão; ausência é `None` |
| `load_character` | identificador do personagem e da conta | `Option<CharacterSnapshot>` ou erro tipado | ler personagem da conta; outra conta/excluído é `None` |
| `commit` | `UnitOfWork` | `CommitReceipt` ou erro tipado | abrir uma transação, executar as operações na ordem, commitar ou desfazer; não expor `begin`/`rollback` |

**Operações da UoW.** `OpenSession` cria a sessão viva; `SelectCharacter` grava `character_id` somente
com sessão viva, ainda sem personagem, da mesma conta e personagem não excluído; `ReleaseSession`
grava `released_at`/`released_reason` uma vez e é idempotente para o mesmo motivo. A primeira fatia usa
uma operação de escrita por comando; a UoW aceita mais de uma quando a atomicidade é necessária.

**Fronteira.** A UoW do core é uma transação de `game`; não cruza `portal`, `ops` ou `content` na
primeira fatia. Uma falha desfaz todas as operações anteriores; não há commit parcial. `P-17` continua
sendo a view de leitura entre domínios; C4 não dá ao core acesso à tabela interna de outro domínio.

**Erros.** O adapter mapeia constraint para o erro tipado do core, sem devolver SQLSTATE, linha ou
SQL: sessão viva duplicada (`23505` em `ux_session_viva_por_conta`/`ux_session_viva_por_personagem`) e
demais invariantes viram `InvariantViolation`; FK/CHECK (`23503`/`23514`) viram `InvariantViolation`
com o motivo correspondente; estado que mudou vira `VersionConflict`; alvo que sumiu vira `NotFound`;
conexão/timeout/concorrência antes do commit vira `Unavailable`; commit cujo resultado não chegou vira
`CommitUncertain`, que exige nova leitura do core, nunca retry cego.

**Proibições.** O core não contém SQL, nome de tabela, conexão, transação ou tipo de linha; o `sim` não
importa a porta. O adapter é o único ponto de SQL. A prova é o grep em `wyd-core-1.0.0.md` §3.2 e o
teste de contrato com repositório falso, além do teste de integração que prova commit/rollback.

**Limite.** C4 cobre sessão, seleção de personagem e liberação; não cobre pessoa, item, economia,
posição persistida nem transferência em lote. Essas portas continuam sem contrato e não são inferidas
de C4.

## 6. O que este guia não cobre

- **SQL, pool e timeout:** são de `adapter/` e de `MigrateConfig`; não há número novo aqui.
- **Contrato de repositório:** C4 é a exceção documentada em §5; as demais portas de jogo continuam
  sem contrato.
- **Teste de cada porta:** o alvo de contrato usa falsos; a página de teste é a dona do cenário.

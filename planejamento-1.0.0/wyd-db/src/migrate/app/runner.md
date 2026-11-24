> Pergunta: "em que ordem o runner faz preflight, lock, prepare, plano e execução — e o que cada comando devolve?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/app/runner.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §3, §5, §6, §7, §8 e §11
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/app/runner.rs` — o fluxo

## 1. Responsabilidade

Orquestrar as portas na ordem correta, para no primeiro erro e devolver `Report`. É o único arquivo do
módulo onde a sequência do spec §8 existe; nenhum outro repete a ordem.

## 2. API

```rust
use crate::migrate::config::{EXPECTED_ROLE, SERVER_MAJOR};
use crate::migrate::error::MigrateError;
use crate::migrate::migration::plan::{plan, Drift};
use crate::migrate::migration::set::MigrationSet;
use crate::migrate::port::{Ledger, LockError, Source, MigrationExecutor, AdvisoryLock};
use crate::migrate::{Command, DownTo, Report};

pub async fn run<B, S>(cmd: Command, backend: &B, source: &S) -> Result<Report, MigrateError>;
```

Assinatura idêntica à da fachada: `migrate::run` só delega.

## 3. Sequência (fonte única)

```text
1. files = source.load()?                       → Source (2)
2. set   = MigrationSet::build(pairs(files))?    → SetInvalid (4); `pairs` = `(stem, content)`, §tabela
3. info  = backend.server_info()?                → Connect (2: não há versão de arquivo aqui)
4. preflight: current_user == EXPECTED_ROLE ou allows_superuser() → WrongRole (2)
              server_version_num / 10000 == SERVER_MAJOR         → WrongServer (2)
5. Status/Apply-dry-run: sem lock, sem prepare; ledger ausente = [] → Plan::Apply(todas)
6. Apply/Down: backend.acquire(cfg_timeout)?     → LockBusy (3)
7. backend.prepare()?                            → Ledger (2)
8. p = plan(&set, &backend.applied().await?)     → Plan; Drift bloqueia antes do `begin`
9. executa conforme o comando                    → Sql { version } (5)
```

O código de saída é decisão de `error.rs` (§3 daquela página); o runner só escolhe a variante.

Detalhes que o implementador não pode improvisar:

| Passo | Regra |
|---|---|
| `stem` | de `LoadedFile.path.file_stem()`; ausente → `SetInvalid` |
| preflight | comparação **estrita** de string (`==`), nunca `pg_has_role` |
| `apply` | `Plan::Drift` → `Err(Drift)` **antes** de qualquer `begin`; `Apply([])` → `Ok` com `skipped = applied.len()` |
| `apply --dry-run` | calcula `plan`, devolve `Report` e **não** pega lock |
| `down --to v` | percorre `applied` em ordem decrescente; para em `version <= v`; `DownTo::Empty` desce até o fim |
| `down` | cada arquivo: `begin` → `run(down)` → `forget(version)` → `commit` |
| `down` sem arquivo | versão no ledger sem `.sql` → `Plan::Drift(MissingFile)` e nada é desfeito |
| `status` | não escreve; `check` acrescenta `Err(Drift)` quando `Plan::Drift`; ledger ausente é lido como vazio, então banco virgem mostra todas pendentes (nunca `Shape` por ausência) |
| `Report` | `applied` = versões aplicadas/desfeitas nesta execução; `skipped` = já aplicadas; `pending` = restantes após a execução; `findings` = drifts |

## 4. Erros

Propaga `MigrateError` sem tradução. As conversões locais existem porque ainda não há versão de arquivo
para pendurar em `Sql`:

| De | Para | Por que |
|---|---|---|
| `SourceError` | `MigrateError::Source` | diretório/leitura é ambiente, não série |
| `LockError::Busy { waited }` | `MigrateError::LockBusy { waited }` | campo copiado, não recalculado |
| `LockError::Query`, `SqlError` de `server_info` | `MigrateError::Connect` | conexão, não SQL de arquivo |

## 5. Dependências (camada)

`migration`, `port`, `error`, `config` (constantes). Proibido: `adapter`, `sqlx`, `std::fs`, `std::env`,
`std::time::Instant` (o tempo do lock vem da config).

## 6. Testes (contrato, portas falsas)

| Teste | O que prova |
|---|---|
| `preflight_rejects_wrong_role` | `current_user = "postgres"` → `WrongRole`, sem chamar `prepare` |
| `allow_superuser_bypasses_role` | porta com `allows_superuser() == true` → segue |
| `apply_stops_at_first_error` | erro no 2º arquivo: 2º não commitado, 3º não tentado |
| `apply_is_noop_when_up_to_date` | ledger completo → `applied` vazio, `skipped == 3` |
| `dry_run_does_not_lock` | `FakeLock` conta chamadas: zero |
| `status_on_virgin_db_reports_all_pending` | ledger vazio, 3 arquivos → `pending == 3`, `applied` vazio, sem `prepare` |
| `source_error_is_not_set_invalid` | diretório ausente → `MigrateError::Source`, nunca `SetInvalid` |
| `dry_run_on_virgin_db_creates_nothing` | `FakeExecutor` conta `prepare`: zero, e o `Report` lista as 3 |
| `drift_blocks_apply` | `Plan::Drift` → erro, nenhum `begin` |
| `down_reverse_order` | `down --to 1` chama `2, 3` em ordem decrescente |
| `down_empty_removes_all` | `DownTo::Empty` desfaz tudo e ledger fica vazio |
| `status_check_exits_4` | `Drift` → `Err(Drift)` |

## 7. Pronto quando

- a sequência de §3 aparece **só** aqui (nem no `mod.rs`, nem no adapter);
- `run` é genérico nas portas e não menciona `PgAdapter`;
- cada ramo de `Command` tem teste de contrato, inclusive `DownTo::Empty`.

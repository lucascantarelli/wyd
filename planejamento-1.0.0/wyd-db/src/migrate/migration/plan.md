> Pergunta: "dado o disco e o ledger, o que fazer — aplicar, nada, ou parar por divergência?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/plan.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §8 e §11
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/plan.rs` — a decisão pura

## 1. Responsabilidade

Comparar `MigrationSet` (disco) com `&[AppliedMigration]` (ledger) e devolver `Plan`. Zero I/O, zero
consulta: é a função que os testes unitários cobrem e da qual `status` e o erro de drift derivam.

## 2. API pública

```rust
use crate::migrate::migration::applied::AppliedMigration;
use crate::migrate::migration::set::MigrationSet;
use crate::migrate::migration::version::Version;

#[non_exhaustive]
pub enum Plan {
    UpToDate,
    Apply(Vec<Version>),   // pendentes, ordem crescente
    Drift(Vec<Drift>),     // todos os achados, ordem crescente de versão
}

#[non_exhaustive]
pub struct Drift {
    pub version: Version,
    pub kind: DriftKind,
}

#[non_exhaustive]
pub enum DriftKind {
    Checksum,     // conteúdo do disco != ledger
    Slug,         // arquivo renomeado depois de aplicado
    MissingFile,  // ledger tem versão sem arquivo no disco
}

#[must_use]
pub fn plan(set: &MigrationSet, applied: &[AppliedMigration]) -> Plan;
```

## 3. Algoritmo

1. Indexar `applied` por `Version` (o ledger garante PK; se vier duplicado, o **último** vence e
   `adapter` já deveria ter falhado em `Shape` — registrar no comentário, não em `panic`).
2. Percorrer `set.iter()` em ordem crescente:
   - sem linha no ledger → `pending.push(version)`;
   - com linha e `checksum` diferente → `Drift { version, kind: Checksum }`;
   - com linha, checksum igual e `slug` diferente → `Drift { version, kind: Slug }`.
3. Percorrer `applied` cuja `version` não existe no set → `Drift { kind: MissingFile }`.
4. Ordenar `drift` por `version` (é a ordem em que o leitor corrige).
5. Retorno: `drift` não vazio → `Plan::Drift(drift)`; senão `pending` não vazio → `Plan::Apply(pending)`;
   senão `UpToDate`.

**Drift vence pendência.** Um arquivo alterado e três pendentes devolvem só `Drift`: aplicar por cima de
conteúdo divergente é o que o runner existe para impedir. `Report.pending` continua contando as
pendentes, para `status` mostrar as duas informações.

**Ledger vazio é estado legítimo**, não erro: é o banco virgem visto por `status` (a porta devolve
`Ok(vec![])` quando a tabela ainda não existe). O resultado é `Apply` com todas as versões do set.

## 4. Regras

| Regra | Por que |
|---|---|
| coleta **todos** os achados | parar no primeiro esconde o segundo e obriga a rodar N vezes |
| compara `Checksum`/`Slug` tipados | string comparada com string aceitaria `" "` |
| não olha `applied_at` | relógio não é decisão |
| puro e sem `Result` | não há erro de entrada: set já validado, ledger já tipado |
| `#[must_use]` | ignorar o plano é bug de chamador |

## 5. Dependências (camada)

`migration::{applied, set, version}` + `std::collections::HashMap`. Proibido: `port`, `sqlx`,
`std::fs`, `std::env`.

## 6. Testes

| Teste | Cenário | Esperado |
|---|---|---|
| `empty_ledger_applies_all` | 3 arquivos, ledger vazio | `Apply([1,2,3])` |
| `empty_ledger_is_not_error` | `[]` vindo de tabela ausente | mesmo `Apply([1,2,3])`, sem `Drift` |
| `full_ledger_is_up_to_date` | 3 aplicados iguais | `UpToDate` |
| `partial_applies_rest` | 1 aplicado, 3 no disco | `Apply([2,3])` |
| `checksum_change_is_drift` | 1 byte diferente | `Drift([{1, Checksum}])` |
| `slug_change_is_drift` | mesmo conteúdo, nome novo | `Drift([{1, Slug}])` |
| `missing_file_is_drift` | ledger com 3, disco com 2 | `Drift([{3, MissingFile}])` |
| `reports_all_drifts` | duas divergências | as duas, ordem crescente |
| `drift_wins_over_pending` | 1 alterado + 1 pendente | `Drift`, não `Apply` |

## 7. Pronto quando

- `plan` não recebe `&PgPool`, `&Path` nem `&MigrateConfig` — só os dois modelos;
- nenhum `clone()` do `MigrationSet` (itera por referência);
- `status --check` deriva `4` de `Plan::Drift` sem recalcular nada (spec §11).

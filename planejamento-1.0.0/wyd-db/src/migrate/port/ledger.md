> Pergunta: "como o runner lê o que já foi aplicado, e o que significa ledger com forma errada?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/port/ledger.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §3
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/port/ledger.rs` — leitura do ledger

## 1. Responsabilidade

Uma pergunta só: **quais versões estão aplicadas e com qual conteúdo**. A porta é somente leitura; criar
a tabela, validar a forma e escrever linha é do `MigrationExecutor`.

## 2. API pública

```rust
use crate::migrate::migration::applied::AppliedMigration;

pub trait Ledger: Send + Sync {
    /// Só consulta: não cria tabela e não escreve. Tabela ausente → `Ok(vec![])`.
    async fn applied(&self) -> Result<Vec<AppliedMigration>, LedgerError>;
}

#[non_exhaustive]
pub enum LedgerError {
    /// A tabela existe, mas a leitura não produz `AppliedMigration` (coluna ausente ou valor fora do tipo).
    Shape { detail: String },
    /// Falha de consulta: mensagem redigida pelo adapter, sem DSN.
    Query { detail: String },
}
```

## 3. Contrato

| Situação | Resultado |
|---|---|
| tabela ausente | `Ok(vec![])` — é o banco virgem; `status` e `apply --dry-run` não chamam `prepare` e precisam listar a série inteira como pendente |
| tabela existente com coluna ausente | `Shape { detail }` nomeando a coluna que a consulta não encontrou |
| tabela existente e legível, mas com forma divergente (coluna a mais, `CHECK` removida) | `applied()` lê as linhas normalmente; a recusa é de `prepare`, no `apply`/`down` — esta porta não é validadora de forma |
| `version` que não converte para `Version` | `Shape` (dado fora do contrato do tipo) |
| `checksum` com 63 chars | `Shape` |
| sucesso | `Vec<AppliedMigration>` em qualquer ordem; `plan` ordena por versão |

**Ausência não é `Shape`.** Tratar tabela ausente como erro faria `status` e `apply --dry-run` falharem no
banco virgem — justamente o primeiro diagnóstico antes do primeiro `apply`. A porta não distingue
"ledger vazio" de "ledger inexistente": devolve `Ok(vec![])` nos dois casos, e só o executor cria a
tabela, sob lock, em `prepare`.

`applied()` **não** valida forma: `prepare` é o dono dessa checagem (spec §3), e repetir aqui criaria duas
mensagens para o mesmo defeito. `Shape` cobre só o que a leitura em si não consegue entregar: coluna
ausente no `SELECT` (SQLSTATE `42703`) e valor que o tipo recusa na conversão. Consequência declarada: um
ledger com coluna a mais ou sem as `CHECK` passa por `status` e só é recusado quando `prepare` roda.

## 4. Regras

| Regra | Por que |
|---|---|
| sem `INSERT`/`CREATE` na porta | leitura e escrita separadas por ISP; o teste de contrato falsifica só leitura |
| ausência de tabela = lista vazia | é o que permite `status`/`dry-run` sem `prepare` e sem escrita |
| devolve modelo (`AppliedMigration`) | `app` não importa `migration::applied` por acidente de driver |
| sem `LIMIT` | o ledger tem no máximo N linhas (uma por versão); paginar complicaria sem ganho |
| erro carrega `detail`, nunca `sqlx::Error` | driver não aparece na API pública |

## 5. Dependências (camada)

`migration::applied` + `std`. Proibido: `sqlx`, `std::fs`, `std::env`, `app`, `adapter`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `fake_ledger_returns_applied` | `FakeLedger` injeta duas linhas e `plan` as usa |
| `absent_table_is_empty_ledger` (sistema) | banco sem `schema_migrations` → `Ok(vec![])`, sem DDL e sem erro |
| `status_on_virgin_db_lists_all_pending` (contrato) | ledger vazio + 3 arquivos → `Apply([1,2,3])`, não erro |
| `shape_error_names_column` (adapter/sistema) | ledger com coluna a menos devolve `Shape` contendo o nome da coluna |
| `bad_checksum_is_shape` (sistema) | linha com hash inválido não vira `AppliedMigration` |

## 7. Pronto quando

- a trait tem **um** método e nenhum `pub` de SQL;
- `LedgerError` não implementa `From<sqlx::Error>` em lugar público (conversão fica no adapter);
- banco virgem produz `Ok(vec![])`, não `Shape`, e o teste de sistema prova que nenhuma tabela foi criada.

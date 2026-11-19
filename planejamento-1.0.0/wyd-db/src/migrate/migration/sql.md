> Pergunta: "que texto pode virar `Sql`, e como recuso transação e marcador dentro do arquivo?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/sql.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2 e §6
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/sql.rs` — texto SQL validado

## 1. Responsabilidade

Ser o **único** tipo que carrega SQL a executar, com três invariantes no construtor: não vazio, sem
marcador `-- migrate:` e sem controle de transação. É o que impede o arquivo de fechar a transação que
o runner abriu.

## 2. API pública

```rust
use std::fmt;
use std::str::FromStr;

/// Trecho já validado. Campo privado: só `FromStr` constrói.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sql(String);

impl FromStr for Sql {
    type Err = SqlTextError;
    fn from_str(raw: &str) -> Result<Self, Self::Err>;
}

impl AsRef<str> for Sql;
impl fmt::Display for Sql;

#[non_exhaustive]
pub enum SqlTextError {
    Empty,
    MarkerInSql,                          // `-- migrate:` reapareceu no corpo
    TransactionControl { keyword: &'static str }, // BEGIN | COMMIT | ROLLBACK | SAVEPOINT
}
```

Exposto como `migrate::migration::sql::Sql`. `AsRef<str>` (não `as_str`) é o que `MigrationTxn::run`
consome: `run(sql.as_ref())`.

## 3. Algoritmo de varredura

A validação **não** é `contains("COMMIT")`: `0001_bootstrap` tem corpo PL/pgSQL com `BEGIN … END;`
dentro de `$$`, e `'COMMIT'` pode aparecer em seed. Por isso o scanner é uma máquina de estados de uma
passada:

| Estado | Entrada | Saída | O que faz |
|---|---|---|---|
| `Normal` | `--` | `LineComment` | nada até `\n` |
| `Normal` | `'` | `SingleQuote` | literal de string |
| `Normal` | `$tag$` | `DollarQuote(tag)` | corpo PL/pgSQL; tag pode ser vazia (`$$`) |
| `SingleQuote` | `''` | `SingleQuote` | aspas escapadas |
| `SingleQuote` | `'` | `Normal` | fecha literal |
| `DollarQuote(t)` | `$t$` | `Normal` | fecha o corpo |
| `Normal` | início de instrução | — | lê a primeira palavra e compara com a lista proibida |

- **Início de instrução** = primeiro token não-espaço depois do começo do texto ou de `;` em `Normal`.
- Comparação **case-insensitive** (`begin`, `Begin`), sempre palavra inteira (não casa `BEGINNING`).
- Só `BEGIN`, `COMMIT`, `ROLLBACK`, `SAVEPOINT` são proibidos. `SET`, `SET LOCAL`, `START TRANSACTION`
  seguido de erro de sintaxe? — `START` não está na lista: a série não o usa e o contrato só fecha o que
  o runner abriu.
- Marcador: qualquer ocorrência de `-- migrate:` fora de `LineComment`/literal → `MarkerInSql`.
- Depois de varrer: `raw.trim()` vazio → `Empty`.

## 4. Regras

| Regra | Por que |
|---|---|
| texto guardado **sem** alteração | checksum é do arquivo, não da versão aparada; `Sql` preserva bytes (só valida) |
| vazio é recusado depois de `trim` | arquivo com só comentário não é migration |
| erro nomeia a palavra | `TransactionControl { keyword: "COMMIT" }` permite mensagem acionável |
| sem reparse | `Sql` não expõe `&mut String` nem `into_inner` |

## 5. Dependências (camada)

Só `core`/`std`. Sem `regex` (scanner manual é mais simples que regex com `$$`), sem `sqlparser`
(analisar SQL de verdade é outro produto).

## 6. Testes

| Teste | Entrada | Esperado |
|---|---|---|
| `accepts_plain_ddl` | `CREATE TABLE x (id INT);` | `Ok` |
| `accepts_plpgsql_begin_end` | `CREATE FUNCTION f() … AS $$ BEGIN … END; $$;` | `Ok` — o caso que quebra `contains("BEGIN")` |
| `accepts_set_local` | `SET LOCAL search_path = '';` | `Ok` |
| `accepts_commit_in_string` | `INSERT … VALUES ('COMMIT');` | `Ok` |
| `rejects_empty` / `_only_comments` | `""`, `"-- nada"` | `Empty` |
| `rejects_commit` | `COMMIT;` | `TransactionControl { keyword: "COMMIT" }` |
| `rejects_begin_after_semicolon` | `SELECT 1; begin;` | `TransactionControl { keyword: "begin" }` |
| `rejects_savepoint` | `SAVEPOINT s1;` | `TransactionControl { keyword: "SAVEPOINT" }` |
| `rejects_marker` | `-- migrate:up` no corpo | `MarkerInSql` |
| `does_not_match_prefix` | `BEGINNING OF …` | `Ok` |

## 7. Pronto quando

- o teste do PL/pgSQL de `0001_bootstrap` está presente — é a regressão que protege a série real;
- `Sql` não implementa `Deref<Target = String>` (permitiria mutação);
- `Display` devolve o texto exato recebido.

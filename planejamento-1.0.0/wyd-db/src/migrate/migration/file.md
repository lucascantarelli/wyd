> Pergunta: "como um arquivo `.sql` vira `RawMigration`, e o que exatamente o parser recusa?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/file.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2 e §6
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/file.rs` — parser dos marcadores

## 1. Responsabilidade

Transformar `(nome_do_arquivo, conteúdo)` em `RawMigration { version, slug, up, down }`, com parser
**puro**: recebe texto e nome, não abre arquivo, não conhece caminho nem banco. É a função que a
tabela de casos cobre.

## 2. API pública

```rust
use crate::migrate::migration::sql::{Sql, SqlTextError};
use crate::migrate::migration::version::{Slug, SlugError, Version, VersionError};

#[non_exhaustive]
pub struct RawMigration {
    pub version: Version,
    pub slug: Slug,
    pub up: Sql,
    pub down: Sql,
}

/// `stem` é o nome sem extensão (`0001_bootstrap`), como veio do diretório.
pub fn parse(stem: &str, content: &str) -> Result<RawMigration, FileError>;

#[non_exhaustive]
pub enum FileError {
    BadName { stem: String, reason: NameReason },
    MissingUp,
    MissingDown,
    MarkerOutOfOrder,
    DuplicateMarker { marker: &'static str },
    MarkerNotAlone { line: usize },
    HeaderNotComment { line: usize },
    EmptySql { part: &'static str },      // "up" | "down"
    Sql { part: &'static str, source: SqlTextError },
    BadVersion(VersionError),
    BadSlug(SlugError),
}

#[non_exhaustive]
pub enum NameReason { NoSeparator, BadVersionDigits, MissingSlug }
```

`RawMigration` é `#[non_exhaustive]`: campo novo não é breaking change (spec §11).

## 3. Algoritmo

1. **Nome**: `stem` casa `^(\d{4})_(.+)$`. Os 4 dígitos **precisam** ser exatamente 4 (é o formato
   `NNNN_slug`); `00001_x` é `BadName { BadVersionDigits }`. Versão e slug saem daqui — é a razão de o
   parser receber `stem` e continuar puro.
2. **Linhas**: varrer com `lines()`. Um marcador só vale se `line.trim_end() == "-- migrate:up"` (ou
   `down`) e a linha **não** começa com espaço: `  -- migrate:up` é `MarkerNotAlone { line }`.
3. **Ordem**: exatamente um `up` e um `down`, `up` primeiro. Repetido → `DuplicateMarker`; `down`
   antes de `up` → `MarkerOutOfOrder`.
4. **Cabeçalho** (antes do `up`): só linha vazia ou começando com `--`. Qualquer outra coisa →
   `HeaderNotComment { line }` (1-based). É o que impede SQL rodar fora de marcador.
5. **Corpos**: texto entre `up` e `down` = `up`; depois do `down` = `down`. Cada corpo passa por
   `Sql::from_str`; erro de validação vira `FileError::Sql { part, source }`, e corpo vazio vira
   `EmptySql { part }` — a distinção importa na mensagem.
6. **Sem `trim` no corpo**: `Sql` recebe o texto como está; espaço em branco é conteúdo e entra no
   checksum.

## 4. Regras

| Regra | Consequência |
|---|---|
| marcador dentro de comentário não conta | o reconhecimento é linha inteira, não `contains` |
| `down` é obrigatório e não vazio | arquivo sem `down` não entra na série (spec §2) |
| parser não sabe caminho | `LoadedFile.path` só serve para mensagem, e fica no `adapter` |
| nada de `IF NOT EXISTS` no parser | idempotência é do SQL, não do formato |

## 5. Dependências (camada)

`core`/`std` + `sql.rs` + `version.rs`. Proibido: `std::fs`, `std::path`, `port`, `sqlx`.

## 6. Testes

| Teste | Entrada | Esperado |
|---|---|---|
| `parses_minimal_file` | cabeçalho + `up`/`down` | campos corretos, `up`/`down` exatos |
| `strips_nothing_from_body` | corpo com linha final | texto preservado |
| `rejects_missing_up` / `_missing_down` | um marcador só | variante correta |
| `rejects_duplicate_up` | dois `up` | `DuplicateMarker` |
| `rejects_down_before_up` | ordem trocada | `MarkerOutOfOrder` |
| `rejects_indented_marker` | `  -- migrate:up` | `MarkerNotAlone { line: 1 }` |
| `rejects_sql_in_header` | `SELECT 1;` antes do `up` | `HeaderNotComment` |
| `rejects_empty_down` | marcador `down` no fim | `EmptySql { part: "down" }` |
| `rejects_bad_stem` | `1_bootstrap`, `0001`, `00001_x` | `BadName` com a `NameReason` certa |
| `maps_sql_error_to_part` | `COMMIT` no `up` | `Sql { part: "up", … }` |

## 7. Pronto quando

- `parse` é `fn` pura: nenhuma leitura de disco no módulo (`grep std::fs` vazio em `migration/`);
- a tabela de casos cobre cada variante de `FileError` ao menos uma vez;
- `cargo test -p wyd-db migration::file` roda em milissegundos, sem fixture de banco.

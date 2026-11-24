> Pergunta: "como o runner recebe os arquivos do disco sem que a camada pura conheça `std::fs`?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/port/source.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2, §7 e §10
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/port/source.rs` — leitura da série

## 1. Responsabilidade

Contrato mínimo de "de onde vêm os arquivos": devolve os `.sql` do diretório com nome e conteúdo. Não
ordena, não valida, não parseia — isso é do `set.rs`, que é puro e testável sem disco.

## 2. API pública

```rust
use std::path::PathBuf;

/// `Send + Sync` é supertrait: quem consome não repete o bound.
pub trait Source: Send + Sync {
    async fn load(&self) -> Result<Vec<LoadedFile>, SourceError>;
}

#[non_exhaustive]
pub struct LoadedFile {
    pub path: PathBuf,   // só para mensagem de erro
    pub content: String, // texto cru; o parser não conhece caminho
}

#[non_exhaustive]
pub enum SourceError {
    Missing { path: PathBuf },
    NotADirectory { path: PathBuf },
    Read { path: PathBuf, detail: String },
}
```

`LoadedFile` não tem `file_name`/`stem`: quem deriva o nome é o **runner** (`app/runner.md` §3), de
`path.file_stem()`, e o `set.rs` recebe `(stem, content)`. O `path` fica para a mensagem ("não consegui
ler …").

## 3. Contrato

| Situação | Resultado |
|---|---|
| diretório ausente | `Err(SourceError::Missing { path })` → binário sai `2` |
| caminho é arquivo | `Err(NotADirectory { path })` → `2` |
| diretório legível e sem `.sql` | `Ok(vec![])` → `SetError::Empty` → `4` |
| arquivo ilegível/sem permissão | `Err(Read { path, detail })` → `2` |
| arquivo não-UTF-8 | `Err(Read { path, detail: "não é UTF-8" })` — série é texto, não blob |
| sucesso | todos os `*.sql`, **ordem do diretório** (o `set` ordena) |

Não filtra por padrão de nome: `0001_bootstrap.sql` e lixo (`README.md`) são separados pela extensão;
nome fora de `NNNN_slug` morre em `set.rs` com `SetError::File { file_name, source: FileError::BadName { .. } }`,
que é erro de série (`4`), não de ambiente (`2`).

## 4. Regras

| Regra | Por que |
|---|---|
| `async fn` (não `fn`) | o adapter de teste pode ler de memória; a assinatura é a mesma do fs real |
| sem ordenação na porta | uma ordem só, no `set`, senão duas fontes de verdade |
| sem `Path` público no erro de conteúdo | caminho é diagnóstico; conteúdo é dado |
| sem cache | cada `run` lê uma vez; cache de arquivo esconderia arquivo editado no meio do processo |

## 5. Dependências (camada)

`std::path::PathBuf` e `std`. Proibido: `std::fs` (é o adapter), `sqlx`, `migration` (a porta não
conhece `RawMigration` — é o que permite `FakeSource` devolver texto arbitrário).

## 6. Testes

| Teste | O que prova |
|---|---|
| `fake_source_returns_files` | `FakeSource` injeta dois arquivos e o runner os vê |
| `fs_missing_dir_is_error` (adapter) | `Missing` com o caminho |
| `fs_ignores_non_sql` (adapter) | `README.md` fora da lista |
| `fs_empty_dir_is_ok_empty` (adapter) | `Ok(vec![])`, não erro |

## 7. Pronto quando

- a trait não menciona `MigrationSet` nem `RawMigration`;
- `SourceError` não carrega `std::io::Error` (não vaza tipo de biblioteca na API pública).

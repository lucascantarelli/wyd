> Pergunta: "como o diretório da série é lido e o que conta como arquivo da série?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/adapter/fs.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2 e §10
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/adapter/fs.rs` — `FsSource`

## 1. Responsabilidade

Listar o diretório, filtrar `*.sql`, ler cada arquivo como UTF-8 e devolver `LoadedFile`. Não ordena, não
valida nome, não parseia marcador: tudo isso é da camada pura.

## 2. Estrutura

```rust
pub struct FsSource { dir: PathBuf }

impl FsSource {
    #[must_use] pub fn new(dir: PathBuf) -> Self;
}

impl Source for FsSource {
    async fn load(&self) -> Result<Vec<LoadedFile>, SourceError>;
}
```

`load` é `async` conforme a porta, mas o corpo usa `tokio::task::spawn_blocking` para `read_dir` +
`read_to_string`; é o único ponto do módulo que toca disco.

## 3. Algoritmo

1. `metadata(&self.dir)`: não existe → `SourceError::Missing`; existe e não é diretório →
   `NotADirectory`.
2. `read_dir`: cada entrada, se `file_type().is_file()` e `extension() == Some("sql")` (comparação
   **case-sensitive**: `.SQL` não é da série).
3. Ordenar por `file_name()` — ordenação estável para mensagem determinística; o `set` reordena por
   versão de qualquer forma.
4. `read_to_string`: `io::Error` → `Read { path, detail }`; bytes inválidos para UTF-8 → `Read` com
   "não é UTF-8" (a série é texto, `lossy` esconderia corrupção).
5. Devolver `LoadedFile { path, content }` — `content` **cru**.

## 4. Regras

| Regra | Por que |
|---|---|
| sem recursão em subdiretório | a série é plana; subpasta com `.sql` seria arquivo esquecido |
| sem seguir symlink | evita ler fora do diretório da série (e loop) |
| sem cache entre chamadas | arquivo editado no meio do processo apareceria como versão antiga |
| `path` no erro, não no sucesso | mensagem precisa do caminho; o parser não |
| diretório vazio é `Ok(vec![])` | quem recusa série vazia é `set.rs` (`SetError::Empty`) |

## 5. Dependências (camada)

`std::fs`, `std::path`, `std::io`, `tokio` (spawn_blocking), `port::source`. Proibido: `sqlx`, `app`.

## 6. Testes
**Sem arquivo no repositório:** `0001_a.sql`, `x.txt`, `0001_a.SQL`, `sub/0002_b.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Teste | Fixture | Esperado |
|---|---|---|
| `loads_only_sql` | `0001_a.sql`, `README.md`, `x.txt` | 1 item |
| `ignores_uppercase_extension` | `0001_a.SQL` | 0 itens |
| `missing_dir` | caminho inexistente | `Missing` |
| `path_is_file` | arquivo em vez de diretório | `NotADirectory` |
| `non_utf8_is_error` | bytes `0xFF` | `Read` com "não é UTF-8" |
| `empty_dir_is_ok` | diretório vazio | `Ok(vec![])` |
| `does_not_recurse` | `sub/0002_b.sql` | 0 itens da subpasta |

Fixtures em `tempfile::TempDir`, criadas por teste; nada de diretório fixo no repositório.

## 7. Pronto quando

- o único `std::fs` do módulo está neste arquivo (`grep` no diretório confirma);
- `load` não abre arquivo duas vezes e não guarda `String` além do retorno;
- o teste de `read_dir` sem permissão existe e é `#[cfg(unix)]` (no Windows a permissão não se aplica).

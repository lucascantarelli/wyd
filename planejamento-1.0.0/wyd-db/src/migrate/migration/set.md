> Pergunta: "como a série inteira é carregada, ordenada e recusada quando tem buraco ou duplicata?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/set.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2, §4, §7 e §8
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/set.rs` — `MigrationSet`

## 1. Responsabilidade

Receber o que o adapter leu do disco (pares nome/conteúdo), ordenar, validar contiguidade e duplicata, e
guardar cada arquivo já parseado com o seu checksum. É a **única** fonte de verdade sobre "quais versões
existem" para `plan` e para o executor.

## 2. API pública

```rust
use crate::migrate::migration::checksum::Checksum;
use crate::migrate::migration::file::{FileError, RawMigration};
use crate::migrate::migration::version::Version;

#[non_exhaustive]
pub struct Entry {
    pub raw: RawMigration,
    pub checksum: Checksum,
    pub file_name: String,   // NNNN_slug.sql, para mensagem de erro
}

pub struct MigrationSet { /* Vec<Entry> ordenado por versão */ }

impl MigrationSet {
    /// Recebe `(stem, content)`; valida, ordena e calcula checksum uma vez.
    pub fn build(files: Vec<(String, String)>) -> Result<Self, SetError>;
    #[must_use] pub fn len(&self) -> usize;
    #[must_use] pub fn is_empty(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &Entry>;
    #[must_use] pub fn get(&self, v: Version) -> Option<&Entry>;
    #[must_use] pub fn versions(&self) -> impl Iterator<Item = Version> + '_;
    #[must_use] pub fn last(&self) -> Option<Version>;
}

#[non_exhaustive]
pub enum SetError {
    Empty,
    NotContiguous { expected: u32, found: u32 },
    DuplicateVersion { version: u32, first: String, second: String },
    DuplicateSlug { slug: String, versions: (u32, u32) },
    File { file_name: String, source: FileError },
}
```

Não há variante para checksum: `Checksum::of` é infalível e o erro de hash inválido é da **leitura** do
ledger (`ChecksumError`, em `migration::checksum`), não do build da série.

`Entry` é `#[non_exhaustive]`; `MigrationSet` não é (não é construído por literal).

## 3. Algoritmo

1. `files` vazio → `SetError::Empty` (spec §7: diretório vazio é `SetInvalid`, não erro de `Source`).
2. Ordenar por `stem` **lexicograficamente** — com 4 dígitos fixos, lexicográfico é numérico.
3. Para cada par, `file::parse(stem, content)`; erro → `SetError::File { file_name, source }`.
4. Contiguidade: a primeira versão **precisa** ser `1`; cada seguinte é `anterior + 1`. Qualquer salto →
   `NotContiguous { expected, found }` (expected = valor que faltou, não a posição).
5. Duplicata de versão (dois arquivos com o mesmo `NNNN`) → `DuplicateVersion`; duplicata de slug em
   versões diferentes → `DuplicateSlug` (renomear arquivo aplicado é erro, spec §3).
6. `checksum = Checksum::of(content.as_bytes())` — do conteúdo cru, antes de qualquer parse.
7. Guardar `file_name = format!("{stem}.sql")` para mensagem; nenhuma outra cópia do caminho.

## 4. Regras

| Regra | Consequência |
|---|---|
| `build` recusa série vazia | o runner nunca aplica "nada" por acidente; vazio é `Empty`, não `Ok` |
| ordem é propriedade do `set`, não do adapter | `adapter/fs` devolve na ordem do diretório; quem ordena é aqui |
| slug duplicado recusado | impede duas páginas/arquivos com o mesmo nome lógico |
| sem `HashMap` público | a ordem de iteração é parte do contrato |

## 5. Dependências (camada)

`core`/`std` (`collections` não é necessário: `Vec` ordenado + busca binária por versão), `file.rs`,
`checksum.rs`, `version.rs`. Proibido: `std::fs`, `port`, `sqlx`.

## 6. Testes

| Teste | Entrada | Esperado |
|---|---|---|
| `builds_single_file` | `0001_bootstrap` | `len == 1`, `last == Version(1)` |
| `sorts_input_regardless_of_order` | `0002_…`, `0001_…` | iterador em `1, 2` |
| `rejects_empty` | `vec![]` | `Empty` |
| `rejects_gap` | `0001`, `0003` | `NotContiguous { expected: 2, found: 3 }` |
| `rejects_duplicate_version` | dois `0002_x`/`0002_y` | `DuplicateVersion` com os dois nomes |
| `rejects_duplicate_slug` | `0002_x`, `0003_x` | `DuplicateSlug` |
| `rejects_non_one_start` | `0002_…` sozinho | `NotContiguous { expected: 1, found: 2 }` |
| `checksum_is_of_raw_content` | conteúdo com `\r\n` | igual ao `Checksum::of` do texto cru |
| `get_returns_entry` | `get(Version(1))` | `Some`, `raw.version == 1` |

## 7. Pronto quando

- `build` é a única porta de construção e o `Vec` interno nunca é exposto mutável;
- `iter()` devolve em ordem crescente de versão por construção (teste com entrada embaralhada);
- nenhuma validação de formato duplicada: nome, marcador e SQL ficam em `file.rs`.

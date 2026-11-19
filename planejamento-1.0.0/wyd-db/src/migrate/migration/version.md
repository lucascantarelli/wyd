> Pergunta: "o que é uma versão de migration e um slug válido, e como recuso os inválidos?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/version.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2, §4 e §8
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/version.rs` — `Version` e `Slug`

## 1. Responsabilidade

Dois newtypes com invariante no construtor: `Version` nunca é zero e `Slug` só existe em `snake_case`
válido. Nenhum código fora deste arquivo constrói `Version(0)` nem um slug com `-`, maiúscula ou espaço.

## 2. API pública

```rust
use std::fmt;
use std::str::FromStr;

/// Versão numérica de uma migration. Invariante: > 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(std::num::NonZeroU32);

impl Version {
    pub fn new(v: u32) -> Result<Self, VersionError>;   // 0 → Zero
    #[must_use] pub fn get(self) -> u32;
}

impl fmt::Display for Version;      // zero-padded em 4: `0007`
impl FromStr for Version;           // 1..=9 dígitos, sem sinal, sem espaço

/// Nome do arquivo sem o prefixo numérico. Invariante: `^[a-z][a-z0-9_]*$`, 1..=60, sem `__` nem `_` final.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug(String);

impl Slug {
    pub fn new(s: &str) -> Result<Self, SlugError>;
    #[must_use] pub fn as_str(&self) -> &str;
}

impl fmt::Display for Slug;
impl FromStr for Slug;
impl AsRef<str> for Slug;
```

`Version` é `Copy` (cabe em `u32`); `Slug` não (tem `String`). `Ord` em `Version` é numérico, não
lexicográfico — é o que permite `down --to` comparar sem `parse`.

## 3. Regras de validação

| Entrada | `Version` | `Slug` |
|---|---|---|
| `"0"` / `0` | `Err(Zero)` | — |
| `"-1"` | `Err(NotANumber)` | — |
| `"0007"` | `Ok(7)` | — |
| `" 7"` | `Err(NotANumber)` (sem `trim`) | — |
| `""` | `Err(NotANumber)` | `Err(Empty)` |
| `"1234567890"` | `Err(TooLarge)` (> `u32::MAX`) | — |
| `"Game_Session"` | — | `Err(InvalidChar { ch: 'G', index: 0 })` |
| `"game-session"` | — | `Err(InvalidChar { ch: '-', index: 4 })` |
| `"game__session"` | — | `Err(DoubleUnderscore { index: 4 })` |
| `"game_"` | — | `Err(TrailingUnderscore)` |
| `"1game"` | — | `Err(LeadingDigit)` |
| 61 caracteres válidos | — | `Err(TooLong { len: 61, max: 60 })` |
| `"arch_name"` | — | `Ok` |

`Slug` **não** normaliza: recusar `game-session` em vez de virar `game_session` é o que faz renomear
arquivo já aplicado ser `DriftKind::Slug` e não linha nova no ledger (spec §3).

## 4. Erros

```rust
#[non_exhaustive]
pub enum VersionError { Zero, NotANumber, TooLarge }

#[non_exhaustive]
pub enum SlugError {
    Empty,
    TooLong { len: usize, max: usize },
    LeadingDigit,
    InvalidChar { ch: char, index: usize },
    DoubleUnderscore { index: usize },
    TrailingUnderscore,
}
```

`Display` de cada variante nomeia o valor recusado; nenhum erro carrega o texto inteiro do arquivo.

## 5. Dependências (camada)

Só `core`/`std` (`num::NonZeroU32`, `fmt`, `str`). Proibido: `std::fs`, `sqlx`, `port`, `regex`
(o crate não depende de `regex`; validação é laço por `char`, sem alocação extra).

## 6. Testes

| Teste | O que prova |
|---|---|
| `version_rejects_zero` | `new(0)` é `Err(Zero)` |
| `version_display_pads_four` | `7.to_string() == "0007"` e `1234 == "1234"` |
| `version_parse_accepts_leading_zeros` | `"0007"` → `7`, `"7"` → `7` |
| `version_ordering_is_numeric` | `Version(9) < Version(10)` |
| `slug_accepts_table_names` | `ops`, `server_channel`, `character_learned_skill` |
| `slug_rejects` | a tabela de §3, caso a caso, conferindo a **variante** |
| `slug_length_boundary` | 60 `Ok`, 61 `TooLong` |

## 7. Pronto quando

- `Slug` e `Version` só são construídos por `new`/`FromStr` (campos privados);
- a tabela de §3 está inteira em teste — cada linha é um caso nomeado;
- nenhum `to_string`/`parse` inventado por cima do `std` (spec §4).

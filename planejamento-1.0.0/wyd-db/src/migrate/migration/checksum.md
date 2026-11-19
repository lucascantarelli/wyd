> Pergunta: "qual é o checksum de um arquivo, com que normalização, e por que ele muda no Windows?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/checksum.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §4
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/checksum.rs` — SHA-256 normalizado

## 1. Responsabilidade

Calcular o checksum do arquivo **inteiro** (cabeçalho, `up` e `down`) com uma normalização de EOL que
faz o mesmo commit produzir o mesmo hash em Windows e Linux. É o que detecta `down` alterado e o que
torna `CRLF` invisível.

## 2. API pública

```rust
use std::fmt;
use std::str::FromStr;

/// SHA-256 do conteúdo normalizado. Invariante: sempre 32 bytes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Checksum([u8; 32]);

impl Checksum {
    /// Único construtor: normaliza EOL e hasheia.
    #[must_use] pub fn of(bytes: &[u8]) -> Self;
    #[must_use] pub fn as_bytes(&self) -> &[u8; 32];
}

impl fmt::Display for Checksum;   // 64 hex minúsculos
impl fmt::Debug for Checksum;     // `Checksum(ab12…)`: 8 primeiros + `…`
impl FromStr for Checksum;        // exatamente 64 chars [0-9a-f]
impl AsRef<[u8]> for Checksum;
```

## 3. Algoritmo (ordem importa)

1. Percorrer os bytes uma vez; cada par `\r\n` vira `\n`. `\r` isolado **permanece** (é conteúdo, não
   terminador de linha do Windows).
2. Se o resultado termina em **exatamente um** `\n`, remover esse único byte (`strip_suffix('\n')`,
   uma vez, nunca em laço).
3. `sha256` sobre os bytes resultantes; guardar os 32 bytes.

| Entrada | Normalizado | Consequência |
|---|---|---|
| `"a"` | `"a"` | — |
| `"a\n"` | `"a"` | mesmo hash de `"a"` |
| `"a\r\n"` | `"a"` | checkout com `autocrlf` não gera drift |
| `"a\n\n"` | `"a\n"` | **dois** `\n` são conteúdo: hash diferente de `"a"` |
| `""` | `""` | hash do vazio é válido (arquivo vazio só falha no parser) |
| `"\r"` | `"\r"` | `\r` isolado não é normalizado |

Dependência: `sha2` (`Sha256`). Proibido `md5`/`crc`: o ledger guarda 64 hex por `CHECK`.

## 4. Erros

```rust
#[non_exhaustive]
pub enum ChecksumError {
    Length { found: usize },     // != 64
    NotLowercaseHex { index: usize },
    NotHex { index: usize },
}
```

`of` é infalível; só `FromStr` (leitura do ledger) falha. `Display` mostra o índice do primeiro caractere
inválido — é o que torna um ledger corrompido diagnosticável sem despejar a linha.

## 5. Regras de integração

| Regra | Onde aparece |
|---|---|
| checksum cobre o arquivo inteiro | `set.rs` chama `Checksum::of(content.as_bytes())` uma vez |
| `Display` é o que o banco guarda | `adapter/pg.rs` grava `checksum.to_string()` |
| comparação é por igualdade do tipo | `plan.rs` compara `Checksum`, nunca `String` |
| normalização mora **aqui** | nenhum outro arquivo reimplementa EOL; `.gitattributes` (`*.sql text eol=lf`) é cinto, não regra |

## 6. Dependências (camada)

Permitido: `core`/`std`, `sha2`. Proibido: `std::fs` (recebe `&[u8]`), `sqlx`, `port`, `hex` (o
`Display` formata com `write!("{:02x}")` num `String` com capacidade 64).

## 7. Testes

| Teste | Entrada | Esperado |
|---|---|---|
| `crlf_equals_lf` | `b"a\r\n"` vs `b"a\n"` | hashes iguais |
| `trailing_newline_ignored_once` | `b"a\n"` vs `b"a"` | iguais |
| `double_newline_is_content` | `b"a\n\n"` vs `b"a\n"` | diferentes |
| `lone_cr_is_content` | `b"a\r"` vs `b"a"` | diferentes |
| `display_is_64_lowercase` | `of(b"x").to_string()` | casa `^[0-9a-f]{64}$` |
| `parse_roundtrip` | `of(b"x")` → `to_string` → `FromStr` | igual ao original |
| `parse_rejects_short` / `_uppercase` / `_non_hex` | `"ab"`, `"AB…"`, `"zz…"` | variante correta |

Referência cruzada: o vetor de `b""` deve constar do teste para provar que vazio não entra em pânico.

## 8. Pronto quando

- o hash de um arquivo do repositório é estável entre checkout `core.autocrlf=true` e `false`
  (teste de sistema com fixture de duas linhas e CRLF no arquivo de teste);
- nenhuma cópia da normalização existe fora deste arquivo.

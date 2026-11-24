> Pergunta: "o que o ledger devolve por linha e por que `applied_at` não entra na comparação?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/migration/applied.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §3
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/migrate/migration/applied.rs` — linha do ledger

## 1. Responsabilidade

Representar **uma linha aplicada** que veio de `public.schema_migrations`, sem I/O e sem `sqlx`. É o
tipo que `plan` consome; se ele não existir, `plan` importaria a porta e a camada pura deixaria de ser
pura.

## 2. API pública

```rust
use crate::migrate::migration::checksum::Checksum;
use crate::migrate::migration::version::{Slug, Version};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedMigration {
    pub version: Version,
    pub slug: Slug,
    pub checksum: Checksum,
}
```

Três campos visíveis, nenhum método. Sem `applied_at`: é auditoria da tabela e não participa de
comparação (spec §3). Sem `Default`, sem `new` — o adapter constrói por literal, e é o único que tem os
dados.

## 3. Regras

| Regra | Consequência |
|---|---|
| não carrega `applied_at` | `plan` não compara relógio; dois hosts não precisam de relógio sincronizado |
| `slug` é `Slug`, não `String` | um ledger com slug malformado falha na leitura, não vira `Drift` silencioso |
| `checksum` é `Checksum` | comparação é de tipo, não de string; `FromStr` valida 64 hex |
| `PartialEq` só para teste | comparar duas linhas é útil em fixture; não é chave de negócio |
| `#[non_exhaustive]` | coluna nova no ledger (ex.: `series`) não quebra o consumidor |

## 4. Quem constrói

`adapter/pg.rs`, no `impl Ledger for PgAdapter`: cada linha vira `AppliedMigration` com
`version: row.try_get::<i32,_>("version")?.try_into()`, `slug: Slug::from_str(...)`,
`checksum: Checksum::from_str(...)`. Erro de conversão é `LedgerError::Shape` (o ledger tem dado que o
tipo recusa) — nunca `unwrap`. Tabela ausente não chega aqui: o adapter devolve `Ok(vec![])` antes de
converter qualquer linha, porque `status` e `apply --dry-run` leem o ledger sem `prepare`.

## 5. Dependências (camada)

`migration::version`, `migration::checksum`. Proibido: `port`, `sqlx`, `std::time`, `chrono`
(nada de data nesta camada).

## 6. Testes

| Teste | O que prova |
|---|---|
| `equality_compares_three_fields` | mudar só o checksum torna a linha diferente |
| `no_default_construction` | compilação: não existe `AppliedMigration::default()` |
| `adapter_maps_row` (sistema) | uma linha do ledger real vira o struct com os três campos |

## 7. Pronto quando

- o struct não tem método além dos derives;
- `chrono`/`time` não aparecem em `Cargo.toml` por causa deste arquivo;
- o adapter não constrói `AppliedMigration` com `String` intermediário no caminho feliz.

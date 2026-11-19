> Pergunta: "como o cluster é representado depois da leitura, sem trazer `sqlx` para o modelo?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/domain/observed.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §5
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/domain/observed.rs` — retrato do cluster

## 1. Responsabilidade

Ser o dado que o `Catalog` devolve: nome, `login` e memberships **diretas** de cada papel encontrado.
`plan` compara isto com `ROLES`; por isso é tipo puro, sem driver.

## 2. API pública

```rust
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedRole {
    pub name: String,
    pub login: bool,
    pub member_of: Vec<String>,
}
```

Sem métodos, sem `Default`, sem construtor: o adapter monta por literal e `plan` só lê.

## 3. Regras

| Regra | Detalhe |
|---|---|
| `member_of` é **direta** | `pg_auth_members` sem expansão transitiva; `plan` compara um nível |
| ordem não importa | `plan` compara como conjunto; o adapter pode devolver na ordem da consulta |
| nomes crus | papel fora do padrão (`app_admin`) entra como veio; `plan` ignora (provision não remove) |
| sem senha | `pg_authid.rolpassword` **nunca** é lido; a consulta é só `pg_roles`/`pg_auth_members` |
| `#[non_exhaustive]` | coluna nova (ex.: `rolsuper`) não quebra quem consome |

## 4. Quem constrói

`adapter/pg.rs`, no `impl Catalog`: uma consulta com `array_agg` de memberships ou duas consultas
(`pg_roles` + `pg_auth_members`) agregadas em memória. Erro de leitura → `CatalogError::Query`, nunca
`unwrap`.

## 5. Dependências (camada)

`std` (`String`, `Vec`). Proibido: `sqlx`, `std::env`, `std::fs`, `port`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `equality_is_structural` | `PartialEq` compara os três campos |
| `no_credentials_in_type` | compilação: não existe campo de senha — `pg_authid` nunca chega ao modelo |

Como `plan` consome estes dados (ordem de `member_of`, papel fora de `ROLES`) é asserção de
`domain/plan.md`; repetir aqui duplicaria o mesmo teste.

## 7. Pronto quando

- o struct não tem `password`, `valid_until` ou qualquer campo de credencial;
- o adapter é o único lugar que constrói `ObservedRole`.

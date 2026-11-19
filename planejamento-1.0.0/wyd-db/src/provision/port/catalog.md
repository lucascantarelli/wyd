> Pergunta: "como o provisionamento lê o estado do cluster sem depender do driver?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/port/catalog.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3 e §5
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/port/catalog.rs` — leitura do cluster

## 1. Responsabilidade

Uma pergunta: **quais papéis existem, com qual `login` e quais memberships diretas**. Não decide, não
cria, não compara com `ROLES`.

## 2. API pública

```rust
use crate::provision::domain::observed::ObservedRole;

pub trait Catalog: Send + Sync {
    async fn observed(&self) -> Result<Vec<ObservedRole>, CatalogError>;
}

#[non_exhaustive]
pub enum CatalogError {
    Connect { detail: String },   // sem DSN; redigido pelo adapter
    Query { detail: String },
}
```

## 3. Contrato

| Situação | Resultado |
|---|---|
| conexão recusada / autenticação | `Connect { detail }` → binário `2` |
| sem permissão em `pg_auth_members` | `Query { detail }` → `2` (não é drift) |
| zero papéis `wyd_*` | `Ok(vec![])` — `plan` cria tudo |
| papéis de outros donos | entram na lista; `plan` ignora |
| memberships | **diretas**, sem expansão transitiva |

## 4. Regras

| Regra | Por que |
|---|---|
| devolve `domain::ObservedRole` | `app` não importa linha de driver |
| nunca lê `pg_authid.rolpassword` | senha não é assunto do provisionamento |
| sem cache | entre `plan` e `apply` o estado pode mudar; o `apply` relê no próximo `run` |
| sem ordenação obrigatória | `plan` compara como conjunto |

## 5. Dependências (camada)

`domain::observed` + `std`. Proibido: `sqlx`, `std::fs`, `std::env`, `app`, `adapter`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `fake_catalog_injects_roles` | `FakeCatalog` alimenta `plan` e produz o `Plan` esperado |
| `connect_error_is_not_drift` (contrato) | `Connect` → `2`, não `4` |
| `reads_membership_direct` (sistema) | `wyd_migrator` membro de `owner` aparece em `member_of` |

## 7. Pronto quando

- a trait tem um método;
- `CatalogError` não tem variante `Drift` (drift é decisão, não leitura).

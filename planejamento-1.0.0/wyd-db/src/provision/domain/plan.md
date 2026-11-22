> Pergunta: "dado o cluster lido, o que criar, o que está divergente e em que ordem?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/domain/plan.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3 e §5; `ADR-DB-0001` itens 3 e 4
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/domain/plan.rs` — a decisão

## 1. Responsabilidade

Comparar `ROLES` (desejado) com `&[ObservedRole]` (lido) e devolver `Plan` com `steps` executáveis e
`drifts` que **não** são corrigíveis por `CREATE`. Puro: sem I/O, sem SQL, sem erro.

## 2. API pública

```rust
use crate::provision::domain::observed::ObservedRole;
use crate::provision::domain::spec::RoleSpec;

#[non_exhaustive]
pub enum Step {
    CreateRole { name: &'static str, login: bool },
    GrantMembership { owner: &'static str, member: &'static str },
}

#[non_exhaustive]
pub struct Drift {
    pub role: &'static str,
    pub kind: DriftKind,
}

#[non_exhaustive]
pub enum DriftKind {
    Login { expected: bool, found: bool },   // LOGIN/NOLOGIN diferente
    MissingMembership { owner: &'static str }, // sem GRANT <owner> TO <role>
}

#[non_exhaustive]
pub struct Plan {
    pub steps: Vec<Step>,
    pub drifts: Vec<Drift>,
}

impl Plan {
    #[must_use] pub fn is_empty(&self) -> bool;   // steps.is_empty()
    #[must_use] pub fn has_drift(&self) -> bool;  // !drifts.is_empty()
}

#[must_use]
pub fn plan(desired: &[RoleSpec], observed: &[ObservedRole]) -> Plan;
```

## 3. Algoritmo (duas passadas)

1. Indexar `observed` por `name` (`HashMap<&str, &ObservedRole>`).
2. **Primeira passada, em ordem de `desired`:** papel ausente → `steps.push(CreateRole)`;
   presente com `login` diferente → `drifts.push(Drift { Login })`.
3. **Segunda passada, em ordem de `desired`:** para cada `member_of` ausente de `observed.member_of` →
   `steps.push(GrantMembership { owner, member })`. Membership faltando é **ação, não drift**, tanto para
   papel novo (o `GRANT` roda depois de todos os `CREATE`) quanto para papel existente.
4. **Todos os `CreateRole` antes de qualquer `GrantMembership`** — garantia de que nenhum `GRANT` cite
   papel que ainda não existe. A ordem relativa dentro de cada grupo segue `desired`.
5. Papel observado fora de `desired` → ignorado (provision não remove, ADR item 3). Membership extra →
   ignorada pelo mesmo motivo.

`Drift::MissingMembership` fica reservado ao `member_of` que aponta para papel inexistente — estado que
o `assert!` de `spec.rs` já impede. Permanece no enum para o `match` do runner e para o teste de
invariante; o comentário da variante registra que o golden não o alcança.

## 4. Regras

| Regra | Por que |
|---|---|
| `plan` não altera papel existente | `ALTER ROLE` é `Drift`, nunca `Step` (ADR item 3) |
| `GRANT` é idempotente | pode rodar de novo sem checar `pg_auth_members` antes — mas o plano evita ruído |
| sem `Result` | entradas já são tipos válidos; não há como falhar |
| `steps` vazio ≠ `drifts` vazio | `apply` só roda com drift zero; `check` olha drift |
| ordem determinística | teste de golden de plano compara `Vec` |

## 5. Dependências (camada)

`domain::{spec, observed}`, `std::collections::HashMap`. Proibido: `sqlx`, `std::env`, `std::fs`,
`std::time`, `port`, `app`.

## 6. Testes

| Teste | Cenário | Esperado |
|---|---|---|
| `empty_cluster_creates_all` | `observed` vazio | 6 `CreateRole` + 1 `GrantMembership`, criações antes do grant |
| `complete_cluster_is_empty` | 6 papéis iguais | `steps` e `drifts` vazios |
| `missing_role_is_created` | falta `wyd_portal` | 1 `CreateRole { wyd_portal, true }` |
| `login_mismatch_is_drift` | `wyd_migrator` com `login = false` | `Drift { Login { expected: true, found: false } }`, **sem** `Step` |
| `missing_membership_is_granted` | `wyd_migrator` sem owner | `GrantMembership { wyd_owner, wyd_migrator }` |
| `extra_role_is_ignored` | `app_admin` no cluster | plano inalterado |
| `extra_membership_is_ignored` | `wyd_game` membro de `wyd_owner` | nenhum `Step` de remoção |
| `plan_ignores_membership_order` | mesmo papel com `member_of` em ordens diferentes | `Plan` idêntico |
| `order_is_deterministic` | duas chamadas | `Vec` idêntico |

## 7. Pronto quando

- `apply` nunca é chamado com `has_drift()` verdadeiro (teste de contrato em `app/runner`);
- o teste `empty_cluster_creates_all` afirma explicitamente a ordem criações → grants;
- `Step` não tem variante `AlterRole`/`DropRole`.

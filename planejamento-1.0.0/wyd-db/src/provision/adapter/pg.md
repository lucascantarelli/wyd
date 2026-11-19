> Pergunta: "quais consultas o adapter faz e como o script é executado numa sessão?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/adapter/pg.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3 e §5
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/adapter/pg.rs` — `PgProvisionAdapter`

## 1. Responsabilidade

Implementar `Catalog` e `Script` sobre `sqlx`: ler `pg_roles`/`pg_auth_members` e executar o texto de
`render()` numa sessão. Único arquivo do provisionamento com SQL.

## 2. Estrutura

```rust
pub struct PgProvisionAdapter { pool: sqlx::PgPool }

impl PgProvisionAdapter {
    pub async fn from_dsn(dsn: &str) -> Result<Self, CatalogError>;
}
```

`from_dsn` usa `PgPoolOptions::new().max_connections(1)`, porque `provision` é serial por natureza.

## 3. `Catalog::observed`

Duas consultas, na mesma conexão:

```sql
SELECT rolname, rolcanlogin
FROM pg_roles
WHERE rolname LIKE 'wyd\_%'
ORDER BY rolname;

SELECT r.rolname AS member, g.rolname AS owner
FROM pg_auth_members m
JOIN pg_roles r ON r.oid = m.member
JOIN pg_roles g ON g.oid = m.roleid
WHERE r.rolname LIKE 'wyd\_%';
```

Agregar em `HashMap<String, ObservedRole>` e devolver `Vec`. `LIKE 'wyd\_%'` filtra o que interessa;
papéis de terceiros não poluem `plan`.

## 4. `Script::apply`

`sqlx::raw_sql(sql).execute(&self.pool)`. O driver envia o texto inteiro; `DO` + `GRANT` rodam na mesma
sessão, que é o que faz a membership valer. `sqlx::Error` → `ScriptError::{Connect, Query { position }}`.

## 5. Conversão de erro

| De | Para | Regra |
|---|---|---|
| `sqlx::Error::Io`/`PoolTimedOut`/`Configuration` | `Connect`/`CatalogError::Connect` | `detail` redigido, sem DSN |
| `sqlx::Error::Database` no catálogo | `CatalogError::Query { detail }` | sem `position`: a leitura é de lista, não de instrução |
| `sqlx::Error::Database` no script | `ScriptError::Query { detail, position }` | `code()` fica fora do `Display`; `position()` entra |
| `array_agg` NULL | lista vazia | nunca `unwrap` em `Option` |

## 6. Regras

| Regra | Por que |
|---|---|
| nunca lê `pg_authid` | credencial fora de escopo |
| sem `SET ROLE` | o provisionamento roda como superusuário do cluster, por desenho |
| transação: contrato em `port/script.md` §3 | `render()` é idempotente; transação ampliaria o lock de catálogo |
| uma conexão | serial, e `max_connections(1)` pega bug de concorrência cedo |

## 7. Testes (sistema, PG17 efêmero)

| Teste | O que prova |
|---|---|
| `observed_empty_cluster` | `Vec` vazio, sem erro |
| `observed_reads_login_flag` | `wyd_migrator` com `login = true` |
| `observed_reads_direct_membership` | `member_of` traz `wyd_owner` |
| `apply_creates_roles` | `render()` aplicado cria os 5 papéis |
| `apply_twice_is_idempotent` | segunda execução sem erro, cluster igual |
| `apply_grants_membership` | `pg_auth_members` tem `owner → migrator` |
| `connect_error_has_no_dsn` | `Display` sem senha |

## 8. Pronto quando

- dois `impl` (uma porta cada) no mesmo arquivo, nenhum `unwrap`;
- `sqlx::Error` não aparece em assinatura pública;
- o teste de idempotência reusa o mesmo adapter (mesmo pool) duas vezes.

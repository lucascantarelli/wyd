> Pergunta: "quais papéis existem, com que atributos, e qual texto exato `render()` emite?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/domain/spec.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §2, §4 e §8
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/provision/domain/spec.rs` — estado desejado e `render()`

## 1. Responsabilidade

Fixar em código os cinco papéis de cluster e suas memberships, e gerar deles o texto de `db/roles.sql`
— mais o bloco fixo de `public` que o ledger pressupõe (`spec.md` §4). O código é a fonte para os
papéis; o `.sql` é artefato. Zero I/O, zero ambiente, zero erro.

## 2. API pública

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleSpec {
    pub name: &'static str,                 // sempre prefixado por `wyd_`
    pub login: bool,                        // true → LOGIN
    pub member_of: &'static [&'static str], // membership direta, um nível
}

pub const ROLES: &[RoleSpec];
pub const OWNER: &str;      // "wyd_owner"
pub const MIGRATOR: &str;   // "wyd_migrator"

pub fn memberships() -> impl Iterator<Item = (&'static str, &'static str)>;
#[must_use] pub fn render() -> String;
```

`RoleSpec` **não** é `#[non_exhaustive]`: o `const` exige construção por literal (spec §5).

## 3. Dados (ordem = ordem de emissão)

| # | `name` | `login` | `member_of` |
|---:|---|---|---|
| 1 | `wyd_owner` | `false` | `[]` |
| 2 | `wyd_migrator` | `true` | `["wyd_owner"]` |
| 3 | `wyd_game` | `true` | `[]` |
| 4 | `wyd_portal` | `true` | `[]` |
| 5 | `wyd_read` | `true` | `[]` |

Acompanha o `const`:

```rust
const _: () = assert!(ROLES.len() == 5);
```

É a prova em tempo de compilação de que papel novo exige golden novo (spec §2).

## 4. `render()` — texto exato

Emite, nesta ordem, com `\n` final único e sem `\r`:

1. duas linhas de comentário (`-- db/roles.sql — GERADO por …` e a linha `-- Papéis do cluster e o schema …`);
2. `DO $$`, `BEGIN`;
3. para cada papel de `ROLES`, o par `IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = '<name>') THEN`
   / `CREATE ROLE <name> LOGIN|NOLOGIN;` / `END IF;`, indentado com dois espaços dentro do bloco;
4. `END $$;`;
5. linha em branco;
6. comentário `-- membership separada: `IN ROLE` só vale na criação do papel`;
7. um `GRANT <owner> TO <member>;` por item de `memberships()`, na ordem de `ROLES` e depois de
   `member_of`;
8. linha em branco, duas linhas de comentário e o bloco de `public` — revogação de `PUBLIC` e concessão a
   `MIGRATOR`, nesta ordem; o texto exato é o de `spec.md` §4.

Implementação: `String` com `push_str`/`writeln!` (nunca `format!` aninhado). O golden de
`tests/golden/roles.sql` é comparado com `assert_eq!(render(), include_str!(…))`.

## 5. Regras

| Regra | Por que |
|---|---|
| atributo dentro do `CREATE ROLE` | `apply` nunca altera papel existente (`ADR-DB-0001`, item 3) |
| `GRANT` fora do `DO` | `IN ROLE` só vale na criação; `GRANT` é idempotente |
| nenhuma senha, nenhum `GRANT ON` nos schemas da série, nenhum `ALTER`/`DROP` | senha é do ambiente; privilégio da série é do `0001`; `public` é o bloco fixo de §4 |
| determinismo total | sem `HashMap`, sem `std::time`, sem iteração não ordenada |
| `memberships()` deriva de `ROLES` | uma fonte: editar a tabela muda o SQL e o golden falha |

## 6. Testes (no próprio arquivo `#[cfg(test)]`)

| Teste | O que prova |
|---|---|
| `spec_table_is_consistent` | nomes únicos, prefixo `wyd_`, só `owner` é `NOLOGIN`, `member_of` de um nível aponta para papel existente, sem auto-membership |
| `render_matches_golden` | igual ao golden de §4 |
| `render_is_stable` | duas chamadas iguais; sem `\r`; termina em um `\n` |
| `every_role_is_guarded_and_created` | 5 `IF NOT EXISTS` e 5 `CREATE ROLE` |
| `migrator_is_member_of_owner` | o `GRANT wyd_owner TO wyd_migrator;` existe |
| `render_emits_no_forbidden_sql` | sem `PASSWORD`, `ALTER ROLE`, `DROP`; `GRANT … ON` só no bloco de `public` |
| `render_emits_public_block` | o bloco de `public` vem depois das memberships e na ordem de `spec.md` §4 |

## 7. Pronto quando

- `render()` não recebe argumento e não lê nada externo;
- editar `ROLES` sem atualizar o golden quebra teste **e** o `assert!` de tamanho quebra o build;
- o texto gerado é byte a byte o de `spec.md` §4.

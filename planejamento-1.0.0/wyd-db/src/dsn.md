> Pergunta: "como uma DSN do `wyd-db` é validada e por que ela não vaza em log?"
> Público: Desenvolvedor que for implementar `crates/wyd-db/src/dsn.rs`
> Dono: equipe do `wyd-db`
> Base: `docs/arquitetura/seguranca.md`, `src/provision/spec.md` §3 (`M24`)
> Revisado em: 2026-11-18

**Estado:** aceito (2026-11-24) — implementado em `crates/wyd-db/src/dsn.rs` na mesma data: `cargo test`
roda **6** testes de unidade no próprio arquivo (parse, redação e `as_str`), nenhum contra PostgreSQL.

# `Dsn` — o tipo único de string de conexão

## 1. Responsabilidade

Carregar, validar e redigir uma string de conexão. Existe **uma vez no crate**, em `src/dsn.rs`:
`migrate` e `provision` usam este tipo e nenhum dos dois define o seu. A regra de redação tem, por isso,
um dono só — e quem a muda muda em um lugar.

**Não é:** leitor de ambiente, resolvedor de default, construtor de pool ou logger. É um valor com
invariante, como `Version`/`Slug` no runner.

## 2. API pública

```rust
#[derive(Clone)]
pub struct Dsn(String);          // campo privado: só o parse constrói

impl FromStr for Dsn {
    type Err = DsnError;
    fn from_str(raw: &str) -> Result<Self, DsnError>;
}

impl Dsn {
    pub fn as_str(&self) -> &str;   // para quem abre pool; nunca para log
}

impl fmt::Debug   for Dsn { /* escreve `Dsn(<redacted>)` */ }
impl fmt::Display for Dsn { /* idem, redigido */ }

#[non_exhaustive]
pub enum DsnError {
    Empty,    // vazia ou só espaço
    Scheme,   // nem postgres:// nem postgresql://
}
```

`DsnError` é o **único** erro deste arquivo e não tem campo: a mensagem nomeia o problema (vazia,
esquema), nunca repete a entrada — o valor ainda pode conter senha. `InvalidDsn { detail }` de
`provision::error` é outro tipo, montado pelo adapter a partir de um `sqlx` erro de parse, e não
substitui estas duas variantes.

`Debug` e `Display` são **escritos à mão**, não derivados. Derivar `Debug` sobre a `String` imprime a
senha, e `tracing::warn!("{dsn:?}")` é exatamente o caminho que um `derive` percorreria sem ninguém
notar.

## 3. Invariantes

| Invariante | Consequência |
|---|---|
| a string crua só sai por `as_str()` | nenhum `Debug`, `Display` ou `Serialize` a expõe |
| vazia ou só espaço é erro de parse | `DsnError::Empty`, não uma conexão que falha três camadas depois |
| esquema diferente de `postgres://`/`postgresql://` é erro | `DsnError::Scheme`, com mensagem que não ecoa a entrada |
| o erro não carrega a DSN | mensagem nomeia o problema (vazia, esquema), nunca o valor |

Não existe `Dsn::default()` nem `Dsn::new(&str)` que aceite qualquer texto: o único construtor é o
parse, e é o que mantém o invariante verdadeiro em todo o crate.

## 4. O que esta página não decide

| Assunto | Onde |
|---|---|
| qual variável traz cada DSN, e que não há default | `src/provision/spec.md` §3 (`M24`) |
| quem lê o ambiente | binário `wyd-db`; o boot recebe `MigrateConfig` pronto (`src/migrate/spec.md` §9) |
| senha, rotação, `pg_hba` | `src/provision/spec.md` §7 e o ambiente |
| papel esperado pelo preflight | `src/migrate/spec.md` §7 (`EXPECTED_ROLE`) |
| parâmetros de pool | `docs/guia-desenvolvedor/pool-e-transacoes.md` (dono dos valores; `docs/guia-administrador/configuracao.md` §3) |

`Dsn` não sabe de papel: o papel é do preflight, e a DSN só aponta para o banco.

## 5. Testes (Definition of Done)

| Caso | Esperado |
|---|---|
| `postgres://u:segredo@h:5432/db` | parse ok; `{:?}` e `{}` contêm `<redacted>` e **não** contêm `segredo` |
| `postgresql://…` | parse ok: os dois esquemas são aceitos |
| `""` e `"   "` | `DsnError::Empty` |
| `mysql://u@h/db` | `DsnError::Scheme` |
| duas DSNs com senhas diferentes | `format!("{dsn:?}")` idêntico nas duas |
| `as_str()` | devolve exatamente o que entrou (sem normalizar) |

O teste de redação é o único que importa para segurança: se ele falhar, todo log do crate vira
vazamento. Por isso ele compara a saída formatada com a senha, e não só com o rótulo.

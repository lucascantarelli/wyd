> Pergunta: "que configuração o runner aceita, o que é constante do desenho e o que é validado na construção?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/migrate/config.rs`
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §7, §9 e §10
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — arquivo inexistente; especificação a implementar.

# `src/migrate/config.rs` — configuração e constantes

## 1. Responsabilidade

Guardar os três — e só três — valores que o runner aceita (`dsn`, `dir`, `lock_timeout`), validar o
único que tem faixa (o timeout) e fixar as duas constantes do desenho (papel esperado e versão de
servidor). Não lê ambiente: quem lê é o binário.

## 2. API pública

```rust
use std::path::{Path, PathBuf};
use std::time::Duration;
use crate::dsn::Dsn;
use crate::migrate::error::ConfigError;

/// Papel que aplica a série: um nome, uma fonte (`provision::MIGRATOR`).
pub const EXPECTED_ROLE: &str = crate::provision::MIGRATOR;

/// Única versão de servidor aceita pelo preflight.
pub const SERVER_MAJOR: u8 = 17;

/// Piso e teto do `lock_timeout` aceito.
pub const LOCK_TIMEOUT_MIN: Duration = Duration::from_secs(1);
pub const LOCK_TIMEOUT_MAX: Duration = Duration::from_secs(600);
pub const LOCK_TIMEOUT_DEFAULT: Duration = Duration::from_secs(10);

/// Campos privados: só `new`/`with_lock_timeout` constroem.
pub struct MigrateConfig {
    dsn: Dsn,
    dir: PathBuf,
    lock_timeout: Duration,
}

impl MigrateConfig {
    pub fn new(dsn: Dsn, dir: PathBuf) -> Self;                 // timeout = DEFAULT
    pub fn with_lock_timeout(self, d: Duration) -> Result<Self, ConfigError>;
    pub fn dsn(&self) -> &Dsn;
    pub fn dir(&self) -> &Path;
    pub fn lock_timeout(&self) -> Duration;
}
```

`MigrateConfig` **não** deriva `Debug` por conta própria: implementa `fmt::Debug` manual imprimindo
`dsn: <redacted>` (delegando ao `Debug` de `Dsn`, que já redige) e o resto normal. `Clone` é permitido;
`PartialEq` não (compara DSN por igualdade de segredo é convite a log).

## 3. Regras

| Regra | Detalhe |
|---|---|
| validação única | `with_lock_timeout` aceita `1 s ..= 600 s`, inclusive nas bordas |
| fora da faixa | `ConfigError::TimeoutOutOfRange { given: Duration }`; **não** corrige em silêncio |
| sem `from_env` | `config.rs` não importa `std::env`; DSN e diretório vêm prontos |
| sem default de DSN | `Dsn` não tem `Default`; ausência é erro do binário que nomeia a variável |
| constantes imutáveis | `EXPECTED_ROLE` e `SERVER_MAJOR` mudam por commit + revisão de ADR, nunca por ambiente |
| `dir` não é validado aqui | existência/legibilidade é `Source::load` (spec §7); validar duas vezes criaria duas mensagens |

## 4. Erros

`ConfigError` tem duas variantes e **nenhuma delas é definida aqui**:

| Variante | Quem produz | Campos |
|---|---|---|
| `TimeoutOutOfRange` | `with_lock_timeout`, este arquivo | `given: Duration` |
| `MissingEnv` | o **binário**, ao montar a config | `name: &'static str` |

`ConfigError` mora em `error.rs` (junto de `MigrateError::Config`) e é reexportado por `config`? **Não**:
o caminho é `crate::migrate::error::ConfigError`, um dono só. `MissingEnv` existe no enum porque a
mensagem "variável ausente nomeia a variável" (spec §9) é contrato do módulo, e o binário só a formata.

## 5. Dependências (camada)

Permitido: `core`, `std` de tipos, `crate::dsn` (tipo) e `crate::provision::MIGRATOR` (constante).
Proibido: `sqlx`, `tokio`, `std::fs`, `std::env`, `port`, `adapter` (spec §10).

## 6. Testes

| Teste | Entrada | Esperado |
|---|---|---|
| `new_uses_default_timeout` | `new(dsn, dir)` | `lock_timeout() == 10 s`, `dir()` igual ao informado |
| `timeout_accepts_bounds` | `1 s` e `600 s` | `Ok` |
| `timeout_rejects_below_min` | `999 ms` | `Err(TimeoutOutOfRange)` |
| `timeout_rejects_above_max` | `601 s` | `Err(TimeoutOutOfRange)` |
| `timeout_is_settable` | `30 s` | getter devolve `30 s` |
| `debug_redacts_dsn` | `format!("{cfg:?}")` | não contém a senha da DSN de teste |

## 7. Pronto quando

- nenhum literal `10`/`600`/`17` solto fora deste arquivo;
- `EXPECTED_ROLE` é o **mesmo** símbolo usado pelo provisionamento (teste de compilação, não de string);
- os seis testes acima passam sem PostgreSQL e sem tocar disco.

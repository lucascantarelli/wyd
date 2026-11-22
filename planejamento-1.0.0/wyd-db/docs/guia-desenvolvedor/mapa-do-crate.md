> Pergunta: "o que é cada pasta do crate e quem pode importar quem?"
> Público: Desenvolvedor novo e revisor de PR
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.16; `src/migrate/spec.md` §10; `src/provision/spec.md` §5
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A árvore abaixo é a **planejada** para `crates/wyd-db/src/`; o crate ainda não existe. Ela separa
domínio, portas, orquestração e infraestrutura em `migrate` e `provision`, e mantém o restante do
crate como superfície de repositório e transferência. O objetivo é que `sqlx` e `std::fs` não vazem
para o domínio.

## 1. Árvore comentada

```text
crates/wyd-db/src/
├── lib.rs                 versão de schema e reexports públicos
├── error.rs               erro de topo do crate
├── dsn.rs                 Dsn: parse, invariante e Debug redigido
├── model/                 tipos de pessoa, conta, sessão, personagem, clã e auditoria
├── pool/                  configuração e criação de conexões
├── repo/                  operações de repositório; SQL concentrado em repo/pg.rs
├── transfer/              save do personagem e transferência de estado
├── migrate/               runner de migrations
│   ├── migration/         modelo puro: versão, checksum, SQL, arquivo, conjunto e plano
│   ├── port/              Source, Ledger, MigrationTxn/Executor e AdvisoryLock
│   ├── app/               runner: preflight, lock, prepare, apply/down
│   └── adapter/           PgAdapter e FsSource
└── provision/             papéis e estado de public
    ├── domain/            ROLES, observado e plano
    ├── port/              Catalog e Script
    ├── app/               runner: render, plan, check, apply
    └── adapter/           PgProvisionAdapter
```

`model/mod.rs`, `pool/mod.rs`, `repo/mod.rs` e `transfer/mod.rs` são reexport, não páginas próprias;
a árvore de páginas está em `wyd-db-1.0.0.md` §6.16. O diretório `src/migrate/` tem 21 arquivos e
`src/provision/`, 13; não são espelhos do restante do crate.

## 2. Regra de dependência

| Camada | Pode importar | Não pode |
|---|---|---|
| `migration/` | `core` e tipos de `std` | `std::fs`, `std::env`, `port`, `app`, `adapter`, `sqlx`, `tokio` |
| `port/` | `migration` e tipos de `std` | `std::fs`, `std::env`, `app`, `adapter`, `sqlx` |
| `app/` | `migration`, `port`, `error`, `crate::dsn` | `adapter`, `sqlx`, `std::fs`, `std::env` |
| `adapter/` | `migration`, `port`, `sqlx`, `tokio`, `std::fs` | `app` |
| `mod.rs` / `error.rs` | `core`/`std`; reexports da fachada | lógica própria, `sqlx`, `std::env` |

A mesma regra vale para `provision`: `domain/` não conhece banco nem ambiente; `app/` não importa
`adapter/`; as duas portas são `Catalog` e `Script`. Entre módulos irmãos, só constante ou tipo
compartilhado (`crate::provision::MIGRATOR` e `crate::dsn::Dsn`).

## 3. Onde cada coisa mora

| Se você procura | Vá para |
|---|---|
| regra de versão, checksum ou parser da série | `migrate/migration/` |
| contrato de leitura de arquivo, ledger ou lock | `migrate/port/` |
| ordem de preflight, lock, prepare e execução | `migrate/app/runner.rs` |
| SQL e conexão do runner | `migrate/adapter/pg.rs` |
| estado desejado dos papéis | `provision/domain/spec.rs` |
| leitura de `pg_roles` e aplicação de script | `provision/port/` e `provision/adapter/pg.rs` |
| tipo que atravessa o crate | `dsn.rs`, `error.rs` ou `model/` |

## 4. O que esta página não cobre

- **Conteúdo de cada arquivo:** as páginas de `src/` são as donas da API; aqui só há o mapa.
- **Contrato de `model`, `pool`, `repo` e `transfer`:** ainda não existe; a árvore é planejada.
- **Build, features e dependências de Cargo:** não foram decididos; não há `Cargo.toml`.
- **Testes por arquivo:** o mapa de alvos é de [`../guia-teste/index.md`](../guia-teste/index.md).

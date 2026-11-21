> Pergunta: "o que o `provision` considera estado desejado de papéis e do schema `public`, e que arquivo ele gera a partir disso?"
> Público: Desenvolvedor do `wyd-db` que vai implementar `crates/wyd-db/src/provision/domain/spec.rs`
> Dono: equipe do `wyd-db`
> Base: `ADR-DB-0001` (Contexto e Decisão), `padrao-banco.md` §8, `src/migrate/spec.md` §3
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18) — nada disto existe no repositório: nem `crates/wyd-db/src/provision/`,
nem `db/roles.sql`. A página é a especificação a implementar, não a descrição de código vigente.

# Estado desejado dos papéis e de `public` — desenho

Este documento define o módulo `crates/wyd-db/src/provision/` **do zero**. Ele é autocontido: quem
implementar só precisa daqui — o estado desejado, o texto gerado byte a byte, os invariantes e os testes.
O que é vizinho aparece com a assinatura exata, sem "veja o módulo ao lado".

## 1. Responsabilidade e anti-objetivos

**Responsabilidade:** fixar em código, de forma verificável pelo compilador, **quais papéis de cluster
existem e como cada um se associa**, mais o **estado do schema `public`** que o ledger do runner
pressupõe, e produzir deles o único texto executável (`db/roles.sql`) que o provisionamento aplica. O
código é a fonte para os papéis; o bloco de `public` é constante do gerador; o `.sql` é artefato gerado.

**Não é:**

- administração de credencial: nenhuma senha, `PASSWORD` ou rotação (§7);
- alteração de papel existente: `apply` só adiciona (`ADR-DB-0001`, Decisão item 3);
- concessão de privilégio nos quatro schemas da série (`content`, `game`, `portal`, `ops`) ou default
  privileges (é `0001-bootstrap.md` §7, `P-18`); `public` é a exceção declarada em §4, porque existe
  antes da série e é onde o ledger mora;
- remoção de papel: ato manual auditado, fora desta página;
- escritor de arquivo: quem escreve `db/roles.sql` é o subcomando `render` da CLI.

**Dependências de `domain/spec.rs`:** nenhuma além de `core`/`std` de tipos. Sem `sqlx`, `tokio`, `std::env`,
`std::fs` e `std::time` — é o que mantém `render()` puro e o golden estável (§8). No módulo, `sqlx`
existe só em `adapter/pg.rs` (§5).

## 2. Estado desejado

| Papel | `login` | `member_of` | Por que existe |
|---|---|---|---|
| `wyd_owner` | `false` | `[]` | Âncora de propriedade (`AUTHORIZATION` dos quatro schemas); nunca conecta |
| `wyd_migrator` | `true` | `["wyd_owner"]` | Aplica a série e cria os objetos; é o dono das tabelas |
| `wyd_game` | `true` | `[]` | Servidor de jogo: escreve `game`, lê `content`, escreve `ops` append-only |
| `wyd_portal` | `true` | `[]` | Portal/BFF: escreve `portal`, lê `content` |
| `wyd_read` | `true` | `[]` | Leitura de operação/BI: lê os quatro schemas, não escreve |

A ordem da tabela **é** a ordem de emissão do SQL e não pode mudar sem regenerar o golden (§4).

**`wyd_admin` fora desta tabela (`M27`, 2026-11-21):** o papel de staff do `wyd-admin` é `proposta`
pendente e **não** entra em `ROLES`, na contagem de cinco nem no golden antes do "sim" do dono; as
opções (forma do papel, `SELECT`, auditoria e arquivo do `GRANT`) estão em
`docs/arquitetura/seguranca.md` §"Proposta — papel `wyd_admin`". Este documento não o antecipa.

```rust
/// Um papel de cluster, com a membership **direta** que ele precisa ter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleSpec {
    pub name: &'static str,                   // sempre prefixado por `wyd_`
    pub login: bool,                          // true → LOGIN, false → NOLOGIN
    pub member_of: &'static [&'static str],   // membership direta, um nível, sem repetição
}

/// Os cinco papéis da série 1.0.0, na ordem de emissão do SQL.
pub const ROLES: &[RoleSpec];

/// Nome do papel que possui os schemas.
pub const OWNER: &str;      // "wyd_owner"

/// Nome do papel que aplica a série — o único aceito pelo preflight do runner.
pub const MIGRATOR: &str;   // "wyd_migrator"

/// Membership direta esperada, como pares `(dono, membro)`.
pub fn memberships() -> impl Iterator<Item = (&'static str, &'static str)>;

/// Renderiza o conteúdo completo de `db/roles.sql`. Determinístico: mesma entrada, mesmo `String`.
#[must_use]
pub fn render() -> String;
```

`ROLES` é literal de `const`, não função: o estado desejado tem que existir mesmo se `render()` for
removido. `RoleSpec` **não** é `#[non_exhaustive]` de propósito — impediria a construção por literal que
o `const` exige. `MIGRATOR` é a mesma constante que o runner usa como `EXPECTED_ROLE`
(`src/migrate/spec.md` §9): um nome de papel, uma fonte — e por isso o root a reexporta
(`crate::provision::MIGRATOR`), que é o caminho citado pelo runner.

`const _: () = assert!(ROLES.len() == 5);` acompanha a tabela: acrescentar ou remover papel sem atualizar
o golden quebra a compilação antes de qualquer teste rodar.

**O bloco de `public` não deriva de `ROLES`.** `render()` emite, depois das memberships, um bloco fixo
(§4): o `CREATE` é revogado de `PUBLIC` e `USAGE` e `CREATE` são concedidos a `MIGRATOR`. Ele é
constante do gerador, não dado de `RoleSpec`, porque não descreve papel nenhum; `plan`/`check` continuam
olhando só `pg_roles`/`pg_auth_members`, e a conferência desse bloco é pelas provas de
`migrations/0001-bootstrap.md` §9 — `check` não lê ACL de schema.

## 3. DSN por papel (`M24`, decidido em 2026-11-18)

`domain/spec.rs` define **papéis**; quem conecta é o binário, e cada conexão tem a sua variável. Uma DSN por
papel, prefixo `WYD_DB_URL_`, **nenhuma com default**, nenhuma lida por este módulo:

| Variável | Papel que conecta | Quem lê |
|---|---|---|
| `WYD_PROVISION_DSN` | admin do cluster (`CREATEROLE`) e dono do banco | binário `wyd-db`, só nos subcomandos de `provision` |
| `WYD_DB_URL_MIGRATOR` | `wyd_migrator` | runner de migrations |
| `WYD_DB_URL_GAME` | `wyd_game` | servidor de jogo |
| `WYD_DB_URL_PORTAL` | `wyd_portal` | portal/BFF |
| `WYD_DB_URL_READ` | `wyd_read` | operação/BI |
| `WYD_DB_URL_TEST` | `wyd_migrator` em banco descartável | suíte contra PG real |

A posse do banco entra na exigência por causa do bloco de `public` (§4): no PG15+ o schema `public`
pertence a `pg_database_owner`, e revogar `CREATE` de `PUBLIC` ou conceder a `wyd_migrator` exige ser o
dono do banco (ou superusuário) — `CREATEROLE` sozinho não basta. Em desenvolvimento a DSN é a do
superusuário do container, que satisfaz as duas condições.

A DSN é `crate::dsn::Dsn` (`src/dsn.md`), o tipo único do crate: parse e `Debug` redigido moram lá,
uma vez. O módulo não define `Dsn`, não lê ambiente e não escolhe a DSN de nenhum papel.

**Não existe `DATABASE_URL` na 1.0.0.** Um nome único para cinco papéis é como um superusuário entra em
runtime sem ninguém notar; o preflight do runner recusa conexão que não seja `wyd_migrator`. DSN ausente
= erro que **nomeia** a variável, nunca `localhost` silencioso. `WYD_MIGRATIONS_DIR` (diretório da
série; **único com default: `/migrations`**, o caminho que as imagens copiam) e `WYD_ENV_FILE` (caminho
do arquivo de ambiente) são **dado de instalação**, não segredo — e `WYD_ENV_FILE` só é lido pelo
binário, nunca por `domain/spec.rs` nem pela biblioteca. O par do legado, `DATABASE_URL` e
`MIGRATIONS_DIR`, é evidência do desenho antigo, não herança da 1.0.0.

**Quem herda esta seção (e cita, não duplica):** `guia-desenvolvedor/como-adicionar-migration.md`
(comandos e pré-requisitos), `planos/plano-de-implantacao.md` (ordem do ambiente),
`arquitetura/seguranca.md` (tabela de segredos) e `guia-teste/testes-de-sistema.md` (só
`WYD_DB_URL_TEST`). Nenhuma repete a tabela: cada uma diz **qual variável usa e para quê**, e aponta
para cá.

Uma exceção, e ela é **opt-out do preflight**, não DSN: `WYD_MIGRATE_ALLOW_SUPERUSER=1` libera conexão
que não seja `wyd_migrator` — só em desenvolvimento e sempre logado (`ADR-DB-0001`, Decisão item 6). A comparação
de `current_user` e o motivo de ela ser estrita estão em `src/migrate/spec.md` §7.

## 4. Arquivo gerado (conteúdo exato)

Este bloco é o conteúdo de `db/roles.sql` e, literalmente, o golden dos testes de §8. Sem linha em branco no fim
além da última `\n`; sem `\r`; UTF-8; indentação de dois espaços dentro do `BEGIN`/`END`.

```sql
-- db/roles.sql — GERADO por `wyd-db provision render`. Não editar à mão.
-- Papéis do cluster e o schema `public` deste banco: uma vez por banco, reexecutar é no-op. Não é migration.
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'wyd_owner') THEN
    CREATE ROLE wyd_owner NOLOGIN;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'wyd_migrator') THEN
    CREATE ROLE wyd_migrator LOGIN;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'wyd_game') THEN
    CREATE ROLE wyd_game LOGIN;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'wyd_portal') THEN
    CREATE ROLE wyd_portal LOGIN;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'wyd_read') THEN
    CREATE ROLE wyd_read LOGIN;
  END IF;
END $$;

-- membership separada: `IN ROLE` só vale na criação do papel
GRANT wyd_owner TO wyd_migrator;

-- `public` existe com o banco, antes da série, e é onde o runner cria o ledger: o estado dele é de
-- ambiente, não de migration. `PUBLIC` não escreve em `public`; `wyd_migrator` atravessa e cria ali.
REVOKE CREATE ON SCHEMA public FROM PUBLIC;
GRANT USAGE, CREATE ON SCHEMA public TO wyd_migrator;
```

Regras de emissão, todas verificadas em teste:

| Regra | Consequência no texto |
|---|---|
| um `IF NOT EXISTS … pg_roles` por papel | reaplicar em cluster existente é no-op |
| atributo dentro do `CREATE ROLE`, nunca `ALTER ROLE` | `apply` não altera papel existente (`ADR-DB-0001`, Decisão item 3) |
| membership fora do `DO` block, em `GRANT` | `IN ROLE` só valeria na criação; o `GRANT` é idempotente |
| nenhuma senha, nenhum privilégio de schema da série, nenhum `DROP` | senha é do ambiente (§7); `GRANT` nos quatro schemas é do `0001`; o único privilégio aqui é o de `public`, fixo em §2 |
| bloco de `public` depois das memberships | é estado de ambiente: fica no fim para o diff de papel continuar no topo do arquivo |
| ordem = ordem de `ROLES`, depois memberships | golden estável, diff de PR legível |

## 5. Arquitetura do módulo

Treze arquivos em quatro pastas (contando os `mod.rs`), com o mesmo desenho do runner
(`src/migrate/spec.md` §10): modelo puro, portas, orquestração e um adapter. O root fica com o que é
de todos — fachada, configuração e erro — como lá.

```text
src/provision/
├── mod.rs              fachada: run(Command, &B) + reexport público (sem I/O)
├── error.rs            ProvisionError + Result (thiserror)
│
├── domain/             modelo puro — não conhece banco, arquivo nem ambiente
│   ├── mod.rs          reexport do que as camadas de cima usam
│   ├── spec.rs         RoleSpec, ROLES, OWNER, MIGRATOR, render()
│   ├── observed.rs     ObservedRole: o que o `Catalog` leu do cluster
│   └── plan.rs         observado × ROLES → Plan, Step, Drift
│
├── port/               contratos; traits pequenas, sem sqlx
│   ├── mod.rs
│   ├── catalog.rs      Catalog (lê pg_roles/pg_auth_members)
│   └── script.rs       Script (aplica o texto de render())
│
├── app/                orquestração — depende de domain + port, nunca de adapter
│   ├── mod.rs
│   └── runner.rs       Runner<B>::run(Command): render | plan | apply | check
│
└── adapter/            infraestrutura — único lugar com sqlx
    ├── mod.rs
    └── pg.rs           PgProvisionAdapter: from_dsn, Catalog, Script
```

Uma pasta por camada, um arquivo por contrato — `port/` tem dois, o runner tem cinco. `domain/`
existe porque três arquivos puros não pertencem ao root: `spec.rs` é o estado desejado, `observed.rs` o
que veio do cluster e `plan.rs` a decisão; nenhum deles pode ser confundido com fachada.

**Regra de dependência:**

| Camada | Pode importar | Não pode |
|---|---|---|
| `domain/` | `core`; `std` de tipos | `std::env`, `std::fs`, `port`, `app`, `adapter`, `sqlx`, `tokio` |
| `port/` | `domain` (só `ObservedRole`) | `std::env`, `std::fs`, `app`, `adapter`, `sqlx` |
| `app/` | `domain`, `port`, `error`, `crate::dsn` | `adapter`, `sqlx`, `std::fs`, `std::env` |
| `adapter/` | `domain`, `port`, `sqlx`, `tokio` | `app` |
| `mod.rs`, `error.rs` | `core`/`std` (error); todas para reexportar (mod) | lógica própria no `mod.rs`, `sqlx`, `std::env` |

```rust
// domain/observed.rs — o retrato do cluster, sem I/O
#[non_exhaustive]
pub struct ObservedRole {
    pub name: String,
    pub login: bool,
    pub member_of: Vec<String>,
}

// domain/plan.rs — a decisão, sem I/O
#[non_exhaustive]
pub enum Step {
    CreateRole { name: &'static str, login: bool },
    GrantMembership { owner: &'static str, member: &'static str },
}

#[non_exhaustive]
pub enum Plan {
    UpToDate,
    Apply(Vec<Step>),            // na ordem de `ROLES`, depois memberships
    Drift(Vec<Drift>),           // o que `apply` não corrige
}

/// A única divergência que `apply` não resolve é `login`: `CHECK`, senha e membership de terceiros são
/// do ambiente. Papel extra em `pg_roles` é ignorado — `provision` não remove.
#[non_exhaustive]
pub struct Drift { pub role: String }

#[must_use]
pub fn plan(observed: &[ObservedRole]) -> Plan;

// port/catalog.rs e port/script.rs — contratos, sem sqlx
/// Leitura do cluster: `pg_roles` + `pg_auth_members`. Nunca escreve.
pub trait Catalog: Send + Sync {
    async fn observed(&self) -> Result<Vec<ObservedRole>, CatalogError>;
}

/// Aplica o texto de `render()` como um script único, numa sessão: sem lock e sem transação explícita.
pub trait Script: Send + Sync {
    async fn apply(&self, sql: &str) -> Result<(), ScriptError>;
}

// mod.rs — fachada, tudo injetado
// `Command::Render` só devolve texto: comparar com `db/roles.sql` é I/O de arquivo, e quem faz isso
// (`render --check`) é o binário — `Command::Check` é outra pergunta (drift entre `pg_roles` e `ROLES`).
#[non_exhaustive]
pub enum Command { Render, Plan, Apply, Check }

#[non_exhaustive]
pub enum Report {
    Rendered(String),      // o texto; o binário escreve e/ou compara
    Planned(Plan),         // resposta de `plan` e de `check`; o código de saída é do binário
    Applied { steps: usize },
}

pub async fn run<B>(cmd: Command, backend: &B) -> Result<Report, ProvisionError>
where
    B: Catalog + Script;   // `Send + Sync` são supertraits das portas
```

- `plan` compara `domain::spec::ROLES`/`memberships()` com o que o `Catalog` leu; **não** parseia
  texto SQL e não abre conexão.
- `apply` chama `domain::spec::render()` e entrega o texto ao `Script`, que abre a sessão e envia o
  script inteiro, sem transação explícita: `DO`/`GRANT` são idempotentes, e um erro no meio deixa o que
  já passou aplicado — rodar de novo converge. A orquestração não escreve SQL de controle.
- `check` é `plan` sem `apply`: nunca chama `render()`.
- `render` devolve o texto em `Report::Rendered`; quem escreve `db/roles.sql` — e quem compara com o
  arquivo existente no `--check` — é o binário, que é também o dono dos códigos `0` e `3` (§6).
- Um só `PgProvisionAdapter::from_dsn` serve `Catalog` e `Script`: as duas portas usam o mesmo pool, e o provisionamento não tem lock.

SRP e DIP não têm linha própria porque **são** a regra de dependência acima: cada arquivo tem uma
responsabilidade, e as setas apontam para dentro. A tabela fica com o que a tabela de camadas não
expressa.

| Princípio | Como aparece |
|---|---|
| **OCP** | comando novo é variante de `Command` (`#[non_exhaustive]`); `Step` ganha variante, não `if` |
| **ISP** | duas portas pequenas: leitura (`Catalog`) e escrita (`Script`) não compartilham trait |
| **Clean code** | estado desejado em `const`, não em função; `#[must_use]` em `render`/`plan`; erro nomeia o papel |

**`async fn` em trait.** Como no runner, as duas portas declaram `async fn` e o `run` é aguardado com
`block_on` (binário e teste de sistema), nunca por `tokio::spawn`; o lint `async_fn_in_trait` fica
silenciado em `port/mod.rs` com `#![allow(async_fn_in_trait)]` e o motivo escrito lá.

**Regra de `#[non_exhaustive]`:** todo tipo público que pode crescer o leva — `Command`, `Plan`, `Step`,
`Report`, `ObservedRole`, `Drift`. A exceção é `RoleSpec`: o `const ROLES` o constrói por literal (§2),
então o atributo impediria a única forma de declarar o estado desejado.

## 6. Erros e códigos de saída

`domain/spec.rs` **não devolve erro**: `render()` é infalível por construção — a entrada é uma constante
compilada, e o golden prova a saída. Nenhum `Result` atravessa esta fronteira.

| Código | Situação | Dono |
|---:|---|---|
| 0 | `render` sem `--check`, ou `render --check` sem divergência | subcomando |
| 2 | configuração ou conexão (DSN ausente nomeia a variável) | binário |
| 3 | `render --check` com divergência entre `db/roles.sql` e o renderizado | subcomando |
| 4 | `plan`/`check` com drift entre `pg_roles` e `ROLES` | `ADR-DB-0001`, Decisão item 5 |

Código novo é decisão registrada, não `catch-all`. `domain/spec.rs` só participa dos códigos 0 e 3, e por
intermédio do subcomando — o módulo em si não formata saída.

## 7. Credenciais

`domain/spec.rs` define **papéis**, não credenciais. Nenhuma senha entra em `ROLES`, no SQL gerado ou no
golden: papel `LOGIN` sem senha só conecta se o `pg_hba.conf` do ambiente permitir, e quando houver
senha o valor é definido no ambiente e rotacionado sem migration.

| O que | Onde vive | Regra |
|---|---|---|
| Nome e atributo do papel | `ROLES`, neste arquivo | versionado; vira `db/roles.sql` por `render` |
| Senha do papel `LOGIN` | ambiente / cofre | nunca no git, nunca no SQL gerado, nunca em log |
| DSN (`WYD_DB_URL_*`, `WYD_PROVISION_DSN`) | ambiente / cofre | **segredo**: a senha viaja dentro; nunca logada em claro, nunca no artefato gerado |
| Autenticação permitida | `pg_hba.conf` do ambiente | fora do escopo desta página |
| Rotação | operação | `ALTER ROLE … PASSWORD` fora da série; `render` não a toca |

O que `render()` **não** pode emitir está em §4 e é provado por teste em §8; aqui a regra é só sobre
onde o segredo vive. Provisionar papel não é administrar credencial.

Não existe `down`: nada reverte papel junto com o banco, e `--reset` derruba schema e ledger, **nunca**
papel (`ADR-DB-0001`).

## 8. Testes (Definition of Done do módulo)

| Invariante | Como é garantido |
|---|---|
| exatamente cinco papéis | `assert!` em `const` (§2), verificado no build |
| nomes únicos, prefixo `wyd_`, só `wyd_owner` é `NOLOGIN`, membership de um nível | `spec_table_is_consistent` |
| todo `member_of` aponta para papel existente, sem auto-membership | `spec_table_is_consistent` |
| `render()` é determinístico e não emite `\r` | `render_is_stable` |
| o texto emitido é exatamente o golden de §4 | `render_matches_golden` |
| nenhum SQL proibido; cada papel guardado e criado | `render_emits_no_forbidden_sql`, `every_role_is_guarded_and_created` |
| `wyd_migrator` é membro de `wyd_owner` | `migrator_is_member_of_owner` |
| `render()` não lê ambiente, arquivo nem relógio | dependências de §1: sem `std::env`, `std::fs`, `std::time` |
| o bloco de `public` é exatamente o de §4, depois das memberships | `render_matches_golden` |

| Nível | O que prova |
|---|---|
| unidade | os seis testes acima, no `#[cfg(test)] mod tests` do próprio arquivo; golden em `tests/golden/roles.sql` via `include_str!` |
| sistema | `render` → `db/roles.sql` → `git diff --exit-code` limpo; `plan`/`apply` contra PG17 com `WYD_DB_URL_TEST` aplicando duas vezes sem mudança |
| proibido | teste que lê ambiente; golden gerado na hora (`render()` contra si mesmo); `SKIP` por variável ausente (falha nomeando) |

## 9. O que esta página não decide

| Assunto | Onde é decidido |
|---|---|
| `plan`/`apply`/`check`, `WYD_PROVISION_DSN`, códigos de saída, execução do script numa sessão | `ADR-DB-0001`, Decisão itens 3–6 |
| Preflight do runner (`current_user`, papéis, membership) | `ADR-DB-0001`, Decisão item 6 |
| Por que papel fica fora da série versionada | `ADR-DB-0001` (Contexto e Alternativas) |
| Senha, `pg_hba`, TLS, rotação | §7 desta página (regra) + ambiente (valor) |
| Remoção de papel | ato manual auditado: `DROP OWNED BY` antes, `DROP ROLE` dos que têm `member_of` e `wyd_owner` por último (`2BP01` enquanto houver dependência) |
| `GRANT` e default privileges nos quatro schemas da série | `migrations/0001-bootstrap.md` §7 (`P-18`) |
| Estado do schema `public` (ACL do ledger) | §4 desta página; decisão em `ADR-DB-0001`, item 8 |
| Quem conecta com cada papel em produção | operação / `docs/rust/05-stack-docker.md` |

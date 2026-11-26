> Pergunta: "como provar a porta `PgAdapter` do runner contra PG17 — publicando a porta do container ou rodando o teste dentro da rede do compose — e o que cada caminho fecha em `CAP-37`/`CAP-38`?"
> Público: Desenvolvedor do `wyd-db`, arquitetura e revisor de CI
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §13; `migrations/serie-e-regras.md` §6 e §9; `tests/specs/capacidades.md`; `.github/workflows/code.yml`
> Revisado em: 2026-11-25

**Estado:** decidido e implementado (2026-11-25) — `Q-ALVO-1` = A1, `Q-ALVO-2` = B1 e `Q-ALVO-3` =
`wyd_migrator` com `LOGIN` só no teste. O alvo `runner_pg` existe, passou **5/5** em PG17 (log
`/tmp/wyd-runner-pg.log`) e fechou `CAP-37`; a página fica como registro da comparação e das decisões.

Esta página propõe **um alvo de teste novo** (`runner_pg`) que exercite o `PgAdapter` real com
`apply`/`status`/`down` contra PostgreSQL 17, e compara as duas formas de o teste alcançar o banco.
Ela não declara o alvo no `Cargo.toml`, não escreve teste e não muda workflow: quem decide é o dono,
nas perguntas do §6.

## 1. Por que o alvo existe

Hoje há duas provas parciais e nenhuma do meio:

| Prova | Como | O que cobre | O que não cobre |
|---|---|---|---|
| `tests/sistema_pg.rs` | `psql` por `docker exec` (`V`, 2026-11-25) | contrato do schema: constraint, índice, privilégio, view, FK | a porta: ledger, checksum, transação por arquivo, lock |
| `tests/reversibilidade_pg.rs` | `psql` por `docker exec` (executado contra PG17 em 2026-11-25; log `/tmp/wyd-reversibilidade.log`) | ciclo `up → down → up` da série materializada (`V`: 3/3) | `PgAdapter`, ledger e advisory lock |
| `src/migrate/app/runner.rs` (`#[cfg(test)]`) | portas falsas (`V`, 63 testes em `--lib`) | ordem de preflight, plano, drift, `down`, falha no meio | o adapter real: `PgAdapter`, `PgTxn`, `pg_advisory_lock` |

O elo que falta é entre o SQL provado pela série e a porta do runner: os `down` e o ciclo SQL passaram,
mas os critérios do `PgAdapter` continuam sem execução contra banco. O alvo `runner_pg` proposto fecha
esse elo: monta
`MigrateConfig` + `PgAdapter` + `FsSource` e executa os mesmos comandos que o binário vai executar.

## 2. O que o alvo prova

Cada item abaixo é verificável pela API que já existe (`V`, 2026-11-25):

- **`apply` idempotente:** segundo `apply` devolve `UpToDate`, não emite `BEGIN` e não altera o schema.
- **Ledger real:** `prepare` cria `public.schema_migrations`, `record` grava versão/slug/checksum e
  `validate_ledger_shape` recusa coluna a mais ou `CHECK` removida.
- **Transação por arquivo:** arquivo que falha no meio não commita nada (`PgTxn::commit` só no fim).
- **`search_path = ''`:** toda instrução da série precisa ser qualificada; o que não for falha aqui e
  não no `psql`.
- **Drift:** um byte alterado no `up` ou no `down` de arquivo aplicado produz `Drift(Checksum)`;
  renomear produz `Drift(Slug)`; apagar produz `Drift(MissingFile)`.
- **`down --to`** e `down --to 0`: desfaz em ordem decrescente e apaga a linha do ledger (`forget`).

## 3. Dimensão A — como o teste alcança o banco

As duas opções executam o **mesmo código** de teste; muda só o caminho até o PostgreSQL.

### A1 — publicar a porta do container

`docker run -p 127.0.0.1:PORT:5432 postgres:17` (ou `ports:` no `docker-compose.yml`), e o teste roda
no host com `cargo test`, como os alvos atuais.

| Item | Leitura |
|---|---|
| Mudança necessária | uma linha no serviço/comando do banco e a montagem do `Dsn` a partir da porta (`V`: `MigrateConfig::new(Dsn, PathBuf)` e `PgAdapter::from_config` já existem) |
| Ferramental novo | nenhum: sem imagem Rust, sem toolchain duplicada, sem cache de build |
| Tempo de CI acrescentado | o do próprio teste (`H`, não medido; o `apply` são 28 arquivos em um container local) |
| Risco | colisão de porta na máquina de quem desenvolve; mitigável com `-p 127.0.0.1:0:5432` e leitura de `docker port` |
| Exposição | `127.0.0.1:PORT` fica restrito ao host; **nunca** `0.0.0.0`, que abre o banco na rede local e dispara o firewall no Windows |
| Fidelidade a produção | alta no transporte: produção usa `Dsn` TCP, não socket nem `docker exec` |

### A2 — rodar o teste dentro da rede do compose

Um serviço de teste com imagem Rust entra na mesma rede do banco e alcança `db:5432` pelo nome do
serviço; o `cargo test` roda dentro do container.

| Item | Leitura |
|---|---|
| Mudança necessária | serviço novo no compose, imagem Rust fixada, `depends_on` com healthcheck, volume do repositório e do `target/` |
| Ferramental novo | toolchain Rust na imagem, versão a manter em dia com a do host (`H`) |
| Tempo de CI acrescentado | pull da imagem e build dentro do container; o build frio é o custo dominante (`H`, não medido) |
| Risco | `target/` compartilhado entre host Windows e container Linux não é reutilizável; o cache precisa de estratégia própria (BuildKit ou registry) |
| Exposição | nenhuma porta publicada — o banco só existe na rede interna |
| Fidelidade a produção | menor: produção não compartilha rede de container com o banco |

### Comparação direta

| Critério | A1 — porta publicada | A2 — rede do compose |
|---|---|---|
| Custo de implementação | baixo (uma linha + `Dsn`) | médio-alto (serviço, imagem, cache, healthcheck) |
| Custo por execução | só o teste | pull + build + teste |
| Depuração | a mesma de hoje (`cargo test`, `RUST_LOG`) | dentro do container; `docker compose run` e logs |
| Superfície exposta | loopback do host | nenhuma |
| Duplicação de ferramental | nenhuma | toolchain e versão de Rust duplicadas |
| Prova de `CAP-37` | sim | sim |

**Recomendação: A1**, com `-p 127.0.0.1:PORT:5432`. O que A2 compra é "nenhuma porta no host", e o
preço é uma segunda toolchain e um cache de build que o projeto não tem hoje; o banco de teste não tem
dado e vive segundos. Se a política do projeto proibir porta publicada mesmo em loopback, A2 é o
caminho — a decisão é do dono, não desta página.

### Detalhe que vale para as duas

O preflight do runner exige `EXPECTED_ROLE = wyd_migrator` e `SERVER_MAJOR = 17` (`V`:
`src/migrate/config.rs`; `preflight` em `app/runner.rs`). A CI cria os papéis como `NOLOGIN` e aplica a
série como `postgres`. Para o alvo, o papel `wyd_migrator` precisa de `LOGIN` (e senha) ou o teste
precisa de `allow_superuser(true)` — que enfraquece o preflight e por isso não é recomendado. É custo
igual nas duas opções e é a primeira coisa a acertar na implementação.

## 4. Dimensão B — que série o alvo aplica

Esta é a restrição que decide se o alvo pode existir **hoje**, e ela é independente da porta.

`MigrationSet::build` exige série contígua a partir de 1 (`V`: `migration/set.rs`, `NotContiguous`).
A série materializada tem **28 arquivos** e **6 versões ausentes** — `0016`, `0017`, `0018`, `0020`,
`0023` e `0027` (`V`, 2026-11-25: `crates/wyd-db/migrations/`). Logo, o runner **não carrega** a série
real hoje: `apply` falha em `SetInvalid(NotContiguous)` antes de tocar o banco.

| Opção | O que aplica | Prova `CAP-37` | Bloqueio |
|---|---|---|---|
| **B1 — fixture contígua** em `tests/resources/` (3–5 arquivos sintéticos) | uma série real para o adapter | **sim**: o lock não depende de qual série é aplicada | nenhum; só não é "a série de 34" |
| **B2 — série materializada** | os 28 arquivos | sim, e prova também os checksums reais | as 6 versões ausentes (seeds e arquivos bloqueados) |

**Recomendação: B1 agora, B2 quando a série fechar.** A série real já tem prova de schema em
`reversibilidade_pg`; o que falta no runner é o adapter, e isso uma fixture contígua prova. B2 sem as
6 versões exigiria inventar arquivo para tapar buraco — o que a curadoria de seed não autoriza.

## 5. O que isso fecha — e o que não fecha

| Capacidade | Enunciado | Fecha com este alvo? |
|---|---|---|
| `CAP-37` | a segunda execução da série espera no advisory lock no máximo **10 s** e falha nomeando a espera (`RNF-DB-002`) | **Sim**, nas duas opções: precisa de dois `apply` concorrentes sobre o mesmo banco; o segundo estoura `lock_timeout` e devolve `LockBusy` (código de saída `3`, `V`: testes do runner). É a única forma de exercitar `pg_advisory_lock` de verdade. |
| `CAP-38` | o p95 da **leitura de personagem** é medido e registrado, sem limiar afirmado (`RNF-DB-004`) | **Não.** É caminho de consulta (`repo`/`pool`, ainda planejado), não de migration. O alvo entrega no máximo o `Dsn` e o container já de pé; a capacidade continua aberta e pertence a outro alvo. |
| `CAP-01`–`CAP-07` (metade runner) | aplicar, reaplicar, drift e `down` | **Sim**, no nível do runner, onde hoje só há portas falsas. |

Dizer que este alvo fecha `CAP-38` seria falso: ele não lê personagem nenhum.

## 6. Perguntas — respondidas em 2026-11-25

1. **`Q-ALVO-1` — transporte:** **A1** — publicar a porta em `127.0.0.1`. É o job `runner` do
   `code.yml`.
2. **`Q-ALVO-2` — série:** **B1** — fixture contígua agora, em `tests/resources/fixture/`.
3. **`Q-ALVO-3` — papel:** **`wyd_migrator` com `LOGIN` só no container de teste**, sem
   `allow_superuser(true)`.

## 7. O que esta página não decide

- **Não declara o alvo:** `Cargo.toml` e `tests/index.md` continuam como estão; a linha `runner_pg` só
  entra depois da resposta às três perguntas.
- **CI da série/schema:** `.github/workflows/code.yml` agora extrai o bloco `up` e executa
  `reversibilidade_pg`; ambos passaram localmente em PG17. A proposta não implementa um alvo pelo
  `PgAdapter` nem resolve a política de nomes/contiguidade da série.
- **`CAP-37` foi fechada por este alvo:** `runner_pg` roda dois `apply` concorrentes e o segundo
  devolve `LockBusy` em 10 s (código 3). `CAP-38` (p95 da leitura de personagem) permanece fora do
  escopo e continua aberta.
- **Não reabre `Q-SERIE-1`:** os 28 `down` e a prova `up → down → up` seguem como decididos e
  implementados; esta proposta trata da porta, não da série.

# WYDDB-002 — congelar contrato público de migrate

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26)
**Aceite:** o dono autorizou a execução e o aceite desta página em 2026-11-26, nesta sessão; a evidência está em §Evidência de execução.

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 1 — higiene e baseline


## Contexto

`Command`, `DownTo`, `Report` e `run` são a fachada que o binário e o boot chamam. Se a assinatura nascer sem teste de contrato, uma mudança de variante quebra o chamador sem aviso.

## Entregável

Fixar a API pública de `src/migrate/mod.rs` e `src/migrate/app/mod.rs` no molde de `src/migrate/spec.md` §8–§10, com `#[non_exhaustive]` nos tipos que podem crescer e `run<B, S>` genérico nas portas.

## DoD

- [x] Um alvo `contrato_api_migrate` compila `Command`, `DownTo`, `Report` e `run` e falha se um campo/variante público mudar sem atualização do teste.
- [x] `status`, `apply` e `down` são representáveis sem `bool` fora do nome do comando.
- [x] `Report` é dado, não string; a formatação fica no binário.
- [x] `src/migrate/mod.md` e `src/migrate/app/mod.md` citam a mesma assinatura.

## Fora de escopo

Não implementar o runner completo nem o adapter; esta issue fecha a fachada e o teste de compilação.

## Risco

`#[non_exhaustive]` mal usado esconde uma variante necessária ao chamador; o teste precisa cobrir os três comandos.

## Depende de

WYDDB-001.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída de `cargo test -p wyd-db --test contrato_api_migrate` e link para o commit da página de módulo.

## Evidência de execução (2026-11-26)

Executado em `crates/wyd-db`:

```text
$ cargo test --test contrato_api_migrate
running 1 test
test contrato_publico_de_migrate_compila ... ok
test result: ok. 1 passed; 0 failed

$ cargo fmt --check
(sem saída; exit 0)

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s

$ cargo test --lib
test result: ok. 63 passed; 0 failed
```

Alvo novo: `crates/wyd-db/tests/contrato/contrato_api_migrate.rs`, registrado em `crates/wyd-db/Cargo.toml`. Ele compila as três variantes de `Command` e seus campos, as duas formas de `DownTo`, os quatro campos públicos de `Report` e `run::<FakeBackend, FakeSource>`, sem PostgreSQL. As páginas `src/migrate/mod.md` §3 e `src/migrate/app/mod.md` §2 citam a mesma assinatura. `Report` é congelado por acesso tipado aos quatro campos (não há construtor público); não há `Display` nem string formatada no teste.

O alvo entrou no `make test` (linha `cargo test --manifest-path crates/wyd-db/Cargo.toml --test contrato_api_migrate --locked`) e na matriz `unit` de `.github/workflows/code.yml` (`args: --lib --test contrato_api_migrate`), espelhada no `code-stable.yml`; não depende de `WYD_DB_TEST_URL` nem do serviço PostgreSQL. A rodada da raiz confirmou a entrada e o gate:

```text
$ make test
     Running tests\contrato_api_migrate.rs (crates\wyd-db\target\debug\build\wyd-db/…)
running 1 test
test contrato_publico_de_migrate_compila ... ok
test result: ok. 1 passed; 0 failed
[… 29 linhas `test result`, 137 passed no total …]
exit 0

$ make gate
gate: PASS (full, 22.8s)
  layout     PASS    0.2s  917 arquivo(s) classificado(s); 0 desvio(s)
exit 0
```

O `actionlint` 1.7.12 (mesma imagem por digest de `security.yml`) sobre `code.yml` e `code-stable.yml` → exit 0.

Revisão de escopo (2026-11-26): o alvo ficou com um único matcher de `Command`/`DownTo`, um item de tipo anônimo para os campos de `Report` e a construção do future de `run`; saíram a enumeração intermediária de `DownTo` e o congelamento acidental de `Version::get`. Nada executa porta ou SQL. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test contrato_api_migrate` (1 passed) e `make gate` verdes.

Não executados: `sistema_pg`, `reversibilidade_pg` e `runner_pg`, que exigem `WYD_DB_TEST_URL` e PostgreSQL 17.

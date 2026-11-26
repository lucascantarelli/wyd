# WYDDB-003 — congelar contrato público de provision

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26)
**Aceite:** o dono autorizou a execução de todas as issues registradas em 2026-11-26, nesta sessão; a evidência fica em §Evidência de execução.

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 1 — higiene e baseline


## Contexto
**Sem arquivo no repositório:** `roles.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

O provisionamento também é chamado pelo binário: `Command`, `Report` e `run<B>` precisam ser estáveis antes de o adapter e o golden de `roles.sql` serem escritos.

## Entregável
**Sem arquivo no repositório:** `provision/app/runner.rs`, `src/provision/mod.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Fixar a API de `src/provision/mod.rs` e `provision/app/runner.rs` com `Command::{Render, Plan, Apply, Check}`, `Report` e `run<B: Catalog + Script>`, conforme `src/provision/spec.md` §5.

## DoD

- [x] Um alvo `contrato_api_provision` compila os quatro comandos e o `Report`.
- [x] `render` devolve texto e não escreve arquivo; a comparação é do binário.
- [x] `plan` e `check` não chamam `Script`.
- [x] `provision/mod.md` e `provision/app/mod.md` citam a mesma assinatura.

## Fora de escopo

Não criar o adapter PostgreSQL, o golden ou o comando de CLI; isso é da fase 7 e do binário.

## Risco

Separar `Plan` de `Report` sem deixar o código de saída dentro da biblioteca.

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

Saída de `cargo test -p wyd-db --test contrato_api_provision` e páginas de módulo atualizadas.

## Evidência de execução (2026-11-26)

Executado em `crates/wyd-db`:

```text
$ cargo test --test contrato_api_provision
running 3 tests
test plan_e_check_leem_o_catalogo_e_nunca_o_script ... ok
test apply_entrega_o_texto_de_render_uma_vez ... ok
test render_devolve_texto_sem_tocar_o_backend ... ok
test result: ok. 3 passed; 0 failed

$ cargo fmt --check
(sem saída; exit 0)

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

Alvo: `crates/wyd-db/tests/contrato/contrato_api_provision.rs`, registrado em `crates/wyd-db/Cargo.toml`. Ele compila `Command::{Render, Plan, Apply, Check}`, `Report::{Rendered, Planned, Applied}` e `run::<Fake>`, e prova: `render` devolve `render()` sem tocar catálogo ou script; `plan`/`check` leem o catálogo e não chamam `Script`; `apply` conta 7 passos no cluster vazio e entrega o texto de `render()` uma vez. O adapter PostgreSQL, o golden de `db/roles.sql` e o CLI não existem nesta entrega — o “Fora de escopo” os atribui à fase 7/`WYDDB-015`.

O alvo entrou no `make test` (`cargo test ... --test contrato_api_provision --locked`) e na matriz `unit` de `.github/workflows/code.yml` (`--test contrato_api_provision`, espelhado em `code-stable.yml`). Rodada da raiz: `make test` → exit 0, **140 passed** em **30** linhas `test result`, com `Running tests\contrato\contrato_api_provision.rs`; `actionlint` 1.7.12 → exit 0; `make gate` → PASS.

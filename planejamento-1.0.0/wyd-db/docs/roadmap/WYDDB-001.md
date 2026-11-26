# WYDDB-001 — fixar lint e documentação pública do crate

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

O crate ainda não existe, mas o contrato de qualidade precisa nascer junto com ele. Sem `missing_docs` e sem baseline de lint, cada arquivo novo entra sem contrato e a revisão vira opinião.

## Entregável

Habilitar `#![deny(missing_docs)]` em `crates/wyd-db/src/lib.rs`, documentar os itens públicos e fixar o baseline de `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` no guia do desenvolvedor.

## DoD

- [x] `cargo doc -p wyd-db --no-deps` termina sem warning de documentação faltante.
- [x] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` passam no commit de referência.
- [x] `guia-desenvolvedor/index.md` §3 deixa de dizer que os três comandos não existem.
- [x] Nenhum `unsafe` novo entra sem bloco de segurança justificado.

## Fora de escopo

Não criar CI real, não definir versão mínima de Rust e não documentar módulo que ainda não tem contrato.

## Risco

O baseline pode virar gate decorativo se só rodar no PR inicial; o checklist de PR precisa cobrar a repetição.

## Depende de

Nenhuma; é a primeira issue da fase 1.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `documentation-writer` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída dos três comandos, diff do `lib.rs` e página do guia atualizada no mesmo PR.

## Evidência de execução (2026-11-26)

Executado em `crates/wyd-db` com Rust 1.100.0:

```text
$ cargo doc -p wyd-db --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s

$ cargo fmt --check
(sem saída; exit 0)

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
```

`lib.rs` recebeu `#![deny(missing_docs)]`; 133 itens públicos (variantes, campos e métodos de porta) foram documentados em 13 arquivos de `src/migrate/`. `cargo test --lib` → **63 passed**.

Arquivos alterados: `crates/wyd-db/src/lib.rs`, `crates/wyd-db/src/migrate/{mod,error}.rs`, `src/migrate/migration/{file,set,plan,version,checksum,applied,sql}.rs`, `src/migrate/port/{unit_of_work,source,lock,ledger}.rs` e `planejamento-1.0.0/wyd-db/docs/guia-desenvolvedor/index.md` §3.

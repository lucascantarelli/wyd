# WYDDB-005 — implementar Dsn com Debug redigido

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26)
**Aceite:** o dono autorizou a execução de todas as issues registradas em 2026-11-26, nesta sessão; o código já existia desde o primeiro slice e a evidência confirma o DoD.

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 2 — contratos e erros tipados


## Contexto

A DSN carrega a senha e seria impressa por `Debug` herdado. O crate precisa de um tipo único que parseie, valide e redija antes de qualquer conexão.

## Entregável

Implementar `src/dsn.rs` com parse, erro tipado e `Debug` que substitui a senha; nenhum `DATABASE_URL` e nenhuma leitura de ambiente na biblioteca.

## DoD

- [x] Teste de parse aceita DSN válida e recusa forma inválida com erro específico.
- [x] `format!("{:?}", dsn)` não contém a senha nem a URL completa.
- [x] `rg -n "DATABASE_URL" crates/wyd-db/src` não encontra leitura.
- [x] A página `src/dsn.md` documenta a garantia e o erro.

## Fora de escopo

Não escolher driver, pool ou política de conexão; esta issue é o tipo e a redação.

## Risco

Um erro de parse pode vazar a entrada; o `Display` também precisa ser testado.

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

Testes de parse/redação e saída do `rg` no PR.

## Evidência de execução (2026-11-26)

Executado em `crates/wyd-db`:

```text
$ grep -rn "DATABASE_URL" src
(sem saída; exit 1)

$ cargo test --lib dsn
running 7 tests
test dsn::tests::parse_ok_and_redacted ... ok
test dsn::tests::postgresql_scheme_is_accepted ... ok
test dsn::tests::empty_and_blank_are_empty_error ... ok
test dsn::tests::other_scheme_is_scheme_error ... ok
test dsn::tests::debug_is_identical_for_different_passwords ... ok
test dsn::tests::as_str_returns_the_input_verbatim ... ok
test result: ok. 7 passed; 0 failed
```

`crates/wyd-db/src/dsn.rs` já implementava parse, `DsnError::{Empty, Scheme}`, `Debug`/`Display` redigidos e `as_str` desde o primeiro slice; a página `src/dsn.md` está aceita e documenta a garantia. Nenhum arquivo de produção foi alterado nesta issue; a execução fechou o DoD e atualizou o estado.

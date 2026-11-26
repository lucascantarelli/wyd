# WYDDB-005 — implementar Dsn com Debug redigido

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 2 — contratos e erros tipados


## Contexto

A DSN carrega a senha e seria impressa por `Debug` herdado. O crate precisa de um tipo único que parseie, valide e redija antes de qualquer conexão.

## Entregável

Implementar `src/dsn.rs` com parse, erro tipado e `Debug` que substitui a senha; nenhum `DATABASE_URL` e nenhuma leitura de ambiente na biblioteca.

## DoD

- [ ] Teste de parse aceita DSN válida e recusa forma inválida com erro específico.
- [ ] `format!("{:?}", dsn)` não contém a senha nem a URL completa.
- [ ] `rg -n "DATABASE_URL" crates/wyd-db/src` não encontra leitura.
- [ ] A página `src/dsn.md` documenta a garantia e o erro.

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

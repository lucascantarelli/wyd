# WYDDB-034 — fechar gate anti-skip e matriz CAP

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 6 — testes de lacuna e harness


## Contexto

Um `SKIP` silencioso faz a suíte parecer verde; a matriz de capacidade é o que impede cobertura por intuição.

## Entregável

Implementar o gate anti-skip descrito em `gates-e-anti-skip.md`: varrer `#[ignore]`/`SKIP`, exigir DSN nomeada e comparar alvos do `Cargo.toml` com a matriz `CAP-nn`.

## DoD

- [ ] O gate reprova `#[ignore]` novo no caminho de aceite.
- [ ] Ausência de `WYD_DB_TEST_URL` falha nomeando a variável; não pula.
- [ ] Um alvo sem capacidade e uma capacidade sem alvo reprovam.
- [ ] O gate roda localmente sem CI real.

## Fora de escopo

Não criar workflow de CI, não definir cobertura percentual e não impor threshold de tempo.

## Risco

O gate pode virar grep frágil; o teste do próprio gate precisa de fixtures de violação.

## Depende de

WYDDB-032, WYDDB-033.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do gate e fixtures de violação.

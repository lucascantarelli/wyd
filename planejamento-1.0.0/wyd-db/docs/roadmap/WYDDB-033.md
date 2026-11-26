# WYDDB-033 — criar fixtures e factories isoladas

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 6 — testes de lacuna e harness


## Contexto

Sem factory, cada teste inventa id e dado; sem limpeza por chave, o paralelismo contamina o resultado.

## Entregável

Criar `tests/common/` com factory por agregado, fixture de conteúdo, limpeza por chave e proibição de estado global; documentar o inventário.

## DoD

- [ ] Nenhum teste usa id mágico; a factory devolve o identificador.
- [ ] Cada teste limpa o que criou e pode rodar em paralelo.
- [ ] Fixture de conteúdo tem versão própria e dado fictício.
- [ ] `tests/fixtures-e-factories.md` lista o inventário real.

## Fora de escopo

Não criar conta de produção, não usar PII real e não definir catálogo de contas (depende de `M19`).

## Risco

Uma factory que compartilha estado global economiza linha e cria flaky; o teste de paralelismo deve pegar.

## Depende de

WYDDB-030.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Testes paralelos verdes e página de fixtures.

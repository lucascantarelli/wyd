# WYDDB-034 — fechar gate anti-skip e matriz CAP

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

Um `SKIP` silencioso faz a suíte parecer verde; a matriz de capacidade é o que impede cobertura por intuição.

## Entregável

Implementar o gate anti-skip descrito em `gates-e-anti-skip.md`: varrer `#[ignore]`/`SKIP`, exigir DSN nomeada e comparar alvos do `Cargo.toml` com a matriz `CAP-nn`.

## DoD

- [ ] O gate reprova `#[ignore]` novo no caminho de aceite.
- [ ] Ausência de `WYD_DB_URL_TEST` falha nomeando a variável; não pula.
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

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do gate e fixtures de violação.

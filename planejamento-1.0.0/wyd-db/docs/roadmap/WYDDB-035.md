# WYDDB-035 — escrever 0001-bootstrap.sql

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 7 — migrations e evolução de schema

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

`0001` cria os quatro schemas, a função de `updated_at` e `content_version`; sem ele, nenhum outro arquivo da série aplica.

## Entregável

Escrever `0001-bootstrap.sql` com `up`/`down`, schemas `content`, `game`, `portal`, `ops`, função e `content_version`, conforme `ddl-confirmado.md` §3.

## DoD

- [ ] `up` cria os quatro schemas, a função e a versão; `down` remove tudo que criou.
- [ ] Nenhum `ALTER`, seed de conteúdo ou FK para arquivo futuro.
- [ ] O contrato estrutural passa e o ciclo `up → down → up` é idempotente.
- [ ] A página `migrations/0001-bootstrap.md` existe e cita o arquivo.

## Fora de escopo

Não criar dados de conteúdo, não trocar `SCHEMA_VERSION` e não aplicar em produção.

## Risco

O `down` que remove schema pode apagar objeto de outro arquivo se a ordem estiver errada; o teste de ciclo é obrigatório.

## Depende de

WYDDB-014, WYDDB-028.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Arquivo versionado, teste de contrato e ciclo local.

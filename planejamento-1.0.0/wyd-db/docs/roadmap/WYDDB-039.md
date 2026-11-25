# WYDDB-039 — escrever 0013 a 0029 de conteúdo e views

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

Conteúdo, regras, mapas, combate, quests e as duas views fecham o catálogo e a leitura cruzada; os seeds bloqueados não podem ser tratados como dados reais.

## Entregável

Escrever `0013`–`0029` com DDL e os seeds aplicáveis, mantendo `0016`–`0018`, `0020`, `0023` e `0027` bloqueados até a conferência da fonte.

## DoD

- [ ] Os arquivos de DDL aplicam na ordem; as views de `0029` são criadas.
- [ ] Os seis seeds bloqueados permanecem sem dado inventado e a página declara a lacuna.
- [ ] Contagens de conteúdo só entram com fonte e data.
- [ ] O contrato passa sem FK para o futuro.

## Fora de escopo

Não extrair dado do cliente 7662, não medir contagem de seed e não liberar seed bloqueado.

## Risco

A pressão para “completar” os seeds pode inventar catálogo; a issue deve falhar explicitamente enquanto o dado não existir.

## Depende de

WYDDB-038, dados de origem dos seeds.

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

Arquivos DDL, views e páginas; seeds bloqueados listados.

# WYDBOT-F3 — Suíte e anti-skip

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-bots`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (todos os `RF-BOT`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — Suíte e anti-skip
**Fase:** 3


## Contexto

A fatia F3 do `wyd-bots` cobre suíte, anti-skip, relatório de carga e existe para
cumprir todos os `RF-BOT` sem antecipar a fatia seguinte.

## Entregável

suíte, anti-skip, relatório de carga

## DoD

- [ ] alvos de `plano-de-testes.md` §2 sem `SKIP`.
- [ ] cobertura ≥ 85% onde aplicável.

## Fora de escopo

migrations

## Depende de

F0–F2; CI real

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F3 concluída em `docs/roadmap/roadmap.md`.

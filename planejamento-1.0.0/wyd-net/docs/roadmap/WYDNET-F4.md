# WYDNET-F4 — Suíte, fixtures e anti-skip

> Pergunta: "o que a fatia F4 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-net`
> Base: `docs/roadmap/roadmap.md` fatia F4; `docs/specs/requisitos.md` (todos os `RF-NET`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F4 — Suíte, fixtures e anti-skip
**Fase:** 4


## Contexto

A fatia F4 do `wyd-net` cobre unitário, contrato, sistema, fixtures centralizadas, anti-skip e existe para
cumprir todos os `RF-NET` sem antecipar a fatia seguinte.

## Entregável

unitário, contrato, sistema, fixtures centralizadas, anti-skip

## DoD

- [ ] alvos de `plano-de-testes.md` §2 passam sem `SKIP`.
- [ ] cobertura ≥ 85%.

## Fora de escopo

migrations

## Depende de

F0–F3; CI real

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F4 concluída em `docs/roadmap/roadmap.md`.

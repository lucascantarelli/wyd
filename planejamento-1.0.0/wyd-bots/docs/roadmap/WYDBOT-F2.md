# WYDBOT-F2 — Convivência e medição

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-bots`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-BOT-005`, `RF-BOT-006`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Convivência e medição
**Fase:** 2


## Contexto

A fatia F2 do `wyd-bots` cobre bot de convivência, medição de `LBOT-03` e existe para
cumprir `RF-BOT-005`, `RF-BOT-006` sem antecipar a fatia seguinte.

## Entregável

bot de convivência, medição de `LBOT-03`

## DoD

- [ ] convivência passa sem crash.
- [ ] `LBOT-03` fecha com número real.

## Fora de escopo

anti-cheat

## Depende de

F1

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F2 concluída em `docs/roadmap/roadmap.md`.

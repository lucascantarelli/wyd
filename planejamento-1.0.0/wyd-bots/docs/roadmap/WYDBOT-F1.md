# WYDBOT-F1 — Bots de QA e carga

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-bots`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-BOT-003`, `RF-BOT-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F1 — Bots de QA e carga
**Fase:** 2


## Contexto

A fatia F1 do `wyd-bots` cobre bots de QA (cenários), bots de carga (N conexões) e existe para
cumprir `RF-BOT-003`, `RF-BOT-004` sem antecipar a fatia seguinte.

## Entregável

bots de QA (cenários), bots de carga (N conexões)

## DoD

- [ ] cenário de QA passa.
- [ ] carga mede `p95` e taxa de erro.
- [ ] número real registrado.

## Fora de escopo

convivência

## Depende de

F0; `wyd-server` rodando

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F1 concluída em `docs/roadmap/roadmap.md`.

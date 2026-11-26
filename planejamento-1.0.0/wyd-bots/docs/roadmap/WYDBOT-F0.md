# WYDBOT-F0 — Ambiente e baseline

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-bots`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-BOT-001`, `RF-BOT-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Ambiente e baseline
**Fase:** 1


## Contexto

A fatia F0 do `wyd-bots` cobre compose efêmero, seed, baseline de bot e existe para
cumprir `RF-BOT-001`, `RF-BOT-002` sem antecipar a fatia seguinte.

## Entregável

Compose efêmero, seed, baseline de bot

## DoD

- [ ] ambiente sobe/desce.
- [ ] seed aplicado.
- [ ] bot conecta e desconecta limpo.

## Fora de escopo

carga

## Depende de

`LBOT-01`/`LBOT-02` (ambiente efêmero)

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

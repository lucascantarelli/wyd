# WYDOPS-F2 — Ambiente

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-OPS-003`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Ambiente
**Fase:** 2


## Contexto

A fatia F2 do `wyd-ops` cobre compose efêmero, seed, up/down e existe para
cumprir `RF-OPS-003` sem antecipar a fatia seguinte.

## Entregável

Compose efêmero, seed, up/down

## DoD

- [ ] ambiente sobe/desce.
- [ ] seed aplicado.
- [ ] sem estado global.

## Fora de escopo

monitoramento

## Depende de

F0; `LBOT-01`/`LBOT-02`

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F2 concluída em `docs/roadmap/roadmap.md`.

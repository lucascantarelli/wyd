# WYDCORE-F2 — Combate

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-core`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-CORE-005`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Combate
**Fase:** 2


## Contexto

A fatia F2 do `wyd-core` cobre cálculo de dano, alcance, durabilidade (modelo) e existe para
cumprir `RF-CORE-005` sem antecipar a fatia seguinte.

## Entregável

cálculo de dano, alcance, durabilidade (modelo)

## DoD

- [ ] fórmulas reproduzem fixture capturada.
- [ ] nenhum número inventado (`stat_source` declarado).

## Fora de escopo

feitiços, pets

## Depende de

F1; offsets de protocolo

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

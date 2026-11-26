# WYDSIM-F1 — Fórmulas e curvas

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-sim`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-SIM-003`, `RF-SIM-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F1 — Fórmulas e curvas
**Fase:** 2


## Contexto

A fatia F1 do `wyd-sim` cobre fórmulas de dano, curva de XP, regeneração e existe para
cumprir `RF-SIM-003`, `RF-SIM-004` sem antecipar a fatia seguinte.

## Entregável

fórmulas de dano, curva de XP, regeneração

## DoD

- [ ] fórmula reproduz fixture capturada.
- [ ] `stat_source` declarado.
- [ ] teste de regressão verde.

## Fora de escopo

simulação completa

## Depende de

F0; captura do cliente 7662

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

# WYDLCH-F0 — Fundação Tauri

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-launcher`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-LCH-001`, `RF-LCH-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Fundação Tauri
**Fase:** 1


## Contexto

A fatia F0 do `wyd-launcher` cobre app Tauri, layout, config, baseline fmt/clippy e existe para
cumprir `RF-LCH-001`, `RF-LCH-002` sem antecipar a fatia seguinte.

## Entregável

app Tauri, layout, config, baseline fmt/clippy

## DoD

- [ ] `cargo tauri build` gera binário.
- [ ] fmt/clippy verdes.

## Fora de escopo

patch, serverlist

## Depende de

projeto Tauri existente

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

# WYDLCH-F1 — Manifesto e patch

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-launcher`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-LCH-003`, `RF-LCH-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F1 — Manifesto e patch
**Fase:** 2


## Contexto

A fatia F1 do `wyd-launcher` cobre leitura de manifesto, download e aplicação de patch e existe para
cumprir `RF-LCH-003`, `RF-LCH-004` sem antecipar a fatia seguinte.

## Entregável

leitura de manifesto, download e aplicação de patch

## DoD

- [ ] patch corrompido é recusado.
- [ ] patch válido aplica.
- [ ] rollback em falha.

## Fora de escopo

serverlist

## Depende de

F0; manifesto íntegro

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

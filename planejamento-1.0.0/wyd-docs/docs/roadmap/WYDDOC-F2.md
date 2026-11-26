# WYDDOC-F2 — Contagens e censos

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-docs`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-DOC-003`, `RF-DOC-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Contagens e censos
**Fase:** 2


## Contexto

A fatia F2 do `wyd-docs` cobre `gate/checks/contagens.py`, `gate/checks/censos.py` e existe para
cumprir `RF-DOC-003`, `RF-DOC-004` sem antecipar a fatia seguinte.

## Entregável

`gate/checks/contagens.py`, `gate/checks/censos.py`

## DoD

- [ ] soma/citação divergente reprova.
- [ ] gate verde.

## Fora de escopo

publicação

## Depende de

F0

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

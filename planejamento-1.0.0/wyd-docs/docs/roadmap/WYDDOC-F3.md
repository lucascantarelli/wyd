# WYDDOC-F3 — Publicação e redirect

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-docs`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (`RF-DOC-005`, `RF-DOC-006`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — Publicação e redirect
**Fase:** 3


## Contexto

A fatia F3 do `wyd-docs` cobre workflow GitHub Pages, `redirects.tsv` e existe para
cumprir `RF-DOC-005`, `RF-DOC-006` sem antecipar a fatia seguinte.

## Entregável

workflow GitHub Pages, `redirects.tsv`

## DoD

- [ ] publicação automática em `develop`.
- [ ] rollback validado.
- [ ] redirect válido/inválido reprova duplicata.

## Fora de escopo

geração de patch

## Depende de

F0–F2; `LPDOC-01`

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F3 concluída em `docs/roadmap/roadmap.md`.

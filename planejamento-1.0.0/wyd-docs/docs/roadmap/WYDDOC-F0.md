# WYDDOC-F0 — Montagem e tema

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-docs`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-DOC-001`, `RF-DOC-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Montagem e tema
**Fase:** 1


## Contexto

A fatia F0 do `wyd-docs` cobre `preparar.py`, tema Jinja, navegação e existe para
cumprir `RF-DOC-001`, `RF-DOC-007` sem antecipar a fatia seguinte.

## Entregável

`preparar.py`, tema Jinja, navegação

## DoD

- [ ] build gera `.build/docs/` com caminho relativo preservado.
- [ ] sem conteúdo interno.

## Fora de escopo

gate

## Depende de

`planejamento-1.0.0/<sistema>/docs/**/*.md`

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `frontend-design` |
| `skills_complementares` | `documentation-writer` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

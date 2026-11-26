# WYDOPS-F1 — Imagem

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-OPS-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F1 — Imagem
**Fase:** 1


## Contexto

A fatia F1 do `wyd-ops` cobre dockerfile, build reproduzível, publicação e existe para
cumprir `RF-OPS-002` sem antecipar a fatia seguinte.

## Entregável

Dockerfile, build reproduzível, publicação

## DoD

- [ ] build ok.
- [ ] sem segredo na imagem.
- [ ] `pull` funciona.

## Fora de escopo

ambiente

## Depende de

F0

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F1 concluída em `docs/roadmap/roadmap.md`.

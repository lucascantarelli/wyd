# WYDOPS-F0 — CI

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-OPS-001`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — CI
**Fase:** 1


## Contexto

A fatia F0 do `wyd-ops` cobre workflow reutilizável, permissões mínimas, pin por SHA e existe para
cumprir `RF-OPS-001` sem antecipar a fatia seguinte.

## Entregável

workflow reutilizável, permissões mínimas, pin por SHA

## DoD

- [ ] workflow verde em PR.
- [ ] varredura de segredo limpa.

## Fora de escopo

deploy

## Depende de

repositório existente

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.

# WYDOPS-F4 — Segredos e deploy

> Pergunta: "o que a fatia F4 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F4; `docs/specs/requisitos.md` (`RF-OPS-005`, `RF-OPS-006`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F4 — Segredos e deploy
**Fase:** 3


## Contexto

A fatia F4 do `wyd-ops` cobre secret store, deploy contínuo, rollback e existe para
cumprir `RF-OPS-005`, `RF-OPS-006` sem antecipar a fatia seguinte.

## Entregável

secret store, deploy contínuo, rollback

## DoD

- [ ] segredo nunca em código.
- [ ] deploy em staging.
- [ ] rollback validado.

## Fora de escopo

gate

## Depende de

F0–F3

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F4 concluída em `docs/roadmap/roadmap.md`.

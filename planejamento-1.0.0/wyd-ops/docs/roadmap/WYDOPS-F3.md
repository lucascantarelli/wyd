# WYDOPS-F3 — Monitoramento

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (`RF-OPS-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — Monitoramento
**Fase:** 2


## Contexto

A fatia F3 do `wyd-ops` cobre prometheus, Loki, métricas, logs de metadados e existe para
cumprir `RF-OPS-004` sem antecipar a fatia seguinte.

## Entregável

Prometheus, Loki, métricas, logs de metadados

## DoD

- [ ] métricas presentes.
- [ ] log sem payload/PII.

## Fora de escopo

segredos

## Depende de

F2

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

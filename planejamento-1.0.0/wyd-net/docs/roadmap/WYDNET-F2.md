# WYDNET-F2 — Rate-limit e hardening

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-net`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-NET-005`, `RF-NET-006`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Rate-limit e hardening
**Fase:** 3


## Contexto

A fatia F2 do `wyd-net` cobre limites por IP/sessão, backpressure, rejeição de pacote malformado e existe para
cumprir `RF-NET-005`, `RF-NET-006` sem antecipar a fatia seguinte.

## Entregável

limites por IP/sessão, backpressure, rejeição de pacote malformado

## DoD

- [ ] limite excedido retorna erro tipado.
- [ ] pacote malformado é descartado e logado sem payload.
- [ ] teste de carga mede `p95`.

## Fora de escopo

observabilidade completa

## Depende de

F1; medição `LNET-01`

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

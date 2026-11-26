# WYDSRV-F2 — Controle e integração

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-server`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-SRV-005`, `RF-SRV-006`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Controle e integração
**Fase:** 3


## Contexto

A fatia F2 do `wyd-server` cobre endpoint HTTP síncrono com ack, integração com net/core/db e existe para
cumprir `RF-SRV-005`, `RF-SRV-006` sem antecipar a fatia seguinte.

## Entregável

endpoint HTTP síncrono com ack, integração com net/core/db

## DoD

- [ ] ack retorna antes do efeito.
- [ ] ação sem permissão retorna 403.
- [ ] auditoria gravada.

## Fora de escopo

observabilidade completa

## Depende de

F1; contrato admin (`LAC-ADM-03`)

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

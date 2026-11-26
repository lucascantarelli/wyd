# WYDNET-F1 — Sessão e transporte async

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-net`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-NET-003`, `RF-NET-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F1 — Sessão e transporte async
**Fase:** 2


## Contexto

A fatia F1 do `wyd-net` cobre sessão TCP/async, handshake, keep-alive, desconexão limpa e existe para
cumprir `RF-NET-003`, `RF-NET-004` sem antecipar a fatia seguinte.

## Entregável

sessão TCP/async, handshake, keep-alive, desconexão limpa

## DoD

- [ ] teste de sessão revogada não acessa canal.
- [ ] desconexão em voo não perde pacote pendente.
- [ ] `cargo test` verde.

## Fora de escopo

rate-limit, hardening

## Depende de

F0; captura `LPRT-03`/`LPRT-06`

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

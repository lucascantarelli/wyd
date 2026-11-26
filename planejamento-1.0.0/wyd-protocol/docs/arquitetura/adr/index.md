# ADRs — `wyd-protocol`

> Pergunta: "quais decisões estruturais do `wyd-protocol` estão registradas como ADR e em que estado?"
> Público: Arquitetura, desenvolvimento e QA de fio
> Dono: equipe do `wyd-protocol`
> Base: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`; `docs/proposta-decisoes-protocol.md` §4; `wyd-protocol-1.0.0.md` §8
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

Uma ADR por decisão estrutural que mude o alvo do fio, a fronteira do codec ou a forma de provar o
layout. O catálogo nasceu vazio (`wyd-protocol-1.0.0.md` §8: "o `index.md` nasce sem linha") e registra
agora a primeira decisão aprovada, com a data do "sim".

| ID | Título | Status |
|---|---|---|
| [`ADR-PRT-0001`](adr-prt-0001-build-alvo-7662.md) | Build-alvo é o cliente 7662 identificado por versão e SHA-256 (`Q-PRT-1` = A) | aceito (2026-11-23) |

`LPRT-03` e `LPRT-06` continuam bloqueadas como prova — a decisão de alvo não captura byte nenhum.
Nenhuma outra pergunta do sistema está aberta.

Índice global: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`.

# ADRs — `wyd-bots`

> Pergunta: "quais decisões estruturais do `wyd-bots` estão registradas como ADR e em que estado?"
> Público: Arquitetura, QA e operação
> Dono: equipe do `wyd-bots`
> Base: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`; `docs/proposta-decisoes-bots.md` §4
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

Uma ADR por decisão estrutural que mude onde o bot roda, como o dado sintético nasce ou como a carga
reprova. `aceito` só para decisão aprovada pelo dono; o resto continua em
`docs/proposta-decisoes-bots.md` (`AGENTS.md` §5, item 3).

| ID | Título | Status |
|---|---|---|
| [`ADR-BOT-0001`](adr-bot-0001-ambiente-ci-efemero.md) | Ambiente de bot é CI efêmero, com servidor e seed descartáveis (`Q-BOT-1` + `Q-BOT-2` = A) | aceito (2026-11-23) |

`LBOT-03` segue bloqueada como medição: a política do limiar (`Q-BOT-3` = A) está decidida, mas não há
número, conexão nem latência medidos. Este índice não fecha a lacuna.

Índice global: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`.

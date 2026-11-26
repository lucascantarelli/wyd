# ADRs — `wyd-sim`

> Pergunta: "quais decisões estruturais do `wyd-sim` estão registradas como ADR e em que estado?"
> Público: Arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-sim`
> Base: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`; `docs/proposta-decisoes-sim.md` §4
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

Uma ADR por decisão estrutural que mude a aritmética do cálculo, o formato dos vetores ou o contrato
`sim` → `core`. Decisão aprovada vira ADR `aceito`; decisão sem "sim" do dono continua em
`docs/proposta-decisoes-sim.md` (`AGENTS.md` §5, item 3).

| ID | Título | Status |
|---|---|---|
| [`ADR-SIM-0001`](adr-sim-0001-aritmetica-inteira-em-bps.md) | Aritmética inteira em bps, arredondamento único, `f64` só em exibição (`Q-SIM-4` = A) | aceito (2026-11-23) |

As quatro lacunas (`LSIM-01`–`LSIM-04`) fecharam por decisão em 2026-11-23 e estão em `lacunas.md`;
escrever fórmulas, vetores e fixtures continua pendente de implementação. Nenhuma pergunta do sistema
está aberta.

Índice global: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`.

# ADR-POR-0001 — Leilão: BFF fecha, wyd-ops agenda/alerta

> Pergunta: "quem fecha o leilão e quem agenda/alerta?"
> Público: Arquitetura e desenvolvimento
> Dono: `wyd-portal`
> Base: `planejamento-1.0.0/wyd-portal/docs/...`; `LAC-POR-09` (fechada 2026-11-23)
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25) — aprovado pelo dono.

## Contexto

`LAC-POR-09` fechou em 2026-11-23: o BFF do portal fecha o leilão; o `wyd-ops` agenda e alerta.

## Decisão

O fechamento do leilão é responsabilidade do BFF do portal; o agendamento e o alerta são
responsabilidade do `wyd-ops`. Job idempotente com `SKIP LOCKED`.

## Consequências

- Positivas: separação clara de responsabilidades.
- Negativas: dependência de job do `wyd-ops` para fechamento no prazo.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Portal agenda | mistura responsabilidades |

## Como verificar

`F9` do roadmap do portal (job roda duas vezes sem cobrar duas vezes).

## Referências

- Lacuna: `LAC-POR-09` (fechada 2026-11-23).

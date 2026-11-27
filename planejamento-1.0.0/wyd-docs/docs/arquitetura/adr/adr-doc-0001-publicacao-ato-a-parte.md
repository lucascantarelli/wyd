# ADR-DOC-0001 — Publicação é ato à parte

> Pergunta: "quem aprova e quem publica o site?"
> Público: Arquitetura e desenvolvimento
> Dono: `wyd-docs`
> Base: `planejamento-1.0.0/wyd-docs/lacunas.md`; `LPDOC-08` (fechada 2026-11-19)
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25) — aprovado pelo dono.

## Contexto

`LPDOC-08` fechou em 2026-11-19: arquitetura aprova o sistema novo (nome, fronteira e entrada em
`SISTEMAS` de `wyd-docs/preparar.py`); documentação publica.

## Decisão

Aprovação de sistema novo é da arquitetura; publicação no site é da documentação. Os dois atos são
separados.

## Consequências

- Positivas: gate de qualidade antes de publicar.
- Negativas: mais um passo no fluxo.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Publicar sem aprovação | risco de conteúdo não revisado |

## Como verificar

`LPDOC-08` fechada + fluxo documentado em `wyd-docs-1.0.0.md`.

## Referências

- Lacuna: `LPDOC-08` (fechada 2026-11-19).

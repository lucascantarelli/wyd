# ADR-NET-0001 — Política de limites (Q-NET-1 = A)

> Pergunta: "qual política de limites o `wyd-net` adota e como verificá-la?"
> Público: Arquitetura e desenvolvimento
> Dono: `wyd-net`
> Base: `planejamento-1.0.0/wyd-net/docs/proposta-decisoes-net.md`; `wyd-net-1.0.0.md`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25) — aprovado pelo dono.

## Contexto

A decisão `Q-NET-1 = A` foi tomada em 2026-11-23 (política de limites), mas o número concreto
(`LNET-01`) segue aberto como medição. Esta ADR registra a política; o valor numérico é medição.

## Decisão

Adotar a política `Q-NET-1 = A` para limites por IP/sessão e backpressure, com medição
`LNET-01` definindo o valor final antes de fixar em código.

## Consequências

- Positivas: fronteira clara para rate-limit e hardening (F2).
- Negativas: número final depende de medição real; não pode ser inventado.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Fixar número sem medição | Viola evidência (`V`/`H`/`P`) |

## Como verificar

Medição `LNET-01` concluída e registrada em `lacunas.md` + `docs/proposta-decisoes-net.md`.

## Referências

- Lacuna: `LNET-01` (aberta, medição).
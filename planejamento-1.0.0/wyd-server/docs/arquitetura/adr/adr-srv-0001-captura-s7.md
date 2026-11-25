# ADR-SRV-0001 — Captura S7 (peer de A1)

> Pergunta: "qual o estado da captura S7 do `wyd-server` e como verificá-la?"
> Público: Arquitetura e desenvolvimento
> Dono: `wyd-server`
> Base: `planejamento-1.0.0/wyd-server/wyd-server-1.0.0.md`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25) — aprovado pelo dono.

## Contexto

`LSRV-05` está em estado C com condição reprovada (peer de A1, implementação pendente). A captura
S7 (A1 autorizada 2026-11-23) ainda não foi executada.

## Decisão

Manter `LSRV-05` como peer de A1: a implementação do canal de controle HTTP síncrono com ack
(`LAC-ADM-03`) depende da captura S7 concluída.

## Consequências

- Positivas: evita implementação sem evidência.
- Negativas: bloqueia F2 (controle e integração) até a captura.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Implementar sem captura | Viola "planejar antes de codar" |

## Como verificar

Captura S7 datada + `LSRV-05` fechada em `lacunas.md`.

## Referências

- Lacuna: `LSRV-05` (C, condição reprovada).
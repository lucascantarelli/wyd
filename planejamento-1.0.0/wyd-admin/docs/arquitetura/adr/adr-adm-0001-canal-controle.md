# ADR-ADM-0001 — Canal de controle HTTP síncrono com ack

> Pergunta: "qual o contrato do canal de controle do `wyd-admin` e como verificá-lo?"
> Público: Arquitetura e desenvolvimento
> Dono: `wyd-admin`
> Base: `planejamento-1.0.0/wyd-admin/docs/arquitetura/contrato-c4.md`; `LAC-ADM-03`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25) — aprovado pelo dono.

## Contexto

`LAC-ADM-03` decidiu (2026-11-22) o canal de controle do `wyd-server`: HTTP síncrono com ack.
Esta ADR formaliza o registro.

## Decisão

Toda ação do `wyd-admin` sobre o processo do `wyd-server` ocorre via HTTP síncrono com ack, com
auditoria gravada e permissão negada retornando 403.

## Consequências

- Positivas: contrato claro (F2 do wyd-server).
- Negativas: latência de ack; dependência de disponibilidade do servidor.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Mensageria assíncrona | perde garantia de ack síncrono |

## Como verificar

Teste de ack + 403 + auditoria (`RF-SRV-005`).

## Referências

- Lacuna: `LAC-ADM-03` (decidida 2026-11-22).

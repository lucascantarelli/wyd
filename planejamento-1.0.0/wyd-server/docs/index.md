# wyd-server — visão geral

> Pergunta: "o que o `wyd-server` é, o que ele já deve ao `wyd-admin` e por onde começo a ler?"
> Público: Desenvolvedor novo, arquitetura e operação
> Dono: equipe do wyd-server
> Base: `wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §3; `LAC-POR-06`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-server` compõe o processo do servidor e roda **um processo por canal**: abre o bind do CPSock,
monta o `wyd-net` sobre o `wyd-core`, publica saúde e prontidão e recebe comando administrativo. É
o dono da topologia e do ciclo de vida do processo; a regra de jogo é do `wyd-core`.

## O que ele não faz

| Fora | Dono |
|---|---|
| byte, opcode, encoding | `wyd-protocol` |
| socket, timeout, rate-limit | `wyd-net` |
| tick, combate, cena | `wyd-core` / `wyd-sim` |
| SQL e migrations | `wyd-db` |
| auditoria e escrita administrativa | `wyd-admin` |

## A obrigação que ele já tem

O canal de controle foi **decidido em 2026-11-22** (`LAC-ADM-03`, opção A): o `wyd-admin` chama
`POST /internal/v1/control` no processo do canal, autentica com segredo próprio e só grava
`released_at`/`audit_log` **depois** do ack `200` com `{delivered_to}`. Processo fora do ar é `503`
sem escrita. O endpoint nasce no plano deste sistema — ele ainda não existe.

## O que falta

Quatro perguntas abertas — composição do processo, número de canais, significado de `ready` e
encerramento gracioso — estão em `lacunas.md` (planejamento).

## Limite desta documentação

- Não afirma quantos canais nem qual porta: nada foi decidido nem medido.
- Não afirma capacidade, latência ou número de jogadores por canal.

# wyd-server — visão geral

> Pergunta: "o que o `wyd-server` é, o que ele já deve ao `wyd-admin` e por onde começo a ler?"
> Público: Desenvolvedor novo, arquitetura e operação
> Dono: equipe do wyd-server
> Base: `wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §3; `LAC-POR-06`; `docs/specs/endpoint-de-captura.md`
> Revisado em: 2026-11-23

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

## Como isso é provado primeiro

A primeira prova deste sistema não é uma feature: é a fatia mínima **login → entrada no mundo → um passo**,
com as mensagens, os goldens e os critérios de ponta a ponta registrados em
[`planos/esqueleto-de-ponta-a-ponta.md`](planos/esqueleto-de-ponta-a-ponta.md) (proposta, 2026-11-23). Ela
depende do handshake provado por captura; a topologia mínima já está decidida (`LSRV-01`, `LSRV-02`),
mas enquanto `LPRT-03` não fechar, nenhum byte desta fatia é afirmável. A captura local é o `ECAP`, não
o endpoint de produção (`EPROD`): a distinção está em
[`specs/endpoint-de-captura.md`](specs/endpoint-de-captura.md).

## O que falta

As cinco lacunas fecharam em 2026-11-23. `Q-SRV-5` foi respondida na **opção C, com quarentena**, mas a
medição do peer **reprovou a condição**: o binário legado sobe em loopback, porém não existe em clone
limpo, depende de `MIGRATIONS_DIR` manual e compila gate de versão 12000 contra o cliente 7640/7662.
**A1 passa a depender da opção A**, com o peer do próprio `wyd-server`. O `ECAP` está especificado e sem
código; falta implementar o processo, o sinal de prontidão, o shutdown gracioso, o `ECAP` e a CLI, e só
então executar a captura A1.

## Limite desta documentação

- Não afirma porta, capacidade, latência ou jogadores por canal; o número de canais está decidido (`LSRV-02`).

## Decisões

As cinco lacunas fecharam em 2026-11-23: `LSRV-01` e `LSRV-02` (composição e canais), `LSRV-03`
(`ready` com socket + banco + tick), `LSRV-04` (shutdown como execução; `WYDDB-021` dona da sequência) e
`LSRV-05` (peer de A1: C com quarentena, condição reprovada em 2026-11-23; A1 depende de A). O registro está em
[Decisões do `wyd-server`](proposta-decisoes-server.md); o estado de cada lacuna continua em `lacunas.md`.

# wyd-core — visão geral

> Pergunta: "o que o `wyd-core` é e por onde começo a ler?"
> Público: Desenvolvedor novo, arquitetura e QA
> Dono: equipe do wyd-core
> Base: `wyd-protocol-1.0.0.md` §1.4; `LPRT-18`; `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-core` é a simulação do mundo em memória: tick, entidades, cena, combate e IA. Recebe a
mensagem já tipada do `wyd-net`, aplica a regra do jogo, chama o cálculo do `wyd-sim` e pede ao
`wyd-db` o que precisa ser lido ou gravado. Ele é o dono do **comportamento** — classes, skills,
combate, guerra, guilda e estados — e não do cálculo nem do fio.

## O que ele não faz

| Fora | Dono |
|---|---|
| byte, opcode, encoding | `wyd-protocol` |
| socket, timeout, ordem | `wyd-net` |
| fórmula, curva, chance, drop | `wyd-sim` |
| SQL, DDL e transação | `wyd-db` |
| processo, canal e topologia | `wyd-server` |

## Contratos que ele já herda

| Contrato | Estado | Onde foi decidido |
|---|---|---|
| recebe payload tipado e validado, nunca offset | decidido | `LPRT-18` |
| recebe em ordem por conexão | decidido | `LPRT-12` |
| comando de processo vem do `wyd-server`, com ack do admin | decidido no elo | `LAC-ADM-03` |

## O que falta

Quatro perguntas abertas — escopo sem catálogo, estado em memória × durável, fronteira com o
`wyd-sim` e forma de persistência — estão em `lacunas.md` (planejamento). O catálogo funcional, que
mede a `1.0.0`, ainda não existe (`wyd-1.0.0.md` §1.5).

## Limite desta documentação

- Não afirma regra de jogo: esta abertura não decidiu classe, dano, drop nem progressão.
- Não afirma capacidade, latência ou quantas entidades cabem em memória: nada foi medido.

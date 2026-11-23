# wyd-ops — visão geral

> Pergunta: "o que o `wyd-ops` entrega, o que ele já decidiu e por onde começo a ler?"
> Público: Desenvolvedor novo, arquitetura e quem opera
> Dono: frente `wyd-ops`
> Base: `wyd-ops/plano-ci-real.md`; `wyd-ops/proposta-versoes-e-cobertura-funcional.md`; `AGENTS.md` §4
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-ops` é a frente de **processo e infraestrutura**: CI, imagens, deploy, observabilidade,
topologia de ambiente e provisionamento. Ele não implementa regra de jogo nem fala o protocolo;
ele garante que o que existe seja construível, publicável, observável e reproduzível.

## O que ele já decidiu

| Decisão | Data | Onde |
|---|---|---|
| CI usa apenas `run:` e action própria; nenhuma action de terceiro sem SHA | 2026-11-19 | `plano-ci-real.md` |
| deploy é `docker compose` em host com systemd; runtime é Compose, não Kubernetes | 2026-11-19 | `plano-ci-real.md` §3 |
| versões por linha de release e a cobertura funcional por versão | 2026-11-20 | `proposta-versoes-e-cobertura-funcional.md` §7 |
| elo de controle admin → servidor com segredo próprio e ack antes de escrita | 2026-11-22 | `wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §3 |
| imagem do site é construída pelo CI e consumida pelo Compose | 2026-11-22 | `plano-imagem-e-compose-do-site.md` |
| layout do repositório: artefato executável na pasta real do projeto; Compose, `.dockerignore` e workflows na raiz | 2026-11-22 | `planejamento-1.0.0/wyd-1.0.0.md` §8 |

## O que ele não faz

| Fora | Dono |
|---|---|
| regra de jogo | `wyd-core` / `wyd-sim` |
| layout do fio | `wyd-protocol` |
| escrita administrativa e auditoria | `wyd-admin` |
| conteúdo de notícia e conta de jogador | `wyd-portal` |
| patch do cliente | `wyd-client` / `wyd-launcher` |

## Onde os artefatos dele moram

O layout do repositório — qual pasta, qual sistema é dono e por que Compose, `.dockerignore` e
`.github/workflows/` ficam na raiz — tem um dono único: `planejamento-1.0.0/wyd-1.0.0.md` §8 (no
repositório; o panorama não é publicado no site). O Dockerfile da imagem do site está em
`wyd-ops/wyd-docs.Dockerfile`; a decisão e o DoD, em `plano-imagem-e-compose-do-site.md`.

## O que falta

O `wyd-ops` tem propostas, não plano diretor: ainda não há `RF-OPS`, roadmap com DoD por issue nem
plano de teste de infraestrutura. Esta página abre a porta; não fecha a lacuna.

## Limite desta documentação

- Não afirma ambiente publicado: nenhum deploy existe.
- Não afirma imagem publicada: o workflow está no repositório; a primeira execução dirá se empurra.

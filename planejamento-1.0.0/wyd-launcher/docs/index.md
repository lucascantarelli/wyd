# wyd-launcher — visão geral

> Pergunta: "o que o launcher faz antes do jogo abrir e de quem ele depende?"
> Público: Desenvolvedor novo, produto e operação
> Dono: equipe do wyd-launcher
> Base: `wyd-portal/docs/arquitetura/fronteira.md` §2; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-launcher` é a aplicação desktop (Tauri) que coloca o cliente no estado certo antes de abrir:
confere integridade dos arquivos, aplica patch publicado, mostra notícias e a lista de canais, e
inicia o jogo. É o dono do manifesto, do patch e da integridade do lado do jogador.

## O que ele não faz

| Fora | Dono |
|---|---|
| conteúdo do patch e versão do cliente | `wyd-client` |
| hospedagem do artefato | `wyd-ops` |
| topologia de canais | `wyd-server` |
| login de pessoa e notícias | `wyd-portal` |
| handshake e login do jogo | `wyd-net` / `wyd-protocol` |

## O que falta

**Nenhuma pergunta aberta.** As quatro lacunas fecharam em 2026-11-23: o formato do manifesto
(`LLCH-01`, `Q-LCH-1` = A: JSON versionado e canônico com `sha256` por arquivo), a lista de canais
(`LLCH-02`, `Q-LCH-2` = A: portal/publicação com cache local, lendo `game.server_channel`), a assinatura
(`LLCH-03`, `Q-LCH-3` = A: Ed25519 destacado, verificado antes de aplicar) e a distribuição (`LLCH-04`,
`Q-LCH-4` = A: instalador Tauri NSIS + updater Ed25519 pelo `wyd-ops`). Esta página não afirma URL
publicada nem artefato implementado.

## Limite desta documentação

- Não afirma versão publicada nem tamanho de patch: nada foi publicado.
- Não promete atualização automática implementada: `LLCH-04` decidiu o desenho, mas o updater não existe.
## Decisões registradas

As quatro decisões — `LLCH-01`–`LLCH-04` — estão em [Decisões do `wyd-launcher`](proposta-decisoes-launcher.md); todas fecharam por resposta do dono (`Q-LCH-1`–`Q-LCH-4` = A) e o estado de cada lacuna continua em `lacunas.md`.

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

Quatro perguntas abertas — formato do manifesto, origem da lista de canais, assinatura e
empacotamento — estão em `lacunas.md` (planejamento). Nenhum delas foi decidida; esta página não
afirma formato de arquivo, URL nem algoritmo de hash.

## Limite desta documentação

- Não afirma versão publicada nem tamanho de patch: nada foi publicado.
- Não promete atualização automática do launcher: isso é `LLCH-04`.

# wyd-launcher 1.0.0 — abertura do planejamento

> Pergunta: "o que o launcher entrega, de quem ele recebe manifesto e patch, e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-launcher`
> Dono: equipe do `wyd-launcher`
> Base: `wyd-portal/docs/arquitetura/fronteira.md` §2; `wyd-protocol-1.0.0.md` §1.2; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código**, nenhum requisito numerado (`RF-LCH`). O aplicativo Tauri não existe.

## 1. O que ele faz

Aplicação desktop que abre o jogo no estado certo: confere integridade dos arquivos locais, aplica
patch publicado, mostra notícias e a lista de servidores/canais, e então inicia o cliente. É o dono
do **manifesto, do patch e da integridade** do lado da máquina do jogador.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| conteúdo do patch e versão do cliente | `wyd-client` |
| hospedagem do artefato e da imagem de release | `wyd-ops` |
| lista canônica de canais e estado do servidor | `wyd-server` (topologia) |
| notícias, contas e login de pessoa | `wyd-portal` |
| login no jogo e handshake do fio | `wyd-net` / `wyd-protocol` |
| regra de jogo | `wyd-core` / `wyd-sim` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Estado |
|---|---|---|---|---|---|
| L1 | manifesto de release (versão, hashes, URL de cada arquivo) | launcher ← `wyd-ops` | JSON versionado e canônico, com hash por arquivo | um manifesto publicado descreve exatamente o que o cliente deve ter | **decidido** `LLCH-01` (`Q-LCH-1` = A, 2026-11-23): JSON canônico com `schema`, `channel`, `version`, `client_build`, `min_version` e `files` |
| L2 | origem da lista de canais e do estado do servidor | launcher ← portal/publicação | rota/BFF lendo `game.server_channel` + presença, com cache local | canal fora do ar não aparece como disponível | **decidido** `LLCH-02` (`Q-LCH-2` = A, 2026-11-23): portal/publicação com cache local |
| L3 | notícias e links do portal | launcher ← `wyd-portal` | HTTP do portal | nenhum conteúdo embutido no binário | **proposto** |
| L4 | integridade do arquivo local | launcher decide | hash conferido contra o manifesto | arquivo divergente não inicia | **proposto** |
| L5 | assinatura do manifesto | launcher verifica | Ed25519 destacado sobre bytes canônicos; chave pública no launcher | manifesto adulterado não aplica patch | **decidido** `LLCH-03` (`Q-LCH-3` = A, 2026-11-23): verificação antes de aplicar, chave privada no `wyd-ops` |
| L6 | entrega do executável ao jogador | `wyd-ops` | instalador Tauri (NSIS) + updater Ed25519 | instalador reproduzível | **decidido** `LLCH-04` (`Q-LCH-4` = A, 2026-11-23): GitHub Releases, updater reusa L1/L5 |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-launcher -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-launcher -type f` (só `.gitkeep`) |
| Manifesto publicado | 0 | nenhum artefato de release existe |

## 5. O que falta

`lacunas.md`: as quatro lacunas fecharam em 2026-11-23 pelas respostas do dono (`Q-LCH-1` = A: JSON
versionado e canônico; `Q-LCH-2` = A: portal/publicação com cache local; `Q-LCH-3` = A: Ed25519
destacado; `Q-LCH-4` = A: instalador NSIS + updater pelo `wyd-ops`). Nenhuma lacuna aberta; o gate
ainda deve `RF-LCH` e a árvore de `src/` (itens 3–4).

## 6. Gate para sair do papel

1. Formato do manifesto definido e versionado, com hash por arquivo — **cumprido em 2026-11-23**
   (`Q-LCH-1` = A: JSON versionado e canônico).
2. Política de assinatura decidida (ou recusada com motivo registrado) — **cumprido em 2026-11-23**
   (`Q-LCH-3` = A: Ed25519 destacado, verificado antes de aplicar).
3. `RF-LCH` numerados com critério verificável.
4. Árvore de `src/` planejada; teste E2E com Playwright **não** se aplica — o alvo é Rust e o
   aplicativo é nativo (`AGENTS.md` §8).
5. Nenhum segredo no binário: URL e chave pública, nunca chave privada.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/manifesto-e-patch.md`, `docs/planos/plano-de-testes.md`
e `docs/roadmap/` com DoD.

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
| L1 | manifesto de release (versão, hashes, URL de cada arquivo) | launcher ← `wyd-ops` | arquivo publicado, com hash por arquivo | um manifesto publicado descreve exatamente o que o cliente deve ter | **lacuna** `LLCH-01` |
| L2 | origem da lista de canais e do estado do servidor | launcher ← `wyd-server`/`wyd-ops` | a definir | canal fora do ar não aparece como disponível | **lacuna** `LLCH-02` |
| L3 | notícias e links do portal | launcher ← `wyd-portal` | HTTP do portal | nenhum conteúdo embutido no binário | **proposto** |
| L4 | integridade do arquivo local | launcher decide | hash conferido contra o manifesto | arquivo divergente não inicia | **proposto** |
| L5 | assinatura do manifesto | launcher verifica | a definir | manifesto adulterado não aplica patch | **lacuna** `LLCH-03` |
| L6 | entrega do executável ao jogador | `wyd-ops` | artefato de release | instalador reproduzível | **lacuna** `LLCH-04` |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-launcher -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-launcher -type f` (só `.gitkeep`) |
| Manifesto publicado | 0 | nenhum artefato de release existe |

## 5. O que falta

`lacunas.md`: `LLCH-01` (formato do manifesto), `LLCH-02` (origem da lista de canais),
`LLCH-03` (assinatura) e `LLCH-04` (empacotamento e distribuição).

## 6. Gate para sair do papel

1. Formato do manifesto definido e versionado, com hash por arquivo.
2. Política de assinatura decidida (ou recusada com motivo registrado).
3. `RF-LCH` numerados com critério verificável.
4. Árvore de `src/` planejada; teste E2E com Playwright **não** se aplica — o alvo é Rust e o
   aplicativo é nativo (`AGENTS.md` §8).
5. Nenhum segredo no binário: URL e chave pública, nunca chave privada.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/manifesto-e-patch.md`, `docs/planos/plano-de-testes.md`
e `docs/roadmap/` com DoD.

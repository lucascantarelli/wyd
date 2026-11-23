# `wyd-launcher` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-launcher` tem dono,
> impacto e estado. As quatro lacunas fecharam em 2026-11-23 pelas respostas do dono (`Q-LCH-1`,
> `Q-LCH-2`, `Q-LCH-3` e `Q-LCH-4` = A); nenhuma segue aberta.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Patch e manifesto | 2 | 0 | 0 | 2 |
| Servidor e conteúdo | 1 | 0 | 0 | 1 |
| Distribuição | 1 | 0 | 0 | 1 |
| **Total** | **4** | **0** | **0** | **4** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LLCH-01` | Qual é o formato do manifesto: campos, hash por arquivo, versão e compatibilidade? | é o contrato do patch; sem ele, launcher e `wyd-client` divergem silenciosamente | arquitetura + `wyd-client` | fechada (2026-11-23: decisão do dono, `Q-LCH-1` opção **A** — **JSON versionado e canônico**, UTF-8, campos `schema`, `channel`, `version`, `client_build`, `min_version` e `files[{path, size, sha256, url}]`, canonização RFC 8785; casa com `portal.launcher_release` (`0030`); não é migration nem código; registro em `docs/proposta-decisoes-launcher.md` §4 e §6) |
| `LLCH-02` | De onde vem a lista de canais e o estado de cada um: portal, arquivo publicado ou o próprio servidor? | lista errada manda o jogador para canal morto; o portal já publica a página de canais no planejamento | arquitetura + `wyd-server` | fechada (2026-11-23: decisão do dono, `Q-LCH-2` opção **A** — **portal/publicação com cache local**, lendo `game.server_channel` (capacidade, manutenção, papel, `status_url`) e a presença de `v_live_session`; assinatura não entra na lista; 0 migration; registro em `docs/proposta-decisoes-launcher.md` §4 e §6) |
| `LLCH-03` | O manifesto é assinado, e o launcher verifica a assinatura? | patch adulterado é execução de código arbitrário na máquina do jogador | arquitetura + segurança | fechada (2026-11-23: decisão do dono, `Q-LCH-3` opção **A** — **Ed25519 destacado** sobre os bytes canônicos do manifesto, verificado antes de aplicar; chave pública + `key_id` no launcher, chave privada só no secret do `wyd-ops`, assinatura publicada como irmã `manifest.json.sig`; 0 migration, 0 coluna; registro em `docs/proposta-decisoes-launcher.md` §4 e §6) |
| `LLCH-04` | Como o aplicativo é empacotado e distribuído (instalador, atualização do próprio launcher)? | `wyd-ops` publica release; sem formato, a release não tem artefato | `wyd-ops` | fechada (2026-11-23: decisão do dono, `Q-LCH-4` opção **A** — **instalador Tauri (NSIS) + updater Ed25519**, publicado em GitHub Releases pelo `wyd-ops`; `portal.launcher_release` alimenta o `/download`; Authenticode fica fora do gate; 0 migration; registro em `docs/proposta-decisoes-launcher.md` §4 e §6) |

## 3. O que não é lacuna deste sistema

- Conteúdo do patch: dono é `wyd-client`.
- Hospedagem e imagem: dono é `wyd-ops`.

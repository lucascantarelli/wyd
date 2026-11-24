# Decisões do `wyd-launcher` — manifesto, canais, assinatura e distribuição

> Pergunta: "qual é o formato do manifesto, de onde vem a lista de canais, o manifesto é assinado e
> como o launcher é distribuído?"
> Público: arquitetura, `wyd-launcher`, `wyd-client`, `wyd-server` e `wyd-ops`
> Dono: equipe do `wyd-launcher`
> Base: `lacunas.md` (`LLCH-01`–`LLCH-04`); `docs/index.md`; `wyd-launcher-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §7.1, §7.3 (X11) e §7.6; `AGENTS.md` §5, item 7 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Nenhuma pergunta segue aberta; `Q-LCH-1`–`Q-LCH-4` foram respondidas
pelo dono (= A, §4) e nada foi aplicado. Não há patch, release nem manifesto; nenhum número desta página
é medição.

## 1. Fronteira

**Fecha:** as quatro lacunas do launcher. `LLCH-01` (formato do manifesto), `LLCH-02` (origem da lista
de canais), `LLCH-03` (assinatura) e `LLCH-04` (empacotamento/distribuição) fecharam em 2026-11-23 pelas
respostas do dono (`Q-LCH-1`–`Q-LCH-4` = A).

**Não decide:** conteúdo do patch (`wyd-client`), hospedagem (`wyd-ops`), topologia de canais
(`wyd-server`), login (`wyd-net`/`wyd-protocol`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **0 abertas, 0 propostas, 4 fechadas** | `lacunas.md` §1 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-launcher-1.0.0.md` §4 |
| manifesto, patch ou release | **0** | idem |
| contrato X11 | manifesto com hash por arquivo: **decidido em 2026-11-23** (JSON canônico + Ed25519) | `wyd-1.0.0.md` §7.3 |
| testes | **0**; alvo nativo/script de fumaça | `wyd-1.0.0.md` §7.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| captura e golden | processo `LPRT-10` decidido; captura é do `wyd-client` | `LPRT-10`; X12 | `LLCH-01` consome o resultado, não o redefine |
| fronteira do conteúdo | conteúdo e versão do cliente são do `wyd-client`; hospedagem é do `wyd-ops` | `docs/index.md` | a lacuna não decide conteúdo nem hosting |

## 4. Decisões

### Q-LCH-1 — Qual é o formato do manifesto?

**Medido.** Não há manifesto, schema nem versão; `X11` diz "manifesto com hash por arquivo" e está em
lacuna. O launcher e o cliente ainda não têm contrato fechado.

**O que está em jogo.** Um formato legível e versionado permite verificar arquivo a arquivo; um
formato binário é compacto, mas difícil de auditar; reutilizar o legado não tem prova.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | JSON versionado: lista de arquivos, `sha256` por arquivo, versão do patch e compatibilidade | auditável e simples; exige schema e canonização para assinar |
| B | Formato binário compacto | menor e mais rápido; difícil de revisar e de evoluir sem ferramenta |
| C | Reutilizar o manifesto do legado | começa com formato conhecido; não há prova de que cobre hash/versão/compatibilidade |

**Recomendação: A.** É o menor formato que cumpre X11 e pode ser assinado por inteiro. **Custo:** um
schema e uma canonização. **Fontes:** `lacunas.md` `LLCH-01`; `wyd-1.0.0.md` X11;
`wyd-launcher-1.0.0.md` §3. **Recusa B** por auditabilidade; **C** por falta de prova.

**Resposta do dono em 2026-11-23: A.** O manifesto é **JSON versionado e canônico** — UTF-8, campos
`schema`, `channel`, `version`, `client_build`, `min_version` e `files[{path, size, sha256, url}]` —,
com canonização RFC 8785 para que a assinatura cubra bytes estáveis. O registro de publicação já
existe: `portal.launcher_release` guarda `channel` (`stable`/`beta`), `version` semver, `manifest_url`,
`manifest_sha256`, `min_version` e status (`0030-portal-content.md:172–203`). A decisão fixa o formato;
**não é migration nem código**, e nenhum manifesto foi criado.

| Alternativa | Por que foi descartada |
|---|---|
| B — binário compacto | exige codec e ferramenta de inspeção; não há ganho de tamanho que pague a auditabilidade |
| C — texto do legado | não tem canal, `min_version`, URL nem assinatura; `deprecated/` não se copia (`AGENTS.md` §5, item 2) |

**Custo do que A ainda exige, não implementado.** **1 página** em `docs/specs/manifesto-e-patch.md`
(planejada em `wyd-launcher-1.0.0.md` §7), **1 JSON Schema**, **1 módulo**
`crates/wyd-launcher/src/manifest.rs` (o crate precisa de `Cargo.toml`) e **1 fixture**; **0 migration**
— `portal.launcher_release` já está em `0030`.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LLCH-01` | `lacunas.md` §1 |
| contrato L1 decidido e item 1 do gate cumprido | `wyd-launcher-1.0.0.md` §3 e §6 |
| `X11` passou a depender só de `LLCH-03` | `wyd-1.0.0.md` §7.3 |
| libera `Q-LCH-3`, que assina o formato já fixado | §4 |

**Não fecha:** `LLCH-02` (origem da lista de canais, `wyd-server` + portal), `LLCH-03` (assinatura,
arquitetura + segurança), `LLCH-04` à época (pacote e release, `wyd-ops`; fechada depois por
`Q-LCH-4`), a implementação do manifesto, `release.yml`/`/admin/launcher` e qualquer `LPRT-*`.

### Q-LCH-2 — De onde vem a lista de canais e o estado de cada um?

**Medido.** Não há fonte decidida. O portal tem `/admin/canais` no planejamento, mas a topologia é do
`wyd-server`; o `wyd-launcher` só consome. A lacuna está aberta.

**O que está em jogo.** Lista no portal é uma fonte editorial; arquivo publicado funciona offline;
o servidor no login é a verdade operacional, mas exige conexão antes de mostrar a lista.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | Portal/publicação como fonte, com cache local do launcher e estado por canal | uma fonte editorial e offline; exige atualização e cache |
| B | Arquivo estático dentro da release | simples e offline; muda com a release, não com a operação |
| C | O próprio servidor no login | estado real; o jogador só vê a lista depois de conectar |

**Recomendação: A.** O portal já tem a tela `/admin/canais`; usá-la como fonte exige uma rota pública
ou contrato de leitura, mas evita uma lista presa à release. **Custo:** cache, invalidação e contrato
com o portal. **Fontes:** `lacunas.md` `LLCH-02`; `docs/index.md`;
`wyd-portal/docs/arquitetura/consumo-do-banco.md` §2 e `wyd-portal/docs/arquitetura/rotas-e-permissoes.md` rota 37; `LLCH-02`.
**Recusa B** por desatualizar; **C** por inverter a ordem.

**Resposta do dono em 2026-11-23: A.** A lista de canais e o estado vêm do **portal/publicação**, com
**cache local** no launcher: a rota lê `game.server_channel` (capacidade, manutenção, papel,
`status_url`) e a presença de `game.v_live_session`; o launcher guarda a última lista boa e gera o
`serverlist.bin` (10 × 11 × 64 B, célula 0 = status, 1–10 = host) a partir dela.
`portal.launcher_release` continua sendo o canal de **release** (`stable`/`beta`, versão), não o canal
de jogo. A decisão fixa a fonte; **não é migration nem código**.

| Alternativa | Por que foi descartada |
|---|---|
| B — arquivo na release | cada mudança de canal/manutenção exige nova release; o estado congela |
| C — servidor no login | exige conectar e provar o handshake antes da lista; depende de `LPRT-03`/`LPRT-06` |

**Custo do que A ainda exige, não implementado.** **1 página** `docs/specs/lista-de-canais.md`, **1
rota/BFF** em `wyd-portal`, **1 módulo** de leitura/cache e **1 gerador** de `serverlist.bin` em
`crates/wyd-launcher` (o crate precisa de `Cargo.toml`) e **1 fixture**; **0 migration, 0 tabela, 0
coluna** — `game.server_channel` e `game.session`/`v_live_session` já existem.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LLCH-02` | `lacunas.md` §1 |
| contrato L2 decidido | `wyd-launcher-1.0.0.md` §3 |
| libera `Q-LCH-4`, última pergunta do launcher | §4 |

**Não fechou:** `LLCH-04` à época (pacote/release, `wyd-ops`; fechada depois por `Q-LCH-4`), a rota/BFF,
`/admin/canais`, o cache, o gerador de `serverlist.bin`, a presença viva do `wyd-server` e qualquer
`LPRT-*`.

### Q-LCH-3 — O manifesto é assinado e o launcher verifica?

**Medido.** Não há assinatura, chave nem verificação; o risco é execução de código arbitrário por
patch adulterado. A lacuna está aberta.

**O que está em jogo.** Sem assinatura, o hash só protege contra corrupção, não contra troca; uma
assinatura exige gestão de chave e rotação.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | Assinatura destacada do manifesto (Ed25519) verificada antes de aplicar, com chave pública no launcher | integridade autenticada; exige chave, rotação e cerimônia de assinatura |
| B | Só `sha256` no manifesto | detecta corrupção; não autentica a origem |
| C | Sem assinatura | mínimo; patch adulterado passa |

**Recomendação: A.** O risco declarado é execução arbitrária; hash sem assinatura não fecha esse
risco. **Custo:** gestão de chave. **Fontes:** `lacunas.md` `LLCH-03`; `AGENTS.md` §5, item 7;
`wyd-launcher-1.0.0.md` §3. **Recusa B** por não autenticar; **C** por risco aceito sem decisão.

**Resposta do dono em 2026-11-23: A.** O manifesto leva **assinatura Ed25519 destacada** sobre os bytes
canônicos (RFC 8785), verificada **antes de aplicar**; o launcher embute a **chave pública** e o
`key_id`, a **chave privada** fica só no secret do `wyd-ops`, e a assinatura é publicada como irmã
`manifest.json.sig`. É o desenho que o legado já usava
(`deprecated/crates/wyd-launcher/core/src/distribuicao.rs:4–12`), mas a nova implementação não copia
código; a decisão fixa a política, não cria chave nem verificador. A rotação exige novo launcher com a
nova chave pública (ou lista de chaves), sem migration.

| Alternativa | Por que foi descartada |
|---|---|
| B — só `sha256` | detecta corrupção, não autentica a origem; adicionar assinatura depois muda formato, launcher e release |
| C — sem verificação | qualquer patch adulterado passa; o risco de execução arbitrária não fica aceito por decisão |

**Custo do que A ainda exige, não implementado.** **1 página** `docs/specs/manifesto-e-patch.md`, **1
assinador** em `tools/launcher/` (o diretório não existe), **1 módulo** de verificação em
`crates/wyd-launcher/src/` (o crate precisa de `Cargo.toml`), **1 chave pública** embutida, **1 fixture**
manifesto+assinatura e **1 passo** no futuro `release.yml`; **0 migration, 0 tabela, 0 coluna** — a
assinatura é irmã do manifesto e `portal.launcher_release` já guarda `manifest_sha256`.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LLCH-03` | `lacunas.md` §1 |
| contrato L5 decidido e item 2 do gate cumprido | `wyd-launcher-1.0.0.md` §3 e §6 |
| `X11` decidida: `LLCH-01` e `LLCH-03` fecharam | `wyd-1.0.0.md` §7.3 |
| libera `Q-LCH-4`, que assina o pacote com a mesma chave | §4 |

**Não fechou:** `LLCH-02` (origem da lista de canais, `wyd-server` + portal) e `LLCH-04` à época (pacote
e release, `wyd-ops`; fechada depois por `Q-LCH-4`), a cerimônia de chave, o assinador, o verificador,
`release.yml`, `/admin/launcher` e qualquer `LPRT-*`.

### Q-LCH-4 — Como o aplicativo é empacotado e distribuído?

**Medido.** Não há instalador, release nem atualização; `wyd-ops` publica release e a lacuna está
aberta. Nada foi publicado.

**O que está em jogo.** Instalador assinado com atualização é o fluxo completo, mas exige infra; zip
manual é simples e não atualiza; sem pacote não há release.

| Opção | O que é | Consequência |
|---|---|---|
| **A (escolhida pelo dono)** | Instalador assinado + atualização do launcher, publicados por `wyd-ops` | fluxo completo; exige assinatura, hosting e rollback |
| B | Zip/portable manual | simples; sem atualização e sem verificação de versão |
| C | Sem empacotamento nesta versão | não bloqueia código; não há release |

**Recomendação: A.** É o único formato que entrega atualização e verificação; a assinatura de Q-LCH-3
se aplica ao pacote. **Custo:** pipeline e hosting. **Fontes:** `lacunas.md` `LLCH-04`;
`wyd-launcher-1.0.0.md` §2; `AGENTS.md` §9. **Recusa B** por não atualizar; **C** por não entregar.

**Resposta do dono em 2026-11-23: A.** O launcher é distribuído como **instalador Tauri (NSIS)** com
**updater Ed25519**, publicado em GitHub Releases pelo `wyd-ops`; o `/download` do portal lê
`portal.launcher_release` (versão, URL do manifesto, hash, `min_version`) e o updater reutiliza o
manifesto e a assinatura de L1/L5. Authenticode (certificado pago) fica fora do gate; a assinatura do
updater é a mesma família Ed25519 de `Q-LCH-3`. Não é migration nem código.

| Alternativa | Por que foi descartada |
|---|---|
| B — zip/portable | não atualiza nem confere versão; adicionar updater depois exige reinstalar a base |
| C — nada | deixa a `1.0.0` sem artefato de entrada e adia todo o caminho de A |

**Custo do que A ainda exige, não implementado.** **1 crate Tauri** em `crates/wyd-launcher` com 6
arquivos mínimos (`Cargo.toml`, `build.rs`, `tauri.conf.json`, `src/main.rs`, `web/index.html`,
`capabilities/default.json`, além de ícones), **1 workflow** `.github/workflows/release.yml`, **1 feed**
`latest.json` gerado, **1 secret** de assinatura e **1 rota** `/download` em `wyd-portal`; **0 migration**
— a tabela `portal.launcher_release` já existe. O gate ainda deve `RF-LCH` e árvore de `src/` (itens 3–4).

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| fecha `LLCH-04` e o contrato L6 | `lacunas.md` §1; `wyd-launcher-1.0.0.md` §3 |
| launcher passa a 0 lacunas abertas | `lacunas.md` §1 |
| libera a onda E de release | `wyd-1.0.0.md` §7.4 |

**Não fecha:** a implementação do crate/instalador/updater/feed, `release.yml`, `/download`, o secret,
Authenticode, `RF-LCH`, a árvore de `src/` e qualquer `LPRT-*`.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LLCH-01` | `Q-LCH-1` | fechada em 2026-11-23 (A): JSON versionado e canônico, `sha256` por arquivo, `channel`/`version`/`min_version`; casa com `portal.launcher_release`; manifesto não implementado. |
| `LLCH-02` | `Q-LCH-2` | fechada em 2026-11-23 (A): portal/publicação com cache local, lendo `game.server_channel`; 0 migration; não implementado. |
| `LLCH-03` | `Q-LCH-3` | fechada em 2026-11-23 (A): Ed25519 destacado, verificado antes de aplicar; chave pública no launcher, privada no `wyd-ops`, assinatura irmã; não implementado. |
| `LLCH-04` | `Q-LCH-4` | fechada em 2026-11-23 (A): instalador Tauri NSIS + updater Ed25519, publicado pelo `wyd-ops`; 0 migration; não implementado. |

## 6. Registro da decisão

**`Q-LCH-1`–`Q-LCH-4` foram respondidas; nenhuma lacuna continua aberta e nada foi aplicado.** O
briefing das respostas (`../../briefing-de-decisoes-1.0.0.md` §6) registra a ordem formato → assinatura
→ pacote, que era dependente nessa sequência; a recomendação não muda.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-LCH-1` | `LLCH-01` | A — JSON versionado e canônico, `sha256` por arquivo, `channel`/`version`/`min_version`; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-LCH-2` | `LLCH-02` | A — portal/publicação com cache local, lendo `game.server_channel`; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-LCH-3` | `LLCH-03` | A — Ed25519 destacado, verificado antes de aplicar; chave pública/`key_id` no launcher, privada no `wyd-ops`, assinatura irmã; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |
| `Q-LCH-4` | `LLCH-04` | A — instalador Tauri NSIS + updater Ed25519, publicado pelo `wyd-ops`; resposta do dono em 2026-11-23, decisão registrada e lacuna fechada |

# Análise de `tools/`, `scripts/` e da suíte de testes

> **Data:** 2026-11-15 · **Escopo:** `tools/` (71 arquivos), `apps/tools/` (10),
> `scripts/` (2), `crates/*/src` + `crates/*/tests`, `benches/`, `wyd-portal/tests`,
> `.github/workflows/`, `Makefile`, `docker-compose*.yml`.
>
> **Método:** leitura direta do código e dos `README.md` de cada pasta, contagem com
> `grep`/`find` na árvore, cruzamento com `docs/arquitetura/testes.md` e
> `docs/arquitetura/visao-pastas.md`. Nada aqui foi copiado de outro documento sem
> conferência no código. Onde a contagem de hoje divergiu do doc normativo, isso está
> dito explicitamente.

---

## 0. Correção de premissa: não existe `tools/scripts`

A pasta `tools/scripts` **não existe**. O que existe são três coisas distintas, e a
confusão entre elas é a raiz do "não está claro o que cada script faz":

| Caminho | O que é | Arquivos |
|---|---|---|
| `tools/` | **biblioteca** de ferramentas (parsers, geradores, gates, captura) | 71 |
| `apps/tools/` | **CLI `wyd-tools`** — a porta de entrada oficial (ADR-0028) | 10 |
| `scripts/` | 2 scripts soltos na raiz (geração do `client.zip` e prova do reaper) | 2 |

Hoje `tools/` mistura cinco papéis diferentes numa árvore só:

1. **biblioteca importada por código** (`pe.mjs`, `strings.mjs`, `lib.mjs`…);
2. **gates do `make check`/CI** (`check-*.mjs`, `bench-gate.mjs`…);
3. **geradores de artefato versionado** (`gen-seed.mjs`, `gen-enciclopedia.mjs`,
   `dist-manifest.mjs`…);
4. **ferramentas de operador** (`.ps1` de captura de tela/OCR, `prova_reaper.sh`);
5. **provas pontuais** de uma issue (`ataque-do-legado.mjs`, `nivel-do-legado.mjs`…).

Essa mistura é o motivo real da falta de clareza — não o número de arquivos. A §10
propõe a separação em comandos de uma `wyd-tools` real.

**Já existe uma `wyd-tools`**: `apps/tools/bin/wyd-tools.mjs`, com 6 subcomandos
(`inventario`, `binarios`, `strings`, `pacotes`, `docs`, `zip-check`). A ideia geral do
pedido é, na prática, **absorver os outros 60 e poucos arquivos** nela — ver §10.

---

## 1. Mapa das pastas

```
tools/
├── README.md                  porta de entrada + regras da pasta
├── client/          (27)      engenharia reversa do WYD.exe 7662 + captura ao vivo
├── content/         (13)      seeds, catálogo do banco, enciclopédia, migrations
├── dev/             (19)      gates de operação, CI, monitoramento, deploy
├── launcher/         (6)      manifesto, canais, assinatura, E2E da janela
├── protocol/         (1)      gen-protocol.mjs (CONGELADO)
└── cliente-harness/  (4)      proxy TCP + replay de sessão real
```

Tudo é **Node ESM sem dependências** (não há `package.json` em `tools/`; os scripts
rodam com `node arquivo.mjs`). Dois módulos são PowerShell (`.ps1`) e só rodam no
Windows com elevação. Nada em `tools/` é runtime: nenhum `crate/` importa daqui.

### Regra que já existe (e vale manter)

`tools/README.md` e `docs/arquitetura/visao-pastas.md` §1.1 dizem: *script descartável
nasce em `tmp/`; `tools/` só recebe o que tem propósito declarado*. Como se vê na §9,
essa regra foi furada em vários pontos (provas de issue viraram arquivos permanentes).

---

## 2. Inventário completo — arquivo por arquivo

Legenda de **situação**:

| Sigla | Significado |
|---|---|
| `gate` | executado automaticamente por `make` e/ou CI; falha quebra o build |
| `bib` | biblioteca importada por outro script/app (não roda sozinha) |
| `ger` | gera artefato versionado (doc, JSON, PNG, SQL) |
| `oper` | ferramenta de operador (manual, Windows, algumas exigem elevação) |
| `prova` | prova pontual de uma issue; não é chamada por nada automatizado |
| `cong` | congelada: só existe para documentar/ conferir o processo |

### 2.1 `tools/client/` — 27 arquivos

Pipeline reproduzível de RE do cliente 7662. Entra `WYD.exe` (+ assets), saem
artefatos versionados em `docs/cliente/gerado/`. Plano: `docs/cliente/engenharia-reversa.md`.

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `analyse.mjs` | Orquestra a análise estática inteira e grava `docs/cliente/gerado/*.json`/`ancoras.txt`. Exige SHA-256 do build de referência (falha em outro build). | manual; docs/cliente | `ger` |
| `pe.mjs` | Parser PE32/PE32+: `parsePe`, `vaToOffset`, `offsetToVa`, `readImports`, `readExports`, `readRichHeader`. Sem I/O. | **importado** por `apps/tools/lib/inventario.mjs` e `pe-analise.mjs` | `bib` |
| `strings.mjs` | Extrai/indexa/classifica strings ASCII e UTF-16 (`extractStrings`, `classify`, `indexStrings`). | **importado** por `apps/tools/lib/pe-analise.mjs` | `bib` |
| `keytable.mjs` | Tabela `pKeyWord[512]` e transformação CPSock — paridade com `crates/wyd-protocol`. | `captura-live.mjs`; docs | `bib` |
| `evidence.mjs` | Confirma/refuta âncoras herdadas (`ANCHORS`, `PATCH_SITES`), chama `objdump` se existir. | `analyse.mjs` | `bib` |
| `formats.mjs` | Estrutura de `.msa`, `sn.bin`, `serverlist.bin`; delega `.wyt` para `wyt.mjs`. | `analyse.mjs` | `bib` |
| `wyt.mjs` | Decodifica/exporta `.wyt` (`WT10`, raster puro) para PNG/BMP/JSON; `--scan`. | manual; gera `wyt.json` | `bib`/`ger` |
| `serverlist.mjs` | Gera/inspeciona `serverlist.bin` (grade 10×11×64, ofuscação própria). | `tools/launcher/canais.mjs`; **golden `serverlist_paridade`** | `bib` |
| `serverlist-servers.txt` | Dados de entrada do `serverlist.mjs` (lista de servidores). | `serverlist.mjs` | dado |
| `extract-tables.mjs` | `ItemList.bin`/`SkillData.bin`: XOR, registro e perfil de colunas → `docs/cliente/gerado/tabelas.json`. | manual; `docs/cliente/tabelas-bin.md` | `ger` |
| `extrair-icones.mjs` | `itemicon.bin` + `UI/itemicon01..10.wyt` → `wyd-portal/public/game-assets/itens/*.png` + `icones.json`. | **asset do portal** (enciclopédia de itens) | `ger` |
| `map-skills.mjs` | Mapeia skills do cliente para o catálogo → `docs/cliente/gerado/skills-map.json`. | manual | `ger` |
| `captura.mjs` | Gera (`--gen`) e valida (`--check`) goldens do caminho login→campo a partir de `payload.rs` + tabela de opcodes. | origem de `crates/wyd-protocol/tests/fixtures/cliente/*.json` | `ger` |
| `captura-live.mjs` | Converte captura JSON em fixtures e confere com o servidor. | manual; `docs/cliente/depuracao.md` | `prova` |
| `diff-captura.mjs` | Diff de duas capturas: opcode, tamanho e offsets que mudaram. | manual; `docs/cliente/captura-selchar.md` | `prova` |
| `trade-layout.mjs` | Decide o layout de `MSG_Trade` (156 B natural × 154 B packed) e valida campos. | manual; `docs/cliente/depuracao.md` | `prova` |
| `strdef.mjs` | Resolve id de mensagem do binário (`0x109a4b8` + n×128) em `client/UI/strdef.bin`. | manual; `docs/cliente/pacotes.md` | `prova` |
| `strdef-patch.mjs` | Reescreve um registro de `strdef.bin` em cp1252 com backup, offsets e checksum recalculado. | manual (evidências de login/OCR) | `oper` |
| `ataque-do-legado.mjs` | Parser **independente** de `_MSG_Attack` (0x0367) / `_MSG_AttackOne` (0x039D) sobre `tmp/combate-pacote/*.txt`, com offsets do legado (não usa o crate Rust). | evidência `combate-pacote-2026-11-14.md` | `prova` |
| `nivel-do-legado.mjs` | Parser independente do nível no fio (`STRUCT_SCORE`/`STRUCT_MOB`/`STRUCT_SELCHAR`) sobre `tmp/nivel-wire/*.txt`. | evidência `nivel-wire-2026-11-14.md` | `prova` |
| `captura-print.ps1` | Screenshot da janela D3D9 (`PrintWindow`/`BitBlt`); exige elevação. | operador | `oper` |
| `captura-tela.ps1` | Captura a tela em intervalo fixo (`target/tela/*.jpg`), porque `PrintWindow` de D3D9 volta preto. | evidências de login/tela | `oper` |
| `captura-sessao.ps1` | Captura handshake/login real: listeners 8281/80, `SendInput` absoluto + scan code, digita conta/senha, captura a recusa e valida o texto via OCR contra `strdef.bin`. | evidências de autenticação/PIN | `oper` |
| `ocr-texto.ps1` | OCR offline (`Windows.Media.Ocr`) em fatias + comparação normalizada com um registro de `strdef.bin`; exit 0 confere / 1 divergiu / 2 sem OCR. | `captura-sessao.ps1` | `oper` |
| `ocr-teclado.ps1` | OCR **com caixas**: localiza cada palavra desenhada e devolve o centro em coordenadas da janela, para o harness clicar. | manual (teclado de PIN) | `oper` |
| `teclado-pin.ps1` | Reconhece o teclado de PIN embaralhado por **máscara + distância de Hamming** (o OCR não lê esses glifos) e devolve o centro de cada dígito. | manual (evita bloquear conta por 5 PINs errados) | `oper` |
| `README.md` | Contrato de cada módulo, padrões (pureza, determinismo) e limitações registradas. | humano | doc |

**Observação:** `ocr-teclado.ps1` e `teclado-pin.ps1` resolvem o **mesmo problema** (clicar
no dígito certo de um teclado embaralhado). O próprio cabeçalho de `teclado-pin.ps1`
registra que o OCR **não** lê esses glifos — ou seja, um dos dois é abordagem vencida.
Candidato a fusão (ver §9/§10).

### 2.2 `tools/content/` — 13 arquivos

Conteúdo e banco. Ferramenta **offline**: nada aqui roda em runtime; o servidor lê o
PostgreSQL (ADR-0009).

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `gen-seed.mjs` | Converte o conteúdo legado (`git show origin/main`) em migrations `0017_seed_item_catalog.sql`, `0018_seed_skill_catalog.sql`, `0019_seed_mundo.sql`. Sanitiza (UTF-8 estrito, NFC, NBSP) e **falha** em vez de "consertar" por chute. | manual (`--dry-run`); docs | `ger` |
| `gen-catalogo-banco.mjs` | Gera `docs/arquitetura/banco-catalogo.md` a partir das migrations + curadoria. `--check` falha se estiver desatualizado. | `make banco-catalogo` | `ger` |
| `gen-enciclopedia.mjs` | Gera `wyd-portal/src/content/itens.json` a partir do seed `0017` (a imagem do portal não tem o SQL). | manual; portal | `ger` |
| `gen-skill-lineage.mjs` | Gera a migration que fecha `skill_catalog.class_code`/`lineage_code` só para linhagem publicada em fonte oficial. | manual | `ger` |
| `banco-fontes.mjs` | Metadados **curados** por tabela: domínio, fonte, `confirmado`/`provisorio` e nota. | `gen-catalogo-banco.mjs` | `bib` |
| `banco-parse.mjs` | Parser único da série de migrations (lê o *estilo* da série, falha fora dele). | `gen-catalogo-banco`, `verify-schema`, `migrate-cycle` | `bib` |
| `pg.mjs` | Helpers de Postgres efêmero (Docker + `psql`), com espera de **duas** consultas reais (a imagem sobe servidor temporário no init). | `migrate-cycle`, `verify-schema` | `bib` |
| `check-migrations.mjs` | Valida ordem, FKs, duplicidade e `down` da série **sem Postgres**. | **`make check`** → `migrations-check`; CI `conteudo` | `gate` |
| `check-issue-status.mjs` | Status de cada issue igual entre `roadmap-rust/index.md` e `docs/rust/03-progresso.md`. | **`make check`** → `issue-status-check`; CI | `gate` |
| `check-audit-coverage.mjs` | Todo item de `docs/audit/` precisa de issue rastreável. | **`make check`** → `audit-coverage-check` | `gate` |
| `verify-schema.mjs` | Container efêmero → migrations → lê `information_schema` → compara com `banco-catalogo.md`. | `make banco-verify` (exige Docker) | `gate`(manual) |
| `migrate-cycle.mjs` | `up` → contagens → `down` → `up` de novo num Postgres real, comparando contagens. | `make migrations-apply` (Docker) | `gate`(manual) |
| `README.md` | Uso, sanitização, parsers, asserções e proveniência (`content_source`). | humano | doc |

### 2.3 `tools/dev/` — 19 arquivos

Gates de operação e CI. **É a pasta que mais conversa com o `Makefile`.**

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `check-docs-consistency.mjs` | Membros do `Cargo.toml` × `crates/*`; contagem/nomes de crates em `README`/`AGENTS`; links relativos. Não corrige nada. | **`make check`** → `docs-check`; CI | `gate` |
| `check-skills.mjs` | Frontmatter, nome = diretório, descrição 1–1024, corpo ≤500 linhas, recursos citados existem, caminhos `.agents/skills/*` existem. | **`make check`** → `skills-check`; CI | `gate` |
| `check-versao.mjs` | Fonte única de versão: workspace × crates × `wyd-portal/package.json`. | **`make check`** → `versao-check`; `release.yml` | `gate` |
| `check-actions-pin.mjs` | Cada `uses:` pinada por SHA de 40 hex, com comentário humano e SHA conferido **na API do GitHub**. | **`make check`** → `actions-pin-check`; CI | `gate` |
| `check-compose-hardening.mjs` | Contrato de superfície do compose de prod (R1 TAG obrigatória, R2 Grafana sem anônimo, R3 limites, R4 lifecycle fora do base…). Só texto, sem Docker. | CI `infra`; `make compose-hardening` | `gate` |
| `check-alertas.mjs` | Valida o Alertmanager **renderizado** (template → `render-config.sh` → `amtool check-config`). | `make monitoring-rules`; CI | `gate` |
| `bench-gate.mjs` | Lê as linhas `BENCH_PONTO {json}` do bench de release e falha por cenário (e falha se um cenário **deixar de reportar**). | `make bench-gate`; `bench.yml` | `gate` |
| `monitoring-smoke.mjs` | Stack de observabilidade viva: Prometheus ready, 5 alvos `up`, regras carregadas, Grafana + datasource + dashboard, Alertmanager com receiver e `WydWatchdog`. | `make monitoring-check` | `gate`(manual) |
| `alerts-prova.mjs` | Prova de rota + disparo + **resolução** de alerta (fecha o ciclo do `resolved`). | `make monitoring-alertas` | `gate`(manual) |
| `alert-sink.mjs` | Receptor local do Alertmanager; cada notificação vira linha em `ALERT_SINK_DIR/alertas.jsonl`. É um **serviço** do compose (`alerts-out`). | `docker-compose.yml` (serviço) | serviço |
| `deploy-provas.mjs` | 4 provas do deploy local: `/login` 200, `/entregas` 307, `/health` com `schema` = `SCHEMA_VERSION`, `/api/health` 200. | `make deploy-local`; CI job `deploy` | `gate` |
| `reportar-deploy.mjs` | Publica falha de deploy no Alertmanager (`WydDeployLocalFalhou`); sucesso não vira alerta. | CI `deploy` | `gate` |
| `anotar-deploy-grafana.mjs` | Anotação de deploy no Grafana (commit + run na linha do tempo). Falha só emite `::warning::`. | CI `deploy` | `gate` |
| `notify-ci.mjs` | Resumo de CI no Slack com tempos reais por job; PR verde não fala. | CI `notify` | `gate` |
| `notify-release.mjs` | Anúncio de release em `#wyd-releases` com binários e checksums. | `release.yml` | `gate` |
| `clean-tmp.mjs` | Remove só artefatos que o projeto gerou em `tmp/`; **preserva** `tmp/captura*`, `tmp/basemob` e PNGs do operador. | `make clean-tmp` | `gate` |
| `limpar-qa.mjs` | Contas de QA acumuladas (login terminando em 5+ dígitos) no banco de teste: contagem, amostra e remoção transacional com `--aplicar`. | `make db-limpa-qa` | `oper` |
| `.gitkeep` | Mantém a pasta no git. | — | — |
| `README.md` | Tabela script → alvo → o que valida. | humano | doc |

### 2.4 `tools/launcher/` — 6 arquivos

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `dist-manifest.mjs` | Gera `manifest.txt` (formato lido por `crates/wyd-launcher/src/manifest.rs`: `# wyd-launcher-manifest v1`, `build`, `protocol`, `<sha256> <tamanho> <caminho>`). `--check` falha se o manifesto mentir. | `docs/cliente/launcher.md`; manual | `ger` |
| `canais.mjs` | Deriva `canais.txt` do que o cliente **real** tem (`serverlist.bin` + `sn.bin`) e confere o round-trip byte a byte. | manual; launcher | `ger` |
| `sign-package.mjs` | Ed25519: `keygen`/`pubkey`/`sign`. Grava `<pacote>/assinatura` (128 hex) conferido por `distribuicao::verify_package`. Chave privada só no secret do CI. | manual/CI de release; ADR-0003 | `gate`(release) |
| `sign-config.mjs` | HMAC-SHA256 sobre os bytes exatos de `launcher.conf`, anexando `assinatura = <64 hex>`. | manual; `docs/cliente/launcher.md` | `oper` |
| `e2e-janela.mjs` | **E2E real da janela** com `tauri-driver` + `msedgedriver`: portal falso, espera notícia, clica em Jogar, prova que o jogo abriu **uma** vez e que manifesto ausente não abre nada. | manual; CLIENT-08 | `prova` |
| `launcher.conf.exemplo` | Corpo de exemplo para o `sign-config.mjs`. | `sign-config.mjs` | dado |

### 2.5 `tools/protocol/` — 1 arquivo

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `gen-protocol.mjs` | **CONGELADO desde 2026-11.** Gerava `opcode_table.rs`, `keyword_table.rs` e o §4 de `protocolo.md` a partir de `Basedef.h`/`CPSock.cpp` (fonte histórica removida). Hoje só `--check`; falha explicitamente se a fonte não existir. Tabelas vigentes em `crates/wyd-protocol/src/wire/generated/`. | manual | `cong` |

### 2.6 `tools/cliente-harness/` — 4 arquivos

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `lib.mjs` | Núcleo: `FrameSplitter` (delimita frames CPSock só pelo `Size` u16 LE, sem conhecer a keyword), `startProxy`, `readSession`, `replaySession`. | `proxy.mjs`, `replay.mjs`, `harness.test.mjs` | `bib` |
| `proxy.mjs` | Grava sessão real: `cliente → proxy → canal`, JSONL com `Size`/hex das duas direções. | **`make harness-cliente`** (indireto); `docs/cliente/harness-e2e.md` §3 | `oper` |
| `replay.mjs` | Reenvia os frames C→S gravados e compara a resposta do servidor (tamanho; `--exact` compara bytes). Exit 0/1. | `docs/cliente/harness-e2e.md` §4 | `oper` |
| `harness.test.mjs` | Testes da própria ferramenta (`node:test`): INITCODE, frame partido, `Size` fora da faixa, gravação e replay positivo/negativo. | **`make harness-cliente`** | `gate`(manual) |

### 2.7 Fora de `tools/`, mas parte do mesmo conjunto

**`apps/tools/` (10) — a CLI oficial:**

| Arquivo | Papel |
|---|---|
| `bin/wyd-tools.mjs` | CLI com `inventario`, `binarios`, `strings`, `pacotes`, `docs`, `zip-check` |
| `lib/inventario.mjs` | Caminhada determinística, bytes por extensão, SHA-256, resumo PE (usa `tools/client/pe.mjs`) |
| `lib/pe-analise.mjs` | Seções, entrypoint, imports por DLL, exports, PDB, strings, candidatos a função e a global |
| `lib/pacotes.mjs` | `opcode_table.rs` × `Opcode::*` em `wyd-server` × `struct`+`OPCODE` em `payload/*.rs` × capturas em `tmp/` |
| `lib/re-docs.mjs` / `lib/docs.mjs` | Renderização Markdown determinística (sem timestamp, ordenada, com comando de regeneração) |
| `lib/zip.mjs` | Lê diretório central do ZIP, infla e confere manifesto (tamanho + SHA-256) |
| `test/inventario.test.mjs`, `test/re.test.mjs` | Testes da CLI (`node:test`, sem dependências) → **`make check`** → `tools-test` e CI `conteudo` |

**`scripts/` na raiz (2):**

| Arquivo | O que faz | Usado por | Sit. |
|---|---|---|---|
| `client-zip.ps1` | Gera `client.zip` com entradas relativas (sem prefixo `client/`) e sem artefatos mutáveis. | **`make client-zip`** | `ger` |
| `prova_reaper.sh` | Prova do reaper de `character_session`: `SIGKILL` no container, restart com TTL=20 s e observação do SQL. Conta fixa `qavivo01`, personagem `136`, log em `/tmp`. | **nenhum alvo** — só manual | `prova` |

---

## 3. Diagramas de conexão

### 3.1 Quem chama quem (`make`/CI → script → artefato/teste)

```
make check ─────────────────────────────────────────────────────────────┐
├─ fmt / clippy / build / test      → cargo (sem script)                │
├─ migrations-check                 → tools/content/check-migrations.mjs
├─ issue-status-check               → tools/content/check-issue-status.mjs
├─ audit-coverage-check             → tools/content/check-audit-coverage.mjs
├─ docs-check                       → tools/dev/check-docs-consistency.mjs
├─ skills-check                     → tools/dev/check-skills.mjs
├─ versao-check                     → tools/dev/check-versao.mjs
├─ actions-pin-check                → tools/dev/check-actions-pin.mjs
├─ tools-test                       → node --test apps/tools/test/*.test.mjs
└─ portal-test                      → tsc --noEmit + vitest (wyd-portal)

make ci (espelho) ───────────────────────────────────────────────────────┐
├─ test-all                         → cargo test --workspace --all-targets
├─ bench-build + bench-gate         → tools/dev/bench-gate.mjs
├─ audit / deny                     → cargo-audit / cargo-deny
├─ monitoring-rules                 → promtool + tools/dev/check-alertas.mjs
├─ stack-config                     → docker compose config + tools/dev/check-compose-hardening.mjs
└─ portal-build                     → next build

CI (.github/workflows/ci.yml) ───────────────────────────────────────────┐
├─ conteudo (matrix)                → check-migrations, check-issue-status,
│                                      check-docs-consistency, check-skills,
│                                      check-versao, apps/tools/test
├─ infra (matrix)                   → check-compose-hardening, docker compose config,
│                                      promtool ×3, check-alertas, actionlint,
│                                      check-actions-pin
├─ portal                           → npm run typecheck + npm run test
├─ testes                           → nextest --workspace + cargo test --doc
├─ sistema (matrix)                 → cargo test -p <crate> -j 1 --test-threads=1
│                                      (Postgres service, WYD_TEST_REQUIRED=1)
├─ cobertura                        → cargo llvm-cov + vitest --coverage
├─ deploy (self-hosted)             → deploy-provas, reportar-deploy, anotar-deploy-grafana
└─ notify                           → notify-ci
```

### 3.2 Fluxo de dados: cliente → artefato → teste

```
client/ (fora do git)
   │
   ├─ WYD.exe ──► tools/client/pe.mjs ──► apps/tools/bin/wyd-tools.mjs binarios|strings
   │                    │                        └─► docs/cliente/re-binarios.md, re-strings.md
   │                    └─► tools/client/analyse.mjs ──► docs/cliente/gerado/{pe,strings,evidencia}.json
   │
   ├─ payload/*.rs + opcode_table ──► tools/client/captura.mjs --gen
   │                                        └─► crates/wyd-protocol/tests/fixtures/cliente/*.json
   │                                                └─► golden_pacotes (bytes)
   │                                                └─► analyse.mjs → goldens.json (revalida)
   │
   ├─ serverlist.bin + sn.bin ──► tools/launcher/canais.mjs
   │        ▲                            └─► canais.txt
   │        └── tools/client/serverlist.mjs ◄── serverlist-servers.txt
   │                                    └─► golden_serverlist_paridade (Rust × Node)
   │
   ├─ UI/itemicon*.wyt + itemicon.bin ──► tools/client/extrair-icones.mjs
   │                                        └─► wyd-portal/public/game-assets/itens/*
   │
   └─ sessão real ──► tools/cliente-harness/proxy.mjs ──► tmp/cliente/*.jsonl
                                                          ├─► replay.mjs (compara framing)
                                                          ├─► decode-captura (Rust, opcode+campos)
                                                          └─► captura-live/diff-captura/ataque-do-legado/nivel-do-legado
```

### 3.3 Fluxo de conteúdo: legado → migrations → banco → contrato

```
origin/main (legado, lido com `git show`)
   └─► tools/content/gen-seed.mjs ──► crates/wyd-db/migrations/0017..0019_seed_*.sql
                                            │
crates/wyd-db/migrations/*.sql ──► tools/content/banco-parse.mjs
        │                                   ├─► gen-catalogo-banco.mjs ──► docs/arquitetura/banco-catalogo.md
        │                                   │        └─► contrato_catalogo (Rust: doc ↔ SQL ↔ SCHEMA_VERSION)
        │                                   └─► verify-schema.mjs (Docker: SQL ↔ information_schema real)
        │
        └─► migrate-cycle.mjs (Docker: up → down → up)

0017_seed_item_catalog.sql ──► tools/content/gen-enciclopedia.mjs ──► wyd-portal/src/content/itens.json
                                                                       └─► contrato do portal (enciclopédia)
```

### 3.4 Onde `tools/` toca a suíte de testes

| Ferramenta | Alvo de teste que depende dela |
|---|---|
| `tools/client/pe.mjs`, `strings.mjs` | indiretamente `tools-test` (`apps/tools`) |
| `tools/client/captura.mjs` | `golden_pacotes` (`crates/wyd-protocol`) |
| `tools/client/serverlist.mjs` | `golden_serverlist_paridade` (`crates/wyd-launcher`) |
| `tools/content/banco-parse.mjs` | `contrato_catalogo`, `contrato_migrations` (`crates/wyd-db`) |
| `tools/content/gen-seed.mjs` | `golden_gema_seed`, `golden_refino_seed`, `contrato_catalogo` |
| `tools/cliente-harness/lib.mjs` | `harness.test.mjs` (`make harness-cliente`) |
| `apps/tools/*` | `apps/tools/test/*` (`make tools-test`) |

**Nenhum teste Rust importa `tools/`** — e isso é correto (regra de `tools/README.md`).
A ligação é sempre por **artefato versionado** (fixture, seed, doc gerado), não por
código. É o que torna a troca de linguagem das ferramentas irrelevante para o servidor.

---

## 4. A suíte de testes: como o Rust funciona

### 4.1 Teste no mesmo arquivo: convenção ou obrigação?

**Convenção.** O Rust não exige que teste fique junto do código. O que a linguagem
oferece são dois *mecanismos*, e você escolhe:

| Mecanismo | Onde | O que é, tecnicamente |
|---|---|---|
| `#[cfg(test)] mod tests { ... }` | dentro de `src/**` | O módulo só é **compilado** no perfil de teste e entra no binário de teste da própria crate. Por isso alcança itens **privados**. É o padrão do *Rust Book* e do ecossistema. |
| `tests/*.rs` | pasta `tests/` | Cada arquivo é compilado como uma **crate separada** que faz `extern crate` da sua lib. Só vê a **API pública**. |

Ou seja: teste inline não é obrigação, é o idioma. A escolha é guiada por **o que o
teste precisa ver**: se precisa de item privado, inline; se quer provar o contrato
público, `tests/`.

### 4.2 Como o Cargo descobre os testes (o detalhe que explica nossa estrutura)

O Cargo descobre automaticamente **apenas**:

- `tests/*.rs` (arquivo na raiz de `tests/`);
- `tests/<dir>/main.rs`.

Ele **não** descobre `tests/integracao/roundtrip.rs`. Por isso nossa árvore
`tests/<tipo>/<tema>.rs` **exige** um `[[test]]` explícito por arquivo em cada
`Cargo.toml`:

```toml
[[test]]
name = "integracao_roundtrip"
path = "tests/integracao/roundtrip.rs"
```

Isso está documentado em `docs/arquitetura/testes.md` §3.2 e repetido em todos os nove
`Cargo.toml`. **A estrutura `src/` + `tests/<tipo>/` é convenção do nosso projeto**, não
do Cargo — o Cargo só conhece `tests/` plano. `tests/common/mod.rs` é o único arquivo
sem `#[test]` permitido ali, justamente porque subpasta sem `main.rs` não vira alvo.

Consequências práticas dessa escolha:

| Ganho | Custo |
|---|---|
| O nome do alvo carrega o tipo (`sistema_login`, `golden_pacotes`) → `cargo test --test sistema_login` funciona e o tipo fica visível na saída | um `[[test]]` manual por arquivo, fácil de esquecer (arquivo novo sem alvo simplesmente **não roda**) |
| Pasta = tipo, o que permite gate por tipo (`make sistema-test`) | 69 alvos × 9 crates para manter |

> **Risco real:** arquivo criado em `tests/<tipo>/` sem `[[test]]` **não é executado e
> não avisa**. Hoje a contagem bate (69 arquivos de teste = 69 alvos), mas nada impede a
> regressão. É um gate que falta (ver §9, P2).

### 4.3 Os quatro tipos do projeto (norma vigente)

De `docs/arquitetura/testes.md` §2 — o tipo é decidido pelo que o teste **precisa para
rodar** e pelo que ele **prova**:

| Tipo | Prefixo | Onde | Precisa de | Prova | Roda no `cargo test` padrão |
|---|---|---|---|---|---|
| **Unidade** | — | `src/**` `#[cfg(test)]` | só CPU | regra local, borda, erro tipado | sim |
| **Integração** | `integracao_` | `tests/` | só a API pública da crate | colaboração entre módulos, fluxo sem I/O | sim |
| **Contrato** | `contrato_` | `tests/` | arquivos do repo (doc, SQL, TS, assets) | doc/schema/rota não divergiu de outra fonte de verdade | sim |
| **Golden** | `golden_` | `tests/` | fixture/seed versionada | bytes ou valor exato com procedência | sim |
| **Sistema** | `sistema_` | `tests/` | Postgres, socket CPSock, processo, FS | fluxo real ponta a ponta | **opt-in por env** |

Regras de fronteira que o doc fixa (e que respondem metade das dúvidas):

- Unidade **não** abre socket, banco, processo, arquivo do repo, nem dorme.
- Integração **não** aceita DSN nem porta — se falha por falta de infra, está no tipo errado.
- Sistema é **opt-in explícito**: sem a env, imprime `SKIP:` com todas as variáveis
  aceitas; nunca em silêncio, nunca `panic!`, nunca `#[ignore]`.

---

## 5. Números medidos hoje (2026-11-15)

Contagem bruta de `#[test]` por crate (`grep`), comparada com a matriz oficial de
2026-11-11 (`docs/arquitetura/testes.md` §1: 173 arquivos / 731 testes):

| Crate | Unidade (`src`) | `tests/` | Alvos `[[test]]` | `#[ignore]` |
|---|---:|---:|---:|---:|
| `wyd-admin` | 74 | 48 | 7 | 0 |
| `wyd-bots` | 13 | 1 | 1 | 0 |
| `wyd-core` | 103 | 63 | 9 | 0 |
| `wyd-db` | 18 | 5 | 5 | 0 |
| `wyd-launcher` | 123 | 17 | 4 | **1** |
| `wyd-net` | 14 | 7 | 2 | 0 |
| `wyd-protocol` | 46 | 14 | 2 | 0 |
| `wyd-server` | 95 | 107 | 36 | 0 |
| `wyd-sim` | 102 | 19 | 3 | 0 |
| **Total** | **588** | **281** | **69** | **1** |

Portal (`wyd-portal`):

| Tipo | Arquivos | Casos (`grep test(`/`it(`) |
|---|---:|---:|
| `tests/unidade/**` | 19 | 95 |
| `tests/contrato/**` | 8 | 55 |
| `tests/integracao/**` | 3 | 18 |
| `tests/componente/**` | 1 | 11 |
| `tests/e2e/**` (`.spec.ts`) | 6 | 6 |
| **Total Vitest** | **31** | **179** |

Mais `benches/` (3, `harness = false`): `wyd-core/benches/tick.rs`,
`wyd-net/benches/limits.rs`, `wyd-server/benches/tick.rs`.

**Divergências com o doc normativo (doc está desatualizado, não o código):**

1. O doc diz `#[ignore]` = 0 e "proibido"; existe **1** em
   `wyd-launcher/tests/integracao/cliente_zip.rs:263` (`#[ignore = "rede: baixa asset
   público do GitHub Releases"]`). É um caso legítimo (rede), mas a norma e o código
   precisam concordar — hoje a norma está errada.
2. O doc conta 731 testes; hoje há ~869 `#[test]`. A matriz §5 precisa de atualização.
3. O doc cita `tests/componente` como pasta do portal — existe, com 1 arquivo
   (`app/public/conta/pin-form.test.tsx`).

---

## 6. Inventário dos arquivos de teste

### 6.1 `crates/wyd-server` — 36 alvos (o crate mais fragmentado)

| Arquivo | Alvo | Tipo | Infra |
|---|---|---|---|
| `contrato/monitoramento.rs` | `contrato_monitoramento` | contrato | arquivos do repo |
| `integracao/campo_drops.rs` | `integracao_campo_drops` | integração | — |
| `integracao/campo_treinamento.rs` | `integracao_campo_treinamento` | integração | — |
| `integracao/canal_verde.rs` | `integracao_canal_verde` | integração | — (mexe em env) |
| `integracao/config_gameplay.rs` | `integracao_config_gameplay` | integração | — |
| `integracao/config_recarga.rs` | `integracao_config_recarga` | integração | — |
| `integracao/tick_processo.rs` | `integracao_tick_processo` | integração | — |
| `sistema/autenticacao.rs` | `sistema_autenticacao` | sistema | DB + CPSock |
| `sistema/campo_vivo.rs` | `sistema_campo_vivo` | sistema | DB + CPSock |
| `sistema/cena_armia.rs` | `sistema_cena_armia` | sistema | DB + CPSock |
| `sistema/cena_mobs.rs` | `sistema_cena_mobs` | sistema | DB + CPSock |
| `sistema/combate_pacote.rs` | `sistema_combate_pacote` | sistema | DB + CPSock |
| `sistema/combate_vivo.rs` | `sistema_combate_vivo` | sistema | DB + CPSock |
| `sistema/conta_de_teste.rs` | `sistema_conta_de_teste` | sistema | DB |
| `sistema/criacao_canal.rs` | `sistema_criacao_canal` | sistema | DB + CPSock |
| `sistema/entrada_mundo.rs` | `sistema_entrada_mundo` | sistema | DB |
| `sistema/gold_inicial.rs` | `sistema_gold_inicial` | sistema | DB + CPSock |
| `sistema/handshake.rs` | `sistema_handshake` | sistema | CPSock |
| `sistema/limite_ip.rs` | `sistema_limite_ip` | sistema | socket local |
| `sistema/login.rs` | `sistema_login` | sistema | DB + socket |
| `sistema/login_canal.rs` | `sistema_login_canal` | sistema | DB + CPSock |
| `sistema/loja_viva.rs` | `sistema_loja_viva` | sistema | DB + CPSock |
| `sistema/lotacao.rs` | `sistema_lotacao` | sistema | DB (mexe em env) |
| `sistema/nivel_wire.rs` | `sistema_nivel_wire` | sistema | DB + CPSock |
| `sistema/painel_login_canal.rs` | `sistema_painel_login_canal` | sistema | DB + CPSock |
| `sistema/personagens_crud.rs` | `sistema_personagens_crud` | sistema | DB + socket |
| `sistema/pin_canal.rs` | `sistema_pin_canal` | sistema | DB + CPSock |
| `sistema/pin_persistencia.rs` | `sistema_pin_persistencia` | sistema | DB |
| `sistema/pin_reset_ingame.rs` | `sistema_pin_reset_ingame` | sistema | DB |
| `sistema/pin_troca.rs` | `sistema_pin_troca` | sistema | DB |
| `sistema/portal_pin.rs` | `sistema_portal_pin` | sistema | DB |
| `sistema/refino_piedade.rs` | `sistema_refino_piedade` | sistema | DB |
| `sistema/selchar.rs` | `sistema_selchar` | sistema | DB + socket (escreve `tmp/captura`) |
| `sistema/selecao.rs` | `sistema_selecao` | sistema | CPSock + Prometheus |
| `sistema/sessao_unica.rs` | `sistema_sessao_unica` | sistema | 2 canais |
| `sistema/status_udp.rs` | `sistema_status_udp` | sistema | UDP local |
| `tests/common/mod.rs` | — | helper | `dsn_ou_skip`, `preparar_conta_de_teste`, `remover_personagem`, `esperar_ate` |

**16 dos 36 alvos exigem `WYD_LIVE_CPSOCK` (canal rodando).** O CI sobe apenas
Postgres, sem canal — logo, esses 16 pulam lá (ver §7, P0).

### 6.2 Demais crates

| Crate | Arquivos de teste |
|---|---|
| `wyd-core` (9) | `golden/gema_seed`, `integracao/{autoridade, compose, content, determinism, gem, inventario, npc}`, `sistema/telemetry` |
| `wyd-admin` (7) | `contrato/{autorizacao, conteudo, login, operacao, painel, transporte}`, `sistema/seed` |
| `wyd-db` (5) | `contrato/{catalogo, migrations}`, `sistema/{pg, sessoes, stack_caps}` |
| `wyd-launcher` (4) | `contrato/identidade`, `integracao/cliente_zip`, `golden/{serverlist_paridade, assinatura_ed25519}` |
| `wyd-sim` (3) | `golden/{formulas, refino_seed}`, `integracao/determinism` |
| `wyd-protocol` (2) | `golden/pacotes`, `integracao/roundtrip` |
| `wyd-net` (2) | `integracao/{flood, metrics}` |
| `wyd-bots` (1) | `sistema/cena_campo` (CPSock) |

### 6.3 Como as duas suítes se relacionam com o código

```
crate/
├── src/
│   ├── regra.rs        ← regra + `#[cfg(test)] mod tests` no fim do arquivo
│   └── lib.rs          ← API pública usada pelos testes de `tests/`
└── tests/
    ├── common/mod.rs   ← helpers (único arquivo sem #[test])
    ├── unidade?        ← NÃO existe: unidade mora em src/
    ├── integracao/     ← API pública, sem I/O
    ├── contrato/       ← compara com doc/SQL/TS/assets do repo
    ├── golden/         ← bytes/valor exato + fixture
    └── sistema/        ← Postgres/socket/processo, opt-in por env
```

O portal segue a mesma ideia, com nomes em português e sem `src/`:
`tests/{unidade,contrato,integracao,componente}` (Vitest) + `tests/e2e` (Playwright).

---

## 7. Respostas diretas

### 7.1 O que é teste unitário?

Testa **uma regra isolada**, em memória, sem I/O, sem relógio real, sem aleatoriedade
não semeada. No nosso caso: `#[cfg(test)] mod tests` no fim do arquivo da regra, dentro
de `src/`. Prova borda, erro tipado e determinismo — ex.: `wyd-sim` com 102 testes de
fórmula, `wyd-core/src/systems/inventory.rs` com 19.

Não é unitário se: abre socket, banco, processo, lê arquivo do repo ou dorme. Se precisa
disso, é integração ou sistema (norma §2).

### 7.2 O que é teste de integração?

Prova a **colaboração entre módulos** usando só a **API pública** da crate, ainda sem
infraestrutura externa. Ex.: `wyd-core/tests/integracao/compose.rs` (11), `npc.rs` (12),
`autoridade.rs` (10 — pacote forjado é recusado), `wyd-net/tests/integracao/flood.rs` (6).

A regra do projeto é dura e correta: **integração não aceita DSN nem porta**. Se o teste
falha por falta de infra, ele está no tipo errado.

### 7.3 O que é teste e2e (ponta a ponta)?

Prova o **fluxo completo atravessando processos reais**, do usuário/cliente até o banco,
sem mock na fronteira. No repositório isso aparece em **três lugares diferentes**, e
confundir os três é metade da confusão atual:

| O que | Onde | Atravessa | Chamado por |
|---|---|---|---|
| **E2E do portal** (navegador real) | `wyd-portal/tests/e2e/*.spec.ts` + `scripts/e2e.mjs` | Chromium → Next → gateway → Postgres/canal | `npm run test:e2e` (manual) |
| **E2E do servidor** (protocolo real) | `crates/wyd-server/tests/sistema/*` (16 alvos) | socket CPSock → canal → motor → Postgres | `make sistema-test`, CI `sistema` |
| **E2E da janela do launcher** | `tools/launcher/e2e-janela.mjs` | WebDriver → WebView2 → portal falso → `WYDLauncher.exe` | manual |

E existe o **E2E do cliente 7662**, que **não** está pronto (ver 7.8).

### 7.4 Os testes unitários são independentes e paralelos?

**Em geral sim, mas não por garantia — por disciplina.** Fatos:

- `cargo test` roda cada binário de teste em paralelo por threads (default = núcleos).
- Unidade não compartilha estado por construção (sem I/O, sem banco) → paralelismo seguro.
- **Exceção medida:** 6 testes de unidade mutam **variável de ambiente do processo**
  (`std::env::set_var`): `wyd-server/src/boot/env.rs:35`, `channels.rs:296,302,304`,
  `cpsock/limits.rs:142,144`. Como o binário de teste é compartilhado, dois desses rodando
  em paralelo disputam o mesmo `environ`. São variáveis diferentes hoje (baixo risco), mas
  é o tipo de acoplamento que **vira `unsafe` na edição 2024** e que pode gerar flake.
- Não há `serial_test`, mutex nem `--test-threads=1` para unidade.

**Veredito:** paralelismo correto na prática; falta blindagem formal para os testes que
usam env. Ação barata: injetar o env como parâmetro (como já se faz em `env.rs`) em vez
de `set_var`.

### 7.5 Os testes de integração são independentes e usam Docker para isolar?

**Independência: sim.** São puros, cada um monta o próprio mundo em memória, e podem
rodar em paralelo sem interferência — é exatamente a definição do tipo no projeto.

**Docker: não, e por design.** Os testes de integração **não podem** precisar de algo
externo, então Docker ali seria sinal de que o teste está no tipo errado. Docker aparece
em outro lugar, e de três formas:

| Onde | Como isola |
|---|---|
| CI `sistema` | `services: postgres:17-alpine` (efêmero, senha derivada do `run_id`) |
| `tools/content/migrate-cycle.mjs` e `verify-schema.mjs` | container `postgres:17-alpine` descartável (`wyd-migcheck`) |
| `wyd-portal/scripts/e2e.mjs` | banco **dedicado** `wyd_e2e` + containers `wyd-gateway-e2e`/`wyd-server-e2e`/`wyd-portal-e2e` na rede `wyd` |

**Não há `testcontainers`** (nem em Rust nem em Node): o isolamento é artesanal, via
`docker run`/`docker compose` chamado por script. Funciona, mas é o ponto mais frágil da
suíte — ver 7.7.

**Onde a independência quebra: sistema.** Testes de sistema compartilham **um** Postgres,
**um** canal e **contas fixas** (`qavivo01`, `cenamob1`, personagem `136`). Por isso:

- o CI roda com `-j 1 --test-threads=1`;
- `make sistema-test` repete o `-j 1`;
- **`make test` (`cargo test --workspace`) NÃO serializa**. Se alguém exportar
  `WYD_TEST_DSN` e rodar `make test`, os 36 binários de `wyd-server` sobem em paralelo
  contra as mesmas contas → colisão. O caminho seguro é `make sistema-test`.

### 7.6 O E2E está organizado para testar o fluxo completo?

**O do portal: sim, tecnicamente bem montado.** 6 specs, um fluxo por arquivo:

| Spec | Fluxo |
|---|---|
| `login.spec.ts` | Staff sem sessão cai no login e entra na área administrativa |
| `painel.spec.ts` | Staff entra no painel e consulta uma conta |
| `conta.spec.ts` | Login abre a área do jogador; logout encerra a sessão |
| `entregas.spec.ts` | Concessão pela tela credita o ouro do personagem |
| `financeiro.spec.ts` | Vitrine cadastra item; crédito repetido devolve replay |
| `idempotencia.spec.ts` | Repetir a mesma chave não duplica entrega nem ouro |

O `scripts/e2e.mjs` faz o que um bom E2E precisa: banco `wyd_e2e` **recriado do zero** a
cada run (nunca toca `wyd`), migra pela série real, sobe gateway/canal/portal de imagens
já construídas, derruba tudo no `finally` e **propaga o exit code** do Playwright.
`support.ts` centraliza login, acesso ao gateway e parsing de `testid`.

**O buraco:** nada disso roda em gate nenhum. Não há alvo `make` para E2E (o
`grep` no `Makefile` por `e2e|playwright` volta vazio), o job `portal` do CI roda só
`typecheck` + `vitest`, e o job `portal-build` só faz `next build`. O próprio
`tmp/rebaixa-status.mjs:39` já registrava: *"as 6 specs Playwright existem e passaram em
2026-11-12, mas nenhum job do CI executa Playwright"*. **E2E que não roda não protege
nada** — hoje ele só prova quando alguém lembra de rodar à mão.

### 7.7 Playwright está bem usado?

**O que está certo:**

- `testDir`, `baseURL` (127.0.0.1:3110) e `workers: 1` fixados com justificativa escrita
  no config (as specs compartilham gateway/banco);
- `trace: "retain-on-failure"` — diagnóstico sem custo no caminho feliz;
- `fullyParallel: false` coerente com o recurso compartilhado;
- um helper (`support.ts`) em vez de copiar login em cada spec;
- fluxos reais, sem mock de rede: navegador → Next → gateway → Postgres/canal;
- nomes em português descrevendo estado esperado, como a norma pede.

**O que falta para "confiável, reproduzível e fácil de manter":**

| Lacuna | Efeito |
|---|---|
| Nenhum job de CI | regressão passa despercebida; foi assim que a suíte ficou 3 dias parada |
| Nenhum alvo `make` | não entra no hábito do dev nem no DoD |
| Só Chromium (sem `projects`) | WebKit/Firefox nunca testados; o portal é público |
| `retries` não configurado (default 0) + `workers: 1` | flake aparece como vermelho e ninguém mede a taxa |
| `reporter` default | sem JUnit/HTML para acompanhar histórico |
| Sem `webServer`/health-gate no config | a espera dos 4 serviços está no `e2e.mjs`; se ele mudar, o config não sabe |
| Sem `storageState` | cada spec refaz login pela UI (ok para 6, caro se crescer) |
| Sem screenshot/diff visual | nenhum teste prova que a tela **desenha** certo |

**Sobre "todos os E2E no navegador, com fluxos reais em navegadores reais":** os 6 do
portal sim (Chromium real, sem mock). O E2E da **janela** do launcher também é real
(`tauri-driver` + `msedgedriver` + WebView2). O E2E do **cliente 7662** não existe (7.8).

### 7.8 Temos harness headless real do cliente 7662?

**Não.** O que existe, e o que cada coisa é:

| Camada | Ferramenta | O que prova | Limite |
|---|---|---|---|
| Protocolo (headless, sem GUI) | `crates/wyd-bots` (cliente CPSock real) + 16 alvos `sistema_*` | login, seleção, cena, combate, loja, gold no **fio** | não abre o `.exe`; não prova render, clique nem layout |
| Captura + replay | `tools/cliente-harness` | o servidor responde à sequência **real** do cliente (framing) | precisa de uma sessão gravada; compara tamanho, não semântica |
| Captura visual + OCR | `captura-sessao.ps1`, `captura-tela.ps1`, `ocr-texto.ps1`, `teclado-pin.ps1` | o que o cliente **desenhou** (print) e o texto do `strdef.bin` | manual, Windows, elevação, sem asserção automática de fluxo |
| Janela do launcher | `tools/launcher/e2e-janela.mjs` | clique real em WebView2, jogo abre uma vez | é o **launcher**, não o 7662 |

E o motivo está documentado com medição em `docs/cliente/harness-e2e.md` §5:

- o 7662 é protegido (`JProtectDll`); `taskkill` sem elevação dá acesso negado;
- `SendInput`, `mouse_event`, `WM_LBUTTONDOWN` e `SendKeys` elevados **já falharam** —
  o cliente usa DirectInput e descarta input sintético sem foco real;
- em 25 s de execução ele não abriu conexão CPSock.

**Resposta honesta:** não temos uma ferramenta robusta que abra o cliente real, execute
comandos, tire prints, colete logs e valide comportamento em cenários. Temos as peças
(proxy, OCR, captura de tela, bots de protocolo) e o caminho declarado — VM com controle
real / scan code, ou proxy que injeta pacotes — mas **nenhuma delas está pronta e nenhuma
é automática**. Hoje a prova visual do cliente continua sendo manual, feita pelo dono
(QA-12/QA-13/CLIENT-12).

### 7.9 Reaper: é obsoleto? Pode remover?

**Não. O reaper está vivo e é necessário.** Evidência no código:

`crates/wyd-server/src/boot/mod.rs:424-451` — no boot do canal, uma thread chama
`wyd_db::Sessions::release_stale_sessions` **no boot e a cada 15 s**, com TTL
`WYD_SERVER_SESSION_STALE_SECS` (default 180 s), logando `reaper result=released`. É ele
que libera a linha de `character_session` quando o cliente morre sem FIN/leave — sem
isso, `/health` conta sessão fantasma e a vaga do jogador fica presa.

O que existe, então, são **dois artefatos de prova**, não a feature:

| Artefato | O que é | Situação |
|---|---|---|
| `docker-compose.reaper.yml` | overlay de 1 linha: `WYD_SERVER_SESSION_STALE_SECS: "20"` para o canal 1 | não é produção; **não tem cabeçalho nem entrada na doc** — o próprio `docs/rust/10-devops-ci.md:231` já registra isso como dívida **C13/DEVOPS-03** |
| `scripts/prova_reaper.sh` | prova manual: `SIGKILL` no container, restart, SQL antes/depois. Conta `qavivo01`, personagem `136`, log em `/tmp/prova_reaper.log` | **nenhum alvo** do `Makefile` ou do CI o chama; só a doc o cita |

**O que já é automático:** a lógica em si tem teste — `crates/wyd-db/tests/sistema/sessoes.rs`
exercita `release_stale_sessions` contra Postgres real. O que **não** tem teste automático
é o *wiring* do boot (a thread de 15 s dentro do processo do canal).

**Recomendação, na ordem:**

1. **Não remover agora.** Sem o script, o único artefato que prova o reaper de ponta a
   ponta no compose some, e o wiring do boot fica sem prova nenhuma.
2. **Promover** a prova para a `wyd-tools` (`wyd-tools ops reaper-prova`) ou, melhor,
   transformar em teste `sistema_reaper` com `WYD_TEST_DSN` + docker kill — aí ela entra
   no fluxo normal e não depende de `/tmp`.
3. **Só então** apagar `docker-compose.reaper.yml` + `scripts/prova_reaper.sh` e remover
   C13 da dívida. Se a pressa for maior, o mínimo é dar cabeçalho ao overlay e citá-lo na
   doc de operação, como o próprio DEVOPS-03 pede.

**Correção de premissa:** eles não estão "na pasta `tools/scripts`" — o overlay está na
**raiz** e o script em `scripts/` (raiz). A limpeza, quando acontecer, é nesses dois
lugares, não em `tools/`.

---

## 8. Achados priorizados

### P0 — falso verde (o gate diz "ok" sem ter provado)

1. **CI `sistema` não tem canal.** O job sobe só Postgres; **16 dos 36 alvos de
   `wyd-server`** (e 1 de `wyd-bots`) exigem `WYD_LIVE_CPSOCK` e pulam com `SKIP:` +
   exit 0. `WYD_TEST_REQUIRED=1` só cobre DSN, **não** cobre o canal. Metade do E2E de
   servidor não roda no CI e o run fica verde.
   *Correção:* subir `wyd/server:ci` no job `sistema` e tratar `WYD_LIVE_CPSOCK` ausente
   como falha quando `WYD_TEST_REQUIRED=1`.
2. **Playwright fora de todo gate.** 6 specs, nenhum job, nenhum alvo `make` (7.6).

### P1 — estrutura e tipo

3. **`make test` pode colidir.** `cargo test --workspace` roda os 36 binários de
   `wyd-server` em paralelo; com DSN exportado eles disputam contas fixas. O caminho
   seguro (`-j 1`) só existe em `make sistema-test`. Alinhar os dois (ou marcar os testes
   de conta fixa como serializados).
4. **`set_var` em teste de unidade** (6 ocorrências) — acoplamento de processo, futuro
   `unsafe` na edição 2024.
5. **Arquivo em `tests/<tipo>/` sem `[[test]]` não roda e não avisa.** Falta um gate que
   compare "arquivos de teste" × "alvos declarados" (hoje: 69 × 69, por sorte).
6. **`tools/`: artefatos de prova morando como se fossem ferramenta**
   (`ataque-do-legado.mjs`, `nivel-do-legado.mjs`, `trade-layout.mjs`, `prova_reaper.sh`).
   Ou viram comando, ou viram fixture de teste, ou saem.

### P2 — higiene e doc

7. **`docs/arquitetura/testes.md` está desatualizado:** 731 → ~869 testes; `#[ignore]` 0 → 1
   (e a norma o proíbe, com um caso legítimo existente).
8. **`docs/arquitetura/visao-pastas.md` §1 subestima `tools/`:** fala de "15 scripts" em
   `tools/client` (são 27 arquivos) e de `tools/dev` com 3 scripts (são 17).
9. **`tools/dev/README.md` lista 5 scripts** dos 17 existentes — os de CI
   (`notify-*`, `deploy-*`, `check-compose-hardening`, `check-actions-pin`) não estão lá.
10. **Escrita em `tmp/` do repo por teste de sistema** (`combate_pacote`, `login_canal`,
    `pin_canal`, `nivel_wire`, `selchar`, `criacao_canal`, `painel_login_canal`) — a norma
    §4.3 manda usar diretório temporário; a auditoria de 2026-11-11 já marcou `selchar`.
11. **Duas ferramentas para o teclado de PIN** (`ocr-teclado.ps1` × `teclado-pin.ps1`).

---

## 9. Duplicidades e redundâncias — base para enxugar

Critério: **duplicação** = a mesma responsabilidade existindo em dois lugares, de forma que
uma mudança em uma ponta pode deixar a outra mentindo. Duplicação **deliberada** (oráculo
independente, separação por segurança) não entra na lista de corte — está na §9.2, e
apagá-la seria piorar.

### 9.1 Duplicações reais, com evidência

#### Classe A — dois escritores para o mesmo formato binário (as mais caras)

| # | Duplicação | Evidência | Risco | Ação |
|---|---|---|---|---|
| **D1** | `serverlist.bin` escrito em Node **e** em Rust | `tools/client/serverlist.mjs` (`build`) × `crates/wyd-launcher/src/serverlist.rs`; `cliente_zip.rs:446` regenera a cada aplicação | o `golden_serverlist_paridade` existe **só** para vigiar isso; divergência fica invisível até alguém rodar o golden | Rust vira o escritor único; Node mantém só o leitor de `--check`; apagar `build` do Node na Fase 1 |
| **D2** | `manifest.txt` escrito em Node e lido em Rust | `tools/launcher/dist-manifest.mjs` × `crates/wyd-launcher/src/manifest.rs` | contrato textual sem round-trip no mesmo código: formato novo passa no writer e quebra no reader | gerar pelo Rust (reusa `manifest.rs`); Node fica com `--check` até a migração |
| **D13** | Duas pipelines de análise PE com saídas sobrepostas | `tools/client/analyse.mjs` (`docs/cliente/gerado/pe.json`, `strings.json`) × `apps/tools` (`docs/cliente/re-binarios.md`, `re-strings.md`) | dois conjuntos de artefato para o mesmo binário; `analyse.mjs` não é chamado por `make` nem CI → `gerado/*.json` apodrece sem avisar | decidir o pipeline canônico (o `tools/README.md` já diz que `re-*.md` é do `wyd-tools`) e reduzir `analyse.mjs` a evidência/âncoras |

#### Classe B — gates redundantes (o `make check` roda a mesma coisa duas vezes)

| # | Duplicação | Evidência | Risco | Ação |
|---|---|---|---|---|
| **D3** | Validação estática da série de migrations em Node **e** em Rust | `tools/content/check-migrations.mjs` × `crates/wyd-db/tests/contrato/migrations.rs` — o cabeçalho do Rust promete exatamente o mesmo (up/down, reversibilidade, FK para tabela criada, seed em tabela existente, tabela criada duas vezes) | `make check` roda `test` **e** `migrations-check`: dois gates para um contrato, mantidos em paralelo; um pode passar onde o outro falha | rodar os dois, comparar o conjunto de falhas e remover o Node do `make check` |
| **D4** | `--check` do catálogo × contrato Rust | `gen-catalogo-banco.mjs --check` **não é chamado por ninguém** (`Makefile:88` chama sem a flag; CI não chama) e `crates/wyd-db/tests/contrato/catalogo.rs` já prende doc ↔ migrations ↔ `SCHEMA_VERSION` | modo morto que aparenta ser gate; quem confiar nele não é avisado de nada | remover o modo `--check` (o gerador continua) |
| **D6** | Três parsers da mesma série SQL | `tools/content/banco-parse.mjs`, parser **próprio** dentro de `check-migrations.mjs` (importa só `fs`/`path`), `catalogo.rs` + `migrations.rs` | três leitores do mesmo formato: um aceita o que o outro recusa | consequência de D3: ao remover o gate Node, sobra um parser Node (`banco-parse`) e um Rust |

#### Classe C — ferramentas duplicadas ou órfãs

| # | Duplicação | Evidência | Risco | Ação |
|---|---|---|---|---|
| **D5** | Dois proxies CPSock | `tools/client/captura-live.mjs` modo `--alvo` (linhas 154-237) × `tools/cliente-harness/proxy.mjs` | o do harness tem `FrameSplitter`, teste próprio (`harness.test.mjs`) e replay; o do `captura-live` é uma segunda implementação, sem teste, com outro formato de saída | `captura-live` passa a delegar ao `lib.mjs` (ou perde o modo proxy e mantém só `--from`) |
| **D7** | Duas capturas de tela | `captura-print.ps1` (`PrintWindow`/`BitBlt`) × `captura-tela.ps1` (grab periódico) | o próprio `tools/client/README.md` registra que `PrintWindow` de D3D9 **volta preto** — `captura-print` está superado e não é chamado por nada além do README | **remover agora** |
| **D8** | Dois resolvedores do teclado de PIN | `ocr-teclado.ps1` × `teclado-pin.ps1` | o cabeçalho de `teclado-pin.ps1` diz que o `Windows.Media.Ocr` **não lê** esses glifos (por isso usa máscara + Hamming); `ocr-teclado.ps1` não é referenciado por nenhum script | **remover agora** (manter `teclado-pin.ps1`) |
| **D9** | POST no Slack repetido | `notify-ci.mjs:144-147` × `notify-release.mjs:49-52` (`text`, `unfurl_links`) | ~8 linhas duplicadas; baixo risco, mas toda mudança de payload precisa ser feita duas vezes | helper `slack.mjs` quando mexer em notificação |

#### Classe D — infra e documentação

| # | Duplicação | Evidência | Risco | Ação |
|---|---|---|---|---|
| **D10** | Provas do deploy em dois lugares | `Makefile:205-211` (`local-rebuild`) repete as provas 1-3 do `deploy-provas.mjs` **inline** e **não tem** a prova 4 (`/api/health`) | a cópia do Makefile é a antiga: deploy pode passar por lá e falhar no CI, ou vice-versa | `local-rebuild` chama `node tools/dev/deploy-provas.mjs` — dedup **e** corrige a lacuna |
| **D11** | Cinco inventários da mesma pasta | `tools/README.md`, `tools/client/README.md`, `tools/content/README.md`, `tools/dev/README.md`, `docs/arquitetura/visao-pastas.md` §1 | todos divergem do real: `visao-pastas` diz "15 scripts" em `tools/client` (são 27) e 3 em `tools/dev` (são 17); `dev/README` lista 5 de 17 | um inventário canônico (gerado), e as tabelas duplicadas saem |
| **D12** | Este documento é o sexto | `analise-tools.md` §2 | vira mais uma cópia a envelhecer | tratar como **transitório**: é insumo da decisão, não fonte; apagar/absorver no fim do trabalho |

### 9.2 O que NÃO é duplicação — não apagar

| Item | Por que parece duplicado | Por que fica |
|---|---|---|
| `ataque-do-legado.mjs`, `nivel-do-legado.mjs`, `trade-layout.mjs` | reimplementam leitura de pacote que o Rust já faz | são **oráculo independente**: se nascerem do mesmo código, não provam nada. É a única checagem que pega erro de porte |
| `tools/client/keytable.mjs` × `crates/wyd-protocol/src/codec/keyword.rs` | mesma transformação CPSock em duas linguagens | `captura.mjs` gera fixture **sem** usar o crate que está sendo testado — dependência circular seria o fim da prova |
| `sign-package.mjs` (assina) × `distribuicao.rs` (verifica) | mesmo assunto | separação por segurança: a chave privada nunca entra no binário (ADR-0003) |
| `crates/wyd-launcher/src/sha256.rs` | o workspace já tem `sha2` (via `wyd-db`) | reimplementação **deliberada** em `std` para o launcher não puxar dependência; outro crate, outro binário |
| `verify-schema.mjs` × `contrato_catalogo.rs` | ambos comparam catálogo | camadas diferentes: um compara doc ↔ SQL, o outro SQL ↔ `information_schema` real |
| `check-alertas.mjs` × `monitoring-smoke.mjs` | ambos sobre monitoramento | config renderizada × stack viva |
| `crates/wyd-bots` usado pelo launcher | código de um crate em outro | **reuso correto** (mesmo renderizador, sem texto duplicado) |

### 9.3 Ordem de corte (ganho × risco)

| Onda | Itens | Ganho | Risco | Pré-requisito |
|---|---|---|---|---|
| **0 — hoje** | D7, D8 (arquivos órfãos), D10 (Makefile → script), D4 (flag morta) | −3 arquivos, −1 cópia de prova, −1 gate fantasma | **nenhum**: nada referencia esses itens | revisar o diff de cada um |
| **1 — gates** | D3 (+D6) | −1 gate duplicado no `make check`, −1 parser SQL | médio: precisa comparar conjuntos de falha antes | rodar os dois no mesmo commit e diffar |
| **2 — formato** | D1, D2 | −1 escritor binário, −1 contrato textual frágil | alto: mexe em arquivo que o cliente consome | escritores em Rust prontos (`wyd-launcher`) e golden de round-trip |
| **3 — captura** | D5, D13 | −1 proxy, −1 pipeline de doc | médio: `analyse.mjs` gerou artefato versionado | decidir o pipeline canônico antes |
| **4 — doc/CI** | D11, D12, D9 | fonte única de inventário | baixo | gerar o inventário a partir do `--help` |

**Saldo se as ondas 0-2 forem feitas:** 2 arquivos removidos (71 → 69), 2 gates duplicados
eliminados (`migrations-check` e o `--check` do catálogo), 1 parser SQL e 1 escritor binário a
menos. Nenhuma capacidade é perdida — o que encolhe é a **redundância**, não a cobertura.

### 9.4 Classificação por destino (para a decisão da `wyd-tools`)

| Destino | Arquivos | Por quê |
|---|---|---|
| **Já é `wyd-tools`** (só consolidar o vocabulário) | `apps/tools/*` (10) | são a CLI oficial desde ADR-0028 |
| **Promover a comando** (útil, hoje disperso) | gates: `check-*`, `bench-gate`, `deploy-provas`, `monitoring-smoke`, `alerts-prova` · geradores: `gen-*`, `dist-manifest`, `canais`, `extrair-icones`, `extract-tables`, `map-skills`, `captura` · operação: `migrate-cycle`, `verify-schema`, `limpar-qa`, `clean-tmp` · captura: `proxy`, `replay`, `captura-live`, `diff-captura` · cliente/launcher: `strdef`, `serverlist`, `sign-package`, `sign-config`, `e2e-janela` · CI: `notify-*`, `reportar-deploy`, `anotar-deploy-grafana` | têm propósito declarado e uso recorrente |
| **Manter como biblioteca** (não expor CLI) | `pe.mjs`, `strings.mjs`, `keytable.mjs`, `formats.mjs`, `wyt.mjs`, `serverlist.mjs`, `evidence.mjs`, `lib.mjs`, `banco-parse.mjs`, `banco-fontes.mjs`, `pg.mjs` | são importados por quem tem CLI |
| **Manter como serviço** | `tools/dev/alert-sink.mjs` | é container do compose, não comando |
| **Fundir / remover** | `ocr-teclado.ps1` e `captura-print.ps1` (superados, §9.1 D7/D8); `scripts/prova_reaper.sh` + `docker-compose.reaper.yml` (virar `sistema_reaper` ou subcomando antes de apagar); `gen-protocol.mjs` (congelado — mover para `docs/` ou marcar `arquivado/`) | um é abordagem vencida, os outros são prova pontual |

### 9.5 Vocabulário proposto (sem mover arquivo nesta entrega)

```
wyd-tools
├── arte    inventario | binarios | strings | pacotes | docs | zip-check   (existe)
├── cliente proxy | replay | captura | diff | strdef | wyt | serverlist   (absorve tools/client)
├── conteudo gen-seed | catalogo | enciclopedia | linhagem | migrate | schema | limpar-qa
├── gate    migrations | issue-status | audit-coverage | docs | skills | versao
│           | actions-pin | compose | alertas | bench | deploy-provas
├── ops     monitoring-smoke | alerts-prova | clean-tmp | notify-ci | notify-release
└── launcher manifest | canais | sign | conf | e2e
```

Ganhos concretos: um só ponto de descoberta (`--help`), um só lugar para achar o que roda
no gate, e a possibilidade de rodar o mesmo comando em CI e na máquina do dev sem
diferença de caminho.

**Cuidado obrigatório:** os gates são chamados pelo `Makefile` **e** pelo CI. Movê-los
exige atualizar os dois no mesmo commit, senão o gate passa a validar nada (o pior tipo de
falha). Sugestão de ordem: (1) criar o roteador chamando os scripts atuais por caminho;
(2) migrar um gate por vez com o `Makefile` apontando para o novo comando; (3) só no fim
mover os arquivos.

---

## 10. Como reproduzir esta análise

```bash
# inventário da pasta
find tools -type f | sort | wc -l                 # 71

# alvos de teste por crate (69 no total)
grep -c '\[\[test\]\]' crates/*/Cargo.toml

# arquivos de teste nas subpastas (69 + 4 common)
find crates -path '*/tests/*' -name '*.rs' | wc -l

# contagem de testes
grep -rho '#\[test\]' crates/*/src | wc -l        # 588
grep -rho '#\[test\]' crates/*/tests | wc -l      # 281
grep -rn '#\[ignore\]' crates                     # 1 (wyd-launcher)

# quem depende de canal vivo (16 alvos em wyd-server + 1 em wyd-bots)
grep -rl WYD_LIVE_CPSOCK crates/*/tests

# quem roda E2E do portal
grep -n 'e2e\|playwright' Makefile                # vazio
grep -n 'playwright' .github/workflows/*.yml      # vazio

# reaper vivo no boot
grep -n 'reaper' crates/wyd-server/src/boot/mod.rs

# duplicidades da §9.1
ls crates/wyd-launcher/src/*.rs                    # serverlist, snbin, manifest, patch: escritores no Rust
grep -n 'CREATE TABLE' tools/content/check-migrations.mjs | head -1   # parser proprio (D6)
grep -rn 'gen-catalogo-banco' Makefile .github/workflows/*.yml         # sem --check (D4)
grep -n 'net.connect\|splitter' tools/client/captura-live.mjs         # segundo proxy (D5)
grep -rn 'ocr-teclado\|captura-print' tools scripts docs .github | grep -v README   # vazio (D7/D8)
```

---

## 11. Resumo em uma página

- **Não existe `tools/scripts`.** Existem `tools/` (biblioteca, 71 arquivos),
  `apps/tools/` (a CLI `wyd-tools`, 10) e `scripts/` na raiz (2).
- `tools/` mistura **biblioteca, gate, gerador, ferramenta de operador e prova pontual**
  — é isso que torna a pasta ilegível, não o volume.
- **Teste no mesmo arquivo é convenção do Rust, não obrigação.** O que é nosso é
  `tests/<tipo>/<tema>.rs` + `[[test]]` explícito, porque o Cargo só descobre `tests/`
  plano.
- **Unidade:** 588 testes, paralelos e isolados na prática; 6 usam `set_var` (risco).
- **Integração:** pura e paralela; Docker ali seria erro de tipo, e não é usado.
- **Sistema:** 69 alvos no total, contas fixas e canal compartilhado → serializado por
  obrigação (`-j 1`), não por escolha.
- **E2E do portal:** bem montado (6 fluxos, banco efêmero, Chromium real) e **fora de
  todo gate** — não roda no CI nem no `make`.
- **E2E do cliente 7662:** não existe. Há protocolo headless (`wyd-bots`), captura/replay
  e OCR manual; input sintético é descartado pelo DirectInput do cliente.
- **Reaper:** vivo e necessário (`boot/mod.rs`). O overlay e o `prova_reaper.sh` são
  prova manual — promover antes de apagar, nunca apagar a feature.
- **Duplicidades:** 13 reais (§9.1). As caras são dois escritores de formato
  (`serverlist.bin`, `manifest.txt` em Node **e** Rust), dois gates para a mesma série de
  migrations e dois proxies CPSock. Quatro saem hoje sem risco (D7, D8, D4, D10); o resto
  tem ordem obrigatória — escritor único primeiro, corte depois.
- **Maior risco atual:** o CI fica verde com 16 alvos de servidor pulando por falta de
  canal, e com o E2E do portal nunca executado.

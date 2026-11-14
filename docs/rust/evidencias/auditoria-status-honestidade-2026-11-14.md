# Auditoria de honestidade dos status — 2026-11-14

> Regra aplicada: uma issue só fica ✅ se existir **teste que o runner executa** e que **falha se o
> comportamento citado quebrar**. Sem isso o status vira 🟢 ("resolvida na prática, sem prova
> automatizável"), com o motivo escrito na própria linha do painel. Evidência não é opinião: cada
> linha abaixo foi conferida no código, e uma amostra foi **mutada** (quebrada de propósito) para
> provar que o teste fica vermelho e volta a verde ao restaurar.

## Escopo e método

- Fonte dos status: `docs/rust/03-progresso.md` — 50 IDs distintos com ✅ (51 linhas; `CLIENT-07`
  aparecia duplicado e a linha redundante foi removida nesta passada).
- Harness considerado:
  - `cargo test --workspace --all-targets` (todos os binários de teste do workspace);
  - job `sistema` do CI (`.github/workflows/ci.yml`), que roda com Postgres 17, `-j 1`,
    `WYD_TEST_REQUIRED=1` e `WYD_TEST_DSN` — por isso teste atrás de `WYD_TEST_DSN` **conta**
    como executado pelo runner;
  - jobs `conteudo` e `infra` (scripts `node tools/...`, `promtool test rules`, `actionlint`) e
    `make check`.
- Para cada ID: teste citado → existe no código → é coletado (sem `#[ignore]`, sem env não
  definida, sem skip silencioso) → assere o comportamento, não só percorre o caminho.
- "Assere" foi aceito quando há `assert*` comparando o valor que a issue promete (byte, contagem,
  evento, status HTTP, linha de banco) e recusado quando o teste só verifica "não deu panic".

## Veredito em números

| Situação | Quantidade |
|---|---|
| Mantidas ✅ com teste executado e asserção | **30** |
| Rebaixadas para 🟢 (documental/operacional, sem teste executável) | **20** |
| Total auditado | **50** |

As 20 rebaixadas continuam "feitas" na prática — o que não existe é prova automatizável. Cada uma
tem a nota do motivo no painel e volta a ✅ quando houver teste que o runner execute.

## Inventário por issue

### Mantidas ✅ (30)

| ID | Prova no código | Executa | Assere |
|---|---|---|---|
| `CLIENT-01` | `crates/wyd-protocol/src/{wire,codec,payload}` + `tests/golden/pacotes.rs` + `tests/integracao/roundtrip.rs` | `cargo test -p wyd-protocol` | layout do HEADER, ofuscação, opcode/tamanho e recusa de frame adulterado |
| `CLIENT-02` | 9 goldens em `crates/wyd-protocol/tests/golden/pacotes.rs` + `tests/sistema/selecao.rs` | workspace + CI `sistema` | ida-volta byte-idêntica, checksum truncado recusado, seleção com autoridade no Postgres |
| `CLIENT-03` | `crates/wyd-launcher/src/{distribuicao,bootstrap,cliente_zip}.rs` + `tests/integracao/cliente_zip.rs` | `cargo test -p wyd-launcher` | aplica+backup+`check` verde e pacote adulterado recusado (1 teste de rede é `#[ignore]`, ver achados) |
| `CLIENT-07` | `crates/wyd-launcher/src/{manifesto,patch,bootstrap,sha256}.rs` | `cargo test -p wyd-launcher` | manifesto/patch/bootstrap/hash recusam conteúdo divergente |
| `CLIENT-09` | `crates/wyd-server/tests/sistema/selchar.rs:413` `logout_explicito_fecha_a_conexao_e_libera_a_sessao` | CI `sistema` | `CNFCharacterLogout` + EOF + sessão liberada em ≤ 5 s |
| `DEVOPS-02` | `docker/monitoring/prometheus/alerts.test.yml` | CI `infra` (`promtool test rules`) | disparo **e** resolução das regras |
| `DEVOPS-03` | `tools/dev/check-compose-hardening.mjs` | CI `infra` | `TAG`, limites, `no-new-privileges`, loopback e teto de log por texto no compose |
| `DEVOPS-06` | job `imagens` (`.github/workflows/ci.yml`) | CI | build + smoke (`event=fatal`) + Trivy `CRITICAL` bloqueante + SBOM por imagem |
| `DEVOPS-07` | job `sistema` | CI | `WYD_TEST_REQUIRED=1` reprova em vez de pular sem DSN |
| `DEVOPS-08` | job `auditoria` + `cobertura` | CI | `cargo audit`/`cargo deny` como gate; cobertura é métrica publicada (sem limiar) |
| `DEVOPS-10` | `tools/dev/check-versao.mjs` + `release.yml` | CI `conteudo` + release | versão única workspace×portal e artefatos/checksums da release |
| `DEVOPS-12` | `tools/dev/check-actions-pin.mjs` | CI `infra` | SHA de 40 hex, comentário com a referência e resolução tag→commit na API |
| `GAME-01` | `crates/wyd-core/src/tick/scheduler.rs` (`fixed_interval_accumulates_remainder`, `config_rejects_out_of_contract_hz`) | workspace | acumulador preserva o resto e Hz fora do contrato é recusado |
| `GAME-02` | `crates/wyd-core/tests/integracao/gem.rs` + `tests/golden/gema_seed.rs` | workspace | Anct +9 → 40 de perfuração, comum recusado, gema consumida; paridade com o seed |
| `GAME-03` | `crates/wyd-core/src/spatial/grid.rs` (`borders_insert_query_and_reject_outside`, `relocate_moves_between_cells`) | workspace | inserção/consulta por célula, recusa fora do mundo e relocação |
| `GAME-06` | `crates/wyd-core/tests/integracao/npc.rs` (12) + `crates/wyd-sim/src/formula/threat.rs` | workspace | cadência, aggro, fuga/leash, recusa atômica, golden determinístico; ameaça mutada (mutação 2) |
| `GAME-07` | `crates/wyd-server/tests/integracao/tick_processo.rs` (4) | workspace | tick contínuo, métricas no scrape, sem tick órfão, Hz inválido recusado |
| `GAME-12` | `crates/wyd-core/src/systems/inventory.rs` (pilha) + `crates/wyd-db/tests/sistema/stack_caps.rs` | workspace + CI `sistema` | completa 100→120 e usa slot livre; teto real vem do banco (mutação 4) |
| `GAME-15` | `crates/wyd-db/tests/sistema/stack_caps.rs` | CI `sistema` | `406` consumível, nome corrigido e teto por classe conferidos no banco |
| `GAME-16` | `crates/wyd-server/tests/sistema/refino_piedade.rs` + `crates/wyd-sim/src/formula/refine.rs` | CI `sistema` + workspace | piedade acumula/queda em 3 de 4 e zera no sucesso (mutação 1) |
| `PROC-13` | `tools/dev/check-docs-consistency.mjs` (+ `check-issue-status.mjs`) | CI `conteudo` | links relativos, contagem de crates e status índice×painel |
| `PROC-15` | `tools/content/check-migrations.mjs` + `crates/wyd-db/tests/contrato/catalogo.rs` | CI `conteudo` + workspace | ordem/FK/`down`, e catálogo doc↔SQL |
| `PROC-16` | `crates/wyd-db/tests/sistema/pg.rs` (6) | CI `sistema` | pool, adapters e fila de save contra Postgres real |
| `QA-02` | `ci.yml` (jobs `fmt`/`testes`/`auditoria`/`bench`) + `Makefile ci` | CI | fmt, clippy `-D warnings`, build, nextest, doc-tests, audit/deny |
| `QA-03` | job `imagens` + `release.yml` | CI | `[profile.release]` compila, imagem distroless sobe e smoke sai 0/2 conforme contrato |
| `QA-04` | `crates/wyd-core/tests/sistema/telemetry.rs` | workspace | linhas `DEBUG`/`INFO`, `/metrics` com `wyd_ticks_total`/histograma/`wyd_entities_active` |
| `QA-05` | `crates/wyd-net/tests/integracao/flood.rs` + `crates/wyd-net/src/limits/rate.rs` + `cpsock::handshake::tests` | workspace | balde/porte, flood sem panic, legítimo preservado e INITCODE divergente recusado (mutação 3) |
| `QA-06` | `crates/wyd-server/tests/sistema/lotacao.rs` (5) | CI `sistema` | `Capacity`/`Maintenance` recusam antes da sessão, métrica e `/health` |
| `WEB-01` | `crates/wyd-admin/tests/contrato/transporte.rs` (14) | workspace | 44 rotas do portal com handler, `admin.pending=0`, 404 só fora da superfície (mutação 6) |
| `WEB-03` | `crates/wyd-admin/tests/contrato/painel.rs` + `contrato/operacao.rs` | workspace | moderação com motivo, trilha `audit_log`, economia de fonte real |

### Rebaixadas para 🟢 (20)

| ID | O que existe | Por que não sustenta ✅ |
|---|---|---|
| `CLIENT-04` | dossiê em `docs/cliente/` com SHA-256 do `WYD.exe` | nenhum teste lê o dossiê nem falha se o binário/offset mudar |
| `DEVOPS-01` | `.dockerignore`, bases por digest, `docker export` sem segredo | a ausência de segredo é inspeção manual; o CI só constrói |
| `DEVOPS-04` | `make check` = lista do CI; prova negativa na branch `tmp/prova-negativa` | nenhum script compara as duas listas hoje; a prova foi um evento, não um teste |
| `DEVOPS-05` | cache/`timeout-minutes`/nextest; 38,7 → 20,0 min medidos | custo de runner é medição de runs passados |
| `DEVOPS-09` | runner `self-hosted,wyd-local` + deploy após `gate` | operação de host e sentinela manual; nada no repo falha se o runner morrer |
| `DEVOPS-11` | estágios por `needs`, `gate` único, `notify` | notificação não tem asserção; estrutura de workflow não é testada |
| `DEVOPS-13` | `CODEOWNERS`, `SECURITY.md`, proteção de branch via API | settings vivem no GitHub; não há teste possível no repo |
| `PROC-02` | Go desinstalado, `PATH`/`GOPATH` limpos | estado da máquina do dono, não comportamento de teste |
| `PROC-05` | tabela das 36 issues em `PROC-05-auditoria-2026-11-12.md` | documento de auditoria |
| `PROC-10` | grafo/ordem topológica em `roadmap-rust/index.md` | documento; nenhum teste lê o grafo |
| `PROC-11` | revisão fria em `PROC-11-revisao-fria-2026-11-12.md` | documento de evidência |
| `PROC-12` | `docs/arquitetura/visao-pastas.md` + `docs/cliente/mapa.md` | documentos |
| `PROC-14` | `docs/arquitetura/modelo-banco.md` + catálogo gerado (69 tabelas) | documento; catálogo não é asserido por teste |
| `PROC-18` | 18 `SKILL.md` após três passadas | `check-skills.mjs` valida frontmatter/caminhos, não a contagem |
| `PROC-19` | `manual-técnico/guia-contribuicao.md` + `manual-usuario/README.md` | documentos |
| `PROC-20` | `docs/arquitetura/c4-pacotes.md` | documento |
| `PROC-22` | `docs/adr/` (primeiras ADRs) | documento; nenhum teste falha se o código divergir da ADR |
| `QA-07` | `docs/wyd/sistemas/ameacas.md` (B1–B17 + 29 ameaças) | catálogo é documento; a matemática da tabela é coberta por `GAME-06` |
| `QA-08` | `docker/monitoring/` + 10 regras + dashboard `wyd-saude` | `promtool test rules` cobre disparo/resolução (`DEVOPS-02`), não os 12 painéis/runbook |
| `WEB-04` | 6 specs Playwright em `wyd-portal/tests/e2e/`; 3 execuções verdes em 2026-11-12 | nenhum job do CI executa Playwright |

## Mutações (prova de que o teste não é decorativo)

Cada mutação foi aplicada no código, o teste alvo foi rodado até ficar **vermelho**, o código foi
restaurado e o mesmo teste voltou a **verde**. Nenhum mutante permaneceu na árvore.

| # | Issue | Mutação | Teste | Vermelho | Verde |
|---|---|---|---|---|---|
| 1 | `GAME-16` | `pity_gain` sempre `false` (`refine.rs`) | `cargo test -p wyd-sim --lib pity_only_grows_on_three_of_four_rolls` | FAILED (assert `refine.rs:632`) | 1 passed |
| 2 | `GAME-06`/`QA-07` | `select_target`: `>` → `<` e desempate invertido (`formula/threat.rs`) | `cargo test -p wyd-sim --lib highest_threat_wins` | FAILED | 1 passed |
| 3 | `QA-05` | gate de INITCODE virou `if false && received != INITCODE` (`cpsock/handshake.rs`) | `cargo test -p wyd-server --test sistema_handshake initcode_errado_nao_entra_no_handshake` | **passou** — teste decorativo (ver achados); novo unit test `cpsock::handshake::tests::initcode_divergente_e_recusado_antes_de_ler_o_frame` | FAILED com o mutante / 1 passed restaurado |
| 4 | `GAME-12` | `cap_for` sempre `1` (`systems/inventory.rs`) | `cargo test -p wyd-core --lib deposito_completa_pilha_depois_usa_slot_livre` | FAILED | 1 passed |
| 5 | `GAME-02` | `effective_refine` sem piso de +10 (`ecs/components.rs`, `plan_gem`) | `cargo test -p wyd-core --test integracao_gem anct_e_estritamente_melhor_que_comum_no_efeito_da_gema` | FAILED | 1 passed |
| 7 | `AUD-18` | escala da perfuração deslocada em 1 (`wyd-sim`, **ponto único** depois da unificação) | `cargo test -p wyd-sim --test golden_formulas ancient_gets_base_value_below_ten` **e** `cargo test -p wyd-core --test integracao_gem anct_e_estritamente_melhor_que_comum_no_efeito_da_gema` | **os dois FAILED** (`41 × 40`) | 6 passed + 6 passed |
| 6 | `WEB-01` | rota `accounts/find` renomeada para `accounts/lookup` (`transport/mod.rs`) | `cargo test -p wyd-admin --test contrato_transporte rota_do_portal_sem_handler_e_501_auditado_e_nunca_404` | FAILED (`404` na rota declarada) | 14 passed |

## Achados e correções

1. **Teste de INITCODE era decorativo (`QA-05`).** O teste de sistema mandava 4 bytes de lixo e
   esperava EOF; com o gate desligado o servidor fechava por timeout do frame, então continuava
   verde. Correção: `initcode_divergente_e_recusado_antes_de_ler_o_frame` em
   `crates/wyd-server/src/cpsock/handshake.rs`, que exige `HandshakeError::InitCode { received }`
   antes de ler o frame — vermelho com o mutante, verde sem ele. O teste de sistema também passou a
   mandar um frame de login bem formado depois do lixo, para não fechar por timeout.
2. **Regra de gema duplicada — corrigida.** Mutar `wyd-sim::apply_gem` não mudava o teste de
   integração porque `ComponentStore::plan_gem` (`crates/wyd-core/src/ecs/components.rs`)
   reimplementava o piso de +10. A `AUD-18` foi aberta e fechada no mesmo turno: a regra mora só
   em `wyd-sim` e o core consome a crate (ADR-0029), com `GemError → CoreError` e o contrato
   numérico `state_code`. Mutar o ponto único agora derruba **os dois** testes (mutação 7).
3. **Contrato de transporte não cobre rota fora da superfície do portal.** Renomear
   `panel/history` ou `accounts/recovery` não derruba o teste porque o portal não declara essas
   rotas; a asserção só protege o que o `admin-client.ts` declara. Não é defeito do teste (o escopo
   é a superfície do portal), mas é limite declarado.
4. **`CLIENT-03` tem 1 teste `#[ignore]` de rede** (`baixa_asset_publico_https_com_hash_e_reusa_cache`,
   `crates/wyd-launcher/tests/integracao/cliente_zip.rs:263`): o painel diz "27/27", o runner executa
   26 + 1 ignorado. O comportamento principal (aplicar/backup/recusar adulterado) é executado.
5. **`CLIENT-07` tinha duas linhas no painel**; a redundante foi removida (50 IDs canônicos).
6. **Eixo cliente/live continua fora deste pass**, como combinado: nenhuma prova visual no 7662 e
   nenhum canal vivo foram exigidos ou executados aqui.

## Medições deste turno

- `cargo test --workspace --all-targets` → **850 passed, 0 failed, 1 ignored** em 76 alvos com
  resultado, exit 0 (`tmp/test-aud18.log`). O único `#[ignore]` é o teste de rede acima; os 850
  incluem o teste novo de INITCODE e os dois testes do contrato de estado da gema (`AUD-18`).
- Suíte de sistema com Postgres real (`WYD_TEST_REQUIRED=1`, DSN no banco descartável
  `wyd_auditoria`) → **300 passed, 0 failed, 0 ignored** em 40 alvos com resultado, exit 0
  (`tmp/sistema-aud18.log`); 7 SKIPs, todos de `WYD_LIVE_CPSOCK` (canal vivo, fora deste pass).
- `cargo fmt --all --check` → 0; `cargo clippy --workspace --all-targets -- -D warnings` → 0.
- `node tools/content/check-issue-status.mjs` → **58/58** issues ativas comparadas, 0 divergências,
  exit 0 (a `AUD-18` foi aberta e fechada no mesmo turno, então a contagem voltou a 58).
- `node tools/dev/check-docs-consistency.mjs` → **58 issues, 9 crates, 10 membros, 50 migrations,
  255 docs, 389 links — consistente**, exit 0.

## O que fica aberto

- As 20 issues em 🟢 precisam de um teste que o runner execute para voltar a ✅; a nota de cada uma
  no painel já diz qual é a prova que falta.
- `WEB-04`: ligar Playwright a um job do CI (hoje a suíte só roda à mão) é o caminho mais curto
  para devolver o ✅.
- Cliente/live (render no 7662, captura de frames) segue como pendência declarada do eixo
  `CLIENT-06`/`CLIENT-12`, sem prova neste turno.

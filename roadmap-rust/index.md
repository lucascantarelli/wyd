# Roadmap Rust — índice executável

> **Estado:** 2026-11-15 · **61 issues ativas** (41 anteriores + 20 da auditoria) + 15
> descritores de fase. 64 issues concluídas estão em [`changelog.md`](changelog.md).
>
> **Recusa de login provada no cliente real (2026-11-15):** o 7662 na tela de LOGIN contra o canal
> vivo desenha o registro 12 de `client/UI/strdef.bin` ("Conexão anterior finalizada. Tente
> novamente.") ao receber **um** `0x011C` header-only — capturas `target/login-recusa-1.jpg` e
> `target/login-inexistente-recusa-1.jpg`, harness `tools/client/captura-sessao.ps1`. Resta a prova
> visual de PIN, cena e Campo de Treinamento (`BUG-02`, `GAME-17`, `GAME-18`).
>
> **Auditoria de DevOps/CI:** das doze issues da análise de pipeline, imagens, compose, testes e
> monitoramento, **todas as doze estão fechadas** (`DEVOPS-01`…`DEVOPS-13`); o
> contrato continua em [`docs/rust/10-devops-ci.md`](../docs/rust/10-devops-ci.md). Em 2026-11-13
> fecharam `DEVOPS-03` (compose de produção com log com teto, `no-new-privileges` e portal em
> loopback; **TLS não adotado por decisão**), `DEVOPS-04` (reprovação por divergência injetada,
> provada em run real), `DEVOPS-05` (toolchain pinado em 1.100.0 e runner de 38,7 → 20,0 min),
> `DEVOPS-07` (anti-skip provado no CI), `DEVOPS-08` (baseline de cobertura e gate × métrica),
> `DEVOPS-09` (deploy por pull em 83 s), `DEVOPS-11` (pipeline em estágios, notificação nos dois
> sentidos), `DEVOPS-13` (governança; **decisão: não migrar para a organização `dolorestec`**) e
> `DEVOPS-12` (**39 usos de action pinados por SHA**, conferidos contra a tag na API pelo
> `check-actions-pin`, com prova negativa) e `DEVOPS-10` (**`v0.1.0` tagueada em `main` e release
> publicada** com binários, `SHA256SUMS`, SBOM por imagem e anúncio no `#wyd-releases`). Não sobrou
> nenhuma issue DEVOPS aberta; o procedimento e o rollback ficam no §8 do contrato.
>
> **IDs canônicos preservados.** A numeração de fila `#01..#27` é nova e estável; os IDs
> `CATEGORIA-NN` continuam os mesmos para não reutilizar IDs já aposentados no changelog
> (ex.: `GAME-01` já é o tick concluído) nem quebrar as centenas de referências em docs, testes
> e comentários. O mapeamento histórico→canônico segue no fim deste índice.
>
> **Regra de ouro:** puxar primeiro os itens de esforço **S/M** e sem dependência externa;
> tarefas **L** só entram depois de decompostas em subtasks no próprio arquivo. Item bloqueado
> por captura, 2FA, patch ou decisão não entra na fila curta.

## Fila curta — ganhos rápidos sem bloqueio externo

| # | ID | Issue | Esforço | Próximo passo |
|---|---|---|---|---|
| 01 | `QA-11` | [Bots jogáveis e carga](issues/QA-11-bots-carga.md) | M | Rodar 2 bots em loopback e a carga de 200 com P99/rejeições. |
| 11 | `GAME-22` | [Árvore de EXP por flags](issues/GAME-22-exp-por-flags.md) | M | Fechar a árvore com o default inalterado e provar por flags. |
| 25 | `PROC-01` | [Organização da `docs/`, dedup e MkDocs](issues/PROC-01-organizacao-docs-mkdocs.md) | M | Fechar a dedup da documentação e publicar o MkDocs. |

> `CLIENT-06`, `CLIENT-10`, `BUG-01`, `BUG-02`, `GAME-17`, `GAME-18`, `WEB-07`, `WEB-08`,
> `WEB-09`, `WEB-10`, `WEB-11`, `CLIENT-11` e `GAME-13` têm resíduo bloqueado por captura/VM,
> 2FA, patch, decisão de contrato ou layout UNVERIFIED; não são “ganho rápido” até o gate cair.

## Resíduos de prova (não são ganhos rápidos)

| Issue | Status | O que já está provado | O que falta | Dependência |
|---|---|---|---|---|
| `BUG-01` | ⬜ | Recusas com motivo no log, testes de fio e entrada com `ClientID`/`UpdateScore`/cena. | Matriz mensagem × exibe/não-exibe e reprodução de cada recusa no 7662. | Cliente/VM (`CLIENT-06`). |
| `BUG-02` | ⬜ | Código, testes de fio e `clippy`; seleção antes do PIN devolve o frame. | Prova viva com `WYD_LIVE_SKIP_PIN=1` e entrada no cliente 7662. | Cliente/VM (`CLIENT-06`). |
| `GAME-17` | 🚧 | Faixa de `ClientID`, `UpdateScore`, `CreateType` e rajada real de `CreateMob`. | Render da entrada no cliente 7662, sem “Falha ao conectar”. | Cliente/VM (`CLIENT-06`). |
| `GAME-18` | 🟡 | 10 NPCs no seed `0046`, Sephira +9 no `0049`, 4 etapas, portões/chaves, loot no inventário, guardiões/boss no pipeline real de morte/loot (`integracao_campo_drops`) e `ReqLvl` do catálogo aplicado. | Buff do `Cabuncle`, portão visível e prova visual no 7662. | Captura no cliente/VM. |
| `GAME-11` | 🚧 | Depósito no inventário no abate e regra pura da caixa (30 s/3 min). | Handler `CreateItem`/`GetItem`/`DecayItem` e faixa de id do chão. | Captura do layout de item no chão (`CLIENT-06`). |
| `GAME-05` | 🚧 | Física, dano, skills e inventário autoritativos; recusa de pacote forjado. | `req_str/int/dex/con`, layout de rede do inventário e baú (`vault_items`). | `CLIENT-06` para o layout. |
| `CLIENT-10` | ⛔ | Layout candidato de 156 B + verificador `trade-layout.mjs` + runbook. | Frame real `MSG_Trade` do 7662. | Captura do usuário/VM. |

> Roteiro consolidado para as quatro provas visuais: [`docs/cliente/roteiro-prova-visual-7662.md`](../docs/cliente/roteiro-prova-visual-7662.md).
>
> **Tentativa de 2026-11-12:** o cliente 7662 abriu e desenhou o server select, mas descartou
> mouse e teclado sintéticos (`Fechar` não reagiu); sem login no canal e sem TCP em `8281`.
> O `Config.bin` foi corrigido do modo 8 (`1024×768×32`, cliente `1024×759`) para o modo 7
> (`800×600×32`, cliente exato `800×600`), mas o erro de render target e o input inerte
> permaneceram; modo 16 bits e GPU NVIDIA também não mudaram o quadro. `BUG-01`, `BUG-02` e
> `GAME-17` ficaram com veredito **B** na matriz, sem marcar aceite visual; `GAME-18` ficou
> **N/A** nesta sessão. Próximo passo: checklist humano do runbook §10.

## Fila de desbloqueio (quem trava o quê)

| Ordem | ID | Issue | Por que agora |
|---|---|---|---|
| 01 | `QA-11` | [Bots e carga](issues/QA-11-bots-carga.md) | Crate existe; falta prova de carga e relatório. |
| 02 | `CLIENT-06` | [RE dinâmica](issues/CLIENT-06-cliente-reverso-dinamico.md) | Captura de layout destrava trade (CLIENT-10; candidato 156 B × 154 B), pacotes S→C e provas visuais. |
| 03 | `BUG-02` | [PIN opcional](issues/BUG-02-entrada-no-mundo-pin-opcional.md) | Código/testes verdes; restam duas provas no cliente real. |
| 04 | `GAME-17` | [Entrada fiel](issues/GAME-17-entrada-no-mundo-fiel.md) | Só falta prova visual; destrava a experiência do cliente e o Campo. |

## Grafo de dependências das fases

Fonte única: o cabeçalho `Depende de`/`Bloqueia` de cada `roadmap-rust/issues/FASE-NN.md`.
A aresta `A → B` significa **B depende de A**; `FASE-02` é um portão de ritmo, não uma
dependência dura de todas as fases.

| Fase | Tema | Depende de | Bloqueia |
|---|---|---|---|
| `FASE-01` | Auditoria do roadmap e issues | — (raiz) | FASE-02 a FASE-15 |
| `FASE-02` | Execução controlada e ritmo | FASE-01 | FASE-03 a FASE-15 (ritmo) |
| `FASE-03` | Engine `wyd-core` (tick, ECS, grid) | FASE-01, FASE-08 | FASE-06, FASE-10 |
| `FASE-04` | Banco e persistência (`wyd-db`) | FASE-01 | FASE-07 |
| `FASE-05` | DevOps e CI/CD | FASE-01 | FASE-15 |
| `FASE-06` | Observabilidade e segurança | FASE-03 | FASE-12 |
| `FASE-07` | Portal web e ecossistema | FASE-04 | — |
| `FASE-08` | Protocolo, rede e launcher | FASE-01 | FASE-03 |
| `FASE-09` | IA, skills e ferramentas | FASE-01 | — (contínua) |
| `FASE-10` | Matemática e balanceamento | FASE-03, FASE-08 | FASE-15 |
| `FASE-11` | Documentação e padronização | FASE-01 | — (contínua) |
| `FASE-12` | Dashboards e visualização | FASE-06 | — |
| `FASE-13` | ADRs | FASE-01 | — (contínua) |
| `FASE-14` | Pesquisa técnica | — (contínua) | — (contínua) |
| `FASE-15` | Auditoria final e release | FASE-03 a FASE-08, FASE-10 | release |

```text
FASE-01 (raiz)
├── FASE-02 ──(ritmo)──> FASE-03..15
├── FASE-08 ──> FASE-03 ──> FASE-06 ──> FASE-12
│                          ├──> FASE-10 ──┐
│                          └──────────────┼──> FASE-15 (release)
├── FASE-04 ──> FASE-07 ──────────────────┤
├── FASE-05 ──────────────────────────────┤
├── FASE-09 / FASE-11 / FASE-13 (contínuas)
└── FASE-14 (contínua, sem aresta)
```

**Ordem topológica:** `01 → 02 → 08 → 04 → 05 → 09 → 11 → 13 → 03 → 06 → 10 → 07 → 12 → 15`;
`FASE-14` é contínua e pode rodar em qualquer ponto.

**Caminho crítico de release:** `FASE-01 → FASE-08 → FASE-03 → FASE-10 → FASE-15`.
É a maior cadeia de dependências duras (5 fases). `FASE-03 → FASE-06 → FASE-12` tem o mesmo
comprimento, mas termina no dashboard; `FASE-15` também espera `FASE-04`, `FASE-05`,
`FASE-06` e `FASE-07`, então atraso em qualquer um desses ramos atrasa o release.

**Caminho crítico de produto/portal:** `FASE-08 → FASE-03 → FASE-04 → FASE-07`.
A aresta `03→04` é a ordem de valor do §16 (contratos → engine → persistência → APIs), **não**
uma dependência declarada: `FASE-04` pode começar após `FASE-01`, mas o portal só fecha com o
contrato de rede e a engine estáveis.

Nenhuma fase fica fora do grafo: `FASE-01` é a raiz, `FASE-15` é o sorvedouro e `FASE-14` é
isolada por desenho contínuo. Não há fase órfã acidental.

## Issues ativas por categoria

> `#` é a posição na fila nova (01–27), não o ID. A ordem prioriza o que não depende de
> captura, 2FA, patch ou decisão pendente; os bloqueados aparecem no fim da fila.

### Bugs de fio (2)

| # | ID | Issue | Status | Esforço | Depende de |
|---:|---|---|---|---|---|
| 06 | `BUG-01` | [Mensagens do cliente ausentes + personagem não entra no mundo](issues/BUG-01-mensagens-char-mundo.md) | ⬜ | M | CLIENT-06 |
| 03 | `BUG-02` | [“Falha ao conectar” ao entrar no mundo: PIN opcional e seleção antes do PIN](issues/BUG-02-entrada-no-mundo-pin-opcional.md) | ⬜ | S | GAME-17 🚧 (mesma prova visual) |

### Jogo e gameplay (14)

| # | ID | Issue | Status | Esforço | Depende de |
|---:|---|---|---|---|---|
| 08 | `GAME-05` | [Autoridade total do servidor (física, dano, skills, inventário)](issues/GAME-05-autoridade-servidor.md) | 🚧 | M | GAME-02 ✅, GAME-03 ✅, GAME-04 ✅; layout CLIENT-06 |
| 13 | `GAME-11` | [Drop no inventário e caixa no chão com posse](issues/GAME-11-drop-inventario-e-caixa.md) | 🚧 | M | CLIENT-06 (id do item no chão); depósito já feito |
| 12 | `GAME-13` | [Familiares: slot, Nyerds, Imp, Griupan e Trajes](issues/GAME-13-familiares-slot-e-itens.md) | ⬜ | M | CLIENT-06 (slot Familiar) |
| 14 | `GAME-14` | [Fadas: bônus de drop/EXP, duração e auto-coleta](issues/GAME-14-fadas-bonus-e-auto-coleta.md) | ⬜ | M | GAME-11 |
| 04 | `GAME-17` | [Entrada no mundo fiel ao legado (id de jogador, `UpdateScore`, `CreateType`)](issues/GAME-17-entrada-no-mundo-fiel.md) | 🚧 | M | GAME-08 ⬜, GAME-09 ⬜; prova visual |
| 05 | `GAME-18` | [Campo de Treinamento: quest, portões, buff, boss e recompensas](issues/GAME-18-campo-de-treinamento.md) | 🟡 | L | GAME-17; prova visual |
| 09 | `GAME-20` | [Anomalia de velocidade (~200%) e hits duplicados](issues/GAME-20-velocidade.md) | ⬜ | M | GAME-19 (reaberta; ordem provisória) |
| 10 | `GAME-21` | [Laboratório `wyd-sim` (milhões de iterações)](issues/GAME-21-laboratorio.md) | ⬜ | M | GAME-19, GAME-20 |
| 11 | `GAME-22` | [Árvore de EXP por flags (default inalterado)](issues/GAME-22-exp-por-flags.md) | ⬜ | M | GAME-08 |
| 07 | `GAME-23` | [Bots autônomos do Campo de Treinamento (um por raça)](issues/GAME-23-bots-do-campo-de-treinamento.md) | 🟡 | M | QA-11 🟡, GAME-18 🟡 |
| 32 | `GAME-08` | [Entrada no mundo: personagem sem atributos, CP errado e nome vermelho](issues/GAME-08-entrada-no-mundo-spawn.md) | ⬜ | M | CLIENT-06 |
| 33 | `GAME-09` | [Cena autoritativa: NPC não clica](issues/GAME-09-cena-autoritativa.md) | ⬜ | M | CLIENT-06 |
| 34 | `GAME-10` | [Cidadania: mensagem e indicador no login](issues/GAME-10-cidadania-canal.md) | ⬜ | S | GAME-17 |
| 35 | `GAME-19` | [Combate: 1 de dano e monstro de treino não morre](issues/GAME-19-combate-dano-real.md) | ⬜ | M | GAME-08 |

### Cliente, protocolo e launcher (5)

| # | ID | Issue | Status | Esforço | Depende de |
|---:|---|---|---|---|---|
| 02 | `CLIENT-06` | [Engenharia reversa dinâmica e correlação](issues/CLIENT-06-cliente-reverso-dinamico.md) | 🚧 | L | CLIENT-05 ✅, CLIENT-02 ✅ |
| 20 | `CLIENT-08` | [GUI do launcher em Tauri v2](issues/CLIENT-08-launcher-gui-tauri.md) | 🚧 | L | CLIENT-03 ✅, CLIENT-07 ✅ |
| 27 | `CLIENT-10` | [Trade atômica por par (layout candidato; captura pendente)](issues/CLIENT-10-trade-atomica.md) | ⛔ | M | CLIENT-06 (captura do layout) |
| 21 | `CLIENT-11` | [Launcher completo + integração com o portal + autoupdate](issues/CLIENT-11-portal-autoupdate.md) | 🚧 | L | CLIENT-03 ✅, CLIENT-08 🚧 |
| 30 | `CLIENT-12` | [Comparação visual: modo de vídeo, render target e gerações](issues/CLIENT-12-comparacao-visual-geracoes.md) | ⬜ | S | cliente 7662 + GPU local |

### Portal e gateway (5)

| # | ID | Issue | Status | Esforço | Depende de |
|---:|---|---|---|---|---|
| 15 | `WEB-07` | [Monitoramento customizado: logs ao vivo, servidor, DB e alertas](issues/WEB-07-monitoramento-custom.md) | 🚧 | L | WEB-01 ✅ |
| 18 | `WEB-08` | [Contas, segurança, auditoria e área pública × administrativa](issues/WEB-08-contas-seguranca-auditoria.md) | 🚧 | L | WEB-03 ✅; 2FA/e-mail |
| 19 | `WEB-09` | [Entregas → Premium Mail + Cash + gestão financeira](issues/WEB-09-premium-cash.md) | 🚧 | L | WEB-04 ✅; inbox do jogador |
| 16 | `WEB-10` | [Catálogo de itens + economia do servidor](issues/WEB-10-catalogo-economia.md) | 🚧 | M | WEB-05 ✅; atributos no gateway |
| 17 | `WEB-11` | [Configurações 100% via portal (sem arquivo manual)](issues/WEB-11-configuracoes.md) | 🚧 | M | PROC-24; rollback/histórico |

### Qualidade, carga e segurança (7)

| # | ID | Issue | Status | Esforço | Depende de |
|---:|---|---|---|---|---|
| 22 | `QA-01` | [Bots população real + LLM + gestão de logs de teste](issues/QA-01-populacao-llm-carga.md) | ⬜ | L | BUG-01 |
| 23 | `QA-09` | [Suite completa + concorrência (tarpaulin/miri)](issues/QA-09-suite-concorrencia.md) | ⬜ | M | QA-02 ✅, GAME-21 |
| 24 | `QA-10` | [Carga de rede + integridade do banco + build de produção](issues/QA-10-carga-banco-release.md) | ⬜ | M | QA-09, QA-08 ✅ |
| 01 | `QA-11` | [Bots jogáveis e carga (`crates/wyd-bots`)](issues/QA-11-bots-carga.md) | 🟡 | M | GAME-06 ✅, CLIENT-02 ✅, QA-05 ✅ |
| 28 | `QA-12` | [Validação manual da entrada no mundo e das mensagens (7662)](issues/QA-12-validacao-manual-entrada-mundo.md) | ⬜ | M | stack no ar, conta de teste |
| 29 | `QA-13` | [Dois clientes: sessão, combate, loot, refino e logout](issues/QA-13-validacao-manual-dois-clientes.md) | ⬜ | M | QA-12 |
| 31 | `QA-14` | [Suíte live do servidor: 3 falhas conhecidas](issues/QA-14-suite-live-falhas.md) | 🟡 | S | stack no ar + banco descartável |

### Processo, documentação e arquitetura (2)

| # | ID | Issue | Status | Esforço | Depende de |
|---:|---|---|---|---|---|
| 25 | `PROC-01` | [Organização da `docs/`, dedup e MkDocs](issues/PROC-01-organizacao-docs-mkdocs.md) | 🚧 | M | PROC-13 ✅ |
| 26 | `PROC-24` | [Reorganização do roadmap: caminho crítico, curva de crescimento e entregas reais](issues/PROC-24-curva-crescimento.md) | 🚧 | M | PROC-01 |

### DevOps e CI (10)

> Origem: auditoria completa de `.github/workflows/rust.yml`, `Makefile`, `docker/Dockerfile.*`,
> `docker-compose*.yml` e `docker/monitoring/**` em 2026-11-12. Contrato e achados por severidade
> (P0/P1/P2) em [`docs/rust/10-devops-ci.md`](../docs/rust/10-devops-ci.md) §3; ordem recomendada
> em §10. Nada aqui depende de captura do cliente ou de VM.
>
> **Bloco encerrado em 2026-11-13:** as doze issues (`DEVOPS-01`…`DEVOPS-13`) estão concluídas e
> registradas em [`changelog.md`](changelog.md); não há issue DevOps/CI ativa.

## Roadmap de paridade — issues abertas pela auditoria (2026-11-14)

> Origem: [`docs/audit/17-priorities-and-roadmap.md`](../docs/audit/17-priorities-and-roadmap.md).
> Cada entrega da auditoria está mapeada para a issue que a executa. Onde já existia issue, ela
> foi reusada (ID em `código`, sem link duplicado); onde não existia, nasceu uma `AUD-NN` com
> objetivo, passos, aceite e testes exigidos. Fase 4 não entra na fila antes de P1 fechar.
> Medição e método: [`docs/rust/evidencias/auditoria-roadmap-2026-11-14.md`](../docs/rust/evidencias/auditoria-roadmap-2026-11-14.md).

### Fase 0 — destravar o conhecimento

| ID | Entrega | Status | Esforço | Depende de |
|---|---|---|---|---|
| `CLIENT-06` | Captura S→C do 7662 (item 0.1) | 🚧 | L | VM + cliente |
| `AUD-01` | [Dissector CPSock e decisão Frida/XTrap](issues/AUD-01-dissector-cpsock-e-frida.md) | ⬜ | M | `CLIENT-06` |
| `AUD-19` | [Fechar as 37 incógnitas do audit (`UNK-01`…`UNK-37`)](issues/AUD-19-incognitas-da-auditoria.md) | ⬜ | XL (decompor) | `CLIENT-06` nas de fio |

### Fase 1 — loop jogável fiel

| ID | Entrega | Status | Esforço | Depende de |
|---|---|---|---|---|
| `BUG-01` | Senha/recusa visível no cliente (item 1.1) | ⬜ | M | `CLIENT-06` |
| `AUD-03` | [`SELCHAR`/`MOB` com offsets reais](issues/AUD-03-offsets-reais-selchar-mob.md) | ⬜ | M | `CLIENT-06`, `AUD-01` |
| `AUD-04` | [Colisão de jogador](issues/AUD-04-colisao-de-jogador.md) | ⬜ | M | `GAME-05` |
| `GAME-09` | Movimento/cena no fio (item 1.5) | ⬜ | M | `CLIENT-06` |
| `AUD-05` | [Regen de HP/MP e respawn de NPC](issues/AUD-05-regen-hp-mp-e-respawn-npc.md) | ⬜ | S | tick |
| `AUD-06` | [Cadência de ataque provada](issues/AUD-06-cadencia-de-ataque-provada.md) | ⬜ | S | `GAME-19`, `GAME-20` |
| `AUD-21` | [Reconciliação documental e achados sem issue](issues/AUD-21-reconciliacao-documental.md) | 🚧 | M | — |

### Fase 2 — itens, economia e comércio

| ID | Entrega | Status | Esforço | Depende de |
|---|---|---|---|---|
| `GAME-11` | Item no chão e pickup (item 2.1) | 🚧 | M | `CLIENT-06` |
| `GAME-05` | Mover/equipar no fio (item 2.2) | 🚧 | M | layout |
| `AUD-07` | [Loja de NPC completa](issues/AUD-07-loja-de-npc-completa.md) | ⬜ | M | `GAME-05` |
| `CLIENT-10` | Trade atômica (item 2.4) | ⛔ | M | `CLIENT-06` |
| `AUD-08` | [Baú/armazém](issues/AUD-08-bau-e-armazem.md) | ⬜ | M | `GAME-05` |
| `AUD-09` | [Skill no fio](issues/AUD-09-skill-no-fio.md) | ⬜ | M | `GAME-05` |
| `AUD-10` | [Mix/compose e cadeia +12…+15/Anct](issues/AUD-10-composicao-e-cadeia-alta.md) | ⬜ | L | `GAME-11` |
| `AUD-20` | [Decidir modelos divergentes: drop, piedade e PK](issues/AUD-20-decisoes-de-modelo.md) | ⬜ | M | `AUD-10` (simulação) |

### Fase 3 — social e progressão

| ID | Entrega | Status | Esforço | Depende de |
|---|---|---|---|---|
| `GAME-25` | Chat, whisper e filtro (item 3.1) | ⬜ | M | — |
| `AUD-11` | [Party e EXP compartilhada](issues/AUD-11-party-e-exp-compartilhada.md) | ⬜ | L | `GAME-25` |
| `GAME-24` | Guilda e eventos (itens 3.3 e 3.7) | ⬜ | L | `GAME-25` |
| `AUD-12` | [`ApplyBonus` no fio](issues/AUD-12-applybonus-no-fio.md) | ⬜ | S | `GAME-05` |
| `AUD-13` | [Teleporte e portal](issues/AUD-13-teleporte-e-portal.md) | ⬜ | M | `AUD-04` |
| `AUD-14` | [Quest genérica](issues/AUD-14-quest-generica.md) | ⬜ | L | `AUD-13` |
| `AUD-15` | [PK/chaos e cor do nome](issues/AUD-15-pk-chaos-e-cor-do-nome.md) | ⬜ | L | `AUD-06` |

### Fase 4 — expansão (bloqueada por desenho)

| ID | Entrega | Status | Esforço | Depende de |
|---|---|---|---|---|
| `AUD-16` | [Escritor `.wyt`](issues/AUD-16-escritor-wyt.md) | ⬜ | S | — |
| `CLIENT-12` | Modo de vídeo e gerações (item 4.2) | ⬜ | S | cliente/GPU |
| `AUD-17` | [Conversor de malha e editor de mapa](issues/AUD-17-conversor-malha-e-editor-mapa.md) | ⬜ | XL | `AUD-16` (decompor antes) |
| `AUD-22` | [Decisão de cliente independente (item 4.5)](issues/AUD-22-decisao-cliente-independente.md) | ⛔ | S | `AUD-17`; paridade P1–P3 |

### Cobertura item a item do §17

| Entrega | Issue | Entrega | Issue | Entrega | Issue |
|---|---|---|---|---|---|
| 0.1 | `CLIENT-06` | 1.6 | `AUD-05` | 2.8 | `AUD-10` |
| 0.2 | `AUD-01` | 1.7 | `AUD-05` | 3.1 | `GAME-25` |
| 0.3 | `AUD-01` | 1.8 | `AUD-06` | 3.2 | `AUD-11` |
| 0.4 | `AUD-02` ✅ | 2.1 | `GAME-11` | 3.3 | `GAME-24` |
| 1.1 | `BUG-01` | 2.2 | `GAME-05` | 3.4 | `AUD-12` |
| 1.2 | `AUD-03` | 2.3 | `AUD-07` | 3.5 | `AUD-13` |
| 1.3 | `AUD-03` | 2.4 | `CLIENT-10` | 3.6 | `AUD-14` |
| 1.4 | `AUD-04` | 2.5 | `AUD-08` | 3.7 | `GAME-24` |
| 1.5 | `GAME-09` | 2.6 | `AUD-09` | 3.8 | `AUD-15` |
| 4.1 | `AUD-16` | 2.7 | `AUD-10` | 4.4 | `AUD-17` |
| 4.2 | `CLIENT-12` | 4.3 | `AUD-17` | 4.5 | `AUD-22` |

### Cobertura do audit além do §17

O §17 lista as 24 entregas, mas os outros documentos da auditoria também abrem itens. Todos têm
dono agora (checado por `node tools/content/check-audit-coverage.mjs`):

| Origem | Itens | Issue |
|---|---|---|
| `18-unknown-features.md` | 37 incógnitas `UNK-01`…`UNK-37` | `AUD-19` (tabela por grupo, com método e fase) |
| `15-bugs-and-divergences.md` §1 | `DIV-03`, `DIV-05`…`DIV-09`, `DIV-12`…`DIV-14` | `AUD-20` (decisões) e `AUD-21` (documental) |
| `15-bugs-and-divergences.md` §3 | `N-1`…`N-10` | `AUD-21`, exceto `N-4`/`N-5` (`AUD-05`), `N-7` (`GAME-05`) e `N-10` (`AUD-19`) |
| `17-priorities-and-roadmap.md` §2 | entrega 0.4 (direção do `CreateItem`) | `AUD-02` — **fechada em 2026-11-14** |
| `17-priorities-and-roadmap.md` §6 | item 4.5 (cliente independente) | `AUD-22` (decisão go/no-go) |

Os itens do §1/§2 e do §3 que já tinham issue continuam mapeados nela: `DIV-01`→`BUG-01`,
`DIV-02`→`AUD-02` (fechada), `DIV-04`→`CLIENT-10`, `DIV-10`→`AUD-04`, `DIV-11`→`GAME-20`,
`PAR-MOV-02`→`AUD-04`.

## Descritores de fase (contexto, não são issues)

Os arquivos `FASE-00.md` … `FASE-15.md` descrevem o escopo de cada fase e permanecem em `issues/` como referência.

## Convenções

- **Status:** ✅ concluída (só permanece aqui se o aceite ainda tem pendência) · 🚧 em andamento · ⬜ não iniciada · ⛔ bloqueada · 🟡 parcial · 🟢 pronta para validar.
- **Concluída de verdade:** critérios de aceite 100% marcados + evidência no arquivo. Ao fechar, remover o arquivo e registrar em `changelog.md`.
- **Skills:** cada issue referencia as skills de código, teste, docs e domínio na seção `Skills`; roteador em `.agents/skills/README.md`.

## Mapeamento histórico → canônico

O ID antigo não é usado como identificador de trabalho; serve apenas para localizar o registro histórico.

<details>
<summary>82 issues mapeadas</summary>

| ID histórico | ID canônico | Onde está |
|---|---|---|
| `BUG-01` | `BUG-01` | [`BUG-01-mensagens-char-mundo.md`](issues/BUG-01-mensagens-char-mundo.md) |
| `BUG-02` | `BUG-02` | [`BUG-02-entrada-no-mundo-pin-opcional.md`](issues/BUG-02-entrada-no-mundo-pin-opcional.md) |
| `FASE-03-1` | `GAME-01` | [`changelog.md`](changelog.md) |
| `FASE-03-2` | `GAME-02` | [`changelog.md`](changelog.md) |
| `FASE-03-3` | `GAME-03` | [`changelog.md`](changelog.md) |
| `FASE-03-4` | `GAME-04` | [`changelog.md`](changelog.md) |
| `FASE-03-5` | `GAME-05` | [`GAME-05-autoridade-servidor.md`](issues/GAME-05-autoridade-servidor.md) |
| `FASE-03-6` | `GAME-06` | [`changelog.md`](changelog.md) |
| `FASE-03-7` | `GAME-07` | [`changelog.md`](changelog.md) |
| `FASE-03-8` | `GAME-08` | [`changelog.md`](changelog.md) |
| `FASE-03-9` | `GAME-09` | [`changelog.md`](changelog.md) |
| `FASE-03-10` | `GAME-10` | [`changelog.md`](changelog.md) |
| `FASE-03-11` | `GAME-11` | [`GAME-11-drop-inventario-e-caixa.md`](issues/GAME-11-drop-inventario-e-caixa.md) |
| `FASE-03-12` | `GAME-12` | [`changelog.md`](changelog.md) |
| `FASE-03-13` | `GAME-13` | [`GAME-13-familiares-slot-e-itens.md`](issues/GAME-13-familiares-slot-e-itens.md) |
| `FASE-03-14` | `GAME-14` | [`GAME-14-fadas-bonus-e-auto-coleta.md`](issues/GAME-14-fadas-bonus-e-auto-coleta.md) |
| `FASE-03-15` | `GAME-15` | [`changelog.md`](changelog.md) |
| `FASE-03-16` | `GAME-16` | [`changelog.md`](changelog.md) |
| `FASE-03-17` | `GAME-17` | [`GAME-17-entrada-no-mundo-fiel.md`](issues/GAME-17-entrada-no-mundo-fiel.md) |
| `FASE-03-18` | `GAME-18` | [`GAME-18-campo-de-treinamento.md`](issues/GAME-18-campo-de-treinamento.md) |
| `FASE-10-1` | `GAME-19` | [`changelog.md`](changelog.md) |
| `FASE-10-2` | `GAME-20` | [`GAME-20-velocidade.md`](issues/GAME-20-velocidade.md) |
| `FASE-10-3` | `GAME-21` | [`GAME-21-laboratorio.md`](issues/GAME-21-laboratorio.md) |
| `FASE-10-4` | `GAME-22` | [`GAME-22-exp-por-flags.md`](issues/GAME-22-exp-por-flags.md) |
| `FASE-15-4` | `GAME-23` | [`GAME-23-bots-do-campo-de-treinamento.md`](issues/GAME-23-bots-do-campo-de-treinamento.md) |
| `FASE-08-1` | `CLIENT-01` | [`changelog.md`](changelog.md) |
| `FASE-08-2` | `CLIENT-02` | [`changelog.md`](changelog.md) |
| `FASE-08-3` | `CLIENT-03` | [`changelog.md`](changelog.md) |
| `FASE-08-4` | `CLIENT-04` | [`changelog.md`](changelog.md) |
| `FASE-08-5` | `CLIENT-05` | [`changelog.md`](changelog.md) |
| `FASE-08-6` | `CLIENT-06` | [`CLIENT-06-cliente-reverso-dinamico.md`](issues/CLIENT-06-cliente-reverso-dinamico.md) |
| `FASE-08-7` | `CLIENT-07` | [`changelog.md`](changelog.md) |
| `FASE-08-8` | `CLIENT-08` | [`CLIENT-08-launcher-gui-tauri.md`](issues/CLIENT-08-launcher-gui-tauri.md) |
| `FASE-08-9` | `CLIENT-09` | [`changelog.md`](changelog.md) |
| `FASE-08-10` | `CLIENT-10` | [`CLIENT-10-trade-atomica.md`](issues/CLIENT-10-trade-atomica.md) |
| `LAUNCHER-01` | `CLIENT-11` | [`CLIENT-11-portal-autoupdate.md`](issues/CLIENT-11-portal-autoupdate.md) |
| `FASE-07-1` | `WEB-01` | [`changelog.md`](changelog.md) |
| `FASE-07-2` | `WEB-02` | [`changelog.md`](changelog.md) |
| `FASE-07-3` | `WEB-03` | [`changelog.md`](changelog.md) |
| `FASE-07-4` | `WEB-04` | [`changelog.md`](changelog.md) |
| `FASE-07-5` | `WEB-05` | [`changelog.md`](changelog.md) |
| `FASE-07-6` | `WEB-06` | [`changelog.md`](changelog.md) |
| `PORTAL-01` | `WEB-07` | [`WEB-07-monitoramento-custom.md`](issues/WEB-07-monitoramento-custom.md) |
| `PORTAL-02` | `WEB-08` | [`WEB-08-contas-seguranca-auditoria.md`](issues/WEB-08-contas-seguranca-auditoria.md) |
| `PORTAL-03` | `WEB-09` | [`WEB-09-premium-cash.md`](issues/WEB-09-premium-cash.md) |
| `PORTAL-04` | `WEB-10` | [`WEB-10-catalogo-economia.md`](issues/WEB-10-catalogo-economia.md) |
| `PORTAL-05` | `WEB-11` | [`WEB-11-configuracoes.md`](issues/WEB-11-configuracoes.md) |
| `BOTS-01` | `QA-01` | [`QA-01-populacao-llm-carga.md`](issues/QA-01-populacao-llm-carga.md) |
| `FASE-05-1` | `QA-02` | [`changelog.md`](changelog.md) |
| `FASE-05-2` | `QA-03` | [`changelog.md`](changelog.md) |
| `FASE-06-1` | `QA-04` | [`changelog.md`](changelog.md) |
| `FASE-06-2` | `QA-05` | [`changelog.md`](changelog.md) |
| `FASE-06-3` | `QA-06` | [`changelog.md`](changelog.md) |
| `FASE-06-4` | `QA-07` | [`changelog.md`](changelog.md) |
| `FASE-12-1` | `QA-08` | [`changelog.md`](changelog.md) |
| `FASE-15-1` | `QA-09` | [`QA-09-suite-concorrencia.md`](issues/QA-09-suite-concorrencia.md) |
| `FASE-15-2` | `QA-10` | [`QA-10-carga-banco-release.md`](issues/QA-10-carga-banco-release.md) |
| `FASE-15-3` | `QA-11` | [`QA-11-bots-carga.md`](issues/QA-11-bots-carga.md) |
| `DOC-01` | `PROC-01` | [`PROC-01-organizacao-docs-mkdocs.md`](issues/PROC-01-organizacao-docs-mkdocs.md) |
| `FASE-00-1` | `PROC-02` | [`changelog.md`](changelog.md) · [evidência](../docs/rust/evidencias/PROC-02-limpeza-ambiente-2026-11-12.md) |
| `FASE-00-2` | `PROC-03` | [`changelog.md`](changelog.md) |
| `FASE-00-3` | `PROC-04` | [`changelog.md`](changelog.md) |
| `FASE-01-1` | `PROC-05` | [`changelog.md`](changelog.md) · [evidência](../docs/rust/evidencias/PROC-05-auditoria-2026-11-12.md) |
| `FASE-01-2` | `PROC-06` | [`changelog.md`](changelog.md) |
| `FASE-01-3` | `PROC-07` | [`changelog.md`](changelog.md) |
| `FASE-02-1` | `PROC-08` | [`changelog.md`](changelog.md) |
| `FASE-02-2` | `PROC-09` | [`changelog.md`](changelog.md) |
| `FASE-02-3` | `PROC-10` | [`changelog.md`](changelog.md) |
| `FASE-02-4` | `PROC-11` | [`changelog.md`](changelog.md) |
| `FASE-02-5` | `PROC-12` | [`changelog.md`](changelog.md) |
| `FASE-02-6` | `PROC-13` | [`changelog.md`](changelog.md) |
| `FASE-04-1` | `PROC-14` | [`changelog.md`](changelog.md) |
| `FASE-04-2` | `PROC-15` | [`changelog.md`](changelog.md) |
| `FASE-04-3` | `PROC-16` | [`changelog.md`](changelog.md) |
| `FASE-09-1` | `PROC-17` | [`changelog.md`](changelog.md) |
| `FASE-09-2` | `PROC-18` | [`changelog.md`](changelog.md) |
| `FASE-11-1` | `PROC-19` | [`changelog.md`](changelog.md) |
| `FASE-11-2` | `PROC-20` | [`changelog.md`](changelog.md) |
| `FASE-13-1` | `PROC-21` | [`changelog.md`](changelog.md) |
| `FASE-13-2` | `PROC-22` | [`changelog.md`](changelog.md) |
| `FASE-14-1` | `PROC-23` | [`changelog.md`](changelog.md) |
| `ROAD-01` | `PROC-24` | [`PROC-24-curva-crescimento.md`](issues/PROC-24-curva-crescimento.md) |

</details>

## Reestruturação 2026-11-14

> Todos os checkboxes de aceite foram **desmarcados** em 2026-11-14 (`PROC-25`): nada é
> "concluído/testado" sem reexecução no commit atual.

Issues abertas para cobrir o restante do prompt master (cobertura/gate, paridade comparativa,
revalidação, sistemas sociais e clientes alternativos). Todas entram como ⬜ e sem aceite marcado.

| ID | Issue | Status |
|---|---|---|
| `QA-15` | [Cobertura mínima com gate (95% unit / 80% integração)](issues/QA-15-cobertura-minima-e-gate.md) | ⬜ |
| `QA-16` | [Testes comparativos OpenWYD × Rust](issues/QA-16-testes-de-paridade-openwyd-rust.md) | ⬜ |
| `PROC-25` | [Revalidação e rebaixamento de status](issues/PROC-25-revalidacao-e-rebaixamento.md) | ⬜ |
| `GAME-24` | [Eventos e guildas](issues/GAME-24-eventos-e-guildas.md) | ⬜ |
| `GAME-25` | [Chat e comunicação](issues/GAME-25-chat-e-comunicacao.md) | ⬜ |
| `CLIENT-13` | [Clientes alternativos e gerações visuais](issues/CLIENT-13-clientes-alternativos-e-geracoes.md) | ⬜ |

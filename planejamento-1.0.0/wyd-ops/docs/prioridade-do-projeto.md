> Pergunta: "qual é o foco efetivo daqui por diante e, para cada decisão aberta de foco, gate de código, escopo das `migrations/`, critério de organização e dependências, qual opção resolve mais bloqueios, o que ela adia e o que só o dono pode decidir?"
> Público: dono do produto, arquitetura, `wyd-ops` e quem coordena as frentes
> Dono: arquitetura publica; **foco e escopo são decisão do dono do produto; gate e dependências são de `wyd-ops` + arquitetura**
> Base: `planejamento-1.0.0/wyd-1.0.0.md` §1 e §6; `planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §6.6.22–§6.6.24; `planejamento-1.0.0/wyd-db/tests/specs/capacidades.md`; `planejamento-1.0.0/wyd-db/docs/specs/rastreabilidade.md` §1.1–§3.4; `AGENTS.md` §8 e §9; `plano-ci-real.md`; `proposta-versoes-e-cobertura-funcional.md`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24) — **quadro canônico de foco e decisões**; recomendação publicada; nenhuma frente iniciada e nenhuma página aceita por causa dela.

**Esta é a página canônica.** As páginas que medem frentes citam este quadro e não publicam uma segunda recomendação. O `foco-e-sequencia.md` continua dono da medição interna do `wyd-db` e das perguntas de seed; o `briefing-de-decisoes-1.0.0.md` continua dono do dossiê das decisões de sistema. Nenhum dos dois substitui a ordem e as escolhas registradas aqui.

## 0. A recomendação

**Foco aprovado em 2026-11-24 (D-01 = A): frente A — finalizar o `wyd-db`. A leva `migrations/` executou 28 `.sql` do zero, 13 cenários verdes e teve as 27 páginas aceitas (D-03 = A).**

O `wyd-db` tem **64** células pendentes (24 `Teste` + 40 `Issue`) em `rastreabilidade.md` §1, **26** das **39**
capacidades sem execução (`capacidades.md` §7) e **142** atos de aceite, todos em `docs/` (`pendencias` §1):
as **27** páginas de `migrations/` foram aceitas em 2026-11-24, e a pasta está com **28 de 28**. Já rodaram
**13** capacidades (`CAP-24`–`CAP-36`, `sistema_pg` §1.1) e a matriz está **13 de 39**.

Por que A: os **141** atos não esperam terceiro — são revisão e aceite; os **7** itens travados por terceiro
(**6** seeds e `CAP-08`) ficam fora deles, e as **2** `Q-BLK` (`20`, `22`) foram respondidas em A em 2026-11-24. B fecha
**0** bloqueio do piloto e libera **48** `RF` (`admin` 15 + `portal` 33) e **1** plano diretor; C fecha **3**
células e **0** das **9** linhas sem página; D fecha **1** página e **1** harness, ambos dentro do caminho de A
(§3). A pergunta de foco está em §0.1 (D-01).

**Recomendação única: manter A.** É a única frente que fecha item do piloto sem terceiro: **64** células
e **141** atos, com **40** issues a autorizar. O **ato 2** (`migrations/serie-e-regras.md`) e o **ato 3**
(runner `src/migrate` + `MigrateConfig`, com **D-02** respondida em B) foram **concluídos em 2026-11-25**
no escopo de lib/fakes; a próxima fatia é o **ato 4** (`CAP-37`–`CAP-39`, cobertura **13 → 16 de 39**),
depois o **ato 5** (**40** issues + **141** atos de `docs/`); o **ato 6** (seeds/captura) espera terceiro.
O **ato 4** que usa a série real depende da resposta **`Q-SERIE-1`** (política dos 28 `down`, recomendação
**B**). B não fecha item do piloto, C fecha **3 de 64** células e D já foi absorvida pelo ato 2 concluído (§3–§5).

Esta página é o **único lugar publicado** que ranqueia as frentes do projeto. Ela não substitui o plano
de fases (`wyd-1.0.0.md` §6), nem o registro de pendências (`pendencias-de-fechamento.md` §6.6), nem a
medição interna do piloto (`wyd-db/foco-e-sequencia.md`): ela usa os números deles e diz qual vem agora.

**Por que ela mora aqui:** o dono convencional do ranking é `wyd-1.0.0.md` §6.2, que **não é publicado**;
o `docs/` do `wyd-db` está no teto de **144** páginas; e `wyd-ops` é o sistema de processo, com `docs/`
publicado e sem teto. Não existe convenção para “página de ranking do projeto”, então esta é a opção mais
próxima já usada — e a única que fica navegável no site.

## 0.1 Quadro canônico de decisões (vigente em 2026-11-25)

Cada decisão traz o que está bloqueado hoje, as opções reais, a recomendação, o motivo medido e o que
deixa de ser possível na opção não escolhida. **D-01, D-03, D-04 e D-05 foram aprovadas em A e D-02 em B, em
2026-11-24 (D-04 em 2026-11-25)**; a execução de D-01/D-03 está em §0.2 e a política de D-02, em §0.1
e no plano de CI. **D-05 foi aprovada em A em 2026-11-25**; o dossiê está em
`../dossie-d-05.md` e o plano de migração, em `../plano-migracao-d-05.md`; a auditoria de D-04 está
em `../auditoria-de-organizacao.md`. **D-06 está ABERTA — aguardando decisão do usuário**
(§0.1, D-06); D-05 permanece aprovada e não é reaberta. **Ato 2 e ato 3 concluídos em 2026-11-25**
(`migrations/serie-e-regras.md` em `aceito` e runner em lib/fakes — 63 testes verdes; ver
`../../wyd-db/migrations/serie-e-regras.md` §8); a próxima fatia é o **ato 4** (`CAP-37`–`CAP-39`).
**`Q-SERIE-1` está ABERTA**: os 28 `.sql` só têm `up`, e a política de rollback precisa ser escolhida
antes do uso da série real (§0.1, D-06; opções e recomendação em `../../wyd-db/migrations/serie-e-regras.md` §9).

### D-01 — Qual frente recebe o foco?

**Aprovada em 2026-11-24 (A):** o foco é finalizar o `wyd-db`; a ordem dos atos está em §0.2.

**Bloqueado hoje:** **142** atos de aceite, **64** células (**24** `teste` + **40** `issue`), **40** issues
abertas (#82–#121) e **0** vinculadas às linhas e **26** capacidades sem execução (**13 de 39**); as **2**
`Q-BLK` (`20`, `22`) foram respondidas em A em 2026-11-24. A alternativa B tem **48** `RF` (15 `admin` + 33 `portal`), **10** sistemas sem
`RF`, **12** planos e **1** plano diretor ausente (`wyd-ops`); a C tem **3** células `teste`, **9** linhas
sem página e **4** sem `CAP-nn`.

| Opção | O que fecha/entrega | O que adia | Custo | Disponível hoje |
|---|---|---|---|---|
| **A — finalizar o `wyd-db`** | **64** células + **141** atos + **40** issues + **26** capacidades; **0** `Q-BLK` | as **48** `RF`, **12** planos e **1** diretor de B; as **3** células de C | 27 aceites (feitos) + `serie-e-regras` + runner + 3 cenários + 141 atos + 40 issues | **sim**: 0 item de A espera terceiro |
| **B — expandir os outros sistemas** | **48** `RF`, **12** planos e **1** plano diretor (`wyd-ops`); **0** bloqueio do piloto | **64** células, **26** capacidades e **40** issues de A; as **3** células de C | 10 sistemas a numerar + 12 planos + 1 diretor | parcial: 10 sistemas andam sem terceiro; `protocol` e `client` não |
| **C — fechar lacunas transversais** (rastreabilidade + regras) | **3** células `teste` (`RF-DB-003`, `006`, `014`) + 1 gate de cobertura; **1** página + **1** harness | **61** células, **26** capacidades e **40** issues de A; **48** `RF` e **12** planos de B | 3 `CAP` + 1 gate + 1 página + `tests/common/` | **sim**, mas o retorno é **3 de 64** células; **0** das 9 linhas sem página |

**Recomendação: A.** É a única frente cujos fechamentos estão disponíveis hoje e cujo produto é
verificável (schema, teste, DDL). B não fecha bloqueio do piloto e C fecha **3 de 64** células; as **9**
linhas sem página não são fecháveis por cenário (2 de outro sistema, 1 gate de CI e 6 gate de série).
Regras e padrões (**D** na leitura antiga) não são uma quarta opção: são o passo 2 de A.

**Se B for escolhida:** as **64** células, **26** capacidades e **40** issues de A ficam paradas; o DDL
de `portal`/`admin` segue proposta e a implementação deles continua esperando A. **Se C for escolhida:**
A fica com **61** células e **26** capacidades sem execução, e B continua sem o DDL aceito. **Se A:**
B pode numerar os **48** `RF` em paralelo, mas a implementação de `portal`/`admin` espera o DDL de A.

### D-02 — Política do gate de código: cobertura e segurança

**Aprovada em 2026-11-24 (B).**

**Bloqueado hoje:** o `code.yml` tem **19** testes (**6** unitários + **13** de schema) e **0** medição de
cobertura; não há `cargo audit` nem `cargo deny`. A matriz tem **24** das **64** células (tipo `teste`) pendentes e **26**
capacidades sem execução. O `AGENTS.md` §8 exige **85%** por sistema, medido no gate; o `full-workflow.yml`
segue stub e não mede nada.

**O que travava:** o **ato 3** (runner) esperava esta política; respondida em B, ele deixa de ter essa
dependência. O **ato 2** (`serie-e-regras`) nunca dependeu dela.

| Opção | Cobertura | Segurança | Efeito imediato |
|---|---|---|---|
| **A — bloquear 85% agora** | `--fail-under-lines 85` no `code.yml` desde já | `cargo audit` + `cargo deny` bloqueantes | pode reprovar PR por cobertura de linha do `dsn`, sem provar as **26** capacidades; força teste do que ainda não tem alvo |
| **B — 85% no gate do sistema; informativo no `code.yml`** | **bloqueante** quando o sistema declarar o gate fechado; **informativo** no incremental até lá | `cargo audit` + `cargo deny` **bloqueantes agora** | mantém o invariante de release sem transformar cobertura de linha em prova de capacidade; segurança entra já |
| **C — tudo informativo até a 1.0.0** | sem bloqueio até a release | sem bloqueio até a release | adia a dívida e deixa vulnerabilidade entrar; não recomendado |

**Recomendação: B.** O invariante é **85% por sistema no gate**, não “85% de qualquer linha no PR”;
os **19** testes atuais não cobrem **24** das **64** células pendentes, então um bloqueio agora mediria o `dsn`,
não a capacidade. `cargo audit`/`deny` são baratos hoje (o crate não tem dependência) e devem ser
obrigatórios; a dependência Python do site (`mkdocs==2.0.dev6`) fica fora do `cargo audit` e exige
`pip-audit`/hash em item separado.

**Se A:** PRs podem ficar vermelhos por uma métrica que não prova capacidade e o time pode ser levado a
escrever teste de linha em vez de teste de contrato. **Se C:** vulnerabilidade e regressão de cobertura
entram sem barreira. **B** preserva a barreira onde ela é exigível e não finge que o slice atual já é o
gate do sistema.

**O que a aprovação em B vale a partir de agora:** a cobertura de **85%** continua **bloqueante no gate do
sistema**, não no PR — no `code.yml` ela é **informativa** até o sistema declarar o gate fechado; `cargo audit`
e `cargo deny` são **bloqueantes**. **Passo 8a executado em 2026-11-24:** o job `security` do `code.yml`
roda os dois com `deny.toml`, o job `cargo` publica cobertura sem `--fail-under`, e a falha aparece como
job vermelho (advisory: exit 1; licença: exit 4; `Cargo.toml` sem lock: `::error::`). O `pip-audit`/hash do
site (`mkdocs==2.0.dev6`) continua item separado, e D-05 foi aprovada em A para toolchain e versões (dossiê em `../dossie-d-05.md`).

### D-03 — Escopo das 27 páginas de `migrations/` e das 2 `Q-BLK`

**Aprovada em 2026-11-24 (A), com `Q-BLK-20` = A e `Q-BLK-22` = A, e executada no mesmo dia.**

**Bloqueado hoje:** nada na assinatura das páginas — as **27** foram aceitas em 2026-11-24 e a pasta está
com **28 de 28** `aceito`. A leva tem **28** `.sql` aplicados e **13** cenários verdes. Restam **141** atos,
todos de `docs/`, **40** issues e **6** seeds; `Q-BLK-20` e `Q-BLK-22` foram respondidas em A.

| Opção | Migrations | `Q-BLK-20` (catálogo) | `Q-BLK-22` (teto) | Efeito |
|---|---|---|---|---|
| **A — aceitar as 27 agora** | **168 → 141** atos; runner liberado | **A**: catálogo pós-`1.0.0`; `CAP-08` declarada não executável | **A**: as 12 páginas de `tests/` não contam no teto de **144** | fecha a leva, libera os passos 2–4 e mantém os **6** seeds fora (terceiro) |
| **B — aceitar só depois do runner** | 27 atos e runner adiados | **B**: catálogo entra e **1** capacidade passa a segurar **28** `.sql` | **B**: teto vai a **156** | amarra o aceite à prova, mas o runner nasce sobre página `proposto` |
| **C — manter `proposto` até a release** | **168** atos permanecem | decisão adiada | decisão adiada | a leva não fecha e o runner nasce sob página não assinada |

**Recomendação: A**, com `Q-BLK-20 = A` e `Q-BLK-22 = A`. O que já foi provado é a série: **28** `.sql`
aplicaram do zero e **13** cenários rodaram; o que falta é a assinatura editorial, não uma nova prova. A
opção B inverte a ordem e transforma **1** capacidade (catálogo) em bloqueio de **28** `.sql`; a C mantém
os **168** atos e o runner sobre página não assinada. Os **6** seeds não entram em nenhuma das opções:
eles dependem de dado e continuam **6** até a curadoria.

**Se B:** os **27** atos e o runner esperam; `Q-BLK-20 = B` impede a leva de fechar sem o catálogo e
`Q-BLK-22 = B` muda o critério de release de **144** para **156**. **Se C:** os **168** atos e as **2**
decisões ficam em aberto no fechamento. **Se A:** o runner nasce sob página assinada e `CAP-08` fica
declarada não executável, sem esconder a lacuna.

### D-04 — Critério para declarar “organização definida” nos sistemas sem código

**Aprovada em 2026-11-25 (A).** A auditoria por sistema está em
`../auditoria-de-organizacao.md`: **2 de 13** sistemas foram declarados organizados (`wyd-db` e
`wyd-admin`) e **11** ainda não.

**Estado medido antes da aprovação:** a afirmação “todos os sistemas têm organização definida” não é verdadeira. São **13**
sistemas; **12** não têm código de produto (o `wyd-docs` tem implementação Python de documentação), **8**
dos **9** diretórios de crate estão vazios, **10** sistemas não têm `RF` numerado e **1** (`wyd-ops`) não
tem plano diretor. Só há **2** manifestos de dependência reais (`crates/wyd-db/Cargo.toml` e
`wyd-docs/requirements.txt`; `deny.toml` é política).

**O que travava:** a declaração dos **12** sistemas, não a sequência de A (§0.2).

| Opção | O que conta como “organizado” | O que permite afirmar | O que deixa de fora |
|---|---|---|---|
| **A — planejamento completo, sem código** | fronteira + contratos com dono + árvore `src/`, `tests/` e `ops/` + requisitos com ID + plano de teste + roadmap com DoD + dashboard com gate | **12** sistemas podem ser auditados e declarados **um a um**; código continua proibido antes do aceite | sistemas que não passarem no checklist; hoje **10** sem `RF` e **1** sem plano |
| **B — código mínimo** | só conta quando houver crate/manifesto, build e teste | apenas `wyd-db` (e o site) hoje; **12** esperam | contradiz “planejar antes de codar” e adia a declaração de **12** sistemas |
| **C — abertura atual basta** | fronteira + contratos + lacunas nomeadas | qualquer sistema aberto vira “organizado” | ignora **10** sem `RF`, **1** sem plano e **8** crates vazios; a afirmação fica falsa |

**Recomendação: A.** “Organização definida” é um estado de **planejamento verificável**, não de código;
mas não é o estado “aberto”. Um sistema só recebe o carimbo depois de mostrar a árvore de código/testes/ops,
os requisitos, o plano de teste, o roadmap com DoD e o gate. **Implementável** é um estado posterior:
páginas aceitas + manifesto + gate de código. A recomendação não declara os **12** de uma vez: ela obriga a
auditar cada um e a aceitar a ausência justificada de `ops/` quando não houver operação própria. O
checklist e a árvore alvo estão em `wyd-1.0.0.md` §8.7 (aprovada em 2026-11-25); D-04 = A aprovada é
o que os torna exigíveis. Na medição de 2026-11-24, **1** crate tem código, **8** crates têm só
`.gitkeep`, **10** sistemas não têm `RF` e `wyd-ops` não tem plano diretor: a afirmação “todos
organizados” ainda era **falsa**.

**Se B:** os **12** sistemas sem código ficam sem declaração até existir crate, o que inverte a ordem do
projeto. **Se C:** “organizado” passa a significar “aberto” e os **10** sem `RF`/`1` sem plano ficam
escondidos. **Se A:** a declaração é seletiva e verificável; nenhum sistema ganha o carimbo por simetria.

**Resultado da auditoria (2026-11-25, segunda passada):** **13 de 13** sistemas passam no checklist de
organização; `wyd-ops` mantém fronteira/contratos O1–O5 pendentes de aprovação. O que falta agora não é
organização: é aceite editorial, captura, medição, implementação ou decisão humana. A lista por sistema
está em `../auditoria-de-organizacao.md` §3 e as perguntas consolidadas em `lacunas.md` §4.

### D-05 — Política de dependências e versões

**Aprovada em 2026-11-25 (A).** Adotar o workspace Cargo único na raiz, com `rust-toolchain.toml`,
`Makefile`, **um** `Cargo.lock`, ações por SHA e imagens por digest. O dossiê completo está em
`../dossie-d-05.md`; a execução, com passos, DoD e rollback, está em `../plano-migracao-d-05.md` e
**ainda não foi executada**.

**O que destrava:** o segundo crate Rust sem repetir lock/toolchain. O passo 2 do `wyd-db` e o gate
de documentação **não** esperam D-05. **O que D-05 não fecha:** as **6** lacunas abertas + **1** proposta,
que dependem de captura, direitos ou medição.

### D-06 — Qual frente executa primeiro depois de D-05?

**ABERTA — aguardando decisão do usuário (2026-11-25).** D-05 está aprovada com plano de migração
pronto, mas ainda não executado. D-06 escolhe a **ordem de execução**; não reabre D-05, não aceita
página e não autoriza código. O **ato 2** e o **ato 3** de A já foram concluídos no escopo de lib/fakes;
**`Q-SERIE-1`** é decisão interna de A e está detalhada em `../../wyd-db/migrations/serie-e-regras.md` §9.

| Opção | O que destrava/fecha hoje | Depende de | O que fica parado sem ela |
|---|---|---|---|
| **A — D-01 / `wyd-db`** — runner concluído em lib/fakes → ato 4 `CAP-37`–`CAP-39` → 40 issues + 141 atos | **141 atos** (141 `docs/`), **64 células** (24 `teste` + 40 `issue`) e **40 issues** (#82–#121) (§2 e §3); ato 2/3 **concluídos**; `CAP-37`–`CAP-39` = **3 células `teste`**; passo 5 = **40 issues + 141 atos** (§0.2) | Ato 4 não depende de terceiro, mas a série real depende de **`Q-SERIE-1`**; **6 seeds** e `CAP-08` ficam para o passo 6, por dado externo | 141 atos, 64 células e 40 issues continuam parados; `portal`/`admin`/`server` seguem sem o DDL aceito |
| **B — Executar a migração D-05** | **9 crates** (1 real + 8 `.gitkeep`), **6 comandos `--manifest-path`** e **2 imagens externas/6 referências sem digest**; **0 atos, 0 issues, 0 células**; destrava o segundo crate e um único lock/toolchain (`dossie-d-05.md` §2; `plano-migracao-d-05.md`) | Nenhuma externa; D-05 já aprovada | Os 8 crates vazios e o segundo crate ficam sem padrão; o CI continua repetindo o manifesto do `wyd-db` |
| **C — A1 / `wyd-protocol`** — captura, peer e segunda leitura | **6 `RF` + 6 `RNF`**, árvore, teste e roadmap na segunda passada; **2 lacunas de captura** (`LPRT-03`, `LPRT-06`); **0 atos/issues** | A1 autorizada em 2026-11-23; captura externa pendente; código também espera D-05 e página `aceita` | O codec e o fio continuam sem captura; `net`, `core` e `bots` não começam a integração |
| **D — `wyd-server`/`wyd-net`** — esqueleto de ponta a ponta e medição | **7 `RF` + 6 `RNF`** no server, **8 `RF` + 6 `RNF`** no net; árvore, teste e roadmap na segunda passada; **`LNET-01` como medição** | A1 e a fatia do server; `LNET-01` depende de CI efêmero | Sem o esqueleto não há E2E; sem medição os limites continuam sem valor |

**Recomendação: A — D-01 / `wyd-db`.** É a única opção que fecha itens já mapeados **hoje**: 141 atos,
64 células e 40 issues; B, C e D fecham **0** atos/issues no curto prazo. D-05 já está aprovada e o
plano pode esperar a vez sem perder a decisão; C depende de captura externa; D depende de A1 e de CI
efêmero.

**doneWhen do caminho A:**

1. `migrations/serie-e-regras.md` em `aceito` — **1 ato** — **concluído em 2026-11-25**.
2. Runner `src/migrate` + `MigrateConfig` com o alvo `contrato_migrations` verde — **1 alvo** —
   **concluído em lib/fakes** (63 testes; o PG17 de sistema ainda não rodou).
3. **`Q-SERIE-1`** respondida: política de `down` dos 28 arquivos, recomendação **B**; sem ela a série
   real e `CAP-37` não rodam.
4. `CAP-37`–`CAP-39` verdes — **3 células `teste`**, de 13 para 16 de 39.
5. **40 issues** vinculadas e **141 atos** de `docs/` aceitos; gate do `wyd-db` verde. Os **6 seeds** e
   `CAP-08` continuam fora por dado/terceiro.

**Pergunta ao usuário:** **A** (D-01/`wyd-db`), **B** (executar D-05), **C** (`wyd-portal`) ou **D**
(`wyd-protocol`)? A recomendação é **A**, pelo volume de 141 atos/64 células contra 0 nas demais.
**Decisão interna que acompanha A:** **`Q-SERIE-1`**, com opções e recomendação **B** em
`../../wyd-db/migrations/serie-e-regras.md` §9.

## 0.2 Ordem recomendada e impacto em bloqueios

A ordem abaixo é a de **execução**, não a de implementação: nenhum passo de código é executado neste
turno. Os passos 1–3 estão concluídos (o 3 em lib/fakes); os passos 4–5 são a frente A; o passo 6 é terceiro.

| # | Passo | Resolve/fecha | Desbloqueia | Depende de |
|---:|---|---|---|---|
| **0** | Responder D-01–D-06 e `Q-SERIE-1` | **5** decisões (D-01, D-02, D-03, D-04, D-05), **1 aberta (D-06)** e **1 política aberta (`Q-SERIE-1`)** | **40** issues, **12** sistemas e o gate de código; a auditoria de D-04 saiu e os **27** aceites de `migrations/` já saíram | dono do produto + `wyd-ops`/arquitetura |
| **1** | Aceitar as **27** páginas de `migrations/` | **feito em 2026-11-24** (D-03 = A): **27** atos, **168 → 141** | runner (passos 2–4); leva de `migrations/` fechada | — (concluído em 2026-11-24) |
| **2** | `migrations/serie-e-regras.md` | **feito em 2026-11-25**: **1** página/ato `aceito` | contrato do ledger para o runner | passo 1 (concluído) |
| **3** | Runner `src/migrate` + `MigrateConfig` | **feito em 2026-11-25** em lib/fakes: **1** alvo (`contrato_migrations`), 63 testes | **3** `CAP` e o gate de código do banco (PG17 ainda não) | passos 1–2; D-02 = B (2026-11-24) |
| **4** | `CAP-37`–`CAP-39` | **3** capacidades / **3** células `teste`; **13 → 16 de 39** | **3** células; restam **21** `teste` para os outros alvos | passo 3; **`Q-SERIE-1`** para a série real |
| **5** | **40** issues + **141** atos de `docs/` | **40** células `issue` vinculadas + **141** atos; **21** `teste` seguem para código | **40** issues autorizadas; fecha os **141** atos restantes | passos 1 e 4; `Q-SERIE-1` não bloqueia o aceite |
| **6** | Seeds/captura | **6** seeds + **6** lacunas abertas + **1** proposta | **0** hoje; só quando o terceiro entregar | terceiro/dado |

**Por que o passo 1 vem antes do passo 5, que fecha mais itens:** o passo 5 depende de o DDL estar
assinado e o runner existir; o passo 1 é o primeiro ato de **27** itens que não espera código; os **141** atos de `docs/` são a maior
carga de revisão e entram depois para não abrir 141 páginas antes do runner. A ordem por
“resolve mais bloqueios primeiro” é: **1** (27 atos, custo de aceite), **5** (141 atos + 40 issues, custo
de revisão), **4** (3 células, custo de código), **2–3** (1 página + 1 alvo, habilitam o runner). O passo
0 não fecha item, mas sem ele nenhum dos outros começa. **Os passos 1–3 estão feitos; o próximo é o 4**
(`CAP-37`–`CAP-39`), condicionado a **`Q-SERIE-1`** na parte que usa a série real.

**O que a ordem não resolve:** os **6** seeds, as **6** lacunas abertas e a **1** proposta não entram em
nenhum passo porque dependem de terceiro; `CAP-08` já está declarada não executável (`Q-BLK-20` = A), mas
segue **0 coberta**.

## 0.3 Sequência depois de D-05

D-04 e D-05 estão aprovadas; **D-06 está aberta** e é o topo da fila. A recomendação é **A: D-01 /
`wyd-db`**, porque é a única candidata que fecha itens já mapeados hoje. Cada linha abaixo traz o
pré-requisito e o que **não** resolve.

| Ordem | Frente | Pré-requisito que a destrava | O que resolve | O que não resolve |
|---:|---|---|---|---|
| **0** | **D-06** — escolher A/B/C/D | resposta do dono; dossiê §0.1 | **A:** 141 atos/64 células/40 issues; **B:** 9 crates/6 manifestos; **C:** 33 `RF`/10 `RNF`; **D:** 5/131 páginas | não executa nenhuma frente por si só |
| **1** | **D-01 / `wyd-db`** — ato 2/3 concluídos → ato 4 `CAP-37`–`CAP-39` → 40 issues + 141 atos | **D-06 = A**; ato 2/3 já feitos; **`Q-SERIE-1`** para a série real | 3 `CAP`, 40 issues e 141 atos ao fim | 6 seeds e `CAP-08` dependem de dado/terceiro; parte de `CAP-37` depende de `Q-SERIE-1` |
| **2** | **D-05 = A** — executar o plano de migração | **D-06 = B**; D-05 já aprovada | workspace, 1 lock, toolchain, Makefile, 6 comandos e digests; segundo crate destravado | **0** atos/issues; não fecha lacunas de captura |
| **3** | **D-04 / `wyd-portal`** — árvore `src/`/`tests/` + plano de teste | **D-06 = C**; D-04 aprovada, sem D-05 | fecha o checklist do portal; 33 `RF`/10 `RNF` já existem | **0** atos/issues; não implementa o portal |
| **4** | **D-04 / `wyd-protocol`** — `RF`, árvore, teste e roadmap | **D-06 = D**; captura `LPRT-03`/`LPRT-06`; código também espera D-05 e página `aceita` | planejamento do codec; depois destrava `net`/`core` | 5/131 páginas; 2 lacunas de captura |
| **5** | **runtime `net`/`core`/`server`** | contratos do protocol + D-05 = A + páginas `aceitas` | implementação em cadeia | captura e dados externos |
| **6** | **`sim`/`launcher`/`client`/`bots`** | decisões próprias + D-05 quando Rust + captura | fórmulas, assinatura, direitos e carga | lacunas de terceiro |
| **7** | **`docs`/`ops`** | depois do runtime; D-04 para o carimbo | `RF-DOC`/`RF-OPS` e gates | não bloqueia a onda jogável |

**Regra de leitura:** D-05 está aprovada, mas é a opção **B** de D-06; o plano pode esperar sem perder
a decisão. **O ato 4 do `wyd-db` é a próxima execução se D-06 = A** (recomendado), depois dos atos 2–3
concluídos; a parte de série real espera **`Q-SERIE-1`**. O `wyd-portal` é a organização mais barata e
não depende de D-05, porque é Next.js. As **6 lacunas abertas + 1 proposta** ficam no passo 6 do
`wyd-db`/captura, não em D-06.

## 0.4 Ordem recomendada das decisões abertas

A ordem abaixo é a de **resposta do dono**, não a de execução; as opções e consequências estão na tabela
§7 e nas notas indicadas, sem exigir abrir outro documento para decidir.

| Ordem | Decisão | Recomendação | O que libera | Por que nesta posição |
|---:|---|---|---|---|
| **1** | **`D-06`** — frente depois de D-05 | **A** D-01/`wyd-db` | **141 atos**, **64 células** e **40 issues**; o DDL deixa de ser proposta para `portal`/`admin`/`server` | É a única opção que fecha itens já mapeados sem terceiro; B, C e D fecham **0 atos/issues** no curto prazo, então as demais decisões só fazem sentido depois desta. |
| **2** | **`Q-SERIE-1`** — política dos 28 arquivos sem `down` | **B** escrever os 28 `down` reais | **28** migrations com rollback, **`CAP-37`**, reset, `down --to` e os testes de sistema | Sem ela, A para no runner de lib/fakes e não aplica a série existente; é o maior desbloqueio técnico interno de A. |
| **3** | **`Q-SEP-1`** — separador `-` × `_` | **A** ler os dois e usar `_` nos novos | **1** regra de nomenclatura; preserva os **28** nomes e checksums atuais | Não fecha ato nem issue, mas evita **28** renomeações e novos checksums; deve ser fixada antes de qualquer arquivo novo e não muda a ordem da série. |

**Leitura:** responda **D-06 = A** primeiro; se A for a escolha, responda **Q-SERIE-1 = B** e **Q-SEP-1 = A**
antes de retomar o ato 4. As três respostas são independentes entre si, mas a ordem evita escrever `down`
antes de decidir se os nomes serão renomeados.

## 1. O critério, declarado antes do número

O dono nomeou três perguntas: **quantos itens bloqueantes e lacunas a frente fecha**, **quantos
planejamentos de outras frentes ela desbloqueia** e **por unidade de trabalho**. Há uma quarta coluna que
o critério exige quando existe terceiro na cadeia: **o que está disponível hoje**. Fechar sete lacunas que
dependem de captura externa vale **zero** enquanto a captura não chega — contar a intenção como fechamento
seria inflar o ranking.

## 2. O que acabou de existir, medido em 2026-11-24

| Medida | Número | Fonte |
|---|---:|---|
| páginas de `tests/` escritas | **12 de 12** `aceito` (2026-11-24) | `find tests -name '*.md'` + `grep -l` de `**Estado:** aceito` |
| capacidades na matriz `CAP-nn` | **39** | `tests/specs/capacidades.md` §7 |
| capacidades com página e alvo escritos | **39** | coluna "Onde é provada" da matriz |
| capacidades cobertas por teste executado | **13 de 39** | `tests/specs/capacidades.md` §7; `crates/wyd-db/tests/sistema_pg.rs` (2026-11-24) |
| capacidades sem requisito numerado (só gate) | **0** | idem §7 |
| capacidade sem ambiente hoje | **1** (`CAP-08`, declarada não executável na `1.0.0` em 2026-11-24) | `tests/alvos/contrato-catalogo.md` §3 |
| linhas da matriz de rastreabilidade com página que **nomeia** o requisito | **31 de 40** (29 em `sistema-pg`, 1 em `sistema-sessoes`, 1 em `sistema-stack-caps`); **9** sem página e **4** sem `CAP-nn` na coluna `Teste` (`RF-DB-003`, `006`, `014` e `RNF-DB-001`); **3** células são `—` por regra | `docs/specs/rastreabilidade.md` §1, §1.1 e §3.4 |
| páginas de `tests/` assinadas pelo dono | **12 de 12** `aceito (2026-11-24)`; **0** `proposto` | `wyd-db/revisao-de-q-blk-21.md` §5 |
| páginas de `migrations/` `aceito` | **28 de 28** (2026-11-24: 27 na leva, `0002` desde 2026-11-20) | `grep -l '^\*\*Estado:\*\* aceito' planejamento-1.0.0/wyd-db/migrations/*.md`; `pendencias` §6.6.31 |
| atos de aceite pendentes no `wyd-db` | **142** (142 `docs/` + **0** `src/` + **0** `migrations/` + **0** `tests/`) | `pendencias-de-fechamento.md` §6.6.22.1, §6.6.27 e §6.6.31 |

## 3. As quatro frentes, com número e fonte

**As três perguntas do critério, uma coluna cada.** "Bloqueios" = item que hoje impede outro de andar;
"lacunas" = célula, ato ou página que sai de pendente para fechado; "planejamentos" = trabalho de outra
frente que deixa de esperar. Cada número cita a fonte; nenhum é estimado.

| Frente | Bloqueios destravados | Lacunas fechadas | Planejamentos liberados | Disponível hoje | Custo | Fonte |
|---|---|---|---|---|---|---|
| **A — Finalizar o `wyd-db`** | **26** capacidades sem execução (**13 de 39** cobertas) + **40** issues abertas (#82–#121) e **0** vinculadas às linhas + **0** `Q-BLK` abertas | **64** células (24 `Teste` + 40 `Issue`) e **141** atos (141 `docs/`); **0** lacuna aberta no sistema | o DDL deixa de ser proposta para `portal`/`admin`/`server` (contrato C4); as **40** issues viram trabalho autorizado; **12** planos de B passam a citar tabela aceita | **sim** — os 141 atos não esperam terceiro | 27 aceites (feitos) + runner + 3 cenários + 40 issues + 141 atos | `rastreabilidade.md` §1; `capacidades.md` §7; `sistema-pg.md` §1.1; `pendencias` §1 e §6.6.31 |
| **B — Expandir os outros sistemas** | **0** do piloto; **48** `RF` a numerar (`admin` 15 + `portal` 33) e **10** dos 12 sistemas sem `RF` | **0** do piloto — as **7** lacunas do projeto são de captura, não de redação | **12** planos, **1** plano diretor (`wyd-ops`) e **51** páginas publicadas de base | **parcial**: 10 sistemas andam sem terceiro; `protocol` e `client` não | 51 páginas + 10 sistemas + 1 plano diretor | §3 desta página; `grep -rhoE 'RF-[A-Z]+-[0-9]{3}'` por sistema; `lacunas.md` de cada sistema |
| **C — Fechar a rastreabilidade** (as **4** linhas sem `CAP-nn` e as **9** sem página) | **3** células `Teste` (`RF-DB-003`, `006` e `014`, as três com página e sem `CAP-nn`); `RNF-DB-001` depende do gate de cobertura, não de cenário | **3 de 64** células; **0 das 9** linhas sem página (**2** são de `wyd-server`, **1** é gate de CI e **6** são gate de série) e **3** células `—` são regra | **0** — nenhum outro sistema depende de `CAP-nn` do banco | **sim** — 3 cenários; as páginas de `tests/` ficam fora do teto de 144 | 3 `CAP` + 3 cenários + 1 gate de cobertura | `rastreabilidade.md` §1 e §3.4; `capacidades.md` §7.1; `tests/index.md` |
| **D — Definir regras e padrões pendentes** | **1** página (`migrations/serie-e-regras.md`, pré-requisito do ledger do runner) + **1** harness (`tests/common/`) para os **4** alvos restantes | **1** página; `planos/metricas.md` **não** fecha (bloqueada pela série) e os **3** `RNF` seguem sem limiar por decisão (`Q-BLK-24` = A), então **0** limiar novo | **0** fora do `wyd-db`; dentro, é a etapa 0 do runner de A | **sim** — 1 página + 1 harness | 1 página + `tests/common/` | `pendencias` §1 (seções de `migrations/` e `metricas`); `tests/index.md`; §6.6.29 (`M20`) |

**O que o quadro diz:** A é a única frente que fecha mais de uma dezena de itens por ato e não espera
terceiro. **C** parece a mais barata, mas fecha **3 de 64** células e **0** das **9** páginas — as 9 não são
fecháveis por cenário: 2 são de outro sistema, 1 é gate e 6 são gate de série. **D** não é frente paralela:
1 página e 1 harness, ambos dentro do caminho de A; sozinha, não move capacidade nem libera outro sistema.

## 3.1 O que trava: decisão, terceiro/dado ou só execução

A separação muda o ranking: um item parado por falta de decisão anda com uma resposta; um item parado por
terceiro só anda quando o terceiro entrega; o resto é execução.

| Frente | Abertos (medido) | Travado por **decisão** | Travado por **terceiro/dado** | **Só execução** |
|---|---:|---:|---:|---:|
| **A — `wyd-db`** | **141** atos + **6** seeds + **1** capacidade + **0** `Q-BLK` | **0** — as 2 `Q-BLK` (`20`, `22`) foram respondidas em 2026-11-24 | **7**: **6** seeds (`0016`, `0017`, `0018`, `0020`, `0023`, `0027`) e `CAP-08` (catálogo ausente) | **141** (141 `docs/`) |
| **B — outros sistemas** | **12** planos, **51** páginas de base, **48** `RF` numerados | **0** | **0** — planejar não depende de ninguém | **51** páginas de base + **10** sistemas a numerar + **1** plano diretor (`wyd-ops`) |
| **C — rastreabilidade** | **3** células `Teste` + **1** gate de cobertura | **0** | **0** — nada externo depende de `CAP-nn` do banco | **3** cenários + 1 gate |
| **D — regras e padrões** | **1** página + **1** harness | **0** | **0** — é etapa 0 de A | **2** itens |

Fontes: `pendencias` §6.6.22.1 (141 atos), §6.6.31 (as 2 `Q-BLK`), §6.6.23 e §6.6.28, §6.5.1 (`Q-FOCO-1` = D);
`migrations-checklist.md` §2 (34 linhas: 28 `concluída` + 6 `bloqueada`); `tests/specs/capacidades.md` §3 e §7;
`lacunas.md` §1 de `wyd-protocol`, `wyd-client`, `wyd-net` e `wyd-bots`; `grep -rhoE 'RF-[A-Z]+-[0-9]{3}'`
por sistema.

**O que a tabela diz:** os **141** atos do `wyd-db` não esperam terceiro — esperam revisão e aceite; o que
falta é a revisão das **141** páginas de `docs/`, porque as 27 de `migrations/` e as **2** `Q-BLK` foram
fechadas em 2026-11-24. B tem trabalho, mas não fecha item do
piloto; C tem 3 células executáveis e 7 itens (6 seeds + `CAP-08`) intocáveis; D é pré-requisito de A, não
alternativa.

## 3.2 A cadeia: o que libera o quê

| Passo | Libera | Não libera |
|---|---|---|
| **A, leva 1** — aceitar as 12 de `tests/` | a leva de `migrations/` (27 atos) e as **28** `.sql` de DDL | capacidade coberta pela leva de aceite (só a execução cobre) e código (esperava `Q-BLK-18`, respondida em A) |
| **A, leva 2** — aceitar as 37 de `src/` | o código do crate sob o invariante §5.1: as páginas que descrevem `dsn` e os módulos já estão `aceito` | implementação, teste executado e capacidade coberta |
| **A, leva `migrations/`** — **27 páginas aceitas em 2026-11-24** | as **40** issues como trabalho autorizado e o DDL que `portal` e `admin` já citam; o runner pode nascer sob página assinada; a matriz fica **13 de 39** e o alvo **1 de 5** | as **6** seeds, que esperam curadoria de dado |
| **A, etapa 0 (regras)** | `migrations/serie-e-regras.md` e o `tests/common/` que os **4** alvos restantes usam | capacidade: o runner ainda não existe, então a página sozinha não muda `13 de 39` |
| **B, planejar** | requisitos numerados nos **10** sistemas que não têm `RF`; o plano diretor de `wyd-ops` | nada do `wyd-db`; e a **implementação** de `portal`/`admin` espera o DDL aceito de A |
| **C, rastreabilidade** | **3** células `Teste` (`RF-DB-003`, `006`, `014`); `RNF-DB-001` espera o gate de cobertura | nada do `wyd-db`: as **9** linhas sem página não viram página (2 de outro sistema, 1 gate de CI, 6 gate de série) |
| **Captura (`A1`)** | as **2** lacunas de `protocol`, a de `net`, a de `bots` e o código de `protocol`/`server` | nada do `wyd-db`: página de teste não depende de captura |

**A dependência que decide o ranking:** A é a única ponta que não espera terceiro — as duas respostas
(`Q-BLK-20` e `22`) foram dadas pelo dono em 2026-11-24 e não travavam os atos 1–4. B depende do DDL aceito de A para implementar;
a captura depende de `A1`; C não depende de nenhum dos dois. A ordem não é “A é maior”: é “A é a única que anda
hoje”.

## 4. O ranking

| Pos. | Frente | Itens fechados | Desbloqueio | Ganho por ato | Por que nesta posição |
|---:|---|---:|---|---:|---|
| **1** | **A — Finalizar o `wyd-db`** | **64** células (24 `teste` + 40 `issue`) + **141** atos | série, `metricas`, crate, 40 issues | **1** aceite por ato; **1 de 5** alvos já executado | é a única frente cujos fechamentos estão **disponíveis hoje**, e a única que libera artefato executável (teste e migration são código, e código espera página `aceita`) |
| **2** | **B — Expandir os outros sistemas** | **0** do piloto; **48** `RF` e **12** planos | 48 `RF`, 12 planos, 1 plano diretor | ~0 por ato do piloto (51 páginas de base) | não fecha bloqueio nenhum e não é pré-requisito de nada do piloto; anda em paralelo, não antes |
| **3** | **C — Fechar a rastreabilidade** | **3 de 64** células | 0 outra frente | 1 célula por `CAP` + cenário | fecha pouco e não toca as **9** linhas sem página; o resto depende de gate ou de outro dono |
| **4** | **D — Definir regras e padrões** | **1** página + **1** harness | 0 outra frente | 1 página + harness | é etapa 0 de A: executada sozinha, deixa A parado no mesmo lugar |

**A composição do item 1, sem dupla contagem:** **64** = **24** células `teste` (as que seguem em `P`) +
**40** células `issue`; o censo tem dono em `rastreabilidade.md` §1. As 37 células `teste` da matriz não
entram inteiras porque **13** já estão em `V` — executadas em 2026-11-24 —, e `V` não é pendência. As **40**
issues do roadmap não entram como item à parte: é na
célula `issue` que a alocação delas se registra, e nenhuma das 40 tem número (`rastreabilidade.md` §3.3). Os **142**
atos de aceite são contados em `pendencias-de-fechamento.md` §6.6.22.1 e não entram nas células.

**A leitura em uma frase:** A ganha por ser o único caminho que converte trabalho em artefato verificável
hoje; C fecha 3 células e para; B abre planejamento alheio sem fechar nada do piloto; D é etapa de A.

## 4.1 O que cada frente não escolhida ganha em seguida

- **B ganha com A:** o DDL deixa de ser proposta e os contratos de `portal`/`admin` passam a citar tabela
  aceita; e B pode numerar os **48** `RF` e os **10** sistemas sem `RF` **em paralelo**, sem competir por atenção.
- **C ganha com A:** **3** células de `Teste` (as três com página e sem `CAP-nn`) podem ser escritas a
  qualquer momento; A não as fecha, mas também não as bloqueia.
- **D ganha com A:** o ato 2 de A (`serie-e-regras`) já foi concluído; restam o `tests/common/` dos **4** alvos e a integração do codec depois dos contratos.
- **B, C e D ganham com o tempo:** cada ato de A fecha de **1** a **27** itens sem depender de ninguém, o que
  compra o intervalo em que `A1` e a curadoria dos seeds podem chegar.

## 5. A trajetória recomendada

**Leva 1 — aceitar as 12 páginas de `tests/` — concluída em 2026-11-24 (`Q-BLK-21` = A).** O que ficou:

1. as **12** páginas estão `**Estado:** aceito (2026-11-24)`, com os **11** itens de §10 conferidos — a
   conferência está em `pendencias` §6.6.25 e as **5** correções de redação, em
   `revisao-de-q-blk-21.md` §4;
2. a matriz continuava em **0 de 39** naquele momento — aceitar página não cobre capacidade; hoje são **13 de 39** (§2);
3. `docs/specs/rastreabilidade.md` tinha as **40** linhas com `P`: **31** com página que nomeia o
   requisito e **9** sem página (§3.4 da matriz); depois da leva `migrations/`, **13** das **37** células
   `Teste` estão em `V` e **24** seguem em `P` (§1 da matriz);
4. **nenhum** arquivo `.rs` nasceu;
5. o aceite **não** cobre o `wyd-db`: os **16** cenários foram escritos em 2026-11-24 com `Q-BLK-23` = A
   (`pendencias` §6.6.29), e restam **4** linhas sem `CAP-nn` na coluna `Teste` (`RF-DB-003`, `006`, `014` e
   `RNF-DB-001`) — a matriz está em **13 de 39** cobertas.

**Leva `migrations/` — executada em 2026-11-24.** Os **28** arquivos de `migrations/` aplicaram do zero
num `postgres:17` vazio, como `wyd_migrator`, em ordem e sem erro; `crates/wyd-db/tests/sistema_pg.rs`
rodou **13** cenários verdes contra essa base (`CAP-24`–`CAP-36`) e a matriz passou a **13 de 39**. As **27**
páginas de `migrations/` seguiam `proposto` naquele momento; foram aceitas em 2026-11-24 (`pendencias`
§6.6.31), porque a série já nascida não substitui o aceite (`§6.6.30`).

**A sequência dentro de A — cada ato com o critério que o fecha:**

| Ato | O que produz | Critério observável de conclusão | Custo |
|---|---|---|---|
| **1. Aceite das 27 páginas de `migrations/`** — **feito em 2026-11-24** (D-03 = A, `Q-FOCO-4`) | 27 páginas `aceito`; pasta **28 de 28**; 168 → 141 atos | `grep -rl '^\*\*Estado:\*\* aceito' planejamento-1.0.0/wyd-db/migrations/*.md \| wc -l` = **28** e `EXIT_GATE=0` — conferido | 27 aceites |
| **2. `migrations/serie-e-regras.md`** | a regra da série: ordem, ledger `schema_migrations`, idempotência e drift | página com cabeçalho de 5 linhas, gate verde, e o contrato do ledger nomeado igual em `src/migrate/spec.md` | 1 página |
| **3. Runner `src/migrate` + `MigrateConfig`** | aplicar, reaplicar e detectar drift com ledger e advisory lock | `cargo test -p wyd-db --test contrato_migrations` verde; reaplicar na base migrada = **0** objeto novo; drift com código de saída ≠ 0 | código + 1 alvo |
| **4. `CAP-37`–`CAP-39`** | espera de DDL, p95 da leitura e boot medidos | `cargo test --test sistema_pg` inclui os **3** e passa; cobertura **13 → 16 de 39** se os três passarem | 3 cenários |
| **5. 40 issues e 141 atos** | issues autorizadas e o resto do aceite | `CAP-08` já declarada **não executável** em 2026-11-24; **40** issues existentes vinculadas às linhas e **141** páginas a aceitar | 40 issues + 141 atos |

**O ato 1 foi concluído em 2026-11-24 (D-03 = A):** as 27 páginas estão `aceito` e o runner pode nascer
sob página assinada. **Os atos 2 e 3 também estão concluídos em 2026-11-25** — a nota `migrations/serie-e-regras.md`
está `aceito` e o runner passou 63 testes de lib/fakes; a verificação PG17 ainda não rodou. O próximo é o
**ato 4** (`CAP-37`–`CAP-39`), condicionado a **`Q-SERIE-1`** na parte de série real; o **ato 5** depende dos
atos 1 e 4.

**O que fica postergado, e por quê:** **B** (48 `RF` + 12 planos) espera A porque a implementação de
`portal`/`admin` cita o DDL que A aceita; **C** (3 células) espera A porque o ganho é 3 de 64 e não move
nenhuma outra frente; **D** não é postergado — o ato 2 de A já foi concluído e restam os contratos/testes de D; as **6** seeds e a captura `A1` esperam
terceiro/dado e não entram na fila.

## 6. O que não entra agora

- **Código novo**: nada além da série `migrations/` (**28** `.sql` aplicados em 2026-11-24) e do alvo
  `sistema_pg` (**13** cenários); runner, pool, porta C4 e boot não entram nesta leva.
- **GitHub**: nenhuma issue, label, board, commit ou push — escrita exige pedido explícito.
- **Outros sistemas**: nenhuma página nova em `wyd-protocol`, `wyd-net`, `wyd-core`, `wyd-server`,
  `wyd-sim`, `wyd-admin`, `wyd-portal`, `wyd-launcher`, `wyd-client`, `wyd-bots`.
- **Lacunas de captura**: ficam como estão até a medição do cliente 7662 chegar.
- **Seeds**: os **6** bloqueados continuam bloqueados por dado, não por decisão.
- **Páginas novas no `wyd-db`**: o teto de **144** está fechado (143 escritas + `metricas`); esta leva não
  abre exceção.
- **Promoção de qualquer outra página a `aceito`**: o aceite desta leva não autoriza as demais.
- **A origem dos “59”** de `pendencias-de-fechamento.md` §6.6.16 continua **não localizada** após a
  varredura ampliada de 2026-11-24 (árvore viva, `deprecated/`, `.build/`, não-`.md`, histórico): fica
  registrada nesse estado, não bloqueia frente nenhuma e a busca não se reabre.

## 7. As decisões do dono

O dossiê de cada pergunta já respondida está em `planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md`
§6.6.22.5 (`Q-BLK-17`), §6.6.23 (`Q-BLK-21`), §6.6.26 (`Q-BLK-24`), §6.6.28 (`Q-BLK-19`), §6.6.29
(`Q-BLK-23`) e §6.6.31 (`Q-BLK-20`, `Q-BLK-22`, D-01 e D-03). **D-05 foi aprovada em A em
2026-11-25**; o dossiê está em `../dossie-d-05.md` e o plano de migração, em
`../plano-migracao-d-05.md`. **D-06 está ABERTA — aguardando decisão do usuário**; ela escolhe a
próxima frente e está detalhada em §0.1. **`Q-SERIE-1` está ABERTA** e é decisão interna de A (política
dos 28 `down`, recomendação **B**), detalhada em `../../wyd-db/migrations/serie-e-regras.md` §9. D-04 foi aprovada em A em 2026-11-25 e a auditoria está em
`../auditoria-de-organizacao.md`; D-02 foi aprovada em B e o dossiê dela é §0.1; o resumo com opções,
recomendação e consequência está em §10.

**Notas correspondentes:** `D-06` é detalhada em §0.1; `Q-SERIE-1` em
`../../wyd-db/migrations/serie-e-regras.md` §9; `Q-SEP-1` na mesma nota, §9.1. As perguntas de sistema
do `briefing-de-decisoes-1.0.0.md` continuam no dossiê de sistema e não são duplicadas aqui; esta tabela
cobre as decisões de foco, execução e contrato que o quadro precisa resolver.

| Pergunta | Opções | Recomendação | Consequência de cada opção |
|---|---|---|---|
| **`D-01`** (aprovada em A em 2026-11-24) — frente do projeto | **A** finalizar o `wyd-db`; **B** expandir os outros sistemas; **C** fechar a rastreabilidade | **A (aprovada)** | **A:** foco em A, com os atos de §0.2; **B/C:** adiariam as **64** células e os **141** atos |
| **`D-02`** (aprovada em B em 2026-11-24) — gate de código: cobertura e segurança | **A** 85% bloqueante já; **B** 85% no gate do sistema e segurança já; **C** tudo informativo até a 1.0.0 | **B (aprovada)** | **B:** cobertura informativa no `code.yml` até o gate do sistema fechar e `cargo audit`/`deny` bloqueantes; **A:** PR reprova por linha sem provar capacidade; **C:** vulnerabilidade e regressão entram sem barreira |
| **`D-03`** (aprovada em A em 2026-11-24) — aceite das 27 e as 2 `Q-BLK` | **A** aceitar agora, com `Q-BLK-20` = A e `Q-BLK-22` = A; **B** esperar o runner; **C** manter `proposto` | **A (aprovada)** | **A:** 168 → 141 atos e runner liberado; **B/C:** leva aberta e runner sob página não assinada |
| **`Q-FOCO-2`** (decidida em A em 2026-11-24) — confirmar `A` como foco e o tamanho da 1ª leva | **A** leva 1 = 12 aceites e parada para verificar; **B** emendar `migrations/` na mesma sessão; **C** trocar o foco para C (captura `A1`) | **A (decidida)** | **A:** 12 atos verificáveis e gate verde antes de escalar. **B:** 39 atos sem ponto de verificação intermediário. **C:** o foco passa a depender de terceiro, e o `wyd-db` fica parado com **0** bloqueio |
| **`Q-BLK-23`** (decidida em A em 2026-11-24) — as **16** linhas sem cenário entram na `1.0.0`? | **A** escrever os 16 cenários (uma `CAP-nn` cada) antes de `migrations/`; **B** adiar para depois; **C** declarar fora da `1.0.0` | **A (decidida)** | **A:** os **16** cenários foram escritos (`CAP-24`–`CAP-39`, `pendencias` §6.6.29) e as **16** linhas que estavam sem `CAP-nn` ganharam cenário. **B:** **13** `CHECK`/FK/índices teriam entrado sem cenário que os recuse. **C:** exigiria rebaixar **16** linhas das páginas donas — decisão de outro dono, recusada |
| **`D-04`** (aprovada em A em 2026-11-25) — organização definida nos sistemas sem código | **A** planejamento completo verificável; **B** só com crate/build/teste; **C** abertura atual basta | **A (aprovada)** | **A:** 2 de 13 organizados (`wyd-db`, `wyd-admin`) e 11 com lacuna nomeada; **B:** 12 esperam crate; **C:** 10 sem `RF` e 1 sem plano ficam escondidos |
| **`D-05`** (aprovada em A em 2026-11-25) — workspace Cargo, toolchain e versões | **A** workspace único + toolchain + Makefile + digests; **B** crates independentes; **C** híbrido | **A (aprovada)** | **A:** 1 lock/1 toolchain, segundo crate destravado; **B:** 1 lock hoje e até 9 se todos os crates nascerem; **C:** local ≠ CI em parte; destrava o segundo crate, não D-01 nem o gate de documentos |
| **`D-06`** (ABERTA — aguardando decisão do usuário em 2026-11-25) — frente depois de D-05 | **A** D-01/`wyd-db`; **B** executar D-05; **C** `wyd-portal`; **D** `wyd-protocol` | **A (recomendada)** | **A:** 141 atos/64 células/40 issues; **B:** 9 crates/6 manifestos, 0 atos; **C:** 33 `RF`/10 `RNF`, 0 atos; **D:** 5/131 páginas, 0 atos |
| **`Q-SERIE-1`** (ABERTA em 2026-11-25) — política dos 28 arquivos sem `down` | **A** forward-only com marcador explícito; **B** escrever os 28 `down` reais; **C** baseline dos 28 | **B (recomendada)** | **A:** aceita a série sem rollback, mas revisa ADR/spec e retira a prova `down`; **B:** mantém o contrato e destrava série/reset/testes de sistema; **C:** evita reexecução, mas pode esconder drift e ainda exige exceção |
| **`Q-SEP-1`** (ABERTA em 2026-11-25) — separador dos nomes (`NNNN-slug.sql` × `NNNN_slug.sql`) | **A** manter a leitura dos dois separadores e usar `_` nos novos; **B** renomear os 28 para `_`; **C** mudar o contrato para `-` | **A (recomendada)** | **A:** preserva a série atual e evita 28 renomeações/checksums novos; **B:** alinha os 28 ao contrato, mas exige 28 renomeações e novos checksums, sem mudar a numeração; **C:** espalha uma forma que o restante da documentação não usa e contraria o spec |


**Fora da tabela:** **`Q-BLK-17` = A**, respondida pela execução em 2026-11-24 — a leva de aceite começou
pelas 12 páginas de `tests/`, como a recomendação dizia (`pendencias` §6.6.22.5). **`Q-BLK-18` = A**, também
em 2026-11-24: a página `aceita` libera o código e a issue registra o vínculo depois, o que abriu o aceite
das **37** de `src/` (`pendencias` §6.6.27). **`Q-BLK-19` = A**, na mesma data: seis `RNF-DB-008`–`013`,
um por gate, repontaram as **11** capacidades e zeraram a dívida da matriz (`pendencias` §6.6.28). E
**`Q-BLK-21` = A**,
executada na mesma data — o dono revisou e assinou as 12 páginas, que estão `aceito (2026-11-24)`
(`wyd-db/revisao-de-q-blk-21.md` §5; `pendencias` §6.6.23). Nenhuma dessas quatro segue aberta; D-04 foi aprovada em A em 2026-11-25 (auditoria em
`../auditoria-de-organizacao.md`) e D-05 foi aprovada em A, com plano em `../plano-migracao-d-05.md`.

## 8. O que esta página não decide

- **Não aceita página**, não promove estado e não assina revisão: a assinatura das 12 de `tests/`
  (`Q-BLK-21` = A) e das 27 de `migrations/` (`Q-FOCO-4` = A) foi ato do dono em 2026-11-24, registrado em
  `wyd-db/revisao-de-q-blk-21.md` §5 e em `pendencias` §6.6.31.
- **Não cria issue, commit, push nem recurso no GitHub.**
- **Não é dona de nenhum número:** cada linha cita a página que mediu; divergência resolve-se na fonte.
- **Não substitui** o plano de fases (`wyd-1.0.0.md` §6), o registro de pendências
  (`pendencias-de-fechamento.md` §6.6) nem a medição interna do piloto (`foco-e-sequencia.md`).
- **Não decide as lacunas de captura:** elas pertencem aos sistemas donos e dependem de terceiro.

## 9. Quando esta página envelhece

| Fato | Deixa de valer quando | Onde atualizar |
|---|---|---|
| **26** capacidades sem execução (13 de 39 cobertas) | os quatro alvos restantes rodarem, ou `CAP-37`–`CAP-39` ganharem código | `tests/specs/capacidades.md` §7 e §2–§6 |
| **142** atos de aceite | qualquer página de `docs/` virar `aceito` | `pendencias` §6.6.31 |
| **28** páginas de `migrations/` `aceito` | uma página deixar de estar `aceito` | §5; `pendencias` §6.6.31 |
| **D-02** = B (cobertura informativa; audit/deny bloqueantes) | uma checagem sair do `code.yml` ou a política mudar | `plano-ci-real.md` §1, §5 e §7; esta página §0.1 |
| **D-04** = A (organização definida) | a auditoria de 2026-11-25 for revista | `../auditoria-de-organizacao.md`; esta página §0.1 |
| **D-05** = A (workspace Cargo) | o plano de migração for executado ou revisto | `../dossie-d-05.md`; `../plano-migracao-d-05.md`; esta página §0.1 e §10 |
| **D-06** = aberta (frente depois de D-05) | o dono responder A/B/C/D | esta página §0.1 e §10 |
| **`Q-SERIE-1`** = aberta (política dos 28 `down`) | o dono responder A/B/C | `../../wyd-db/migrations/serie-e-regras.md` §9; esta página §0.1 e §10 |
| **`Q-SEP-1`** = aberta (separador `-` × `_` no nome das migrations) | o dono responder A/B/C | `../../wyd-db/migrations/serie-e-regras.md` §9.1; esta página §7 e §10 |
| **6** lacunas abertas + 1 proposta | a captura do 7662 chegar | `lacunas.md` §1 de cada sistema |
| **39** capacidades com requisito | `Q-BLK-19` = A (2026-11-24): dívida de gate solto zerada; `Q-BLK-23` = A: os 16 cenários | `tests/specs/capacidades.md` §7 |
| **31 de 40** linhas com página | uma `CAP-nn` deixar de nomear uma linha que hoje tem página | `rastreabilidade.md` §2 e §3.4; esta página §2 |

## 10. O que segue aberto (2026-11-25)

| Item | Quantos | Dono |
|---|---:|---|
| **D-06** — **ABERTA — aguardando decisão do usuário** (frente depois de D-05) | **1** | dono do produto; dossiê em §0.1 |
| **`Q-SERIE-1`** — **ABERTA** (política dos 28 arquivos sem `down`) | **1** | dono do produto; opções e recomendação em `../../wyd-db/migrations/serie-e-regras.md` §9 |
| **`Q-SEP-1`** — **ABERTA** (separador `-` × `_` no nome das migrations) | **1** | dono do produto; opções e recomendação **A** em `../../wyd-db/migrations/serie-e-regras.md` §9.1 |
| lacunas abertas | **6** + **1** proposta | `protocol` 2, `client` 2 + 1, `net` 1, `bots` 1 |
| seeds bloqueadas | **6** | curadoria de dados (`migrations-checklist.md` §2) |
| capacidade sem ambiente | **1** (`CAP-08`), declarada não executável na `1.0.0` | dicionário de dados ausente (`Q-BLK-20` = A) |
| linhas de requisito sem `CAP-nn` na coluna `Teste` | **4** (`RF-DB-003`, `006`, `014` e `RNF-DB-001`) | `rastreabilidade.md` §1 |
| origem dos “59” | **1** pendência declarada **não localizável** | `pendencias` §6.6.16 |

**D-02 = B aprovada e passo 8a executado em 2026-11-24:** 85% bloqueante no gate do sistema, cobertura
informativa no `code.yml` e `cargo audit`/`deny` bloqueantes; a alteração está em `plano-ci-real.md` §5
(dossiê §0.1).

**D-04 = A aprovada em 2026-11-25 e auditada:** **2 de 13** sistemas organizados (`wyd-db` e
`wyd-admin`); os **11** restantes têm a lacuna nomeada em `../auditoria-de-organizacao.md`. A aprovação
fecha a pergunta de critério, não a de prioridade.

**D-05 = A aprovada em 2026-11-25:** workspace Cargo único na raiz, `rust-toolchain.toml`, `Makefile`,
um `Cargo.lock`, ações por SHA e imagens por digest. A execução está em
`../plano-migracao-d-05.md` e **ainda não foi executada**; D-05 não fecha lacuna de captura nem muda
D-01.

**Pergunta de decisão aberta — D-06:** qual frente executa primeiro depois de D-05?

| Opção | Frente | Recomendação | O que destrava |
|---|---|---|---|
| **A** | **D-01 / `wyd-db`**: atos 2–3 concluídos → ato 4 `CAP-37`–`CAP-39` → 40 issues + 141 atos | **A (recomendada)** | 141 atos, 64 células e 40 issues |
| **B** | Executar a migração D-05 | — | 9 crates e 6 comandos; 0 atos/issues |
| **C** | D-04 / `wyd-portal`: árvore `src/`/`tests/` + plano de teste | — | 33 `RF`/10 `RNF`; 0 atos/issues |
| **D** | `wyd-protocol`: `RF`, árvore, teste e roadmap | — | 5/131 páginas; 2 lacunas de captura |

**A recomendação é A**, porque é a única que fecha 141 atos/64 células/40 issues hoje; B, C e D
fecham 0 atos/issues no curto prazo e D ainda depende de captura. Os **atos 2–3 já estão feitos**; o
**ato 4** depende de **`Q-SERIE-1`** na parte de série real; o **ato 6** espera terceiro/dado. As
**6 lacunas abertas + 1 proposta**, os **6** seeds e a capacidade `CAP-08` continuam como estão; D-05
continua aprovada e não é reaberta.

**Pergunta de decisão aberta — `Q-SERIE-1`:** qual política para os 28 arquivos sem `down`? **A**
forward-only com marcador explícito; **B** escrever os 28 `down` reais (**recomendada**); **C** baseline
dos 28. A recomendação **B** mantém o contrato e destrava a série, o reset e os testes de sistema; **A**
exige revisão da `ADR-DB-0002` e retira a prova de rollback; **C** pode esconder drift e ainda exige
exceção para os arquivos legados. A análise completa está em `../../wyd-db/migrations/serie-e-regras.md` §9.

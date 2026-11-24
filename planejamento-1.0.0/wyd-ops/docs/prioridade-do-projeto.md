> Pergunta: "das quatro frentes candidatas — finalizar o `wyd-db`, expandir os outros sistemas, fechar as lacunas de rastreabilidade (as 4 linhas sem `CAP-nn` e as 9 sem página) e definir regras/padrões pendentes — qual vem primeiro, com que evidência contada e o que fica fora?"
> Público: dono do produto, arquitetura e quem coordena as frentes
> Dono: arquitetura publica; **a decisão é do dono do produto**
> Base: `planejamento-1.0.0/wyd-1.0.0.md` §1 e §6; `planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §6.6.22–§6.6.24; `planejamento-1.0.0/wyd-db/tests/specs/capacidades.md`; `planejamento-1.0.0/wyd-db/docs/specs/rastreabilidade.md` §1.1–§3.4
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24) — recomendação publicada; nenhuma frente iniciada e nenhuma página aceita por causa dela.

## 0. A recomendação

**Foco: frente A — finalizar o `wyd-db`. Em 2026-11-24 a leva `migrations/` executou: 28 `.sql` aplicados do zero e 13 cenários verdes.**

O `wyd-db` tem **64** células pendentes (24 `Teste` + 40 `Issue`) em `rastreabilidade.md` §1, **26** das **39**
capacidades sem execução (`capacidades.md` §7), **168** atos de aceite (141 `docs/` + 27 `migrations/`) e **27**
páginas de `migrations/` ainda `proposto` (`pendencias` §1). Já rodaram **13** capacidades (`CAP-24`–`CAP-36`,
`sistema_pg` §1.1) e a matriz está **13 de 39**.

Por que A: os **168** atos não esperam terceiro — são revisão e aceite; os **7** itens travados por terceiro
(**6** seeds e `CAP-08`) ficam fora deles, e as **2** `Q-BLK` abertas (`20`, `22`) são decisão do dono. B fecha
**0** bloqueio do piloto e libera **48** `RF` (`admin` 15 + `portal` 33) e **1** plano diretor; C fecha **3**
células e **0** das **9** linhas sem página; D fecha **1** página e **1** harness, ambos dentro do caminho de A
(§3). A sequência de atos está em §5; a pergunta de foco, em §11.

Esta página é o **único lugar publicado** que ranqueia as frentes do projeto. Ela não substitui o plano
de fases (`wyd-1.0.0.md` §6), nem o registro de pendências (`pendencias-de-fechamento.md` §6.6), nem a
direção do piloto (`wyd-db/foco-e-sequencia.md`): ela usa os números deles e diz qual vem agora.

**Por que ela mora aqui:** o dono convencional do ranking é `wyd-1.0.0.md` §6.2, que **não é publicado**;
o `docs/` do `wyd-db` está no teto de **144** páginas; e `wyd-ops` é o sistema de processo, com `docs/`
publicado e sem teto. Não existe convenção para “página de ranking do projeto”, então esta é a opção mais
próxima já usada — e a única que fica navegável no site.

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
| capacidade sem ambiente hoje | **1** (`CAP-08`, catálogo ausente) | `tests/alvos/contrato-catalogo.md` §3 |
| linhas da matriz de rastreabilidade com página que **nomeia** o requisito | **31 de 40** (29 em `sistema-pg`, 1 em `sistema-sessoes`, 1 em `sistema-stack-caps`); **9** sem página e **4** sem `CAP-nn` na coluna `Teste` (`RF-DB-003`, `006`, `014` e `RNF-DB-001`); **3** células são `—` por regra | `docs/specs/rastreabilidade.md` §1, §1.1 e §3.4 |
| páginas de `tests/` assinadas pelo dono | **12 de 12** `aceito (2026-11-24)`; **0** `proposto` | `wyd-db/revisao-de-q-blk-21.md` §5 |
| páginas de `migrations/` `aceito` | **1 de 28** (`0002`); **27** seguem `proposto` | `grep -l '^\*\*Estado:\*\* aceito' planejamento-1.0.0/wyd-db/migrations/*.md`; `pendencias` §1 |
| atos de aceite pendentes no `wyd-db` | **168** (141 `docs/` + **0** `src/` + 27 `migrations/` + **0** `tests/`) | `pendencias-de-fechamento.md` §6.6.22.1 e §6.6.27 |

## 3. As quatro frentes, com número e fonte

**As três perguntas do critério, uma coluna cada.** "Bloqueios" = item que hoje impede outro de andar;
"lacunas" = célula, ato ou página que sai de pendente para fechado; "planejamentos" = trabalho de outra
frente que deixa de esperar. Cada número cita a fonte; nenhum é estimado.

| Frente | Bloqueios destravados | Lacunas fechadas | Planejamentos liberados | Disponível hoje | Custo | Fonte |
|---|---|---|---|---|---|---|
| **A — Finalizar o `wyd-db`** | **26** capacidades sem execução (**13 de 39** cobertas) + **40** issues abertas (#82–#121) e **0** vinculadas às linhas + **2** `Q-BLK` (`20`, `22`) | **64** células (24 `Teste` + 40 `Issue`) e **168** atos (141 `docs/` + 27 `migrations/`); **0** lacuna aberta no sistema | o DDL deixa de ser proposta para `portal`/`admin`/`server` (contrato C4); as **40** issues viram trabalho autorizado; **12** planos de B passam a citar tabela aceita | **sim** — os 168 atos não esperam terceiro; 27 são aceite e 2 são `Q-BLK` | 27 aceites + runner + 3 cenários + 40 issues + 141 atos | `rastreabilidade.md` §1; `capacidades.md` §7; `sistema-pg.md` §1.1; `pendencias` §1 e §6.6.30 |
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
| **A — `wyd-db`** | **168** atos + **6** seeds + **1** capacidade + **2** `Q-BLK` | **2** `Q-BLK` (`20`, `22`); os **27** aceites são ato do dono, mas são os 27 atos de `migrations/` da coluna ao lado | **7**: **6** seeds (`0016`, `0017`, `0018`, `0020`, `0023`, `0027`) e `CAP-08` (catálogo ausente) | **168** (141 `docs/` + 27 `migrations/`, dos quais 27 são aceite) |
| **B — outros sistemas** | **12** planos, **51** páginas de base, **48** `RF` numerados | **0** | **0** — planejar não depende de ninguém | **51** páginas de base + **10** sistemas a numerar + **1** plano diretor (`wyd-ops`) |
| **C — rastreabilidade** | **3** células `Teste` + **1** gate de cobertura | **0** | **0** — nada externo depende de `CAP-nn` do banco | **3** cenários + 1 gate |
| **D — regras e padrões** | **1** página + **1** harness | **0** | **0** — é etapa 0 de A | **2** itens |

Fontes: `pendencias` §6.6.22.1 (168 atos), §6.6.22.5, §6.6.23 e §6.6.28 (as 3 perguntas), §6.5.1 (`Q-FOCO-1` = D);
`migrations-checklist.md` §2 (34 linhas: 28 `concluída` + 6 `bloqueada`); `tests/specs/capacidades.md` §3 e §7;
`lacunas.md` §1 de `wyd-protocol`, `wyd-client`, `wyd-net` e `wyd-bots`; `grep -rhoE 'RF-[A-Z]+-[0-9]{3}'`
por sistema.

**O que a tabela diz:** os **168** atos do `wyd-db` não esperam terceiro — esperam revisão e aceite; o que
falta é a assinatura das 27 páginas de `migrations/` e as **2** `Q-BLK`. B tem trabalho, mas não fecha item do
piloto; C tem 3 células executáveis e 7 itens (6 seeds + `CAP-08`) intocáveis; D é pré-requisito de A, não
alternativa.

## 3.2 A cadeia: o que libera o quê

| Passo | Libera | Não libera |
|---|---|---|
| **A, leva 1** — aceitar as 12 de `tests/` | a leva de `migrations/` (27 atos) e as **28** `.sql` de DDL | capacidade coberta pela leva de aceite (só a execução cobre) e código (esperava `Q-BLK-18`, respondida em A) |
| **A, leva 2** — aceitar as 37 de `src/` | o código do crate sob o invariante §5.1: as páginas que descrevem `dsn` e os módulos já estão `aceito` | implementação, teste executado e capacidade coberta |
| **A, leva `migrations/`** | as **40** issues como trabalho autorizado e o DDL que `portal` e `admin` já citam; a matriz fica **13 de 39** e o alvo **1 de 5** (2026-11-24) | as **6** seeds, que esperam curadoria de dado |
| **A, etapa 0 (regras)** | `migrations/serie-e-regras.md` e o `tests/common/` que os **4** alvos restantes usam | capacidade: o runner ainda não existe, então a página sozinha não muda `13 de 39` |
| **B, planejar** | requisitos numerados nos **10** sistemas que não têm `RF`; o plano diretor de `wyd-ops` | nada do `wyd-db`; e a **implementação** de `portal`/`admin` espera o DDL aceito de A |
| **C, rastreabilidade** | **3** células `Teste` (`RF-DB-003`, `006`, `014`); `RNF-DB-001` espera o gate de cobertura | nada do `wyd-db`: as **9** linhas sem página não viram página (2 de outro sistema, 1 gate de CI, 6 gate de série) |
| **Captura (`A1`)** | as **2** lacunas de `protocol`, a de `net`, a de `bots` e o código de `protocol`/`server` | nada do `wyd-db`: página de teste não depende de captura |

**A dependência que decide o ranking:** A é a única ponta que não espera terceiro — as duas respostas que
faltam (`Q-BLK-20` e `22`) são do dono e não travam os atos 1–4. B depende do DDL aceito de A para implementar;
a captura depende de `A1`; C não depende de nenhum dos dois. A ordem não é “A é maior”: é “A é a única que anda
hoje”.

## 4. O ranking

| Pos. | Frente | Itens fechados | Desbloqueio | Ganho por ato | Por que nesta posição |
|---:|---|---:|---|---:|---|
| **1** | **A — Finalizar o `wyd-db`** | **64** células (24 `teste` + 40 `issue`) + 168 atos | série, `metricas`, crate, 40 issues | **1** aceite por ato; **1 de 5** alvos já executado | é a única frente cujos fechamentos estão **disponíveis hoje**, e a única que libera artefato executável (teste e migration são código, e código espera página `aceita`) |
| **2** | **B — Expandir os outros sistemas** | **0** do piloto; **48** `RF` e **12** planos | 48 `RF`, 12 planos, 1 plano diretor | ~0 por ato do piloto (51 páginas de base) | não fecha bloqueio nenhum e não é pré-requisito de nada do piloto; anda em paralelo, não antes |
| **3** | **C — Fechar a rastreabilidade** | **3 de 64** células | 0 outra frente | 1 célula por `CAP` + cenário | fecha pouco e não toca as **9** linhas sem página; o resto depende de gate ou de outro dono |
| **4** | **D — Definir regras e padrões** | **1** página + **1** harness | 0 outra frente | 1 página + harness | é etapa 0 de A: executada sozinha, deixa A parado no mesmo lugar |

**A composição do item 1, sem dupla contagem:** **64** = **24** células `teste` (as que seguem em `P`) +
**40** células `issue`; o censo tem dono em `rastreabilidade.md` §1. As 37 células `teste` da matriz não
entram inteiras porque **13** já estão em `V` — executadas em 2026-11-24 —, e `V` não é pendência. As **40**
issues do roadmap não entram como item à parte: é na
célula `issue` que a alocação delas se registra, e nenhuma das 40 tem número (`rastreabilidade.md` §3.3). Os
**168** atos de aceite são contados em `pendencias-de-fechamento.md` §6.6.22.1 e não entram nas células.

**A leitura em uma frase:** A ganha por ser o único caminho que converte trabalho em artefato verificável
hoje; C fecha 3 células e para; B abre planejamento alheio sem fechar nada do piloto; D é etapa de A.

## 4.1 O que cada frente não escolhida ganha em seguida

- **B ganha com A:** o DDL deixa de ser proposta e os contratos de `portal`/`admin` passam a citar tabela
  aceita; e B pode numerar os **48** `RF` e os **10** sistemas sem `RF` **em paralelo**, sem competir por atenção.
- **C ganha com A:** **3** células de `Teste` (as três com página e sem `CAP-nn`) podem ser escritas a
  qualquer momento; A não as fecha, mas também não as bloqueia.
- **D ganha com A:** vira o ato 2 de A (`serie-e-regras`) e o `tests/common/` dos **4** alvos restantes.
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
páginas de `migrations/` seguem `proposto` — o aceite delas é ato à parte, e a série já nascida não o
substitui (`pendencias` §6.6.30).

**A sequência dentro de A — cada ato com o critério que o fecha:**

| Ato | O que produz | Critério observável de conclusão | Custo |
|---|---|---|---|
| **1. Aceite das 27 páginas de `migrations/`** (ato do dono; `Q-FOCO-4`) | 27 páginas `proposto` → `aceito`; 168 → 141 atos | `grep -rl '^\*\*Estado:\*\* aceito' planejamento-1.0.0/wyd-db/migrations/*.md \| wc -l` = **28** (27 + `0002`) e `EXIT_GATE=0` | 27 aceites |
| **2. `migrations/serie-e-regras.md`** | a regra da série: ordem, ledger `schema_migrations`, idempotência e drift | página com cabeçalho de 5 linhas, gate verde, e o contrato do ledger nomeado igual em `src/migrate/spec.md` | 1 página |
| **3. Runner `src/migrate` + `MigrateConfig`** | aplicar, reaplicar e detectar drift com ledger e advisory lock | `cargo test -p wyd-db --test contrato_migrations` verde; reaplicar na base migrada = **0** objeto novo; drift com código de saída ≠ 0 | código + 1 alvo |
| **4. `CAP-37`–`CAP-39`** | espera de DDL, p95 da leitura e boot medidos | `cargo test --test sistema_pg` inclui os **3** e passa; cobertura **13 → 16 de 39** se os três passarem | 3 cenários |
| **5. `Q-BLK-20`/`CAP-08`, 40 issues e 141 atos** | `CAP-08` declarada, issues autorizadas e o resto do aceite | `CAP-08` declarada **não executável** (se `Q-BLK-20` = A); **40** issues existentes vinculadas às linhas e **141** páginas a aceitar | 1 decisão + 40 issues + 141 atos |

**O primeiro ato é o único que depende do dono, e é o que destrava os outros:** sem as 27 páginas `aceito` a
leva de `migrations/` não fecha e o runner nasceria sobre página não assinada — o invariante §5.1 do
`AGENTS.md` proíbe. Com ele, os atos 2–5 são execução.

**O que fica postergado, e por quê:** **B** (48 `RF` + 12 planos) espera A porque a implementação de
`portal`/`admin` cita o DDL que A aceita; **C** (3 células) espera A porque o ganho é 3 de 64 e não move
nenhuma outra frente; **D** não é postergado — vira o ato 2 de A; as **6** seeds e a captura `A1` esperam
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
§6.6.22.5 (`Q-BLK-17`), §6.6.23 (`Q-BLK-21`), §6.6.26 (`Q-BLK-24`), §6.6.28 (`Q-BLK-19`) e §6.6.29
(`Q-BLK-23`); `Q-BLK-20` e `22` seguem com dossiê em §6.6.22.5 e §6.6.23. As perguntas de **projeto** têm
dossiê nesta página: `Q-FOCO-2` era o tamanho da leva 1, `Q-FOCO-3` é a frente e `Q-FOCO-4` é o aceite das 27
páginas; as duas últimas detalham as opções em §11.

| Pergunta | Opções | Recomendação | Consequência de cada opção |
|---|---|---|---|
| **`Q-FOCO-3`** — com a leva `migrations/` feita, qual frente recebe o foco? | **A** `wyd-db`; **B** outros sistemas; **C** rastreabilidade; **D** regras | **A** | **A:** 64 células + 168 atos e 40 issues; **B:** 48 `RF` + 12 planos, 0 do piloto; **C:** 3 de 64 células; **D:** 1 página + 1 harness, dentro de A |
| **`Q-FOCO-4`** — aceitar as **27** páginas de `migrations/` agora? | **A** agora; **B** só depois do runner verde; **C** manter `proposto` até a release | **A** | **A:** fecha a leva e libera os atos 2–5; **B:** amarra o aceite ao runner e adia 27 atos; **C:** mantém 168 atos e o runner sobre página não assinada |
| **`Q-BLK-20`** — o dicionário `banco-catalogo.md` entra na `1.0.0`? | **A** pós-`1.0.0`, com `CAP-08` declarada não executável; **B** entra e bloqueia a leva de `migrations/` | **A** | **A:** a leva anda sem ferramenta externa e a lacuna fica datada. **B:** 1 capacidade passa a segurar 28 `.sql` por um PR de terceiro |
| **`Q-BLK-22`** — as 12 páginas contam no critério de release de **144**? | **A** não contam; **B** contam | **A** | **A:** o critério segue **144** páginas de `docs/` e as 12 ficam como artefato de engenharia. **B:** o teto e o critério passam a **156**, e a release de documentação muda de tamanho |
| **`Q-FOCO-2`** (decidida em A em 2026-11-24) — confirmar `A` como foco e o tamanho da 1ª leva | **A** leva 1 = 12 aceites e parada para verificar; **B** emendar `migrations/` na mesma sessão; **C** trocar o foco para C (captura `A1`) | **A (decidida)** | **A:** 12 atos verificáveis e gate verde antes de escalar. **B:** 39 atos sem ponto de verificação intermediário. **C:** o foco passa a depender de terceiro, e o `wyd-db` fica parado com **0** bloqueio |
| **`Q-BLK-23`** (decidida em A em 2026-11-24) — as **16** linhas sem cenário entram na `1.0.0`? | **A** escrever os 16 cenários (uma `CAP-nn` cada) antes de `migrations/`; **B** adiar para depois; **C** declarar fora da `1.0.0` | **A (decidida)** | **A:** os **16** cenários foram escritos (`CAP-24`–`CAP-39`, `pendencias` §6.6.29) e as **16** linhas que estavam sem `CAP-nn` ganharam cenário. **B:** **13** `CHECK`/FK/índices teriam entrado sem cenário que os recuse. **C:** exigiria rebaixar **16** linhas das páginas donas — decisão de outro dono, recusada |

**Fora da tabela:** **`Q-BLK-17` = A**, respondida pela execução em 2026-11-24 — a leva de aceite começou
pelas 12 páginas de `tests/`, como a recomendação dizia (`pendencias` §6.6.22.5). **`Q-BLK-18` = A**, também
em 2026-11-24: a página `aceita` libera o código e a issue registra o vínculo depois, o que abriu o aceite
das **37** de `src/` (`pendencias` §6.6.27). **`Q-BLK-19` = A**, na mesma data: seis `RNF-DB-008`–`013`,
um por gate, repontaram as **11** capacidades e zeraram a dívida da matriz (`pendencias` §6.6.28). E
**`Q-BLK-21` = A**,
executada na mesma data — o dono revisou e assinou as 12 páginas, que estão `aceito (2026-11-24)`
(`wyd-db/revisao-de-q-blk-21.md` §5; `pendencias` §6.6.23). Nenhuma dessas quatro segue aberta; as que
restam estão na tabela acima.

## 8. O que esta página não decide

- **Não aceita página**, não promove estado e não assina revisão: a assinatura das 12 foi ato do dono
  (`Q-BLK-21` = A, 2026-11-24), registrado em `wyd-db/revisao-de-q-blk-21.md` §5.
- **Não cria issue, commit, push nem recurso no GitHub.**
- **Não é dona de nenhum número:** cada linha cita a página que mediu; divergência resolve-se na fonte.
- **Não substitui** o plano de fases (`wyd-1.0.0.md` §6), o registro de pendências
  (`pendencias-de-fechamento.md` §6.6) nem a direção do piloto (`foco-e-sequencia.md`).
- **Não decide as lacunas de captura:** elas pertencem aos sistemas donos e dependem de terceiro.

## 9. Quando esta página envelhece

| Fato | Deixa de valer quando | Onde atualizar |
|---|---|---|
| **26** capacidades sem execução (13 de 39 cobertas) | os quatro alvos restantes rodarem, ou `CAP-37`–`CAP-39` ganharem código | `tests/specs/capacidades.md` §7 e §2–§6 |
| **168** atos de aceite | qualquer página virar `aceito` | `pendencias` §6.6.22.1 |
| **27** páginas de `migrations/` `proposto` | o dono aceitar (`Q-FOCO-4`) | §5; `pendencias` §6.6.30 |
| **6** lacunas abertas + 1 proposta | a captura do 7662 chegar | `lacunas.md` §1 de cada sistema |
| **39** capacidades com requisito | `Q-BLK-19` = A (2026-11-24): dívida de gate solto zerada; `Q-BLK-23` = A: os 16 cenários | `tests/specs/capacidades.md` §7 |
| **31 de 40** linhas com página | uma `CAP-nn` deixar de nomear uma linha que hoje tem página | `rastreabilidade.md` §2 e §3.4; esta página §2 |

## 10. O que segue aberto (2026-11-24)

| Item | Quantos | Dono |
|---|---:|---|
| `Q-BLK-20` e `22` (`17`, `18`, `19`, `21`, `23` e `24` respondidas) | **2** | dono do produto (`pendencias` §6.6.22.5, §6.6.23 e §6.6.24) |
| `Q-FOCO-2` | **1** | dono do produto (dossiê nesta página) |
| `Q-FOCO-3` | **1** | dono do produto (dossiê em §11 desta página) |
| `Q-FOCO-4` | **1** | dono do produto (aceite das 27 páginas; dossiê em §11) |
| lacunas abertas | **6** + **1** proposta | `protocol` 2, `client` 2 + 1, `net` 1, `bots` 1 |
| seeds bloqueadas | **6** | curadoria de dados (`migrations-checklist.md` §2) |
| capacidade sem ambiente | **1** (`CAP-08`) | dicionário de dados ausente |
| linhas de requisito sem `CAP-nn` na coluna `Teste` | **4** (`RF-DB-003`, `006`, `014` e `RNF-DB-001`) | `rastreabilidade.md` §1 |
| origem dos “59” | **1** pendência declarada **não localizável** | `pendencias` §6.6.16 |

Nenhuma delas é decisão desta página: ela recomenda, mede e aponta o dono.

## 11. Depois de `Q-BLK-24` e de `Q-BLK-21`: a próxima frente (2026-11-24)

`Q-BLK-24` foi aprovada em **A** e executada: `RNF-DB-006`, `RNF-DB-007` e `RF-DB-017` existem, as três
capacidades apontam para eles e a matriz de rastreabilidade tem **40** linhas. `Q-BLK-21` também foi
aprovada em **A** e executada: as 12 páginas de `tests/` estão `aceito (2026-11-24)`. O que sobra não é mais
dúvida de mapeamento nem assinatura — é **ordem e código**. Esta seção mede o estado de agora e mantém a
pergunta que o §0 não fazia.

| Item | Agora | Fonte |
|---|---:|---|
| páginas de `tests/` assinadas | **12 de 12** `aceito (2026-11-24)` — **0** pendente | `wyd-db/revisao-de-q-blk-21.md` §5 |
| alvos em `crates/wyd-db/Cargo.toml` | **1 de 5** (`sistema_pg`; o `Cargo.toml` tem `src/lib.rs`, `src/dsn.rs` e o `[[test]]`) | `find crates -type f`; `grep -c '\[\[test\]\]' crates/wyd-db/Cargo.toml` |
| capacidades cobertas | **13 de 39** | `tests/specs/capacidades.md` §7 |
| linhas da rastreabilidade (total / com página / sem página / sem `CAP-nn`) | **40** / **31** / **9** / **4** | `rastreabilidade.md` §1 e §3.4 |
| atos de aceite pendentes | **168** | `pendencias` §6.6.22.1 |
| arquivos `.sql` da série | **28**, aplicados do zero como `wyd_migrator` em 2026-11-24 | `ls migrations/0*.sql` |
| decisões abertas do `wyd-db` | **2** (`Q-BLK-20` e `22`) | §7 e §10 |
| sistemas sem plano diretor | **1** (`wyd-ops`) | `ls planejamento-1.0.0/wyd-ops/*.md` |
| lacunas abertas no projeto | **6** + **1** proposta | `lacunas.md` §1 de cada sistema |

**Q-FOCO-3 — com a leva 1 feita, qual frente recebe o foco?**

| Opção | O que produz | Custo | O que adia | Veredito |
|---|---|---|---|---|
| **A** — **finalizar o `wyd-db`:** aceite das **27** páginas de `migrations/`, `serie-e-regras`, runner e os **3** cenários restantes | **64** células + **168** atos; **40** issues; **26** capacidades | 27 aceites + 1 página + runner + 3 cenários | B (48 `RF`, 12 planos) e C (3 células) | **Recomendada** |
| **B** — **expandir os outros sistemas**, começando pelo plano diretor do `wyd-ops` (o único dos 13 sem plano diretor) | **48** `RF` numerados, **12** planos e **1** diretor | 51 páginas + 10 sistemas | as **64** células e as **26** capacidades de A | Preterida |
| **C** — **fechar a rastreabilidade** (as **4** sem `CAP-nn` e as **9** sem página) | **3 de 64** células; **0** das 9 páginas | 3 `CAP` + 3 cenários + 1 gate | 61 células, 26 capacidades e 40 issues | Preterida |
| **D** — **definir regras e padrões pendentes** | **1** página (`serie-e-regras`) + **1** harness | 1 página + `tests/common/` | nada que A não faça: é o ato 2 de A | Preterida — fatia de A |

**Por que A.** É a única frente que fecha **64** células e **168** atos com trabalho executável: **0** item
dela espera terceiro. O que falta é o aceite das **27** páginas (ato do dono, `Q-FOCO-4`) e o runner — as
páginas de `src/` já estão `aceito`, então o código pode nascer assim que a etapa 1 fechar. Sem A, a leva de
`migrations/` não fecha, as **26** capacidades sem execução seguem sem prova e as **40** issues não viram
trabalho autorizado.

**Por que não B.** Medido em §3: **nada do `wyd-db` trava o planejamento dos outros sistemas** — B anda em
paralelo e libera **48** `RF` + **12** planos, mas fecha **0** bloqueio do piloto. Elegê-lo como foco adia as
**64** células e as **26** capacidades sem execução. O plano do CI, argumento de B, já existe
(`wyd-ops/plano-ci-real.md`, `P`).

**Por que não C.** Fecha **3 de 64** células e **0** das **9** linhas sem página — as 9 não são fecháveis por
cenário (2 de outro sistema, 1 gate de CI, 6 gate de série), e o `RNF-DB-001` depende do gate de cobertura, não
de `CAP`. É barato por `CAP`, mas o retorno é 3 células e nenhuma outra frente.

**Por que não D.** `serie-e-regras` e `tests/common/` não são alternativa a A: são o ato 2 e o harness dos
**4** alvos restantes. Executada sozinha, D fecha **1** página e deixa a matriz em **13 de 39**.

**A captura `A1` fica fora do ranking:** **6** seeds + **6** lacunas abertas, todas por terceiro/dado. Entra no
topo da fila no dia em que a medição chegar; hoje, escolhê-la é escolher esperar.

**O caminho curto, se A:** os cinco atos de §5, na ordem — (1) aceite das **27** páginas (dono), (2)
`serie-e-regras.md`, (3) runner, (4) `CAP-37`–`CAP-39`, (5) `CAP-08` + **40** issues + **141** atos. O ato 1 é
o de menor custo e maior efeito: **27** atos, e a leva fecha com o mesmo `EXIT_GATE=0`.

**Q-FOCO-4 — aceitar as 27 páginas de `migrations/` agora?**

| Opção | Ganha | Adia | Veredito |
|---|---|---|---|
| **A** — aceitar as 27 agora | **168 → 141** atos e a leva de `migrations/` fecha com `EXIT_GATE=0`; o runner nasce sob página assinada | nada que a série já não tenha provado: 28 `.sql` aplicaram e 13 cenários rodaram | **Recomendada** |
| **B** — aceitar só depois do runner verde | amarra o aceite à prova automatizada de reaplicação | **27** atos e os atos 2–5 de A; o runner nasceria sob página `proposto` | Preterida |
| **C** — manter `proposto` até a release | nenhuma assinatura agora | **168** atos e a leva de `migrations/` aberta no fechamento | Preterida |

**O que esta seção não decide:** não assina página, não responde `Q-BLK-20`, `22` ou `Q-FOCO-4`, não inicia
frente e não promove nada a `aceito`. Ela mede, recomenda e nomeia as decisões que faltam.

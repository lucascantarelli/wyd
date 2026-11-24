> Pergunta: "quais frentes internas do `wyd-db` estão abertas, quanto cada uma custa e de que terceiro depende — sem substituir o foco do projeto?"
> Público: Dono do projeto, curadoria de conteúdo e equipe do `wyd-db`
> Dono: equipe do `wyd-db`
> Base: `migrations-checklist.md` §1–§4; `plano-migrations.md` §3–§4; `docs/planos/plano-de-dados-e-seeds.md` §2; `docs/planos/plano-de-testes.md` §5; `docs/specs/rastreabilidade.md`; `pendencias-de-fechamento.md` §1 e §6.6.20; `pedido-de-curadoria-dos-seeds.md`; `../wyd-1.0.0.md` §1
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

**Foco do projeto (2026-11-24): D-01 = A — finalizar o `wyd-db`.** A página canônica é
`../wyd-ops/docs/prioridade-do-projeto.md` §0.1; esta página mede as frentes internas do `wyd-db` e
mantém as perguntas de seed. Ela **não** publica um segundo ranking do projeto: onde divergir, vale a
página canônica.

Este documento mede as frentes do `wyd-db` e preserva as perguntas de seed: enumera as frentes abertas
com o número que cada uma tem no arquivo dono e registra o custo/dependência de cada uma. Ele não é
proposta de página nova nem seed: é medição e insumo. **A decisão de foco e a ordem do projeto são da
página canônica §0.1.** O ranking de §6.6.20 de `pendencias-de-fechamento.md` continua sendo o registro
do lote G; aqui ele é apenas citado. Onde houver divergência, vale a página canônica.

## 1. As frentes abertas hoje, medidas

Nenhum número abaixo foi estimado: todos saem de uma tabela de somas ou de uma contagem de arquivo, com a
seção onde vivem.

| # | Frente | Número | Fonte |
|---:|---|---|---|
| **F1** | **Curadoria e medição do 7662** | **6** datasets parados; **6** linhas `bloqueada` por dado; **0** por decisão; **0** por dependência; **6** páginas de migration faltando (28 de 34) | `migrations-checklist.md` §3 (motivo por arquivo), §4 (soma `28 + 6 = 34`), §1 (o que cada estado significa); `plano-de-dados-e-seeds.md` §2 |
| **F2** | **Issues + suíte de testes** | **40** issues abertas (#82–#121) e **0** vinculadas; **12 de 12** páginas de `tests/`, todas `aceito` (2026-11-24); matriz `CAP-nn` com **39** capacidades e **13** cobertas por teste executado; **64** células pendentes na matriz (**24** `teste` + **40** `issue`; **13** `teste` já em `V` e outras **3** são `—` por regra); **31 de 40** linhas com página que nomeia o requisito (**9** sem página, **0** sem cenário) | `docs/roadmap/` (40 páginas `WYDDB-nnn`); `docs/planos/riscos.md` `R-5`; `wyd-db-1.0.0.md` §6.17; `rastreabilidade.md` §2; `plano-de-testes.md` §5 |
| **F3** | **Aceite de página** | **168** atos pendentes: **141** em `docs/` + **0** em `src/` + **27** em `migrations/` + **0** em `tests/`; **52** páginas `aceito` (2 ADRs + `migrations/0002` + 12 de `tests/` + 37 de `src/`); a série de **28** `.sql` já aplica do zero e 13 cenários rodaram, mas nenhuma das 27 páginas de `migrations/` virou `aceito` | contagem de `**Estado:**`; `wyd-db-1.0.0.md` §10 (11 itens por página); `proposta-decisoes-m33.md` §2 |
| **F4** | **Requisito das 109 tabelas** | **16 de 109** tabelas com `RF`/`RN`; o critério de §11.1 exige cobertura de todas | `rastreabilidade.md` §3.2; `wyd-db-1.0.0.md` §11.1 |
| **F5** | **Expansão para outros sistemas** | **12** sistemas fora do `wyd-db`; **6** lacunas abertas + **1** proposta, todas de captura (protocol 2, client 2+1, net 1, bots 1); `wyd-protocol` com **5 de 131** páginas | `../wyd-1.0.0.md` §1.1; `AGENTS.md` §6 |
| **F6** | **Perguntas ao dono** | **2** `Q-BLK` abertas (`Q-BLK-20` e `22`; `17`, `18`, `19`, `21`, `23` e `24` foram respondidas em A em 2026-11-24) + **2** de foco do projeto (`Q-FOCO-3` e `Q-FOCO-4`) + **4** de seed (§5 deste documento) = **8**; **0** lacunas abertas no `wyd-db` | `pendencias-de-fechamento.md` §6.6.22.5, §6.6.23, §6.6.24 e §6.6.29; `../wyd-ops/docs/prioridade-do-projeto.md` §0.1, §7 e §10; `lacunas.md` §1 |

**O que os três zeros de F1 querem dizer:** o bloqueio dos seis não é de desenho nem de decisão — o DDL
fechou em 2026-11-20/21 e os cinco lotes de 2026-11-21 foram aprovados. Sobrou **dado**: nenhuma linha
pode ser escrita antes de o cliente 7662 ser medido (`plano-migrations.md` §4, decisão de 2026-11-23).
O instrumento do pedido já existe (`pedido-de-curadoria-dos-seeds.md`) e não muda esse número.

## 2. Frentes internas do `wyd-db` — medição, não ranking

> **O ranking das frentes do projeto é a página canônica** `../wyd-ops/docs/prioridade-do-projeto.md`
> (D-01 = A, 2026-11-24). A tabela abaixo mede as frentes internas do `wyd-db` sem posição de projeto;
> serve para mostrar o custo e a dependência de F1–F6, não para escolher a frente do projeto. Onde as
> duas divergirem, vale a página canônica.

Critério da missão: itens bloqueantes resolvidos, regras/padrões fixados e planejamentos liberados, **por
unidade de esforço** — e uma coluna que os números de F1 obrigam a ter: **depende de terceiro?**

| Frente | Itens que libera | Quais | Regras fixadas | Depende de terceiro | Custo |
|---|---:|---|---|---|---|
| **F2 — suíte + issues** | **31 de 40** linhas com cenário e **5** alvos prontos | 12 páginas de `tests/` aceitas (**0** atos de aceite restantes na leva 1), a matriz `CAP-nn` (**39** capacidades), as 40 linhas com natureza e onda, 40 vínculos de issue | **3**: fixture/factory, dado de teste, matriz `CAP-nn` | **não** | **0** atos de aceite (feito em 2026-11-24) + 3 sessões (issues) |
| **F1 — curadoria 7662** | **19 itens** + `R-6` | 6 páginas de migration, 6 `.sql`, 6 linhas de checklist, 1 página de fluxo (`carga-de-conteudo`) | 0 | **sim** — cliente 7662 | 6 medições + 12 artefatos |
| **F3 — aceite** | **168 atos** | `src/` escrito (**37** páginas, aceitas em 2026-11-24), série aplicada, `planos/metricas.md`, 144/144 | 0 (decididas em `Q-M33-1`–`6`) | não | 100–168 h |
| **F4 — requisito das 109** | **1 critério** | cobertura de §11.1; matriz sem lacuna | **1**: política de cobertura | não | 19 a 109 blocos |
| **F5 — outros sistemas** | **0** no `wyd-db` | nada daqui; 12 planos alheios | 0 | **sim** — captura A1 | alto |

**Leitura interna, sem ranking de projeto:** F1 depende do 7662; F2/F3/F4 medem o `wyd-db`; F5 não move
o piloto. O foco do projeto é D-01 = A na página canônica; esta ordem interna não substitui aquela.

## 3. A frente do projeto é A; F2 é a leva de testes/issues

**Feito em 2026-11-24:** as 12 páginas foram aceitas (`Q-BLK-21` = A) e, com `Q-BLK-18` = A, as **37** de
`src/` também — o que deixou **5** alvos prontos para código e, com os **16** cenários de `Q-BLK-23` = A, a matriz
em **31 das 40** linhas com cenário nomeado. `sistema_pg` nasceu e **13** cenários rodaram verdes contra a base
migrada; as **26** capacidades restantes seguem sem execução. O próximo elo é o código dos outros quatro
alvos, com a página já aceita (invariante §5.1), e a recomendação de foco vigente está em
`../wyd-ops/docs/prioridade-do-projeto.md` §0.1 (D-01).
Os elos 1 e 2 estão entregues desde o lote H: as 12 páginas e a matriz de **39** capacidades existem, com
**13** cobertas; os elos 3 e 4 citam alvo escrito e 13 cenários executados. O elo 5 (issues) é o
passo 5 da ordem canônica, e não o próximo — `Q-BLK-17` foi respondida pela execução (a leva 1 começou por `tests/`)
e `Q-BLK-20` e `22` seguem abertas — `18`, `19` e `23` foram respondidas em A (o aceite libera o código; os
seis `RNF` de gate zeraram a dívida da matriz; os 16 cenários sem `CAP-nn` foram escritos).

| Elo | O que entrega | Itens liberados | Fonte do limite |
|---|---|---:|---|
| 1 | as **12** páginas de `tests/` (index, fixtures, dados, `resources/README`, 5 alvos, 3 specs) | 12 páginas | `wyd-db-1.0.0.md` §6.17 |
| 2 | `specs/capacidades.md` com a matriz `CAP-nn → alvo → cenário → requisito` | 1 critério de saída (o item 6) | `plano-de-testes.md` §5 |
| 3 | `P` deixa de ser promessa **nas 31 linhas com cenário escrito** | **31** linhas com `Página`; **9** seguem sem página — 2 de `wyd-server`, 1 de gate de CI e 6 de gate de série (`rastreabilidade.md` §3.4) | `rastreabilidade.md` §2 |
| 4 | os **29** DoDs que citam teste passam a ter onde apontar | 29 issues | `docs/roadmap/` (medido no `## DoD`) |
| 5 | as **40** issues em 3 lotes por onda, a partir das ondas 1–2 | 31 vínculos na coluna `issue` + PR ↔ issue | `pendencias-de-fechamento.md` §6.6.20 |

**O que a leva não cobre:** **9** das **40** linhas seguem sem página que prove o requisito — **2** por serem
de `wyd-server` (`RF-DB-015`, `RF-DB-016`), **1** por ser gate de CI (`RNF-DB-001`) e **6** por serem gate de
série (`RNF-DB-008`–`013`, medidos pelo próprio gate) — e nenhuma por falta de cenário
(`rastreabilidade.md` §3.4). Os **16** cenários que faltavam foram escritos em 2026-11-24 (`Q-BLK-23` = A,
`pendencias-de-fechamento.md` §6.6.29), antes da leva de `migrations/`.

**O que F2 não destrava:** código. A página de teste é planejamento; escrever executor de teste continua
exigindo `aceita` (F3). F2 também não fecha nenhuma lacuna — o `wyd-db` não tem lacuna aberta.

## 4. A trilha paralela: F1 e a cadeia até `content_version`

Quando a medição chegar, F1 anda sem decisão nova e passa na frente das frentes internas do `wyd-db`
pelo custo por item (não é a frente do projeto; D-01 = A):

| Elo | O que entrega | Itens liberados | Fonte |
|---|---|---:|---|
| 1 | **1 entrega por dataset**: `.tsv` + `source.txt` (`path`, `sha256`, `size_bytes`, `row_count`) + `ausentes.tsv` | as **6** linhas `bloqueada` saem de §3 | `pedido-de-curadoria-dos-seeds.md` §4 |
| 2 | a **página** da migration do seed, escrita e revisada | **6** páginas (28 → 34) | `migrations-checklist.md` §4; `plano-migrations.md` §3 |
| 3 | o **`.sql`** escrito à mão e aplicado depois do DDL do domínio | **6** arquivos; série fecha **34 de 34** | `plano-migrations.md` §2 e §4 |
| 4 | `content_version` incrementa **uma vez por seed aplicado** | a versão fecha em **7** | `plano-migrations.md` §4 e §7 |
| 5 | a página de fluxo deixa de declarar dependência | **1** página (`docs/specs/fluxos/carga-de-conteudo.md` §"Depende de") e o risco **`R-6`** | `riscos.md` |

O elo 1 é o único que depende de terceiro; os elos 2 a 5 são do `wyd-db` e são **mecânicos** — por isso a
frente não precisa de nova pergunta ao dono além das quatro de §5.

## 5. O que só o dono decide

Quatro perguntas, cada uma com opções curtas, recomendação e consequência. Nenhuma está respondida aqui;
nenhum valor foi preenchido para encurtar a decisão. As da leva de testes/issues (F2) foram respondidas em **A**
em §6.6.20 — `Q-BLK-12` (testes antes das issues), `Q-BLK-13` (as 12 páginas) e `Q-BLK-14` (issues em 3
lotes) — e não se repetem aqui.

### `Q-SEED-1` — as 4 cidades entram em qual dataset?

**Medido.** As **4 cidades** aparecem na lista de contagens a preservar (`plano-migrations.md` §4) sem
arquivo atribuído, e não há seed de economia entre os seis (`plano-de-dados-e-seeds.md` §2).

- **A)** entram em **`0020_seed_world.sql`**, junto de `region` e `guild_zone` — sem arquivo novo, série
  continua **34**. **Recomendada:** é o dataset de conteúdo do mundo e a ordem já satisfaz a FK de
  `game.city` (`0011`).
- **B)** ganham **arquivo próprio** (`0020` continua mundo) — a série vai a **35** e o teto de §3 muda.
- **C)** ficam **fora da `1.0.0`**, declaradas ausentes no seed de eventos.
- **Consequência:** A fecha a contagem sem tocar na série; B muda `34 → 35` em `plano-migrations.md` §3,
  §4 e no checklist; C deixa `world_event.requires_city` sem origem de dado e vira lacuna declarada.

### `Q-SEED-2` — o que significam `grade` e `requirements` do item?

**Medido.** As duas colunas existem em `content.item_catalog` (`ddl-confirmado.md` §5.18) e a semântica é
`H` no dicionário: "semântica do legado, sem trace no planejamento novo".

- **A)** a curadoria entrega a **lista de valores observados** e o significado de cada faixa; o dicionário
  passa a descrever o que foi medido. **Recomendada:** fecha o `H` com dado, sem tocar no DDL.
- **B)** `grade` vira **domínio fechado** com `CHECK` dos valores medidos — muda o DDL e a página de
  `0014`.
- **C)** as duas ficam como `H`, sem `CHECK` e sem descrição.
- **Consequência:** A fecha a semântica sem mudar schema; B cria decisão de DDL e exige republicar
  `0014`/dicionário; C mantém a dívida e o dicionário segue dizendo que não sabe.

### `Q-SEED-3` — as 3 skills da Kersef entram como quê?

**Medido.** **154** skills = 151 + 3 da Kersef; **63** com linhagem (`plano-migrations.md` §4). O `H` do
dicionário diz: "origem medida, significado não".

- **A)** entram **habilitadas** com a linhagem da Kersef registrada em `skill_lineage`, e a curadoria diz o
  que elas são. **Recomendada:** preserva a contagem declarada e a origem medida; a pergunta é de
  significado, não de inclusão.
- **B)** entram com `enabled = false` até haver regra de uso.
- **C)** saem do seed, e a contagem passa a **151** em `plano-migrations.md` §4, `plano-de-dados-e-seeds` §2
  e no pedido.
- **Consequência:** A mantém 154 e fecha o `H` com nota; B mantém 154 mas esconde o conteúdo do jogo; C
  muda três documentos de contagem e deixa o cliente com skill jogável fora da `1.0.0`.

### `Q-SEED-4` — e se a receita de acessório não existir no 7662?

**Medido.** `content.accessory_refine_rule` tem DDL fechado (`ddl-confirmado.md` §5.23) e o `H` do domínio
registra "origem posterior ao 7662; sem regra oficial".

- **A)** se existir no cliente, entra; se não existir, a tabela fica **vazia na `1.0.0`** e a ausência é
  declarada em `ausentes.tsv`. **Recomendada:** não inventa linha e não reabre o DDL fechado.
- **B)** a tabela **sai** do DDL — muda `109` tabelas em `ddl-confirmado.md` §1, no checklist e no
  dicionário.
- **C)** o seed `0023` **inteiro** é adiado até a receita aparecer.
- **Consequência:** A mantém a série e o schema, com uma tabela vazia justificada; B mexe em contagem de
  tabela e em três documentos; C segura também refino, gemas, SANC e combine, que já têm fonte.

## 6. O que este passo não faz

- **Não escreve página de seed** nem o `.sql` dos seis: a escrita é do lote seguinte, depois da medição.
- **Não afirma valor**: as contagens de `plano-migrations.md` §4 são de 2026-11-08/15 e seguem `H` até a
  reconferência; este documento repete a data, não o valor.
- **Não responde** `Q-SEED-1`–`4` nem `Q-BLK-12`–`16`: recomenda e mostra a consequência de cada opção.
- **Não cria lacuna** nem renumera ID: o bloqueio é o dos seis arquivos em `migrations-checklist.md` §3.
- **Não altera o DDL** nem o teto de 144 páginas: nenhuma opção de §5 está aplicada.

## 7. Como saber que este documento envelheceu

| Fato | Quando deixa de valer | Onde atualizar |
|---|---|---|
| 6 datasets `bloqueada` | a primeira medição chegar | `migrations-checklist.md` §3; §4 deste arquivo |
| 26 de 39 capacidades cobertas | os quatro alvos restantes rodarem (ou as lacunas de `CAP-37`–`CAP-39` fecharem) | `tests/specs/capacidades.md`; §1 deste arquivo |
| **168** atos de aceite | qualquer página virar `aceito` | §1 deste arquivo; `proposta-decisoes-m33.md` §2 |
| 16 de 109 tabelas | `Q-BLK-15` = A ou B for executada | `rastreabilidade.md` §3.2 |

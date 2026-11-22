# WYD 1.0.0 — panorama e plano de execução

> **Estado: plano de trabalho, não decisão aprovada.** Este documento é o painel único do projeto
> inteiro: o que existe, o que está medido, em que ordem o planejamento avança e qual é o gate de
> cada etapa — inclusive o gate de cobertura funcional que fecha a `1.0.0` (§4.2). Ele **não**
> substitui o plano diretor de nenhum sistema e **não** decide arquitetura.
>
> Medido em **2026-11-19**. Toda contagem traz a fonte; número sem fonte é `H`.
>
> **Alvo declarado pelo dono em 2026-11-20:** a `1.0.0` é o **jogo completo** — todos os sistemas e
> funcionalidades do jogo oficial funcionando —, e o critério de saída é **cobertura funcional de
> 100%** (§4.2). O número de cobertura é medido em §1.5; nenhum total de funcionalidade é estimado
> aqui.

---

## 0. Como ler

| Se você quer… | Vá para |
|---|---|
| saber o que está pronto e o que não está | §1 |
| saber quanto do jogo já funciona | §1.5 |
| saber o que já foi decidido e o que está aberto | §2 |
| ver contradições que precisam de decisão do time | §3 |
| saber a ordem de execução e o gate de cada fase | §4 |
| saber o que este plano não decide | §5 |
| saber o que fazer em seguida, por ganho ÷ esforço | §6 |

---

## 1. Estado medido

### 1.1 Por sistema

| Sistema | Documentos | Estado | Evidência |
|---|---:|---|---|
| `wyd-db` | 246 | piloto parcial: 236 páginas com cabeçalho, **28 de 34** migrations escritas (os 6 seeds seguem bloqueados), 37 páginas de `src/`, 0 em `tests/`, `prompts/` e `skills/`; dicionário **completo**, **109 das 109** páginas de tabela | `find`/`grep` em 2026-11-22 |
| `wyd-protocol` | 7 | 2º piloto aberto: plano diretor com teto de 131 páginas, 5 escritas, 20 lacunas (17 decididas em 2026-11-20, 3 abertas por dependerem de captura), nenhum ADR; série de entrega do crate `0.1.0` → `1.0.0` (§4.1) | `wyd-protocol-1.0.0.md` e `lacunas.md`, 2026-11-22 |
| `wyd-docs` | 9 | plano diretor e lacunas + site MkDocs 2.0 implementado; 5 páginas em `docs/` | `wyd-docs-1.0.0.md`; `wyd-docs/`, 2026-11-22 |
| `wyd-portal` | 12 | planejado: dashboard, `lacunas.md`, plano de armaria e mercado e 8 páginas em `docs/` (11 de planejamento + o prompt); **33 `RF-POR`**, 10 `RNF-POR`, 0 lacunas abertas (8 fechadas; `09` em proposta) | `wyd-portal/`; `lacunas.md` §1, 2026-11-22 |
| `wyd-admin` | 11 | planejado: dashboard, `lacunas.md` e 9 páginas em `docs/`; contrato C4 com 14 rotas (7 de staff, 4 de custódia de item, 3 utilitárias); **15 `RF-ADM`**, 10 fatias; zero lacunas abertas (as sete fecharam até 2026-11-22) | `wyd-admin/`, 2026-11-22 |
| `wyd-ops` | 3 | propostas de gestão de GitHub/ambientes e de versões/cobertura funcional; sem plano diretor | `wyd-ops/`, 2026-11-22 |
| `wyd-bots`, `wyd-client`, `wyd-core`, `wyd-launcher`, `wyd-net`, `wyd-server`, `wyd-sim` | 0 | não planejado; só `.gitkeep` | `find planejamento-1.0.0`, 2026-11-22 |

### 1.2 `wyd-db`, página por pasta

| Pasta | Escritas | Planejadas | Observação |
|---|---:|---:|---|
| `docs/` raiz (`index`, `convencoes`, `glossario`) | 3 | 3 | fecha |
| `docs/arquitetura/` | 10 | 17 | `index` + 9; faltam C4 de componentes, implantação, fronteira, fluxo, modelo conceitual/lógico e retenção |
| `docs/specs/` | 8 | 20 | `index` + 7 escritos; faltam 4 de topo, 5 fluxos e 3 estados |
| `docs/dicionario-dados/` | 21 | 21 (+109 de tabela) | `index`, `como-ler` e os **19 domínios** (2026-11-21); as 109 por tabela são manuais (`M26`) e **109 existem** (2026-11-22) |
| `docs/planos/` | 7 | 10 | `index` + 6 planos escritos, inclusive `plano-de-documentacao` |
| `docs/roadmap/` | 1 | 42 | só `index.md`; 40 issues `WYDDB-nnn` planejadas, nenhuma criada |
| `docs/guia-desenvolvedor/` | 4 | 11 | 2 páginas + `checklist-de-pr` + `index` |
| `docs/guia-administrador/` | 2 | 8 | `index` + `configuracao` |
| `docs/guia-teste/` | 3 | 7 | `index` + `testes-de-sistema` + `testes-de-contrato` |
| `docs/guia-documentacao/` | 2 | 8 | `index` + `estilo-e-linguagem` |
| `migrations/` | 28 | 36 | 2 páginas de seção (por escrever) + 34 arquivos, 28 escritos |
| `src/` | 37 | 59 | `migrate/` e `provision/` têm página por arquivo |
| `tests/` | 0 | 12 | |
| `prompts/` | 0 | 9 | |
| `skills/` | 0 | 9 | |

**Totais planejados:** 147 páginas autorais em `docs/` + 109 por tabela, escritas à mão (`M26`) + 36 de migration + 59 de `src/`
+ 12 de teste + 9 prompts + 9 skills (`wyd-db-1.0.0.md` §4). **Escritas:** **58 páginas autorais com
corpo** em `docs/` (61 arquivos, 3 índices de uma linha), fora de `gerado/`, que reúne **110 arquivos**
(as 109 páginas de tabela e o índice); 28 em `migrations/`, 37 em `src/` (2026-11-22).

### 1.3 Série de migrations

| Indicador | Valor | Fonte |
|---|---:|---|
| Arquivos da série | 34 (29 originais + os adendos `0030`, `0031` e `0032` + os lotes `0033`, `0034`) | `plano-migrations.md` §3 |
| Tabelas | 109 | `plano-migrations.md` §3 (planejado) |
| Arquivos de seed | 6 | idem |
| Tabelas com DDL fechado | 109 de 109 | `ddl-confirmado.md` §1 |
| Tabelas desenhadas e aguardando aprovação | 0 | `ddl-confirmado.md` §1 |
| Arquivos com DDL inteiro fechado | 28 (todos menos os 6 seeds) | `migrations-checklist.md` §4, 2026-11-22 |
| `SCHEMA_VERSION` alvo | 34 | `plano-migrations.md` §6 |

### 1.4 Repositório, GitHub e operação

| Fato | Valor | Fonte |
|---|---|---|
| Visibilidade / branch padrão | público / `develop` | API do GitHub, 2026-11-19 |
| Issues / milestones / Projects | 0 abertas / 6 milestones (M0–M5) / habilitado, project `WYD 1.0.0` nº 10 | API do GitHub, 2026-11-19 |
| Merge permitido | squash, branch apagada no merge | API do GitHub |
| Secret scanning / Dependabot | ativos | API do GitHub |
| `crates/*` | 9 pastas, só `.gitkeep` | `git ls-files` |
| Código Rust real | `deprecated/` com 375 arquivos, **0 versionados** | `git ls-files deprecated` |
| Raiz | sem `Cargo.toml`, `rust-toolchain.toml`, `Makefile` | `ls` |
| CI | `full-workflow`: 1 job (`Get Info`), **verde**, sem build/teste e com `version`/`commit` vazios (`V`: run #14 `44505419869`, job `135495629487`; detalhe em `wyd-ops` §2.1); `docs`: gate + publicação | `.github/workflows/` |
| Workflows de referência no legado | 3 (`ci`, `bench`, `release`) | `deprecated/.github/workflows/` |
| Site de documentação | MkDocs 2.0 (`2.0.dev6`) + Python 3.15; **200 páginas montadas de 5 sistemas**; build, `verificar.py` e `verificar-contagens.py` verdes | `wyd-docs-1.0.0.md` §9.1; `wyd-docs/`, 2026-11-22 |
| Cliente | 7.147 arquivos em `wyd-client/files/`, ignorado | `find` |

### 1.5 Cobertura funcional da `1.0.0`

A `1.0.0` é o jogo completo (§4.2), então a medida que decide a versão é **cobertura funcional**, não
página escrita. Esta seção é o **dono do número agregado**: total de funcionalidades do catálogo e
fração com prova, medidos com data, quando o catálogo existir.

Uma entrada conta quando está `aceita` — fluxo com passo a passo, dono entre os 13 sistemas, prova
(golden, captura datada ou leitura do cliente, com segunda leitura) e versão em que entra; sem prova
ela não conta. O molde da entrada é de `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7, que
ainda é proposta: enquanto não for executada, esta tabela fica em “não medido”, não em estimativa.

| Indicador | Valor (2026-11-20) | Fonte |
|---|---|---|
| Funcionalidades no catálogo | não medido | catálogo funcional não existe (`wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7) |
| Funcionalidades `aceitas` | não medido | idem |
| Cobertura | meta **100%**, não medida | §4.2 |

Nenhum total é estimado antes do levantamento: os 13 sistemas técnicos não são 13 funcionalidades, a
lista de domínios citada pelo dono (classes, skills, refinamento, guerras, cidadania, estados
especiais, montarias, invocações, transformações, magias, quests, guildas) é pauta de levantamento,
não catálogo, e o inventário do legado (`wyd-protocol` §0.4) é baseline de referência, não total
alvo. Contagem de funcionalidade sem catálogo é `H`, nunca estimativa. Cobertura declarada sem data é
`H` também: cada valor desta tabela entra com a data em que foi medido.

---

## 2. O que está decidido e o que está aberto

### 2.1 Decidido (`aceito`)

| ID | Decisão | Data |
|---|---|---|
| `P-01` | 4 schemas por domínio: `content`, `game`, `portal`, `ops`; nada em `public` | 2026-11-17 |
| `P-02` | tabela no singular, `snake_case` | 2026-11-17 |
| `P-03`–`P-22` | PK identity, PK composta em filha, sem UUID, `TIMESTAMPTZ`, dinheiro inteiro, `TEXT`+`CHECK`, `INET`, coluna gerada, `NOT NULL` por padrão, constraint nomeada, índice parcial, FK indexada, trigger só mecânica, view de leitura, papéis separados, migration imutável, sem soft-delete genérico, retenção declarada, PII só em `portal` | 2026-11-17 |
| `ADR-DB-0001` | provisionamento de papel é fase própria, fora da série | 2026-11-17 |
| `ADR-DB-0002` | runner de migrations próprio, com ledger por versão | 2026-11-18 |
| `M1`–`M25`, `A9`–`A16`, `D1`–`D14` | decisões de desenho fechadas, cada uma com evidência em `lacunas.md` | 2026-11-17/18 |

São **dois** ADRs. Não existe catálogo prévio; ADR nasce de decisão tomada, não de planejamento.

### 2.2 Aberto

| Grupo | Abertas | Exemplos |
|---|---:|---|
| Lacunas `B`/`A`/`M` | 0 pendentes | `M27` e `M28` fecharam em 2026-11-21, `M29`, `M30` e `M31` em 2026-11-22; 33 fecharam; 3 seguem `externa` (`A11`, `A20`, `M21`); a lista completa está em `wyd-db/lacunas.md` §1 |
| Externas | 3 | `A11`, `A20`, `M21` |
| Divergências do plano diretor | 0 | `P1`–`P11` fechadas (`wyd-db/lacunas.md` §1 e §3) |
| DDL não desenhado | 0 tabelas | todas as 109 têm coluna desenhada; 34 arquivos de migration para 109 tabelas (34/109, `plano-migrations.md` §3, 2026-11-22); os lotes `0022`, `0025`, `0026`, `0028` e `0029` foram aprovados em 2026-11-21, os `0033` e `0034` em 2026-11-22, e os seis seeds são escritos à mão (`plano-migrations.md` §4) |
| ADR de fronteira pessoa × conta | sem número | citada em `lacunas.md` §0.2 e `rascunho-accounts.md` |

---

## 3. Divergências que precisam de decisão do time

Estas não são tarefas: são contradições entre documentos ou entre documento e realidade. Ficam
registradas aqui até alguém decidir; nenhuma foi corrigida por conta própria.

| # | Divergência | Evidência | Efeito |
|---|---|---|---|
| D1 | `README.md` da raiz afirma desenvolvimento ativo, 10 crates, 34 issues ativas, 58 concluídas, 53 migrations e serviços no ar — **resolvido em 2026-11-19**: reescrito para o estado medido, sem contagem replicada | `README.md` × `git ls-files`, API do GitHub | agente e visitante leem estado falso |
| D2 | `crates/*` estão vazios, mas o README e o CI pressupõem workspace Rust | `find crates` | qualquer `cargo build` na raiz falha |
| D3 | CI tem um job só, e o `run:` monta `$(0.0.0-<sha>)` e `$(<sha>)`, que o shell executa como comando — **medido em 2026-11-19**: o job **não** falha e publica metadado vazio | `V`: run #14 (`44505419869`) e job `Get Info` (`135495629487`) com `success`; log traz `command not found` nas linhas 40 e 44; 0 runs `failure`; detalhe em `wyd-ops` §2.1 | gate verde que não publica `version`/`commit`; não há build, teste, cobertura nem gate de código |
| D4 | O planejamento exige CI composto por actions reutilizáveis; `.github/actions/` tem só `docs-check` | `V`: `git ls-files .github`, 2026-11-19 | meta da `1.0.0` sem base: faltam as actions de Rust, cobertura e segurança de `wyd-ops` §4 |
| D5 | Gestão de trabalho: o roadmap de 40 issues do `wyd-db` não tinha onde viver | `V`, 2026-11-19: `has_projects: true`, 6 milestones (`M0`–`M5`) e project `WYD 1.0.0` nº 10; execução registrada em `wyd-ops` §10 | **resolvido em 2026-11-19**: a estrutura existe (labels, formulários, board); falta criar as issues, que dependem de página com DoD |
| D6 | O plano diretor declara 147 páginas autorais; 84 estavam bloqueadas em 2026-11-18 e **69 seguem** desde que as 19 páginas de domínio ficaram escritas | `proposta-paginas.md` §6 | escrever mais rápido exige fechar lacuna, não escrever autour |
| D7 | Não existe `mkdocs.yml` em lugar nenhum, mas `wyd-docs` é dono do site e o guia do `wyd-db` já prescreve recursos do MkDocs — **resolvido em 2026-11-19**: site em MkDocs 2.0 no `wyd-docs/`, sem `mkdocs.yml`; recursos do MkDocs 1.x convertidos na origem (§2.2 e `LPDOC-14`) | `find . -iname 'mkdocs*.yml'` sem resultado | site não publicável; exigência sem base executável |
| D8 | `docs/` na raiz não existe; no legado, `docs/` é referência — **resolvido em 2026-11-19**: a documentação publicada sai de `planejamento-1.0.0/*/docs/` por `wyd-docs/`, e o legado não é publicado | `ls` | risco de alguém publicar o legado como doc oficial |

---

## 4. Plano de fases

Cada fase termina com um gate verificável. Nenhuma fase começa antes do gate anterior, salvo decisão
explícita do time.

### Fase 0 — Fundação e memória do projeto ✅ (2026-11-19)

- [x] Medir repositório, planejamento, GitHub e CI sem alterar nada.
- [x] Reescrever `AGENTS.md` com hierarquia de fontes e invariantes.
- [x] Registrar panorama neste arquivo.
- [x] Criar memória de trabalho em `.agents/memory/estado-do-projeto.md`.
- [x] Criar skills `auditar-planejamento` e `planejar-sistema`, e o prompt de auditoria.

**Gate:** um agente novo consegue dizer o que está planejado, o que é referência e o que é proibido
sem abrir o legado.

### Fase 1 — Gestão do trabalho no GitHub ✅ (2026-11-19)

Proposta em `wyd-ops/proposta-gestao-github-e-ambientes.md` aprovada e executada (§10 dela).

- [x] Habilitar Projects do repositório e criar o project `WYD 1.0.0` com campos `Sistema`,
      `Fase`, `Prioridade` e `Marco` (além do `Status` padrão).
- [x] Criar as labels `area/*` (13), `estado/*` (3), `origem/legado` e `risco/*` (2), sem duplicar
      tipo em label.
- [x] Criar os milestones `M0 — Fundação` a `M5 — Operação`.
- [x] Criar `.github/ISSUE_TEMPLATE/` com `bug`, `planejamento`, `decisao`, `tarefa` e `config.yml`,
      e `.github/pull_request_template.md` com gate de documentação, prova e escopo.
- [x] Configurar ruleset de `develop` e `main`: PR obrigatório, sem deleção, sem force-push.
      **Sem *required status check***: não há check que reporte (entra na Fase 4).
- [ ] Migrar as 40 issues planejadas de `docs/roadmap/` para issues reais **só depois** de as páginas
      existirem; issue sem DoD verificável não é criada.

**Gate:** uma issue criada pelo formulário chega ao board com sistema, fase e prioridade; o board
mostra o mesmo número de itens que o roadmap do sistema. **Pendente:** views do project e o primeiro
item real — nenhuma issue foi criada.

### Fase 2 — Fechar o `wyd-db`

Ordem derivada de `plano-migrations.md` §9 e `proposta-paginas.md`; não é ordem nova.

- [ ] Fechar as lacunas pendentes com o dono nomeado em `lacunas.md` (bloqueantes primeiro).
- [ ] Escrever as 6 páginas de migration restantes (seeds), quando o dado de origem estiver verificado; os arquivos são escritos à mão, sem gerador (`plano-migrations.md` §4).
- [x] Aprovar os cinco lotes propostos (`0022`, `0025`, `0026`, `0028`, `0029`) em `ddl-confirmado.md` antes da página — aprovados em 2026-11-21; nenhum lote aguarda aprovação.
- [ ] Escrever `specs/` (requisitos, regras, NFR, casos de uso, fluxos, estados, rastreabilidade).
- [x] Concluir as 109 páginas por tabela (`M26`: à mão, sem gerador; **109/109** escritas em 2026-11-22); as **19** páginas de domínio também estão escritas (2026-11-22).
- [ ] Escrever `planos/` (testes, dados, qualidade, métricas, riscos, execução).
- [ ] Escrever `guia-administrador/`, `guia-documentacao/` e o restante de `guia-teste/`.
- [ ] Criar `tests/` e `prompts/`/`skills/` do sistema.
- [ ] Validar o site: `mkdocs build --strict` sem link morto nem diagrama inválido.

**Gate:** 34 páginas de migration (28 escritas, 6 seeds `bloqueada`) + 19 páginas de domínio + 109 páginas por tabela (**109/109** concluídas em 2026-11-22, `wyd-db-1.0.0.md` §6.8); `planos/metricas.md` mede o
fechamento; nenhuma afirmação de comportamento sem `V`; site estrito passa.

### Fase 3 — Planejar os 12 sistemas restantes

- [ ] Aprovar o molde de planejamento por sistema (o de `wyd-db` é a referência, não o texto).
- [x] Escolher o segundo piloto por dependência: **`wyd-protocol`**, decidido em 2026-11-19;
      `wyd-docs` também foi aberto para publicar a documentação oficial.
- [ ] Para cada sistema: dashboard `<sistema>-1.0.0.md`, `docs/` com público e pergunta por página,
      `src/` por unidade de responsabilidade, `tests/` por alvo, `prompts/`, `skills/`.
- [ ] Definir a comunicação entre sistemas (protocolo, mensagens, dono do contrato) antes de planejar
      o interior de cada um.
- [ ] Mapear o que cada sistema consome do `wyd-db` por porta, não por tabela.

**Gate:** todo sistema tem fronteira, entradas/saídas, requisitos com ID, plano de teste e roadmap
com DoD; nenhum sistema depende de tabela de outro sem contrato.

### Fase 4 — Implementação

- [ ] Criar o workspace Rust na raiz com `Cargo.toml`, `rust-toolchain.toml` e `Makefile`.
- [ ] Implementar na ordem de dependência, uma issue por PR, TDD, cobertura ≥ 85%.
- [ ] Substituir o CI stub por workflows reutilizáveis + actions compostas.
- [ ] Rodar o corte vertical do `wyd-db` (`0001`–`0007`, 22 tabelas) antes dos 25 arquivos restantes da série.
- [ ] Provar isolamento entre `portal` e `game` por `permission denied`, não por revisão.

**Gate:** `fmt`, `clippy -D warnings`, testes, cobertura, migrations e build verdes em PR; E2E do
sistema executando contra Postgres real; cobertura funcional de §1.5 medida a cada versão entregue.

### Fase 5 — Operação

- [ ] Ambiente local em Docker Compose; CI efêmero em runner hospedado.
- [ ] Homologação em free tier só para portal e banco, com dado sintético.
- [ ] Produção com servidor de jogo em máquina própria/VPS; free tier não serve (hiberna).
- [ ] Observabilidade e alertas definidos antes do primeiro deploy.

**Gate:** nenhum segredo em repositório; deploy reproduzível; rollback ensaiado; alerta testado.

### 4.1 Série de versões do `wyd-protocol` (`0.1.0` → `1.0.0`)

O `wyd-protocol` entrega em versões menores, abertas na `0.1.0` e fechadas na `1.0.0`. Escopo, gate
e pré-requisito de cada fatia são de `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §9.1 e
não se repetem aqui. Nesta seção a `1.0.0` é a versão do **crate**; o alvo do jogo completo é §4.2, e
as duas coisas não se substituem. Estado `P`: nenhuma versão entregue, nenhum opcode gerado, nenhuma
data.

A série é o eixo de entrega para fatiar sprints, issues e milestones: cada issue pertence a
exatamente uma versão — a que serve o gate dela — e a ordem das versões é ordem de entrega, não
calendário. Quem escreve issue usa o fatiamento de
`planejamento-1.0.0/wyd-protocol/docs/roadmap/fases.md`; como a versão aparece no board foi decidido
pelo dono de operação em 2026-11-20 — milestone por versão, com migração ainda não executada
(`wyd-ops/proposta-versoes-e-cobertura-funcional.md` §4) — e não é decisão desta página. Só o
`wyd-protocol` tem série escrita hoje: o fatiamento `0.1.0` → `1.0.0` dos outros sistemas é decisão
do dono de cada um, na mesma escala de versões já escolhida para o board.

Isso não muda as fases acima: fase é estágio do projeto; versão é fatia de entrega do crate e do
catálogo, dentro da Fase 4.

### 4.2 Critério de saída da `1.0.0` — jogo completo

`1.0.0` aqui não significa “planejamento concluído”: significa **jogo completo**. Declarado pelo dono
em 2026-11-20. A versão sai quando 100% das entradas do catálogo funcional estiverem `aceitas` — cada
uma com fluxo, dono, prova e versão —, sem entrada aberta e com todos os gates de sistema verdes.

O mesmo critério mede as versões menores: uma fatia `0.x` fecha quando as entradas que ela promete
estão `aceitas`, e o jogo fecha quando todas estiverem. A fatia não recalcula cobertura: lê a de §1.5,
que é a única contagem agregada.

| Critério | Como se verifica |
|---|---|
| fatia `0.x` | entradas prometidas pela fatia `aceitas`, lidas de §1.5 |
| cobertura funcional | total e fração de §1.5, medidos do catálogo, com data |
| fluxo por funcionalidade | entrada do catálogo com passo a passo e estado |
| prova por funcionalidade | golden, captura datada ou leitura do cliente, com segunda leitura |
| sistema dono | cada entrada aponta um dos 13 sistemas, sem dono duplicado |
| release | gate da Fase 5: deploy reproduzível e rollback ensaiado |

Consequências para este plano:

1. **Planejar página não é entregar funcionalidade.** Fechar as páginas de um sistema continua sendo
   pré-requisito para implementá-lo, mas não conta cobertura.
2. **O catálogo funcional é pré-requisito do plano de sprint.** Sem ele a `1.0.0` não tem como ser
   medida e nenhuma versão menor pode ser declarada completa — planejamento fechado não supre a
   falta do catálogo.
3. **A contagem é daqui.** Total de funcionalidades e fração coberta são números deste panorama
   (§1.5); o catálogo é dono da entrada e da prova, e nenhuma página recalcula o agregado.
4. **As fases continuam estágios.** A mudança é de alvo e de critério de saída, não de ordem: a Fase 4
   implementa, a Fase 5 opera, e a `1.0.0` exige as duas.

O catálogo funcional não existe hoje: a divisão de dono (`wyd-core`/`wyd-sim`/`wyd-db`) e o molde de
entrada estão propostos em `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.

---

## 5. O que este documento não decide

- Não decide arquitetura de nenhum sistema; o dono é o plano diretor de cada um.
- Não decide data nem prioridade de release; o alvo “jogo completo” está registrado como declaração
  do dono (2026-11-20), com o critério em §4.2.
- Não fecha lacuna: lacuna é fechada em `lacunas.md`, pelo dono nomeado.
- Não estima cobertura funcional: §1.5 mede, com data e fonte, e “não medido” é resposta válida
  enquanto o catálogo não existir.
- Não declara versão entregue: declarar `0.x` ou `1.0.0` exige o critério de §4.2, não a conclusão
  das páginas de planejamento.
- Não cria ADR: ADR nasce de decisão tomada, com “como verificar”.
- Não define o molde dos 12 sistemas restantes; propõe que o de `wyd-db` seja a base.
- Não escolhe hospedagem: a proposta está em `wyd-ops/proposta-gestao-github-e-ambientes.md` e
  depende de aprovação.
- Não mede qualidade de código: não há código de produção para medir.

---

## 6. Próximos passos priorizados por ganho ÷ esforço (2026-11-19, revisto em 2026-11-20)

Ordem por **desbloqueio ÷ esforço**, não pela sequência das fases de §4. `automação` significa
executável agora, sem decisão pendente; `usuário` significa que só o dono decide ou autoriza. Todo
item termina num resultado observável — nenhum é "revisar" ou "amadurecer". O único passo que publicou foi o 2,
dentro da janela declarada pelo dono.

| # | Passo | Desbloqueia | Por que é rápido (o que já existe) | Pré-requisito | Executa | Termina quando |
|---:|---|---|---|---|---|---|
| 1 | Capturar o marcador público atual e registrar o custo de publicação | dimensiona a janela de congelamento e fixa o baseline `D0` do conteúdo público antes de congelar | reutilizou runs e deployments já identificados, os horários de `created_at`/status final e o marcador das duas árvores | nenhum | automação | **feito** (`V`, 2026-11-19): `assets/site.js` respondeu 200 com a URL literal do Highlight.js e as durações #4/#6/#8 foram 49/43/29 s; o registro está em `wyd-docs-1.0.0.md` §7.3 |
| 2 | Declarar a janela sem publicação e executar o checklist de rollback | fecha `LPDOC-01`, a lacuna que segura a publicação contínua do site | os dois runs estão elegíveis, o build dos dois commits passou em Linux (`V`, §7.3) e o checklist de 10 passos já está escrito | declaração de congelamento do dono | usuário declara; automação executa | **feito** (`V`, 2026-11-19): #2 publicou `2378b62b` (`npm/highlight.js`), #4 restaurou `0d765858` (`cdn-release`); deployments `7911886524` e `7911904845` `success`; `LPDOC-01` fechada |
| 3 | Decidir o conteúdo que fica público depois da restauração | evita manter o site atrás de `develop` sem decisão registrada | a consequência já estava no critério de parada de §7.3; a decisão coube em uma frase | nenhum | usuário | **feito** (`V`, 2026-11-19): decisão do dono — o site fica em `0d765858…` até o merge do PR, que publica o tip `aedd1f73…` de conteúdo idêntico; registrado em `wyd-docs-1.0.0.md` §7.3 (linha 3) e §9 |
| 4 | Fechar `LPDOC-12` fixando o build suportado em Linux/container | remove ambiguidade do build local; era a lacuna `P1` que faltava fechar | o erro `PosixPath` está reproduzido e o mesmo comando passa em container nos dois commits | nenhum | usuário (dono: operação) | **feito** (`V`, 2026-11-19): decisão do dono fixou Linux/container como build suportado, com Windows fora de escopo; `lacunas.md` fechou `LPDOC-12` e §7.1 registra que o build roda em Linux e que no Windows falha por `PosixPath` |
| 5 | Medir o comportamento real do CI `full-workflow` como baseline do plano do `wyd-ops` | dá base verificável para a Fase 4 (CI real) e tira a causa de `H` | runs, jobs e log já estavam acessíveis por leitura; faltava medir o efeito real do defeito de shell que `D3` apontava | nenhum | automação | **feito** (`V`, 2026-11-19): run #14 (`44505419869`) e job `Get Info` (`135495629487`) `success`, saída `version`/`commit` vazias; registrado em `wyd-ops` §2 e §2.1 |

**A fila que serve à `1.0.0` é outra.** Os cinco passos acima fecharam fundação e publicação do site;
nenhum deles mede funcionalidade. Com o alvo de §4.2, o primeiro passo que destrava a versão é o
catálogo funcional (§1.5): enquanto ele não existir, a cobertura fica “não medido” e nenhuma versão
pode ser declarada completa, por mais páginas que o planejamento feche. Executar a divisão
`wyd-core`/`wyd-sim`/`wyd-db` e o molde de entrada está proposto em
`wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7 e é passo de `usuário`, não de automação.

**Próxima ação imediata:** as duas filas correm em paralelo. Na documentação, executar o próximo
passo ativo da fila do sistema dono — no `wyd-db`, as **109 das 109** páginas por tabela (`M26`,
2026-11-22) estão escritas em `wyd-db/docs/dicionario-dados/gerado/`; o que resta da árvore são as seis
páginas de migration dos seeds (`bloqueada`) e as pastas `specs/`, `planos/`, `tests/`, `prompts/` e
`skills/`. Na cobertura, a ação é a decisão do catálogo acima; o dicionário fecha planejamento, não a
`1.0.0`. O pacote anterior já foi mesclado no
PR #48 (`e08f7d1`, 2026-11-19), portanto não há commit, push nem PR pendente daquela entrega.

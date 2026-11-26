# Status do planejamento — WYD 1.0.0

> Pergunta: "o que está feito, parcial ou faltando no planejamento do projeto e o que depende do usuário?"
> Público: Produto, arquitetura, desenvolvimento e agentes
> Dono: arquitetura (consolidação) — cada item tem dono próprio citado
> Base: missão "Fechar todo o planejamento do projeto"; `AGENTS.md`; `planejamento-1.0.0/wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-25

**Estado:** **parcial** (2026-11-25). Relatório consolidado do planejamento; não substitui os documentos donos. O planejamento **não está fechado**: cinco itens da §2 seguem `Parcial` e um segue `Contínuo`. As 12 decisões de dono passaram a ter resposta em 2026-11-25 (§11), então o que falta deixou de ser pergunta: é execução, ADR ou terceiro. A lista está na §9.

## 1. Legenda

- **Feito** — artefato existe, gate verde, dono definido.
- **Parcial** — existe mas falta fechamento (captura, medição, aprovação).
- **Faltando** — não existe; ação necessária.
- **N/A** — não se aplica ao estado atual.

## 2. Checklist da missão

| # | Item da missão | Estado | Evidência |
|---|---|---|---|
| 1 | Análise dos achados da revisão anterior | Feito | gate verde (`censos: medidas e citações coincidem`); contagens 471 páginas / 469 fontes / 35 aceito / 433 proposto / 0 sem linha |
| 2 | Planejar todos os gaps | Feito | `lacunas.md` por sistema; `wyd-ops/auditoria-de-organizacao.md` |
| 3 | Estrutura de pastas com `.gitkeep` (todo o projeto) | Feito | `planejamento-1.0.0/*/src/.gitkeep`, `*/tests/.gitkeep` (12 sistemas); `crates/*/` (`.gitkeep`); `wyd-client/files/.gitkeep`; `wyd-db/prompts/.gitkeep` e `wyd-db/skills/.gitkeep`; mapa em `wyd-ops/docs/estrutura-de-pastas-e-modulos.md` |
| 4 | Documentação detalhada por módulo | Feito | `docs/arquitetura/componentes.md` em 12 sistemas (o `wyd-db` usa `componentes-c4.md`) + `docs/modulos.md` em 6 sistemas; os 7 restantes usam `componentes.md` como documento de módulo, sem duplicar `modulos.md` |
| 5 | Docs organizados, limpos, padronizados, sem redundância | Feito | gate valida links/âncoras/censos; o gate de links (`wyd-docs/verificar.py`) confere link, âncora e navegação lateral das **471 páginas montadas**; não publica razão de alcançáveis, então esta célula não afirma percentual; índice local de ADR alinhado ao estado `aceito` |
| 6 | Projeto organizado e mapeado no GitHub | Parcial | `.github/ISSUE_TEMPLATE/*`, `pull_request_template.md`, workflows; **40 páginas de issue `WYDDB-nnn`** e milestones no repositório; o estado do board é do GitHub e **não é medido localmente** (leitura de 2026-11-25), e board/novas issues exigem autorização |
| 7 | Padronização de issues (incl. skills a carregar) | Feito | `skill_principal` obrigatória em lista fechada de 9 skills nos 4 templates; `skills_complementares` opcional na mesma lista; conferido contra `.agents/skills` e cloud skills |
| 8 | Uso das skills pelo agente | Feito (processo) | `.agents/skills/` (2) + cloud skills; `ia-e-skills.md` documenta quando carregar |
| 9 | Planejar skills/docs/testes/código/integração/deploy/monitoramento/segurança | Feito | `plano-de-testes.md` nos 13 sistemas e `componentes.md` em 12 (o `wyd-db` usa `componentes-c4.md`); todos os 13 planos declaram CI, cobertura e monitoramento; `wyd-ops` cobre deploy/segurança; `ia-e-skills.md` |
| 10 | Padrões de qualidade de código/testes/boas práticas/docs/segurança/monitoramento/deploy | Feito | documentado em `plano-de-testes.md` de 13 sistemas e `componentes.md` em 12 sistemas (o `wyd-db` usa `componentes-c4.md`); código só em `wyd-db` |
| 11 | Todos os testes implementados, documentados e funcionando | Parcial | `wyd-db`: 19 testes locais (medido em 2026-11-25 no crate: 13 em `tests/sistema_pg.rs` + 6 em `src/dsn.rs`; contagem de código, fora do gate de docs); demais: 0 (planejados) |
| 12 | SOLID, design patterns, clean code, arquitetura | Parcial | documentado (componentes, contratos); sem código para validar |
| 13 | Padrões de testes, fixtures, mocks, stubs, CI, cobertura | Feito (planejamento) | documentado em `plano-de-testes.md` de 13 sistemas, com seção própria de CI, cobertura e monitoramento; CI de código segue stub (`full-workflow.yml`) — implementação pendente |
| 14 | Boas práticas GitHub Actions, pipelines/workflows complexos, CI/CD, monitoramento | Parcial | `code.yml`, `docs.yml`, `docs-image.yml` (nunca rodou), `full-workflow.yml` (stub) |
| 15 | Reuso de código, modularização, desacoplamento, coesão, baixo acoplamento | Feito (planejamento) | documentado em `componentes.md` em 12 sistemas (o `wyd-db` usa `componentes-c4.md`) |
| 16 | Centralização de fixtures, mocks, stubs, dados de teste | Feito (planejamento) | documentado em `plano-de-testes.md` de 13 sistemas |
| 17 | Baixa complexidade, alta legibilidade/manutenibilidade/escalabilidade | N/A | sem código ainda |
| 18 | Funcionamento de integrações, skills e recursos Freebuff | N/A | nenhuma integração de código hoje |
| 19 | Organizar arquivos de IA (freebuff, skills, integrações) | Feito | `wyd-ops/docs/arquitetura/ia-e-skills.md` |
| 20 | Sugerir melhorias/otimizações/refatorações/atualizações | Contínuo | seção 6 deste relatório |
| 21 | ADRs para todas as decisões importantes | Feito | cada ADR aceito está listado com estado e data no índice global (`planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` §2); a contagem e a origem não são repetidas aqui. Índice + template + registro atualizados |
| 22 | Decisões documentadas, justificadas e aprovadas | Feito | 11 decisões formalizadas em 2026-11-25; `decisoes-registradas.md` §2/§3 atualizados |
| 23 | Funcionalidades do cliente mapeadas e documentadas | Feito | `wyd-client/docs/specs/mapa-de-funcionalidades.md` + inventário |
| 24 | Sistemas de jogo funcionando (guerras, mobs, …) | N/A (planejamento) | `wyd-core/docs/specs/catalogo-funcional.md` (mapa de fronteira) |
| 25 | Documentação de planejamento organizada, números reais, indexada | Feito | gate verde; 471 páginas / 469 fontes / 35 aceito / 433 proposto / 0 sem linha de estado; link, âncora e navegação conferidos em todas as páginas montadas |
| 26 | Skills do agente funcionando, documentadas, testadas, aprovadas | Parcial | documentadas em `ia-e-skills.md`; sem testes automatizados de skill |
| 27 | Documentação atualizada, coerente, sem duplicidade/redundância/erros | Feito (gate) | gate de links/âncoras/censos verde; `componentes.md` é o documento de módulo dos sistemas sem `modulos.md`, sem página duplicada |
| 28 | Documentação rigorosamente padronizada | Feito (gate) | cabeçalho de 5 linhas, `Estado` com data, links relativos e ADR local `aceito`; gate reprova link, âncora, navegação e sintaxe |
| 29 | Remover arquivos obsoletos/revisões | Feito | nenhuma página foi superseded nesta rodada; `deprecated/` e `wyd-docs/.build/` permanecem fora do versionamento; referências ativas revisadas |

## 3. Sistemas — estado do D-04 (organização)

| Sistema | D-04 | Roadmap com DoD | RF/RNF | Lacunas abertas |
|---|---|---|---|---|
| wyd-db | fechado | sim (40 páginas `WYDDB-nnn`) | (piloto) | várias (ver lacunas.md) |
| wyd-admin | fechado | sim (10 fatias) | sim | 0 |
| wyd-portal | fechado | sim (F0–F11) | 33 RF | 0 (9 fechadas) |
| wyd-protocol | fechado | sim (fases) | 6 RF | 2 captura |
| wyd-net | fechado | sim (F0–F4) | 8 RF | 1 medição |
| wyd-core | fechado | sim (F0–F4) | 8 RF | 0 |
| wyd-server | fechado | sim (F0–F3) | 7 RF | 1 peer |
| wyd-sim | fechado | sim (F0–F3) | 7 RF | 0 |
| wyd-launcher | fechado | sim (F0–F3) | 8 RF | 0 |
| wyd-client | fechado | sim (F0–F2) | 7 RF | 3 |
| wyd-bots | fechado | sim (F0–F3) | 6 RF | 1 medição |
| wyd-docs | fechado | sim (F0–F3) | 7 RF | 1 decisão |
| wyd-ops | fechado | sim (F0–F5) | 7 RF | 3 bloqueadas (ver `docs/lacunas.md` §4) |

## 4. Artefatos criados nesta rodada (além das anteriores)

- `planejamento-1.0.0/slice-wyd-db-001-wyd-net-001.md` — artefato de planejamento da primeira fatia de produção aceita (`WYDDB-001` + `WYDNET-001`), com escopo, aceite, gate, evidência, donos, skills e referências cruzadas
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` (índice global de ADRs)
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/template.md`
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/decisoes-registradas.md`
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/ia-e-skills.md`
- `planejamento-1.0.0/wyd-ops/docs/github-mapping.md`
- `planejamento-1.0.0/wyd-client/docs/specs/mapa-de-funcionalidades.md`
- `planejamento-1.0.0/wyd-core/docs/specs/catalogo-funcional.md`
- `.github/ISSUE_TEMPLATE/*` — campo "Skills que o agente deve carregar" (4 templates)
- `planejamento-1.0.0/status-planejamento.md` (este arquivo)
- `planejamento-1.0.0/wyd-ops/docs/modulos.md`, `wyd-core/docs/modulos.md`, `wyd-client/docs/modulos.md`, `wyd-bots/docs/modulos.md`, `wyd-docs/docs/modulos.md`, `wyd-db/docs/modulos.md`
- `planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` — estrutura completa de pastas com `.gitkeep`, mapa de documentação por módulo e cruzamento com auditoria/especificação
- `planejamento-1.0.0/wyd-ops/docs/planos/pipeline-ci-cd.md` — planejamento de CI/CD com workflows, jobs, gates, required checks, boas práticas de GitHub Actions e skill-loading
- `planejamento-1.0.0/wyd-ops/docs/planos/estrategia-de-testes.md` — estratégia transversal de testes com pirâmide, centralização de fixtures/mocks/stubs, cobertura e integração com CI
- `planejamento-1.0.0/wyd-ops/docs/planos/baseline-seguranca-monitoramento-deploy.md` — baseline de segurança, monitoramento e deploy com gates e decisões abertas
- `planejamento-1.0.0/wyd-core/docs/specs/sistemas-de-jogo.md` — mapa de sistemas de jogo (guerras, mobs, NPCs, canais, skills, classes, quests, armas, armaduras, lojas, mercado, crafting, magias, feitiços, poções, pets, companheiros, montarias) com donos, fontes, testes e DoD
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/candidatos.md` — backlog de decisões candidatas a ADR, gatilhos, evidência mínima e processo
- `planejamento-1.0.0/wyd-ops/docs/planos/verificacao-de-skills-e-integracoes.md` — verificação de skills e integrações do Freebuff, matriz de skill-loading e anti-padrões
- `planejamento-1.0.0/wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` — tarefa concreta de medição de `LNET-01` e `LBOT-03`, com dono, método, critérios de aceite, entregáveis e riscos
- `planejamento-1.0.0/wyd-ops/docs/lacunas.md` — registro único de lacunas do `wyd-ops`, com 2 fechadas, 3 bloqueadas e a lista consolidada de perguntas/recomendações humanas
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/adr-ops-0001-workspace-cargo-unico.md` — ADR da decisão D-05 = A, com verificação e consequências
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/adr-ops-0002-hospedagem-producao.md` — ADR da decisão Q-OPS-2 = A, com aplicação ainda pendente
- `.github/ISSUE_TEMPLATE/*` — campo "Skills que o agente deve carregar" obrigatório e padronizado conforme `AGENTS.md` §7
- `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` e `template.md` — índice/template de ADR com cruzamento à auditoria e à especificação
- `planejamento-1.0.0/wyd-{net,core,server,admin,portal,docs,client}/docs/arquitetura/adr/` (7 ADRs formalizados como `aceito` em 2026-11-25; estado e contagem aceita são do índice global de ADR)

## 5. Decisões necessárias do usuário (não automatizáveis)

Skill a carregar em cada item (AGENTS.md §5): `planejar-sistema` (abrir/fechar sistema),
`auditar-planejamento` (auditoria antes de aceitar), `documentation-writer` (redação).

**Formalizados em 2026-11-25 por aprovação do dono:** 9 ADRs (`ADR-NET-0001`, `ADR-CORE-0001`,
`ADR-SRV-0001`, `ADR-ADM-0001`, `ADR-POR-0001`, `ADR-DOC-0001`, `ADR-CLI-0001`, `ADR-OPS-0001`,
`ADR-OPS-0002`) → `aceito`; esta lista é o registro do lote de 2026-11-25, e o total aceito é o do índice global de ADR; as decisões de sistema aprovadas estão em `decisoes-registradas.md` §2.
**Não estão fechadas:** `LNET-01` e `LBOT-03` (medição bloqueada por ausência de runtime; a prontidão de
2026-11-25 mediu 0 manifestos em net/server/bots e 0 listeners), `LCLI-02`/`LCLI-05` (direitos) e as
perguntas consolidadas em `docs/lacunas.md` §4.

1. **Organizar o board e as issues restantes no GitHub** — o repositório tem as **40 páginas `WYDDB-nnn`**
   medidas pelo gate; o estado do board é do GitHub, **não medido localmente** (leitura de
   2026-11-25), e a convenção está em `github-mapping.md`. Recomendação: montar o
   board e criar as fatias F0 dos demais sistemas com as labels da §2 do mapping. Skill:
   `planejar-sistema`. **Nada foi criado nesta rodada** (sem autorização).
2. **Segredos e deploy** — valores de DSN/tokens só via secret do GitHub ou `.env` local; nunca em
   chat. Deploy/release/push exigem pedido explícito. Skill: `planejar-sistema`.
3. **Imagem do site** — `.github/workflows/docs-image.yml` versionado mas nunca rodou; publicar
   requer aprovação (ver `wyd-ops/plano-imagem-e-compose-do-site.md`). Skill: `planejar-sistema`.
4. **Primeira fatia de código** — D-01 mantém `wyd-db` como foco; recomendação: `WYDDB-001`
   (baseline do crate) e, em paralelo, `WYDNET-001`. Skill: `planejar-sistema`.
5. **Roadmaps de sistema** — `wyd-protocol` e `wyd-admin` têm roadmap com DoD após a segunda passada;
   atualizar a página quando uma fatia fechar. Skill: `planejar-sistema`.
6. **net/bots medições pendentes** — `LNET-01` (limites) e `LBOT-03` (carga) continuam como medição
   mesmo após aprovação da política; a tarefa concreta está em `docs/planos/medicoes-lnet-01-lbot-03.md`
   e os valores só congelam após a primeira execução. Skill: `auditar-planejamento`.

## 6. Sugestões (melhorias/otimizações/refatorações)

- Renomear `wyd-portal/docs/roadmap/roadmap.md` para padrão único (wyd-db usa `index.md`+`fases.md`+
  `WYDDB-nnn.md`). Definir um padrão único para todos os sistemas.
- Criar `planos/metricas.md` no `wyd-db` (citado, não escrito) para percentual de progresso.
- Adicionar `docs/planos/metricas.md` por sistema (hoje só `wyd-docs` tem).
- Manter o backlog de candidatos a ADR em `docs/arquitetura/adr/candidatos.md`; não criar ADR sem decisão aprovada.
- Rodar `auditar-planejamento` em cada sistema antes de aceitar (skill disponível).
- Padronizar fronteira/contratos dos sistemas que ainda têm "parcial" (wyd-ops, wyd-client).

## 7. Verificação final

- `docker compose --profile gate run --rm gate` → `censos: medidas e citações coincidem`.
- Contagens: 471 páginas / 469 fontes / 433 proposto / 35 aceito / 0 sem linha de estado.
- Nenhum commit/push/PR/issue/deploy realizado nesta rodada.

## 8. Fechamentos da terceira passada (2026-11-25)

| Item | Evidência concreta |
|---|---|
| Pastas reservadas completas | `planejamento-1.0.0/wyd-db/prompts/.gitkeep` e `planejamento-1.0.0/wyd-db/skills/.gitkeep` criados; `find planejamento-1.0.0 -type d -empty` retorna 0 |
| Links e navegação | `wyd-docs/verificar.py` confere link interno, âncora e navegação lateral em **471 páginas montadas**; a razão alcançáveis/total não é publicada por nenhum gate, então não há percentual nem contagem de órfãos nesta célula |
| Índices de ADR coerentes | cada sistema que registra ADR mantém índice local com o estado e o link do arquivo; a contagem de aceitos é do índice global (`planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` §2), que não é repetida aqui |
| Issues com skill verificável | `skill_principal` obrigatória nos 4 templates, com 9 opções fechadas; `skills_complementares` opcional; lista conferida contra `.agents/skills` (2) e cloud skills (7) |
| Planos de teste completos | 13 de 13 `plano-de-testes.md` com `## CI, cobertura e monitoramento`, comando de CI, piso de cobertura e métrica/log por sistema; `LNET-01`/`LBOT-03` sem limiar inventado |
| Numeração e contagens | naquela passada, `status-planejamento.md`, `wyd-docs-1.0.0.md` e `wyd-portal-1.0.0.md` citavam 400/398/31/366/0; as contagens correntes são as do topo desta página, medidas pelo gate |

**Placar do ledger, medido:** lacunas medidas: 118 = 109 fechadas + 9 bloqueadas — a conta é a das tabelas de `lacunas.md`, deduplicada por id e conferida por `wyd-docs/verificar-lacunas.py` no gate; `LCLI-02` fechou em 2026-11-25 com lista medida, manifesto e verificador, e as 9 bloqueadas são `LPRT-03`, `LPRT-06`, `LCLI-03`, `LCLI-05`, `LNET-01`, `LBOT-03`, `A11`, `A20` e `M21` (registro em `wyd-db/lacunas.md` e `wyd-db/pendencias-de-fechamento.md`).

**O que continua aberto:** `LPRT-03`/`LPRT-06` **bloqueadas por artefato inexistente** (sem `ECAP`/`PEER-A1`), `LNET-01`/`LBOT-03` autorizados com execução pendente, `LCLI-05` aguardando o titular, `LCLI-03` aguardando o operador e as capturas, `A11` na mesma janela de captura e `A20`/`M21` externas — o mapa está em `wyd-ops/auditoria-de-organizacao.md` §8; `LCLI-02` fechou em 2026-11-25 com lista, manifesto e verificador. As decisões de dono têm resposta: `OPS-03.1`–`OPS-03.5` aprovadas na forma, `D-06` = A, `Q-SERIE-1` = B implementada, `OPS-05` **executada em 2026-11-26** e `OPS-04` adiada. `Q-SEP-1` = A′ foi decidida e aplicada, então não resta escolha executável em aberto. A lista consolidada é `wyd-ops/docs/lacunas.md` §4.

## Issues padronizadas por página (2026-11-25)

Medido no repositório: **101 páginas de issue** — 40 `WYDDB-nnn` e 61 por fatia nos 11 sistemas —,
cada uma com cabeçalho de 5 linhas, `**Estado:**`, dependências, DoD observável, fora de escopo e bloco
de skills (`skill_principal` + `skills_complementares`) na lista fechada de nove skills. O inventário por
sistema e a regra de numeração estão em `wyd-ops/docs/github-mapping.md` §7; os índices locais são o
`docs/roadmap/index.md` de cada sistema (no `wyd-portal`, o `roadmap.md`). **111** issues existem no GitHub:
as **40** `WYDDB-nnn`, as **61** fatias (#129–#189) e **10** de lacuna (#190–#199); nenhuma aguarda
`OPS-05`.

## 9. Por que o planejamento ainda não está fechado (2026-11-25)

O carimbo `fechado` no topo desta página seria falso: cinco itens da §2 seguem `Parcial` e um segue
`Contínuo`. O que falta é execução, artefato de terceiro ou um “sim” do dono — **nenhum item se resolve
escrevendo outra página**.

| Bloqueio | O que falta | Dono | Registro |
|---|---|---|---|
| `OPS-03.4`, `OPS-03.5` | escrever `ADR-OPS-0003`/`0004` antes de implementar; O1–O3 já aprovados na forma | arquitetura + `wyd-ops` | `wyd-ops/docs/lacunas.md` §4 |
| `OPS-04` | aplicar `Q-OPS-2`: provedor, VPS, backup e RPO/RTO — adiada para depois do primeiro deploy interno | produto + `wyd-ops` | `wyd-ops/docs/arquitetura/adr/adr-ops-0002-hospedagem-producao.md` |
| `OPS-05` | issues e labels executados em 2026-11-26 (#129–#199, `area/*`); o board do projeto segue não montado | produto | `wyd-ops/docs/github-mapping.md` §7 |
| `D-06` | já decidida em A (frente `wyd-db`); falta executar o ato 4 | produto | `wyd-ops/docs/prioridade-do-projeto.md` §0.1 |
| `CAP-38`/`CAP-39` | p95 da leitura de personagem e boot medidos; falta código | `wyd-db` | `wyd-db/tests/specs/capacidades.md` §7 |
| `LPRT-03`, `LPRT-06`, `A11` | captura do cliente 7662 sobre `ECAP` | `wyd-server` + `wyd-client` | `wyd-protocol/lacunas.md` §1 |
| `LNET-01`, `LBOT-03` | executar a medição já autorizada | QA + `wyd-ops` | `wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` |
| `LCLI-03`, `LCLI-05` | operador da ficha e resposta do titular | produto + revisão jurídica | `wyd-client/lacunas.md` §2 |
| `A20`, `M21` | telas e rotas do portal nas fatias | `wyd-portal` | `wyd-db/lacunas.md` §1 |
| catálogo funcional | medir total e cobertura | `wyd-core` + `wyd-sim` + `wyd-db` | `wyd-1.0.0.md` §1.5 |
| placar do `wyd-ops` | o ledger vive em `wyd-ops/docs/lacunas.md` e `wyd-docs/verificar-lacunas.py` lê `<sistema>/lacunas.md`; as decisões OPS ficam fora do placar impresso pelo gate | `wyd-ops` | `wyd-docs/verificar-lacunas.py` |

**Atualização de 2026-11-26:** `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `Q-ALVO-1`–`03`, `OPS-03.1`–`OPS-03.3` e
`OPS-05` têm resposta do dono registrada (§11); `Q-SERIE-1` foi implementada e provada como ciclo SQL,
`Q-SEP-1` = A′ renomeou os 28 arquivos para `_`, **`CAP-37` foi fechada por `runner_pg` (5/5 em PG17)** e
`OPS-05` publicou labels e as 71 issues novas. Das linhas acima, o que resta é execução
(`LNET-01`/`LBOT-03`, board), `CAP-38`/`CAP-39`, ADR (`OPS-03.4`/`OPS-03.5`), terceiro (`LCLI-05`) ou
adiamento (`OPS-04`).

## 10. Correções desta revisão (2026-11-25)

| Defeito encontrado | Correção aplicada |
|---|---|
| esta página declarava `fechado` com itens `Parcial` | estado passa a `parcial`, com a lista da §9 |
| `auditoria-de-organizacao.md` §4 dizia que 11 sistemas não tinham o carimbo, contra o §2 | §4 remete à segunda passada e ao §3 |
| `Q-SERIE-1` e `Q-SEP-1` apareciam com opções e recomendação diferentes em três páginas do `wyd-ops` | as três páginas citam o documento dono (`serie-e-regras.md` §9 e §9.1) em vez de repetir as letras |
| `verificar-contagens.py` emitia `SyntaxWarning` de escape inválido na regex de data | regex passou a `raw string`, com o mesmo comportamento |

## 11. Respostas de dono registradas em 2026-11-25 (segunda rodada)

| Decisão | Resposta | Registro canônico |
|---|---|---|
| `D-06` | A — finalizar o `wyd-db` | `wyd-ops/docs/prioridade-do-projeto.md` §0.1 |
| `Q-SERIE-1` | B implementada: 28 `down` e ciclo PG17 via `psql` passaram | `wyd-db/migrations/serie-e-regras.md` §9.2 |
| `Q-SEP-1` | **A′ decidida e aplicada**: 28 arquivos renomeados para `_`; todos parseiam | `wyd-db/migrations/serie-e-regras.md` §9.1 |
| `Q-ALVO-1`–`03` | A1 (porta em `127.0.0.1`), B1 (fixture contígua) e papel de teste; `runner_pg` verde 5/5 | `wyd-db/tests/alvos/runner-pg.md` |
| `OPS-03.1`–`OPS-03.3` | aprovados na forma | `wyd-ops/docs/lacunas.md` §4 e §5 |
| `OPS-03.4`, `OPS-03.5` | aprovados na forma, com ADR antes de implementar | `wyd-ops/docs/lacunas.md` §4 |
| `OPS-05` | fatias, lacunas e labels publicados em 2026-11-26 (#129–#199); board pendente | `wyd-ops/docs/github-mapping.md` §7 |
| `OPS-04` | adiada para depois do primeiro deploy interno | `wyd-ops/docs/lacunas.md` §4 |

## 12. Publicação no GitHub (2026-11-26)

O trabalho local foi publicado sem reescrever histórico: branch `docs/fechar-planejamento-1.0.0`, commits
`387d9a3` (launcher), `aadb56d` (wyd-db), `33ec823` (documentação), `8b16490` (estado) e `0b9056d` (CI), com PR
[#128](https://github.com/lucascantarelli/wyd/pull/128) contra `develop` — **sem merge**. O namespace de
label ficou em `area/*` (decisão de 2026-11-25), e `OPS-05` criou:

| Artefato | Resultado |
|---|---|
| Labels novas | `estado/proposta`, `estado/aceita`, `fase/1`–`fase/6`, além das `area/*` e `estado/bloqueada` existentes |
| 61 fatias | [#129](https://github.com/lucascantarelli/wyd/issues/129)–[#189](https://github.com/lucascantarelli/wyd/issues/189), uma por página de `docs/roadmap/` |
| 10 lacunas | [#190](https://github.com/lucascantarelli/wyd/issues/190)–[#199](https://github.com/lucascantarelli/wyd/issues/199): `CAP-38`, `CAP-39`, os seis seeds `0016`/`0017`/`0018`/`0020`/`0023`/`0027`, `OPS-04` e `LCLI-05` |
| Board do Project | não montado nesta rodada |

Cada issue nova tem documento de origem com link, critérios de aceite observáveis e a seção
"Skills a carregar" com `skill_principal` e complementares. A criação foi idempotente: o script consulta os
títulos existentes antes de criar e a reexecução não duplica issue.

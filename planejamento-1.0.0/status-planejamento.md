# Status do planejamento — WYD 1.0.0

> Pergunta: "o que está feito, parcial ou faltando no planejamento do projeto e o que depende do usuário?"
> Público: Produto, arquitetura, desenvolvimento e agentes
> Dono: arquitetura (consolidação) — cada item tem dono próprio citado
> Base: missão "Fechar todo o planejamento do projeto"; `AGENTS.md`; `planejamento-1.0.0/wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-26

**Estado:** **parcial** (2026-11-26). Relatório consolidado do planejamento; não substitui os documentos donos. O planejamento **não está fechado**: quatro itens da §2 seguem `Parcial`, um segue `Contínuo` e dois são `N/A`; o mapeamento do GitHub e a verificação automatizada de skills fecharam em 2026-11-26 (§14). O que falta deixou de ser pergunta de desenho: é execução, implementação ou terceiro. A lista está na §9.

## 1. Legenda

- **Feito** — artefato existe, gate verde, dono definido.
- **Parcial** — existe mas falta fechamento (captura, medição, aprovação).
- **Faltando** — não existe; ação necessária.
- **N/A** — não se aplica ao estado atual.

## 2. Checklist da missão

| # | Item da missão | Estado | Evidência |
|---|---|---|---|
| 1 | Análise dos achados da revisão anterior | Feito | gate verde (`censos: medidas e citações coincidem`); contagens 500 páginas / 498 fontes / 54 aceito / 440 proposto / 0 sem linha |
| 2 | Planejar todos os gaps | Feito | `lacunas.md` por sistema; `wyd-ops/auditoria-de-organizacao.md` |
| 3 | Estrutura de pastas com `.gitkeep` (todo o projeto) | Feito | `planejamento-1.0.0/*/src/.gitkeep`, `*/tests/.gitkeep` (12 sistemas); `crates/*/` (`.gitkeep`); `wyd-client/files/.gitkeep`; `wyd-db/prompts/.gitkeep` e `wyd-db/skills/.gitkeep`; mapa em `wyd-ops/docs/estrutura-de-pastas-e-modulos.md` |
| 4 | Documentação detalhada por módulo | Feito | `docs/arquitetura/componentes.md` em 12 sistemas (o `wyd-db` usa `componentes-c4.md`) + `docs/modulos.md` em 6 sistemas; os 7 restantes usam `componentes.md` como documento de módulo, sem duplicar `modulos.md` |
| 5 | Docs organizados, limpos, padronizados, sem redundância | Feito | gate valida links/âncoras/censos; o gate de links (`wyd-docs/gate/checks/links.py`) confere link, âncora e navegação lateral das **500 páginas montadas**; não publica razão de alcançáveis, então esta célula não afirma percentual; índice local de ADR alinhado ao estado `aceito` |
| 6 | Projeto organizado e mapeado no GitHub | Feito | `.github/ISSUE_TEMPLATE/*`, `pull_request_template.md`, workflows; **121 issues** no repositório (40 `WYDDB`, 61 fatias, 10 lacunas e 10 `ADM` #208–#217) e **121 itens** no board `WYD 1.0.0` nº 10, com 5 views e todos os campos `Sistema`/`Status`/`Fase`/`Prioridade`/`Marco`; prova na §14 |
| 7 | Padronização de issues (incl. skills a carregar) | Feito | `skill_principal` obrigatória em lista fechada de 12 skills nos 4 templates; `skills_complementares` opcional na mesma lista; a igualdade entre a §3 do template e os formulários é conferida pelo gate `skills` |
| 8 | Uso das skills pelo agente | Feito (processo) | `.agents/skills/` (2) + cloud skills; `ia-e-skills.md` documenta quando carregar |
| 9 | Planejar skills/docs/testes/código/integração/deploy/monitoramento/segurança | Feito | `plano-de-testes.md` nos 13 sistemas e `componentes.md` em 12 (o `wyd-db` usa `componentes-c4.md`); todos os 13 planos declaram CI, cobertura e monitoramento; `wyd-ops` cobre deploy/segurança; `ia-e-skills.md` |
| 10 | Padrões de qualidade de código/testes/boas práticas/docs/segurança/monitoramento/deploy | Feito | documentado em `plano-de-testes.md` de 13 sistemas e `componentes.md` em 12 sistemas (o `wyd-db` usa `componentes-c4.md`); código só em `wyd-db` |
| 11 | Todos os testes implementados, documentados e funcionando | Parcial | `wyd-db`: 19 testes locais (medido em 2026-11-25 no crate: 13 em `tests/sistema/sistema_pg.rs` + 6 em `src/dsn.rs`; contagem de código, fora do gate de docs); demais: 0 (planejados) |
| 12 | SOLID, design patterns, clean code, arquitetura | Parcial | documentado (componentes, contratos); sem código para validar |
| 13 | Padrões de testes, fixtures, mocks, stubs, CI, cobertura | Parcial (F0+F1) | `plano-de-testes.md` de 13 sistemas; CI de código, banco e docs implementado em F0+F1 (2026-11-26), cobertura informativa com retenção de 7 dias; E2E e observabilidade na F4 |
| 14 | Boas práticas GitHub Actions, pipelines/workflows complexos, CI/CD, monitoramento | Parcial (F0+F1) | `code.yml` (matriz), `ci-db.yml` (service containers), `docs.yml`, `docs-image.yml` (Buildx/GHCR publicado), `security.yml` (actionlint), `code-stable.yml` (executado), `release.yml`; prova real no PR #205 e runs de 2026-11-26; Slack e métricas na F4; deploy na F3 |
| 15 | Reuso de código, modularização, desacoplamento, coesão, baixo acoplamento | Feito (planejamento) | documentado em `componentes.md` em 12 sistemas (o `wyd-db` usa `componentes-c4.md`) |
| 16 | Centralização de fixtures, mocks, stubs, dados de teste | Feito (planejamento) | documentado em `plano-de-testes.md` de 13 sistemas |
| 17 | Baixa complexidade, alta legibilidade/manutenibilidade/escalabilidade | N/A | sem código ainda |
| 18 | Funcionamento de integrações, skills e recursos Freebuff | N/A | nenhuma integração de código hoje |
| 19 | Organizar arquivos de IA (freebuff, skills, integrações) | Feito | `wyd-ops/docs/arquitetura/ia-e-skills.md` |
| 20 | Sugerir melhorias/otimizações/refatorações/atualizações | Contínuo | seção 6 deste relatório |
| 21 | ADRs para todas as decisões importantes | Feito | cada ADR aceito está listado com estado e data no índice global (`planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` §2); a contagem e a origem não são repetidas aqui. Índice + template + registro atualizados |
| 22 | Decisões documentadas, justificadas e aprovadas | Feito | 12 decisões respondidas em 2026-11-25; O4/O5 formalizadas em `ADR-OPS-0011`, `ADR-OPS-0007` revisado e `ADR-OPS-0012` criada em 2026-11-26 |
| 23 | Funcionalidades do cliente mapeadas e documentadas | Feito | `wyd-client/docs/specs/mapa-de-funcionalidades.md` + inventário |
| 24 | Sistemas de jogo funcionando (guerras, mobs, …) | N/A (planejamento) | `wyd-core/docs/specs/catalogo-funcional.md` (mapa de fronteira) |
| 25 | Documentação de planejamento organizada, números reais, indexada | Feito | gate verde; 500 páginas / 498 fontes / 54 aceito / 440 proposto / 0 sem linha de estado; link, âncora e navegação conferidos em todas as páginas montadas |
| 26 | Skills do agente funcionando, documentadas, testadas, aprovadas | Feito | documentadas em `ia-e-skills.md` e no inventário; `wyd-docs/tests/test_skill_contract.py` compara a lista do gate, o template, os quatro formulários e as duas skills em disco; 4/4 verdes em 2026-11-26 (§14) |
| 27 | Documentação atualizada, coerente, sem duplicidade/redundância/erros | Feito (gate) | gate de links/âncoras/censos verde; `componentes.md` é o documento de módulo dos sistemas sem `modulos.md`, sem página duplicada |
| 28 | Documentação rigorosamente padronizada | Feito (gate) | cabeçalho de 5 linhas, `Estado` com data, links relativos e ADR local `aceito`; gate reprova link, âncora, navegação e sintaxe |
| 29 | Remover arquivos obsoletos/revisões | Feito | `briefing-de-decisoes-1.0.0.md` removido e suas respostas apontadas para a pauta/lacunas; `wyd-ops/plano-ci-real.md` absorvido por `wyd-ops/docs/devops/index.md` e removido; `deprecated/` e `wyd-docs/.build/` seguem fora do versionamento; links e índices revisados |

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
| wyd-ops | fechado | sim (F0–F5) | 7 RF | 2 bloqueadas (ver `docs/lacunas.md` §4) |

## 4. Artefatos criados nesta rodada (além das anteriores)

- `planejamento-1.0.0/wyd-ops/docs/planos/fatia-001-wyd-db-wyd-net.md` — artefato de planejamento da primeira fatia de produção aceita (`WYDDB-001` + `WYDNET-001`), com escopo, aceite, gate, evidência, donos, skills e referências cruzadas
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
- `planejamento-1.0.0/wyd-ops/docs/lacunas.md` — registro único de lacunas do `wyd-ops`, com 3 fechadas, 2 bloqueadas e a lista consolidada de perguntas/recomendações humanas
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

1. **Executar as fatias do GitHub** — o mapeamento está fechado: 121 issues e 121 itens no board
   `WYD 1.0.0`, com as 10 `ADM` criadas em 2026-11-26. O que resta é executar as fatias na ordem
   de D-06, não criar mais estrutura. Skill: `planejar-sistema`; `writing-plans` antes de tocar código.
2. **Segredos e deploy** — valores de DSN/tokens só via secret do GitHub ou `.env` local; nunca em
   chat. Deploy/release/push exigem pedido explícito. Skill: `planejar-sistema`.
3. **Imagem do site** — `docs-image` publicou em `develop` em 2026-11-26 (run `45827609542`), com `:SHA`/`:develop`/`:latest`, provenance/SBOM e smoke 200; `make rollback` foi provado com duas tags publicadas. Restam runtime nginx e produção (ver `wyd-ops/plano-imagem-e-compose-do-site.md`). Skill: `planejar-sistema`.
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
- Padronizar a fronteira/contratos do `wyd-client`, que ainda tem direitos parciais.

## 7. Verificação da terceira passada (2026-11-25)

- `docker compose --profile gate run --rm gate` → `censos: medidas e citações coincidem`.
- Contagens naquela passada: 494 páginas / 492 fontes / 443 proposto / 48 aceito / 0 sem linha de estado.
- Naquela rodada não houve commit, push, PR, issue ou deploy; a rodada de 2026-11-26 está nas §12–§14.

## 8. Fechamentos da terceira passada (2026-11-25)

| Item | Evidência concreta |
|---|---|
| Pastas reservadas completas | `planejamento-1.0.0/wyd-db/prompts/.gitkeep` e `planejamento-1.0.0/wyd-db/skills/.gitkeep` criados; `find planejamento-1.0.0 -type d -empty` retorna 0 |
| Links e navegação | `wyd-docs/gate/checks/links.py` confere link interno, âncora e navegação lateral em **500 páginas montadas**; a razão alcançáveis/total não é publicada por nenhum gate, então não há percentual nem contagem de órfãos nesta célula |
| Índices de ADR coerentes | cada sistema que registra ADR mantém índice local com o estado e o link do arquivo; a contagem de aceitos é do índice global (`planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` §2), que não é repetida aqui |
| Issues com skill verificável | `skill_principal` obrigatória nos 4 templates, com 9 opções fechadas; `skills_complementares` opcional; lista conferida contra `.agents/skills` (2) e cloud skills (7) |
| Planos de teste completos | 13 de 13 `plano-de-testes.md` com `## CI, cobertura e monitoramento`, comando de CI, piso de cobertura e métrica/log por sistema; `LNET-01`/`LBOT-03` sem limiar inventado |
| Numeração e contagens | naquela passada, `status-planejamento.md`, `wyd-docs-1.0.0.md` e `wyd-portal-1.0.0.md` citavam 400/398/31/366/0; as contagens correntes são as do topo desta página, medidas pelo gate |

**Placar do ledger, medido:** lacunas medidas: 120 = 109 fechadas + 11 bloqueadas — a conta é a das tabelas de `lacunas.md`, deduplicada por id e conferida por `wyd-docs/gate/checks/lacunas.py` no gate; `LCLI-02` fechou em 2026-11-25 com lista medida, manifesto e verificador, e as 11 bloqueadas são `LPRT-03`, `LPRT-06`, `LCLI-03`, `LCLI-05`, `LCLI-06`, `LCLI-07`, `LNET-01`, `LBOT-03`, `A11`, `A20` e `M21` (registro em `wyd-db/lacunas.md` e `wyd-db/pendencias-de-fechamento.md`).

**O que continua aberto:** `LPRT-03`/`LPRT-06` **bloqueadas por artefato inexistente** (sem `ECAP`/`PEER-A1`), `LNET-01`/`LBOT-03` autorizados com execução pendente, `LCLI-05` aguardando o titular, `LCLI-03` aguardando o operador e as capturas, `A11` na mesma janela de captura e `A20`/`M21` externas — o mapa está em `wyd-ops/auditoria-de-organizacao.md` §8; `LCLI-02` fechou em 2026-11-25 com lista, manifesto e verificador. As decisões de dono têm resposta: `OPS-03.1`–`OPS-03.5` aprovadas e formalizadas (`ADR-OPS-0011` e `ADR-OPS-0007` revisado), `D-06` = A, `Q-SERIE-1` = B implementada, `OPS-05` **fechada em 2026-11-26** (121 issues e board de 121 itens) e `OPS-04` adiada. `Q-SEP-1` = A′ foi decidida e aplicada; não resta escolha de desenho em aberto. A pauta canônica é `wyd-ops/docs/pauta-de-decisoes-pendentes.md`; o estado fica em `wyd-ops/docs/lacunas.md` §4.

## Issues padronizadas por página (2026-11-25; padrão único em 2026-11-26)

Medido no repositório: **111 páginas de issue** — 40 `WYDDB-nnn`, 61 fatias nos 11 sistemas e 10
páginas `ADM-nn` no `wyd-admin` —, cada uma com cabeçalho de 5 linhas, `**Estado:**`, o campo
`**Gate de verificação:**`, dependências, DoD observável, fora de escopo e bloco de skills
(`skill_principal` + `skills_complementares`) na lista fechada de doze skills do
`template-de-issue.md` §3. O inventário por sistema e a regra de numeração estão em
`wyd-ops/docs/github-mapping.md` §7; os índices locais são o `docs/roadmap/index.md` de cada sistema
(no `wyd-portal`, o `roadmap.md`). No GitHub existem **121** issues: as **40** `WYDDB-nnn`, as **61**
fatias (#129–#189), **10** de lacuna (#190–#199) e as **10** `ADM-nn` (#208–#217). O board tem os
**121** itens; a prova está na §14.

## 9. Por que o planejamento ainda não está fechado (2026-11-26)

O carimbo `fechado` no topo desta página seria falso: quatro itens da §2 seguem `Parcial`, um segue
`Contínuo` e dois são `N/A`. O que falta é execução, artefato de terceiro ou um “sim” do dono — **nenhum
item se resolve escrevendo outra página**.

| Bloqueio | O que falta | Dono | Registro |
|---|---|---|---|
| `OPS-04` | aplicar `Q-OPS-2`: provedor, VPS, backup e RPO/RTO — adiada para depois do primeiro deploy interno | produto + `wyd-ops` | `wyd-ops/docs/arquitetura/adr/adr-ops-0002-hospedagem-producao.md` |
| `D-06` | já decidida em A (frente `wyd-db`); falta executar o ato 4 | produto | `wyd-ops/docs/prioridade-do-projeto.md` §0.1 |
| `CAP-38`/`CAP-39` | p95 da leitura de personagem e boot medidos; falta código | `wyd-db` | `wyd-db/tests/specs/capacidades.md` §7 |
| `LPRT-03`, `LPRT-06`, `A11` | captura do cliente 7662 sobre `ECAP` | `wyd-server` + `wyd-client` | `wyd-protocol/lacunas.md` §1 |
| `LNET-01`, `LBOT-03` | executar a medição já autorizada | QA + `wyd-ops` | `wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` |
| `LCLI-03`, `LCLI-05` | operador da ficha e resposta do titular | produto + revisão jurídica | `wyd-client/lacunas.md` §2 |
| `A20`, `M21` | telas e rotas do portal nas fatias | `wyd-portal` | `wyd-db/lacunas.md` §1 |
| catálogo funcional | medir total e cobertura | `wyd-core` + `wyd-sim` + `wyd-db` | `wyd-1.0.0.md` §1.5 |
| placar do `wyd-ops` | o ledger vive em `wyd-ops/docs/lacunas.md` e `wyd-docs/gate/checks/lacunas.py` lê `<sistema>/lacunas.md`; as decisões OPS ficam fora do placar impresso pelo gate | `wyd-ops` | `wyd-docs/gate/checks/lacunas.py` |

**Atualização de 2026-11-26:** `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `Q-ALVO-1`–`03`, `OPS-03.1`–`OPS-03.5` e
`OPS-05` têm resposta do dono registrada (§11); O4/O5 foram formalizados em `ADR-OPS-0011` e no
`ADR-OPS-0007` revisado; `Q-SERIE-1` foi implementada e provada como ciclo SQL,
`Q-SEP-1` = A′ renomeou os 28 arquivos para `_`, **`CAP-37` foi fechada por `runner_pg` (7/7 em PG17, 2026-11-27)** e
`OPS-05` publicou labels, as 71 issues novas e o board de 121 itens. Das linhas acima, o que resta é
execução (`LNET-01`/`LBOT-03`), `CAP-38`/`CAP-39`, terceiro (`LCLI-05`) ou
adiamento (`OPS-04`).

## 10. Correções desta revisão (2026-11-25)

| Defeito encontrado | Correção aplicada |
|---|---|
| esta página declarava `fechado` com itens `Parcial` | estado passa a `parcial`, com a lista da §9 |
| `auditoria-de-organizacao.md` §4 dizia que 11 sistemas não tinham o carimbo, contra o §2 | §4 remete à segunda passada e ao §3 |
| `Q-SERIE-1` e `Q-SEP-1` apareciam com opções e recomendação diferentes em três páginas do `wyd-ops` | as três páginas citam o documento dono (`serie-e-regras.md` §9 e §9.1) em vez de repetir as letras |
| `gate/checks/contagens.py` emitia `SyntaxWarning` de escape inválido na regex de data | regex passou a `raw string`, com o mesmo comportamento |

## 11. Respostas de dono registradas em 2026-11-25 (segunda rodada)

| Decisão | Resposta | Registro canônico |
|---|---|---|
| `D-06` | A — finalizar o `wyd-db` | `wyd-ops/docs/prioridade-do-projeto.md` §0.1 |
| `Q-SERIE-1` | B implementada: 28 `down` e ciclo PG17 via `psql` passaram | `wyd-db/migrations/serie-e-regras.md` §9.2 |
| `Q-SEP-1` | **A′ decidida e aplicada**: 28 arquivos renomeados para `_`; todos parseiam | `wyd-db/migrations/serie-e-regras.md` §9.1 |
| `Q-ALVO-1`–`03` | A1 (porta em `127.0.0.1`), B1 (fixture contígua) e papel de teste; `runner_pg` verde 7/7 | `wyd-db/tests/alvos/runner-pg.md` |
| `OPS-03.1`–`OPS-03.3` | aprovados na forma | `wyd-ops/docs/lacunas.md` §4 e §5 |
| `OPS-03.4`, `OPS-03.5` | aprovados e formalizados em `ADR-OPS-0011` e `ADR-OPS-0007` revisado | `wyd-ops/docs/lacunas.md` §4 |
| `OPS-05` | fatias, lacunas, labels, 10 `ADM` e board de 121 itens publicados em 2026-11-26 (#129–#217) | `wyd-ops/docs/github-mapping.md` §7 |
| `OPS-04` | adiada para depois do primeiro deploy interno | `wyd-ops/docs/lacunas.md` §4 |

## 12. Publicação no GitHub (2026-11-26)

O PR [#128](https://github.com/lucascantarelli/wyd/pull/128), aberto em 2026-11-26 contra `develop`, não foi
integrado e permanece apenas como registro histórico. A publicação efetiva desta rodada é a §14. O namespace de
label ficou em `area/*` (decisão de 2026-11-25), e `OPS-05` criou:

| Artefato | Resultado |
|---|---|
| Labels novas | `estado/proposta`, `estado/aceita`, `fase/1`–`fase/6`, além das `area/*` e `estado/bloqueada` existentes |
| 61 fatias | [#129](https://github.com/lucascantarelli/wyd/issues/129)–[#189](https://github.com/lucascantarelli/wyd/issues/189), uma por página de `docs/roadmap/` |
| 10 lacunas | [#190](https://github.com/lucascantarelli/wyd/issues/190)–[#199](https://github.com/lucascantarelli/wyd/issues/199): `CAP-38`, `CAP-39`, os seis seeds `0016`/`0017`/`0018`/`0020`/`0023`/`0027`, `OPS-04` e `LCLI-05` |
| Board do Project | 121 itens em 2026-11-26, após a importação das 81 pendências; prova na §14 |

Cada issue nova tem documento de origem com link para `develop`, `**Gate de verificação:**`, critérios de
aceite observáveis e o bloco `## Skills` com `skill_principal` e complementares. A criação foi
idempotente: o script consulta os títulos existentes antes de criar e a reexecução não duplica issue.

## 13. Prova real do DevOps 1A–11A (2026-11-26)

| Prova | Evidência | Estado |
|---|---|---|
| PR e checks do pacote | PR [#205](https://github.com/lucascantarelli/wyd/pull/205), squash `0c4a612`; `gate-code`, `gate-db`, `gate-docs` e `gate-security` com conclusão **success** | comprovado |
| Imagem publicada | run [`45827609542`](https://github.com/lucascantarelli/wyd/actions/runs/45827609542), commit `0c4a612`; naquela publicação, tags `:SHA`, `:develop` e `:latest`, digest `sha256:462029e…`; SBOM SPDX + provenance SLSA v1; smoke HTTP 200 | comprovado |
| Sinal stable | run [`45828077654`](https://github.com/lucascantarelli/wyd/actions/runs/45828077654); `wyd-db` e `wyd-ops-signer` concluíram com Rust `stable` | comprovado |
| Rollback | `make rollback TAG=f271caab…` e `TAG=0c4a612a…` puxaram as imagens do GHCR e responderam HTTP 200 sem build local | comprovado |
| Node 20 | logs recentes de `code`, `docs`, `docs-image`, `security`, `ci-db` e `code-stable` sem o aviso nem a SHA `ea165f8…` | comprovado |
| Produção, `OPS-04`, `PROJECT_TOKEN` | provedor/VPS, backup, RPO/RTO e PAT continuam fora deste pacote; nenhum deploy de produção foi feito. `OPS-05` fechou em separado (§14) | pendente do dono |
| Runtime nginx | F2 ainda usa o servidor Python atual; migração não foi decidida nem executada | pendente de decisão |

O PR foi integrado em `develop`; a publicação e o rollback acima usam o commit integrado, não uma árvore local.

## 14. Fechamento do mapeamento GitHub e das skills (2026-11-26)

| Prova | Medição |
|---|---|
| Issues no repositório | **121** issues abertas: 40 `WYDDB`, 61 fatias, 10 lacunas e 10 `ADM` (#208–#217) |
| Board `WYD 1.0.0` nº 10 | **121 itens**, 5 views (`Fila`, `Por sistema`, `Bloqueados`, `Minhas issues`, `Roadmap`) e todos os campos preenchidos |
| Campos do board | `Sistema` em 121/121; `Status` em 121/121 (96 Backlog, 24 Bloqueado, 1 Pronto); `Fase`, `Prioridade` e `Marco` em 121/121 |
| Links e corpo das issues | 111 corpos que apontavam para uma branch não integrada foram corrigidos para `develop`; os mesmos 111 receberam `**Gate de verificação:**` e o bloco `## Skills`; a varredura final encontrou **0** links antigos, **0** headings antigos e **0** corpos sem gate |
| `ADM-01`–`ADM-10` | páginas locais convertidas em issues #208–#217, com `area/wyd-admin`, `estado/proposta`, fase e corpo da página dona |
| Título e estado | as 40 `WYDDB` receberam o prefixo `[tarefa] wyd-db:` e a label `estado/proposta`; a varredura final encontrou **0** títulos fora do formato e **0** issues sem label de estado |
| Skills | `make test-gate`: 23 testes, 22 OK + 1 skip Windows; `test_skill_contract` 5/5: lista do gate = template, 4 formulários com as 9 skills de então (a lista tem doze desde 2026-11-27), 2 skills locais com frontmatter e PR template com registro da skill |
| Entrega versionada | PR [#218](https://github.com/lucascantarelli/wyd/pull/218), squash [adcf018](https://github.com/lucascantarelli/wyd/commit/adcf01864d5b79a0d526adf41f319f48dc9ffed1) em `develop`; checks do PR e workflows pós-merge `code`, `ci-db`, `docs`, `docs-image` e `security` com conclusão **success** |

A importação usou as labels `area/*` como fonte de `Sistema`, `estado/bloqueada` e
`estado/aguardando-decisao` como `Bloqueado`, e os marcos por sistema (`wyd-db` M2, `wyd-ops` M5, os
demais M3). Nenhum segredo foi impresso. O board não substitui as páginas donas: ele é o espelho de
estado; a página da fatia continua dona do DoD e da evidência.

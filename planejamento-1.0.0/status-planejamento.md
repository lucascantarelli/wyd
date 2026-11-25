# Status do planejamento — WYD 1.0.0

> Pergunta: "o que está feito, parcial ou faltando no planejamento do projeto e o que depende do usuário?"
> Público: Produto, arquitetura, desenvolvimento e agentes
> Dono: arquitetura (consolidação) — cada item tem dono próprio citado
> Base: missão "Fechar todo o planejamento do projeto"; `AGENTS.md`; `planejamento-1.0.0/wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-25

**Estado:** fechado (2026-11-25). Relatório consolidado de fechamento do planejamento; não substitui os documentos donos.

## 1. Legenda

- **Feito** — artefato existe, gate verde, dono definido.
- **Parcial** — existe mas falta fechamento (captura, medição, aprovação).
- **Faltando** — não existe; ação necessária.
- **N/A** — não se aplica ao estado atual.

## 2. Checklist da missão

| # | Item da missão | Estado | Evidência |
|---|---|---|---|
| 1 | Análise dos achados da revisão anterior | Feito | gate verde (`censos: medidas e citações coincidem`); contagens 386/384/364 |
| 2 | Planejar todos os gaps | Feito | `lacunas.md` por sistema; `wyd-ops/auditoria-de-organizacao.md` |
| 3 | Estrutura de pastas com `.gitkeep` (todo o projeto) | Feito | `planejamento-1.0.0/*/src/.gitkeep`, `*/tests/.gitkeep` (12 sistemas); `crates/*/` (`.gitkeep`); `wyd-client/files/.gitkeep` |
| 4 | Documentação detalhada por módulo | Feito | `docs/arquitetura/componentes.md` em 13 sistemas + `docs/modulos.md` em 6 sistemas (wyd-db, wyd-ops, wyd-core, wyd-client, wyd-bots, wyd-docs) com objetivo, escopo, interfaces, dependências, testes, segurança, monitoramento, DoD |
| 5 | Docs organizados, limpos, padronizados, sem redundância | Parcial | gate valida links/âncoras/censos; padrão de cabeçalho 5 linhas aplicado nas páginas novas |
| 6 | Projeto organizado e mapeado no GitHub | Parcial | `.github/ISSUE_TEMPLATE/*`, `pull_request_template.md`, workflows; **0 issues criadas** (requer aprovação) |
| 7 | Padronização de issues (incl. skills a carregar) | Feito | campo "Skills que o agente deve carregar" adicionado aos 4 templates |
| 8 | Uso das skills pelo agente | Feito (processo) | `.agents/skills/` (2) + cloud skills; `ia-e-skills.md` documenta quando carregar |
| 9 | Planejar skills/docs/testes/código/integração/deploy/monitoramento/segurança | Feito | `plano-de-testes.md` e `componentes.md` em 13 sistemas; `wyd-ops` cobre deploy/monitoramento/segurança; `ia-e-skills.md` |
| 10 | Padrões de qualidade de código/testes/boas práticas/docs/segurança/monitoramento/deploy | Feito | documentado em `plano-de-testes.md` e `componentes.md` de 13 sistemas; código só em `wyd-db` |
| 11 | Todos os testes implementados, documentados e funcionando | Parcial | `wyd-db`: 19 testes locais; demais: 0 (planejados) |
| 12 | SOLID, design patterns, clean code, arquitetura | Parcial | documentado (componentes, contratos); sem código para validar |
| 13 | Padrões de testes, fixtures, mocks, stubs, CI, cobertura | Feito (planejamento) | documentado em `plano-de-testes.md` de 13 sistemas; CI de código é stub (`full-workflow.yml`) — implementação pendente |
| 14 | Boas práticas GitHub Actions, pipelines/workflows complexos, CI/CD, monitoramento | Parcial | `code.yml`, `docs.yml`, `docs-image.yml` (nunca rodou), `full-workflow.yml` (stub) |
| 15 | Reuso de código, modularização, desacoplamento, coesão, baixo acoplamento | Feito (planejamento) | documentado em `componentes.md` de 13 sistemas |
| 16 | Centralização de fixtures, mocks, stubs, dados de teste | Feito (planejamento) | documentado em `plano-de-testes.md` de 13 sistemas |
| 17 | Baixa complexidade, alta legibilidade/manutenibilidade/escalabilidade | N/A | sem código ainda |
| 18 | Funcionamento de integrações, skills e recursos Freebuff | N/A | nenhuma integração de código hoje |
| 19 | Organizar arquivos de IA (freebuff, skills, integrações) | Feito | `wyd-ops/docs/arquitetura/ia-e-skills.md` |
| 20 | Sugerir melhorias/otimizações/refatorações/atualizações | Contínuo | seção 6 deste relatório |
| 21 | ADRs para todas as decisões importantes | Feito | 9 ADRs aceitos (2 originais + 7 formalizados em 2026-11-25); índice + template + registro atualizados |
| 22 | Decisões documentadas, justificadas e aprovadas | Feito | 11 decisões formalizadas em 2026-11-25; `decisoes-registradas.md` §2/§3 atualizados |
| 23 | Funcionalidades do cliente mapeadas e documentadas | Feito | `wyd-client/docs/specs/mapa-de-funcionalidades.md` + inventário |
| 24 | Sistemas de jogo funcionando (guerras, mobs, …) | N/A (planejamento) | `wyd-core/docs/specs/catalogo-funcional.md` (mapa de fronteira) |
| 25 | Documentação de planejamento organizada, números reais, indexada | Feito | gate verde; 386 páginas / 384 fontes / 357 proposto / 22 aceito / 4 sem linha de estado |
| 26 | Skills do agente funcionando, documentadas, testadas, aprovadas | Parcial | documentadas em `ia-e-skills.md`; sem testes automatizados de skill |
| 27 | Documentação atualizada, coerente, sem duplicidade/redundância/erros | Parcial | gate de links/censos verde; revisão humana pendente |
| 28 | Documentação rigorosamente padronizada | Parcial | padrão de cabeçalho aplicado nas páginas novas |
| 29 | Remover arquivos obsoletos/revisões | Parcial | `.gitignore` cobre gerados; revisão manual pendente |

## 3. Sistemas — estado do D-04 (organização)

| Sistema | D-04 | Roadmap com DoD | RF/RNF | Lacunas abertas |
|---|---|---|---|---|
| wyd-db | fechado | sim (40 issues) | (piloto) | várias (ver lacunas.md) |
| wyd-admin | fechado | parcial | sim | 0 |
| wyd-portal | fechado | sim (F0–F11) | 33 RF | 0 (9 fechadas) |
| wyd-protocol | fechado | parcial (fases) | 6 RF | 2 captura |
| wyd-net | fechado | sim (F0–F4) | 8 RF | 1 medição |
| wyd-core | fechado | sim (F0–F4) | 8 RF | 0 |
| wyd-server | fechado | sim (F0–F3) | 7 RF | 1 peer |
| wyd-sim | fechado | sim (F0–F3) | 7 RF | 0 |
| wyd-launcher | fechado | sim (F0–F3) | 8 RF | 0 |
| wyd-client | fechado | sim (F0–F2) | 7 RF | 3 |
| wyd-bots | fechado | sim (F0–F3) | 6 RF | 1 medição |
| wyd-docs | fechado | sim (F0–F3) | 7 RF | 1 decisão |
| wyd-ops | fechado | sim (F0–F5) | 7 RF | fronteira parcial |

## 4. Artefatos criados nesta rodada (além das anteriores)

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
- `planejamento-1.0.0/wyd-{net,core,server,admin,portal,docs,client}/docs/arquitetura/adr/` (7 ADRs formalizados como `aceito` em 2026-11-25)

## 5. Decisões necessárias do usuário (não automatizáveis)

Skill a carregar em cada item (AGENTS.md §5): `planejar-sistema` (abrir/fechar sistema),
`auditar-planejamento` (auditoria antes de aceitar), `documentation-writer` (redação).

**Formalizados em 2026-11-25 por aprovação do dono:** 7 ADRs (`ADR-NET-0001`, `ADR-CORE-0001`,
`ADR-SRV-0001`, `ADR-ADM-0001`, `ADR-POR-0001`, `ADR-DOC-0001`, `ADR-CLI-0001`) → `aceito`;
11 decisões (`LNET-01`, `LSRV-05`, `LPRT-03`/`06`, `LCLI-02`/`03`/`05`, `LBOT-03`, `LPDOC-12`,
`Q-IP-1`/`2`) → decididas/fechadas. Evidência: `adr/index.md` §2, `decisoes-registradas.md` §2.

1. **Criar issues/milestones/boards no GitHub** — convenção pronta (`github-mapping.md`); a criação é
   ato explícito. Recomendação: criar a partir dos roadmaps (F0 de cada sistema) com labels por
   namespace da §2 do mapping. Skill: `planejar-sistema`. **Não criado** (sem autorização).
2. **Segredos e deploy** — valores de DSN/tokens só via secret do GitHub ou `.env` local; nunca em
   chat. Deploy/release/push exigem pedido explícito. Skill: `planejar-sistema`.
3. **Imagem do site** — `.github/workflows/docs-image.yml` versionado mas nunca rodou; publicar
   requer aprovação (ver `wyd-ops/plano-imagem-e-compose-do-site.md`). Skill: `planejar-sistema`.
4. **Primeira fatia de código** — D-01 mantém `wyd-db` como foco; recomendação: `WYDDB-001`
   (baseline do crate) e, em paralelo, `WYDNET-001`. Skill: `planejar-sistema`.
5. **Roadmaps parciais** — `wyd-protocol` e `wyd-admin` têm roadmap parcial; fechar com F0–Fn + DoD.
   Skill: `planejar-sistema`.
6. **net/bots medições pendentes** — `LNET-01` (limites), `LBOT-03` (medição) continuam como
   medição mesmo após aprovação da política; valores congelam após primeira execução. Skill: `auditar-planejamento`.

## 7. Verificação final

- `docker compose --profile gate run --rm gate` → `censos: medidas e citações coincidem` (2026-11-25).
- Contagens: 386 páginas / 384 fontes / 357 proposto / 22 aceito / 4 sem linha de estado.
- Nenhum commit/push/PR/issue/milestone/board/deploy/secreto realizado nesta rodada.
- Arquivos criados/atualizados nesta rodada: `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/template.md` (estado corrigido), contagens sincronizadas em `wyd-docs-1.0.0.md`, `wyd-portal-1.0.0.md`, `status-planejamento.md`, `prioridade-do-projeto.md`, `foco-e-sequencia.md`, `pendencias-de-fechamento.md`.

## 6. Sugestões (melhorias/otimizações/refatorações)

- Renomear `wyd-portal/docs/roadmap/roadmap.md` para padrão único (wyd-db usa `index.md`+`fases.md`+
  `WYDDB-nnn.md`). Definir um padrão único para todos os sistemas.
- Criar `planos/metricas.md` no `wyd-db` (citado, não escrito) para percentual de progresso.
- Adicionar `docs/planos/metricas.md` por sistema (hoje só `wyd-docs` tem).
- Formalizar ADRs para as 7 decisões candidatas.
- Rodar `auditar-planejamento` em cada sistema antes de aceitar (skill disponível).
- Padronizar fronteira/contratos dos sistemas que ainda têm "parcial" (wyd-ops, wyd-client).

## 7. Verificação final

- `docker compose --profile gate run --rm gate` → `censos: medidas e citações coincidem`.
- Contagens: 386 páginas / 384 fontes / 357 proposto / 22 aceito / 4 sem linha de estado.
- Nenhum commit/push/PR/issue/deploy realizado nesta rodada.
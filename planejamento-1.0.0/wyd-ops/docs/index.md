# wyd-ops — visão geral

> Pergunta: "o que o `wyd-ops` entrega, o que ele já decidiu e por onde começo a ler?"
> Público: Desenvolvedor novo, arquitetura e quem opera
> Dono: frente `wyd-ops`
> Base: `docs/devops/index.md`; `proposta-versoes-e-cobertura-funcional.md`; `AGENTS.md` §4
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-22)

O `wyd-ops` é a frente de **processo e infraestrutura**: CI, imagens, deploy, observabilidade,
topologia de ambiente e provisionamento. Ele não implementa regra de jogo nem fala o protocolo;
ele garante que o que existe seja construível, publicável, observável e reproduzível.

## O que ele já decidiu

| Decisão | Data | Onde |
|---|---|---|
| CI usa apenas `run:` e action própria; nenhuma action de terceiro sem SHA | 2026-11-19 | `docs/devops/index.md` |
| deploy é `docker compose` em host com systemd; runtime é Compose, não Kubernetes | 2026-11-19 | `arquitetura/adr/adr-ops-0002-hospedagem-producao.md` |
| versões por linha de release e a cobertura funcional por versão | 2026-11-20 | `proposta-versoes-e-cobertura-funcional.md` §7 |
| elo de controle admin → servidor com segredo próprio e ack antes de escrita | 2026-11-22 | `wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §3 |
| imagem do site é construída pelo CI e consumida pelo Compose | 2026-11-22 | `plano-imagem-e-compose-do-site.md` |
| CI híbrido por domínio, com um gate agregador por domínio | 2026-11-26 | `docs/arquitetura/adr/adr-ops-0003-arquitetura-hibrida-de-workflows.md` |
| gate de documentação com fonte única em `wyd-docs/gate.sh` | 2026-11-26 | `docs/arquitetura/adr/adr-ops-0006-gate-unico-de-documentacao.md` |
| deploy sobre Compose, fornecedor de VPS ainda a definir | 2026-11-26 | `docs/arquitetura/adr/adr-ops-0007-deploy-sobre-compose-com-provedor-a-definir.md` |
| layout do repositório: artefato executável na pasta real do projeto; Compose, `.dockerignore` e workflows na raiz | 2026-11-22 | `planejamento-1.0.0/wyd-1.0.0.md` §8 |
| hospedagem de produção: VPS paga com processo persistente e Compose (A); contrato e backup pendentes | 2026-11-24 | `proposta-decisoes-ops.md` §4 e §6 |

## O que ele não faz

| Fora | Dono |
|---|---|
| regra de jogo | `wyd-core` / `wyd-sim` |
| layout do fio | `wyd-protocol` |
| escrita administrativa e auditoria | `wyd-admin` |
| conteúdo de notícia e conta de jogador | `wyd-portal` |
| patch do cliente | `wyd-client` / `wyd-launcher` |

## Onde os artefatos dele moram

O layout do repositório — qual pasta, qual sistema é dono e por que Compose, `.dockerignore` e
`.github/workflows/` ficam na raiz — tem um dono único: `planejamento-1.0.0/wyd-1.0.0.md` §8 (no
repositório; o panorama não é publicado no site). O Dockerfile da imagem do site está em
`wyd-ops/wyd-docs.Dockerfile`; a decisão e o DoD, em `plano-imagem-e-compose-do-site.md`.

## Qual frente vem agora

O ranking publicado das frentes do projeto — finalizar o `wyd-db`, expandir os outros sistemas e resolver
as lacunas abertas — e a trajetória recomendada estão em [Prioridade do projeto](prioridade-do-projeto.md).
É o único lugar publicado que ranqueia as frentes com número e fonte; a decisão continua do dono do produto.

## O que falta

O planejamento de organização está fechado: dashboard (`wyd-ops-1.0.0.md`), `RF-OPS`/`RNF-OPS`, plano de
teste e roadmap com DoD existem. O CI, o gate único, o cache e a matriz PostgreSQL foram implementados
em F0+F1 (2026-11-26); o estado real e o que falta (imagem, deploy, observabilidade e governança) estão
em [DevOps](devops/index.md). Os contratos O1–O5 estão aprovados (O4 em `ADR-OPS-0011`, O5 no
`ADR-OPS-0007` revisado); restam a aplicação de `OPS-04` e a escolha do fornecedor de VPS. O registro
único é [lacunas.md](lacunas.md).

## Limite desta documentação

- Não afirma ambiente publicado: nenhum deploy existe.
- Não afirma imagem publicada: o workflow está no repositório; a primeira execução dirá se empurra.
## Decisão de hospedagem

A hospedagem de produção — `Q-OPS-2` — foi respondida em **A** em 2026-11-24: VPS paga com processo
persistente e Compose. Contrato, VPS, backup e operação continuam **não executados**. O registro e a
divergência de §3.2 estão em [Decisões do `wyd-ops`](proposta-decisoes-ops.md); §8 registra a ação 2
dos tipos/campos como executada em 2026-11-19.

## Mapa de páginas

Este mapa garante que toda página de `docs/` seja alcançável a partir deste índice; a pasta dona continua sendo a fonte do conteúdo.

### raiz

- [Estrutura de pastas e mapa de documentação por módulo](estrutura-de-pastas-e-modulos.md)
- [Mapeamento GitHub — labels, nomes e convenções](github-mapping.md)
- [Lacunas e decisões pendentes — registro único](lacunas.md)
- [Módulos do `wyd-ops` — detalhamento](modulos.md)
- [Pacote de aprovação — contratos O1–O5](pacote-aprovacao-o1-o5.md)
- [Pacote de aprovação — decisões fora dos contratos O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md)
- [Pauta de decisões pendentes — registro canônico das 11 pendências](pauta-de-decisoes-pendentes.md)

### arquitetura

- [componentes](arquitetura/componentes.md)
- [Decisões registradas — projeto WYD 1.0.0](arquitetura/decisoes-registradas.md)
- [IA, skills e integrações — organização](arquitetura/ia-e-skills.md)

### arquitetura/adr

- [ADR-OPS-0001 — Workspace Cargo único, toolchain e versões](arquitetura/adr/adr-ops-0001-workspace-cargo-unico.md)
- [ADR-OPS-0002 — Hospedagem de produção em VPS paga com Compose](arquitetura/adr/adr-ops-0002-hospedagem-producao.md)
- [Candidatos a ADR — backlog de decisões](arquitetura/adr/candidatos.md)
- [Índice de ADRs — projeto WYD 1.0.0](arquitetura/adr/index.md)
- [ADR-<SISTEMA>-NNNN — <título curto>](arquitetura/adr/template.md)

### planos

- [Baseline de segurança, monitoramento e deploy — planejamento](planos/baseline-seguranca-monitoramento-deploy.md)
- [Estratégia de testes — planejamento transversal](planos/estrategia-de-testes.md)
- [Tarefa de medição — `LNET-01` e `LBOT-03`](planos/medicoes-lnet-01-lbot-03.md)
- [Pipeline de CI/CD — planejamento](planos/pipeline-ci-cd.md)
- [plano-de-testes](planos/plano-de-testes.md)
- [Template de issue/fatia — padrão único](planos/template-de-issue.md)
- [Verificação de skills e integrações — planejamento](planos/verificacao-de-skills-e-integracoes.md)

### roadmap

- [index](roadmap/index.md)
- [roadmap](roadmap/roadmap.md)

### specs

- [requisitos](specs/requisitos.md)

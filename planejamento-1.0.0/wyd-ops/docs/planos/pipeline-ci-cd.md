# Pipeline de CI/CD — planejamento

> Pergunta: "quais workflows, jobs e gates o projeto precisa para provar CI, imagem, ambiente e deploy sem inventar métrica?"
> Público: Desenvolvimento, QA, arquitetura e agentes
> Dono: frente `wyd-ops`
> Base: `AGENTS.md` §7 e §9; `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-002`, `RF-OPS-006`, `RF-OPS-007`, `RNF-OPS-001`–`005`); `wyd-ops/docs/arquitetura/componentes.md`; `wyd-ops/docs/github-mapping.md`; `wyd-ops/auditoria-de-organizacao.md` §2–§3
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26) — F0+F1 implementados; F2 parcial (Buildx/cache/atestados e digests) e F5 parcial (rulesets, allowlist e ADR). O que está no ar e o que falta estão em [DevOps](../devops/index.md); este documento mantém o desenho e as decisões.

## 1. Escopo e dono

Este documento mantém o desenho e as decisões da esteira de CI/CD. O estado aplicado, os comandos e o que falta ficam em [DevOps](../devops/index.md); a implementação de F0+F1 já foi feita, mas este arquivo não é o runbook. O dono da execução é `wyd-ops`; os sistemas consumidores continuam donos de seus próprios gates de código e de teste.

## 2. Estado medido dos workflows

| Workflow | Gatilho medido | Status | O que prova hoje |
|---|---|---|---|
| `.github/workflows/docs.yml` | todo push/PR em `develop` | implementado | `gate-docs` (montagem, links, âncoras, navegação, sintaxe e censos) e Pages em push de `develop` |
| `.github/workflows/code.yml` | todo push/PR em `develop`/`main` | implementado | `fmt`, `clippy` e `unit` em matriz de 3 crates (Windows para signer/launcher), cobertura informativa e `gate-code` |
| `.github/workflows/ci-db.yml` | todo push/PR em `develop`/`main` | implementado | matriz `schema`/`reversibility`/`runner` com `postgres:17` por digest como service container e `gate-db` |
| `.github/workflows/docs-image.yml` | PR/push quando toca docs/site/compose/Dockerfile | implementado | Buildx + cache GHA, fumaça HTTP 200, push no GHCR só em `develop` e provenance/SBOM |
| `.github/workflows/security.yml` | todo push/PR + schedule semanal | implementado | `cargo audit`/`cargo deny` em cada crate, `dependency-review` em PR, `workflow-lint` com actionlint e `gate-security` |
| `.github/workflows/code-stable.yml` | schedule diário + dispatch | implementado | testes com Rust `stable` em `wyd-db`/`wyd-ops-signer`; não é check obrigatório |
| `.github/workflows/issue-status.yml` | eventos de issue e PR | implementado | sequência de labels `estado/*` e espelho no `Status` do Project; não é gate de código |
| `.github/workflows/release.yml` | tag `v*` | implementado | assinatura e publicação do manifesto; sem binários até a F3 |

Fonte: `AGENTS.md` §4 e §9; `wyd-ops/docs/index.md`.

## 3. Checks obrigatórios em PR (aplicados)

| Check | Workflow/job | Bloqueante? | Observação |
|---|---|---|---|
| documentação | `docs.yml` / `gate-docs` | sim | chama `wyd-docs/gate.sh`; censos devem coincidir |
| formatação, lint e unitários | `code.yml` / `gate-code` | sim | matriz de 3 crates; Windows para signer/launcher |
| PostgreSQL | `ci-db.yml` / `gate-db` | sim | matriz `schema`, `reversibility`, `runner` em PG17 |
| segurança de dependências | `security.yml` / `gate-security` | sim | `cargo audit`/`deny`, `dependency-review` em PR e actionlint |
| cobertura de linha | `code.yml` / `cobertura` | não, até o gate do sistema fechar | informativa por decisão `D-02 = B`; 2 arquivos, 7 dias |
| build/fumaça da imagem do site | `docs-image.yml` / `imagem` | não nesta fase | publicação no GHCR restrita a push não-PR; vira obrigatório na F2 |
| gate de sistema | a definir por sistema | sim quando o sistema declarar o gate fechado | não existe hoje para `wyd-net`, `wyd-core`, `wyd-server`, `wyd-portal`, etc. |

## 4. Gates por camada

| Camada | Comando ou evidência mínima | Dono |
|---|---|---|
| Documentação | `docker compose --profile gate run --rm gate` | `wyd-docs` + `wyd-ops` |
| Código Rust | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` | sistema dono do crate |
| Schema PostgreSQL | migrations aplicadas do zero em PG17 + `sistema_pg` | `wyd-db` |
| Segurança | `cargo audit`, `cargo deny`, varredura de segredo, revisão de permissões | `wyd-ops` |
| Imagem | `docker buildx build`, fumaça HTTP 200, sem segredo na imagem | `wyd-ops` |
| Deploy | deploy em ambiente efêmero/staging, rollback validado, aprovação explícita | `wyd-ops` |

## 5. Boas práticas de GitHub Actions (obrigatórias)

- Permissões mínimas por workflow e por job (`permissions: contents: read` por padrão).
- Actions de terceiros pinadas por SHA, com versão em comentário.
- Usar actions oficiais pinadas para build/publicação (Buildx) e `run:` para login e operações de shell; não introduzir action de terceiro sem justificativa.
- `concurrency` com cancelamento em PR; não cancelar release/deploy sem decisão explícita.
- **Check obrigatório não pode ser filtrado por caminho**: o check ficaria em `Expected` em PR que não toca o caminho. Por isso `code`, `ci-db`, `docs` e `security` rodam em todo PR; `docs-image` mantém filtro porque não é obrigatório.
- `paths` são usados só em workflow não obrigatório, para evitar execução inútil sem bloquear merge.
- Segredos apenas em secret do GitHub ou `.env` local ignorado; nunca em workflow, log ou documento.
- Artefatos de cobertura com retenção definida e sem segredo.
- Proibido runner self-hosted em job que executa código de PR em repositório público.
- Toda mudança de workflow deve ser revisada como mudança de gate, não como cosmética.

## 6. Skills que o agente deve carregar

| Trabalho | Skill | Quando |
|---|---|---|
| Abrir ou fechar sistema, issue ou frente | `planejar-sistema` | antes de escrever ou alterar planejamento |
| Revisar página, workflow ou decisão antes de aceitar | `auditar-planejamento` | antes de declarar `aceito` |
| Redação técnica de documentação | `documentation-writer` | ao redigir ou revisar docs |
| Decisão de arquitetura ou fronteira | `planejar-sistema` + revisão humana | antes de ADR ou mudança de contrato |

Fonte de organização de skills: `wyd-ops/docs/arquitetura/ia-e-skills.md`.

## 7. Cruzamento com auditoria e especificação

| Referência | Relação com este documento |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | `wyd-ops` está organizado e tem os contratos O1–O5 aprovados; este documento fecha parte do checklist de CI/CD sem declarar a implementação por si só |
| `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-002`, `RF-OPS-006`, `RF-OPS-007`, `RNF-OPS-001`–`005`) | requisitos funcionais e não funcionais que este planejamento deve satisfazer |
| `wyd-ops/docs/arquitetura/componentes.md` (`ci`, `imagem`, `ambiente`, `monitoramento`, `segredos`, `deploy`, `gate`) | unidades de responsabilidade e invariantes do CI/CD |
| `wyd-ops/docs/planos/plano-de-testes.md` | alvos de teste de CI, imagem, ambiente, monitoramento, segredos e deploy |
| `wyd-ops/docs/github-mapping.md` | nomes de issue, branch, PR, label e milestone usados pelos workflows |
| `wyd-ops/docs/prioridade-do-projeto.md` (`D-02`, `D-05`, `D-06`) | política de cobertura, workspace e frente de execução |

## 8. Decisões aplicadas (2026-11-26)

1. **Required checks** — `gate-code`, `gate-db`, `gate-docs` e `gate-security` aplicados nos rulesets de `develop`/`main`; `strict=false` em `develop` e `true` em `main`; `docs-image` não é obrigatório até a F2. Ver `ADR-OPS-0003` e `ADR-OPS-0012`.
2. **`full-workflow.yml`** — removido; era stub e duplicava os gatilhos dos workflows reais.
3. **Deploy** — adiado até a escolha do fornecedor de VPS; o contrato é Compose + imagem por SHA + smoke + rollback, sem workflow fictício. Ver `ADR-OPS-0007`.
4. **Branch e Pages** — `main` arquivada; `docs`/`docs-image` publicam somente `develop`; a migração para `main` fica para o release 1.0.0. Ver `ADR-OPS-0012`.
5. **Runner self-hosted** — proibido para código de PR; nenhum job usa runner self-hosted.
6. **Cache e artefatos** — `rust-cache` por crate, cobertura com 7 dias, GHCR público com Buildx/cache GHA e tags `SHA`/`develop`/`latest` em `develop`. Ver `ADR-OPS-0004`.
7. **Gate de documentação** — um script (`wyd-docs/gate.sh`) para CI e Compose. Ver `ADR-OPS-0006`.
8. **Supply chain** — SHA completo, allowlist selecionada e `rust-cache` no commit peelado da tag v2.9.3. Ver `ADR-OPS-0008`.
9. **Actionlint** — dentro do `gate-security`, imagem por digest. Ver `ADR-OPS-0012`.

## 9. O que esta página não cobre

- Runbook, estado aplicado, métricas de execução e alteração operacional de workflow: ver `devops/index.md`.
- Criação de issue, label, milestone, board, commit, push, release ou deploy.
- Valores de segredo ou DSN.
- Gate de documentação em si: seu comando e evidência estão em `AGENTS.md` §7 e `wyd-docs`.
- Estratégia de teste detalhada por sistema: ver `estrategia-de-testes.md`.
- Baseline de segurança, monitoramento e deploy: ver `baseline-seguranca-monitoramento-deploy.md`.
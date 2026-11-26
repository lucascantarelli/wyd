# Pipeline de CI/CD — planejamento

> Pergunta: "quais workflows, jobs e gates o projeto precisa para provar CI, imagem, ambiente e deploy sem inventar métrica?"
> Público: Desenvolvimento, QA, arquitetura e agentes
> Dono: frente `wyd-ops`
> Base: `AGENTS.md` §7 e §9; `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-002`, `RF-OPS-006`, `RF-OPS-007`, `RNF-OPS-001`–`005`); `wyd-ops/docs/arquitetura/componentes.md`; `wyd-ops/docs/github-mapping.md`; `wyd-ops/auditoria-de-organizacao.md` §2–§3
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Planejamento; nenhum workflow, job, issue ou deploy foi criado ou alterado por este documento.

## 1. Escopo e dono

Este documento planeja a esteira de CI/CD do projeto. Ele não implementa workflow, não altera `.github/workflows/` e não cria recurso no GitHub. O dono da execução é `wyd-ops`; os sistemas consumidores continuam donos de seus próprios gates de código e de teste.

## 2. Estado medido dos workflows

| Workflow | Gatilho medido | Status | O que prova hoje |
|---|---|---|---|
| `.github/workflows/docs.yml` | PR/push em `develop`/`main` quando toca `planejamento-1.0.0/**`, `wyd-docs/**`, `AGENTS.md`, workflow ou action | verde no gate | montagem, links, âncoras, navegação, sintaxe e censos |
| `.github/workflows/code.yml` | PR/push em `develop`/`main` quando toca `crates/**`, `deny.toml`, workflow | parcial | fmt, clippy, 6 testes unitários, 13 testes de schema contra PG17, cobertura informativa, `cargo audit`/`cargo deny` bloqueantes |
| `.github/workflows/docs-image.yml` | PR/push em `develop`/`main` quando toca docs/site/compose/Dockerfile/workflow | nunca rodou/publicou | build da imagem do site e fumaça HTTP 200; publicação só fora de PR |
| `.github/workflows/full-workflow.yml` | push/PR em `develop`/`main` e cron semanal | stub | versão e commit do run; não mede cobertura nem gate de sistema |

Fonte: `AGENTS.md` §4 e §9; `wyd-ops/docs/index.md`.

## 3. Checks obrigatórios em PR (propostos)

| Check | Workflow/job | Bloqueante? | Observação |
|---|---|---|---|
| gate de documentação | `docs.yml` / `verificar` | sim | `docker compose --profile gate run --rm gate`; censos devem coincidir |
| formatação Rust | `code.yml` / `cargo` | sim | `cargo fmt --check` |
| lint Rust | `code.yml` / `cargo` | sim | `cargo clippy -- -D warnings` |
| testes unitários | `code.yml` / `cargo` | sim | `cargo test --lib` |
| testes de schema | `code.yml` / `cargo` | sim | PG17 migrado, `sistema_pg` com `--test-threads=1` |
| segurança de dependências | `code.yml` / `security` | sim | `cargo audit` e `cargo deny` |
| cobertura de linha | `code.yml` / `cargo` | não, até o gate do sistema fechar | informativa por decisão `D-02 = B` |
| build/fumaça da imagem do site | `docs-image.yml` / `imagem` | sim em PR quando o caminho for tocado | publicação no GHCR continua restrita a push não-PR |
| gate de sistema | a definir por sistema | sim quando o sistema declarar o gate fechado | não existe hoje para `wyd-net`, `wyd-core`, `wyd-server`, `wyd-portal`, etc. |

## 4. Gates por camada

| Camada | Comando ou evidência mínima | Dono |
|---|---|---|
| Documentação | `docker compose --profile gate run --rm gate` | `wyd-docs` + `wyd-ops` |
| Código Rust | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` | sistema dono do crate |
| Schema PostgreSQL | migrations aplicadas do zero em PG17 + `sistema_pg` | `wyd-db` |
| Segurança | `cargo audit`, `cargo deny`, varredura de segredo, revisão de permissões | `wyd-ops` |
| Imagem | `docker build`, fumaça HTTP 200, sem segredo na imagem | `wyd-ops` |
| Deploy | deploy em ambiente efêmero/staging, rollback validado, aprovação explícita | `wyd-ops` |

## 5. Boas práticas de GitHub Actions (obrigatórias)

- Permissões mínimas por workflow e por job (`permissions: contents: read` por padrão).
- Actions de terceiros pinadas por SHA, com versão em comentário.
- Preferir `run:` para operações de login/publicação; não introduzir action sem justificativa.
- `concurrency` com cancelamento em PR; não cancelar release/deploy sem decisão explícita.
- `paths` amplos o suficiente para não perder gate, mas específicos o suficiente para evitar execução inútil.
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
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | `wyd-ops` está organizado e mantém fronteira/contratos O1–O5 pendentes de aprovação; este documento fecha parte do checklist de CI/CD sem declarar a implementação por si só |
| `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-002`, `RF-OPS-006`, `RF-OPS-007`, `RNF-OPS-001`–`005`) | requisitos funcionais e não funcionais que este planejamento deve satisfazer |
| `wyd-ops/docs/arquitetura/componentes.md` (`ci`, `imagem`, `ambiente`, `monitoramento`, `segredos`, `deploy`, `gate`) | unidades de responsabilidade e invariantes do CI/CD |
| `wyd-ops/docs/planos/plano-de-testes.md` | alvos de teste de CI, imagem, ambiente, monitoramento, segredos e deploy |
| `wyd-ops/docs/github-mapping.md` | nomes de issue, branch, PR, label e milestone usados pelos workflows |
| `wyd-ops/docs/prioridade-do-projeto.md` (`D-02`, `D-05`, `D-06`) | política de cobertura, workspace e frente de execução |

## 8. Decisões abertas (não decididas por este documento)

1. **Nomes dos required checks no GitHub** — hoje os workflows não declaram `required_status_checks`.
   - Opções: A) exigir `docs`, `code` e `security` em todos os PRs para `develop`/`main`; B) exigir também `docs-image`; C) exigir por caminho alterado.
   - Recomendação: A para o gate editorial/código/segurança; B apenas após a primeira publicação bem-sucedida da imagem.
2. **`full-workflow.yml`** — hoje é stub de versão.
   - Opções: A) transformá-lo em orquestrador de gates; B) removê-lo e manter workflows separados; C) mantê-lo como sentinela semanal.
   - Recomendação: C no curto prazo e A quando houver mais de um crate.
3. **Deploy** — hoje nenhum deploy existe.
   - Opções: A) deploy automático em `main` após gates verdes; B) deploy manual com aprovação; C) deploy apenas por release tag.
   - Recomendação: B até existir rollback validado; A apenas depois de staging verde.
4. **Runner self-hosted** — proibido para código de PR em repositório público.
   - Opções: A) manter GitHub-hosted; B) self-hosted apenas para tarefas sem código de PR; C) híbrido por job.
   - Recomendação: A para PR; B apenas para publicação de imagem se necessário.

## 9. O que esta página não cobre

- Implementação ou alteração de workflow.
- Criação de issue, label, milestone, board, commit, push, release ou deploy.
- Valores de segredo ou DSN.
- Gate de documentação em si: seu comando e evidência estão em `AGENTS.md` §7 e `wyd-docs`.
- Estratégia de teste detalhada por sistema: ver `estrategia-de-testes.md`.
- Baseline de segurança, monitoramento e deploy: ver `baseline-seguranca-monitoramento-deploy.md`.
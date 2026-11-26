# DevOps — CI/CD, containers e operação local

> Pergunta: "quais workflows existem, o que cada gate prova, como rodar local e como reverter?"
> Público: Desenvolvimento, operação, arquitetura e agentes
> Dono: `wyd-ops`
> Base: `.github/workflows/*`; `Makefile`; `docker-compose*.yml`; `wyd-docs/gate.sh`; ADRs `ADR-OPS-0003`–`0008`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26) — F0 + F1 implementados; F2–F5 estão planejados e **não** implementados. Nenhum deploy de produção existe.

## 1. O que está no ar

| Workflow | Gatilho | Jobs | Check obrigatório |
|---|---|---|---|
| `code.yml` | todo push/PR em `develop`/`main` | `fmt` (matriz 3 crates), `clippy` (db/signer em Ubuntu; launcher em Windows), `unit` (db e signer em Ubuntu/Windows; launcher só em Windows), `cobertura` (informativa), `gate-code`, `notificar falha` | `gate-code` |
| `ci-db.yml` | todo push/PR | `pg` (matriz `schema`, `reversibility`, `runner` com `postgres:17` como service), `gate-db`, `notificar falha` | `gate-db` |
| `docs.yml` | todo push/PR | `gate-docs`, `publicar` (Pages, só push), `notificar falha` | `gate-docs` |
| `docs-image.yml` | caminhos de docs/imagem | build, smoke HTTP 200, push GHCR (só push) | não (F2 torna obrigatório) |
| `security.yml` | todo push/PR + schedule semanal | `analise` (audit/deny em cada crate), `dependency-review` (PR), `gate-security`, `notificar falha` | `gate-security` |
| `issue-status.yml` | issue aberta/rotulada/fechada/reaberta; PR aberto/ready/mergeado/fechado | `sincronizar`: label `estado/*` + espelho no Project | não (automação, não gate) |
| `release.yml` | tag `v*` | assina e publica manifesto | não |

`full-workflow.yml` foi removido: era um stub com os mesmos gatilhos do CI real.

### Por que os workflows obrigatórios não têm filtro de caminho

Um check obrigatório filtrado por caminho fica em **Expected** em PR que não toca aquele caminho e bloqueia o merge para sempre. Por isso `code`, `ci-db`, `docs` e `security` rodam em todo PR; o custo é baixo em repositório público e o bloqueio eterno é pior. `docs-image` continua filtrado porque não é obrigatório nesta fase.

## 2. O que cada gate prova

- **`gate-code`**: formatação, lint `-D warnings`, testes unitários dos três crates; `wyd-db` e `wyd-ops-signer` são portáveis, enquanto `wyd-launcher` é um produto Windows (EXE + Tauri/NSIS) e só roda em `windows-latest`. Cobertura é informativa (`D-02 = B`) e sobe como `coverage-summary.txt` + `lcov.info` por 7 dias.
- **`gate-db`**: os três alvos PostgreSQL. `schema` aplica a série e roda `sistema_pg`; `reversibility` prova `up → down → up`; `runner` prova a porta `PgAdapter` e o advisory lock com `wyd_migrator` LOGIN.
- **`gate-docs`**: `wyd-docs/gate.sh` — skills, evidência de lacunas, montagem, build MkDocs, links/âncoras/navegação, contagens e censos.
- **`gate-security`**: `cargo audit` e `cargo deny` em cada crate; em PR, `dependency-review` falha em severidade `high`.
- CodeQL continua em **default setup** do GitHub (actions, JS/TS, Python, Rust); não há workflow CodeQL duplicado.

## 3. Como rodar local

```bash
make gate        # mesmo gate do CI, dentro da imagem :gate
make fmt         # cargo fmt --check nos três crates
make lint        # cargo clippy -D warnings nos três crates
make test        # testes unitários (wyd-db --lib; signer --locked; launcher só no Windows)

make docs-up     # puxa a imagem publicada e sobe em 127.0.0.1:8000
make docs        # constrói da árvore de trabalho (dev, tag :dev)
make docs-down   # para o site

make db-up       # PostgreSQL 17 local em 127.0.0.1:5432, tmpfs
make db-down     # para e descarta o banco local
make rollback TAG=<sha>   # volta a imagem do site para o SHA indicado
```

O gate local usa a imagem `:gate` construída da árvore (`pull_policy: build`); o CI usa `setup-python` + `wyd-docs/gate.sh`. A ordem dos verificadores é a mesma nos dois.

### Testes PostgreSQL locais

O harness aceita dois modos:

- `WYD_DB_TEST_URL=postgres://user:senha@127.0.0.1:porta/banco` — usado no CI com service container; o `psql` roda no runner.
- `WYD_DB_TEST_CONTAINER=<nome>` — usado no desenvolvimento; o `psql` roda via `docker exec`.

No CI, `scripts/ci/db-prepare.sh` cria os papéis e `scripts/ci/apply-series.sh` aplica só os blocos `up`. O container é descartável; nenhuma senha é de produção.

## 4. Cache, artefatos e limites do plano gratuito

| Recurso | Limite | Uso neste projeto |
|---|---|---|
| Runners padrão em repo público | gratuitos | todos os jobs |
| Cache | 10 GB/repo, 7 dias por entrada | `rust-cache` por crate; pip via `setup-python` |
| Artefatos | 500 MB (Free), 90 dias padrão | cobertura: 2 arquivos, 7 dias |
| GHCR | imagem pública gratuita | `ghcr.io/lucascantarelli/wyd-docs` |
| CodeQL | incluído | default setup |

Não há `sccache`, registry pago, runner self-hosted nem action de cobertura paga.

## 5. Segurança

- Toda action é pinada por SHA; `sha_pinning_required` deve estar ligado na configuração do repositório.
- Dependabot semanal cobre `github-actions`, `cargo` (três crates) e `pip`; sem ele, o SHA congela o runtime (foi a causa do warning de Node 20).
- `GITHUB_TOKEN` começa em `contents: read`; `pages: write`, `packages: write` e `pull-requests: write` são concedidos só no job que precisa.
- `checkout` usa `persist-credentials: false` onde o job não publica.
- `SLACK_WEBHOOK_URL` é secret opcional; sem ele, o step de aviso é ignorado e o gate continua valendo.
- Nenhum segredo é impresso; `WYD_LAUNCHER_SIGNING_KEY` continua restrito ao `release.yml`.

## 6. Regras de branch

A proteção de `develop` e `main` deve exigir:

- PR (sem push direto);
- checks `gate-code`, `gate-db`, `gate-docs`, `gate-security`;
- histórico linear; force push bloqueado;
- bypass apenas para admin, auditado.

Os nomes são os jobs agregadores, não os jobs internos da matriz. Se um job interno for renomeado, a regra não muda.

## 7. Notificação de falha

O job `notificar falha` roda com `if: always() && (failure() || cancelled())` e:

1. escreve um resumo com run, commit e ref em `$GITHUB_STEP_SUMMARY`;
2. se `SLACK_WEBHOOK_URL` existir, posta uma mensagem curta via `curl` + `jq`.

O secret é configurado em **Settings → Secrets and variables → Actions**. Sem ele, não há Slack, mas o run continua vermelho e o check bloqueia.

## 7.1 Transição de issues

O workflow `issue-status.yml` mantém a sequência `proposta → aceita → em-andamento → em-revisao → Done`.
Ele roda em eventos de issue e em `pull_request_target` (sem checkout do PR), aplica no máximo uma
label `estado/*` e espelha no campo `Status` do Project quando `PROJECT_TOKEN` existe. A tabela de
eventos, as regras de não regressão e o que fazer sem o PAT estão em `github-mapping.md` §2.2 e
`ADR-OPS-0009`.

## 8. O que falta (F2–F5)

| Fase | Escopo | Critério de saída |
|---|---|---|
| **F2 — imagem** | buildx + cache GHA; runtime nginx unprivileged; base por digest; SBOM/provenance; rollback testado | PR constrói sem push; smoke 200; rollback por SHA reproduzido |
| **F3 — deploy** | contrato sobre Compose; workflow manual com `environment`; OIDC; smoke e rollback; **provedor de VPS a definir** | nenhum segredo de longa duração; deploy não roda sem alvo |
| **F4 — observabilidade** | métricas de duração/fila/falha; relatório semanal; sentinelas de advisory e imagem | falha notifica em menos de 1 min; relatório semanal publicado |
| **F5 — governança** | ruleset aplicado; allowlist de actions; CODEOWNERS; onboarding e ADRs revisados | PR vermelho não mergeia; action fora da allowlist falha |

F2 depende de decisão sobre runtime (nginx) e de medir o build; F3 depende do fornecedor de VPS, não de código.

## 9. Solução de problemas

| Sintoma | Causa provável | Ação |
|---|---|---|
| check obrigatório em "Expected" | workflow com filtro de caminho | remover o filtro ou não exigir o check |
| `psql: command not found` no CI | runner sem cliente | o step instala `postgresql-client`; verificar o log do job `Garantir o psql` |
| `role "wyd_migrator" does not exist` | papéis não criados antes do alvo | rodar `scripts/ci/db-prepare.sh migrator` com `PGUSER=postgres` |
| `WYD_DB_TEST_URL` ignorada | harness sem `psql` no PATH | instalar cliente ou usar `WYD_DB_TEST_CONTAINER` |
| aviso de Node 20 | action antiga pinada | atualizar o SHA e rodar Dependabot; não usar `ACTIONS_ALLOW_USE_UNSECURE_NODE_VERSION` |
| site não responde no Compose | porta ocupada ou healthcheck falhando | `docker compose logs docs`; `docker compose up -d --wait docs` |

## 10. Skills que o agente deve carregar

| Trabalho | Skill |
|---|---|
| Alterar workflow, Dockerfile ou Compose | `github-actions-docs` (documentação oficial) + `auditar-planejamento` |
| Redigir ou revisar esta página | `documentation-writer` |
| Decidir arquitetura de pipeline | `planejar-sistema` + aprovação do dono |
| Auditar afirmação sem evidência | `auditar-planejamento` |

## 11. O que esta página não cobre

- Deploy de produção: não existe; ver `ADR-OPS-0007`.
- Valores de segredo, DSN ou webhook.
- Gate de sistema por crate além do que `gate-code`/`gate-db` já provam.
- E2E de portal/launcher: não há aplicação executável que justifique o job hoje.

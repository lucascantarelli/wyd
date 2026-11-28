# DevOps — CI/CD, containers e operação local

> Pergunta: "quais workflows existem, o que cada gate prova, como rodar local e como reverter?"
> Público: Desenvolvimento, operação, arquitetura e agentes
> Dono: `wyd-ops`
> Base: `.github/workflows/*`; `Makefile`; `docker-compose*.yml`; `wyd-docs/gate.sh`; ADRs `ADR-OPS-0003`–`0012`
> Revisado em: 2026-11-27

**Estado:** aceito (2026-11-26; política de dependências e de pin medida em 2026-11-27; sentinelas agendadas fechadas em 2026-11-27) — F0 + F1 implementados; F2 com imagem publicada, atestados e rollback reproduzido (runtime nginx ainda pendente); F3 pendente; F4 com as duas sentinelas agendadas no ar e métricas/relatório semanal pendentes; F5 com rulesets, allowlist e ADR-OPS-0012 aplicados. Nenhum deploy de produção existe. Esta página absorveu o antigo `plano-ci-real.md`, removido em 2026-11-26; a história não é mantida em paralelo.

## 1. O que está no ar

| Workflow | Gatilho | Jobs | Check obrigatório |
|---|---|---|---|
| `code.yml` | todo push/PR em `develop`/`main` | `fmt` (matriz 3 crates), `clippy+unit` no mesmo job por crate — db e launcher em Ubuntu, signer em Ubuntu e Windows; `wyd-db` roda `--lib` + `--test contrato_api_migrate` + `--test contrato_api_provision` —, `cobertura` (matriz: `wyd-db` informativa, `wyd-launcher` com o piso de linha do plano do crate), `esqueleto`, `gate-code`, `notificar resultado` | `gate-code` |
| `ci-db.yml` | todo push/PR em `develop`/`main` | `pg` (matriz `schema`, `reversibility`, `runner` com `postgres:17` por digest), `gate-db`, `notificar resultado` | `gate-db` |
| `docs.yml` | todo push/PR em `develop` | `gate-docs`, `publicar` (Pages, só push de `develop`), `notificar resultado` | `gate-docs` |
| `docs-image.yml` | caminhos de docs/imagem | Buildx + cache GHA, smoke HTTP 200, push GHCR (só `develop`) com provenance/SBOM | não (obrigatoriedade é decisão da F2) |
| `security.yml` | todo push/PR + schedule semanal | `analise` (audit/deny em cada crate), `portal` (npm audit do `wyd-portal`), `pinos` (action por SHA, imagem por digest), `dependency-review` (PR), `workflow-lint` (actionlint), `gate-security`, `notificar resultado` | `gate-security` |
| `code-stable.yml` | schedule diário + dispatch | testes de `wyd-db` e `wyd-ops-signer` com Rust `stable`; sinal não bloqueante | não |
| `issue-status.yml` | issue aberta/rotulada/fechada/reaberta; PR aberto/ready/mergeado/fechado | `sincronizar`: label `estado/*` + espelho no Project; exceção de `pull_request_target` escopada (ADR-OPS-0009/0012) | não (automação, não gate) |
| `release.yml` | tag `v*` | assina e publica manifesto | não |
| `images.yml` | schedule semanal (quarta) + dispatch | `frescor`: compara o digest pinado de cada base com o digest atual da tag via `check-image-freshness.sh`; `notificar resultado` | não (sinal agendado, não check de PR) |

`full-workflow.yml` foi removido: era um stub com os mesmos gatilhos do CI real.

### Prova real do pacote 1A–11A (2026-11-26)

- PR [#205](https://github.com/lucascantarelli/wyd/pull/205), squash em `0c4a612`: `gate-code`, `gate-db`, `gate-docs` e `gate-security` concluíram **success** na execução real do PR; CodeQL, GitGuardian e `docs-image` de PR também passaram.
- `docs-image` em `develop`: run [`45827609542`](https://github.com/lucascantarelli/wyd/actions/runs/45827609542), commit `0c4a612`, **success** em 50 s; **naquela publicação**, `:SHA`, `:develop` e `:latest` apontaram para `sha256:462029e…`; o manifesto de atestação contém SPDX/SBOM e provenance SLSA v1; o smoke respondeu HTTP 200.
- `code-stable`: run [`45828077654`](https://github.com/lucascantarelli/wyd/actions/runs/45828077654) (dispatch em `develop`), **success**; `wyd-db` e `wyd-ops-signer` concluíram com Rust `stable`.
- Rollback real: `make rollback TAG=f271caab…` e `TAG=0c4a612a…` puxaram imagens publicadas e responderam HTTP 200, sem build da árvore; digests `sha256:f789081…` e `sha256:462029e…`.
- Os logs dos runs recentes de `code`, `docs`, `docs-image`, `security`, `ci-db` e `code-stable` não contêm o aviso de Node 20 nem a SHA `ea165f8…`.

### Por que os workflows obrigatórios não têm filtro de caminho

Um check obrigatório filtrado por caminho fica em **Expected** em PR que não toca aquele caminho e bloqueia o merge para sempre. Por isso `code`, `ci-db`, `docs` e `security` rodam em todo PR; o custo é baixo em repositório público e o bloqueio eterno é pior. `docs-image` continua filtrado porque não é obrigatório nesta fase. Os quatro agregadores estão aplicados nos rulesets de `develop` e `main` desde 2026-11-26; `strict` é `false` em `develop` e `true` em `main`.

## 2. O que cada gate prova

- **`gate-code`**: formatação, lint `-D warnings`, testes unitários dos três crates e os contratos públicos de `migrate`/`provision` (`wyd-db --test contrato_api_migrate --test contrato_api_provision`); os três crates são portáveis — o núcleo do `wyd-launcher` é Rust puro, sem Tauri nem `cfg(windows)` —, e só o empacotamento NSIS futuro do launcher exigirá `windows-latest`. Cobertura é informativa em `wyd-db` (`D-02 = B`) e, desde 2026-11-27, bloqueante em `wyd-launcher` pelo piso de linha do plano do próprio crate; cada crate sobe `coverage-summary.txt` + `lcov.info` por 7 dias.
- **`gate-db`**: os três alvos PostgreSQL. `schema` aplica a série e roda `sistema_pg`; `reversibility` prova `up → down → up`; `runner` prova a porta `PgAdapter` e o advisory lock com `wyd_migrator` LOGIN.
- **`gate-docs`**: `wyd-docs/gate.sh` — roda primeiro os testes unitários dos verificadores e depois o pacote `python -m gate` (skills, evidência de lacunas, montagem, build MkDocs, links/âncoras/navegação, contagens e censos). A saída é um resumo por domínio; `--json` é o contrato do agente (ADR-OPS-0010).
- **`gate-security`**: `cargo audit` e `cargo deny` em cada crate; `npm audit --omit=dev --audit-level=high` no lockfile do `wyd-portal` (o audit completo roda informativo); `pinos` roda `wyd-ops/scripts/ci/check-pins.sh` — action sem SHA de 40 hex ou imagem de terceiro sem `@sha256:` reprova; em PR, `dependency-review` falha em severidade `high`; `workflow-lint` roda actionlint 1.7.12 em imagem por digest.
- CodeQL continua em **default setup** do GitHub (actions, JS/TS, Python, Rust); não há workflow CodeQL duplicado.

## 3. Como rodar local

```bash
make gate        # mesmo gate do CI, dentro da imagem :gate (testes + gate full)
make gate-fast   # checks nativos (layout, alvos, capacidades, skills, lacunas, contagens, censos, hoje), sem Docker/MkDocs
make test-gate   # atalho isolado para os testes de fixture dos verificadores
make fmt         # cargo fmt --check nos três crates
make lint        # cargo clippy -D warnings nos três crates
make test        # testes unitários (wyd-db --lib + contratos de migrate/provision; signer e launcher --locked)

make docs-up     # puxa a imagem publicada e sobe com --no-build em 127.0.0.1:8000
make docs        # constrói da árvore de trabalho (dev, tag :dev)
make docs-down   # para o site

make db-up       # PostgreSQL 17 local em 127.0.0.1:5432, tmpfs
make db-down     # para e descarta o banco local
make rollback TAG=<sha>   # puxa a tag e sobe com --no-build, sem construir a árvore
```

Depuração por linha de comando (sem UI), com o mesmo alvo das configs de `.vscode/`:
[`depuracao-local.md`](depuracao-local.md) — evidência executada em 2026-11-26, breakpoint atingido em
`wyd-server` e `wyd-admin`.

O gate local usa a imagem `:gate` construída da árvore (`pull_policy: build`); o CI usa `setup-python` + `wyd-docs/gate.sh`. A ordem dos verificadores é a mesma nos dois, e `gate.sh` roda os testes unitários antes do `gate full`. O teste paralelo-vs-sequencial é executado no Linux e no macOS, onde `fork` existe; o **skip é apenas do Windows**, porque não há `fork` lá. A imagem copia a árvore de evidência no build e roda `/app` **sem bind mount**, que é a condição da medição controlada abaixo; as duas últimas execuções após as edições de 2026-11-26 ficaram em **24,2–27,0s** (494 páginas); a medição controlada de 480 páginas permanece **20,5–21,5s**. Medições com bind mount pertencem a outra condição e não são repetidas aqui. Cliente e legado continuam fora do contexto por `.dockerignore`.

### Medição do gate completo e do check de links (2026-11-26)

Ambiente: Docker Desktop 29.9.3, imagem `python:3.15-slim` (Python 3.15.2), 8 CPUs e 16 GB, host Windows; `MSYS_NO_PATHCONV=1` no Git Bash para preservar `/app`. A mesma árvore de 480 páginas e os mesmos argumentos foram usados nos dois lados: a imagem construída de um worktree de `f271caa` (`git worktree add .gate-antes f271caa`, removido após a medição) e a árvore de trabalho. A partir da raiz:

```bash
docker build -f wyd-ops/wyd-docs.Dockerfile --target gate -t wyd-gate-antes:local .
docker run --rm wyd-gate-antes:local python -m gate full --root /app --json
docker run --rm wyd-gate-antes:local bash -c "python preparar.py && mkdocs build --dir .build/docs --output site && python -m gate check links --root /app --json"
```

Para o lado atual, os mesmos comandos com a tag `wyd-gate-depois:local`, construída da árvore de trabalho. Cada lado foi executado duas vezes; a tabela registra as duas medições, não uma estimativa.

| Medição | Antes (f271caa) | Depois (árvore atual) | Fração de `links` no gate |
|---|---|---|---|
| Gate completo | 35,9s e 40,1s | 21,5s e 20,5s | — |
| `links` dentro do gate completo | 25,0s e 28,0s | 9,4s e 8,6s | 69,6%/69,9% → 43,5%/41,9% |
| `links` isolado | 23,9s e 27,8s | 8,8s e 8,1s | — |

Estado, resumo e lista de problemas são idênticos nas oito execuções: PASS, `480 páginas, 478 fontes`, zero problemas. O `build` (9,5–10,1s depois) passou a ser o maior domínio; `links` deixou de dominar o total. A primeira etapa (índice do site em memória, hash por string normalizada e limpeza única de code span) levou o gate a 25,2–29,4s e `links` a 11,9–16,0s; a segunda (parse HTML em processos com `fork` explícito, sem reimportar o módulo principal) fechou em 20,5–21,5s. O parse sequencial de 480 páginas leva 10,9s no container e 5,6s com 8 processos.

A medição de ~43s citada antes desta revisão não pôde ser reproduzida: não há comando ou condição de ambiente registrados, e a própria imagem pré-otimização mede 25,0–28,0s de `links` e 35,9–40,1s de gate completo nesta condição. Este documento registra apenas as medições acima.

O ciclo curto é nativo e não depende de Docker/MkDocs porque o `mkdocs==2.0.dev6` usa `PosixPath` e não roda no Windows. O caminho completo é o do CI/container.

### Testes PostgreSQL locais

O harness aceita dois modos:

- `WYD_DB_TEST_URL=postgres://user:senha@127.0.0.1:porta/banco` — usado no CI com service container; o `psql` roda no runner.
- `WYD_DB_TEST_CONTAINER=<nome>` — usado no desenvolvimento; o `psql` roda via `docker exec`.

No CI, `wyd-ops/scripts/ci/db-prepare.sh` cria os papéis e `wyd-ops/scripts/ci/apply-series.sh` aplica só os blocos `up`. O container é descartável; nenhuma senha é de produção.

### Mapa dos scripts

| Arquivo | Linguagem | Papel | Entrada / saída |
|---|---|---|---|
| `wyd-docs/gate.sh` | bash | porta estável do gate; delega ao pacote | raiz opcional; exit 0/1/2 |
| `wyd-docs/gate/cli.py` | Python | `full`, `fast`, `check` e saída/JSON | raiz; exit 0/1/2 |
| `wyd-docs/gate/workspace.py` | Python | índice e cache compartilhados | uma instância por execução |
| `wyd-docs/gate/checks/links.py` | Python | links, âncoras, navegação e sintaxe 1.x | `site/` |
| `wyd-docs/gate/checks/lacunas.py` | Python | evidência de lacuna e placar | raiz |
| `wyd-docs/gate/checks/contagens.py` | Python | contagens publicadas | raiz |
| `wyd-docs/gate/checks/censos.py` | Python | censo de fontes e atos | raiz |
| `wyd-docs/gate/checks/skills.py` | Python | skills e páginas de issue | raiz |
| `wyd-docs/tests/test_gate.py` | Python | testes de fixture das regras e do JSON | `python -m unittest discover -s tests` |
| `wyd-docs/tests/fixtures.py` | Python | árvores mínimas de link, lacuna, contagem, censo e skills | importado pelos testes |
| `wyd-docs/preparar.py` | Python | monta `.build/docs` e a navegação | árvore do repositório |
| `wyd-ops/scripts/ci/db-prepare.sh` | bash | cria papéis do PostgreSQL de teste | URL/psql |
| `wyd-ops/scripts/ci/apply-series.sh` | bash | aplica só os blocos `up` da série | URL/psql |
| `wyd-ops/scripts/ci/check-pins.sh` | bash | política de pin: action por SHA de 40 hex, imagem de terceiro por digest | raiz; exit 0/1 |
| `wyd-ops/scripts/ci/check-image-freshness.sh` | bash | frescor da base pinada: digest do arquivo contra o digest da tag no registro | raiz; exit 0/1 |
| `wyd-ops/scripts/sync-issue-status.mjs` | Node | labels e Project da issue | evento do GitHub |
| `wyd-ops/tests/unitario/sync-issue-status.test.mjs` | Node | regras da transição de estado | `node --test wyd-ops/tests/unitario/` |

Os composites de `.github/actions/` só instalam ferramentas e chamam esses arquivos; não duplicam regra. Um script novo fora deste mapa precisa de issue e dono.

## 4. Cache, artefatos e limites do plano gratuito

| Recurso | Limite | Uso neste projeto |
|---|---|---|
| Runners padrão em repo público | gratuitos | todos os jobs |
| Cache | 10 GB/repo, 7 dias por entrada | `rust-cache` por crate; pip via `setup-python`; Buildx `type=gha` com escopo `docs-image` |
| Artefatos | 500 MB (Free), 90 dias padrão | cobertura: 2 arquivos por crate, 7 dias |
| GHCR | imagem pública gratuita | `ghcr.io/lucascantarelli/wyd-docs` com SHA, `develop` e `latest` em `develop` |
| CodeQL | incluído | default setup |

Não há `sccache`, registry pago, runner self-hosted nem action de cobertura paga.

## 5. Segurança

- Toda action é pinada por SHA; `sha_pinning_required=true` e `allowed_actions=selected` estão aplicados no repositório. A allowlist cobre GitHub, `Swatinem/rust-cache`, `taiki-e/install-action`, `dtolnay/rust-toolchain` e `docker/*`.
- Dependabot semanal cobre `github-actions`, `cargo` (três crates), `npm` (`/wyd-portal`), `pip`, `docker` (Dockerfile do site) e `docker-compose`; sem ele, o SHA congela o runtime (foi a causa do warning de Node 20).
- Imagem de terceiro entra por tag **e** digest em Dockerfile, Compose e workflow; `check-pins.sh` reprova tag móvel. As imagens do próprio repositório (`wyd/*` e `ghcr.io/<dono>/wyd-docs`) ficam fora: a tag delas é contrato de publicação.
- `GITHUB_TOKEN` começa em `contents: read`; `pages: write`, `packages: write` e `pull-requests: write` são concedidos só no job que precisa.
- `checkout` usa `persist-credentials: false` onde o job não publica.
- `SLACK_CI_WEBHOOK` é o secret opcional; os workflows o expõem como `SLACK_WEBHOOK_URL` para a action. Sem ele, o step de aviso é ignorado e o gate continua valendo.
- Nenhum segredo é impresso; `WYD_LAUNCHER_SIGNING_KEY` continua restrito ao `release.yml`.

### 5.1 Política de dependências e retenções (2026-11-27)

Medição de 2026-11-27: API pública do crates.io para os crates, `npm view` e `npm audit` para o
portal, `docker buildx imagetools inspect` para as bases e `rustup check` para a toolchain (que
estava em 1.100.0 e tinha 1.100.1 disponível). A regra é subir só o que cabe no mesmo major; major
com breaking fica retido, com o motivo escrito aqui e o caminho de saída.

| Dependência | Declarado | Última stable | Decisão |
|---|---|---|---|
| toolchain Rust | 1.100.0 | 1.100.1 | **atualizada** (patch) |
| `sha2` (`wyd-db`) | 0.11.1 | 0.11.2 | **atualizada** |
| `regex` (`wyd-launcher`, dev) | 1.13.1 | 1.13.2 | **atualizada** |
| `sqlx` (`wyd-db`) | 0.8.6 | 0.9.1 | retida: `0.9` é breaking e o adapter é a única fronteira com o banco; a troca exige re-provar a série e os três alvos PostgreSQL, então vai em issue própria |
| `ed25519-dalek` (launcher e signer) | 2.2.0 | 3.0.0 | retida: `3.0` muda a API de assinatura e obriga a re-provar as fixtures byte-exatas e o manifesto assinado |
| `sha2` (`wyd-launcher`) | 0.10.9 | 0.11.2 | retida: `0.11` quebra a API de digest usada no hash de integridade; sobe junto com a re-prova das fixtures |
| `thiserror`, `tokio`, `tempfile`, `serde`, `serde_json` | — | iguais | nada a fazer |
| `next`, `react`, `react-dom` (`wyd-portal`) | 16.4.6 / 19.3.1 | iguais | nada a fazer; `npm audit` = 0 vulnerabilidades |
| `mkdocs` (`wyd-docs`) | 2.0.dev6 | pré-release | retido: o site é construído no 2.0 dev; Dependabot pip acompanha |
| `postgres:17` | digest anterior | digest atual | **atualizado** (patch 17.x); provado nos alvos `pg/*` |
| `python:3.15-slim` | digest anterior | digest atual | **atualizado**; provado pelo `make gate` completo |
| `rust:1.100-slim-bookworm` | tag | digest | **pinado por digest**; a tag `1.100` acompanha a toolchain |
| `debian:bookworm-slim` | tag | digest | **pinado por digest** |
| `node:22-alpine` | tag | digest | **pinado por digest**; runtime do portal |

Cada retenção tem motivo e saída declarados; nenhuma sobe só porque existe versão mais nova.

O digest pinado não se mantém atual sozinho: `check-pins.sh` prova que existe pin, e o
`check-image-freshness.sh` prova que o pin ainda é o da tag. O primeiro é determinístico e roda em todo PR;
o segundo depende de registro e roda no `images.yml` semanal, para não trazer rede ao gate de PR. CVE
dentro da imagem base continua sem scanner: o repositório não tem Trivy nem outra ferramenta instalada,
e a única menção é proposta em `proposta-gestao-github-e-ambientes.md`. Ligar um scanner exige
decisão de allowlist de action, que é ato do dono.

## 6. Regras de branch

A proteção de `develop` e `main` está aplicada e exige:

- PR (sem push direto);
- checks `gate-code`, `gate-db`, `gate-docs`, `gate-security`;
- histórico linear; force push bloqueado;
- bypass apenas para admin, auditado.

`develop` usa `strict=false` para não exigir rebase a cada merge; `main` usa `strict=true`. Os nomes são os jobs agregadores, não os jobs internos da matriz. `main` está arquivada e não é publicada enquanto o release 1.0.0 não reconciliar a história (ADR-OPS-0012).

## 7. Notificação de falha

O job `notificar resultado` roda com `if: always()` — em sucesso e em falha — e executa a action
composta `.github/actions/resultado`:

1. lê os jobs do run pela API (`actions: read`) e monta uma tabela com status e tempo de cada job, a
   soma dos jobs e run, commit, ref e evento, publicada em `$GITHUB_STEP_SUMMARY`;
2. se `SLACK_WEBHOOK_URL` existir, posta o mesmo resumo com selo de sucesso/falha, os três jobs mais
   lentos e um botão para o run.

O job lê o secret `SLACK_CI_WEBHOOK` e o expõe como `SLACK_WEBHOOK_URL`. O secret é configurado em **Settings → Secrets and variables → Actions**. Sem ele, não há Slack, mas o run continua vermelho e o check bloqueia.

## 7.1 Transição de issues

O workflow `issue-status.yml` mantém a sequência `proposta → aceita → em-andamento → em-revisao → Done`.
Ele roda em eventos de issue e em `pull_request_target` (sem checkout do PR), aplica no máximo uma
label `estado/*` em **uma** requisição `PUT /labels` — a troca é atômica desde 2026-11-26 —, ignora o
evento de label que ele mesmo gerou (autor `github-actions[bot]`) e espelha no campo `Status` do Project
quando `PROJECT_TOKEN` existe. A tabela de eventos, as regras de não regressão e o que fazer sem o PAT
estão em `github-mapping.md` §2.2 e `ADR-OPS-0009`; a regressão unitária roda com
`node --test wyd-ops/tests/unitario/`.

## 7.2 Sinais agendados

Dois sinais rodam fora do PR, de propósito: o gate de PR não depende de rede nem de advisory que muda
sem commit.

| Sinal | Workflow e cadência | O que reprova | Aviso |
|---|---|---|---|
| Advisory, licença e pin | `security.yml`, segunda 06:00 UTC | `cargo audit`/`cargo deny`, `npm audit` do portal e `check-pins.sh` | `gate-security` → `notificar resultado` |
| Frescor de imagem base | `images.yml`, quarta 07:00 UTC | digest pinado diferente do digest da tag (`check-image-freshness.sh`) | `frescor` → `notificar resultado` |
| Toolchain futura | `code-stable.yml`, diário 07:00 UTC | suíte dos três crates com Rust `stable` | `stable` → `notificar resultado` |

Os três usam o mesmo runner padrão e o mesmo `SLACK_CI_WEBHOOK` opcional; sem o secret, o run fica
vermelho do mesmo jeito e o aviso no Slack é ignorado. `workflow_dispatch` permite rodar cada um na hora.

## 8. O que falta (F2–F5)

| Fase | Escopo | Critério de saída |
|---|---|---|
| **F2 — imagem** | Buildx + cache GHA, bases por digest, provenance/SBOM, tags por branch e rollback por SHA **feitos**; runtime nginx unprivileged pendente | PR constrói sem push; smoke 200; rollback por SHA reproduzido |
| **F3 — deploy** | contrato sobre Compose; workflow manual com `environment`; OIDC; smoke e rollback; **provedor de VPS a definir** | nenhum segredo de longa duração; deploy não roda sem alvo |
| **F4 — observabilidade** | métricas de duração/fila/falha e relatório semanal pendentes; sentinela de advisory **feita em 2026-11-27** (`security.yml` semanal com `notificar resultado`); sentinela de imagem **feita em 2026-11-27** (`images.yml` semanal com `check-image-freshness.sh`) | falha das duas sentinelas notifica em menos de 1 min; relatório semanal publicado |
| **F5 — governança** | ruleset, allowlist e ADR-OPS-0012 **feitos**; CODEOWNERS, onboarding e revisão de ADRs pendentes | PR vermelho não mergeia; action fora da allowlist falha |

F2 depende de decisão sobre runtime (nginx) e de medir o build; F3 depende do fornecedor de VPS, não de código.

## 9. Solução de problemas

| Sintoma | Causa provável | Ação |
|---|---|---|
| check obrigatório em "Expected" | workflow com filtro de caminho | remover o filtro ou não exigir o check |
| `psql: command not found` no CI | runner sem cliente | o step instala `postgresql-client`; verificar o log do job `Garantir o psql` |
| `role "wyd_migrator" does not exist` | papéis não criados antes do alvo | rodar `wyd-ops/scripts/ci/db-prepare.sh migrator` com `PGUSER=postgres` |
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

## 12. Proposta de centralização do CI — veredito (2026-11-28)

Revisão das propostas de reorganização do CI, com veredito por item. Medição de 2026-11-27: `code`
103–138 s, `ci-db` 69–83 s, `security` 41–49 s, `docs` 29–44 s — o que estoura vem de `clippy+unit`
Windows (53 s) e `cobertura` (41 s), não da estrutura. Em 2026-11-28 o pipeline virou **um arquivo**
(`.github/workflows/ci.yml`), com sete actions em `.github/actions/` e **um** job de aviso por run.

### 12.1 Feito

- **Um run por evento, um aviso no fim.** Todo o pipeline de PR/push está em `ci.yml`: código, banco,
  documentação, segurança e imagem no mesmo run, ligados por `needs`, e o job `notificar` como último
  estágio. Antes eram oito workflows, oito runs e oito avisos por push — o defeito era estrutural, não
  de formato.
- **Jobs planos, de propósito.** Chamar domínio como reusable workflow (`uses:`) renomeia o check para
  `code / gate-code`, e o ruleset de `develop` exige `gate-code`, `gate-db`, `gate-docs` e
  `gate-security` com o nome exato. Reusable serve para reaproveitar **passos** (as actions); para
  preservar o nome do check obrigatório, os jobs moram no mesmo arquivo.
- **Um relatório por run, para sucesso e falha.** A action `.github/actions/resultado` lê os jobs pela
  API (`actions: read`) e monta uma vez o texto com veredito, run/commit/ref/evento, contagem de jobs,
  soma das durações e a tabela por job (maior primeiro). O texto vai para `$GITHUB_STEP_SUMMARY`, para
  o **log do job** e para o artefato `resultado-ci` — o summary não é legível pela API, log e artefato
  são. No Slack vão o selo, os três jobs mais lentos e o botão para o run; o segredo
  `SLACK_CI_WEBHOOK` existe no repositório (aparece mascarado no log) e o `curl --fail` do passo
  fecha com sucesso. Job sem conclusão na coleta (o próprio job de aviso) não entra no total nem no
  veredito.
  Comprovação: run `46091921212` (`pull_request`, commit `6ced372`) — artefato `resultado-ci` com
  `### ✅ ci — sucesso`, `| Jobs | 33 (✅ 32 · ❌ 0 · ⏭️ 1) |`, `| Soma dos jobs | 762s |`, e o run tem
  **um** job `notificar resultado`; no PR, os checks `gate-code`, `gate-db`, `gate-docs` e
  `gate-security` aparecem com o nome exigido pelo ruleset.
- **`gate-agregado`.** Os três laços de `gate-code`, `gate-db` e `gate-security` viraram
  `.github/actions/gate-agregado`, que recebe `nome=resultado` e, no `security`, ignora
  `dependency-review` fora de `pull_request` — o job que só existe em PR não reprova push nem agenda.
  O check obrigatório continua sendo o `gate-*`, nunca os jobs da matriz.
- **`testes-rust`.** `.github/actions/testes-rust` (setup + `fmt` opcional + clippy com `-D warnings` +
  test) atende `verify` e `esqueleto`; `args` e `fmt` vêm da matriz, então crate novo ou alvo novo não
  copiam passos.
- **`testes-pg`.** `.github/actions/testes-pg` prepara o `psql`, cria os papéis, aplica a série quando
  pedida e roda o alvo. O service container `postgres:17` por digest continua no workflow, o único
  lugar que pode declará-lo.
- **`imagem`.** `.github/actions/imagem` é buildx + cache GHA por escopo + login/push com
  `GITHUB_TOKEN` + fumaça HTTP. Os jobs `imagem-docs` e `imagem` (matriz `wyd-server`, `wyd-admin`,
  `wyd-portal`) usam a mesma action; os três serviços tinham Dockerfile e nenhum workflow, então o
  Dockerfile nunca era exercitado. Em PR constrói e faz fumaça (`/health` nas duas sondas, `/` no
  portal); em push de `develop` publica `:sha` e `:dev`. `latest` segue exclusiva do site e **não há
  deploy em tag** (ADR-OPS-0012).
- **Scan de CVE da imagem (Trivy).** Roda dentro da action `imagem`, na imagem recém-construída:
  `--vuln-type os --severity HIGH,CRITICAL --ignore-unfixed --exit-code 1`, ou seja, falha o job só no
  que tem correção disponível. Não é `aquasecurity/trivy-action`: a allowlist de actions do repositório
  aceita só `docker/*`, `dtolnay/*`, `Swatinem/*` e `taiki-e/*`, então o binário roda pela imagem
  `aquasec/trivy` pinada por digest, com `--image-src docker` (usa o daemon local, sem pull de
  registro). O primeiro run (commit `2ae25e3`) encontrou CVE-2025-6020 (`libpam`, HIGH, correção em
  `1.5.2-6+deb12u3`) em `wyd-server`/`wyd-admin` e CVE-2026-124851 (`libzstd1`) no site: o digest
  pinado de `debian:bookworm-slim` não trazia o pacote já publicado no arquivo. O runtime das três
  imagens aplica `apt-get update && apt-get upgrade` antes de copiar o binário, e `wyd-server` e
  `wyd-docs` foram reconstruídos e escaneados à mão: `0` HIGH/CRITICAL. Run de fecho: `46095134592`
  (33 jobs, ✅ sucesso, artefato `resultado-ci`).
- **Aviso rico, no nível do `deprecated`.** O relatório traz veredito, ref/commit/ator, **parede** do
  run (`run_started_at` → `updated_at`), jobs ok/falha/pulado, **soma dos jobs** e o **paralelismo
  efetivo** (soma ÷ parede), os cinco mais lentos e a lista de falhas com link do log de cada job. O
  Slack usa Block Kit com cor por resultado (verde/vermelho), emojis e campos; a cobertura por crate
  entra quando o artefato `cobertura-*` existe. O resumo em Markdown tem tabela com link por job e vai
  para o summary, para o log e para o artefato `resultado-ci`.
  Comprovação: run `46097685728` — artefato `resultado-ci` com `Parede 1m 21s`, `Jobs 33 (✅ 32 · ❌ 0 ·
  ⏭️ 1)`, `Soma 13m 24s`, `Paralelismo 9,9×` e `Cobertura wyd-db 67,77% · wyd-launcher 92,48%`; o passo
  do Slack logou `aviso montado: ✅ ci passou (33 jobs, parede 1m 21s, 0 falha(s))` e `aviso enviado ao
  Slack`. A parede é medida pelos jobs (primeiro início → último fim), porque `run_started_at` vem nulo
  em `pull_request`.
- **Metadados de PR e de marco, sem action de terceiro.** `wyd-ops/scripts/sync-pr-metadata.mjs` (job
  `metadados` do `issue-status`) aplica `area/*` por caminho, `size/*` pelo diff e herda o milestone da
  issue que o PR fecha; `wyd-ops/scripts/ci/fechar-milestone.sh` (job `marco` do `release`) fecha o
  marco da versão final e abre o minor seguinte, ignorando `-rc.N`. Ambos usam `gh`/API com o
  `GITHUB_TOKEN` do job, porque a allowlist de actions só aceita `docker/*`, `dtolnay/*`, `Swatinem/*`
  e `taiki-e/*`.
- **Custo aceito.** As imagens rodam em todo PR, sem filtro de caminho: Dockerfile quebrado só aparece
  quando alguém builda, e um build em matriz paralela custa menos que a imagem errada.
- **Agenda.** `ci.yml` também roda semanalmente (segunda, 06:00 UTC): o advisory envelhece sem commit e
  o run agendado reaproveita os mesmos gates. `code-stable` (diário, toolchain `stable`) e `images`
  (frescor de digest, quarta) seguem workflows próprios, cada run com o seu aviso.

O que isso dá de DORA sem serviço externo: duração por job e soma por run (lead time de integração),
taxa de falha por run e o link direto para o run lento.

### 12.2 Viável e não feito

Nada em aberto. O que restava da lista de 2026-11-27 foi fechado em 2026-11-28: tag de release e
changelog (§13), scan de CVE na action `imagem` e CODEOWNERS em `.github/CODEOWNERS`.

### 12.3 Não viável como proposto

- **Rodar só nos arquivos alterados com check obrigatório.** Filtro de caminho em workflow cujo check é
  obrigatório deixa o check em `Pending` e trava o merge — o mesmo motivo da §1. Alternativa: manter o
  check obrigatório e encurtar o job (cache, matriz, `fail-fast`), ou filtrar **dentro** do workflow e
  manter um agregador sempre presente (o filtro externo exigiria action nova, sujeita à allowlist).
- **Um workflow por estágio (lint, build, test).** Aumenta o fan-out de checks e a fila; legibilidade se
  ganha com estágios **dentro** do workflow via `needs`, que é o que a §1 já faz por domínio.
- **`master` como produção rodando tudo.** O padrão é `develop` e `main` está arquivada (ADR-OPS-0012);
  trocar o modelo é decisão de arquitetura.
- **Testes de performance.** Não há alvo nem harness; criar agora seria inventar escopo de jogo sem
  página aceita.

### 12.4 Pendente por decisão do dono

Runtime nginx (F2) e provedor de VPS/OIDC (F3): nenhum é bloqueio técnico do CI atual. Em 2026-11-28 o
dono decidiu os três itens que faltavam — SemVer `vX.Y.Z[-rc.N]` (§13), CODEOWNERS com dono único
(`.github/CODEOWNERS`) e Trivy na action `imagem` (sistema operacional da imagem, HIGH/CRITICAL com
correção disponível, falha o job). F5 fecha com isso.

## 13. Política de versionamento e release (2026-11-28)

Decisão do dono: SemVer, com a tag como fonte da versão. Vale para tag, release e imagem.

- **Formato.** `vX.Y.Z` para release e `vX.Y.Z-rc.N` para candidato; `0.y.z` até o `1.0.0`. A tag é
  anotada e nasce em `main`, a branch de release (ADR-OPS-0012).
- **Validação.** O job `release` confere o nome da tag contra `^v[0-9]+\.[0-9]+\.[0-9]+(-rc\.[0-9]+)?$`
  antes de assinar o manifesto: tag fora do padrão é recusada, não vira release.
- **Changelog.** As notas saem de `gh release create --generate-notes` (PRs mergeados desde a tag
  anterior) e sobem junto do manifesto assinado, no mesmo job.
- **Marco.** Release final fecha o milestone da versão (`X.Y.Z` ou `vX.Y.Z`) e abre o minor seguinte;
  `-rc.N` não fecha nada. O job avisa quando o marco ainda tinha issue aberta e fecha assim mesmo — a
  tag é a fonte da versão.
- **Imagem.** `:sha` e `:dev` acompanham `develop`; `:latest` é exclusiva do site enquanto não existe
  release. Imagem com a tag da versão entra quando a primeira tag `v*` for criada — hoje o CI não
  publica em tag e não há tag no repositório (`git tag --list 'v*'` vazio).
- **Nada de versão dentro do código.** Não há `VERSION` nem workspace: enquanto isso, a tag é a única
  fonte e nenhum `Cargo.toml` é reescrito pelo CI.

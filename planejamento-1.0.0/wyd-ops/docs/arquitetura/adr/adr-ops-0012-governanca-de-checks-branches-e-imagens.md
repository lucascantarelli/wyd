# ADR-OPS-0012 — Governança de checks, branches, Pages e imagens

> Pergunta: "como impedir merge sem gate, publicar a imagem certa e manter o CI sustentável no plano gratuito?"
> Público: Arquitetura, desenvolvimento, operação e segurança
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `docs/devops/index.md`; `docs/planos/pipeline-ci-cd.md`; `ADR-OPS-0003`; `ADR-OPS-0004`; `ADR-OPS-0008`; `ADR-OPS-0009`; `ADR-OPS-0011`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O diagnóstico de 2026-11-26 encontrou quatro lacunas de governança, não de código:

1. os rulesets de `develop` e `main` tinham PR obrigatório, deleção e non-fast-forward, mas **nenhum `required_status_checks`**; o merge podia passar sem os gates;
2. `main` não tem ancestral comum com `develop` e está parada na release 0.1.0; o ambiente `github-pages` aceita somente `develop`, enquanto `docs.yml` e `docs-image.yml` declaravam `main`;
3. `pull_request_target` seria bloqueado por padrão em 2/dez/2026, e o `issue-status.yml` depende dele para escrever labels em PR de fork sem executar código do PR;
4. a imagem do site era construída por `docker build` sem cache, publicava `latest` em qualquer push de `develop` ou `main` e o Compose base mantinha um `build:` de fallback que podia servir a árvore local sob uma tag antiga.

O repositório é público, usa runners padrão do GitHub e não tem provedor de produção. O custo de runner não é o gargalo; a previsibilidade e a segurança são.

## Decisão

1. **Quatro checks obrigatórios**, iguais nos dois rulesets: `gate-code`, `gate-db`, `gate-docs` e `gate-security`. Em `develop`, `strict_required_status_checks_policy=false` para não exigir rebase a cada merge; em `main`, `true` porque a branch é de release. Os nomes são dos jobs agregadores; jobs internos podem mudar sem alterar a regra.
2. **`main` fica arquivada** enquanto não houver release 1.0.0 com história reconciliada. `docs.yml` e `docs-image.yml` publicam somente `develop`; o ambiente `github-pages` continua aceitando somente `develop`; `latest` acompanha `develop`. A migração para `main` é uma decisão separada, no release.
3. **Exceção de evento escopada** para `.github/workflows/issue-status.yml`, permitindo `issues`, `pull_request_target` e `workflow_dispatch`. O workflow continua sem checkout do head do PR; a exceção não autoriza executar código não confiável.
4. **Imagem do site**: Buildx com cache `type=gha`, tags `SHA` sempre, `develop` em `develop` e `latest` em `develop`; PR constrói e carrega localmente, sem push; push publica com `provenance` e `sbom`; smoke HTTP 200 antes de considerar a entrega válida. Bases `python` e `postgres` ficam por digest e o Dependabot cobre Docker/Docker Compose.
5. **Compose base não constrói**: `docs` consome a imagem publicada; o build da árvore fica somente em `docker-compose.dev.yml`. `make docs-up` e `make rollback TAG=<sha>` fazem `pull` e `up --no-build`; `container_name` foi removido para não colidir entre checkouts.
6. **Sem Dockerfile universal de CI** enquanto os runners hospedados + `rust-toolchain.toml` + composites entregarem o gate em menos de dois minutos. Um `ci-rust` só entra se houver dependência nativa ou divergência de runner medida.
7. **Actionlint dentro do `gate-security`**, com imagem fixada por digest. Não cria um quinto check e impede YAML inválido de chegar ao merge.
8. **Supply chain**: `sha_pinning_required=true`, allowlist selecionada (GitHub + `Swatinem/rust-cache`, `taiki-e/install-action`, `dtolnay/rust-toolchain`, `docker/*`) e `rust-cache` apontando para o commit `d1527c8...` da tag v2.9.3, não para o objeto da tag anotada.
9. **Matriz estável é sinal, não gate**: `code-stable.yml` roda diariamente com Rust `stable` para `wyd-db` e `wyd-ops-signer`; falha ali não bloqueia PR.
10. **Slack usa o secret existente**: o env `SLACK_WEBHOOK_URL` dos workflows é preenchido com `secrets.SLACK_CI_WEBHOOK`; sem ele, o aviso é ignorado e o gate continua valendo.

## Consequências

- Positivas: merge sem gate deixa de ser possível; Pages e `latest` apontam para a mesma branch; rollback busca a imagem publicada e não constrói a árvore; build de imagem tem cache e atestados; a política de actions passa a ser aplicada, não só documentada; o aviso de falha deixa de depender de um secret inexistente.
- Negativas/custos: `main` não é a branch estável durante esta fase; a exceção de `pull_request_target` precisa ser revista se o workflow mudar; buildx adiciona uma action oficial; `code-stable` pode falhar por mudança do compilador e exige triagem; cache GHA tem limite de 10 GB e retenção de 7 dias.
- Contratos afetados: `.github/workflows/{docs,docs-image,security,code,ci-db,code-stable}.yml`; `docker-compose*.yml`; `Makefile`; `wyd-ops/wyd-docs.Dockerfile`; `dependabot.yml`; configuração de rulesets, permissões e políticas do repositório; `docs/devops/index.md` e `docs/planos/pipeline-ci-cd.md`.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Monolito por estágios | docs-only bloquearia código; tempo de parede maior; ADR-OPS-0003 já rejeitou |
| Reconstruir `main` agora com force push | história sem ancestral comum; operação destrutiva e sem release 1.0.0 para justificar |
| Publicar `latest` em `develop` e `main` | corrida entre branches e site servindo commit divergente |
| Manter `build:` no Compose base | rollback podia construir a árvore atual sob a tag antiga |
| Dockerfile universal de CI | DinD, perda de cache do runner e launcher Windows fora do alcance |
| Deixar `pull_request_target` sem exceção | o workflow de labels pararia em 2/dez/2026 |
| Actionlint em workflow separado | mais um check obrigatório sem ganho sobre o agregador de segurança |

## Como verificar

```bash
# 1. Checks obrigatórios nos dois rulesets
gh api repos/lucascantarelli/wyd/rulesets/27603854 \
  --jq '.rules[] | select(.type=="required_status_checks") | .parameters'
gh api repos/lucascantarelli/wyd/rulesets/27603857 \
  --jq '.rules[] | select(.type=="required_status_checks") | .parameters'

# 2. Política de evento do issue-status
gh api repos/lucascantarelli/wyd/actions/policies \
  --jq '.policies[] | {name, enforcement, conditions, rules}'

# 3. Supply chain
gh api repos/lucascantarelli/wyd/actions/permissions
gh api repos/lucascantarelli/wyd/actions/permissions/selected-actions

# 4. Compose e rollback
docker compose config                 # docs sem build
make -n rollback TAG=<sha>            # pull + up --no-build

# 5. Workflows
actionlint .github/workflows/*.yml
```

O PR de teste deve mostrar `gate-code`, `gate-db`, `gate-docs` e `gate-security` como checks obrigatórios; a imagem publicada em `develop` deve ter `latest`, `develop` e o SHA; o smoke deve responder 200.

**Prova real em 2026-11-26:** PR [#205](https://github.com/lucascantarelli/wyd/pull/205), squash `0c4a612`, com os quatro checks `success`; `docs-image` run [`45827609542`](https://github.com/lucascantarelli/wyd/actions/runs/45827609542) publicou as três tags com SBOM SPDX/provenance SLSA v1 e smoke 200; `make rollback` com `f271caa` e `0c4a612` serviu HTTP 200 sem build local.

## Referências

- Decisão do dono: respostas 1A–11A em 2026-11-26
- Documentos donos: `docs/devops/index.md`, `docs/planos/pipeline-ci-cd.md`, `plano-imagem-e-compose-do-site.md`
- ADRs relacionadas: `ADR-OPS-0003`, `ADR-OPS-0004`, `ADR-OPS-0005`, `ADR-OPS-0008`, `ADR-OPS-0009`, `ADR-OPS-0011`
- GitHub: [required status checks](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets), [pull_request_target](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target), [Docker build cache](https://docs.docker.com/build/cache/backends/gha)

---
name: operar-ci-e-imagem
description: Mexe em workflow, action composta, imagem GHCR ou rollback do WYD sem quebrar gate, permissões ou custo do plano gratuito. Use ao adicionar job, corrigir workflow vermelho, publicar imagem ou executar rollback por tag.
---

# Operar CI e imagem

## Quando usar

- Alterar `.github/workflows/*.yml`, `.github/actions/*` ou `docker-compose*.yml`.
- Publicar/rollback de imagem `ghcr.io/lucascantarelli/wyd-docs`.

## Pré-requisitos

- `planejamento-1.0.0/wyd-ops/docs/devops/index.md` (estado real do CI).
- `ADR-OPS-0010` (camada única de gate) e `plano-imagem-e-compose-do-site.md`.

## Passos

1. **Um workflow por domínio**: `docs`, `code`, `ci-db`, `security`, `docs-image`, `code-stable`, mais
   `release` e `issue-status` (estado e labels). Não crie `full-workflow` de novo — foi removido em
   2026-11-26.
2. **Permissões mínimas** no topo (`permissions:`) e por job; nada de `write-all`.
3. **Action fixada por SHA** com comentário da versão; action nova é decisão, não conveniência.
4. **Gate em um ponto só**: job que precisa verificar usa `make gate-fast`/`make gate`, não comando
   novo inventado no YAML.
5. **Imagem**: `build:` não entra no Compose base; `docker-compose.override.yml` traz `gate`/`db` e
   `docker-compose.dev.yml` é opt-in.
6. **Rollback** se prova contra tag existente: `make rollback TAG=<sha>` servindo HTTP 200, sem build.
7. **Segredo** só em secret do repositório/ambiente; nunca em workflow, log ou documento.

## Como verificar

```bash
make gate-fast                                    # layout, alvos, capacidades, skills, lacunas, contagens, censos, hoje — sem Docker
gh run list --workflow=security.yml --limit 3     # actionlint roda aqui, no job `actionlint`
gh run list --workflow=docs-image.yml --limit 3
curl -sI http://127.0.0.1:8080/ | head -1         # após subir a tag
```

## Quando parar e perguntar

- A mudança exige runner self-hosted, secret novo ou serviço pago.
- O limite do plano gratuito (minutos, armazenamento de imagem) pode ser atingido.
- A alteração muda gate obrigatório de branch protection.

## Armadilhas

- **Action em `@v3`** em vez de SHA.
- **Job que roda código de PR com permissão de escrita.**
- **Publicar `:latest` a partir de branch que não é `develop`.**
- **Deploy/release sem pedido explícito** do dono.

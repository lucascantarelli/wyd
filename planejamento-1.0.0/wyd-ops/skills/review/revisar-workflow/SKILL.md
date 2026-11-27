---
name: revisar-workflow
description: Revisa PR que altera workflow, action composta, Compose ou imagem do WYD contra permissões mínimas, actions fixadas, gate único e custo do plano gratuito. Use antes de mergear mudança de CI ou ao investigar workflow vermelho.
---

# Revisar workflow

## Quando usar

- PR toca `.github/workflows/`, `.github/actions/` ou `docker-compose*.yml`.
- Um workflow falhou e é preciso decidir entre corrigir o job ou o código.

## Pré-requisitos

- `docs/devops/index.md` e `ADR-OPS-0010`.
- Saída do `actionlint` (roda dentro do `gate-security`).

## Passos

1. **Permissão**: cada job declara só o que usa; `contents: read` é o default.
2. **Action**: fixada por SHA, com versão comentada; nenhuma action desconhecida.
3. **Gatilho**: `pull_request` sem `target` perigoso; `workflow_run` com filtro de branch.
4. **Cache**: chave estável e sem segredo; cache que cresce sem limite é defeito.
5. **Gate**: o job chama o comando único; verificação duplicada no YAML é dívida.
6. **Custo**: matriz mínima (SO × crate que precisa), sem job que roda sempre e não decide nada.
7. **Falha**: `notificar falha` continua existindo e não engole erro.

## Como verificar

```bash
gh run list --workflow=security.yml --limit 3     # actionlint roda no CI, job `actionlint`
gh workflow view <arquivo> --yaml | head -40
```

O `make gate-fast` **não** lê YAML de workflow: ele cobre layout, skills, lacunas, contagens e censos.

## Quando parar e perguntar

- O ajuste mexe em branch protection, secret ou permissão de organização.
- A correção depende de runner pago ou self-hosted.

## Armadilhas

- `permissions: write-all` para "resolver rápido".
- `continue-on-error` escondendo gate obrigatório.
- Workflow novo que duplica `gate-docs` ou `gate-code`.

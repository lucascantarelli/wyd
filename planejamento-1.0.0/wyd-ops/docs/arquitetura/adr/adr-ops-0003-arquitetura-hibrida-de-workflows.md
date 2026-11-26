# ADR-OPS-0003 — Arquitetura híbrida de workflows por domínio

> Pergunta: "como o CI se organiza para ter paralelismo sem multiplicar checks e sem acoplar docs a código?"
> Público: Arquitetura, desenvolvimento e operação
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `planejamento-1.0.0/wyd-ops/docs/planos/pipeline-ci-cd.md`; `auditoria-de-organizacao.md` §2–§3
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O `code.yml` concentrava fmt, clippy, testes de três crates, cobertura e três alvos PostgreSQL em um job sequencial; `docs.yml` e `docs-image.yml` já eram separados. Um monolito único faria docs-only bloquear código e um workflow por job produziria 10–20 checks obrigatórios e setup repetido.

## Decisão

Um workflow por domínio (`code.yml`, `ci-db.yml`, `docs.yml`, `docs-image.yml`, `security.yml`, `release.yml`), com jobs paralelos e **um job agregador por domínio** (`gate-code`, `gate-db`, `gate-docs`, `gate-security`) como check obrigatório.

## Consequências

- Positivas: paralelismo real; falha de docs não bloqueia código; a matriz pode crescer sem mudar a regra de branch; cache por domínio.
- Negativas/custos: mais arquivos para revisar; setup repetido dentro do domínio; o agregador precisa tratar `skipped` com cuidado.
- Contratos afetados: a proteção de branch passa a exigir os agregadores, não os jobs internos; `full-workflow.yml` deixa de existir.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Monolito único com `needs` | docs-only executaria (ou bloquearia) o pipeline de código; YAML grande e difícil de revisar |
| Workflow independente por job | 10–20 checks, setup e cache repetidos, proteção de branch ruidosa |
| `workflow_run` como agregador | atraso, permissões e dificuldade de provar o resultado no mesmo run |

## Como verificar

`ls .github/workflows`, a lista de checks do PR e os jobs `gate-*` verdes. A proteção de branch deve citar `gate-code`, `gate-db`, `gate-docs` e `gate-security`.

## Referências

- Decisão do dono: Q2 do plano de DevOps (2026-11-26)
- Documentos donos: `docs/planos/pipeline-ci-cd.md`, `docs/devops/index.md`
- Workflows: `.github/workflows/{code,ci-db,docs,docs-image,security,release}.yml`

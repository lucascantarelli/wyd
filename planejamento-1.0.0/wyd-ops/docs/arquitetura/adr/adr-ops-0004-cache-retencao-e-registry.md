# ADR-OPS-0004 — Cache, retenção e registry no plano gratuito

> Pergunta: "o que cachear, por quanto tempo guardar artefato e onde publicar imagem sem custo?"
> Público: Arquitetura, desenvolvimento e operação
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: documentação oficial de billing/cache do GitHub Actions; `docs/planos/pipeline-ci-cd.md`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

Repositório público tem runners padrão gratuitos, mas cache de 10 GB por repositório (retenção padrão de 7 dias), artefatos de 500 MB compartilhados com Packages e retenção padrão de 90 dias. O CI não tinha cache de Rust e publicava HTML de cobertura por 90 dias.

## Decisão

- Rust: `Swatinem/rust-cache` com chave por crate (poda automática), SHA pinado.
- Cobertura: somente `coverage-summary.txt` e `lcov.info`, `retention-days: 7`; o HTML é gerado sob demanda.
- Python: cache nativo do `setup-python` sobre `wyd-docs/requirements.txt`.
- Imagem: GHCR com `SHA` sempre, `develop` e `latest` em `develop`; Buildx com cache `type=gha`, `provenance` e `sbom`; bases por digest e Dependabot Docker/Docker Compose. Enquanto `main` estiver arquivada, `latest` acompanha `develop`; no release 1.0.0 a política migra para `main`. Imagem pública é gratuita; se virar privada, a cota de Packages se aplica.
- Nenhum cache/artefato guarda segredo.

## Consequências

- Positivas: menos recompilação; cota de artefatos sob controle; rollback por tag SHA.
- Negativas/custos: cache de Rust pode chegar perto de 10 GB; a chave por crate e a poda mitigam. `rust-cache` é action de terceiro e entra na allowlist.
- Contratos afetados: o Compose aceita `WYD_DOCS_TAG=<sha>` e não constrói no caminho base; a publicação da imagem é restrita a push de `develop` até o release.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| `actions/cache` puro para `target/` | oficial, mas sem poda automática e com risco de estourar a cota |
| Publicar HTML de cobertura por 90 dias | desperdício de cota e sem uso no dia a dia |
| Registry externo | custo/credencial extra sem ganho sobre o GHCR público |

## Como verificar

`gh api repos/lucascantarelli/wyd/actions/artifacts` mostra retenção curta; o job de cobertura sobe dois arquivos; `docker compose config` mostra a tag SHA e não mostra `build` em `docs`; o workflow `docs-image` usa Buildx com cache, publica somente em `develop` e build de PR não faz push.

## Referências

- Decisões do dono: Q3 e Q6 do plano de DevOps (2026-11-26)
- Documentação: dependency caching e billing do GitHub Actions
- Workflows: `code.yml`, `docs-image.yml`

# ADR-OPS-0005 — O que containerizar: Compose para docs/gate, runner para Rust

> Pergunta: "o CI de Rust precisa de Dockerfile próprio e o Compose ainda é necessário?"
> Público: Arquitetura, desenvolvimento e operação
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `plano-imagem-e-compose-do-site.md`; `docs/planos/pipeline-ci-cd.md`; Dockerfiles atuais
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O Compose serve o site e o gate local; o CI de Rust roda direto no runner. Criar um "Dockerfile padrão de CI" adicionaria Docker-in-Docker e perderia o toolcache, sem ganho de reprodutibilidade se a toolchain estiver pinada. Ao mesmo tempo, o Compose publicava `0.0.0.0:8000`.

## Decisão

- **Não** criar Dockerfile de CI para Rust. Consistência vem de `rust-toolchain.toml`, lockfiles e actions pinadas.
- Manter o Compose para docs e gate; adicionar perfil `db` com `postgres:17` fixado por digest e `tmpfs`; publicar portas apenas em `127.0.0.1`. O Compose base consome a imagem publicada, sem `build:`; o build da árvore fica só em `docker-compose.dev.yml`.
- Serviços de banco no CI usam **service containers**, não `docker run` manual.
- A imagem do site continua em `wyd-ops/wyd-docs.Dockerfile`; runtime nginx unprivileged fica na F2.

## Consequências

- Positivas: um comando reproduz o gate; banco local descartável; menos exposição de rede; CI sem DinD.
- Negativas/custos: dois caminhos de banco (service container no CI, Compose local) precisam do mesmo contrato de URL/roles; o Compose continua sendo dependência local.
- Contratos afetados: `docker-compose.yml` (loopback), `docker-compose.override.yml` (gate + `db`), testes do `wyd-db` aceitam `WYD_DB_TEST_URL`.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Dockerfile padrão para todo job | DinD, perda de cache do runner, complexidade sem ganho |
| Remover o Compose | perde gate reproduzível e deploy local de um comando |
| `docker run` manual no CI | repete scripts e não usa healthcheck do service container |

## Como verificar

`docker compose config` não contém `0.0.0.0` nem `build` em `docs`; `docker compose --profile db config` mostra `postgres:17@sha256:...`; `grep -R WYD_DB_TEST_URL crates/wyd-db/tests`; jobs `ci-db` usam `services:`.

## Referências

- Decisões do dono: Q5 e Q6 do plano de DevOps (2026-11-26)
- Documentos donos: `plano-imagem-e-compose-do-site.md`, `docs/devops/index.md`

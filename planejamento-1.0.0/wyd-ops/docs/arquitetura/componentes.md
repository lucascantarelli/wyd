> Pergunta: "quais unidades de código o `wyd-ops` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-ops`
> Dono: equipe do wyd-ops
> Base: `wyd-ops/docs/index.md`; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum código próprio existe; o sistema é composto de workflows, configs e scripts. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `ci`

- **Responsabilidade:** pipelines de CI (build, teste, cobertura, gate de docs).
- **API:** workflows em `.github/workflows/`.
- **Invariantes:** boas práticas GitHub Actions; permissões mínimas; pin por SHA de terceiros.
- **Depende de:** todos os sistemas.
- **Testes:** workflow verde em PR.
- **Risco:** alto — é o gate.

## `imagem`

- **Responsabilidade:** imagem Docker do servidor e do site.
- **API:** Dockerfile, compose.
- **Invariantes:** reproduzível; sem segredo na imagem.
- **Depende de:** `wyd-server`, `wyd-docs`.
- **Testes:** build de imagem; smoke test.
- **Risco:** médio.

## `ambiente`

- **Responsabilidade:** ambientes (dev, homologação, produção) e provisionamento.
- **API:** scripts/compose de ambiente.
- **Invariantes:** ambiente descartável por execução em CI; sem estado global.
- **Depende de:** `wyd-db`, `wyd-server`.
- **Testes:** up/down; seed aplicado.
- **Risco:** médio.

## `monitoramento`

- **Responsabilidade:** Prometheus, Loki, métricas, logs de metadados.
- **API:** configuração de coleta.
- **Invariantes:** sem payload/PII em log; fonte externa.
- **Depende de:** `wyd-server`, `wyd-net`.
- **Testes:** métricas presentes; log sem payload.
- **Risco:** médio.

## `segredos`

- **Responsabilidade:** emissão e rotação de segredos (tokens, chaves).
- **API:** secret store do GitHub/ambiente.
- **Invariantes:** nunca em código; rotação documentada.
- **Depende de:** nenhum.
- **Testes:** varredura de segredo no repo.
- **Risco:** alto — segurança.

## `deploy`

- **Responsabilidade:** deploy contínuo (Pages, servidor, launcher).
- **API:** workflows de deploy.
- **Invariantes:** rollback validado; permissões mínimas.
- **Depende de:** `ci`, `imagem`.
- **Testes:** deploy em staging; rollback.
- **Risco:** alto.

## `gate`

- **Responsabilidade:** comando de gate do projeto (`docker compose --profile gate`).
- **API:** `docker compose --profile gate run --rm gate`.
- **Invariantes:** falha de verdade; censos coincidem.
- **Depende de:** todos.
- **Testes:** gate verde.
- **Risco:** alto.

## `dashboard`

- **Responsabilidade:** dashboard do sistema (a criar: `wyd-ops-1.0.0.md`).
- **API:** documento dono.
- **Invariantes:** mede páginas, requisitos, issues, gate.
- **Depende de:** todos.
- **Testes:** gate de contagem.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-ops/ (raiz)
├── ci/                   # workflows
├── imagem/               # Docker
├── ambiente/             # ambientes
├── monitoramento/        # Prometheus/Loki
├── segredos/             # secret store
├── deploy/               # deploy contínuo
├── gate/                 # comando de gate
└── dashboard/            # wyd-ops-1.0.0.md
tests/
├── unitario/            # (n/a)
├── contrato/            # (n/a)
├── integracao/          # smoke de ambiente
├── sistema/             # deploy/rollback
├── e2e/                 # (n/a)
└── propriedade/         # (n/a)
```

*Página de planejamento. Nenhuma unidade foi implementada.*
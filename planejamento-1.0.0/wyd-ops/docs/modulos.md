# Módulos do `wyd-ops` — detalhamento

> Pergunta: "o que cada módulo do `wyd-ops` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento e operação
> Dono: `wyd-ops`
> Base: `docs/arquitetura/componentes.md`; `docs/specs/requisitos.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

## Módulos

### ci
- **Objetivo:** pipelines de CI (build, teste, cobertura, gate de docs).
- **Escopo:** workflows em `.github/workflows/`.
- **Interfaces:** PR/push em `develop`/`main`.
- **Dependências:** todos os sistemas.
- **Testes:** workflow verde em PR; permissões mínimas; pin por SHA.
- **Segurança:** permissões mínimas; segredos só em secret store.
- **Monitoramento:** status do workflow.
- **DoD:** `code.yml`/`docs.yml` verdes; `full-workflow.yml` sai de stub.

### imagem
- **Objetivo:** imagem Docker do servidor e do site.
- **Escopo:** Dockerfile, compose.
- **Interfaces:** `docker compose build/pull`.
- **Dependências:** wyd-server, wyd-docs.
- **Testes:** build reproduzível; smoke test.
- **Segurança:** sem segredo na imagem.
- **Monitoramento:** tamanho/camadas.
- **DoD:** `pull` funciona (hoje não publicado).

### ambiente
- **Objetivo:** ambientes dev/homologação/produção e provisionamento.
- **Escopo:** scripts/compose de ambiente.
- **Interfaces:** `docker compose up/down`.
- **Dependências:** wyd-db, wyd-server.
- **Testes:** up/down; seed aplicado.
- **Segurança:** ambiente descartável por execução.
- **Monitoramento:** healthcheck.
- **DoD:** ambiente efêmero por execução (LBOT-01/02).

### monitoramento
- **Objetivo:** Prometheus, Loki, métricas, logs de metadados.
- **Escopo:** configuração de coleta.
- **Interfaces:** `/metrics`, Loki.
- **Dependências:** wyd-server, wyd-net.
- **Testes:** métricas presentes; log sem payload.
- **Segurança:** sem payload/PII em log.
- **Monitoramento:** dashboards.
- **DoD:** incidente simulado diagnosticado sem ler DSN/SQL.

### segredos
- **Objetivo:** emissão e rotação de segredos.
- **Escopo:** secret store do GitHub/ambiente.
- **Interfaces:** `sites_set_secret`/secret store.
- **Dependências:** nenhum.
- **Testes:** varredura de segredo no repo.
- **Segurança:** nunca em código; rotação documentada.
- **Monitoramento:** auditoria de acesso.
- **DoD:** varredura limpa.

### deploy
- **Objetivo:** deploy contínuo (Pages, servidor, launcher).
- **Escopo:** workflows de deploy.
- **Interfaces:** push em `develop`/`main`.
- **Dependências:** ci, imagem.
- **Testes:** deploy em staging; rollback.
- **Segurança:** permissões mínimas.
- **Monitoramento:** status de deploy.
- **DoD:** rollback validado.

### gate
- **Objetivo:** comando de gate do projeto.
- **Escopo:** `docker compose --profile gate`.
- **Interfaces:** container `gate`.
- **Dependências:** todos.
- **Testes:** gate verde; censos coincidem.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** `censos: medidas e citações coincidem`.

### dashboard
- **Objetivo:** dashboard do sistema (`wyd-ops-1.0.0.md`).
- **Escopo:** documento dono.
- **Interfaces:** leitura humana.
- **Dependências:** todos.
- **Testes:** gate de contagem.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** D-04 fechado.
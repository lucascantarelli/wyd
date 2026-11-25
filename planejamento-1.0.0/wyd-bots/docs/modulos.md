# Módulos do `wyd-bots` — detalhamento

> Pergunta: "o que cada módulo do `wyd-bots` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento e QA
> Dono: `wyd-bots`
> Base: `docs/arquitetura/componentes.md`; `docs/specs/requisitos.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

## Módulos

### ambiente
- **Objetivo:** Compose efêmero, seed.
- **Escopo:** `docker-compose` de teste.
- **Interfaces:** `up/down`.
- **Dependências:** wyd-server, wyd-db.
- **Testes:** ambiente sobe/desce; seed aplicado.
- **Segurança:** sem estado global.
- **Monitoramento:** healthcheck.
- **DoD:** `LBOT-01`/`LBOT-02`.

### qa
- **Objetivo:** bots de cenários de QA.
- **Escopo:** scripts de cenário.
- **Interfaces:** execução de cenário.
- **Dependências:** ambiente.
- **Testes:** cenário passa.
- **Segurança:** n/a.
- **Monitoramento:** resultado.
- **DoD:** cenário verde.

### carga
- **Objetivo:** bots de carga (N conexões).
- **Escopo:** gerador de carga.
- **Interfaces:** métricas de carga.
- **Dependências:** ambiente, wyd-server.
- **Testes:** medição real de `p95` e taxa de erro.
- **Segurança:** n/a.
- **Monitoramento:** `p95`, erros.
- **DoD:** número real registrado (`LBOT-03`).

### convivencia
- **Objetivo:** bot de convivência.
- **Escopo:** cenário de convivência.
- **Interfaces:** execução.
- **Dependências:** carga.
- **Testes:** sem crash.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** convivência passa.

### fechamento
- **Objetivo:** suíte, anti-skip, relatório.
- **Escopo:** CI.
- **Interfaces:** gate.
- **Dependências:** anteriores.
- **Testes:** sem `SKIP`.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** cobertura ≥ 85% onde aplicável.
# Módulos do `wyd-core` — detalhamento

> Pergunta: "o que cada módulo do `wyd-core` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento
> Dono: `wyd-core`
> Base: `docs/arquitetura/componentes.md`; `docs/specs/requisitos.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

## Módulos

### tick
- **Objetivo:** loop de tick determinístico.
- **Escopo:** scheduler, passo de simulação.
- **Interfaces:** `Tick::step(seed)`.
- **Dependências:** nenhum.
- **Testes:** mesmo seed → mesmo resultado.
- **Segurança:** n/a.
- **Monitoramento:** ticks/s, drift.
- **DoD:** teste de regressão de tick verde.

### ecs
- **Objetivo:** entidades/componentes/sistemas mínimos.
- **Escopo:** storage de componentes.
- **Interfaces:** `World`, `Query`.
- **Dependências:** tick.
- **Testes:** add/remove/query.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** cobertura ≥ 85%.

### cena
- **Objetivo:** cena/mapa e posicionamento.
- **Escopo:** grid, colisão.
- **Interfaces:** `Scene::load`.
- **Dependências:** ecs.
- **Testes:** load de cena válida/inválida.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** teste de cena verde.

### combate
- **Objetivo:** cálculo de dano, alcance, durabilidade (modelo).
- **Escopo:** fórmulas.
- **Interfaces:** `Combat::damage`.
- **Dependências:** ecs, cena, wyd-sim.
- **Testes:** fixture capturada; `stat_source` declarado.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** fórmula reproduz fixture; nenhum número inventado.

### ia
- **Objetivo:** IA de mob.
- **Escopo:** estados, transições.
- **Interfaces:** `Ai::tick`.
- **Dependências:** ecs, cena.
- **Testes:** comportamento por estado.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** catálogo fechado.

### capacidades
- **Objetivo:** skills, classes, quests (modelo).
- **Escopo:** catálogo funcional.
- **Interfaces:** `Skill`, `Class`, `Quest`.
- **Dependências:** combate, wyd-db (catálogo).
- **Testes:** cobertura por classe.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** catálogo fechado (sem placeholder).

### baseline
- **Objetivo:** higiene do crate (docs, fmt, clippy).
- **Escopo:** `lib.rs`, `Cargo.toml`.
- **Interfaces:** `cargo doc/fmt/clippy`.
- **Dependências:** nenhum.
- **Testes:** comandos verdes.
- **Segurança:** `#![deny(missing_docs)]`.
- **Monitoramento:** n/a.
- **DoD:** `cargo doc` sem warning.
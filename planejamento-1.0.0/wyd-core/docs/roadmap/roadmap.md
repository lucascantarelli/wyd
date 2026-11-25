> Pergunta: "em que ordem o `wyd-core` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-core`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Fatia bloqueada por lacuna ou catálogo não começa antes de fechar.
3. Toda fatia de combate inclui teste determinístico (mesmo seed, mesmo resultado).
4. Dependência de protocolo é explícita.

## 2. Fatias

### F0 — Fundação do crate

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-CORE-001`, `RF-CORE-002` |
| Depende de | crate existente |
| Entrega | `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline fmt/clippy |
| DoD | `cargo doc` sem warning; fmt/clippy verdes |
| Fora | CI real |

### F1 — Tick e ECS

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-CORE-003`, `RF-CORE-004` |
| Depende de | F0 |
| Entrega | loop de tick determinístico, ECS mínimo, cena |
| DoD | mesmo seed produz mesmo resultado em N execuções; teste de regressão de tick verde |
| Fora | combate, IA |

### F2 — Combate

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-CORE-005` |
| Depende de | F1; offsets de protocolo |
| Entrega | cálculo de dano, alcance, durabilidade (modelo) |
| DoD | fórmulas reproduzem fixture capturada; nenhum número inventado (`stat_source` declarado) |
| Fora | feitiços, pets |

### F3 — IA e capacidades

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-CORE-006`, `RF-CORE-007` |
| Depende de | F1; catálogo funcional |
| Entrega | IA de mob, skills, classes, quests (modelo) |
| DoD | comportamento por classe coberto por teste; catálogo fechado (sem placeholder) |
| Fora | implementação de conteúdo |

### F4 — Suíte e anti-skip

| Campo | Valor |
|---|---|
| Fase | 4 |
| Requisitos | todos os `RF-CORE` |
| Depende de | F0–F3; CI real |
| Entrega | unitário, contrato, sistema, fixtures, anti-skip |
| DoD | alvos de `plano-de-testes.md` §2 passam sem `SKIP`; cobertura ≥ 85% |
| Fora | migrations |

## 3. Ordem e dependências

```text
F0 → F1 → F2
F1 → F3
F4 depende de F0–F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F2 | fórmula não provada | fixture capturada; `stat_source` explícito |
| F3 | escopo infinito | catálogo fechado antes de codar |
| F4 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
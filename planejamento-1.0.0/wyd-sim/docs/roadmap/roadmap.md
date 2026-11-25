> Pergunta: "em que ordem o `wyd-sim` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-sim`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Toda fórmula é determinística: mesmo seed, mesmo resultado.
3. Nenhum número é inventado; `stat_source` declara o que não foi medido.
4. Dependência de captura/catálogo é explícita.

## 2. Fatias

### F0 — Fundação do crate

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-SIM-001`, `RF-SIM-002` |
| Depende de | crate existente |
| Entrega | `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline fmt/clippy |
| DoD | `cargo doc` sem warning; fmt/clippy verdes |
| Fora | CI real |

### F1 — Fórmulas e curvas

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-SIM-003`, `RF-SIM-004` |
| Depende de | F0; captura do cliente 7662 |
| Entrega | fórmulas de dano, curva de XP, regeneração |
| DoD | fórmula reproduz fixture capturada; `stat_source` declarado; teste de regressão verde |
| Fora | simulação completa |

### F2 — Simulações determinísticas

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-SIM-005`, `RF-SIM-006` |
| Depende de | F1; catálogo funcional |
| Entrega | simulação de cenário com seed, relatório |
| DoD | mesmo seed produz mesmo relatório; relatório citável por hash |
| Fora | IA de mob |

### F3 — Suíte e anti-skip

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | todos os `RF-SIM` |
| Depende de | F0–F2; CI real |
| Entrega | unitário, contrato, sistema, fixtures, anti-skip |
| DoD | alvos de `plano-de-testes.md` §2 sem `SKIP`; cobertura ≥ 85% |
| Fora | migrations |

## 3. Ordem e dependências

```text
F0 → F1 → F2 → F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | número inventado | fixture capturada + `stat_source` |
| F2 | não-determinismo | seed fixo em teste |
| F3 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
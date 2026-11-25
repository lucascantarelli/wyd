> Pergunta: "em que ordem o `wyd-launcher` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-launcher`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Toda fatia de patch inclui verificação de integridade do manifesto.
3. Toda fatia administrativa inclui teste de permissão negada.
4. Dependência de `wyd-ops` (upload) é explícita.

## 2. Fatias

### F0 — Fundação Tauri

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-LCH-001`, `RF-LCH-002` |
| Depende de | projeto Tauri existente |
| Entrega | app Tauri, layout, config, baseline fmt/clippy |
| DoD | `cargo tauri build` gera binário; fmt/clippy verdes |
| Fora | patch, serverlist |

### F1 — Manifesto e patch

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-LCH-003`, `RF-LCH-004` |
| Depende de | F0; manifesto íntegro |
| Entrega | leitura de manifesto, download e aplicação de patch |
| DoD | patch corrompido é recusado; patch válido aplica; rollback em falha |
| Fora | serverlist |

### F2 — Serverlist e itemlist

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-LCH-005`, `RF-LCH-006` |
| Depende de | F1 |
| Entrega | leitura de serverlist/itemlist, conexão ao canal |
| DoD | lista vazia mostra estado vazio; canal inválido é recusado |
| Fora | telemetria |

### F3 — Suíte, E2E e anti-skip

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | todos os `RF-LCH` |
| Depende de | F0–F2; CI real |
| Entrega | unitário, E2E (Playwright/Tauri), anti-skip |
| DoD | alvos de `plano-de-testes.md` §2 sem `SKIP`; cobertura ≥ 85% |
| Fora | upload de binário (wyd-ops) |

## 3. Ordem e dependências

```text
F0 → F1 → F2 → F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | manifesto corrompido aplicado | verificação de hash obrigatória |
| F2 | canal errado | validação de URL |
| F3 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
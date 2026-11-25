> Pergunta: "em que ordem o `wyd-server` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-server`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Um processo por canal; sem estado global compartilhado.
3. Toda fatia inclui teste de shutdown limpo.
4. Dependência de captura e contrato admin é explícita.

## 2. Fatias

### F0 — Fundação do crate

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-SRV-001`, `RF-SRV-002` |
| Depende de | crate existente |
| Entrega | `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline fmt/clippy |
| DoD | `cargo doc` sem warning; fmt/clippy verdes |
| Fora | CI real |

### F1 — Composição e canal

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-SRV-003`, `RF-SRV-004` |
| Depende de | F0; captura S7 |
| Entrega | um processo por canal, bootstrap, graceful shutdown |
| DoD | processo sobe/desce sem leak; shutdown drena fila pendente; teste de restart verde |
| Fora | controle HTTP |

### F2 — Controle e integração

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-SRV-005`, `RF-SRV-006` |
| Depende de | F1; contrato admin (`LAC-ADM-03`) |
| Entrega | endpoint HTTP síncrono com ack, integração com net/core/db |
| DoD | ack retorna antes do efeito; ação sem permissão retorna 403; auditoria gravada |
| Fora | observabilidade completa |

### F3 — Observabilidade e suíte

| Campo | Valor |
|---|---|
| Fase | 4 |
| Requisitos | `RF-SRV-007` e todos |
| Depende de | F0–F2; CI real |
| Entrega | spans, métricas, logs de metadados, suíte, anti-skip |
| DoD | incidente simulado diagnosticado; alvos de `plano-de-testes.md` §2 sem `SKIP`; cobertura ≥ 85% |
| Fora | migrations |

## 3. Ordem e dependências

```text
F0 → F1 → F2 → F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | shutdown não drena | teste de restart obrigatório |
| F2 | controle sem ack | contrato `LAC-ADM-03` explícito |
| F3 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
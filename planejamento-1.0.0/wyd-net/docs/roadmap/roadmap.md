> Pergunta: "em que ordem o `wyd-net` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-net`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

Fatias de entrega, cada uma com DoD executável. **Nenhuma issue foi criada**: o roadmap planeja
ordem e prova; abrir issue, milestone ou board é ato separado.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Fatia bloqueada por lacuna (`lacunas.md`) não começa antes de a lacuna fechar.
3. Toda fatia de transporte inclui teste de sessão revogada e de limite excedido.
4. Dependência de captura de protocolo é explícita: sem `LPRT-03`/`LPRT-06`, a fatia não fecha.

## 2. Fatias

### F0 — Fundação do crate (baseline)

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-NET-001` a `RF-NET-002` |
| Depende de | crate existente |
| Entrega | `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline de `cargo fmt`/`clippy` |
| DoD | `cargo doc -p wyd-net --no-deps` sem warning; `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` verdes |
| Fora | CI real, versão mínima de Rust |

### F1 — Sessão e transporte async

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-NET-003`, `RF-NET-004` |
| Depende de | F0; captura `LPRT-03`/`LPRT-06` |
| Entrega | sessão TCP/async, handshake, keep-alive, desconexão limpa |
| DoD | teste de sessão revogada não acessa canal; desconexão em voo não perde pacote pendente; `cargo test` verde |
| Fora | rate-limit, hardening |

### F2 — Rate-limit e hardening

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-NET-005`, `RF-NET-006` |
| Depende de | F1; medição `LNET-01` |
| Entrega | limites por IP/sessão, backpressure, rejeição de pacote malformado |
| DoD | limite excedido retorna erro tipado; pacote malformado é descartado e logado sem payload; teste de carga mede `p95` |
| Fora | observabilidade completa |

### F3 — Observabilidade

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-NET-007`, `RF-NET-008` |
| Depende de | F1 |
| Entrega | spans, métricas de conexão/erro/bytes/latência, logs de metadados |
| DoD | incidente simulado diagnosticado sem ler DSN ou payload; métricas presentes no `/metrics` |
| Fora | dashboard Grafana (wyd-ops) |

### F4 — Suíte, fixtures e anti-skip

| Campo | Valor |
|---|---|
| Fase | 4 |
| Requisitos | todos os `RF-NET` |
| Depende de | F0–F3; CI real |
| Entrega | unitário, contrato, sistema, fixtures centralizadas, anti-skip |
| DoD | alvos de `plano-de-testes.md` §2 passam sem `SKIP`; cobertura ≥ 85% |
| Fora | migrations |

## 3. Ordem e dependências

```text
F0 → F1 → F2
      ↘ F3
F4 depende de F0–F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | captura de protocolo incompleta | bloquear por `LPRT-03`/`LPRT-06`; não inventar offset |
| F2 | limite mal calibrado | medição `LNET-01` antes de fixar número |
| F3 | log com payload | varredura de segredo/payload no CI |
| F4 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
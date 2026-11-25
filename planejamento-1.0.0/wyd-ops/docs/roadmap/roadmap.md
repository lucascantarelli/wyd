> Pergunta: "em que ordem o `wyd-ops` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-ops`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Permissões mínimas em workflows; terceiros pinados por SHA.
3. Nenhum segredo em código, log ou documento.
4. Deploy e rollback validados antes de produção.

## 2. Fatias

### F0 — CI

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-OPS-001` |
| Depende de | repositório existente |
| Entrega | workflow reutilizável, permissões mínimas, pin por SHA |
| DoD | workflow verde em PR; varredura de segredo limpa |
| Fora | deploy |

### F1 — Imagem

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-OPS-002` |
| Depende de | F0 |
| Entrega | Dockerfile, build reproduzível, publicação |
| DoD | build ok; sem segredo na imagem; `pull` funciona |
| Fora | ambiente |

### F2 — Ambiente

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-OPS-003` |
| Depende de | F0; `LBOT-01`/`LBOT-02` |
| Entrega | Compose efêmero, seed, up/down |
| DoD | ambiente sobe/desce; seed aplicado; sem estado global |
| Fora | monitoramento |

### F3 — Monitoramento

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-OPS-004` |
| Depende de | F2 |
| Entrega | Prometheus, Loki, métricas, logs de metadados |
| DoD | métricas presentes; log sem payload/PII |
| Fora | segredos |

### F4 — Segredos e deploy

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-OPS-005`, `RF-OPS-006` |
| Depende de | F0–F3 |
| Entrega | secret store, deploy contínuo, rollback |
| DoD | segredo nunca em código; deploy em staging; rollback validado |
| Fora | gate |

### F5 — Gate e fechamento

| Campo | Valor |
|---|---|
| Fase | 4 |
| Requisitos | `RF-OPS-007` e todos |
| Depende de | F0–F4; CI real |
| Entrega | gate verde, censos coincidem, cobertura |
| DoD | `docker compose --profile gate run --rm gate` verde; cobertura ≥ 85% onde aplicável |
| Fora | migrations |

## 3. Ordem e dependências

```text
F0 → F1 → F2 → F3 → F4 → F5
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | permissão excessiva | revisão de workflow |
| F1 | segredo na imagem | varredura |
| F4 | deploy sem rollback | rollback validado |
| F5 | gate decorativo | censos coincidem |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
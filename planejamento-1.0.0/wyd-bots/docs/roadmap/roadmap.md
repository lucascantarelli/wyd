> Pergunta: "em que ordem o `wyd-bots` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-bots`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Toda fatia de carga inclui medição real (sem número inventado).
3. Ambiente efêmero por execução; sem estado global.
4. Dependência de `wyd-server`/`wyd-net` é explícita.

## 2. Fatias

### F0 — Ambiente e baseline

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-BOT-001`, `RF-BOT-002` |
| Depende de | `LBOT-01`/`LBOT-02` (ambiente efêmero) |
| Entrega | Compose efêmero, seed, baseline de bot |
| DoD | ambiente sobe/desce; seed aplicado; bot conecta e desconecta limpo |
| Fora | carga |

### F1 — Bots de QA e carga

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-BOT-003`, `RF-BOT-004` |
| Depende de | F0; `wyd-server` rodando |
| Entrega | bots de QA (cenários), bots de carga (N conexões) |
| DoD | cenário de QA passa; carga mede `p95` e taxa de erro; número real registrado |
| Fora | convivência |

### F2 — Convivência e medição

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-BOT-005`, `RF-BOT-006` |
| Depende de | F1 |
| Entrega | bot de convivência, medição de `LBOT-03` |
| DoD | convivência passa sem crash; `LBOT-03` fecha com número real |
| Fora | anti-cheat |

### F3 — Suíte e anti-skip

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | todos os `RF-BOT` |
| Depende de | F0–F2; CI real |
| Entrega | suíte, anti-skip, relatório de carga |
| DoD | alvos de `plano-de-testes.md` §2 sem `SKIP`; cobertura ≥ 85% onde aplicável |
| Fora | migrations |

## 3. Ordem e dependências

```text
F0 → F1 → F2 → F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | número inventado | medição real obrigatória |
| F2 | ambiente não efêmero | Compose por execução |
| F3 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
# AUD-20 — Decidir os modelos divergentes do legado (drop, piedade, PK)

> **ID canônico:** `AUD-20`
> **ID histórico:** `AUD-20` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Decisão / economia · **Esforço:** M · **Depende de:** `AUD-10` (laboratório de simulação) · **Bloqueia:** `AUD-15` (PK), balanceamento de loot e refino

**Origem:** `docs/audit/15-bugs-and-divergences.md` `DIV-05`, `DIV-06`, `DIV-13`; `docs/audit/17-priorities-and-roadmap.md` §4 (Fase 2).

## Objetivo

Transformar três divergências de **modelo** (não bugs) em decisão registrada, com simulação
comparada, para que o código pare de carregar uma regra “nossa” sem ADR.

## Itens

| ID | Divergência | Hoje no Rust | Decisão exigida |
|---|---|---|---|
| `DIV-05` | Drop por **slot do `Carry`** (legado) × um roll por item (Rust) | `config_gameplay.rs` usa tabela do snapshot com um roll por item | manter por item (mais simples e testável) **ou** portar por slot; registrar probabilidade efetiva das duas curvas |
| `DIV-06` | Piedade de refino do legado (`g_pSuccessRate`) × `0043_refine_pity` | piedade nossa, documentada em `docs/wyd/sistemas/refino.md` | manter a nossa com número medido **ou** alinhar ao legado; ADR com fonte |
| `DIV-13` | PK/chaos: “less than −65” (EN) × “−75” (PT) | regra não implementada (`AUD-15`) | escolher um número e citar a fonte oficial usada |

## Critérios de aceite

- [ ] Cada item tem ADR (ou nota datada em `docs/wyd/sistemas/`) com a fonte e o número medido.
- [ ] Simulação em `wyd-sim` compara as duas curvas e publica tabela (sem “achismo”).
- [ ] Nenhum documento do repo afirma a regra antiga depois da decisão.

## Testes exigidos

- Golden de fórmula para o modelo escolhido (drop e piedade) com o número da ADR.
- Teste de PK/chaos só quando `AUD-15` entrar; aqui basta o golden da regra escolhida.

## Definição de pronto

- `docs/audit/15` marca os itens como “decidido em …” com ponteiro para a ADR.
- Issue, índice e painel com o mesmo status; `make check` verde.

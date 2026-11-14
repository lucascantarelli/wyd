# AUD-14 — Quest genérica: aceitar, progredir e entregar

> **ID canônico:** `AUD-14`
> **ID histórico:** `AUD-14` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / conteúdo · **Esforço:** L · **Depende de:** `AUD-13`, `GAME-25` · **Bloqueia:** conteúdo de progressão (story/Kefra/Arch)

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §5 (3.6).

## Objetivo

Ter um motor de quest mínimo e genérico, com estado persistido e recompensa idempotente.

## Contexto

- A auditoria marca quests como ausentes; `0013_quests.sql` já existe no schema.
- Aceite exige aceitar/progredir/entregar e não pagar recompensa duas vezes.

## Passos

1. Decompor: modelo de quest, gatilhos, progresso e entrega.
2. Implementar um NPC de quest com os opcodes já catalogados.
3. Chave de idempotência para a recompensa.

## Critérios de aceite

- [ ] Quest completa entrega recompensa uma única vez.
- [ ] Progresso persiste após relogin.

## Testes exigidos

- Integração: fluxo completo e repetição da entrega sem duplicar prêmio.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

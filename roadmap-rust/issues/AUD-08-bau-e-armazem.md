# AUD-08 — Baú/armazém: depositar e retirar com persistência

> **ID canônico:** `AUD-08`
> **ID histórico:** `AUD-08` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / persistência · **Esforço:** M · **Depende de:** `GAME-05` · **Bloqueia:** economia de itens entre personagens

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §4 (2.5).

## Objetivo

Ligar `vault_items` (0–127) à sessão com depósito/retirada transacionais e recusa por cheio.

## Contexto

- A tabela existe e o `CNFAccountLogin` já envia `Cargo[128]`; não há handler de depósito.
- Aceite inclui conferência item a item no boot (anti-dupe), como no inventário.

## Passos

1. Implementar os opcodes de baú do catálogo e validar faixa de slot.
2. Gravar/debitar do `vault_items` e do `character_items` na mesma transação.
3. Recusar quando o baú ou a mochila estiverem cheios.

## Critérios de aceite

- [ ] Item depositado some da mochila e aparece no baú; retirada faz o inverso.
- [ ] Nenhuma operação parcial em falha de transação.

## Testes exigidos

- Integração com Postgres real: depósito, retirada e saldo de itens conferidos.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

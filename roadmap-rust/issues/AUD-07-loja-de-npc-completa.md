# AUD-07 — Loja de NPC completa no fio (comprar e vender)

> **ID canônico:** `AUD-07`
> **ID histórico:** `AUD-07` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / economia · **Esforço:** M · **Depende de:** `GAME-05`, `GAME-04` ✅ · **Bloqueia:** economia jogável

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §4 (2.3).

## Objetivo

Fechar compra e venda com saldo, estoque e recusas tipadas, usando `npc_shop_item` do snapshot e o ouro do personagem.

## Contexto

- O catálogo de loja já carrega no snapshot (`config/service.rs`) e o portal tem `WEB-10`.
- A auditoria marca loja como parcial e sem teste de limite/saldo.

## Passos

1. Implementar `_MSG_ReqBuy`/`_MSG_Buy` (0x0398/0x0379) e `_MSG_Sell` (0x037A), layouts já no
   catálogo, com validação de slot, preço e quantidade.
2. Persistir ouro e item na mesma transação (`replace_items` + carteira).
3. Recusar sem mutação: saldo insuficiente, mochila cheia, item não vendável.

## Critérios de aceite

- [ ] Compra debita o preço e entrega o item; venda credita e remove.
- [ ] Saldo/mochila insuficientes recusam com opcode de falha.

## Testes exigidos

- Sistema loopback com banco real: comprar, vender, saldo final e inventário conferidos.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

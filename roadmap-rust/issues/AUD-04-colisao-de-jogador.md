# AUD-04 — Colisão de jogador: não atravessar parede nem outro corpo

> **ID canônico:** `AUD-04`
> **ID histórico:** `AUD-04` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / autoridade · **Esforço:** M · **Depende de:** `AttributeMap` do seed (`0019`), `GAME-05` · **Bloqueia:** movimento fiel (`QA-13`)

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §3 (1.4).

## Objetivo

Validar no tick que o destino do jogador é andável e não está ocupado, recusando o passo sem mover a posição autoritativa.

## Contexto

- O movimento já é validado no tick (`world::move_to`), mas a auditoria marca colisão de jogador como ausente.
- Aceite exige fuzz de posição: pacote com coordenada forjada não pode atravessar obstáculo.

## Passos

1. Conferir `AttributeMap` e o grid do mundo para obstáculo por tile.
2. Adicionar checagem de ocupação por jogador no passo (não bloquear NPC do mesmo tile quando o legado permite).
3. Cobrir recusa sem mutação de posição, como em `crates/wyd-core/tests/integracao/autoridade.rs`
   (`teleporte_forjado_nao_move_nem_publica`).

## Critérios de aceite

- [ ] Destino em parede ou em tile ocupado por jogador é recusado.
- [ ] Recusa não altera posição, HP nem fila de eventos.

## Testes exigidos

- Integração: 1000 posições forjadas, nenhuma atravessa parede, nenhum panic.
- Sistema: dois clientes no mesmo tile — o segundo é recusado.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

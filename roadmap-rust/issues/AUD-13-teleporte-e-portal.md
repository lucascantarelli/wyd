# AUD-13 — Teleporte e portal entre mapas

> **ID canônico:** `AUD-13`
> **ID histórico:** `AUD-13` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / mundo · **Esforço:** M · **Depende de:** `AUD-04`, `GAME-09` · **Bloqueia:** quests e campo

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §5 (3.5).

## Objetivo

Trocar de mapa com posição autoritativa, reemitindo a cena (NPCs/jogadores) e persistindo `pos_x`/`pos_y`.

## Contexto

- `routes` já carregam no snapshot; falta o fluxo de troca de mapa no fio.
- O portal precisa validar nível/custo e recusar sem mover.

## Passos

1. Implementar o handler de portal/teleporte usando a rota do snapshot.
2. Recriar a cena e enviar `CreateMob`/`RemoveMob` coerentes.
3. Persistir a nova posição e testar recusa por requisito.

## Critérios de aceite

- [ ] Personagem aparece no mapa de destino na coordenada da rota.
- [ ] Requisito não atendido recusa e mantém a posição anterior.

## Testes exigidos

- Sistema: viajar por uma rota do seed e conferir posição e cena.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

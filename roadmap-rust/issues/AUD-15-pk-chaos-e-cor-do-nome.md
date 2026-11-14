# AUD-15 — PK/chaos: penalidade e cor do nome

> **ID canônico:** `AUD-15`
> **ID histórico:** `AUD-15` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / PvP · **Esforço:** L · **Depende de:** `AUD-06`, `GAME-17` · **Bloqueia:** PvP fiel

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §5 (3.8).

## Objetivo

Persistir CP/penalidade de PK e refletir a cor do nome no `STRUCT_MOB` (`ChaosRate`), com regra citada do legado.

## Contexto

- `character_chaos` existe e `chaos_rate_de(cp) = CP + 75` já está documentado/implementado.
- Aceite exige penalidade após morte de jogador e cor correta no fio.

## Passos

1. Definir a tabela de penalidade com fonte citada e marcá-la como CONFIRMADA/UNVERIFIED.
2. Aplicar CP na morte PvP e persistir.
3. Enviar `ChaosRate` no `CreateMob`/`UpdateScore`.

## Critérios de aceite

- [ ] Morte PvP altera CP conforme a tabela e persiste.
- [ ] Cliente recebe `ChaosRate` coerente com o CP (75 neutro).

## Testes exigidos

- Integração: matar jogador e conferir CP e byte no `CreateMob`.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

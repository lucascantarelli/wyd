# AUD-05 — Regen de HP/MP e respawn de NPC

> **ID canônico:** `AUD-05`
> **ID histórico:** `AUD-05` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / tick · **Esforço:** S · **Depende de:** `GAME-01` (tick) · **Bloqueia:** sustentação do loop jogável

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §3 (1.6, 1.7).

## Objetivo

Fazer HP/MP subirem com o tempo e o NPC voltar após a morte, ambos por regra de configuração e sem recarga de processo.

## Contexto

- A auditoria lista regen e respawn como ausentes; o abate hoje remove o NPC da cena.
- A taxa precisa vir do snapshot (`Channel Settings`) e não de constante, como manda o invariante de conteúdo.

## Passos

1. Definir a taxa no snapshot com default reproduzindo o legado e validar na carga (`config/service.rs`).
2. Aplicar regen no tick para jogadores vivos e respawn no sistema de NPC com timer por template.
3. Não conceder EXP/loot no respawn e preservar `ClientID`/faixa de ids.

## Critérios de aceite

- [ ] HP/MP sobem ao longo de N ticks e param no máximo.
- [ ] NPC morto reaparece no mesmo ponto após o intervalo configurado.

## Testes exigidos

- Unitário do tick: regen por faixa de tempo e teto.
- Integração: matar NPC, avançar ticks, NPC volta e `CreateMob` é reemitido.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

# AUD-09 — Skill no fio: dano, custo e cooldown

> **ID canônico:** `AUD-09`
> **ID histórico:** `AUD-09` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / protocolo · **Esforço:** M · **Depende de:** `GAME-05`, `CLIENT-06` (layout) · **Bloqueia:** party e progressão social

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §4 (2.6).

## Objetivo

Levar `systems/skill.rs` ao fio: `_MSG_UseSkill` valida skill aprendida, MP e cooldown, e devolve dano/efeito autoritativo.

## Contexto

- A engine já valida alcance/MP/cooldown em `cast`, mas o dispatch de skill não está no `cpsock`.
- O layout do pacote depende da captura (`CLIENT-06`); até lá, só opcodes já provados.

## Passos

1. Confirmar o invólucro do uso de skill (`_MSG_Action` 0x036C ou opcode próprio) na captura e
   ligá-lo ao dispatch, com recusa tipada para skill não aprendida.
2. Aplicar efeito no alvo via engine e emitir o evento de domínio.
3. Cobrir recusa sem consumir MP nem cooldown.

## Critérios de aceite

- [ ] Skill aceita aplica dano/cura e cobra MP.
- [ ] Skill não aprendida, sem MP ou em cooldown recusa sem mutação.

## Testes exigidos

- Sistema: usar skill no loopback e conferir HP/MP do alvo e do lançador.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

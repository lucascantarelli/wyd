# AUD-12 — Distribuição de pontos (`ApplyBonus`) no fio

> **ID canônico:** `AUD-12`
> **ID histórico:** `AUD-12` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / progressão · **Esforço:** S · **Depende de:** `GAME-05` · **Bloqueia:** progressão de atributos e skills

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §5 (3.4).

## Objetivo

Ligar `_MSG_ApplyBonus` (atributo, maestria e skill) à regra já implementada em `character/crud.rs`, com persistência e recusa sem mutação.

## Contexto

- A regra (`apply_bonus`, `special_points`, `learn_skill`) já existe com testes de unidade; falta o dispatch no fio.
- O dispatch já responde no fio, mas o padrão de `Stage::InGame` precisa aceitar `entity = None`
  (processo sem mundo, harness de teste): exigir entidade viva transformava o pedido em
  `reason=not_in_game` e deixava o teste esperando frame. Coberto por
  `sistema_selchar::{bonus_de_atributo_e_skill_chegam_ao_cliente_e_ao_banco,
  maestria_do_bonus_tipo_1_vai_ao_banco_e_volta_no_relogin}` quando há `WYD_TEST_DSN`.
- Aceite exige ponto aplicado e persistido, com teto de nível/atributo respeitado.

## Passos

1. Adicionar o opcode ao dispatch de seleção/action conforme o catálogo.
2. Responder `UpdateScore`/falha e gravar em `character_scores`/`character_learned_skill`.
3. Cobrir as recusas já tipadas na regra.

## Critérios de aceite

- [ ] Ponto aplicado altera atributo e persiste após relogin.
- [ ] Ponto sem saldo ou acima do teto recusa sem alterar nada.

## Testes exigidos

- Sistema loopback: aplicar ponto, relogar e conferir o valor do banco.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

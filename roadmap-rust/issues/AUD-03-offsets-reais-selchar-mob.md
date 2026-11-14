# AUD-03 — Offsets reais de `STRUCT_SELCHAR` e `STRUCT_MOB` no fio

> **ID canônico:** `AUD-03`
> **ID histórico:** `AUD-03` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Protocolo / paridade · **Esforço:** M · **Depende de:** `CLIENT-06` (captura), `AUD-01` · **Bloqueia:** `GAME-17`, render do seletor e da entrada

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §3 (1.2, 1.3) e `16-test-strategy.md` §5.

## Objetivo

Fechar os offsets de `SELCHAR` (840 B) e `STRUCT_MOB` (816 B) contra frame real do 7662, substituindo qualquer layout provisório por golden com hash da captura.

## Contexto

- O código já fala o layout do `Basedef.h` conferido com o porte Go, mas o oráculo final é o frame do cliente.
- A issue de RE (`CLIENT-06`) produz a captura; esta issue transforma a captura em teste de contrato e corrige o que divergir.
- A prova visual de `GAME-17` depende disso.

## Passos

1. Capturar `CNFAccountLogin`, `CNFCharacterLogin` e `CNFNewCharacter` do 7662 com nome/build/hash.
2. Comparar byte a byte com `character/snapshot.rs` (`Selchar`, `character_login_body`) e listar divergências.
3. Corrigir apenas com evidência e congelar fixture em `crates/wyd-server/tests/golden/`.
4. Remover o rótulo “provisório” do checklist quando o golden passar.

## Critérios de aceite

- [ ] Golden de bytes para os três corpos com fixture de captura versionada.
- [ ] Nenhum offset UNVERIFIED usado em produção sem marcação.

## Testes exigidos

- `cargo test -p wyd-server --test golden_*` com os fixtures novos.
- Teste negativo: frame com 1 byte alterado falha com erro tipado, sem panic.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

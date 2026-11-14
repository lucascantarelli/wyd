# AUD-06 — Cadência de ataque provada (≥ 800 ms entre golpes)

> **ID canônico:** `AUD-06`
> **ID histórico:** `AUD-06` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / teste de tempo · **Esforço:** S · **Depende de:** `GAME-19`, `GAME-20` · **Bloqueia:** PK/chaos (`AUD-16`)

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §3 (1.8) e `16-test-strategy.md` §9.3.

## Objetivo

Provar que o servidor recusa golpe dentro do cooldown e que o cliente não ganha hits duplicados, com relógio controlado (sem `sleep` real).

## Contexto

- `next_attack_ms` existe no componente e `GAME-20` investiga a anomalia de ~200% de velocidade.
- A auditoria considera este teste mais valioso que dezenas de testes de unidade novos.

## Passos

1. Criar teste de integração com relógio injetado: dois ataques em 100 ms, o segundo recusado.
2. Provar que a recusa não consome cooldown nem aplica dano.
3. Medir o intervalo efetivo em uma sequência longa e comparar com o esperado.

## Critérios de aceite

- [ ] Nenhum ataque aceito abaixo do intervalo configurado.
- [ ] Sequência de 100 ataques gera exatamente 100 golpes, sem duplicidade.

## Testes exigidos

- `tests/integracao/` com clock duplo, sem `sleep` de sincronização.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

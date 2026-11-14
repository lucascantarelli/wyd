# AUD-17 — Conversor de malha/arma e editor de mapa (decompor antes)

> **ID canônico:** `AUD-17`
> **ID histórico:** `AUD-17` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Customização / expansão · **Esforço:** XL · **Depende de:** `AUD-01`, `AUD-16` · **Bloqueia:** conteúdo novo (campo jogável)

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §6 (4.3, 4.4).

## Objetivo

Preparar a expansão de conteúdo: pipeline de malha/arma e editor de mapa, ambos com licença clara e sem bloquear a paridade.

## Contexto

- Esforço XL: a regra do índice proíbe entrar na fila antes de decomposição em subtasks.
- Depende de decisão de licença e do pipeline `.wyt`.

## Passos

1. Decompor em: (a) especificação de formato, (b) conversor mínimo, (c) validador, (d) editor.
2. Só abrir subtasks quando P1–P3 estiverem fechadas.

## Critérios de aceite

- [ ] Cada subtask com fixture e aceite próprio; nenhuma antes de P1.

## Testes exigidos

- Round-trip de uma malha existente antes de qualquer criação nova.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

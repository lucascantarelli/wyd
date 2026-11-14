# AUD-11 — Party e EXP compartilhada

> **ID canônico:** `AUD-11`
> **ID histórico:** `AUD-11` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Social / progressão · **Esforço:** L · **Depende de:** `GAME-25` (chat) · **Bloqueia:** guilda e eventos sociais

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §5 (3.2).

## Objetivo

Criar grupo, convidar/aceitar e dividir EXP conforme a regra do legado, com persistência de membros.

## Contexto

- Não existe party no schema nem no fio; a regra de divisão de EXP precisa de fonte citada.
- Esforço L: decompor antes de entrar na fila.

## Passos

1. Modelar party (tabela + limites) e os opcodes do catálogo.
2. Implementar divisão de EXP no abate com teste de 2–8 membros.
3. Registrar a regra em `docs/wyd/` e marcar o que for UNVERIFIED.

## Critérios de aceite

- [ ] Convidar/aceitar/expulsar refletem no roster.
- [ ] EXP total do grupo é dividida conforme a regra documentada.

## Testes exigidos

- Integração: 2 e 4 membros, EXP somada e comparada ao total do abate.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

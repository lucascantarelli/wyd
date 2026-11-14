# AUD-10 — Mix/compose no fio e cadeia +12…+15/Anct

> **ID canônico:** `AUD-10`
> **ID histórico:** `AUD-10` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Gameplay / economia · **Esforço:** L · **Depende de:** `GAME-11`, `GAME-19` · **Bloqueia:** economia de refino de alto nível

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §4 (2.7, 2.8).

## Objetivo

Aplicar receitas de composição e as taxas do legado na cadeia +12…+15/Anct, com consumo e geração transacionais.

## Contexto

- As fórmulas e o seed de refino existem (`wyd-sim`, `0022_seed_refino.sql`), mas o fluxo de composição não está no fio.
- Esforço L: decompor em subtasks antes de entrar na fila (regra do índice).

## Passos

1. Decompor: (a) leitura de receita, (b) consumo de materiais, (c) sorteio e aplicação, (d) persistência.
2. Ligar ao `_MSG_UseItem` já existente quando o layout permitir.
3. Golden das taxas por faixa contra a tabela do snapshot.

## Critérios de aceite

- [ ] Receita consome exatamente os materiais e gera o resultado.
- [ ] Falha não duplica nem destrói além da regra declarada.

## Testes exigidos

- Golden de fórmulas por faixa + sistema com banco real para consumo/persistência.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

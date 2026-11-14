# PROC-25 — Revalidação: retirar “concluído/testado” sem prova e rebaixar o roadmap

> **ID canônico:** `PROC-25`
> **ID histórico:** `PROC-25`
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Processo / auditoria · **Esforço:** M · **Depende de:** — · **Bloqueia:** confiança no painel e no changelog

## Objetivo

Nenhuma linha do roadmap, painel ou changelog pode afirmar “concluído” ou “testado” sem
evidência reproduzível **na versão atual**. Esta issue conduz o rebaixamento: desmarca
checkbox de aceite, reabre o que foi fechado por teste de fio e registra o que falta.

## Contexto

- Em 2026-11-13, issues fechadas com teste de fio (`GAME-08`, `GAME-09`, `GAME-10`, `GAME-19`,
  `BUG-02`) falharam no cliente real; foram reabertas, mas o padrão continua.
- O prompt master (§2.2, §2.3 e §18) exige que “portado” só exista com evidência comportamental.
- `docs/audit/20-validacao-2026-11-13.md` registra a validação, mas o painel ainda tem ✅/🟡
  herdados de execuções antigas.

## Passos

1. Varrer `roadmap-rust/issues/*.md` e desmarcar todo `- [ ]` sem evidência no commit atual.
2. Varrer `roadmap-rust/index.md` e `docs/rust/03-progresso.md`: status ✅ só com prova
   (teste + captura/print quando o critério pedir tela).
3. Registrar no topo de `docs/audit/20-validacao-2026-11-13.md` que os vereditos foram
   **retirados** até reexecução.
4. Criar checklist de revalidação por issue rebaixada (o que rodar e onde anexar).
5. Rodar o gate `issue-status-check` e deixar índice/painel consistentes.

## Critérios de aceite

- [ ] Nenhum `- [ ]` sem evidência no mesmo commit.
- [ ] Índice e painel sem ✅ sem prova.
- [ ] Aviso de revalidação no documento de validação.
- [ ] Lista de issues rebaixadas e o teste que falta para cada uma.

## Skills relacionadas

- `rust-test-quality`, `documentation-writer`.

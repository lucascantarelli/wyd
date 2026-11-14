# AUD-22 — Decisão de cliente independente (item 4.5 do audit)

> **ID canônico:** `AUD-22`
> **ID histórico:** `AUD-22` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⛔ · ver `roadmap-rust/index.md`.

**Tipo:** Decisão de produto · **Esforço:** S (decisão) · **Depende de:** `AUD-17`, paridade P1–P3 · **Bloqueia:** qualquer trabalho de cliente próprio

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §6 (4.5) e §7 (“não misturar paridade com customização”).

## Objetivo

Registrar a decisão go/no-go sobre escrever um cliente independente, hoje listada no audit como
“decisão após paridade” e **sem issue**. Enquanto a decisão não existir, nenhum trabalho de cliente
próprio pode começar.

## Contexto

- O audit é explícito: 4.5 é XL e só entra depois de P1–P3; a auditoria não abriu issue porque a
  resposta depende de produto.
- O repositório não pode ter uma linha de roadmap sem dono; esta issue é o dono da decisão.

## Passos

1. Medir o que a paridade já entrega (checklist + `QA-12`/`QA-13`) e o que só um cliente próprio resolve.
2. Comparar custo: manter o 7662 (patch/retheme, `AUD-16`) × cliente novo (formato, rede, UI, assets).
3. Publicar ADR “cliente independente: sim/não/adiado até <marco>” com critério de reavaliação.

## Critérios de aceite

- [ ] ADR publicada em `docs/arquitetura/adr/` com a decisão e o custo estimado.
- [ ] `docs/audit/17` §6 aponta para a ADR na linha 4.5.
- [ ] Se “sim”, o trabalho é decomposto em issues próprias antes de qualquer código.

## Testes exigidos

- Nenhum: decisão documental. A prova é a ADR e o ponteiro no audit.

## Definição de pronto

- Issue fechada com ADR registrada no changelog; índice e painel com o mesmo status.

# QA-16 — Testes comparativos OpenWYD × Rust (paridade comportamental)

> **ID canônico:** `QA-16`
> **ID histórico:** `QA-16`
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Teste de paridade · **Esforço:** L · **Depende de:** `CLIENT-06` para bytes do 7662 · **Bloqueia:** declarar “portado” sem equivalência

## Objetivo

Provar equivalência de comportamento entre o legado OpenWYD e o Rust com testes comparativos,
não com leitura de código: mesma entrada → mesma decisão/saída (fórmula, recusa, pacote), com
goldens versionados e diff explicável.

## Contexto

- O prompt master (§13.1 e §4) exige matriz de paridade com evidência; hoje a matriz
  (`docs/audit/05-feature-parity-matrix.md`) é documental.
- A auditoria de 2026-11-13 mostrou que teste de fio escrito pelo próprio Rust não pega
  divergência de regra (ex.: `GAME-08`, `GAME-19`).
- `docs/audit/02-legacy-analysis.md` mapeia as rotinas do legado que podem virar oráculo.

## Passos

1. Escolher 10 rotinas de maior risco (dano, drop, EXP, refino, buy/sell, decomposição, quest).
2. Gerar vetores do legado (tabela/constante/fórmula documentada) e gravá-los em
   `crates/wyd-sim/tests/golden/` ou `crates/wyd-core/tests/golden/`.
3. Escrever testes que comparam a implementação Rust com os vetores, sem reimplementar a
   fórmula no teste.
4. Para o que depende de pacote, usar captura/replay (`tools/cliente-harness`) e bytes golden.
5. Registrar cada divergência encontrada como issue própria e não “ajustar até passar”.

## Critérios de aceite

- [ ] 10 rotinas com vetor de origem rastreável (arquivo/linha do legado ou doc).
- [ ] Testes comparativos verdes, com mensagem de divergência campo a campo.
- [ ] Divergências reais viram issue (não são mascaradas no teste).
- [ ] Matriz de paridade atualizada com o ID do teste que prova cada linha.

## Skills relacionadas

- `rust-test-quality`, `software-architecture`, `wyd-economy`.

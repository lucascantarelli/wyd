# QA-15 — Cobertura mínima com gate: 95% unitário, 80% integração e 80% global

> **ID canônico:** `QA-15`
> **ID histórico:** `QA-15`
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Qualidade / teste · **Esforço:** L · **Depende de:** — · **Bloqueia:** confiança no “verde” de todas as issues

## Objetivo

Transformar cobertura em número medido e cobrado: unitário ≥ 95%, integração ≥ 80% e global
≥ 80% (buscando o máximo), com relatório por crate e gate no `make check`/CI. Hoje
`make coverage` mede (`cargo llvm-cov` + `vitest --coverage`) mas **não é gate**, e não existe
piso documentado por camada.

## Contexto

- `make coverage` gera `coverage/rust-lcov.info` e a cobertura do portal, mas o target está fora
  do `check` e nenhum número é comparado com piso.
- O prompt master (§13 e §16-FASE 7) exige testes de paridade, protocolo, funcionais e regressão,
  com evidência — cobertura sem piso não prova nada.
- Cobertura alta não substitui teste de contrato: o gate precisa combinar piso **e** os testes
  de fio/golden já existentes.

## Passos

1. Rodar `make coverage` e publicar o baseline por crate em `docs/rust/evidencias/`.
2. Definir o piso por camada: `wyd-core`/`wyd-sim`/`wyd-protocol` 95% de linha; integração
   (socket/DB) 80%; global 80%.
3. Criar `tools/dev/check-coverage.mjs` lendo o LCOV (sem depender de serviço externo) e
   falhando abaixo do piso por crate.
4. Ligar o gate ao `make check` (com opção de rodar localmente sem instrumentar tudo) e ao CI.
5. Fechar as lacunas de teste que o baseline apontar, por ordem de risco (não por número).

## Critérios de aceite

- [ ] Baseline medido e versionado em `docs/rust/evidencias/`.
- [ ] Piso por crate documentado e cobrado por script (exit 1 abaixo do piso).
- [ ] `make check` roda a verificação de cobertura ou delega ao CI com o mesmo piso.
- [ ] Unitário ≥ 95%, integração ≥ 80%, global ≥ 80% medidos no relatório final.
- [ ] Nenhum teste removido/afrouxado para atingir número.

## Skills relacionadas

- `rust-test-quality`, `rust-code-quality`, `wyd-rust-toolchain`.

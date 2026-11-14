# GAME-24 — Eventos e guildas: paridade dos sistemas sociais do prompt §5.11–5.12

> **ID canônico:** `GAME-24`
> **ID histórico:** `GAME-24`
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Feature / paridade · **Esforço:** L · **Depende de:** `CLIENT-06` (layouts não portados) · **Bloqueia:** onda social

## Objetivo

Inventariar e portar o que existe no legado para **eventos** (agendados, GM, spawn especial) e
**guildas** (criação, cargos, guerra, cidade), com regra no servidor e packets só quando o layout
estiver provado.

## Contexto

- O prompt master §5.11 e §5.12 exigem auditoria desses sistemas; o roadmap não tem issue
  dedicada — `GAME-10` cobre cidadania e `GAME-05` autoridade, não guildas/eventos.
- `docs/audit/02-legacy-analysis.md` e `docs/audit/systems/` têm o inventário inicial; o que
  falta é a decisão de escopo e os testes.
- Packet de guilda/guerra não está portado (`docs/arquitetura/protocolo.md` §6): sem captura,
  fica UNVERIFIED e o servidor só mantém o estado/regra.

## Passos

1. Fechar o inventário do legado (tabelas, comandos GM, timers) e classificar Confirmado/UNVERIFIED.
2. Modelar estado mínimo no banco (guilda, membro/cargo; evento agendado) com migrations.
3. Implementar regras no servidor sem inventar packet; expor no portal apenas leitura/administração.
4. Portar os packets quando houver captura do 7662, com golden.
5. Cobrir com testes de regra + integração (duas contas, cargos, guerra simulada).

## Critérios de aceite

- [ ] Inventário do legado fechado, com classificação e fonte.
- [ ] Estado de guilda/evento persistido e autoritativo no servidor.
- [ ] Nenhum layout de packet inventado; cada um com captura ou marcado UNVERIFIED.
- [ ] Testes de regra e de integração verdes; prova no cliente quando o layout cair.

## Skills relacionadas

- `software-architecture`, `rust-test-quality`, `wyd-economy`.

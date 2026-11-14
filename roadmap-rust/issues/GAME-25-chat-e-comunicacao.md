# GAME-25 — Chat e comunicação: canais, sussurro, comando e moderação

> **ID canônico:** `GAME-25`
> **ID histórico:** `GAME-25`
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Feature / paridade · **Esforço:** M · **Depende de:** `CLIENT-06` para o layout de `MessageChat` no cliente · **Bloqueia:** operação social e denúncia

## Objetivo

Fechar a paridade de comunicação do prompt §5.9: chat público, sussurro, grupo/guilda, comandos
(`/`) e moderação (mute, filtro, log), com o mesmo recorte de 140 B já documentado para
`MessagePanel`/`MessageChat`.

## Contexto

- O canal já tem `ChatHub` e entrega `_MSG_MessageChat` (140 B, texto em +12), mas não há issue
  de paridade: falta confirmar canais, comandos e regra de moderação do legado.
- `docs/audit/07-network-flows.md` mapeia o fluxo; `docs/arquitetura/protocolo.md` traz o layout
  de 140 B.
- Sem captura do 7662, o texto do cliente para canais diferentes continua UNVERIFIED.

## Passos

1. Inventariar canais, comandos e regras do legado (quem ouve, alcance, mute, flood).
2. Implementar no servidor: roteamento por canal, comandos, limite de taxa e log de moderação.
3. Portar `MessageChat` para os canais com layout provado; não inventar campo.
4. Cobrir com testes de regra (flood, mute, alcance) e integração com duas sessões.
5. Registar no portal o histórico de moderação lendo do banco, sem escrever estado vivo.

## Critérios de aceite

- [ ] Canais/comandos inventariados com fonte e classificação.
- [ ] Roteamento e rate-limit no servidor, com log sem conteúdo sensível.
- [ ] Testes de regra e de integração (duas sessões) verdes.
- [ ] Moderação auditada e visível no portal (somente leitura).

## Skills relacionadas

- `rust-code-quality`, `wyd-web-admin-ui`, `wyd-rust-obs`.

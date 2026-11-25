# ADR-PRT-0001 — Build-alvo é o cliente 7662, identificado por versão e SHA-256

> Pergunta: "contra qual build do cliente o layout do fio é provado, e o que acontece com os outros?"
> Público: Arquitetura, desenvolvimento do protocolo e QA de fio
> Dono: equipe do `wyd-protocol` (decisão do dono em 2026-11-23)
> Base: `docs/proposta-decisoes-protocol.md` §4 (`Q-PRT-1`); `lacunas.md` `LPRT-09`; `wyd-client/registro-do-cliente.md` §4; `goldens/como-capturar.md` §5
> Revisado em: 2026-11-23

**Estado:** aceito (2026-11-23) — aprovado pelo dono em `Q-PRT-1` = A.

## Contexto

Sem build fixo, offset e opcode ficam ambíguos e "o build mais recente" não tem prova de layout.

**Medido em 2026-11-23 (`V`):** o `WYD.exe` do repositório herdado tem **2.347.008 bytes** e SHA-256
completo registrado em `wyd-client/registro-do-cliente.md` §4; esse era o único build com medição no
registro. A ficha de toda captura identifica a build usada (`goldens/como-capturar.md` §5), e `LNET-02`
declarava depender deste identificador.

## Decisão

O build-alvo do `wyd-protocol` é o **cliente 7662**, identificado pela **versão e pelo SHA-256 completo**
já medidos; **qualquer outro build é não suportado**. Offset, opcode e golden passam a ter alvo, o
`client_version` de `game.session` ganha referência e a ficha de captura passa a nomear o alvo.

## Consequências

- **Positivas:** o fio tem alvo declarado; `LNET-02` fica decidível; a captura não pode ser declarada
  conclusiva contra build que a ficha não nomeia.
- **Negativas/custos:** o suporte fica preso a um binário externo; comparar com build anterior é leitura
  datada marcada `H`, não decisão pendente.
- **Contratos afetados:** `wyd-client` (build distribuído), `wyd-net` (`LNET-02`), `wyd-server`
  (`client_version` da sessão) e `wyd-bots` (ficha do cenário).

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Usar o build mais recente encontrado | sem captura, o layout não está provado e o alvo muda sem registro |
| Não fixar o alvo até a captura | mantém a pergunta aberta e o fio sem alvo; a captura não teria contra o que comparar |

## Como verificar

A decisão está em vigor quando cada ficha de golden nomear o build-alvo por versão e SHA-256 e o codec
recusar layout de outro build. **Hoje nenhum byte foi capturado:** `LPRT-03` (origem da prova) e
`LPRT-06` (capturas B/C) continuam bloqueadas como prova, e a decisão não as fecha.

## Referências

- Decisão: `Q-PRT-1` = A (2026-11-23), em `docs/proposta-decisoes-protocol.md` §4.
- Lacuna fechada: `LPRT-09`.
- Especificação: `wyd-protocol-1.0.0.md` §3 e §8; `goldens/como-capturar.md` §5.
- Medição do binário: `wyd-client/registro-do-cliente.md` §4.

# AUD-01 — Dissector CPSock no Wireshark e decisão sobre Frida/XTrap

> **ID canônico:** `AUD-01`
> **ID histórico:** `AUD-01` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Conhecimento / ferramenta · **Esforço:** M · **Depende de:** `CLIENT-06` · **Bloqueia:** captura legível de S→C para `AUD-03`/`CLIENT-10`

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §2 (0.2, 0.3).

## Objetivo

Transformar a captura do 7662 em leitura de pacote na GUI (dissector CPSock) e registrar, com prova, se Frida/XTrap carrega no cliente — sem instrumentar anti-cheat além do autorizado.

## Contexto

- Hoje a captura é decodificada por CLI próprio (`decode-captura`), não por ferramenta padrão; cada dúvida de layout exige código novo.
- O dissector depende do enquadramento já provado (HEADER 12 B LE, ofuscação a partir do offset 4, `pKeyWord[512]`), documentado em `docs/arquitetura/protocolo.md`.
- A decisão sobre Frida/XTrap é pré-requisito para qualquer RE dinâmica mais profunda; o auditor marca como S/M e risco médio.

## Passos

1. Escrever `tools/protocol/cpsock.lua` com os opcodes já portados e o campo `Size` validado em [12, 8192].
2. Rodar a captura existente pelo `tshark` com o dissector e comparar com `decode-captura` (mesmos campos, mesmos bytes).
3. Registrar em `docs/cliente/` o relatório “Frida carrega/não carrega” com hash do binário e método (sem burlar proteção).
4. Anexar uma captura de referência com nome, build e SHA-256 ao lado do dissector.

## Critérios de aceite

- [ ] `tshark -X lua_script:cpsock.lua` abre um `.pcap` da stack e mostra opcode, tamanho e campos nomeados.
- [ ] O dissector concorda com `decode-captura` no frame de `CNFCharacterLogin` capturado em 2026-11-14.
- [ ] Relatório de viabilidade de Frida/XTrap versionado, com veredito binário e evidência.

## Testes exigidos

- `tools/protocol` ganha teste de tabela do dissector contra fixtures de bytes (opcode + campo).
- Smoke no CI: `tshark` só quando disponível; sem ele, o teste de tabela continua valendo.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

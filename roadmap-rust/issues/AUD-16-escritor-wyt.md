# AUD-16 — Escritor `.wyt` (tema de UI sem patch)

> **ID canônico:** `AUD-16`
> **ID histórico:** `AUD-16` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Customização / ferramentas · **Esforço:** S · **Depende de:** — · **Bloqueia:** tema novo sem patchar o cliente

**Origem:** `docs/audit/17-priorities-and-roadmap.md` §6 (4.1).

## Objetivo

Gerar `.wyt` válido a partir de uma definição versionada, com validação de esquema e round-trip contra o formato documentado.

## Contexto

- O formato está em `docs/cliente/formatos.md`; não existe escritor, só leitura/inspeção.
- Fase 4: não entra na fila antes de P1 fechar.

## Passos

1. Modelar o arquivo e escrever o gerador com teste de round-trip.
2. Comparar bytes com um `.wyt` real do cliente.

## Critérios de aceite

- [ ] Round-trip byte a byte em fixture real.
- [ ] Arquivo gerado abre no cliente sem erro de formato.

## Testes exigidos

- Golden de round-trip e verificação de tamanho/campos.

## Definição de pronto

- Teste que falha antes e passa depois, sem skip silencioso.
- Docs da área atualizados na mesma entrega (issue, índice e painel com o mesmo status).
- `make check` verde; quando a prova exigir stack, o log/print entra em `docs/rust/evidencias/`.

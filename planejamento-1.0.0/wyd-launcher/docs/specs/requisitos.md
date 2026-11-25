> Pergunta: "quais requisitos o `wyd-launcher` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-launcher-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; `LLCH-01`–`LLCH-04` fechadas, implementação pendente.

Requisito sem critério de aceitação é intenção. Numeração `RF-LCH-nnn` (funcional) e `RNF-LCH-nnn` (não funcional).

## Requisitos funcionais

### RF-LCH-001 — Ler e validar manifesto
- **Entrada:** URL do manifesto.
- **Saída:** manifesto validado.
- **Aceitação:** JSON canônico; hash por arquivo; divergência reprova.
- **Contrato:** `LLCH-01`, L1.

### RF-LCH-002 — Verificar assinatura Ed25519
- **Entrada:** manifesto + assinatura.
- **Saída:** válido/inválido.
- **Aceitação:** verificação antes de aplicar; chave errada reprova.
- **Contrato:** `LLCH-03`, L5.

### RF-LCH-003 — Aplicar patch
- **Entrada:** manifesto válido.
- **Saída:** arquivos atualizados.
- **Aceitação:** hash conferido; falha de hash aborta.
- **Contrato:** `LLCH-01`, L4.

### RF-LCH-004 — Conferir integridade local
- **Entrada:** arquivos locais.
- **Saída:** lista de divergências.
- **Aceitação:** arquivo divergente não inicia.
- **Contrato:** L4.

### RF-LCH-005 — Obter lista de canais
- **Entrada:** portal/publicação.
- **Saída:** lista de canais.
- **Aceitação:** canal fora do ar oculto; cache local.
- **Contrato:** `LLCH-02`, L2.

### RF-LCH-006 — Buscar notícias
- **Entrada:** portal.
- **Saída:** notícias.
- **Aceitação:** nenhum conteúdo embutido no binário.
- **Contrato:** L3.

### RF-LCH-007 — Atualizar o próprio launcher
- **Entrada:** GitHub Releases.
- **Saída:** update aplicado.
- **Aceitação:** NSIS + updater Ed25519.
- **Contrato:** `LLCH-04`, L6.

### RF-LCH-008 — Iniciar o cliente
- **Entrada:** integridade ok.
- **Saída:** cliente iniciado.
- **Aceitação:** só após integridade ok; passa canal.
- **Contrato:** L4.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-LCH-001 | Sem segredo no binário | varredura de binário |
| RNF-LCH-002 | Manifesto verificável | assinatura Ed25519 |
| RNF-LCH-003 | Patch atômico | falha de hash aborta |
| RNF-LCH-004 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-LCH-005 | Sem PII em log | varredura de log |

*Página de planejamento. Nenhum requisito foi implementado.*
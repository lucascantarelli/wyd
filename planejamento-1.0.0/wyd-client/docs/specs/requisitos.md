> Pergunta: "quais requisitos o `wyd-client` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-client-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; `LCLI-01` e `LCLI-02` fechados, `LCLI-03`/`LCLI-05` abertas.

Requisito sem critério de aceitação é intenção. Numeração `RF-CLI-nnn` (funcional) e `RNF-CLI-nnn` (não funcional).

## Requisitos funcionais

### RF-CLI-001 — Medir identidade do build-alvo
- **Entrada:** binário do cliente.
- **Saída:** versão + hash.
- **Aceitação:** 7662 + SHA-256 medido.
- **Contrato:** `LCLI-01`, `LPRT-09`.

### RF-CLI-002 — Capturar em ambiente controlado
- **Entrada:** cenário sintético.
- **Saída:** captura datada.
- **Aceitação:** sem dado de jogador; interrupção se surgir dado real.
- **Contrato:** `LCLI-04`, D3–D6.

### RF-CLI-003 — Anonimizar captura
- **Entrada:** bytes brutos.
- **Saída:** frame sem PII.
- **Aceitação:** conta/nome/IP/chat removidos.
- **Contrato:** `LPRT-10`, K5.

### RF-CLI-004 — Produzir ficha de captura
- **Entrada:** captura.
- **Saída:** ficha assinada.
- **Aceitação:** data, build, cenário, operador, QA; segunda leitura por pessoa diferente.
- **Contrato:** `LCLI-03`, K2.

### RF-CLI-005 — Registrar golden
- **Entrada:** frame anonimizado.
- **Saída:** golden `G-<familia>-<nn>`.
- **Aceitação:** origem datada; rejeição sem origem.
- **Contrato:** `LPRT-10`, K1.

### RF-CLI-006 — Listar conteúdo do patch
- **Entrada:** árvore do cliente.
- **Saída:** lista candidata.
- **Aceitação:** descreve exatamente os arquivos; sem redistribuição sem autorização.
- **Contrato:** `LCLI-02`, K4.

### RF-CLI-007 — Manter direitos congelados
- **Entrada:** uso pretendido.
- **Saída:** autorizado/não.
- **Aceitação:** distribuição congelada; evidência do titular para `Q-IP-1`/`Q-IP-2`.
- **Contrato:** `LCLI-05`.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-CLI-001 | Sem dado de jogador em artefato | varredura de PII |
| RNF-CLI-002 | Captura reproduzível | ficha assinada |
| RNF-CLI-003 | Golden com origem | rejeição sem origem |
| RNF-CLI-004 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-CLI-005 | Sem segredo no binário | varredura |

*Página de planejamento. Nenhum requisito foi implementado.*
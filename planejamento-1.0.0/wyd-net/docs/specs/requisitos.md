> Pergunta: "quais requisitos o `wyd-net` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-net-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; `LNET-01` mantém valores de limite como medição aberta.

Requisito sem critério de aceitação é intenção. Numeração `RF-NET-nnn` (funcional) e `RNF-NET-nnn` (não funcional); não se renumera o que já existe.

## Requisitos funcionais

### RF-NET-001 — Aceitar e manter conexão TCP por socket
- **Entrada:** conexão TCP na porta do canal.
- **Saída:** sessão por socket aberta.
- **Aceitação:** aceite registrado; fechamento libera recursos; sem compartilhamento entre conexões.
- **Contrato:** N1, N6.

### RF-NET-002 — Entregar buffer ao codec e devolver mensagem tipada
- **Entrada:** bytes da conexão.
- **Saída:** mensagem tipada ao `wyd-core`.
- **Aceitação:** buffer completo por conexão; bytes crus não saem do crate; nenhum offset cru atravessa.
- **Contrato:** N1, N5, `LPRT-18`.

### RF-NET-003 — Reagir a erro de codec conforme classificação
- **Entrada:** `HeaderError`/`TransformError`/`PayloadError`.
- **Saída:** recuperação ou fechamento conforme classe.
- **Aceitação:** classificação é do codec, reação é da conexão; `UnknownType` não causa queda por si só.
- **Contrato:** N2, N3, `LPRT-13`, `LPRT-08`.

### RF-NET-004 — Garantir ordem por conexão
- **Entrada:** mensagens em sequência.
- **Saída:** despacho FIFO por conexão.
- **Aceitação:** nenhuma reordenação; sem paralelismo de despacho.
- **Contrato:** N4, `LPRT-12`.

### RF-NET-005 — Impor versão de protocolo no handshake
- **Entrada:** build do cliente.
- **Saída:** sessão aberta ou erro tipado.
- **Aceitação:** build-alvo 7662 imposto; build fora do alvo recusado; sem tabela de compatibilidade.
- **Contrato:** N7, `LNET-02`, `Q-NET-2` = A.

### RF-NET-006 — Manter uma sessão viva por conta
- **Entrada:** login/segunda conexão.
- **Saída:** sessão aberta ou recusa.
- **Aceitação:** segunda conexão da mesma conta recusada; queda libera conta.
- **Contrato:** N5, `LNET-03`, `R6`/`D9`.

### RF-NET-007 — Aplicar limites e rate-limit
- **Entrada:** conexão, IP, shard.
- **Saída:** aceite ou recusa tipada + contador + fechamento.
- **Aceitação:** forma fechada por `Q-NET-1` = A; valores medidos no CI efêmero (`LNET-01` aberta).
- **Contrato:** N6, `LNET-01`.

### RF-NET-008 — Expor observabilidade sem payload
- **Entrada:** operação do transporte.
- **Saída:** `/metrics` e logs de metadados.
- **Aceitação:** métricas de conexões/erros/bytes/latência; nenhum payload/PII em log.
- **Contrato:** N6 (observabilidade), `LNET-04`.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-NET-001 | Determinismo da reação a erro | mesma classe → mesma reação |
| RNF-NET-002 | Sem panic em buffer malicioso | fuzz não panica |
| RNF-NET-003 | Ordem por conexão sob carga | teste de carga preserva ordem |
| RNF-NET-004 | Backpressure sem bloqueio indefinido | overflow fecha em tempo limitado |
| RNF-NET-005 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-NET-006 | Observabilidade sem PII | varredura de log |

*Página de planejamento. Nenhum requisito foi implementado.*
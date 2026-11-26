> Pergunta: "quais requisitos o `wyd-server` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-server-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; `LSRV-01`–`LSRV-05` fechadas, implementação pendente.

Requisito sem critério de aceitação é intenção. Numeração `RF-SRV-nnn` (funcional) e `RNF-SRV-nnn` (não funcional).

## Requisitos funcionais

### RF-SRV-001 — Compor um processo por canal
- **Entrada:** configuração de canal.
- **Saída:** processo rodando.
- **Aceitação:** um binário; net+core+dados como bibliotecas; `main` só compõe.
- **Contrato:** `LSRV-01`, S5.

### RF-SRV-002 — Abrir bind por canal
- **Entrada:** canal existente.
- **Saída:** porta aberta.
- **Aceitação:** uma porta por canal; canal inexistente não abre.
- **Contrato:** `LSRV-02`, S5.

### RF-SRV-003 — Responder health e ready
- **Entrada:** requisição HTTP.
- **Saída:** 200 ou 503.
- **Aceitação:** ready 200 só com socket + banco + tick.
- **Contrato:** `LSRV-03`, S3.

### RF-SRV-004 — Receber comando de controle
- **Entrada:** `POST /internal/v1/control`.
- **Saída:** 200 `{delivered_to}` | 401 | 503.
- **Aceitação:** 401 sem token; 503 processo caído; sem ack/escrita no admin.
- **Contrato:** S1, S2, `LAC-ADM-03`.

### RF-SRV-005 — Expor métricas
- **Entrada:** coleta do `wyd-ops`.
- **Saída:** `/metrics` Prometheus.
- **Aceitação:** amostra com instante; sem série no banco; sem PII.
- **Contrato:** S4, `LAC-POR-06`.

### RF-SRV-006 — Encerrar com drenagem e persistência
- **Entrada:** `SIGTERM`.
- **Saída:** processo encerrado.
- **Aceitação:** drena, persiste, fecha fila, sem perda silenciosa.
- **Contrato:** `LSRV-04`, `WYDDB-021`.

### RF-SRV-007 — Suportar captura local (ECAP)
- **Entrada:** bytes em loopback.
- **Saída:** arquivo `.wydcap`.
- **Aceitação:** só loopback; separado de `EPROD`.
- **Contrato:** S7, `docs/specs/endpoint-de-captura.md`.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-SRV-001 | Isolamento entre canais | teste de dois canais |
| RNF-SRV-002 | Prontidão correta | ready 503 sem condição |
| RNF-SRV-003 | Segurança do elo | 401 sem token |
| RNF-SRV-004 | Encerramento sem perda | teste de shutdown |
| RNF-SRV-005 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-SRV-006 | Sem PII em log | varredura de log |

*Página de planejamento. Nenhum requisito foi implementado.*
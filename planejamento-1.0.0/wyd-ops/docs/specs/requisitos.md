> Pergunta: "quais requisitos o `wyd-ops` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-ops/docs/index.md`; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados do índice do sistema; nenhum `RF-OPS` existia antes.

Requisito sem critério de aceitação é intenção. Numeração `RF-OPS-nnn` (funcional) e `RNF-OPS-nnn` (não funcional).

## Requisitos funcionais

### RF-OPS-001 — Executar CI em PR
- **Entrada:** PR tocando código/docs.
- **Saída:** workflow verde.
- **Aceitação:** build, teste, cobertura, gate de docs.
- **Contrato:** `ci`.

### RF-OPS-002 — Construir imagem reproduzível
- **Entrada:** Dockerfile + compose.
- **Saída:** imagem.
- **Aceitação:** build ok; sem segredo na imagem.
- **Contrato:** `imagem`.

### RF-OPS-003 — Provisionar ambiente efêmero
- **Entrada:** CI efêmero.
- **Saída:** ambiente pronto.
- **Aceitação:** Postgres + servidor descartáveis; seed aplicado.
- **Contrato:** `ambiente`, `LBOT-01`/`LBOT-02`.

### RF-OPS-004 — Coletar métricas e logs
- **Entrada:** operação.
- **Saída:** Prometheus/Loki.
- **Aceitação:** métricas de conexões/erros/bytes/latência; sem payload/PII.
- **Contrato:** `monitoramento`, `LNET-04`.

### RF-OPS-005 — Gerenciar segredos
- **Entrada:** secret store.
- **Saída:** segredo rotacionado.
- **Aceitação:** nunca em código; rotação documentada.
- **Contrato:** `segredos`.

### RF-OPS-006 — Publicar e fazer rollback
- **Entrada:** deploy.
- **Saída:** site/servidor publicado.
- **Aceitação:** permissões mínimas; rollback validado.
- **Contrato:** `deploy`, `LPDOC-01`.

### RF-OPS-007 — Executar gate do projeto
- **Entrada:** `docker compose --profile gate`.
- **Saída:** verde.
- **Aceitação:** censos coincidem.
- **Contrato:** `gate`.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-OPS-001 | Permissões mínimas em workflows | revisão de workflow |
| RNF-OPS-002 | Terceiros pinados por SHA | varredura |
| RNF-OPS-003 | Sem segredo no repo | varredura |
| RNF-OPS-004 | Sem payload/PII em log | varredura de log |
| RNF-OPS-005 | Cobertura mínima de 85% onde aplicável | gate de cobertura |

*Página de planejamento. Nenhum requisito foi implementado.*

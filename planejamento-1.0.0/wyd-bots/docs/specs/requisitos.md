> Pergunta: "quais requisitos o `wyd-bots` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-bots-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; `LBOT-01`/`LBOT-02`/`LBOT-04` fechadas, `LBOT-03` aberta como medição.

Requisito sem critério de aceitação é intenção. Numeração `RF-BOT-nnn` (funcional) e `RNF-BOT-nnn` (não funcional).

## Requisitos funcionais

### RF-BOT-001 — Executar cenário de QA de fio
- **Entrada:** cenário declarado.
- **Saída:** sucesso/falha.
- **Aceitação:** login + entrada na cena; critério de sucesso verificável.
- **Contrato:** B1, B2.

### RF-BOT-002 — Executar carga com perfil declarado
- **Entrada:** perfil, duração, degraus.
- **Saída:** relatório de carga.
- **Aceitação:** P50/P99, sucesso, recusas; saturação detectada.
- **Contrato:** B4, `LBOT-03`.

### RF-BOT-003 — Executar coexistência sintética
- **Entrada:** contagem, duração.
- **Saída:** presença estável.
- **Aceitação:** ambiente descartável por execução.
- **Contrato:** B3, B5.

### RF-BOT-004 — Produzir relatório por execução
- **Entrada:** resultado.
- **Saída:** log + captura.
- **Aceitação:** falha reproduzível; sem segredo de jogador.
- **Contrato:** `LBOT-04`.

### RF-BOT-005 — Provisionar ambiente efêmero
- **Entrada:** CI efêmero.
- **Saída:** ambiente pronto.
- **Aceitação:** Postgres + servidor descartáveis; seed aplicado; sem estado entre cenários.
- **Contrato:** `LBOT-01`, `LBOT-02`.

### RF-BOT-006 — Congelar limiar de carga
- **Entrada:** primeira medição.
- **Saída:** limiar com data.
- **Aceitação:** valor medido; cenário exploratório até lá.
- **Contrato:** `LBOT-03`, `Q-BOT-3` = A.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-BOT-001 | Sem segredo de jogador | varredura de config |
| RNF-BOT-002 | Isolamento entre execuções | ambiente descartável |
| RNF-BOT-003 | Sem `sleep` como sincronização | revisão de código |
| RNF-BOT-004 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-BOT-005 | Relatório reproduzível | log + captura |

*Página de planejamento. Nenhum requisito foi implementado.*

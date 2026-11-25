> Pergunta: "qual é a próxima fatia do `wyd-net`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-net`
> Base: `wyd-net-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

A fila tem **6 fatias** em 4 fases. Cada item aponta uma página `WYDNET-nnn` com entregável, DoD e
evidência de fechamento. Fatia sem DoD executável não entra na fila.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDNET-001`–`002` | baseline: lint, docs públicas, contrato do crate |
| 2 | `WYDNET-003`–`004` | sessão e transporte async; sem isso não há canal |
| 3 | `WYDNET-005`–`006` | rate-limit, hardening e observabilidade |
| 4 | `WYDNET-007`–`008` | suíte, fixtures, anti-skip e captura de protocolo |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| captura do protocolo | `WYDNET-003`–`004` | `LPRT-03` e `LPRT-06` sem captura datada do cliente 7662 |
| CI real | `WYDNET-007`–`008` | workflow de código é stub; gate de docs não é suíte do crate |
| medição de limites | `WYDNET-005` | `LNET-01` aberta como medição (decisão `Q-NET-1` = A) |

O bloqueio é de **dado ou ambiente**, não de decisão: `Q-NET-1` está decidido.

## 3. Como ler uma fatia

Cada `WYDNET-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar. O DoD cita comando, arquivo ou documento de saída.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
- **Regra de negócio ou schema:** são de `specs/`; o roadmap só fatia o trabalho.
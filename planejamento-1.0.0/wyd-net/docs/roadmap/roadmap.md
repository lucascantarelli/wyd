> Pergunta: "em que ordem o `wyd-net` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-net`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

Fatias de entrega; o DoD executável de cada uma está na página citada em §2. **Nenhuma issue foi criada**: o roadmap planeja
ordem e prova; abrir issue, milestone ou board é ato separado.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Fatia bloqueada por lacuna (`lacunas.md`) não começa antes de a lacuna fechar.
3. Toda fatia de transporte inclui teste de sessão revogada e de limite excedido.
4. Dependência de captura de protocolo é explícita: sem `LPRT-03`/`LPRT-06`, a fatia não fecha.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Fundação do crate (baseline) | [`WYDNET-F0`](WYDNET-F0.md) §Skills |
| 2 | F1 — Sessão e transporte async | [`WYDNET-F1`](WYDNET-F1.md) §Skills |
| 3 | F2 — Rate-limit e hardening | [`WYDNET-F2`](WYDNET-F2.md) §Skills |
| 4 | F3 — Observabilidade | [`WYDNET-F3`](WYDNET-F3.md) §Skills |
| 5 | F4 — Suíte, fixtures e anti-skip | [`WYDNET-F4`](WYDNET-F4.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | captura de protocolo incompleta | bloquear por `LPRT-03`/`LPRT-06`; não inventar offset |
| F2 | limite mal calibrado | medição `LNET-01` antes de fixar número |
| F3 | log com payload | varredura de segredo/payload no CI |
| F4 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.

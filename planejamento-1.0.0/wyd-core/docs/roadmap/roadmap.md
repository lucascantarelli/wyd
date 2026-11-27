> Pergunta: "em que ordem o `wyd-core` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-core`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Fatia bloqueada por lacuna ou catálogo não começa antes de fechar.
3. Toda fatia de combate inclui teste determinístico (mesmo seed, mesmo resultado).
4. Dependência de protocolo é explícita.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Fundação do crate | [`WYDCORE-F0`](WYDCORE-F0.md) §Skills |
| 2 | F1 — Tick e ECS | [`WYDCORE-F1`](WYDCORE-F1.md) §Skills |
| 3 | F2 — Combate | [`WYDCORE-F2`](WYDCORE-F2.md) §Skills |
| 4 | F3 — IA e capacidades | [`WYDCORE-F3`](WYDCORE-F3.md) §Skills |
| 5 | F4 — Suíte e anti-skip | [`WYDCORE-F4`](WYDCORE-F4.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F2 | fórmula não provada | fixture capturada; `stat_source` explícito |
| F3 | escopo infinito | catálogo fechado antes de codar |
| F4 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.

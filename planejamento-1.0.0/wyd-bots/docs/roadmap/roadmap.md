> Pergunta: "em que ordem o `wyd-bots` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-bots`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Toda fatia de carga inclui medição real (sem número inventado).
3. Ambiente efêmero por execução; sem estado global.
4. Dependência de `wyd-server`/`wyd-net` é explícita.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Ambiente e baseline | [`WYDBOT-F0`](WYDBOT-F0.md) §Skills |
| 2 | F1 — Bots de QA e carga | [`WYDBOT-F1`](WYDBOT-F1.md) §Skills |
| 3 | F2 — Convivência e medição | [`WYDBOT-F2`](WYDBOT-F2.md) §Skills |
| 4 | F3 — Suíte e anti-skip | [`WYDBOT-F3`](WYDBOT-F3.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | número inventado | medição real obrigatória |
| F2 | ambiente não efêmero | Compose por execução |
| F3 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
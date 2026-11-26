> Pergunta: "em que ordem o `wyd-sim` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-sim`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Toda fórmula é determinística: mesmo seed, mesmo resultado.
3. Nenhum número é inventado; `stat_source` declara o que não foi medido.
4. Dependência de captura/catálogo é explícita.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Fundação do crate | [`WYDSIM-F0`](WYDSIM-F0.md) §Skills |
| 2 | F1 — Fórmulas e curvas | [`WYDSIM-F1`](WYDSIM-F1.md) §Skills |
| 3 | F2 — Simulações determinísticas | [`WYDSIM-F2`](WYDSIM-F2.md) §Skills |
| 4 | F3 — Suíte e anti-skip | [`WYDSIM-F3`](WYDSIM-F3.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F1 | número inventado | fixture capturada + `stat_source` |
| F2 | não-determinismo | seed fixo em teste |
| F3 | SKIP silencioso | gate de anti-skip |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
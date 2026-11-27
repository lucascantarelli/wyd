> Pergunta: "em que ordem o `wyd-docs` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-docs`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Site implementado; plano aberto.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Nenhum conteúdo interno no artefato publicado.
3. Toda fatia de publicação inclui rollback validado.
4. Redirect obrigatório ao renomear.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Montagem e tema | [`WYDDOC-F0`](WYDDOC-F0.md) §Skills |
| 2 | F1 — Gate de link, âncora, nav, título | [`WYDDOC-F1`](WYDDOC-F1.md) §Skills |
| 3 | F2 — Contagens e censos | [`WYDDOC-F2`](WYDDOC-F2.md) §Skills |
| 4 | F3 — Publicação e redirect | [`WYDDOC-F3`](WYDDOC-F3.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | conteúdo interno publicado | varredura de `.build/docs/` |
| F1 | gate decorativo | sem modo tolerante |
| F3 | Windows | `LPDOC-12` como decisão |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.

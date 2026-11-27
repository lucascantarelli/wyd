> Pergunta: "em que ordem o `wyd-ops` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-ops`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Permissões mínimas em workflows; terceiros pinados por SHA.
3. Nenhum segredo em código, log ou documento.
4. Deploy e rollback validados antes de produção.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — CI | [`WYDOPS-F0`](WYDOPS-F0.md) §Skills |
| 2 | F1 — Imagem | [`WYDOPS-F1`](WYDOPS-F1.md) §Skills |
| 3 | F2 — Ambiente | [`WYDOPS-F2`](WYDOPS-F2.md) §Skills |
| 4 | F3 — Monitoramento | [`WYDOPS-F3`](WYDOPS-F3.md) §Skills |
| 5 | F4 — Segredos e deploy | [`WYDOPS-F4`](WYDOPS-F4.md) §Skills |
| 6 | F5 — Gate e fechamento | [`WYDOPS-F5`](WYDOPS-F5.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | permissão excessiva | revisão de workflow |
| F1 | segredo na imagem | varredura |
| F4 | deploy sem rollback | rollback validado |
| F5 | gate decorativo | censos coincidem |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.

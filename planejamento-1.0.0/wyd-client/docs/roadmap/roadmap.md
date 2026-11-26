> Pergunta: "em que ordem o `wyd-client` é construído?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-client`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Nenhuma informação de IP/credencial do titular entra em documento.
3. Toda fatia de captura inclui hash e tamanho do artefato.
4. Dependência de pedido externo é explícita.

## 2. Fatias

Fase, requisitos, dependência, entregável, DoD e fora de escopo de cada fatia têm **dono único na
página da issue**; esta página cita o link e não repete. A ordem abaixo é a do roadmap, e a prova
de término é o `## DoD` da página citada.

| Ordem | Fatia | Página (detalhe e §Skills) |
|---:|---|---|
| 1 | F0 — Inventário e captura | [`WYDCLI-F0`](WYDCLI-F0.md) §Skills |
| 2 | F1 — Patches e configuração do cliente | [`WYDCLI-F1`](WYDCLI-F1.md) §Skills |
| 3 | F2 — Fechamento | [`WYDCLI-F2`](WYDCLI-F2.md) §Skills |

## 3. Ordem e dependências

A sequência de execução é a da tabela de §2, na ordem das linhas. O que cada fatia exige antes
de começar — outra fatia, lote de schema, lacuna, captura ou decisão — é o campo `## Depende de`
da página dela; esta página não repete aresta nem grafo.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | pedido não respondido | escopo B; responsável nominal |
| F1 | config quebra cliente | teste de regressão |
| F2 | lacuna sem dono | `lacunas.md` com owner+prazo |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
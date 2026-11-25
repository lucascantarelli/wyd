> Pergunta: "qual é a próxima fatia do `wyd-docs`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-docs`
> Base: `wyd-docs-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Site implementado; plano aberto. Nenhuma issue criada.

A fila tem **4 fatias**, uma por página citada abaixo; cada página traz entregável, DoD
e evidência de fechamento. Fatia sem DoD executável não entra na fila.

## 1. Fila

A ordem das fatias é a de `roadmap.md` §2; a fase e a prioridade de cada uma são os campos
`**Fase:**` e `**Prioridade:**` da página dela. Esta página não repete fila, fase nem prioridade.

## 2. O que está bloqueado

O bloqueio de cada fatia — lacuna, captura, schema ou CI — é o `## Depende de` da página
dela; esta página não repete o motivo nem a lista de afetadas.

## 3. Como ler uma fatia

Cada página de fatia tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.

## Páginas de issue por fatia

Medido em 2026-11-25: **4** páginas, uma por fatia, no formato de`WYDDB-nnn`
(dependências, DoD observável e bloco de skills); o estado de cada uma é o `**Estado:**` da própria página. O número sequencial de cada issue é
atribuído na criação pelo `github-mapping.md`; a chave aqui é a fatia.

| Issue | Título |
|---|---|
| [`WYDDOC-F0`](WYDDOC-F0.md) | Montagem e tema |
| [`WYDDOC-F1`](WYDDOC-F1.md) | Gate de link, âncora, nav, título |
| [`WYDDOC-F2`](WYDDOC-F2.md) | Contagens e censos |
| [`WYDDOC-F3`](WYDDOC-F3.md) | Publicação e redirect |

> Pergunta: "qual é a próxima fatia do `wyd-sim`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-sim`
> Base: `wyd-sim-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; as 4 fatias foram publicadas como issues e estão no board (inventário em `wyd-ops/docs/github-mapping.md` §7).

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

Medido em 2026-11-25: **4** páginas, uma por fatia, no formato de `WYDDB-nnn`
(dependências, DoD observável e bloco de skills); o estado de cada uma é o `**Estado:**` da própria página. O número sequencial de cada issue é
atribuído na criação pelo `github-mapping.md`; a chave aqui é a fatia.

| Issue | Título |
|---|---|
| [`WYDSIM-F0`](WYDSIM-F0.md) | Fundação do crate |
| [`WYDSIM-F1`](WYDSIM-F1.md) | Fórmulas e curvas |
| [`WYDSIM-F2`](WYDSIM-F2.md) | Simulações determinísticas |
| [`WYDSIM-F3`](WYDSIM-F3.md) | Suíte e anti-skip |

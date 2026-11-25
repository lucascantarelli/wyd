> Pergunta: "qual é a próxima fatia do `wyd-ops`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-ops`
> Base: `wyd-ops/docs/index.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada. `wyd-ops` não tinha
dashboard próprio até esta rodada.

A fila tem **6 fatias**, uma por página citada abaixo; cada página traz entregável, DoD
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

Medido em 2026-11-25: **6** páginas, uma por fatia, no formato de`WYDDB-nnn`
(dependências, DoD observável e bloco de skills); o estado de cada uma é o `**Estado:**` da própria página. O número sequencial de cada issue é
atribuído na criação pelo `github-mapping.md`; a chave aqui é a fatia.

| Issue | Título |
|---|---|
| [`WYDOPS-F0`](WYDOPS-F0.md) | CI |
| [`WYDOPS-F1`](WYDOPS-F1.md) | Imagem |
| [`WYDOPS-F2`](WYDOPS-F2.md) | Ambiente |
| [`WYDOPS-F3`](WYDOPS-F3.md) | Monitoramento |
| [`WYDOPS-F4`](WYDOPS-F4.md) | Segredos e deploy |
| [`WYDOPS-F5`](WYDOPS-F5.md) | Gate e fechamento |

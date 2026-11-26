> Pergunta: "qual é a próxima fatia do `wyd-net`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-net`
> Base: `wyd-net-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; as 5 fatias foram publicadas como issues e estão no board (inventário em `wyd-ops/docs/github-mapping.md` §7).

A fila tem **5 fatias**, uma por página citada abaixo; cada página traz entregável, DoD
e evidência de fechamento. Fatia sem DoD executável não entra na fila.

## 1. Fila

A ordem das fatias é a de `roadmap.md` §2; a fase e a prioridade de cada uma são os campos
`**Fase:**` e `**Prioridade:**` da página dela. Esta página não repete fila, fase nem prioridade.

## 2. O que está bloqueado

O bloqueio de cada fatia — lacuna, captura, schema ou CI — é o `## Depende de` da página
dela; esta página não repete o motivo nem a lista de afetadas.

## 3. Como ler uma fatia

Cada página de fatia tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar. O DoD cita comando, arquivo ou documento de saída.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
- **Regra de negócio ou schema:** são de `specs/`; o roadmap só fatia o trabalho.

## Páginas de issue por fatia

Medido em 2026-11-25: **5** páginas, uma por fatia, no formato de`WYDDB-nnn`
(dependências, DoD observável e bloco de skills); o estado de cada uma é o `**Estado:**` da própria página. O número sequencial de cada issue é
atribuído na criação pelo `github-mapping.md`; a chave aqui é a fatia.

| Issue | Título |
|---|---|
| [`WYDNET-F0`](WYDNET-F0.md) | Fundação do crate (baseline) |
| [`WYDNET-F1`](WYDNET-F1.md) | Sessão e transporte async |
| [`WYDNET-F2`](WYDNET-F2.md) | Rate-limit e hardening |
| [`WYDNET-F3`](WYDNET-F3.md) | Observabilidade |
| [`WYDNET-F4`](WYDNET-F4.md) | Suíte, fixtures e anti-skip |

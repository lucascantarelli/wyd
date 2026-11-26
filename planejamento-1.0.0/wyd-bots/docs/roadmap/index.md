> Pergunta: "qual é a próxima fatia do `wyd-bots`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-bots`
> Base: `wyd-bots-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; as 4 fatias foram publicadas como issues e estão no board (inventário em `wyd-ops/docs/github-mapping.md` §7).

As lacunas `LBOT-01`, `LBOT-02` e `LBOT-04` fecharam em 2026-11-23; `LBOT-03` tem política
decidida (`Q-BOT-3` = A) e só fecha com a primeira execução. A fila tem **4 fatias**, uma por página citada abaixo; cada página traz entregável, DoD
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
| [`WYDBOT-F0`](WYDBOT-F0.md) | Ambiente e baseline |
| [`WYDBOT-F1`](WYDBOT-F1.md) | Bots de QA e carga |
| [`WYDBOT-F2`](WYDBOT-F2.md) | Convivência e medição |
| [`WYDBOT-F3`](WYDBOT-F3.md) | Suíte e anti-skip |

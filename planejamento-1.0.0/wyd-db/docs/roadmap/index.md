> Pergunta: "qual é a próxima tarefa, o que está bloqueado e onde está o DoD?"
> Público: Produto, desenvolvimento, QA e operação
> Dono: equipe do `wyd-db`
> Base: `D-7` aprovada em 2026-11-22; `fases.md`; `wyd-db-1.0.0.md` §6.10
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A fila tem **40 fatias**, uma por página citada abaixo; cada página traz entregável, DoD
e evidência de fechamento. Fatia sem DoD executável não entra na fila.

## 1. Fila

A ordem das fatias é a de `fases.md` §2; a fase e a prioridade de cada uma são os campos
`**Fase:**` e `**Prioridade:**` da página dela. Esta página não repete fila, fase nem prioridade.

## 2. O que está bloqueado

O bloqueio de cada fatia — lacuna, captura, schema ou CI — é o `## Depende de` da página
dela; esta página não repete o motivo nem a lista de afetadas.

## 3. Como ler uma issue

Cada página de fatia tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar. O DoD cita comando, arquivo ou documento de saída; a evidência fica
na issue do GitHub e o número é registrado no relatório de entrega.

A dependência de cada página é **espelhada no GitHub como vínculo `blocked by`**, e não como checklist
em texto: em 2026-11-22 são 63 vínculos entre as 40 issues, e `WYDDB-001` é a única sem bloqueador.
Bloqueio por dado ou ambiente que não é outra issue (seed, captura, CI, matriz) fica na label
`estado/bloqueada` e no `Status` do board, com o detalhe em
`planejamento-1.0.0/wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3.

## 4. O que esta página não cobre

- **Números medidos de progresso:** `planos/metricas.md` ainda não foi escrito; não há percentual.
- **Prazo e custo:** não há estimativa aprovada.
- **Regra de negócio ou schema:** são de `specs/` e do dicionário; o roadmap só fatia o trabalho.

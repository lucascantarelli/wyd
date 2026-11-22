# WYDDB-018 — drenar ops.delivery_queue com idempotência

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 4 — filas, save e shutdown
**Prioridade:** P2

## Contexto

A fila é a única ponte de escrita entre portal e jogo; sem consumidor idempotente, um retry entrega item ou moeda duas vezes.

## Entregável

Implementar o consumidor de `ops.delivery_queue` com `ops.idempotency_record`, marcando a entrega antes/depois do efeito conforme o DDL, e liberar a linha só após o efeito confirmado.

## DoD

- [ ] Teste prova que a mesma chave de idempotência aplica o efeito uma vez.
- [ ] Falha no meio não marca a entrega como concluída.
- [ ] O consumidor não escreve em `portal`;
- [ ] O teste cobre fila vazia, linha pendente e reentrega.

## Fora de escopo

Não definir frequência de drenagem, tamanho de lote ou exactly-once; a garantia é efeito único por chave.

## Risco

A ordem entre marcar idempotência e aplicar efeito é uma decisão de transação; o DDL e o teste devem fixá-la.

## Depende de

WYDDB-012, WYDDB-017.

## Evidência ao fechar

Teste de sistema da fila e página do domínio `ponte-web-jogo`.

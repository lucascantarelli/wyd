# WYDDB-021 — fechar save e filas no shutdown

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 4 — filas, save e shutdown
**Prioridade:** P2

## Contexto

Um shutdown que mata o processo sem flush perde o último lote de save e deixa entrega de fila em estado ambíguo; o operador não sabe o que foi aplicado.

## Entregável

Implementar a sequência de shutdown: parar de aceitar trabalho, drenar o lote de save, fechar a fila no ponto seguro e só então encerrar o pool; registrar o que ficou pendente.

## DoD

- [ ] Teste com trabalho em voo prova que o lote é gravado ou explicitamente descartado.
- [ ] A fila não é marcada como concluída antes do efeito.
- [ ] O shutdown não espera indefinidamente; usa o timeout já configurado, sem inventar número.
- [ ] O log final diz quantos lotes/entregas ficaram pendentes.

## Fora de escopo

Não definir sinal, tempo de graça ou política de kill; o ambiente operacional decide.

## Risco

Drenar sem parar de aceitar novo trabalho pode nunca terminar; a ordem de parada é o contrato.

## Depende de

WYDDB-017, WYDDB-018.

## Evidência ao fechar

Teste de shutdown com trabalho em voo e página `src/transfer-save.md`.

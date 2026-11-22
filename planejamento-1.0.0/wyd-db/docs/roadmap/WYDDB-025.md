# WYDDB-025 — expor sinais de lock e drift

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 5 — observabilidade e diagnóstico
**Prioridade:** P2

## Contexto

O operador precisa ver lock ocupado e drift sem abrir o código; o runner já tem os estados, mas não há sinal operacional definido.

## Entregável

Expor no `Report`/log a contagem de `findings` por `DriftKind` e o tempo de espera do lock, sem criar limiar ou alerta automático.

## DoD

- [ ] Teste confere contagem por tipo de drift e tempo de lock no resumo.
- [ ] Nenhum limiar ou alerta é inventado; o dado é descritivo.
- [ ] O sinal não inclui SQL, DSN ou PII.
- [ ] `status --check` e `apply` usam o mesmo cálculo.

## Fora de escopo

Não configurar alerta, painel ou SLO; isso depende do `wyd-ops` e de medição.

## Risco

Um contador sem dono vira métrica órfã; a issue deve registrar onde o dado é consumido.

## Depende de

WYDDB-022, WYDDB-024.

## Evidência ao fechar

Teste de `Report` e exemplo de saída no PR.

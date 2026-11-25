> Pergunta: "como o `wyd-core` prova o tick, o combate e a persistência sem inventar estado?"
> Público: QA e desenvolvedor do `wyd-core`
> Dono: equipe do wyd-core
> Base: `AGENTS.md` §8; `wyd-core-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `tick`, `entities`, `validation` | passos, CRUD, pré-condições | funções puras |
| Contrato | `tests/contrato/` | repositório falso, `wyd-sim` | frames/mensagens |
| Integração | `tests/integracao/` | cena + combate | mundo em memória |
| Sistema | `tests/sistema/` | PostgreSQL + UoW | banco efêmero |
| E2E | `tests/e2e/` | ponta a ponta | ambiente efêmero |
| Propriedade | `tests/propriedade/` | determinismo com semente | corpus gerado |

## O que cada teste precisa provar

1. **Determinismo do tick:** mesma semente → mesma sequência; sem `sleep`.
2. **Fronteira core × sim:** fórmula vem do `sim`; duplicação é defeito; mudar estado muda decisão do core, não saída do `sim`.
3. **Persistência sem SQL no core:** grep sem `sqlx`/`SELECT`/etc.; contrato com repositório falso.
4. **UoW atômica:** duas operações gravam ambas ou nenhuma; constraint vira `InvariantViolation`.
5. **`CommitUncertain`:** nova leitura, sem reenvio cego.
6. **Ausência é `None`:** `load_session`/`load_character` ausentes.
7. **`grep` de proibição:** `wyd-sim` sem `CoreRepository`/`UnitOfWork`/`StoreError`.

## Isolamento

- Testes Rust com `cargo test`; mundo em memória por teste; banco efêmero para sistema.
- Fixtures por factory; nenhum id mágico.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade.

## Comandos

```bash
cargo test -p wyd-core --all-targets
cargo llvm-cov -p wyd-core --fail-under-lines 85
```

## Critério de saída da `1.0.0`

Todos os testes de tick, combate, persistência e contrato verdes; nenhuma lacuna de prova aberta; `grep` de proibição limpo. `LCORE-01`–`LCORE-04` já fechadas; implementação pendente.
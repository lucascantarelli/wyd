> Pergunta: "como o `wyd-sim` prova determinismo e aritmética sem inventar número?"
> Público: QA e desenvolvedor do `wyd-sim`
> Dono: equipe do wyd-sim
> Base: `AGENTS.md` §8; `wyd-sim-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `arithmetic`, `version` | primitivas, versão | funções puras |
| Contrato | `tests/contrato/` | fixtures JSON | entrada/saída esperada |
| Integração | `tests/integracao/` | core × sim | vetor |
| Sistema | (n/a) | — | — |
| E2E | (n/a) | — | — |
| Propriedade | `tests/propriedade/` | determinismo com vetor | corpus gerado |

## O que cada teste precisa provar

1. **Determinismo:** mesma entrada → mesma saída, em todas as plataformas.
2. **Aritmética inteira:** sem `f64` em estado/economia; percentual em bps.
3. **Arredondamento:** meio-para-longe-do-zero uma vez por fórmula.
4. **Overflow verificado:** sem overflow silencioso.
5. **`formula_version`:** declarada na spec e no cabeçalho da fixture.
6. **Fixture:** entrada, saída esperada, versão de fórmula; divergência reprova.
7. **Sem número inventado:** valor sem fonte é `H` com lacuna.

## Isolamento

- Testes Rust com `cargo test`; sem I/O, sem relógio, sem RNG.
- Fixtures por arquivo JSON versionado.
- Proibido `sleep`.
- Cobertura mínima de 85% por capacidade.

## Comandos

```bash
cargo test -p wyd-sim --all-targets
cargo llvm-cov -p wyd-sim --fail-under-lines 85
```

## Critério de saída da `1.0.0`

Todos os testes de fórmula e fixture verdes; `LSIM-01`–`LSIM-04` implementados; nenhuma lacuna de prova aberta; nenhum número de balanceamento inventado. `LSIM-01`–`LSIM-04` fechadas; implementação pendente.
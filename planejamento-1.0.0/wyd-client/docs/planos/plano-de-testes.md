> Pergunta: "como o `wyd-client` prova captura, anonimização e golden sem inventar dado?"
> Público: QA e desenvolvedor do `wyd-client`
> Dono: equipe do wyd-client
> Base: `AGENTS.md` §8; `wyd-client-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `build-identity`, `anonymization` | hash, remoção de PII | funções puras |
| Contrato | `tests/contrato/` | golden, ficha | captura datada |
| Integração | `tests/integracao/` | captura local | ambiente controlado |
| Sistema | (n/a) | — | — |
| E2E | `tests/e2e/` | fluxo do launcher/cliente | Playwright |
| Propriedade | `tests/propriedade/` | fuzz de manifesto | corpus gerado |

## O que cada teste precisa provar

1. **Build-alvo:** 7662 + SHA-256 medido; comparação entre builds não medida.
2. **Captura:** ambiente sintético; sem dado de jogador; interrupção se surgir dado real.
3. **Anonimização:** frame sem PII; varredura de log.
4. **Ficha:** data, build, cenário, operador, QA; segunda leitura por pessoa diferente.
5. **Golden:** origem datada; rejeição sem origem.
6. **IP/direitos:** distribuição congelada; nenhum conteúdo publicado sem autorização.
7. **Patch:** lista candidata; hash verificado.

## Isolamento

- Testes Rust com `cargo test`; ambiente controlado por teste.
- Fixtures por captura datada; nenhum id mágico.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade.

## Comandos

```bash
cargo test -p wyd-client --all-targets
cargo llvm-cov -p wyd-client --fail-under-lines 85
```

## Critério de saída da `1.0.0`

Todos os testes de captura, anonimização e golden verdes; `LCLI-01` fechado; `LCLI-02`/`LCLI-03`/`LCLI-05` abertas até captura/evidência; nenhuma lacuna de prova aberta. `LCLI-04` fechada; implementação pendente. A1 autorizada.
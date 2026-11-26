> Pergunta: "como o `wyd-net` prova o transporte sem inventar byte nem latência?"
> Público: QA e desenvolvedor do `wyd-net`
> Dono: equipe do wyd-net
> Base: `AGENTS.md` §8; `wyd-net-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `transport`, `session`, `error` | aceite, timeout, identidade, taxonomia | funções puras, sem I/O |
| Contrato | `tests/contrato/` | contrato com `wyd-protocol` e `wyd-core` | frames e mensagens tipadas |
| Integração | `tests/integracao/` | conexão real, buffer parcial, timeout | socket efêmero |
| Sistema | `tests/sistema/` | portas falsas, backpressure, overflow | servidor local |
| E2E | `tests/e2e/` | fio ponta a ponta com codec real | ambiente efêmero |
| Propriedade | `tests/propriedade/` | fuzz de buffer sem panic | corpus gerado |

## O que cada teste precisa provar

1. **Nenhum byte afirmado sem prova:** toda afirmação de layout aponta para golden do `wyd-protocol` ou captura datada.
2. **Ordem por conexão preservada:** nenhuma reordenação de mensagens por conexão.
3. **Bytes crus não atravessam:** nenhum offset cru chega ao `wyd-core`.
4. **`UnknownType` não causa queda:** logado/descartado, conexão segue.
5. **Segunda conexão da mesma conta recusada:** `R6`/`D9`.
6. **Build fora do alvo recusado:** erro tipado no handshake.
7. **Overflow fecha a conexão:** com erro tipado.
8. **Timeout ocioso fecha:** com contador.
9. **Medição de limites:** valores entram só após primeira execução no CI efêmero (`LNET-01`).

## Isolamento

- Testes Rust com `cargo test`; socket efêmero por teste, sem estado compartilhado.
- Fixtures por factory de bytes; nenhum id mágico.
- Proibido `sleep` como sincronização; usar timeout explícito.
- Nenhum estado global entre testes; nenhum `#[ignore]` sem issue dona.
- Cobertura mínima de 85% por capacidade, não por linha.

## Comandos

```bash
cargo test -p wyd-net --all-targets
cargo llvm-cov -p wyd-net --fail-under-lines 85
```

## CI, cobertura e monitoramento

- **CI:** o job `code` de `.github/workflows/code.yml` roda `cargo fmt --all -- --check`, `cargo clippy -p wyd-net --all-targets -- -D warnings`, `cargo test -p wyd-net --all-targets` e `cargo deny check`; o PR para `develop` e `main` só passa com os quatro verdes.
- **Cobertura:** `cargo llvm-cov -p wyd-net --fail-under-lines 85` é o piso; a cobertura de linha não substitui os comportamentos listados acima e `#[ignore]` exige issue dona e prazo.
- **Monitoramento:** `/metrics` com conexões abertas, bytes por sentido, erros por classe, timeouts e recusas por motivo; log só de metadados. `LNET-01` continua aberta: nenhum limite ou P99 é afirmado antes da medição.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Todos os testes de transporte, sessão, contrato e backpressure verdes; `LNET-01` fechada com medição datada; nenhuma lacuna de prova aberta; nenhuma página de `transport/` reprovada. `LNET-01` permanece aberta até medição real.
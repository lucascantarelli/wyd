> Pergunta: "como o `wyd-bots` prova cenário, carga e relatório sem inventar número?"
> Público: QA e desenvolvedor do `wyd-bots`
> Dono: equipe do wyd-bots
> Base: `AGENTS.md` §8; `wyd-bots-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `config`, `scenario-runner` | config, execução | funções puras |
| Contrato | `tests/contrato/` | codec em modo cliente | frames |
| Integração | `tests/integracao/` | cenário completo | ambiente efêmero |
| Sistema | `tests/sistema/` | carga, coexistência | CI efêmero |
| E2E | (n/a) | — | — |
| Propriedade | `tests/propriedade/` | fuzz de cenário | corpus gerado |

## O que cada teste precisa provar

1. **QA de fio:** cenário de login + entrada na cena com critério de sucesso.
2. **Carga:** P50/P99, sucesso, recusas em degraus; saturação = primeiro degrau com recusa ou P99 sem retorno.
3. **Coexistência:** presença estável.
4. **Isolamento:** ambiente descartável por execução; sem estado global.
5. **Relatório:** log + captura por execução falha; sem segredo de jogador.
6. **Sem `sleep`:** sincronização por timeout explícito.
7. **Limiar congelado:** valor medido com data; cenário exploratório até lá.

## Isolamento

- Testes Rust com `cargo test`; ambiente efêmero por cenário.
- Seed declarativo em `crates/wyd-db/seeds/dev.sql`.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade.

## Comandos

```bash
cargo test -p wyd-bots --all-targets
cargo llvm-cov -p wyd-bots --fail-under-lines 85
```

## Critério de saída da `1.0.0`

Todos os testes de cenário, carga e relatório verdes; `LBOT-01`/`LBOT-02`/`LBOT-04` implementados; `LBOT-03` aberta até medição; nenhuma lacuna de prova aberta. `LBOT-03` segue aberta como medição.
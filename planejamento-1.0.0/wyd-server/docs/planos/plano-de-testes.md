> Pergunta: "como o `wyd-server` prova composição, prontidão e encerramento sem perder estado?"
> Público: QA e desenvolvedor do `wyd-server`
> Dono: equipe do wyd-server
> Base: `AGENTS.md` §8; `wyd-server-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `config`, `auth`, `health` | config, token, ready | funções puras |
| Contrato | `tests/contrato/` | controle, composição | requisições HTTP |
| Integração | `tests/integracao/` | dois canais isolados | processos locais |
| Sistema | `tests/sistema/` | shutdown, ready | ambiente efêmero |
| E2E | `tests/e2e/` | ponta a ponta | ambiente efêmero |
| Propriedade | `tests/propriedade/` | fuzz de config | corpus gerado |

## O que cada teste precisa provar

1. **Composição:** um binário, um processo por canal; dois canais isolados.
2. **`ready`:** 200 só com socket + banco + tick; senão 503.
3. **Controle:** 401 sem token; 503 processo caído; 200 com `delivered_to`.
4. **Shutdown:** `SIGTERM` drena e persiste, sem perda silenciosa.
5. **ECAP:** captura local em loopback; não exposto externamente.
6. **Sem PII em log:** varredura de log.

## Isolamento

- Testes Rust com `cargo test`; processo efêmero por teste.
- Fixtures por factory; nenhum id mágico.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade.

## Comandos

```bash
cargo test -p wyd-server --all-targets
cargo llvm-cov -p wyd-server --fail-under-lines 85
```

## Critério de saída da `1.0.0`

Todos os testes de composição, ready, controle e shutdown verdes; `LSRV-01`–`LSRV-05` implementadas; A1 com captura; nenhuma lacuna de prova aberta. `LSRV-05` (peer) segue dependente da captura.
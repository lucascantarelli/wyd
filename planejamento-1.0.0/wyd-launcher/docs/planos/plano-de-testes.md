> Pergunta: "como o `wyd-launcher` prova manifesto, assinatura e patch sem inventar byte?"
> Público: QA e desenvolvedor do `wyd-launcher`
> Dono: equipe do wyd-launcher
> Base: `AGENTS.md` §8; `wyd-launcher-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `manifest`, `signature`, `integrity` | validação, Ed25519, hash | funções puras |
| Contrato | `tests/contrato/` | manifesto, canais | JSON versionado |
| Integração | `tests/integracao/` | patch end-to-end local | arquivos locais |
| Sistema | (n/a) | — | — |
| E2E | (n/a) | app nativo | — |
| Propriedade | `tests/propriedade/` | fuzz de manifesto | corpus gerado |

## O que cada teste precisa provar

1. **Manifesto:** JSON canônico; hash por arquivo; divergência reprova.
2. **Assinatura:** Ed25519 verificada antes de aplicar; chave errada reprova.
3. **Patch:** arquivo divergente não inicia; patch aplicado só com assinatura válida.
4. **Integridade:** arquivo ok/divergente/ausente classificado.
5. **Canais:** canal fora do ar oculto; cache local.
6. **Sem segredo no binário:** varredura de binário.
7. **Updater:** update disponível/indisponível.

## Isolamento

- Testes Rust com `cargo test`; arquivos temporários por teste.
- Fixtures por arquivo JSON versionado.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade.

## Comandos

```bash
cargo test -p wyd-launcher --all-targets
cargo llvm-cov -p wyd-launcher --fail-under-lines 85
```

## CI, cobertura e monitoramento

- **CI:** o job `code` de `.github/workflows/code.yml` roda `cargo fmt --all -- --check`, `cargo clippy -p wyd-launcher --all-targets -- -D warnings`, `cargo test -p wyd-launcher --all-targets` e `cargo deny check`; o PR para `develop` e `main` só passa com os quatro verdes.
- **Cobertura:** `cargo llvm-cov -p wyd-launcher --fail-under-lines 85` é o piso; a cobertura de linha não substitui os comportamentos listados acima e `#[ignore]` exige issue dona e prazo.
- **Monitoramento:** Resultado da verificação de manifesto, hash divergente, falha de assinatura e update aplicado. Log local sem token nem conteúdo do jogador; não há telemetria remota por desenho.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Todos os testes de manifesto, assinatura, patch e integridade verdes; `LLCH-01`–`LLCH-04` implementados; nenhuma lacuna de prova aberta. `LLCH-01`–`LLCH-04` fechadas; implementação pendente.
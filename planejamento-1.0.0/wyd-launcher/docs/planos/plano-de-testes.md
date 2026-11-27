> Pergunta: "como o `wyd-launcher` prova manifesto, assinatura e patch sem inventar byte?"
> Público: QA e desenvolvedor do `wyd-launcher`
> Dono: equipe do wyd-launcher
> Base: `AGENTS.md` §8; `wyd-launcher-1.0.0.md` §6; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-27

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

## Fixtures byte-exatas

As fixtures vivem em `crates/wyd-launcher/tests/fixtures/` e são **byte-exatas**: o teste compara
bytes, não texto. `assinatura/manifest.json.sig` é a assinatura Ed25519 em hexadecimal sobre a forma
canônica RFC 8785 de `assinatura/manifest.json`, `chave-publica.hex` e `chave-errada.hex` são chaves
em hexadecimal, e `dados/tabela.bin` é a tabela cujo hash e tamanho o manifesto declara. Um `\n` a
mais no fim de qualquer uma delas muda o valor lido e reprova o teste sem que nenhuma linha de código
mude.

`dados/tabela.bin` não tem byte NUL, então o git não a reconhece como binária e `text=auto` a
trataria como texto. Por isso a exclusão é explícita, em dois lugares, e as duas precisam continuar
existindo:

- `.gitattributes`: `crates/wyd-launcher/tests/fixtures/** -text` impede a conversão de fim de linha
  no checkout.
- `.pre-commit-config.yaml`: `trailing-whitespace` e `end-of-file-fixer` excluem o mesmo caminho.

Não remova a exclusão nem “formate” a fixture: o valor esperado vem dos bytes. Fixture nova nasce da
geração com a chave de teste e é conferida pelo hash, não de edição manual.

## Comandos

```bash
cargo test --manifest-path crates/wyd-launcher/Cargo.toml --locked --all-targets
# Mesmo comando do job `coverage` do CI, que falha abaixo de 85% de linha.
cargo llvm-cov --manifest-path crates/wyd-launcher/Cargo.toml --fail-under-lines 85
```

## CI, cobertura e monitoramento

- **CI:** o job `code` de `.github/workflows/code.yml` roda `cargo fmt --manifest-path crates/wyd-launcher/Cargo.toml -- --check`, `cargo clippy --manifest-path crates/wyd-launcher/Cargo.toml --all-targets -- -D warnings` e `cargo test --manifest-path crates/wyd-launcher/Cargo.toml --locked --all-targets` em `ubuntu-latest`; `cargo deny` cobre o crate em `security.yml`. O núcleo é Rust puro (sem Tauri nem `cfg(windows)`), então fmt, clippy e teste rodam em Linux; só o empacotamento NSIS do instalador, quando existir, exige Windows. O PR para `develop` e `main` só passa com os checks verdes.
- **Cobertura:** `cargo llvm-cov --fail-under-lines 85` é o piso, aplicado pelo job `coverage` do `code.yml` em `ubuntu-latest` desde 2026-11-27; medição de 2026-11-27: 92,48% de linha (545 linhas, 41 não cobertas). A cobertura de linha não substitui os comportamentos listados acima e `#[ignore]` exige issue dona e prazo. `wyd-db` continua informativo (`D-02 = B`).
- **Monitoramento:** Resultado da verificação de manifesto, hash divergente, falha de assinatura e update aplicado. Log local sem token nem conteúdo do jogador; não há telemetria remota por desenho.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Todos os testes de manifesto, assinatura, patch e integridade verdes; `LLCH-01`–`LLCH-04` implementados; nenhuma lacuna de prova aberta. `LLCH-01`–`LLCH-04` fechadas; implementação pendente.

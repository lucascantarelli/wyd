---
name: atualizar-manifesto
description: Atualiza o manifesto de patch do launcher WYD com caminho, papel, tamanho e sha256 medidos, mantendo a verificação de integridade e sem versionar binário de terceiro. Use ao publicar patch, ao corrigir hash divergente e ao registrar arquivo novo do cliente.
---

# Atualizar manifesto

## Quando usar

- Arquivo novo do cliente entra no patch.
- Hash ou tamanho do manifesto divergiu do arquivo real.

## Pré-requisitos

- Arquivo medido na máquina: `stat -c '%s'` e `sha256sum`.
- `planejamento-1.0.0/wyd-client/registro-do-cliente.md` e a página do manifesto do launcher.

## Passos

1. **Meça antes de escrever**: bytes e `sha256` do arquivo real; manifesto é declaração, não estimativa.
2. **Registre a linha** com caminho relativo, papel, origem, bytes e hash — na ordem da tabela do
   documento dono.
3. **Não versione binário**: o que entra no git é o manifesto e a medição, nunca o `.exe`/`.zip`.
4. **Verifique pelo código**: o launcher compara tamanho e hash; teste com arquivo alterado deve falhar.
5. **Build Windows-only**: `cargo test --manifest-path crates/wyd-launcher/Cargo.toml` — a matriz do CI
   roda esse crate só em `windows-latest`, e `cargo test -p wyd-launcher` não resolve sem workspace.

## Como verificar

- `sha256sum` do arquivo igual ao manifesto.
- Teste do verificador passa para arquivo íntegro e falha para arquivo adulterado.
- CI `unit / wyd-launcher / windows-latest` verde.

## Quando parar e perguntar

- O patch muda a origem de download (CDN, hospedagem) — é decisão de operação.
- O arquivo do cliente muda de versão sem registro: pare e meça de novo.

## Armadilhas

- **Hash copiado de execução antiga.**
- **Colocar o binário no repositório** "só para o teste".
- **Manifesto sem `sha256`**, que torna a verificação decorativa.

# ADR-OPS-0001 — Workspace Cargo único, toolchain e versões

> Pergunta: "qual decisão estrutural esta ADR registra e como verificá-la?"
> Público: Arquitetura e desenvolvimento
> Dono: `wyd-ops` (com aprovação do dono do produto)
> Base: `dossie-d-05.md`; `plano-migracao-d-05.md`; `docs/arquitetura/adr/index.md`; `auditoria-de-organizacao.md` §3; `AGENTS.md` §5 e §8
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25). Decisão D-05 = A aprovada pelo dono do produto em 2026-11-25; a execução do plano ainda não ocorreu.

## Contexto

O repositório tem **9** diretórios em `crates/`, mas só **1** `Cargo.toml` real (`crates/wyd-db`) e **8** diretórios com `.gitkeep`. Não há `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` ou `Makefile` na raiz. O `code.yml` usa **6** comandos `--manifest-path crates/wyd-db/Cargo.toml`, e o job `security` escolhe o primeiro manifesto/lock com `find ... -print -quit`. Duas imagens externas (`python:3.15-slim`, `postgres:17`) e três tags GHCR não estão fixadas por digest.

Sem workspace, o segundo crate repetiria lock, toolchain e configuração; com vários locks, o `security` poderia auditar apenas o primeiro. A decisão foi tomada com a árvore nesse estado (`V`, 2026-11-25).

## Decisão

Adotar **um workspace Cargo único na raiz**, com membros explícitos, **um** `Cargo.lock`, `rust-toolchain.toml` com canal exato, `Makefile` com alvos únicos, ações por SHA e imagens por digest.

## Consequências

- Positivas: um único lock/toolchain; o segundo crate nasce sem repetir configuração; o `security` deixa de escolher um lock arbitrário; comandos locais e de CI convergem.
- Negativas/custos: criar workspace, mover/gerar o lock, criar toolchain e Makefile, atualizar **6** comandos e os gatilhos do `code.yml`, resolver digests de **2** imagens externas.
- O que muda nos outros sistemas: todos os crates Rust passam a usar o workspace; `wyd-db` continua o único membro real até outro crate ganhar manifesto. O gate de documentação não é afetado, porque é Python e não lê Cargo.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| B — crates independentes | pode chegar a **9** locks; o `security` audita só o primeiro; cada crate pode divergir em toolchain. |
| C — híbrido | comando local pode não provar o mesmo que o CI; a dívida de versões permanece parcial. |

## Como verificar

```bash
cargo metadata --no-deps --format-version 1
test -f Cargo.toml && test -f Cargo.lock && test -f rust-toolchain.toml && test -f Makefile
find crates -name Cargo.lock
docker compose --profile gate run --rm gate
```

A decisão só está em vigor quando os comandos acima passam e o `code.yml` usa o workspace; hoje o plano está aprovado, mas **não executado**.

## Referências

- Issue: não criada.
- Lacuna fechada: D-05 (decisão de foco, não lacuna de sistema).
- Achado da auditoria: `wyd-ops/auditoria-de-organizacao.md` §3.
- Seção de especificação: `wyd-1.0.0.md` §8.7.
- Documentos donos: `dossie-d-05.md`; `plano-migracao-d-05.md`; `docs/prioridade-do-projeto.md` §0.1.
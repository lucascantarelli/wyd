# Plano de migração D-05 — Workspace Cargo único

> Pergunta: "como migrar para o workspace Cargo único aprovado em D-05 = A, com toolchain, Makefile, lock único e imagens por digest?"
> Público: `wyd-ops`, arquitetura e equipe Rust
> Dono: `wyd-ops` (execução); decisão D-05 é do dono do produto
> Base: `dossie-d-05.md`; `prioridade-do-projeto.md` §0.1; `wyd-1.0.0.md` §8.7
> Revisado em: 2026-11-25

**Estado: plano derivado da decisão D-05 = A (2026-11-25); ainda NÃO executado.** Nenhum `Cargo.toml`,
`rust-toolchain.toml`, `Makefile`, lock na raiz, alteração de CI ou imagem foi criado nesta rodada.
A execução exige um pedido explícito de implementação e começa pelo passo 1.

## 1. Decisão que este plano executa

**D-05 = A:** workspace Cargo único na raiz, com membros explícitos, `rust-toolchain.toml`, `Makefile`,
**um** `Cargo.lock` na raiz, ações por SHA e imagens por digest. O `wyd-db` continua sendo o foco de
execução (D-01); este plano não muda o passo 2 (`migrations/serie-e-regras.md`) nem cria código de
produto.

## 2. Pré-condições e limite

- D-05 = A registrada em `prioridade-do-projeto.md` e no `dossie-d-05.md`.
- O crate `crates/wyd-db` é o único membro real hoje; os outros 8 diretórios só têm `.gitkeep`.
- Este plano não aceita página do `wyd-db`, não abre issue, não cria crate e não muda o gate de
  documentação.
- A execução deve preservar `planejamento-1.0.0/` como documentação: nenhum `.rs`, `.sql`, `.py` ou
  script executável será criado lá.

## 3. Passos

| # | Passo | Ação | DoD |
|---:|---|---|---|
| **0** | Baseline | Registrar `rustc --version`, `cargo --version`, `git status`, os 9 diretórios de crate, o único `Cargo.lock` e os comandos atuais do `code.yml` no registro de execução. | Baselines anexados com data; nenhuma alteração de arquivo. |
| **1** | Workspace | Criar `Cargo.toml` na raiz com `[workspace]`, membros explícitos (`crates/wyd-db` primeiro) e `resolver = "2"`. Não usar `crates/*` enquanto os 8 diretórios não tiverem manifesto. | `cargo metadata --no-deps` lista **1** membro; `cargo test --workspace` acha o crate existente. |
| **2** | Lock único | Gerar o lock na raiz a partir do workspace; remover o lock do crate depois de comparar as entradas e verificar que não há dependência externa inesperada. | Existe **1** `Cargo.lock` na raiz; `find crates -name Cargo.lock` vazio; `cargo audit --file Cargo.lock` funciona. |
| **3** | Toolchain | Criar `rust-toolchain.toml` com canal exato (baseline medido: **1.100.0**) e componentes `rustfmt`, `clippy` e `llvm-tools-preview`. Atualização de versão é PR próprio, com data e gate. | `rustup show active-toolchain` usa o canal do arquivo; `cargo fmt`/`clippy` rodam sem instalação manual no CI. |
| **4** | Makefile | Criar alvos `fmt`, `lint`, `test`, `audit`, `deny` e `gate`, todos com comandos de workspace e sem segredo. | `make fmt`, `make lint`, `make test`, `make audit`, `make deny` e `make gate` executam o mesmo comando documentado. |
| **5** | CI | Atualizar `.github/workflows/code.yml`: incluir `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` e `Makefile` em `paths`; trocar os **6** `--manifest-path crates/wyd-db/Cargo.toml` por comandos de workspace; fazer `security` usar o manifesto/lock da raiz, sem `find ... -print -quit`. | Workflow dispara por mudança de configuração da raiz; fmt, clippy, testes, cobertura, audit e deny rodam no workspace. |
| **6** | Imagens por digest | Resolver os digests de `python:3.15-slim` e `postgres:17` no registro e fixá-los no Dockerfile/`code.yml`; para o GHCR do site, publicar o digest da imagem e usar digest no caminho de deploy, mantendo tags só como conveniência local. Não inventar digest. | Nenhum `FROM`/`image:` de runtime fica por tag; o digest usado está registrado no plano de execução e no comentário da imagem. |
| **7** | Verificação | Rodar os comandos de §4 e o gate de documentação; comparar contagens com 309 páginas / 307 fontes / 141 atos / 79 aceitas / 64 células / 0 Q-BLK. | Todos os comandos verdes; contagens iguais às publicadas; `git diff --check` limpo. |
| **8** | Rollback | Se o workspace falhar, reverter o `Cargo.toml`/lock/toolchain/Makefile, restaurar o lock do crate e reverter o `code.yml`; nenhum dado é migrado. | `git status` limpo e `cargo test` do crate volta ao estado anterior. |

## 4. Comandos de verificação

```bash
# workspace e lock
cargo metadata --no-deps --format-version 1
find crates -name Cargo.lock
test -f Cargo.lock && test -f Cargo.toml && test -f rust-toolchain.toml && test -f Makefile

# qualidade e segurança
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo audit --file Cargo.lock
cargo deny --config deny.toml check advisories bans licenses sources

# gate de documentação (não compila Rust)
docker compose --profile gate run --rm gate

# higiene
git diff --check
```

## 5. DoD

- **1** workspace na raiz, **1** `Cargo.lock`, **1** `rust-toolchain.toml`, **1** `Makefile`.
- **9** diretórios de crate continuam existindo; **1** membro real no workspace, **8** vazios não
  entram como membros até ganharem manifesto.
- `.github/workflows/code.yml` cobre fmt, clippy, testes, cobertura, audit e deny no workspace e
  dispara quando a configuração da raiz muda.
- `python:3.15-slim` e `postgres:17` fixados por digest; GHCR com digest no caminho de deploy.
- Gate de documentação continua em **309 páginas / 307 fontes / 141 atos / 79 aceitas / 64 células /
  0 Q-BLK**.
- Nenhum código de produto, issue, commit ou PR criado por este plano.

## 6. Riscos e mitigação

| Risco | Mitigação |
|---|---|
| O runner baixar uma toolchain diferente | `rust-toolchain.toml` com canal exato; CI usa o arquivo, não `stable` implícito. |
| O `security` continuar auditando só o primeiro lock | Remover `find ... -print -quit` e apontar para `Cargo.lock` da raiz; verificar no job. |
| O lock da raiz misturar dependências futuras | O crate atual não tem dependências; revisar o diff do lock antes de remover o lock do crate. |
| Imagem por tag continuar em algum caminho | Auditar `FROM`/`image:` no Dockerfile, Compose e workflow; digest resolvido no registro, nunca inventado. |
| O workspace quebrar o gate de documentação | O gate é Python e não lê Cargo; rodar o gate depois da migração e comparar as contagens. |

## 7. O que este plano não faz

- Não implementa agora; não cria `Cargo.toml`, `Makefile`, `rust-toolchain.toml` nem altera `code.yml`.
- Não muda D-01: o próximo trabalho de produto continua `migrations/serie-e-regras.md`.
- Não abre os 8 crates vazios; eles entram quando tiverem página aceita e manifesto próprio.
- Não fecha as 6 lacunas abertas + 1 proposta; elas dependem de captura, direitos ou medição.

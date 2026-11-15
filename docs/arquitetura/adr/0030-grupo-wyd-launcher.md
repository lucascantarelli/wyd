# ADR-0030 — Grupo `crates/wyd-launcher/` com `core/` e `ui/`

**Status:** aceita (implementada) — 2026-11-15
**Substitui parcialmente:** ADR-0002 (árvore da §9.1, antes `apps/launcher-ui/`)

## Contexto

O launcher estava partido em dois lugares do repositório: o núcleo em
`crates/wyd-launcher/` e a janela Tauri em `apps/launcher-ui/`. Duas consequências
práticas:

1. `apps/launcher-ui` era **membro do workspace** fora de `crates/`, o que mantinha uma
   exceção na regra "crate mora em `crates/`" — e os Dockerfiles precisavam de um `COPY`
   extra só para o manifesto resolver.
2. `apps/` passou a misturar naturezas diferentes: uma crate Rust (janela) e ferramentas
   Node (`apps/tools`), com políticas de teste, gate e dependência distintas.

A leitura desejada é "cada pacote na sua pasta, e a família do launcher agrupada".

## Decisão

1. `crates/wyd-launcher/` passa a ser **grupo de pacotes**, sem `Cargo.toml` próprio:

   ```
   crates/wyd-launcher/
   ├── core/   # lib `wyd_launcher` + bin `wyd-launcher`  (era crates/wyd-launcher/)
   └── ui/     # bin `WYDLauncher`, Tauri v2              (era apps/launcher-ui/)
       └── web/                                            (era ui/)
   ```

2. **Nomes de pacote não mudam** (`wyd-launcher`, `wyd-launcher-ui`): `cargo -p`,
   `--bin WYDLauncher`, `Cargo.lock`, `release.yml` e a matriz do CI seguem válidos.
3. O grupo tem **um nível só**: `crates/<grupo>/<crate>`. Grupo dentro de grupo não é
   árvore aprovada.
4. `apps/` continua existindo para apps de ferramenta (`apps/tools`), nunca para crate Rust.
5. O gate `tools/dev/check-docs-consistency.mjs` passa a identificar crate pelo caminho
   relativo a `crates/` (`wyd-core`, `wyd-launcher/core`) e a varrer dois níveis; a frase
   de contagem do `README.md`, a tabela do `AGENTS.md` e as visões de pasta acompanham na
   mesma entrega.

## Consequências

- `include_str!` de `core/src/identidade.rs` ganha um nível (`../../../../docs/...`), e o
  comentário do `docker/Dockerfile.launcher` (que documenta essa profundidade) acompanha.
- Os três Dockerfiles deixam de copiar `apps/launcher-ui`: o `COPY crates` já inclui a
  janela, porque ela é membro do workspace sob `crates/`.
- A janela deixa de ter `ui/ui` como caminho de frontend (`tauri.conf.json` →
  `frontendDist: "web"`).
- O gate de estrutura **precisou** de suporte a grupo; sem ele, `make check` falharia com
  "membro do workspace sem Cargo.toml". A regra de grupo fica escrita em
  `estrutura-e-padroes.md` §9/§9.1 para a próxima família não redescobrir isso.
- `docs/audit/*` são snapshots históricos: não são reescritos (continuam dizendo
  `apps/launcher-ui`, como estava quando foram medidos).

## Verificação

- `cargo build -p wyd-launcher -p wyd-launcher-ui`
- `node tools/dev/check-docs-consistency.mjs`
- `make check` (gate completo, incluindo clippy com o pacote Tauri no workspace)

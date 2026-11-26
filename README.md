<div align="center">

# ⚔️ WYD 1.0.0

**A reimplementação do MMORPG WYD2 em Rust, PostgreSQL e Next.js — planejada sistema por sistema antes do primeiro arquivo de código.**

<img src="https://img.shields.io/badge/status-planejamento-f59e0b" alt="Planejamento">
<img src="https://img.shields.io/badge/vers%C3%A3o-1.0.0-7c3aed" alt="Versão 1.0.0">
<img src="https://img.shields.io/badge/Rust-stable-000?logo=rust&logoColor=white" alt="Rust">
<img src="https://img.shields.io/badge/PostgreSQL-17-4169E1?logo=postgresql&logoColor=white" alt="PostgreSQL 17">
<img src="https://img.shields.io/badge/MkDocs-2.0-526CFE?logo=materialformkdocs&logoColor=white" alt="MkDocs 2.0">
<img src="https://img.shields.io/badge/licen%C3%A7a-GPL--2.0-blue" alt="GPL-2.0">

[🌐 Site publicado (GitHub Pages)](https://lucascantarelli.github.io/wyd/) · [🧭 Panorama](planejamento-1.0.0/wyd-1.0.0.md) · [🤖 Regras para agentes](AGENTS.md) · [🗄️ Piloto `wyd-db`](planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md)

</div>

---

## O que é este repositório

O alvo é a **`1.0.0`**: um servidor de WYD2 com o protocolo CPSock do cliente original, motor em
Rust, persistência em PostgreSQL 17, portal em Next.js, launcher Tauri e documentação publicada.

O repositório está em **planejamento**, com o primeiro slice do `wyd-db` já em código. Não há
workspace Cargo (`Cargo.toml` na raiz), runner de migrations nem serviço no ar: o que existe é o plano de cada
sistema, medido e com dono, e o código que nasceu sob páginas `aceitas`. Essa é a decisão do projeto —
planejar antes de codar — e o `deprecated/` é o contraexemplo que ela evita.

| Existe | Não existe ainda |
|---|---|
| `planejamento-1.0.0/` com o panorama e os sistemas já abertos | workspace Cargo (`Cargo.toml` na raiz); existem `rust-toolchain.toml` e `Makefile` |
| site MkDocs 2.0 montado em `wyd-docs/` | runner de migrations e serviço no ar |
| `crates/wyd-db` com `Cargo.toml`, `src/lib.rs`, `src/dsn.rs` e `tests/sistema/sistema_pg.rs`; testes unitários e de schema (contagem no panorama §1) | código de produção fora do slice `dsn`/schema |
| série de `.sql` em `crates/wyd-db/migrations/`, aplicada do zero num PG17 em 2026-11-24 (contagem no panorama §1) | os seeds, bloqueados por curadoria de dado |
| plano do `wyd-protocol` com lacunas e teto de páginas registrados | goldens de fio e payloads implementados |
| planos do `wyd-admin` e do `wyd-portal` e propostas do `wyd-ops` | código de admin, portal e ops |
| `docs.yml` faz build/verificação e publica Pages em `develop`; `code.yml` roda fmt, clippy, testes e cobertura informativa; `security.yml` roda audit/deny, dependency-review e actionlint; `code-stable.yml` é sinal diário não bloqueante | gate de cobertura de 85% |

Números medidos em **2026-11-24** vivem em [`planejamento-1.0.0/wyd-1.0.0.md`](planejamento-1.0.0/wyd-1.0.0.md) §1,
com comando e data. Este README não replica contagem: aponta o dono.

O site oficial de documentação está publicado em [GitHub Pages](https://lucascantarelli.github.io/wyd/).
Em **2026-11-19**, o PR #44 corrigiu o carregamento do bundle browser do Highlight.js 11.12.0; o deploy automático terminou com sucesso e a verificação no site público encontrou o bundle HTTP 200, sem `require is not defined` nem o erro em `highlightElement`. Isso valida o site de documentação, não significa que o jogo esteja implementado.

## Por onde começar

| Se você é… | Leia |
|---|---|
| **novo no projeto** | [`planejamento-1.0.0/wyd-1.0.0.md`](planejamento-1.0.0/wyd-1.0.0.md) — estado medido, divergências e fases |
| **agente de IA ou contribuidor** | [`AGENTS.md`](AGENTS.md) — hierarquia de fontes, invariantes e fluxo |
| **vai trabalhar no banco** | [`wyd-db-1.0.0.md`](planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md), [`lacunas.md`](planejamento-1.0.0/wyd-db/lacunas.md) e [`convencoes.md`](planejamento-1.0.0/wyd-db/docs/convencoes.md) |
| **vai trabalhar no fio** | [`wyd-protocol-1.0.0.md`](planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md) e [`lacunas.md`](planejamento-1.0.0/wyd-protocol/lacunas.md) |
| **vai mexer no site** | [`wyd-docs-1.0.0.md`](planejamento-1.0.0/wyd-docs/wyd-docs-1.0.0.md) e [`guia de contribuição`](wyd-docs/docs/contribuir/index.md) |
| **vai trabalhar no admin ou no portal** | [`wyd-admin-1.0.0.md`](planejamento-1.0.0/wyd-admin/wyd-admin-1.0.0.md) e [`wyd-portal-1.0.0.md`](planejamento-1.0.0/wyd-portal/wyd-portal-1.0.0.md) |

## Estrutura

```text
planejamento-1.0.0/      # fonte de verdade nível 1: 13 sistemas, um por pasta
├── wyd-1.0.0.md         # panorama medido: estado, fases, mapa (§7) e layout (§8)
└── <sistema>/           # por sistema: dashboard, lacunas.md e docs/
wyd-docs/                # projeto do site MkDocs 2.0 (montagem, tema, gate e verificação)
wyd-portal/              # Next.js: app em src/app, módulos do plano, tests/ e public/
wyd-client/              # cliente real: files/ ignorado; tools/ reservado para a CLI (LCLI-04)
wyd-ops/                 # 4 Dockerfiles, scripts/ (ci/) e deny.toml
crates/wyd-db/           # crate com comportamento: Cargo.toml, src/, tests/ e 28 .sql
crates/                  # 10 crates com esqueleto (Cargo.toml, lock e src/lib.rs); sem workspace
.vscode/                 # settings, launch, tasks e extensions para a IDE
docker-compose.yml       # serviço docs: consome a imagem publicada pelo CI
docker-compose.override.yml  # carregado automaticamente: serviço gate (perfil)
docker-compose.dev.yml   # opt-in (-f): build da árvore, tag :dev e perfil stack
.github/                 # workflows e actions compostas (dono: wyd-ops)
deprecated/              # referência legada, ignorada pelo git: nunca é fonte
```

Dono de cada artefato, pasta reservada, regra por tipo e as exceções de raiz estão na página
autoritativa [`estrutura-de-pastas-e-modulos.md`](planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md)
§2 e §2.4; a verificação é `make layout`. A árvore acima é só o mapa visual da raiz.

## Documentação

**Sem arquivo no repositório:** `mkdocs.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

O site é gerado de `planejamento-1.0.0/*/docs/` e roda em **MkDocs 2.0** com **Python 3.15**.
O MkDocs 2.0 é pré-release (`2.0.dev6`), sem `mkdocs.yml`, plugin, tema embutido nem modo estrito —
por isso o projeto tem tema próprio, montagem em `preparar.py` e gate no pacote `gate/`
(`python -m gate`, com resumo curto e `--json`; contrato no ADR-OPS-0010).

Nada é montado à mão: a imagem é construída e publicada pelo CI
(`.github/workflows/docs-image.yml`) e o `docker compose` a consome.

```bash
docker compose pull docs                     # baixa a imagem publicada
docker compose up -d --no-build docs         # serve a imagem em http://127.0.0.1:8000
docker compose --profile gate run --rm gate  # monta, builda e roda a cadeia de verificação
make gate-fast                               # ciclo curto nativo (sem Docker/MkDocs)
```

Para construir da árvore de trabalho, sem tocar na imagem publicada:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d docs --build
make rollback TAG=<sha>   # puxa a tag publicada e sobe com --no-build
```

> No Windows o build do `mkdocs 2.0.dev6` no host falha (`PosixPath`): use o container, como o CI.
> Os três passos equivalentes no host e a evidência de verificação estão em
> `planejamento-1.0.0/wyd-ops/plano-imagem-e-compose-do-site.md` §5.

## Como o trabalho anda

1. **Medir** o que existe antes de afirmar qualquer coisa.
2. **Planejar** no sistema dono, com requisito, evidência (`V`/`H`/`P`) e lacuna com dono.
3. **Validar** pelo gate do sistema: cabeçalho, link que resolve, sem placeholder, sem segredo,
   revisão de quem não escreveu.
4. **Implementar** só depois do gate de planejamento: uma issue por PR, teste antes, cobertura mínima
   definida em [`AGENTS.md`](AGENTS.md) §8, documentação atualizada na mesma entrega.

Mudança de arquitetura, escopo ou nomenclatura é **proposta**, não commit: o time aprova, a decisão
vira ADR com "como verificar" e a lacuna fecha com data.

## Próximos passos

Ordem de planejamento e gates conforme [`planejamento-1.0.0/wyd-1.0.0.md` §4](planejamento-1.0.0/wyd-1.0.0.md#4-plano-de-fases):

1. **Concluir a Fase 2 (`wyd-db`):** fechar lacunas com seus donos e avançar o DDL e as páginas de migration sem antecipar implementação.
2. **Avançar a Fase 3:** completar o planejamento do `wyd-protocol` como segundo piloto e, em seguida, planejar os demais sistemas com fronteiras e contratos explícitos.
3. **Continuar o plano de `wyd-docs`:** a publicação no GitHub Pages e a validação pública do PR #44 já estão registradas no plano dono. Permanecem pendentes o padrão editorial (Fase 3) e a robustez (Fase 4); `LPDOC-01` continua aberta até aprovação explícita do dono.
4. **Depois dos gates de planejamento:** iniciar implementação e substituir o workflow principal limitado por CI de código com build, testes e cobertura.

**Ação operacional independente do Highlight.js:** revisar a configuração CodeQL Default setup e retirar Rust da análise enquanto não houver fonte Rust versionada para analisar. Isso elimina o job `Analyze (rust)` sem alterar o PR #44 nem desativar a análise das linguagens com código no repositório; reavaliar quando Rust entrar no Git.

`deprecated/` guarda a implementação anterior e **não é versionado**; a contagem está em
[`planejamento-1.0.0/wyd-1.0.0.md`](planejamento-1.0.0/wyd-1.0.0.md) §1.4. Ele serve
para entender comportamento e defeito; nada dele entra por cópia — se o dado não foi reverificado no
planejamento novo, é hipótese.

Distribuído sob **GNU GPL v2.0** — veja [`LICENSE`](LICENSE). Binários e assets do cliente WYD são
propriedade dos seus donos e **não** são distribuídos aqui; `wyd-client/files/` é ignorado pelo git.

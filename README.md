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

O repositório está em **planejamento**. Não há workspace Rust, migration executável nem serviço no
ar: o que existe é o plano de cada sistema, medido e com dono. Essa é a decisão do projeto — planejar
antes de codar — e o `deprecated/` é o contraexemplo que ela evita.

| Existe | Não existe ainda |
|---|---|
| `planejamento-1.0.0/` com o panorama e 3 sistemas abertos | `Cargo.toml`, `rust-toolchain.toml`, `Makefile` |
| site MkDocs 2.0 montado em `wyd-docs/` | código de produção em `crates/` (só `.gitkeep`) |
| plano do `wyd-db` com 15 páginas, 2 ADRs e 29 lacunas | migrations da `1.0.0` (a série é planejada, não escrita) |
| plano do `wyd-protocol` com 20 lacunas e teto de 131 páginas | goldens de fio e payloads implementados |
| workflow principal `.github/workflows/full-workflow.yml` limitado ao `Get Info`; o workflow `docs.yml` faz build/verificação e publica Pages | CI de código com build, teste e cobertura |

Números medidos em **2026-11-19** vivem em [`planejamento-1.0.0/wyd-1.0.0.md`](planejamento-1.0.0/wyd-1.0.0.md) §1,
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

## Estrutura

```text
planejamento-1.0.0/      # fonte de verdade nível 1: um sistema por pasta
├── wyd-1.0.0.md         # panorama medido, divergências e fases
├── wyd-db/              # piloto: PostgreSQL, migrations, portas, filas
├── wyd-protocol/        # segundo piloto: CPSock, opcodes, payloads, goldens
├── wyd-docs/            # site: montagem, tema, gate e publicação
└── <10 outros sistemas> # só .gitkeep: não planejado, não "livre para inventar"
wyd-docs/                # projeto do site MkDocs 2.0 (tema, montagem, verificação)
wyd-portal/              # portal Next.js (ainda sem código)
wyd-client/              # patches e config do cliente (files/ ignorado)
crates/                  # 9 pastas reservadas, só .gitkeep
deprecated/              # referência legada, ignorada pelo git: nunca é fonte
```

## Documentação

O site é gerado de `planejamento-1.0.0/*/docs/` e roda em **MkDocs 2.0** com **Python 3.15**.
O MkDocs 2.0 é pré-release (`2.0.dev6`), sem `mkdocs.yml`, plugin, tema embutido nem modo estrito —
por isso o projeto tem tema próprio, montagem em `preparar.py` e gate em `verificar.py`.

```bash
cd wyd-docs
py -3.15 -m venv .venv                       # Linux/macOS: python3 -m venv .venv
.venv/Scripts/python.exe -m pip install -r requirements.txt
.venv/Scripts/python.exe preparar.py
.venv/Scripts/mkdocs.exe build --dir .build/docs --output site
.venv/Scripts/python.exe verificar.py site
```

> No Windows o build do `mkdocs 2.0.dev6` falha (`PosixPath`): use o container
> `python:3.15-slim`, como o CI. O comando completo está no plano do `wyd-docs` §7.1.

## Como o trabalho anda

1. **Medir** o que existe antes de afirmar qualquer coisa.
2. **Planejar** no sistema dono, com requisito, evidência (`V`/`H`/`P`) e lacuna com dono.
3. **Validar** pelo gate do sistema: cabeçalho, link que resolve, sem placeholder, sem segredo,
   revisão de quem não escreveu.
4. **Implementar** só depois do gate de planejamento: uma issue por PR, teste antes, cobertura mínima
   de 85%, documentação atualizada na mesma entrega.

Mudança de arquitetura, escopo ou nomenclatura é **proposta**, não commit: o time aprova, a decisão
vira ADR com "como verificar" e a lacuna fecha com data.

## Próximos passos

Ordem de planejamento e gates conforme [`planejamento-1.0.0/wyd-1.0.0.md` §4](planejamento-1.0.0/wyd-1.0.0.md#4-plano-de-fases):

1. **Concluir a Fase 2 (`wyd-db`):** fechar lacunas com seus donos e avançar o DDL e as páginas de migration sem antecipar implementação.
2. **Avançar a Fase 3:** completar o planejamento do `wyd-protocol` como segundo piloto e, em seguida, planejar os demais sistemas com fronteiras e contratos explícitos.
3. **Continuar o plano de `wyd-docs`:** a publicação no GitHub Pages e a validação pública do PR #44 já estão registradas no plano dono. Permanecem pendentes o padrão editorial (Fase 3) e a robustez (Fase 4); `LPDOC-01` continua aberta até aprovação explícita do dono.
4. **Depois dos gates de planejamento:** iniciar implementação e substituir o workflow principal limitado por CI de código com build, testes e cobertura.

**Ação operacional independente do Highlight.js:** revisar a configuração CodeQL Default setup e retirar Rust da análise enquanto não houver fonte Rust versionada para analisar. Isso elimina o job `Analyze (rust)` sem alterar o PR #44 nem desativar a análise das linguagens com código no repositório; reavaliar quando Rust entrar no Git.

`deprecated/` guarda a implementação anterior (375 arquivos Rust) e **não é versionado**. Ele serve
para entender comportamento e defeito; nada dele entra por cópia — se o dado não foi reverificado no
planejamento novo, é hipótese.

Distribuído sob **GNU GPL v2.0** — veja [`LICENSE`](LICENSE). Binários e assets do cliente WYD são
propriedade dos seus donos e **não** são distribuídos aqui; `wyd-client/files/` é ignorado pelo git.

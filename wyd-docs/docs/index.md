# WYD 1.0.0 — documentação oficial

**A reimplementação do WYD2 em Rust, PostgreSQL e Next.js.** Este site publica o planejamento
oficial da `1.0.0`, sistema por sistema, do jeito que o sistema dono escreveu.

> [!NOTE] Estado do projeto
> Planejamento, sem código de produção. Nenhuma página publicada aqui é promessa de release: o
> estado de cada documento está declarado no cabeçalho dele (`proposto`, `aceito`).

## Sistemas

| Sistema | Responsabilidade | Planejamento |
|---|---|---|
| [`wyd-protocol`](sistemas/wyd-protocol/index.md) | pacotes CPSock, opcodes, offsets, goldens | aberto |
| `wyd-net` | transporte async, sessão, rate-limit, hardening | não iniciado |
| `wyd-core` | tick, ECS, combate, cena, IA | não iniciado |
| `wyd-server` | composição do servidor; um processo por canal | não iniciado |
| `wyd-sim` | fórmulas, curvas e simulações determinísticas | não iniciado |
| [`wyd-db`](sistemas/wyd-db/index.md) | PostgreSQL 17, migrations, repositórios, filas | piloto |
| `wyd-admin` | administração, HTTP/JSON interno, staff | planejado |
| `wyd-portal` | portal Next.js: site, login, cadastro, BFF | planejado |
| `wyd-launcher` | launcher Tauri: patch, manifesto, serverlist/itemlist | não iniciado |
| `wyd-client` | patches e arquivos de configuração do cliente | não iniciado |
| `wyd-bots` | bots de QA, carga e convivência | não iniciado |
| [`wyd-docs`](sistemas/wyd-docs/index.md) | este site: montagem, publicação e padrão editorial | aberto |
| `wyd-ops` | CI, imagens, deploy, ambientes, monitoramento | proposta |

## Como este site é montado

O site roda em **MkDocs 2.0** (`2.0.dev6`) com **Python 3.15**. O MkDocs 2.0 não tem `mkdocs.yml`,
plugin, tema embutido nem configuração de Markdown: ele converte um diretório de Markdown com um
template Jinja e nada mais. Por isso o projeto tem quatro peças:

| Peça | O que faz |
|---|---|
| `preparar.py` | copia `planejamento-1.0.0/<sistema>/docs/` para `.build/docs/sistemas/<sistema>/`, aplica a ponte de sintaxe e gera a navegação |
| `tema/base.html` | o tema inteiro: topo, navegação, sumário, conteúdo e rodapé |
| `tema/estilo.css` e `tema/site.js` | apresentação, tema claro e escuro, cópia de código, destaque e avisos |
| `verificar.py` | o gate: link interno, âncora, navegação e título de toda página |

Nenhuma página é editada dentro de `wyd-docs/`: a fonte é o diretório do sistema dono.

## Rodar localmente

### Windows

    ```bash
    cd wyd-docs
    py -3.15 -m venv .venv
    .venv/Scripts/python.exe -m pip install -r requirements.txt
    .venv/Scripts/python.exe preparar.py
    .venv/Scripts/mkdocs.exe build --dir .build/docs --output site
    .venv/Scripts/python.exe verificar.py site
    .venv/Scripts/mkdocs.exe serve --dir .build/docs
    ```

### Linux e macOS

    ```bash
    cd wyd-docs
    python3 -m venv .venv
    .venv/bin/python -m pip install -r requirements.txt
    .venv/bin/python preparar.py
    .venv/bin/mkdocs build --dir .build/docs --output site
    .venv/bin/python verificar.py site
    .venv/bin/mkdocs serve --dir .build/docs
    ```

## O que este site não publica

Plano diretor, `lacunas.md`, `src/`, `tests/`, `prompts/` e `skills/` de cada sistema continuam
internos. `deprecated/` e os arquivos do cliente também ficam fora: são referência e binário
proprietário.

O ponto de entrada para quem vai escrever é o [guia de contribuição](contribuir/index.md). As regras
do repositório para agentes e pessoas estão em `AGENTS.md`.

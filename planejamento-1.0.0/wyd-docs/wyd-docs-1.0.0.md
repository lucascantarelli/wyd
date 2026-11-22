# wyd-docs 1.0.0 — plano diretor do site de documentação

> **Estado: RASCUNHO PARA APROVAÇÃO (2026-11-19).** Este documento define **o que** o site oficial
> publica, **de onde** cada página vem, **como** o site é montado e **qual** é o gate de publicação.
>
> Decisão do time em **2026-11-19**: a documentação oficial é publicada por este sistema, e tudo o
> que é do site (tema, montagem, verificação, configuração) mora em `wyd-docs/`, na raiz do
> repositório. Nenhum outro sistema ganha configuração de site própria.
>
> **Stack decidida pelo time:** **MkDocs 2.0** com **Python 3.15**. Medido em **2026-11-19**:
> `mkdocs 2.0.dev6` é a única versão 2.0 publicada (pré-release de 2026-09-15), roda em Python 3.15
> e **não tem** `mkdocs.yml`, plugin, tema embutido, configuração de Markdown, modo estrito nem
> suporte a Windows (§2). O site daquela entrega foi a primeira montagem real: **30 páginas** (histórico; o número atual está em §9.1), build e
> verificação executados (§7.1).
>
> **Evidência de publicação (`V`, 2026-11-19):** o PR [#44](https://github.com/lucascantarelli/wyd/pull/44)
> foi mesclado no commit `0d76585821583d5ef60e790fa44fb65572576d4b`; o workflow
> [docs](https://github.com/lucascantarelli/wyd/actions/runs/44498880453) concluiu o build/verificação
> e a publicação no Pages com sucesso. Após recarga do site público
> <https://lucascantarelli.github.io/wyd/>, o bundle browser do Highlight.js 11.12.0 respondeu HTTP
> 200 e o console ficou sem `require is not defined` e sem o erro em `highlightElement`.
> Decisão do dono (`V`, 2026-11-19): GitHub Pages é a hospedagem canônica, com publicação
> contínua nele. Aquela evidência, por si só, não fechava `LPDOC-01`; o teste real de rollback em
> 2026-11-19 (§7.3) é a prova que a fechou.

---

## 0. Como ler

| Se você quer… | Vá para |
|---|---|
| entender o que o site é e o que não publica | §1 |
| saber o que o MkDocs 2.0 entrega e o que ele não tem | §2 |
| saber de onde vem cada página | §3 |
| ver as regras de navegação, URL e título | §4 |
| ver a árvore do projeto | §5 |
| ver o mapeamento dos recursos usados antes × agora | §6 |
| saber o gate de build e de publicação | §7 |
| conhecer os identificadores | §8 |
| ver a ordem de produção | §9 |
| saber o que depende de decisão do time | §10 |

---

## 1. Objetivo, escopo e limites

### 1.1 Objetivo

1. **Uma porta de entrada** para toda a documentação oficial da `1.0.0`, por sistema.
2. **Fonte única**: a página é escrita em `planejamento-1.0.0/<sistema>/docs/` e o site a coleta;
   não existe cópia editável no projeto do site.
3. **Gate verificável**: link interno, âncora, navegação e título são checados por script, porque o
   MkDocs 2.0 não valida nada (§2.3).
4. **Publicação gratuita**: artefato estático em GitHub Pages por workflow de CI.
5. **Nenhum conteúdo interno vaza**: plano diretor, lacunas, `src/`, `tests/`, `prompts/` e `skills/`
   de cada sistema não são publicados.

### 1.2 Escopo

| Dentro | Fora |
|---|---|
| tema completo em `templates/base.html` + CSS + JS | conteúdo de sistema: dono é cada `<sistema>/docs/` |
| montagem das páginas em `.build/docs/` e navegação gerada | edição de página de sistema dentro de `wyd-docs/` |
| verificação de link, âncora, navegação e título | revisão de conteúdo: dono é o sistema |
| publicação em GitHub Pages por Actions | hospedagem paga, CDN paga, domínio próprio |

### 1.3 Contratos

| Sistema | Fornece ao site | Exige do site |
|---|---|---|
| os 13 sistemas | página em `planejamento-1.0.0/<sistema>/docs/**/*.md` com cabeçalho de 5 linhas | URL estável, navegação automática, gate que reprova link quebrado |
| `wyd-db` | estado das páginas de migration: `migrations-checklist.md` §2/§4; escopo da série: `plano-migrations.md` §3; resumo consolidado: `wyd-1.0.0.md` §1 | seção própria e ordem de leitura preservada |
| `wyd-protocol` | 2 páginas escritas + teto de 131 (`planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §4) | seção própria |
| `wyd-ops` | runner, Pages e deploy | workflow com permissão mínima |

---

## 2. O que o MkDocs 2.0 é (medido em 2026-11-19)

### 2.1 Fatos do pacote

| Fato | Medição |
|---|---|
| Versão publicada | `2.0.dev6`, de 2026-09-15; **não existe 2.0 final** (`pip index versions --pre mkdocs`) |
| Dependências | `click`, `httpx`, `jinja2`, `markdown` — sem PyYAML, porque não há arquivo de configuração |
| CLI | `mkdocs build --dir <entrada> --output <saída>` e `mkdocs serve --dir <entrada>` |
| Configuração | **nenhuma**: o CLI aborta com `Found mkdocs.yml config, but mkdocs 2.0 pre-release is installed` |
| Plugin, tema, hook | **não existem**; o pacote traz um único `theme/base.html` |
| Navegação | lista plana de todas as páginas, título = nome do arquivo, sem agrupamento |
| Markdown | `fenced_code`, `footnotes`, `tables`, `toc`, `relative_urls`, `short_codes`, `strike_thru`; sem admonition, sem abas, sem `attr_list`, sem `md_in_html` |
| Modo estrito | não existe; link relativo quebrado é mantido como está |
| Windows | **não roda**: `Page.__init__` instancia `pathlib.PosixPath`, que falha no Windows |

Fonte: inspeção de `mkdocs 2.0.dev6` instalado em 2026-11-19
(`mkdocs/mkdocs.py`, `mkdocs/theme/base.html`, `mkdocs/extensions/`) e documentação oficial em
<https://www.encode.io/mkdocs>.

### 2.2 O que isso obriga o projeto a fazer

| Ausência na 2.0 | O que o `wyd-docs` faz |
|---|---|
| sem configuração | `requirements.txt` fixa a versão; o comando de build é o contrato |
| sem coleta de vários diretórios | `preparar.py` monta `.build/docs/` a partir dos sistemas |
| sem navegação agrupada | `preparar.py` gera `templates/navegacao.html` com a árvore real |
| sem tema | `tema/base.html`, `estilo.css` e `site.js` são o tema |
| sem admonition/abas | material antigo convertido para GFM em 2026-11-19; página nova usa GFM (§6) |
| sem validação | `verificar.py` é o gate (§7.1) |
| sem Windows | build roda em Linux: container `python:3.15-slim` no local e runner Ubuntu no CI |

### 2.3 Consequência declarada

O MkDocs 2.0 é **pré-release sem data de lançamento**. O site fica preso a `2.0.dev6` até haver
versão estável; atualizar é decisão registrada (`LPDOC-11`), não `pip install -U` silencioso. A
contrapartida de usar a 2.0 agora é assumir tema, navegação e gate próprios — não é uma escolha
estética, é o que a ferramenta exige.

---

## 3. De onde vem cada página

| Origem | Vai para | Regra |
|---|---|---|
| `planejamento-1.0.0/<sistema>/docs/**/*.md` | `sistemas/<sistema>/<caminho relativo>` | caminho preservado; link relativo continua válido |
| `wyd-docs/docs/*.md` | raiz do site | página inicial e guia de contribuição |
| `planejamento-1.0.0/<sistema>/<plano>.md`, `lacunas.md`, `src/`, `tests/`, `prompts/`, `skills/` | **não publicado** | interno; citado por caminho em `code` |
| `deprecated/`, `wyd-client/files/` | **não publicado** | referência e cliente proprietário |

A montagem **copia** e converte sintaxe; nunca edita o arquivo do sistema. O caminho relativo é
preservado porque todo link entre páginas é relativo — e é assim que o `relative_urls` do MkDocs 2.0
resolve o destino.

---

## 4. Navegação, URL e título

### 4.1 Ordem

| Ordem | Seção | Origem |
|---:|---|---|
| 1 | **Início** | `wyd-docs/docs/index.md` |
| 2 | **Sistemas** | uma seção por sistema, na ordem de `AGENTS.md` §6 |
| 3 | **Contribuir** | `wyd-docs/docs/contribuir/index.md` |

Dentro do sistema: `index.md` vira "Visão geral", depois as pastas em ordem alfabética com rótulo
humano (`arquitetura` → "Arquitetura", `specs` → "Requisitos e especificações").

### 4.2 URL

O MkDocs 2.0 gera `index.md` → `/pasta/` e `nome.md` → `/pasta/nome/index.html` (com o radical em
minúsculas). URL é **contrato**: renomear página publicada exige redirect (`LPDOC-06`).

### 4.3 Título

Da primeira heading `#` do arquivo; sem H1, o nome do arquivo é humanizado e isso é defeito a
corrigir na página. O `verificar.py` reprova página sem título.

---

## 5. Árvore do projeto

Legenda: `●` implementado nesta entrega · `▲` a escrever.

```text
wyd-docs/
├── requirements.txt                  ● mkdocs==2.0.dev6
├── preparar.py                       ● monta .build/docs e a navegação
├── verificar.py                      ● gate de link, âncora, nav, título e sintaxe 1.x
├── redirects.tsv                     ● mapa origem → destino dos redirects
├── tema/
│   ├── base.html                     ● tema Jinja completo
│   ├── estilo.css                    ● claro/escuro, layout, avisos, tabelas, código
│   ├── site.js                       ● tema, filtro, avisos, cópia, destaque
│   └── mermaid.js                    ● renderização dos diagramas
├── docs/
│   ├── index.md                      ● página inicial
│   └── contribuir/index.md           ● guia do autor
├── .build/docs/                      ● gerado (fonte montada; fora do git)
└── site/                             ● gerado (artefato; fora do git)
```

```text
planejamento-1.0.0/wyd-docs/
├── wyd-docs-1.0.0.md                 ● este plano diretor
├── lacunas.md                        ● registro de lacuna e dono
├── docs/
│   ├── index.md                      ● porta de entrada do sistema
│   ├── convencoes.md                 ● padrão editorial do site
│   ├── arquitetura/
│   │   ├── index.md                  ▲
│   │   ├── montagem-do-site.md       ▲ preparar, build, verificar
│   │   ├── tema-e-recursos.md        ▲ o que a 2.0 não tem e como o tema cobre
│   │   ├── publicacao.md             ▲ Pages, permissões, rollback
│   │   └── dominio-e-url.md          ▲ URL como contrato
│   ├── planos/
│   │   ├── index.md                  ▲
│   │   ├── plano-de-documentacao.md  ▲
│   │   └── metricas.md               ● definição da métrica (valor em §9.1)
│   ├── roadmap/
│   │   ├── index.md                  ▲
│   │   ├── fases.md                  ▲
│   │   └── WYDDOC-001..WYDDOC-012.md ▲
│   ├── guia-autor/                   ●
│   ├── guia-operador/                ▲
│   ├── guia-desenvolvedor/           ▲
│   └── revisao.md                    ● critério de revisão
└── prompts/  skills/                 ▲ (4 + 4)
```

**Medido em 2026-11-19 (`find planejamento-1.0.0/wyd-docs/docs -name '*.md'` e `git ls-files wyd-docs`, sem `.gitkeep`):** 5 páginas em `docs/` do sistema e 8 arquivos implementados no projeto do site, fora de `docs/`.

---

## 6. Recursos: como eram no MkDocs 1.x e como são na 2.0

| Recurso usado até aqui | No MkDocs 1.x / Material | No MkDocs 2.0 | O que o site faz (ponte removida em 2026-11-19) |
|---|---|---|---|
| Configuração do site | `mkdocs.yml` (YAML) | **não existe**; o CLI aborta se encontrar o arquivo | comando fixo em §7.1 |
| Tema | `theme: material` com `features` | **não existe** tema embutido; só `theme/base.html` | tema próprio em `tema/` |
| Navegação | `nav:` hierárquica com títulos | lista plana, título = nome do arquivo | `preparar.py` gera a árvore |
| Busca | plugin `search` com `lang: pt` | **não existe** plugin | filtro de navegação no cliente; busca em página é `LPDOC-13` |
| Admonition (`!!! tip`) | extensão `admonition` + cores | **não existe** | convertido para citação `[!TIPO]`; `site.js` monta o aviso |
| Abas (`=== "A"`) | `pymdownx.tabbed` | **não existe** | convertido para subtítulo |
| Detalhe (`??? note`) | `pymdownx.details` | **não existe** | convertido para aviso aberto |
| Botão (`.md-button`) | `attr_list` + CSS | sem `attr_list` | atributo removido na conversão; vira link |
| Grid cards | `md_in_html` + `attr_list` | sem `md_in_html` | convertido em `<div class="cartoes"></div>` mais lista; CSS vira grade |
| Ícone `:material-x:` | `pymdownx.emoji` | shortcode próprio, só 11 emojis | emoji Unicode no material convertido |
| Tasklist | `pymdownx.tasklist` | **não existe** | `☐`/`☑` no material convertido |
| Destaque de sintaxe | `pymdownx.highlight` | **não existe** | `highlight.js` 11.12.0 no cliente |
| Copiar código | `content.code.copy` | **não existe** | `site.js` adiciona o botão |
| Mermaid | `custom_fences` | **não existe** | `mermaid.js` 12.3.0 renderiza `language-mermaid` |
| Tema claro/escuro | `palette` com alternância | **não existe** | variáveis CSS + `localStorage` |
| Modo estrito | `--strict` | **não existe** | `verificar.py` |
| Build no Windows | suportado | **quebrado** (`PosixPath`) | build em container Linux |

**Regra:** página nova usa só GFM + `> [!TIPO]` + cerca `mermaid`. Os recursos da coluna 1 existiram
apenas no material já escrito e foram convertidos uma vez em 2026-11-19 (`LPDOC-14` fechada); não são
padrão novo.

---

## 7. Gate de build e de publicação

### 7.1 Build

O build roda em **Linux com Python 3.15** — no Windows o MkDocs 2.0.dev6 falha por `PosixPath`.

```bash
# Windows: build dentro do container
docker run --rm -v "D:/dolorestec/wyd2:/work" -w /work/wyd-docs python:3.15-slim sh -c \
  "pip install -r requirements.txt && python preparar.py && mkdocs build --dir .build/docs --output site && python verificar.py site"

# Linux/macOS
cd wyd-docs
python3 -m venv .venv && .venv/bin/pip install -r requirements.txt
.venv/bin/python preparar.py
.venv/bin/mkdocs build --dir .build/docs --output site
.venv/bin/python verificar.py site
```

`preparar.py` e `verificar.py` são Python puro e rodam também no Windows, para iteração rápida.

### 7.2 O que o gate reprova

| Verificação | Reprova quando |
|---|---|
| link interno | destino não existe no site gerado |
| âncora | `#secao` não corresponde a nenhum `id` do destino |
| navegação | página gerada sem `<aside class="barra">` |
| título | página sem título no `<title>` |
| redirect | origem do `redirects.tsv` sem página-stub, destino inexistente, `refresh`/`canonical` fora do destino, destino que é outro stub ou origem duplicada |
| sintaxe 1.x | fonte publicada em `planejamento-1.0.0/*/docs` com `!!!`, `=== "…"`, `???`, atributo de link `](pagina.md){ … }` ou `<div class="grid cards" markdown>` fora de bloco de código e de code span |

**Medido em 2026-11-19 (gate em container Linux, `python:3.15-slim`):** 34 páginas montadas de 3 sistemas; `verificar.py` sem problema. Censo por estado na mesma montagem: **12 `proposto` · 16 sem linha de estado · 6 `aceito` = 34**. Esta é a medição histórica de 2026-11-19; o total e o censo vigentes estão em §9.1, e `convencoes.md`, `AGENTS.md`, `proximos-passos.md` e `docs/planos/metricas.md` citam aquela seção em vez de repetir os números.

### 7.3 Publicação

| Etapa | Onde | Condição |
|---|---|---|
| gate | `.github/actions/docs-check/` | todo PR que toca `planejamento-1.0.0/**` (a árvore inteira, sem lista de arquivos) ou `wyd-docs/**` |
| publicação | `.github/workflows/docs.yml` | push em `develop` e `main` |
| artefato | GitHub Pages | estático, sem servidor |

Permissões: `contents: read`, `pages: write`, `id-token: write`; `concurrency` de um deploy. Ação de
terceiro fixada por commit SHA. Rollback proposto: reconstruir e republicar o conteúdo de um commit
anterior conhecido; não há dado migrado. A execução já foi verificada (`V`, 2026-11-19): o workflow
`docs` tem 13 execuções (`/actions/workflows/docs.yml/runs`, `total_count` consultado na API em 2026-11-19) e o Pages serve o `success` mais recente (`7911904845`,
`0d765858`).

**Baseline do conteúdo público (`V`, 2026-11-19):** leitura HTTP somente `GET` de
<https://lucascantarelli.github.io/wyd/assets/site.js> devolveu `200 OK`,
`Content-Type: application/javascript; charset=utf-8`, `Content-Length: 4219`,
`Last-Modified: Thu, 19 Nov 2026 10:25:10 GMT` e `Date: Thu, 19 Nov 2026 11:27:30 GMT`, com
`x-proxy-cache: MISS` e `Cache-Control: max-age=600`. O corpo servido referencia o bundle browser
do Highlight.js por URL literal:
`https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.12.0/build/highlight.min.js`. O
`Last-Modified` cai entre o `created_at` (10:25:00Z) e o `success` (10:25:17Z) do deployment
`7910399204`, SHA `aedd1f73` — à época, o único `success` do ambiente.

Durações do workflow `docs`, medidas do `created_at` da execução até a conclusão — não apenas o
tempo do job:

| Execução | SHA | `created_at` UTC | conclusão UTC | duração | Deployment | estado atual |
|---|---|---:|---:|---:|---|---|
| [#4](https://github.com/lucascantarelli/wyd/actions/runs/44498880453) | `0d765858` | 09:42:27 | 09:43:16 | 49 s | `7909492410` | `inactive` desde 10:18:34 |
| [#6](https://github.com/lucascantarelli/wyd/actions/runs/44504331348) | `f65a45bb` | 10:17:50 | 10:18:33 | 43 s | `7910251866` | `inactive` desde 10:25:19 |
| [#8](https://github.com/lucascantarelli/wyd/actions/runs/44505419875) | `aedd1f73` | 10:24:48 | 10:25:17 | 29 s | `7910399204` | `inactive` desde 11:33:10 |

Os três estão `inactive` desde 10:18:34, 10:25:19 e 11:33:10; em cada caso há um deployment `success`
posterior (#6, #8 e a restauração). A duração acima é o tempo até a publicação daquela execução,
não o estado atual daquela versão. A baseline dimensiona a janela de congelamento; não promete
duração para uma janela futura. A coleta foi só leitura — nenhum workflow, deployment ou arquivo do site foi
acionado por ela.

**Runbook de rollback (`V`, 2026-11-19: executado até o fim; resultado no bloco "Teste real
executado", ao final desta seção):**

**Base verificada (`V`, 2026-11-19):** `.github/workflows/docs.yml` faz checkout sem `ref` explícito,
`workflow_dispatch` não declara inputs, o job `publicar` roda para qualquer evento que não seja PR e
a concorrência usa `docs-${{ github.ref }}` com cancelamento do run ativo. Portanto, reexecutar uma
execução anterior completa preserva SHA/ref do evento original; um dispatch não escolhe SHA arbitrário
e pode publicar o tip da ref escolhida. Referências: [eventos e workflow_dispatch](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows),
[reexecução](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/re-run-workflows-and-jobs),
[checkout](https://github.com/actions/checkout) e [concorrência](https://docs.github.com/en/actions/how-tos/writing-workflows/choosing-when-your-workflow-runs/control-the-concurrency-of-workflows-and-jobs).

**Pré-checagens sem publicação**

1. Na página Deployments / ambiente `github-pages`, identifique o SHA atualmente publicado. Escolha
   o SHA anterior que deve ser restaurado e uma página ou trecho cujo conteúdo diferencie as duas
   versões; registre os SHAs e a URL antes do teste. Não procure de novo: o **Baseline do conteúdo
   público**, acima, fixa a URL (`https://lucascantarelli.github.io/wyd/assets/site.js`), o marcador
   `cdn-release@11.12.0` e a medição HTTP completa; o bloco **Teste real executado**, ao final da
   seção, registra o SHA restaurado (`0d765858…`, deployment `7911904845`). O baseline foi medido com
   `aedd1f73…` no ar, antes do teste — use-o para a URL e o marcador, não como SHA a restaurar.
2. Na aba Actions, localize uma execução bem-sucedida do workflow `docs` para o SHA-alvo exato.
   Confirme `head_sha`, branch/ref, evento, conclusão do job `build e verificação` e do job
   `publicar no GitHub Pages`. Para usar **Re-run all jobs**, a execução deve estar dentro da janela
   de reexecução do GitHub (até 30 dias); reexecuções preservam `GITHUB_SHA`/`GITHUB_REF` e usam as
   permissões do ator da execução original.
3. Sem publicar, valide o commit-alvo num checkout local destacado ou container Linux: confirme
   `git rev-parse HEAD` contra o SHA registrado e rode `preparar.py`, `mkdocs build` e
   `verificar.py site`. Guarde o resultado do gate.
4. Confirme que não há deploy `docs` concorrente para a mesma ref. O grupo atual
   `docs-${{ github.ref }}` usa `cancel-in-progress: true`; uma nova execução da mesma ref pode
   cancelar a execução ativa. Verifique também as regras de proteção/revisão do ambiente
   `github-pages` e se o ator original pode satisfazê-las.

**Teste real (publica; exige autorização explícita antes de começar)**

1. Autorizar expressamente a publicação temporária do SHA anterior escolhido e a publicação de
   restauração do SHA que deve ficar ativo ao final. Pause mudanças que possam acionar um deploy na
   mesma ref durante a janela do teste.
2. Na execução `docs` verificada no passo 2, escolher **Re-run all jobs**, não apenas um job. Isso
   refaz checkout, gate, artefato e deploy. Como `actions/checkout` não especifica `ref`, usa o SHA
   do evento; a execução original precisa estar associada ao SHA-alvo exato. Antes de confirmar a
   reexecução, confira novamente o run e o SHA. Uma reexecução completa reconstrói o artefato; não
   depende de promover manualmente um artefato antigo.
3. Não usar `workflow_dispatch` como seletor de SHA: existe sem inputs e uma execução que não seja
   `pull_request` alcança o job de publicação. Selecionar uma branch executa o tip daquela ref, não
   escolhe um commit arbitrário.
4. Após o run, guarde o SHA/ref do run, a saída do gate e a conclusão do deploy. Confirme no histórico
   do ambiente Pages que o deploy-alvo foi registrado. Na URL pública, force recarga e confira o
   conteúdo identificador da versão anterior e os recursos necessários.
5. Republicar a versão que deve permanecer ativa usando sua execução bem-sucedida identificada na
   pré-checagem e **autorizada**; verificar novamente run, SHA no histórico de Pages e conteúdo
   público identificador restaurado. Se não for possível identificar/reexecutar com segurança os
   dois SHAs, interromper: o workflow atual não fornece um parâmetro de commit para dispatch.

**Limite operacional (`V`):** a documentação do GitHub permite reexecutar workflows até 30 dias após
a execução original. Há registro de execução `docs` bem-sucedida para o commit base
`2378b62b658303e6378c15be0a8ff38d248b019c` (`#44492052127`, 2026-11-19), anterior ao deploy do
PR #44 (`0d76585821583d5ef60e790fa44fb65572576d4b`, `#44498880453`, 2026-11-19). A data está dentro
do limite de 30 dias medido em 2026-11-19; antes do teste, confirme no GitHub que a execução original
continua acessível e oferece **Re-run all jobs**. Se não estiver elegível, não há no workflow atual
um caminho seguro para selecionar SHA arbitrário: `workflow_dispatch` escolhe branch/tag e usa seu
tip. Nesse caso, interromper e obter aprovação para um mecanismo separado, revisado e não usado sem
autorização. O teste real e a restauração foram executados em 2026-11-19 (bloco ao final da seção).

**Verificação somente leitura (`V`, 2026-11-19; não é teste real):** a execução `docs` **#2**
(ID `44492052127`) corresponde ao SHA `2378b62b658303e6378c15be0a8ff38d248b019c`, foi concluída
com sucesso em `develop` (`push`; criada em `2026-11-19T08:57:34Z`) e, na data da consulta,
permanece dentro da janela de reexecução de 30 dias. Evidência: [run #2](https://github.com/lucascantarelli/wyd/actions/runs/44492052127)
(e [endpoint da execução](https://api.github.com/repos/lucascantarelli/wyd/actions/runs/44492052127)).
A leitura da configuração do ambiente `github-pages` retornou a política de branch `develop`, sem
revisores obrigatórios nem temporizador de espera listados; não há proteções customizadas listadas
(`total_count: 0`) e `can_admins_bypass` é `true`. Evidências: [ambiente](https://api.github.com/repos/lucascantarelli/wyd/environments/github-pages),
[políticas de branch](https://api.github.com/repos/lucascantarelli/wyd/environments/github-pages/deployment-branch-policies)
e [proteções customizadas](https://api.github.com/repos/lucascantarelli/wyd/environments/github-pages/deployment-protection-rules).
A consulta foi somente leitura: não acionou `Re-run all jobs`, não publicou e não verificou um
rollback/restauração real nem as permissões de escrita do ator. Estes dados são pré-checagem, não
evidência para fechar `LPDOC-01`; antes do teste, reconfirmar a disponibilidade da reexecução e as
proteções vigentes.

**Pré-checagem somente leitura (`V`, 2026-11-19; não é evidência de rollback):** o deployment mais
recente de `github-pages` consultado é o ID `7909492410`, para o SHA
`0d76585821583d5ef60e790fa44fb65572576d4b`, com status final `success` ([deployment](https://github.com/lucascantarelli/wyd/deployments/7909492410),
[execução `docs` #4](https://github.com/lucascantarelli/wyd/actions/runs/44498880453)). Na consulta às
execuções do workflow `docs` em `develop`, não havia runs em `in_progress`, `queued` ou `waiting`
([workflow](https://github.com/lucascantarelli/wyd/actions/workflows/docs.yml?query=branch%3Adevelop)).
A verificação foi somente leitura; registra o SHA publicado e a ausência observada de concorrência,
não executa nem comprova rollback/restauração.

**Interrupção antes do teste (`V`, 2026-11-19; não é evidência de rollback):** antes de qualquer
reexecução, os runs `docs` #2 e #4 foram reconfirmados como concluídos com sucesso e com os SHAs
alvo `2378b62b658303e6378c15be0a8ff38d248b019c` e `0d76585821583d5ef60e790fa44fb65572576d4b`,
respectivamente ([#2](https://github.com/lucascantarelli/wyd/actions/runs/44492052127),
[#4](https://github.com/lucascantarelli/wyd/actions/runs/44498880453)). Porém, o histórico apresentou
um deployment posterior de `github-pages`, ID `7910251866`, com SHA
`f65a45bbfde806ec5bbf6122293a81b73b02b2d3`, status final `success` ([deployment](https://github.com/lucascantarelli/wyd/deployments/7910251866),
[execução `docs` #6](https://github.com/lucascantarelli/wyd/actions/runs/44504331348)). Esse SHA diverge
do estado que havia sido reconfirmado antes como atual (`0d76585821583d5ef60e790fa44fb65572576d4b`).
Conforme a condição de parada, nenhuma reexecução foi iniciada. Não houve validação do conteúdo
público das duas versões do teste, nem rollback ou restauração real.

**Segunda interrupção (`V`, 2026-11-19):** após o dono confirmar que o SHA final deveria ser o do run
#4, uma nova reconfirmação encontrou outro deployment posterior: ID `7910399204`, SHA
`aedd1f7338fdfd7125c0c25f4c47255ca88d8672`, status final `success`, gerado pela execução
`docs` #8 ([deployment](https://github.com/lucascantarelli/wyd/deployments/7910399204),
[run #8](https://github.com/lucascantarelli/wyd/actions/runs/44505419875)); os runs #2 e #4 seguiam
válidos, com os SHAs originais e conclusão `success`. Nenhuma reexecução foi acionada nesta nova
rodada.

**Causa do bloqueio (`V`):** o workflow publica em todo evento que não seja PR. Enquanto `develop`
recebe merges — três deploys em 43 minutos nesta data (#4 às 09:42:27, #8 às 10:25:17) — o Pages
avança para o tip da branch e qualquer versão restaurada por reexecução é sobrescrita pelo push
seguinte. O ensaio não era executável com a branch ativa; a pré-condição que faltava não era técnica
do workflow, era a janela sem publicação.

**Pré-condição que faltava (`V`):** declarar um congelamento explícito de `develop` (sem merge nem push)
durante todo o teste, do rollback até a restauração e a validação pública; registrar os SHAs
verificados imediatamente antes de cada reexecução. Sem essa janela, o teste mede apenas a última
publicação automática, não o rollback. A aprovação do dono para o teste foi consumida em 2026-11-19,
na janela declarada, com o resultado registrado ao final desta seção.

**Próximos passos para desbloquear o teste (`P`; ordem por ganho ÷ esforço, não por completude)**

| # | Passo | Desbloqueia concretamente | Exige do usuário | Custo / risco |
|---:|---|---|---|---|
| 1 | Abrir a janela de congelamento de `develop` e executar o checklist abaixo — **feito** em 2026-11-19 | o teste real inteiro, que fechou `LPDOC-01` | declarar a janela e não mergear durante ela (~20 min) | baixo: coordenação apenas; nenhuma alteração de código, workflow ou configuração |
| 2 | Validar os dois SHAs localmente antes da janela (checkout destacado + `preparar.py`, `mkdocs build`, `verificar.py site`) | tira falha de build do meio da janela, onde cada minuto conta | nenhuma autorização nova: não publica | baixo: minutos de container Linux; **feito** em 2026-11-19, ver bloco de validação local abaixo |
| 3 | Decidir o estado do site depois da restauração — **decidido** em 2026-11-19: fica `0d765858…` até o merge do PR, que publica o tip `aedd1f73…` de conteúdo idêntico | fecha sem execução extra: o merge do PR já dispara `docs` | — | nenhum: sem republicação manual |
| 4 | Se a janela falhar de novo, mudar o mecanismo (alternativa D) — **não aplicável**: a janela funcionou em 2026-11-19 | reduz a dependência de disciplina de merge | autorização + admin do repositório | médio-alto e permanente; ler a tabela de alternativas |

**Alternativas ao congelamento (`V` = documentação ou configuração lida; `P` = avaliação)**

| Alternativa | Como funcionaria | Evidência | Custo / risco | Autorização |
|---|---|---|---|---|
| **A. Congelar `develop` na janela** | nenhum merge ou push durante rollback, restauração e validação | o workflow publica em todo evento que não seja PR; três deploys saíram em 43 min nesta data (#4 a #8) | baixo: coordenação; a janela precisa ser curta porque merges são frequentes | **gate do usuário**: declarar a janela |
| B. Desabilitar o workflow `docs` | `gh workflow disable` impede novos disparos | `V`: [não é possível reexecutar](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/re-run-workflows-and-jobs) run ou job com o workflow desabilitado; é preciso reabilitar antes, e reabilitado o próximo push volta a publicar | não cria janela protegida: reabilitar para reexecutar devolve o risco; acrescenta o risco de esquecer reabilitar | autorização + escrita no repositório |
| C. Publicar só sob `workflow_dispatch` | editar o gatilho para o `push` não publicar | `V`: a reexecução usa `GITHUB_SHA`/`GITHUB_REF` do evento e o workflow do commit original, então os runs #2/#4 seguiriam publicando como hoje | alto: PR + merge para ligar (o merge já roda `docs`), outro PR para reverter; o teste deixa de medir o fluxo de produção | **autorização explícita antes de tocar em workflow** |
| D. `github-pages` com revisor obrigatório | deploy fica pendente até aprovação manual | `V` (2026-11-19): o ambiente tem só a política de branch `develop`, sem revisores, sem proteções customizadas e com `can_admins_bypass: true` | médio-alto e permanente: todo deploy passa a exigir aprovação manual; não impede a criação do deploy, apenas o segura | autorização + admin; muda a operação, então não cabe em página |

**Recomendação (`P`): alternativa A, congelar `develop` na janela.** É a única que não altera o objeto
medido: o teste precisa exercitar o fluxo de produção como ele é, e A mantém workflow, ambiente e
gatilhos idênticos. O sintoma observado — reimplementação do tip em 43 minutos — é concorrência de
`push`, não defeito de publicação; o remédio é não publicar durante a janela, não reescrever a
automação. B e C custam mais e mexem no fluxo sob teste; C ainda exige reverter depois e, pela regra de
reexecução, não mudaria o comportamento dos runs #2/#4. D só se justifica se a janela de congelamento
falhar de forma repetida, porque introduz aprovação manual permanente em toda publicação.

**Checklist da janela de congelamento (`P`; gate do usuário — só abrir com a janela declarada)**

Sequencial sempre: os runs #2 e #4 compartilham o grupo `docs-${{ github.ref }}` com
`cancel-in-progress: true`, então nunca rodar os dois ao mesmo tempo.

| Passo | Ação | Critério de sucesso | Falha |
|---:|---|---|---|
| 0 | Declarar o congelamento de `develop` (sem merge, sem push) e registrar hora de início | nenhuma publicação durante os passos 1–8 | houve push → parar e reabrir o gate |
| 1 | Ler o deployment mais recente de `github-pages` e o run `docs` mais recente | registrar `D0` (ID + SHA) e dizer qual SHA está publicado; em 2026-11-19 o último observado era `7910399204` / `aedd1f7338fdfd7125c0c25f4c47255ca88d8672` | deploy `in_progress`/`queued` → esperar terminar e reler |
| 2 | Reconfirmar run #2 (`44492052127`): `head_sha`, branch, conclusão | `2378b62b658303e6378c15be0a8ff38d248b019c`, `develop`, `success` | divergiu → parar |
| 3 | Reconfirmar run #4 (`44498880453`): `head_sha`, branch, conclusão | `0d76585821583d5ef60e790fa44fb65572576d4b`, `develop`, `success` | divergiu → parar |
| 4 | Confirmar ausência de execução `docs` concorrente em `develop` | nenhuma em `in_progress`, `queued` ou `waiting` | existe → parar |
| 5 | **Rollback:** Re-run all jobs no run #2 | novo deployment `D1` com SHA `2378b62b…` e status `success` | falhou ou SHA diferente → parar |
| 6 | Validar o conteúdo público de `D1` | `assets/site.js` servido contém `cdn.jsdelivr.net/npm/highlight.js@11.12.0/lib/common.min.js` e a página com bloco de código acusa `require is not defined` no console (`V`: URL presente na árvore do commit) | conteúdo igual ao de `0d765858…` → parar |
| 7 | **Restauração**, só se 5 e 6 passaram: Re-run all jobs no run #4 | novo deployment `D2` com SHA `0d765858…` e status `success` | falhou ou SHA diferente → parar e reabrir |
| 8 | Validar o conteúdo público de `D2` | igual à referência do deployment `7909492410`: `assets/site.js` contém `cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.12.0/build/highlight.min.js`, o recurso responde 200 e `hljs` existe, sem `require is not defined` | divergiu → parar e reabrir |
| 9 | Registrar em §7.3 e em `LPDOC-01` | data, janela, `D0`/`D1`/`D2`, links dos runs e dos deploys, prova pública de cada versão | falta de prova → lacuna aberta |

**Critério de parada (`P`):** qualquer divergência de SHA, de branch, de conclusão ou de conteúdo, ou
qualquer novo deployment não explicado pelo passo em curso, encerra a janela no ato. Não se repete o
passo dentro da mesma janela: reabre-se o gate com o estado novo e decide-se entre nova janela ou
alternativa D. Restaurar para `0d765858…` deixa o site atrás do tip de `develop` até o próximo push;
espa é a decisão do passo 3.

**Limites já medidos (`V`):** reexecução vale até 30 dias e no máximo 50 vezes por run, e usa as
permissões do ator original. Os deploys de referência conhecidos são `7908551978` (`2378b62b…`),
`7909492410` (`0d765858…`), `7910251866` (`f65a45bb…`) e `7910399204` (`aedd1f73…`).

**Validação local dos dois commits (`V`, 2026-11-19; passo 2 da lista; não é rollback e não publica)**

Método: cada commit foi extraído com `git archive <sha> | tar -x` para
`wyd-docs/.build/validacao/<short>` (diretório ignorado pelo git), com `git rev-parse <sha>^{tree}`
registrado como vínculo árvore↔SHA, e o gate rodou no mesmo container Linux do build:

```bash
docker run --rm -v "D:/dolorestec/wyd2:/work" -w <árvore>/wyd-docs python:3.15-slim sh -c \
  "pip install -r requirements.txt && python preparar.py && mkdocs build --dir .build/docs --output site && python verificar.py site"
```

| Commit | Tree | `preparar.py` | `mkdocs build` | `verificar.py site` | Exit |
|---|---|---|---|---|---|
| `2378b62b658303e6378c15be0a8ff38d248b019c` | `747989dd0d8a9e6f83feecc5926f50852dd6237b` | 30 páginas de 3 sistemas | ok | 30 páginas, links, âncoras e navegação ok | 0 |
| `0d76585821583d5ef60e790fa44fb65572576d4b` | `f817fa94e80a75763cd18aebc3db14f9395136e3` | 30 páginas de 3 sistemas | ok | 30 páginas, links, âncoras e navegação ok | 0 |

As duas árvores diferem em um único arquivo publicado: `wyd-docs/tema/site.js`, na URL do
Highlight.js — `cdn.jsdelivr.net/npm/highlight.js@11.12.0/lib/common.min.js` em `2378b62b…` e
`cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.12.0/build/highlight.min.js` em `0d765858…`. É esse o
marcador usado nos passos 6 e 8 do checklist: dá para conferir por HTTP em `assets/site.js`, sem
depender do console.

**Teste real executado (`V`, 2026-11-19; prova de `LPDOC-01`)**

Janela declarada pelo dono antes do primeiro comando: nenhum merge, push ou outra publicação em
`develop` durante o teste. O checklist foi seguido de 0 a 9, com os dois runs em sequência.

| Passo | Resultado medido |
|---:|---|
| 1 | `D0` = deployment `7910399204`, SHA `aedd1f73` — último `success` antes do teste |
| 2 | run #2 `44492052127`: `2378b62b`, `develop`, `success` |
| 3 | run #4 `44498880453`: `0d765858`, `develop`, `success` |
| 4 | nenhuma execução `docs` em `in_progress`, `queued` ou `waiting` antes de começar |
| 5 | rollback: [deployment `7911886524`](https://github.com/lucascantarelli/wyd/deployments/7911886524), SHA `2378b62b`, `success` às `11:33:08Z` |
| 6 | `assets/site.js` público passou ao marcador `npm/highlight.js@11.12.0` |
| 7 | restauração: [deployment `7911904845`](https://github.com/lucascantarelli/wyd/deployments/7911904845), SHA `0d765858`, `success` às `11:34:01Z` |
| 8 | `assets/site.js` voltou ao marcador `cdn-release@11.12.0`; `200` na raiz e em `/sistemas/wyd-db/`, `/sistemas/wyd-docs/` e `/contribuir/` |
| 9 | este registro e `LPDOC-01` fechada em `lacunas.md` |

Reexecuções por **Re-run all jobs**, com o mesmo `head_sha` e a mesma ref das execuções originais:
[#2 attempt 2](https://github.com/lucascantarelli/wyd/actions/runs/44492052127) concluído em
`11:33:08Z` (deployment `created_at` `11:32:52Z`, 16 s até `success`) e
[#4 attempt 2](https://github.com/lucascantarelli/wyd/actions/runs/44498880453) em `11:34:01Z`
(`created_at` `11:33:44Z`, 17 s). O job `build e verificação` (que chama o `docs-check`) e o
`publicar no GitHub Pages` concluíram `success` nas duas reexecuções. Após `D2`, o `7911886524` ficou
`inactive` (`11:34:02Z`) e o `7910399204` ficou `inactive` (`11:33:10Z`): resta um deployment
`success`, o `7911904845`.

**O que não foi medido:** o erro de console (`require is not defined`) do passo 6 e a presença de
`hljs` no navegador no passo 8 — a validação pública usou o marcador HTTP em `assets/site.js`, como
o próprio checklist prevê; nenhum navegador foi automatizado.

**Efeito de estado (`V`, 2026-11-19):** o site está em `0d765858…` (`D2`) e `develop` está em
`aedd1f73…`. As duas versões publicam conteúdo idêntico — `git diff --name-only 0d765858 aedd1f73 --
'planejamento-1.0.0/*/docs/**' 'wyd-docs/**'` volta vazio —, então a diferença é de registro de
deployment, não de conteúdo. `P`: o próximo push em `develop` publica o tip outra vez.

Conclusão: os dois commits constroem e verificam em Linux, então falha de build deixa de ser causa
provável de parada dentro da janela. Isso, sozinho, não validava rollback nem fechava `LPDOC-01`; o
teste real registrado ao final da seção fez as duas coisas.

**Próximos passos do projeto:** a ordem priorizada por ganho ÷ esforço — incluindo o que a automação
executa sem decisão do dono e o que só o dono decide — mora em `planejamento-1.0.0/wyd-1.0.0.md` §6;
esta seção guarda o detalhe operacional do teste.

**Critério de evidência aplicado no fechamento:** data e janela, SHA antes/alvo/restaurado, links das
execuções e dos deploys Pages, resultado dos jobs de cada reexecução e prova pública do conteúdo de
cada SHA — tudo no bloco acima. Sem rollback e restauração validados, a lacuna ficaria aberta.

**Estado observado (`V`, 2026-11-19; validação do PR #44, não do teste de rollback):** o PR #44 foi
mesclado e o workflow `docs` executou com sucesso o gate e a publicação em GitHub Pages; a URL
pública foi recarregada, o bundle do Highlight.js 11.12.0 respondeu HTTP 200 e o console não exibiu
os dois erros descritos acima.

**Decisão registrada (`V`, 2026-11-19):** o dono aprovou GitHub Pages como hospedagem canônica,
com publicação contínua nele. O runbook foi executado em 2026-11-19 e a evidência está em §7.3, o que
fechou `LPDOC-01`. Não alterar a hospedagem sem nova decisão.

---

## 8. Identificadores

| Prefixo | Uso | Dono |
|---|---|---|
| `LPDOC-nn` | lacuna do site | `lacunas.md` |
| `WYDDOC-nnn` | issue do roadmap | `roadmap/` |
| `ADR-DOC-nnnn` | decisão estrutural do site | `docs/arquitetura/adr/` (vazio) |

Não há ADR nesta entrega: GitHub Pages foi aprovado como hospedagem canônica e a decisão está registrada neste plano; tema, montagem e as demais decisões continuam regidos pelas lacunas correspondentes.

---

## 9. Ordem de produção

| Fase | Escopo | Estado |
|---|---|---|
| 1 | site funcionando: montagem, tema, gate local | **feito** (2026-11-19) |
| 2 | publicação: workflow, Pages, rollback | workflow e Pages publicados e validados (`V`, 2026-11-19); GitHub Pages aprovado como hospedagem canônica (`V`, 2026-11-19); validação de rollback feita (`V`, 2026-11-19; `LPDOC-01` fechada) |
| 3 | padrão editorial: convenções, guia do autor, revisão | **feito** (2026-11-19): `docs/convencoes.md`, `docs/guia-autor/index.md` e `docs/revisao.md` `aceito` pelo dono de `documentação` |
| 4 | robustez: redirect, busca, métrica | pendente: commit e CI do pacote — redirect implementado em 2026-11-19 (`LPDOC-06` fechada; `wyd-docs/redirects.tsv`, stub e checagens no `verificar.py`); busca decidida e métrica aceita em 2026-11-19 |
| 5 | 109 páginas por tabela do `wyd-db` | **decidido em 2026-11-21 (`M26`)**: escritas à mão em lotes no `wyd-db`, sem gerador; **71 escritas** em 2026-11-22 (lotes `0001`–`0012`, `0013`, `0014`, `0015`, `0019`, `0021` em parte e `0024`) |

### 9.1 As 109 páginas por tabela — escritas à mão (`M26`)

**Estado: decidido em 2026-11-21.** O dono recusou o gerador (`make dicionario-dados`, projeto
`tools/wyd-db-pages`): as 109 páginas são escritas à mão, em lotes, no `wyd-db`, e o site apenas publica
o que existir. A decisão é a lacuna `M26` (`planejamento-1.0.0/wyd-db/lacunas.md` §2.3). **Nenhuma
ferramenta é criada; 71 das 109 páginas existem em 2026-11-22** (lotes `0001`–`0012`, `0013`, `0014`, `0015`, `0019`, `0021` em parte e `0024`; índice em `planejamento-1.0.0/wyd-db/docs/dicionario-dados/gerado/index.md`).

| Item | Definição |
|---|---|
| Projeto | nenhum: o gerador `wyd-db-pages` foi recusado em `M26` (2026-11-21); a produção é manual e mora no `wyd-db` |
| Entrada | migrations e DDL confirmado do `wyd-db`; o formato da página é de `planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md` §6.8, e cada campo declara sua origem, sem copiar valor do legado como fato |
| Saída | `planejamento-1.0.0/wyd-db/docs/dicionario-dados/gerado/`: **109 páginas, escritas à mão**, uma por tabela planejada em `plano-migrations.md` §3, + `gerado/index.md` (índice de navegação, fora da contagem) |
| Teto | **109 páginas**, uma por tabela planejada em `plano-migrations.md` §3; o `index.md` não amplia o teto. A 98ª página é defeito de escopo, não página nova por conveniência |
| Fora do escopo | editar página manual, decidir coluna, publicar direto no Pages e gerar catálogo de opcodes (`LPDOC-10`) |

**Critério de pronto das páginas** (todos os itens, quando escritas):

1. Exatamente **109 páginas + `gerado/index.md`**, uma por tabela de `plano-migrations.md` §3; a 98ª é defeito de escopo.
2. Cada página tem cabeçalho de 5 campos, título, referência explícita à página de `migrations/` dona da forma (em `code`, porque `migrations/` não é publicado) e origem declarada; não contém PII nem segredo.
3. Nenhuma página copia o DDL: a forma é ligada, não duplicada (`wyd-db-1.0.0.md` §3, item 3).
4. `preparar.py` e `verificar.py` passam com as páginas presentes; **nenhum stub afirma que elas existem** antes de existirem.

**Reconciliação e estado (2026-11-22):** não há exceção de tabela — as páginas, quando escritas, cobrem as 109 tabelas planejadas, uma por tabela, conforme `planejamento-1.0.0/wyd-db/plano-migrations.md` §3; o índice é artefato auxiliar fora do teto. As 109 são contagem planejada, não medida de catálogo aplicado. **71 das 109 páginas existem em 2026-11-22:** `gerado/` tem as páginas de tabela dos arquivos `0001`–`0012`, `0013`, `0014`, `0015`, `0019`, `0021` em parte e `0024` e o `index.md` de navegação, que lista as 71 existentes. A cobertura não é pré-requisito do site atual: o site passa com **144 páginas** (medido em 2026-11-22 pelo build) e publica as que existem (`V`, 2026-11-22). O censo das 142 fontes na mesma data é **14 `aceito` · 115 `proposto` · 12 sem linha de estado**, mais o `gerado/index.md` de navegação, que tem a contagem no lugar do estado editorial; as 2 páginas próprias do site ficam fora do censo, e 142 + 2 = 144 páginas montadas. Este parágrafo é o único dono do total e do censo; as demais páginas citam esta seção em vez de repetir os números. As quatro grandezas são conferidas por comando, `wyd-docs/verificar-contagens.py`, que reprova quando qualquer uma diverge da montagem.

A fila de próximos passos do sistema fica em [proximos-passos.md](proximos-passos.md).

**Gate da `1.0.0` do site:** build e verificação verdes no CI, publicação automática em `develop`,
URL estável para toda página publicada e nenhum conteúdo interno no artefato. A URL estável está
declarada desde 2026-11-19: `https://lucascantarelli.github.io/wyd/`, sem domínio próprio na `1.0.0`
(`V`, `LPDOC-04`). O deploy público e a
validação descritos em §7.3, junto da aprovação de GitHub Pages, não comprovavam rollback por si sós;
o teste real de 2026-11-19 é a prova registrada que fechou `LPDOC-01`.

**Conteúdo no ar após a restauração (decisão do dono, 2026-11-19):** o site fica em `0d765858…` até
o merge do PR, que publica o tip `aedd1f73…` de conteúdo idêntico. Conferência:
`git diff --name-only 0d765858 aedd1f73 -- 'planejamento-1.0.0/*/docs/**' 'wyd-docs/**'` volta
vazio. Sem republicação manual.

---

## 10. Decisões que dependem do time

As decisões do site foram fechadas em 2026-11-19 e a tabela mantém o registro. A hospedagem canônica
foi aprovada pelo dono: GitHub Pages com publicação contínua (`V`, 2026-11-19). `LPDOC-01` foi fechada
com o rollback validado (§7.3), e `LPDOC-04`: sem domínio próprio na `1.0.0`, o site publica em
`lucascantarelli.github.io/wyd`. O dono também fechou `LPDOC-05` (só a branch), `LPDOC-06` (redirect
obrigatório), `LPDOC-10` (sem catálogo gerado), `LPDOC-13` (sem busca de texto completo), `LPDOC-08`
(arquitetura aprova o sistema novo e a entrada em `SISTEMAS`; documentação publica), `LPDOC-02`
(não publicar `AGENTS.md` nem o panorama), `LPDOC-03` (publicar `proposto` com aviso visível),
`LPDOC-07` (só pt-BR), `LPDOC-09` (gerar e publicar as 109 páginas, uma por tabela planejada; ver §9.1) e `LPDOC-11` (manter `2.0.dev6`
fixo até a próxima dev).

| # | Decisão | Lacuna |
|---|---|---|
| 1 | Publicação contínua no GitHub Pages como hospedagem canônica (aprovado pelo dono em 2026-11-19); rollback validado em 2026-11-19 | `LPDOC-01` |
| 2 | Não publicar `AGENTS.md` nem o panorama (decidido em 2026-11-19) | `LPDOC-02` |
| 3 | Publicar página com estado `proposto`, com aviso visível no topo (decidido em 2026-11-19) | `LPDOC-03` |
| 4 | Sem domínio próprio na `1.0.0`: publica em `lucascantarelli.github.io/wyd` (decidido em 2026-11-19) | `LPDOC-04` |
| 5 | Só a branch, sem `/1.0.0/`: uma versão viva (decidido em 2026-11-19) | `LPDOC-05` |
| 6 | Redirect ao renomear página: obrigatório, mapa em `wyd-docs/redirects.tsv` (decidido e implementado em 2026-11-19) | `LPDOC-06` |
| 7 | Só pt-BR na `1.0.0`; i18n fora desta versão (decidido em 2026-11-19) | `LPDOC-07` |
| 8 | Arquitetura aprova sistema novo e a entrada em `SISTEMAS`; documentação publica (decidido em 2026-11-19) | `LPDOC-08` |
| 9 | Publicar as 109 páginas do `wyd-db`, uma por tabela planejada (decidido em 2026-11-19); a **produção** voltou à mesa em 2026-11-21 como `M26` do `wyd-db`, sem gerador presumido (§9.1) | `LPDOC-09` |
| 10 | Não publicar catálogo de opcodes gerado na `1.0.0` (decidido em 2026-11-19) | `LPDOC-10` |
| 11 | Manter `2.0.dev6` fixo; revisar quando a próxima dev for publicada, com build e `verificar.py` antes (decidido em 2026-11-19) | `LPDOC-11` |
| 12 | Build no Windows ou só em Linux/CI | `LPDOC-12` |
| 13 | Sem busca de texto completo na `1.0.0`: o filtro de navegação é a navegação (decidido em 2026-11-19) | `LPDOC-13` |
| 14 | Até quando a ponte `compat.py` existiu | `LPDOC-14` |

---

## 11. O que este documento não decide

- Não decide conteúdo de sistema: publica o que o dono escreveu.
- A validação operacional do rollback (`V`, 2026-11-19) é a prova que fechou `LPDOC-01`; aprovar a
  hospedagem e provar o rollback continuam coisas distintas.
- Não cria ADR.
- Não decide o padrão editorial de cada sistema: cada um tem o seu; o mínimo comum é
  `wyd-docs/docs/contribuir/`.
- Não mede a documentação do projeto: `metricas.md` do site mede o site; `wyd-1.0.0.md` §1 mede o
  projeto.

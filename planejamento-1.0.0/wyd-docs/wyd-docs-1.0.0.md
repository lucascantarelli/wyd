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
> suporte a Windows (§2). O site desta entrega é a primeira montagem real: **30 páginas**, build e
> verificação executados (§7.1).
>
> **Evidência de publicação (`V`, 2026-11-19):** o PR [#44](https://github.com/lucascantarelli/wyd/pull/44)
> foi mesclado no commit `0d76585821583d5ef60e790fa44fb65572576d4b`; o workflow
> [docs](https://github.com/lucascantarelli/wyd/actions/runs/44498880453) concluiu o build/verificação
> e a publicação no Pages com sucesso. Após recarga do site público
> <https://lucascantarelli.github.io/wyd/>, o bundle browser do Highlight.js 11.12.0 respondeu HTTP
> 200 e o console ficou sem `require is not defined` e sem o erro em `highlightElement`.
> Decisão do dono (`V`, 2026-11-19): GitHub Pages é a hospedagem canônica, com publicação
> contínua nele. A validação do rollback ainda está pendente em `LPDOC-01`; esta evidência não fecha
> essa lacuna.

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
| ponte de sintaxe para o material escrito em MkDocs 1.x | reescrever página de sistema por causa do site |
| verificação de link, âncora, navegação e título | revisão de conteúdo: dono é o sistema |
| publicação em GitHub Pages por Actions | hospedagem paga, CDN paga, domínio próprio |

### 1.3 Contratos

| Sistema | Fornece ao site | Exige do site |
|---|---|---|
| os 13 sistemas | página em `planejamento-1.0.0/<sistema>/docs/**/*.md` com cabeçalho de 5 linhas | URL estável, navegação automática, gate que reprova link quebrado |
| `wyd-db` | 15 páginas escritas + 91 geradas (planejadas) | seção própria e ordem de leitura preservada |
| `wyd-protocol` | 2 páginas escritas + teto de 131 | seção própria |
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
| sem admonition/abas | `compat.py` traduz o material antigo; página nova usa GFM (§6) |
| sem validação | `verificar.py` é o gate (§7.2) |
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
├── compat.py                         ● ponte MkDocs 1.x → GFM da 2.0
├── verificar.py                      ● gate de link, âncora, nav e título
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
│   ├── convencoes.md                 ▲ padrão editorial do site
│   ├── arquitetura/
│   │   ├── index.md                  ▲
│   │   ├── montagem-do-site.md       ▲ preparar, compat, build, verificar
│   │   ├── tema-e-recursos.md        ▲ o que a 2.0 não tem e como o tema cobre
│   │   ├── publicacao.md             ▲ Pages, permissões, rollback
│   │   └── dominio-e-url.md          ▲ URL como contrato
│   ├── planos/
│   │   ├── index.md                  ▲
│   │   ├── plano-de-documentacao.md  ▲
│   │   └── metricas.md               ▲ dono dos números do site
│   ├── roadmap/
│   │   ├── index.md                  ▲
│   │   ├── fases.md                  ▲
│   │   └── WYDDOC-001..WYDDOC-012.md ▲
│   ├── guia-autor/                   ▲
│   ├── guia-operador/                ▲
│   └── guia-desenvolvedor/           ▲
└── prompts/  skills/                 ▲ (4 + 4)
```

**Medido:** 2 páginas em `docs/` do sistema, 6 arquivos implementados no projeto do site.

---

## 6. Recursos: como eram no MkDocs 1.x e como são na 2.0

| Recurso usado até aqui | No MkDocs 1.x / Material | No MkDocs 2.0 | O que o site faz |
|---|---|---|---|
| Configuração do site | `mkdocs.yml` (YAML) | **não existe**; o CLI aborta se encontrar o arquivo | comando fixo em §7.1 |
| Tema | `theme: material` com `features` | **não existe** tema embutido; só `theme/base.html` | tema próprio em `tema/` |
| Navegação | `nav:` hierárquica com títulos | lista plana, título = nome do arquivo | `preparar.py` gera a árvore |
| Busca | plugin `search` com `lang: pt` | **não existe** plugin | filtro de navegação no cliente; busca em página é `LPDOC-13` |
| Admonition (`!!! tip`) | extensão `admonition` + cores | **não existe** | `compat.py` converte para citação `[!TIPO]`; `site.js` monta o aviso |
| Abas (`=== "A"`) | `pymdownx.tabbed` | **não existe** | `compat.py` converte para subtítulo |
| Detalhe (`??? note`) | `pymdownx.details` | **não existe** | `compat.py` converte para aviso aberto |
| Botão (`.md-button`) | `attr_list` + CSS | sem `attr_list` | `compat.py` remove o atributo; vira link |
| Grid cards | `md_in_html` + `attr_list` | sem `md_in_html` | `compat.py` marca a lista; CSS vira grade de cartões |
| Ícone `:material-x:` | `pymdownx.emoji` | shortcode próprio, só 11 emojis | `compat.py` troca por emoji Unicode |
| Tasklist | `pymdownx.tasklist` | **não existe** | `compat.py` troca por `☐`/`☑` |
| Destaque de sintaxe | `pymdownx.highlight` | **não existe** | `highlight.js` 11.12.0 no cliente |
| Copiar código | `content.code.copy` | **não existe** | `site.js` adiciona o botão |
| Mermaid | `custom_fences` | **não existe** | `mermaid.js` 12.3.0 renderiza `language-mermaid` |
| Tema claro/escuro | `palette` com alternância | **não existe** | variáveis CSS + `localStorage` |
| Modo estrito | `--strict` | **não existe** | `verificar.py` |
| Build no Windows | suportado | **quebrado** (`PosixPath`) | build em container Linux |

**Regra:** página nova usa só GFM + `> [!TIPO]` + cerca `mermaid`. Os recursos da coluna 1 existem
apenas para o material já escrito; a ponte é dívida declarada (`LPDOC-14`), não padrão novo.

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
| contagem | `preparar.py` monta menos páginas que as existentes nos sistemas |

**Medido em 2026-11-19:** 30 páginas montadas de 3 sistemas; `verificar.py` sem problema.

### 7.3 Publicação

| Etapa | Onde | Condição |
|---|---|---|
| gate | `.github/actions/docs-check/` | todo PR que toca `planejamento-1.0.0/**/docs/**` ou `wyd-docs/**` |
| publicação | `.github/workflows/docs.yml` | push em `develop` e `main` |
| artefato | GitHub Pages | estático, sem servidor |

Permissões: `contents: read`, `pages: write`, `id-token: write`; `concurrency` de um deploy. Ação de
terceiro fixada por commit SHA. Rollback proposto: reconstruir e republicar o conteúdo de um commit
anterior conhecido; não há dado migrado. A execução ainda não foi verificada.

**Runbook de rollback (`P`; não executado nesta atualização; depende de execução-alvo elegível dentro da janela de 30 dias):**

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
   versões; registre os SHAs e a URL antes do teste.
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
autorização. O teste real e a restauração continuam pendentes.

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
não executa nem comprova rollback/restauração. `LPDOC-01` permanece aberta.

**Evidência para fechar `LPDOC-01`:** registrar data, aprovação para o teste e para restauração, SHA
antes/alvo/restaurado, links das execuções e dos deploys Pages, resultado do gate de cada execução e
prova pública do conteúdo correspondente a cada SHA. Fechar somente depois de rollback e restauração
validados; falha ou ausência de evidência mantém a lacuna aberta.

**Estado observado (`V`, 2026-11-19):** o PR #44 foi mesclado e o workflow `docs` executou com
sucesso o gate e a publicação em GitHub Pages; a URL pública foi recarregada, o bundle do
Highlight.js 11.12.0 respondeu HTTP 200 e o console não exibiu os dois erros descritos acima.

**Decisão registrada (`V`, 2026-11-19):** o dono aprovou GitHub Pages como hospedagem canônica,
com publicação contínua nele. `LPDOC-01` permanece **aberta** até que o runbook seja executado e a
evidência acima seja registrada. Não alterar a hospedagem nem fechar a lacuna antes dessa prova.

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
| 2 | publicação: workflow, Pages, rollback | workflow e Pages publicados e validados (`V`, 2026-11-19); GitHub Pages aprovado como hospedagem canônica (`V`, 2026-11-19); validação de rollback pendente (`LPDOC-01` aberta) |
| 3 | padrão editorial: convenções, guia do autor, revisão | pendente |
| 4 | robustez: redirect, busca, métrica | pendente |

**Gate da `1.0.0` do site:** build e verificação verdes no CI, publicação automática em `develop`,
URL estável para toda página publicada e nenhum conteúdo interno no artefato. O deploy público e a
validação descritos em §7.3, junto da aprovação de GitHub Pages, não comprovam rollback; `LPDOC-01`
segue aberta até essa validação.

---

## 10. Decisões que dependem do time

As decisões ainda pendentes apontam a lacuna que as fecha. A hospedagem canônica já foi aprovada
pelo dono: GitHub Pages com publicação contínua (`V`, 2026-11-19). `LPDOC-01` permanece aberta
somente para acompanhar a validação do rollback.

| # | Decisão | Lacuna |
|---|---|---|
| 1 | Publicação contínua no GitHub Pages como hospedagem canônica (aprovado pelo dono em 2026-11-19); validar rollback | `LPDOC-01` |
| 2 | Publicar `AGENTS.md` e o panorama no site | `LPDOC-02` |
| 3 | Publicar página com estado `proposto` | `LPDOC-03` |
| 4 | Domínio próprio | `LPDOC-04` |
| 5 | Versionar o site por release | `LPDOC-05` |
| 6 | Redirect ao renomear página | `LPDOC-06` |
| 7 | Idioma único ou i18n | `LPDOC-07` |
| 8 | Quem aprova publicação de sistema novo | `LPDOC-08` |
| 9 | Incluir as 91 páginas geradas do `wyd-db` | `LPDOC-09` |
| 10 | Publicar catálogo de opcodes gerado | `LPDOC-10` |
| 11 | Quando atualizar o `2.0.dev6` | `LPDOC-11` |
| 12 | Build no Windows ou só em Linux/CI | `LPDOC-12` |
| 13 | Busca de texto completo | `LPDOC-13` |
| 14 | Até quando a ponte `compat.py` existe | `LPDOC-14` |

---

## 11. O que este documento não decide

- Não decide conteúdo de sistema: publica o que o dono escreveu.
- A aprovação de GitHub Pages como hospedagem canônica não substitui a validação operacional do rollback; `LPDOC-01` permanece aberta até essa prova.
- Não cria ADR.
- Não decide o padrão editorial de cada sistema: cada um tem o seu; o mínimo comum é
  `wyd-docs/docs/contribuir/`.
- Não mede a documentação do projeto: `metricas.md` do site mede o site; `wyd-1.0.0.md` §1 mede o
  projeto.

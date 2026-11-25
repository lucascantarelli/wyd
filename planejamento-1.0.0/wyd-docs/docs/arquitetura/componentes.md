> Pergunta: "quais unidades de código o `wyd-docs` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-docs`
> Dono: equipe do wyd-docs
> Base: `wyd-docs-1.0.0.md` §1, §2, §5, §7; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). O site está implementado em `wyd-docs/` (raiz); a árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `preparar`

- **Responsabilidade:** montar `.build/docs/` a partir dos sistemas e gerar a navegação.
- **API:** `preparar.py` (Python puro).
- **Invariantes:** caminho relativo preservado; nenhum conteúdo interno publicado; coleta só `planejamento-1.0.0/<sistema>/docs/**/*.md` e `wyd-docs/docs/*.md`.
- **Depende de:** `wyd-docs/docs/`, sistemas.
- **Testes:** montagem de N páginas; navegação gerada.
- **Risco:** alto — é a fonte do site.

## `verificar`

- **Responsabilidade:** gate de link, âncora, nav, título, redirect e sintaxe 1.x.
- **API:** `verificar.py site`.
- **Invariantes:** reprova quando qualquer verificação falha; sem modo tolerante.
- **Depende de:** `site/` gerado.
- **Testes:** link quebrado reprova; âncora inexistente reprova.
- **Risco:** alto — é o gate.

## `verificar-contagens`

- **Responsabilidade:** contagens da série e censo por estado.
- **API:** `verificar-contagens.py`.
- **Invariantes:** soma de `Tab.`, arquivos, tabelas, páginas do site ou censo por estado diverge → reprova.
- **Depende de:** montagem.
- **Testes:** contagem divergente reprova.
- **Risco:** médio.

## `verificar-censos`

- **Responsabilidade:** censos citados entre páginas (mede e compara).
- **API:** `verificar-censos.py`.
- **Invariantes:** citação de atos, páginas aceitas, células pendentes ou `Q-BLK` abertas diverge → reprova.
- **Depende de:** fontes.
- **Testes:** censo divergente reprova.
- **Risco:** médio.

## `tema`
**Sem arquivo no repositório:** `mkdocs.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Responsabilidade:** tema Jinja completo (`base.html`, `estilo.css`, `site.js`, `mermaid.js`).
- **API:** templates + assets.
- **Invariantes:** sem `mkdocs.yml`; tema próprio cobre o que a 2.0 não tem.
- **Depende de:** MkDocs 2.0.dev6.
- **Testes:** renderização; claro/escuro.
- **Risco:** médio.

## `publicacao`

- **Responsabilidade:** workflow de CI e GitHub Pages.
- **API:** `.github/workflows/docs.yml`, `.github/actions/docs-check/`.
- **Invariantes:** permissões mínimas; rollback validado.
- **Depende de:** `wyd-ops`.
- **Testes:** build + verificação no CI.
- **Risco:** alto — é a publicação.

## `redirects`

- **Responsabilidade:** mapa origem → destino em `redirects.tsv`; stub e checagens.
- **API:** `redirects.tsv`.
- **Invariantes:** redirect obrigatório ao renomear; origem duplicada reprova.
- **Depende de:** `verificar.py`.
- **Testes:** redirect válido/inválido.
- **Risco:** baixo.

## `metricas`

- **Responsabilidade:** definição e coleta da métrica do site (páginas, fontes, estado).
- **API:** `docs/planos/metricas.md`.
- **Invariantes:** uma fonte, um dono; números citados por seção.
- **Depende de:** `verificar-contagens.py`.
- **Testes:** métrica reproduzível.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-docs/ (raiz)
├── preparar/             # montagem e navegação
├── verificar/            # gate de link/âncora/nav/título
├── contagens/            # verificar-contagens
├── censos/               # verificar-censos
├── tema/                 # base.html, css, js, mermaid
├── publicacao/           # workflow, Pages
├── redirects/            # redirects.tsv
└── metricas/             # definição da métrica
tests/
├── unitario/            # preparar, verificar
├── contrato/            # (n/a)
├── integracao/          # build completo
├── sistema/             # (n/a)
├── e2e/                 # (n/a)
└── propriedade/         # (n/a)
```

*Página de planejamento. Nenhuma unidade foi implementada.*
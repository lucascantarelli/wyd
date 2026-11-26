# Módulos do `wyd-docs` — detalhamento

> Pergunta: "o que cada módulo do `wyd-docs` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento
> Dono: `wyd-docs`
> Base: `docs/arquitetura/componentes.md`; `docs/specs/requisitos.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

## Módulos

### preparar
- **Objetivo:** montar `.build/docs/` a partir dos sistemas e gerar navegação.
- **Escopo:** `preparar.py`.
- **Interfaces:** build.
- **Dependências:** sistemas.
- **Testes:** caminho relativo preservado; sem conteúdo interno.
- **Segurança:** sem conteúdo interno publicado.
- **Monitoramento:** n/a.
- **DoD:** build gera `.build/docs/`.

### verificar
- **Objetivo:** gate de link, âncora, nav, título, redirect, sintaxe.
- **Escopo:** `verificar.py site`.
- **Interfaces:** pass/fail.
- **Dependências:** site gerado.
- **Testes:** link quebrado reprova.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** gate verde.

### verificar-contagens
- **Objetivo:** contagens da série e censo por estado.
- **Escopo:** `verificar-contagens.py`.
- **Interfaces:** pass/fail.
- **Dependências:** montagem.
- **Testes:** contagem divergente reprova.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** gate verde.

### verificar-censos
- **Objetivo:** censos citados entre páginas.
- **Escopo:** `verificar-censos.py`.
- **Interfaces:** pass/fail.
- **Dependências:** fontes.
- **Testes:** censo divergente reprova.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** gate verde.

### tema
- **Objetivo:** tema Jinja (base.html, estilo.css, site.js, mermaid.js).
- **Escopo:** templates + assets.
- **Interfaces:** renderização.
- **Dependências:** MkDocs 2.0.dev6.
- **Testes:** claro/escuro.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** tema próprio.

### publicacao
- **Objetivo:** workflow de CI e GitHub Pages.
- **Escopo:** `.github/workflows/docs.yml`, `.github/actions/docs-check/`.
- **Interfaces:** push.
- **Dependências:** wyd-ops.
- **Testes:** build + verificação no CI.
- **Segurança:** permissões mínimas.
- **Monitoramento:** status de publicação.
- **DoD:** rollback validado.

### redirects
- **Objetivo:** mapa origem → destino.
- **Escopo:** `redirects.tsv`.
- **Interfaces:** stub de redirect.
- **Dependências:** verificar.py.
- **Testes:** redirect válido/inválido.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** origem duplicada reprova.

### metricas
- **Objetivo:** definição e coleta da métrica do site.
- **Escopo:** `docs/planos/metricas.md`.
- **Interfaces:** número reproduzível.
- **Dependências:** verificar-contagens.py.
- **Testes:** métrica reproduzível.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** uma fonte, um dono.
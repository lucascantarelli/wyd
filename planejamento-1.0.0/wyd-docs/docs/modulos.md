# Módulos do `wyd-docs` — detalhamento

> Pergunta: "o que cada módulo do `wyd-docs` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento
> Dono: `wyd-docs`
> Base: `docs/arquitetura/componentes.md`; `docs/specs/requisitos.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25; módulo `gate` implementado em 2026-11-26).

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

### gate
- **Objetivo:** orquestrar `full`, `fast` e `check` com resumo curto e `--json`.
- **Escopo:** `gate/cli.py`, `gate/workspace.py`, `gate.sh`.
- **Interfaces:** pass/fail e JSON; detalhes sob demanda.
- **Dependências:** site gerado no full; checks nativos no fast.
- **Testes:** `tests/test_gate.py`; `make test-gate`.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** gate verde.

### checks
- **Objetivo:** regras de layout, links, alvos, lacunas, contagens, censos, capacidades, hoje e skills; `_texto.py` guarda a leitura comum (linha lógica, frase e isenção por data).
- **Escopo:** `gate/checks/*.py`.
- **Interfaces:** pass/fail.
- **Dependências:** `Workspace` compartilhado.
- **Testes:** fixtures centralizadas em `tests/fixtures.py`.
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
- **Dependências:** gate/checks/links.py.
- **Testes:** redirect válido/inválido.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** origem duplicada reprova.

### metricas
- **Objetivo:** definição e coleta da métrica do site.
- **Escopo:** `docs/planos/metricas.md`.
- **Interfaces:** número reproduzível.
- **Dependências:** gate/checks/contagens.py.
- **Testes:** métrica reproduzível.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** uma fonte, um dono.

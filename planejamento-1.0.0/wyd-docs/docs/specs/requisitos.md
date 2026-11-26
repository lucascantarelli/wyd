> Pergunta: "quais requisitos o `wyd-docs` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-docs-1.0.0.md` §1, §3, §7, §10; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados do plano diretor; `LPDOC-01` fechada, `LPDOC-12` aberta.

Requisito sem critério de aceitação é intenção. Numeração `RF-DOC-nnn` (funcional) e `RNF-DOC-nnn` (não funcional).

## Requisitos funcionais

### RF-DOC-001 — Montar site a partir dos sistemas
- **Entrada:** `planejamento-1.0.0/<sistema>/docs/**/*.md`.
- **Saída:** `.build/docs/` com navegação gerada.
- **Aceitação:** caminho relativo preservado; nenhum conteúdo interno publicado.
- **Contrato:** §3, §4.

### RF-DOC-002 — Verificar link, âncora, nav e título
- **Entrada:** site gerado.
- **Saída:** pass/fail.
- **Aceitação:** link quebrado, âncora inexistente, nav ausente, título ausente reprova.
- **Contrato:** §7.2.

### RF-DOC-003 — Verificar contagens da série
- **Entrada:** montagem.
- **Saída:** pass/fail.
- **Aceitação:** soma divergente reprova.
- **Contrato:** `gate/checks/contagens.py`.

### RF-DOC-004 — Verificar censos citados
- **Entrada:** fontes.
- **Saída:** pass/fail.
- **Aceitação:** citação divergente reprova.
- **Contrato:** `gate/checks/censos.py`.

### RF-DOC-005 — Publicar em GitHub Pages
- **Entrada:** push em `develop`/`main`.
- **Saída:** site publicado.
- **Aceitação:** workflow com permissões mínimas; artefato estático.
- **Contrato:** §7.3.

### RF-DOC-006 — Suportar redirect
- **Entrada:** `redirects.tsv`.
- **Saída:** stub de redirect.
- **Aceitação:** origem duplicada reprova; destino inexistente reprova.
- **Contrato:** `LPDOC-06`.

### RF-DOC-007 — Aplicar tema próprio
**Sem arquivo no repositório:** `mkdocs.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Entrada:** MkDocs 2.0.dev6.
- **Saída:** tema com claro/escuro, filtro, avisos, cópia, destaque.
- **Aceitação:** sem `mkdocs.yml`; tema em `tema/`.
- **Contrato:** §2.2, §6.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-DOC-001 | Build em Linux com Python 3.15 | container `python:3.15-slim` |
| RNF-DOC-002 | Sem conteúdo interno no artefato | varredura de `.build/docs/` |
| RNF-DOC-003 | URL estável | `lucascantarelli.github.io/wyd/` |
| RNF-DOC-004 | Rollback validado | `LPDOC-01` fechada |
| RNF-DOC-005 | Cobertura mínima de 85% onde aplicável | gate de cobertura |

*Página de planejamento. Nenhum requisito foi implementado.*
> Pergunta: "em que ordem o `wyd-docs` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-docs`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Site implementado; plano aberto.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Nenhum conteúdo interno no artefato publicado.
3. Toda fatia de publicação inclui rollback validado.
4. Redirect obrigatório ao renomear.

## 2. Fatias

### F0 — Montagem e tema

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-DOC-001`, `RF-DOC-007` |
| Depende de | `planejamento-1.0.0/<sistema>/docs/**/*.md` |
| Entrega | `preparar.py`, tema Jinja, navegação |
| DoD | build gera `.build/docs/` com caminho relativo preservado; sem conteúdo interno |
| Fora | gate |

### F1 — Gate de link, âncora, nav, título

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-DOC-002` |
| Depende de | F0 |
| Entrega | `verificar.py site` |
| DoD | link quebrado, âncora inexistente, nav/título ausentes reprova |
| Fora | contagens |

### F2 — Contagens e censos

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-DOC-003`, `RF-DOC-004` |
| Depende de | F0 |
| Entrega | `verificar-contagens.py`, `verificar-censos.py` |
| DoD | soma/citação divergente reprova; gate verde |
| Fora | publicação |

### F3 — Publicação e redirect

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-DOC-005`, `RF-DOC-006` |
| Depende de | F0–F2; `LPDOC-01` |
| Entrega | workflow GitHub Pages, `redirects.tsv` |
| DoD | publicação automática em `develop`; rollback validado; redirect válido/inválido reprova duplicata |
| Fora | geração de patch |

## 3. Ordem e dependências

```text
F0 → F1 → F2 → F3
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | conteúdo interno publicado | varredura de `.build/docs/` |
| F1 | gate decorativo | sem modo tolerante |
| F3 | Windows | `LPDOC-12` como decisão |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.
"""Fixtures mínimas, centralizadas, para cada regra do gate.

Cada dicionário é `caminho relativo -> conteúdo`. Os testes criam a árvore em um
diretório temporário; nenhuma fixture depende do repositório real.
"""

LACUNA_OK = {
    "planejamento-1.0.0/wyd-teste/lacunas.md": """# Lacunas de teste

## 2. Lacunas

| ID | Pergunta | Evidência |
|---|---|---|
| `LT-01` | o item foi decidido? | **fechada** (2026-01-01: decidido); evidência em `planejamento-1.0.0/wyd-teste/evidencia.md` |
""",
    "planejamento-1.0.0/wyd-teste/evidencia.md": """# Evidência

`LT-01` decidido: o artefato existe e trata do item.
""",
}

LACUNA_SEM_EVIDENCIA = {
    "planejamento-1.0.0/wyd-teste/lacunas.md": LACUNA_OK[
        "planejamento-1.0.0/wyd-teste/lacunas.md"
    ],
}

SKILLS_OK = {
    "planejamento-1.0.0/wyd-teste/docs/roadmap/roadmap.md": """# Roadmap

| Página |
|---|
| [`WYDX-F1`](WYDX-F1.md) §Skills |
""",
    "planejamento-1.0.0/wyd-teste/docs/roadmap/WYDX-F1.md": """# WYDX-F1 — fatia de teste

## Skills

| campo | valor |
|---|---|
| `skill_principal` | `teste` |

**Dono do valor:** equipe de teste
""",
}

SKILLS_REPETE_FASE = {
    **SKILLS_OK,
    "planejamento-1.0.0/wyd-teste/docs/roadmap/roadmap.md": """# Roadmap

| Página |
|---|
| [`WYDX-F1`](WYDX-F1.md) §Skills |

| Fase | F1 |
""",
}

CONTAGEM_OK = {
    "planejamento-1.0.0/wyd-db/migrations-checklist.md": """# Checklist

## 2. Arquivos

| Nº | Página | Tab. |
|---|---|---|
| 0001 | `0001_bootstrap.md` | 2 |
""",
    "planejamento-1.0.0/wyd-db/ddl-confirmado.md": """# DDL

CREATE TABLE exemplo (id integer);
""",
    "planejamento-1.0.0/wyd-db/plano-migrations.md": """# Plano

0001_bootstrap.sql
""",
}

CENSO_OK = {
    "planejamento-1.0.0/wyd-db/docs/a.md": """# A

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-db/src/b.md": """# B

**Estado:** aceito
""",
    "planejamento-1.0.0/wyd-db/migrations/0001_bootstrap.md": """# Migration

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-db/tests/t.md": """# Teste

**Estado:** aceito
""",
    "planejamento-1.0.0/wyd-db/docs/specs/rastreabilidade.md": """# Rastreabilidade

## 2. Requisitos

| Requisito | História | Ator | Estado | Teste | Issue | Entrega | Nota |
|---|---|---|---|---|---|---|---|
| `RF-DB-001` | login | ator | ok | `P` | — | onda 1 | nota |
""",
    "planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md": """# Pendências

| ID | Situação |
|---|---|
| `Q-BLK-01` | aberta |
""",
}

SITE_OK = {
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="alvo.html">alvo</a></body></html>""",
    "alvo.html": """<html><body><aside class="barra"></aside><h1>Alvo</h1></body></html>""",
}

SITE_QUEBRADO = {
    **SITE_OK,
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="falta.html">quebrado</a></body></html>""",
}

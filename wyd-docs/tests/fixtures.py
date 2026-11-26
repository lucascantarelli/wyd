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

> Pergunta: "a fatia de teste entrega o contrato mínimo?"
> Público: Desenvolvimento e agentes
> Dono: `wyd-teste`
> Base: fixture do gate
> Revisado em: 2026-01-01

**Estado:** proposto
**Fase:** F1
**Gate de verificação:** `python -m gate fast --root ..`

## Contexto

Fixture mínima para a regra de skills.

## Entregável

Uma página de issue no padrão único.

## DoD

- [ ] o gate `skills` passa.

## Fora de escopo

Implementação de produção.

## Depende de

Nada.

## Skills

| campo | valor |
|---|---|
| `skill_principal` | `planejar-sistema` |

**Dono do valor:** equipe de teste

## Evidência ao fechar

Saída verde do gate `skills`.
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

# Link para diretório resolve `index.html`, como o site publicado.
SITE_DIRETORIO = {
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="pasta/">pasta</a></body></html>""",
    "pasta/index.html": """<html><body><aside class="barra"></aside><h1>Pasta</h1></body></html>""",
}

# Link relativo com `..` é normalizado antes de existir.
SITE_RELATIVO = {
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="sub/pagina.html">sub</a></body></html>""",
    "alvo.html": """<html><body><aside class="barra"></aside><h1>Alvo</h1></body></html>""",
    "sub/pagina.html": """<html><body><aside class="barra"></aside><h1>Sub</h1>
<a href="../alvo.html">alvo</a></body></html>""",
}

# Sintaxe 1.x citada em code span não é uso; fora dele reprova.
SINTAXE_1X_CITADA = {
    "planejamento-1.0.0/wyd-teste/docs/pagina.md": """# Página

O bloco `!!! nota` é citado como exemplo.
""",
}

SINTAXE_1X_USADA = {
    "planejamento-1.0.0/wyd-teste/docs/pagina.md": """# Página

!!! nota

Conteúdo.
""",
}


def site_repetido(paginas: int) -> dict[str, str]:
    """Site válido com N páginas e link cruzado; volume força o parse em processos."""
    return {
        f"p{indice:02d}.html": (
            '<html><body><aside class="barra"></aside>'
            f"<h1>Página {indice}</h1>"
            f'<a href="p{(indice + 1) % paginas:02d}.html">próxima</a>'
            "</body></html>"
        )
        for indice in range(paginas)
    }


def site_com_falhas(paginas: int) -> dict[str, str]:
    """Site com N páginas, cada uma com link quebrado e duas âncoras inexistentes.

    O volume (`>= 32`) força o caminho paralelo quando há `fork`; o link remoto aponta
    para uma página que também está no lote, então o alvo já foi analisado e a ordem dos
    problemas não depende de análise sob demanda.
    """
    return {
        f"p{indice:02d}.html": (
            '<html><body><aside class="barra"></aside>'
            f"<h1>Página {indice}</h1>"
            f'<a href="falta-{indice:02d}.html">quebrado</a>'
            '<a href="#nao-existe">âncora local</a>'
            f'<a href="p{(indice + 1) % paginas:02d}.html#sumiu">âncora remota</a>'
            "</body></html>"
        )
        for indice in range(paginas)
    }
